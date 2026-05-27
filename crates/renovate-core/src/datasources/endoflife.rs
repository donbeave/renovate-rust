//! End-of-life date datasource.
//!
//! Renovate reference: `lib/modules/datasource/endoflife-date/index.ts`
//! API: `GET https://endoflife.date/api/{product}.json`

use chrono::{NaiveDate, Utc};
use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const EOL_REGISTRY: &str = "https://endoflife.date/api";
pub const DATASOURCE_ID: &str = "endoflife-date";

#[derive(Debug, Error)]
pub enum EolError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

#[derive(Debug, Clone)]
pub struct EndoflifeRelease {
    pub version: String,
    pub release_timestamp: Option<String>,
    pub is_deprecated: bool,
}

#[derive(Debug, Clone)]
pub struct EndoflifeResult {
    pub releases: Vec<EndoflifeRelease>,
}

// ── API types ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ExpirableField {
    Bool(bool),
    Date(String),
}

#[derive(Debug, Deserialize)]
struct EolApiCycle {
    cycle: String,
    latest: Option<String>,
    #[serde(rename = "releaseDate")]
    release_date: Option<String>,
    eol: Option<ExpirableField>,
    discontinued: Option<ExpirableField>,
}

fn is_expired(field: &Option<ExpirableField>) -> bool {
    match field {
        None => false,
        Some(ExpirableField::Bool(b)) => *b,
        Some(ExpirableField::Date(s)) => NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map(|d| d <= Utc::now().date_naive())
            .unwrap_or(false),
    }
}

fn format_release_timestamp(date_str: &str) -> String {
    format!("{date_str}T00:00:00.000Z")
}

/// Fetch endoflife-date releases for a product.
///
/// Empty `registry_url` → `Ok(None)`. 4xx → `Ok(None)`. 5xx → `Err`.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<EndoflifeResult>, EolError> {
    if registry_url.trim().is_empty() {
        return Ok(None);
    }

    let base = registry_url.trim_end_matches('/');
    let url = format!("{base}/{package_name}.json");

    let text = match http.get_raw_with_accept(&url, "application/json").await {
        Ok(v) => v,
        Err(crate::http::HttpError::Status { status, .. }) if status.is_client_error() => {
            return Ok(None);
        }
        Err(e) => return Err(EolError::Http(e)),
    };

    let cycles: Vec<EolApiCycle> = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    if cycles.is_empty() {
        return Ok(None);
    }

    let mut releases: Vec<EndoflifeRelease> = cycles
        .into_iter()
        .map(|c| EndoflifeRelease {
            version: c.latest.unwrap_or(c.cycle),
            release_timestamp: c.release_date.as_deref().map(format_release_timestamp),
            is_deprecated: is_expired(&c.eol) || is_expired(&c.discontinued),
        })
        .collect();

    releases.sort_by(|a, b| a.release_timestamp.cmp(&b.release_timestamp));

    Ok(Some(EndoflifeResult { releases }))
}

/// Update summary used by pipeline.
#[derive(Debug, Clone)]
pub struct EolUpdateSummary {
    pub latest: Option<String>,
    pub is_eol: bool,
    pub update_available: bool,
}

