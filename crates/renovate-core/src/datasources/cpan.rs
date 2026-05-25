//! CPAN (MetaCPAN) datasource.
//!
//! Renovate reference: `lib/modules/datasource/cpan/index.ts`
//! API: `POST https://fastapi.metacpan.org/v1/file/_search`

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY_URL: &str = "https://fastapi.metacpan.org/";
pub const DATASOURCE_ID: &str = "cpan";

#[derive(Debug, Error)]
pub enum CpanError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

#[derive(Debug, Clone)]
pub struct CpanRelease {
    pub version: String,
    pub distribution: String,
    pub release_timestamp: Option<String>,
    pub is_deprecated: bool,
    pub is_stable: bool,
    pub is_latest: bool,
}

#[derive(Debug, Clone)]
pub struct CpanResult {
    pub releases: Vec<CpanRelease>,
    pub changelog_url: Option<String>,
    pub homepage: Option<String>,
    pub tags: Option<std::collections::HashMap<String, String>>,
}

// ── Response types ────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct HitsWrapper {
    hits: HitsInner,
}

#[derive(Debug, Deserialize)]
struct HitsInner {
    hits: Vec<HitEntry>,
}

#[derive(Debug, Deserialize)]
struct HitEntry {
    _source: Option<SourceEntry>,
}

#[derive(Debug, Deserialize)]
struct SourceEntry {
    module: Option<Vec<ModuleEntry>>,
    distribution: Option<String>,
    date: Option<String>,
    deprecated: Option<bool>,
    maturity: Option<String>,
    status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ModuleEntry {
    version: Option<String>,
}

fn to_ms_timestamp(s: &str) -> Option<String> {
    // "2020-11-30T00:21:36" → "2020-11-30T00:21:36.000Z"
    if s.ends_with('Z') {
        let dot_pos = s.find('.')?;
        let prefix = &s[..dot_pos];
        let rest = &s[dot_pos + 1..s.len() - 1];
        let digits: String = rest.chars().take(3).collect();
        let ms = format!("{:0<3}", digits);
        return Some(format!("{}.{}Z", prefix, ms));
    }
    // no timezone — append .000Z
    if s.contains('T') && !s.contains('+') {
        return Some(format!("{}.000Z", s));
    }
    None
}

fn parse_release(source: SourceEntry) -> Option<CpanRelease> {
    let version = source.module?.into_iter().find_map(|m| {
        m.version.filter(|v| !v.is_empty())
    })?;
    let distribution = source.distribution?;
    // status must be one of backpan/cpan/latest; missing or unknown → skip
    let status = source.status.as_deref()?;
    if !matches!(status, "backpan" | "cpan" | "latest") {
        return None;
    }
    Some(CpanRelease {
        version,
        distribution,
        release_timestamp: source.date.as_deref().and_then(to_ms_timestamp),
        is_deprecated: source.deprecated.unwrap_or(false),
        is_stable: source.maturity.as_deref() == Some("released"),
        is_latest: status == "latest",
    })
}

/// Fetch CPAN releases for a package.
///
/// Returns `None` for 4xx or empty results.
/// Returns `Err` for 5xx server errors.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<CpanResult>, CpanError> {
    let base = registry_url.trim_end_matches('/');
    let search_url = format!("{}/v1/file/_search", base);

    let body = serde_json::json!({
        "query": {
            "bool": {
                "filter": [
                    { "term": { "module.name": package_name } },
                    { "term": { "module.authorized": true } },
                    { "exists": { "field": "module.associated_pod" } }
                ]
            }
        },
        "_source": ["module.name", "module.version", "distribution", "date", "deprecated", "maturity", "status"],
        "sort": [{ "date": "desc" }]
    });
    let body_str = serde_json::to_string(&body).unwrap_or_default();

    let resp: HitsWrapper = match http.post_json(&search_url, &body_str).await {
        Ok(v) => v,
        Err(crate::http::HttpError::Status { status, .. }) if status.is_client_error() => {
            return Ok(None)
        }
        Err(crate::http::HttpError::Request(_)) => return Ok(None),
        Err(e) => return Err(CpanError::Http(e)),
    };

    // Collect descending (newest first from API), find latest metadata, then reverse to ascending.
    let mut releases: Vec<CpanRelease> = resp
        .hits
        .hits
        .into_iter()
        .filter_map(|h| h._source)
        .filter_map(parse_release)
        .collect();

    if releases.is_empty() {
        return Ok(None);
    }

    let latest_distribution = releases[0].distribution.clone();
    let latest_version = releases.iter().find(|r| r.is_latest).map(|r| r.version.clone());

    // Sort ascending by timestamp so newest is last (matches Renovate framework behavior).
    releases.sort_by(|a, b| a.release_timestamp.cmp(&b.release_timestamp));

    let mut tags = None;
    if let Some(ref v) = latest_version {
        let mut map = std::collections::HashMap::new();
        map.insert("latest".to_string(), v.clone());
        tags = Some(map);
    }

    Ok(Some(CpanResult {
        releases,
        changelog_url: Some(format!("https://metacpan.org/dist/{}/changes", latest_distribution)),
        homepage: Some(format!("https://metacpan.org/pod/{}", package_name)),
        tags,
    }))
}

