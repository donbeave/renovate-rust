//! Devbox package datasource.
//!
//! Renovate reference: `lib/modules/datasource/devbox/index.ts`
//! API: `GET https://search.devbox.sh/v2/pkg?name=<package>`

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const DEVBOX_API_BASE: &str = "https://search.devbox.sh/v2";
pub const DATASOURCE_ID: &str = "devbox";

#[derive(Debug, Error)]
pub enum DevboxError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

#[derive(Debug, Clone)]
pub struct DevboxRelease {
    pub version: String,
    pub release_timestamp: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DevboxResult {
    pub releases: Vec<DevboxRelease>,
    pub homepage: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DevboxResponseOuter {
    releases: Option<Vec<DevboxReleaseRaw>>,
    homepage_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DevboxReleaseRaw {
    version: String,
    last_updated: Option<String>,
}

fn normalize_timestamp(ts: &str) -> String {
    // "2024-05-22T06:18:38Z" → "2024-05-22T06:18:38.000Z"
    if ts.ends_with('Z') && !ts.contains('.') {
        let without_z = &ts[..ts.len() - 1];
        return format!("{}.000Z", without_z);
    }
    ts.to_owned()
}

/// Fetch Devbox package releases.
///
/// 4xx → `Ok(None)`. 5xx / network → `Err`. Empty/missing → `Ok(None)`.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<DevboxResult>, DevboxError> {
    let encoded: String = package_name
        .bytes()
        .flat_map(|b| {
            if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.') {
                vec![b as char]
            } else {
                format!("%{b:02X}").chars().collect::<Vec<_>>()
            }
        })
        .collect();
    let base = registry_url.trim_end_matches('/');
    let url = format!("{}/pkg?name={}", base, encoded);

    let text = match http.get_raw_with_accept(&url, "application/json").await {
        Ok(v) => v,
        Err(crate::http::HttpError::Status { status, .. }) if status.is_client_error() => {
            return Ok(None);
        }
        Err(e) => return Err(DevboxError::Http(e)),
    };

    let outer: DevboxResponseOuter = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    let raw_releases = match outer.releases {
        Some(r) if !r.is_empty() => r,
        _ => return Ok(None),
    };

    let mut releases: Vec<DevboxRelease> = raw_releases
        .into_iter()
        .map(|r| DevboxRelease {
            version: r.version,
            release_timestamp: r.last_updated.as_deref().map(normalize_timestamp),
        })
        .collect();

    // Sort ascending by timestamp (oldest first, newest last).
    releases.sort_by(|a, b| a.release_timestamp.cmp(&b.release_timestamp));

    Ok(Some(DevboxResult {
        releases,
        homepage: outer.homepage_url,
    }))
}

/// Update summary used by pipeline.
#[derive(Debug, Clone)]
pub struct DevboxUpdateSummary {
    pub update_available: bool,
    pub current_version: String,
    pub latest: Option<String>,
}

/// Fetch the latest version (pipeline helper).
pub async fn fetch_latest(
    http: &HttpClient,
    package: &str,
    current_version: &str,
) -> Result<DevboxUpdateSummary, DevboxError> {
    let result = fetch_releases(DEVBOX_API_BASE, package, http).await?;
    let latest = result.and_then(|r| r.releases.into_iter().last().map(|rel| rel.version));
    let update_available = latest
        .as_deref()
        .map(|l| l != current_version)
        .unwrap_or(false);
    Ok(DevboxUpdateSummary {
        update_available,
        current_version: current_version.to_owned(),
        latest,
    })
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    // Ported: "throws for error" — lib/modules/datasource/devbox/index.spec.ts line 29
    #[tokio::test]
    async fn throws_for_network_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/pkg"))
            .and(query_param("name", "nodejs"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "nodejs", &http).await;
        assert!(result.is_err());
    }

    // Ported: "returns null for 404" — lib/modules/datasource/devbox/index.spec.ts line 43
    #[tokio::test]
    async fn returns_null_for_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/pkg"))
            .and(query_param("name", "nodejs"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "nodejs", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null for empty result" — lib/modules/datasource/devbox/index.spec.ts line 53
    #[tokio::test]
    async fn returns_null_for_empty_result() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/pkg"))
            .and(query_param("name", "nodejs"))
            .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "nodejs", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null for empty 200 OK" — lib/modules/datasource/devbox/index.spec.ts line 63
    #[tokio::test]
    async fn returns_null_for_empty_releases_array() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/pkg"))
            .and(query_param("name", "nodejs"))
            .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"releases":[]}"#))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "nodejs", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "throws for 5xx" — lib/modules/datasource/devbox/index.spec.ts line 76
    #[tokio::test]
    async fn throws_for_5xx() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/pkg"))
            .and(query_param("name", "nodejs"))
            .respond_with(ResponseTemplate::new(502))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "nodejs", &http).await;
        assert!(result.is_err());
    }

    // Ported: "processes real data" — lib/modules/datasource/devbox/index.spec.ts line 86
    #[tokio::test]
    async fn processes_real_data() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/pkg"))
            .and(query_param("name", "nodejs"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"{
                "name": "nodejs",
                "summary": "Event-driven I/O framework for the V8 JavaScript engine",
                "homepage_url": "https://nodejs.org",
                "license": "MIT",
                "releases": [
                    {"version": "22.2.0", "last_updated": "2024-05-22T06:18:38Z"},
                    {"version": "22.0.0", "last_updated": "2024-05-12T16:19:40Z"},
                    {"version": "21.7.3", "last_updated": "2024-04-19T21:36:04Z"}
                ]
            }"#,
            ))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "nodejs", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.homepage.as_deref(), Some("https://nodejs.org"));
        assert_eq!(result.releases.len(), 3);
        // sorted ascending by timestamp
        assert_eq!(result.releases[0].version, "21.7.3");
        assert_eq!(
            result.releases[0].release_timestamp.as_deref(),
            Some("2024-04-19T21:36:04.000Z")
        );
        assert_eq!(result.releases[1].version, "22.0.0");
        assert_eq!(
            result.releases[1].release_timestamp.as_deref(),
            Some("2024-05-12T16:19:40.000Z")
        );
        assert_eq!(result.releases[2].version, "22.2.0");
        assert_eq!(
            result.releases[2].release_timestamp.as_deref(),
            Some("2024-05-22T06:18:38.000Z")
        );
    }

    // Ported: "processes empty data" — lib/modules/datasource/devbox/index.spec.ts line 118
    #[tokio::test]
    async fn processes_empty_data() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/pkg"))
            .and(query_param("name", "nodejs"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"{
                "name": "nodejs",
                "summary": "...",
                "homepage_url": "https://nodejs.org",
                "license": "MIT",
                "releases": []
            }"#,
            ))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "nodejs", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null when no body is returned" — lib/modules/datasource/devbox/index.spec.ts line 133
    #[tokio::test]
    async fn returns_null_for_empty_body() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/pkg"))
            .and(query_param("name", "nodejs"))
            .respond_with(ResponseTemplate::new(200).set_body_string("null"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "nodejs", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "falls back to a default homepage_url" — lib/modules/datasource/devbox/index.spec.ts line 145
    #[tokio::test]
    async fn falls_back_for_missing_homepage() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/pkg"))
            .and(query_param("name", "nodejs"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"{
                "name": "nodejs",
                "summary": "...",
                "license": "MIT",
                "releases": [
                    {"version": "22.2.0", "last_updated": "2024-05-22T06:18:38Z"}
                ]
            }"#,
            ))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "nodejs", &http)
            .await
            .unwrap()
            .unwrap();
        assert!(result.homepage.is_none());
    }
}