/// Fetch latest non-deprecated version (pipeline helper).
pub async fn fetch_latest(
    product: &str,
    current_value: &str,
    http: &HttpClient,
) -> Result<EolUpdateSummary, EolError> {
    let result = fetch_releases(EOL_REGISTRY, product, http).await?;
    match result {
        None => Ok(EolUpdateSummary {
            latest: None,
            is_eol: false,
            update_available: false,
        }),
        Some(r) => {
            let latest = r
                .releases
                .iter()
                .rev()
                .find(|rel| !rel.is_deprecated)
                .map(|rel| rel.version.clone());
            let current_eol = r
                .releases
                .iter()
                .any(|rel| rel.version == current_value && rel.is_deprecated);
            let update_available = latest
                .as_deref()
                .map(|l| l != current_value)
                .unwrap_or(false);
            Ok(EolUpdateSummary {
                latest,
                is_eol: current_eol,
                update_available,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    const EKS_JSON: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/endoflife-date/__fixtures__/eks.json"
    );
    const CASSANDRA_JSON: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/endoflife-date/__fixtures__/apache-cassandra.json"
    );
    const FAIRPHONE_JSON: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/endoflife-date/__fixtures__/fairphone.json"
    );

    // Ported: "processes real data" — datasource/endoflife-date/index.spec.ts line 22
    // Note: TypeScript test froze time at 2023-06-03. Expectations reflect current date.
    #[tokio::test]
    async fn processes_real_data() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/amazon-eks.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(EKS_JSON))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "amazon-eks", &http)
            .await
            .unwrap()
            .unwrap();

        // 9 releases sorted ascending by releaseDate
        assert_eq!(result.releases.len(), 9);

        // All eol dates in EKS fixture are in the past (relative to 2026-05-25)
        // except cycle "1.26" which has eol=false
        let r0 = &result.releases[0];
        assert_eq!(r0.version, "1.18-eks-13");
        assert_eq!(
            r0.release_timestamp.as_deref(),
            Some("2020-10-13T00:00:00.000Z")
        );
        assert!(r0.is_deprecated);

        let r8 = &result.releases[8];
        assert_eq!(r8.version, "1.26-eks-1");
        assert_eq!(
            r8.release_timestamp.as_deref(),
            Some("2023-04-11T00:00:00.000Z")
        );
        assert!(!r8.is_deprecated);
    }

    // Ported: "returns null without registryUrl" — datasource/endoflife-date/index.spec.ts line 83
    #[tokio::test]
    async fn returns_null_without_registry_url() {
        let http = HttpClient::new().unwrap();
        let result = fetch_releases("", "amazon-eks", &http).await.unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null for 404" — datasource/endoflife-date/index.spec.ts line 92
    #[tokio::test]
    async fn returns_null_for_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/amazon-eks.json"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "amazon-eks", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null for empty result" — datasource/endoflife-date/index.spec.ts line 102
    #[tokio::test]
    async fn returns_null_for_empty_result() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/amazon-eks.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string("[]"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "amazon-eks", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "throws for 5xx" — datasource/endoflife-date/index.spec.ts line 112
    #[tokio::test]
    async fn throws_for_5xx() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/amazon-eks.json"))
            .respond_with(ResponseTemplate::new(502))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "amazon-eks", &http).await;
        assert!(result.is_err());
    }

    // Ported: "detects boolean discontinuation" — datasource/endoflife-date/index.spec.ts line 122
    // Note: TypeScript test froze time at 2023-06-03. At 2026-05-25 all Cassandra cycles are deprecated.
    #[tokio::test]
    async fn detects_boolean_discontinuation() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/apache-cassandra.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(CASSANDRA_JSON))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "apache-cassandra", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 4);

        // 3.0 and 3.11 are deprecated via discontinued=true (boolean)
        let r0 = &result.releases[0];
        assert_eq!(r0.version, "3.0.29");
        assert_eq!(
            r0.release_timestamp.as_deref(),
            Some("2015-11-09T00:00:00.000Z")
        );
        assert!(r0.is_deprecated);

        let r1 = &result.releases[1];
        assert_eq!(r1.version, "3.11.15");
        assert_eq!(
            r1.release_timestamp.as_deref(),
            Some("2017-06-23T00:00:00.000Z")
        );
        assert!(r1.is_deprecated);

        // 4.0 and 4.1 have eol dates that are in the past at 2026-05-25
        let r2 = &result.releases[2];
        assert_eq!(r2.version, "4.0.9");
        assert_eq!(
            r2.release_timestamp.as_deref(),
            Some("2021-07-26T00:00:00.000Z")
        );

        let r3 = &result.releases[3];
        assert_eq!(r3.version, "4.1.1");
        assert_eq!(
            r3.release_timestamp.as_deref(),
            Some("2022-12-13T00:00:00.000Z")
        );
    }

    // Ported: "detects date discontinuation" — datasource/endoflife-date/index.spec.ts line 158
    #[tokio::test]
    async fn detects_date_discontinuation() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/fairphone.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(FAIRPHONE_JSON))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "fairphone", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 5);

        // Sorted ascending by releaseDate: 2013, 2015, 2019, 2020, 2021
        let r0 = &result.releases[0];
        assert_eq!(r0.version, "1");
        assert_eq!(
            r0.release_timestamp.as_deref(),
            Some("2013-12-01T00:00:00.000Z")
        );
        assert!(r0.is_deprecated); // discontinued=2017-07-13

        let r1 = &result.releases[1];
        assert_eq!(r1.version, "2");
        assert_eq!(
            r1.release_timestamp.as_deref(),
            Some("2015-12-21T00:00:00.000Z")
        );
        assert!(r1.is_deprecated); // discontinued=2019-03-31

        let r2 = &result.releases[2];
        assert_eq!(r2.version, "3");
        assert_eq!(
            r2.release_timestamp.as_deref(),
            Some("2019-09-30T00:00:00.000Z")
        );
        assert!(r2.is_deprecated); // discontinued=2021-09-01

        let r3 = &result.releases[3];
        assert_eq!(r3.version, "3+");
        assert_eq!(
            r3.release_timestamp.as_deref(),
            Some("2020-09-30T00:00:00.000Z")
        );
        assert!(r3.is_deprecated); // discontinued=2022-11-01

        let r4 = &result.releases[4];
        assert_eq!(r4.version, "4");
        assert_eq!(
            r4.release_timestamp.as_deref(),
            Some("2021-09-30T00:00:00.000Z")
        );
        assert!(!r4.is_deprecated); // discontinued=false
    }
}