/// Update summary from the CPAN datasource (used by pipeline).
#[derive(Debug)]
pub struct CpanUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Fetch the latest version of a CPAN module.
pub async fn fetch_latest(
    http: &HttpClient,
    module_name: &str,
    current_value: &str,
) -> Result<CpanUpdateSummary, CpanError> {
    let result = fetch_releases(DEFAULT_REGISTRY_URL, module_name, http).await?;
    let latest = result.and_then(|r| {
        r.tags.and_then(|t| t.get("latest").cloned())
            .or_else(|| r.releases.into_iter().next().map(|rel| rel.version))
    });
    let update_available = latest.as_deref().map(|l| l != current_value).unwrap_or(false);
    Ok(CpanUpdateSummary {
        current_value: current_value.to_owned(),
        latest,
        update_available,
    })
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    // ── Schema tests (ported from cpan/schema.spec.ts) ────────────────────────

    // Ported: "filters out entries with empty module arrays" — datasource/cpan/schema.spec.ts line 5
    #[test]
    fn schema_filters_empty_module_array() {
        let source = SourceEntry {
            module: Some(vec![]),
            distribution: Some("Test-Package".into()),
            date: Some("2023-01-01T00:00:00".into()),
            deprecated: Some(false),
            maturity: Some("released".into()),
            status: Some("latest".into()),
        };
        assert!(parse_release(source).is_none());
    }

    // Ported: "filters out entries where module has no version" — datasource/cpan/schema.spec.ts line 27
    #[test]
    fn schema_filters_empty_version() {
        let source = SourceEntry {
            module: Some(vec![ModuleEntry { version: Some("".into()) }]),
            distribution: Some("Test-Package".into()),
            date: Some("2023-01-01T00:00:00".into()),
            deprecated: Some(false),
            maturity: Some("released".into()),
            status: Some("latest".into()),
        };
        assert!(parse_release(source).is_none());
    }

    // Ported: "includes valid entries" — datasource/cpan/schema.spec.ts line 49
    #[test]
    fn schema_includes_valid_entries() {
        let source = SourceEntry {
            module: Some(vec![ModuleEntry { version: Some("1.0".into()) }]),
            distribution: Some("Test-Package".into()),
            date: Some("2023-01-01T00:00:00".into()),
            deprecated: Some(false),
            maturity: Some("released".into()),
            status: Some("latest".into()),
        };
        let release = parse_release(source).unwrap();
        assert_eq!(release.version, "1.0");
        assert_eq!(release.distribution, "Test-Package");
        assert!(!release.is_deprecated);
        assert!(release.is_stable);
        assert!(release.is_latest);
    }

    // ── HTTP tests ────────────────────────────────────────────────────────────

    // Ported: "returns null for empty result" — datasource/cpan/index.spec.ts line 11
    #[tokio::test]
    async fn returns_null_for_empty_result() {
        let fixture = include_str!(
            "../../../../../renovate/lib/modules/datasource/cpan/__fixtures__/empty.json"
        );
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/file/_search"))
            .respond_with(ResponseTemplate::new(200).set_body_string(fixture))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "FooBar", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null for 404" — datasource/cpan/index.spec.ts line 27
    #[tokio::test]
    async fn returns_null_for_404() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/file/_search"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "Plack", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "throws for 5xx" — datasource/cpan/index.spec.ts line 37
    #[tokio::test]
    async fn throws_for_5xx() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/file/_search"))
            .respond_with(ResponseTemplate::new(502))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "Plack", &http).await;
        assert!(result.is_err());
    }

    // Ported: "returns null for unknown error" — datasource/cpan/index.spec.ts line 47
    #[tokio::test]
    async fn returns_null_for_unknown_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/file/_search"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "Plack", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "processes real data" — datasource/cpan/index.spec.ts line 57
    #[tokio::test]
    async fn processes_real_data() {
        let fixture = include_str!(
            "../../../../../renovate/lib/modules/datasource/cpan/__fixtures__/Plack.json"
        );
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/file/_search"))
            .respond_with(ResponseTemplate::new(200).set_body_string(fixture))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "Plack", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 10);
        assert_eq!(
            result.changelog_url.as_deref(),
            Some("https://metacpan.org/dist/Plack/changes")
        );
        assert_eq!(
            result.homepage.as_deref(),
            Some("https://metacpan.org/pod/Plack")
        );
        assert_eq!(result.tags.as_ref().and_then(|t| t.get("latest")).map(|s| s.as_str()), Some("1.0048"));

        // releases[1]: isDeprecated=false, isStable=false, releaseTimestamp="2016-04-01T16:58:21.000Z", version="1.0040"
        let r1 = &result.releases[1];
        assert!(!r1.is_deprecated);
        assert!(!r1.is_stable);
        assert_eq!(r1.release_timestamp.as_deref(), Some("2016-04-01T16:58:21.000Z"));
        assert_eq!(r1.version, "1.0040");

        // releases[9]: isDeprecated=false, isStable=true, releaseTimestamp="2020-11-30T00:21:36.000Z", version="1.0048"
        let r9 = &result.releases[9];
        assert!(!r9.is_deprecated);
        assert!(r9.is_stable);
        assert_eq!(r9.release_timestamp.as_deref(), Some("2020-11-30T00:21:36.000Z"));
        assert_eq!(r9.version, "1.0048");
    }
}
