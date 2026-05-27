//! CDNJS datasource.
//!
//! Renovate reference: `lib/modules/datasource/cdnjs/index.ts`
//! API: `GET {registry}libraries/{library}?fields=homepage,repository,versions`

use std::collections::HashMap;

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY: &str = "https://api.cdnjs.com/";
pub const DATASOURCE_ID: &str = "cdnjs";

#[derive(Debug, Error)]
pub enum CdnjsError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Parse(serde_json::Error),
}

#[derive(Debug, Clone)]
pub struct CdnjsRelease {
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct CdnjsResult {
    pub releases: Vec<CdnjsRelease>,
    pub source_url: Option<String>,
    pub homepage: Option<String>,
}

// ── API types ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct ApiRepository {
    url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiVersionResponse {
    #[serde(default)]
    versions: Vec<String>,
    homepage: Option<String>,
    repository: Option<ApiRepository>,
}

#[derive(Debug, Deserialize)]
struct ApiSriResponse {
    sri: Option<HashMap<String, String>>,
}

/// Fetch cdnjs library releases.
///
/// 404 → `Ok(None)`. All other HTTP errors / parse errors → `Err(...)`. Empty versions → `Ok(None)`.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<CdnjsResult>, CdnjsError> {
    let base = registry_url.trim_end_matches('/');
    let library = package_name.split('/').next().unwrap_or(package_name);
    let url = format!("{base}/libraries/{library}?fields=homepage,repository,versions");

    let text = match http.get_raw_with_accept(&url, "application/json").await {
        Ok(v) => v,
        Err(crate::http::HttpError::Status { status, .. }) if status.as_u16() == 404 => {
            return Ok(None);
        }
        Err(e) => return Err(CdnjsError::Http(e)),
    };

    let api: ApiVersionResponse = serde_json::from_str(&text).map_err(CdnjsError::Parse)?;

    if api.versions.is_empty() {
        return Ok(None);
    }

    let releases = api
        .versions
        .into_iter()
        .map(|v| CdnjsRelease { version: v })
        .collect();
    let source_url = api.repository.and_then(|r| r.url);

    Ok(Some(CdnjsResult {
        releases,
        source_url,
        homepage: api.homepage,
    }))
}

/// Fetch the SRI digest for a specific file and version.
///
/// Returns `None` if the file is not found in the SRI map.
/// HTTP errors → `Err(...)`.
pub async fn get_digest(
    registry_url: &str,
    package_name: &str,
    version: &str,
    http: &HttpClient,
) -> Result<Option<String>, CdnjsError> {
    let base = registry_url.trim_end_matches('/');
    let library = package_name.split('/').next().unwrap_or(package_name);
    let asset_name = package_name
        .strip_prefix(&format!("{library}/"))
        .unwrap_or(package_name);
    let url = format!("{base}/libraries/{library}/{version}?fields=sri");

    let text = match http.get_raw_with_accept(&url, "application/json").await {
        Ok(v) => v,
        Err(e) => return Err(CdnjsError::Http(e)),
    };

    let api: ApiSriResponse = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    let digest = api.sri.and_then(|m| m.get(asset_name).cloned());
    Ok(digest)
}

/// Update summary used by pipeline.
#[derive(Debug)]
pub struct CdnjsUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Fetch latest version (pipeline helper).
pub async fn fetch_latest(
    http: &HttpClient,
    library: &str,
    current_value: &str,
) -> Result<CdnjsUpdateSummary, CdnjsError> {
    let result = fetch_releases(DEFAULT_REGISTRY, library, http).await?;
    let latest = result.and_then(|r| r.releases.last().map(|rel| rel.version.clone()));
    let update_available = latest
        .as_deref()
        .map(|l| l != current_value)
        .unwrap_or(false);
    Ok(CdnjsUpdateSummary {
        current_value: current_value.to_owned(),
        latest,
        update_available,
    })
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    const D3_FORCE: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/cdnjs/__fixtures__/d3-force.json"
    );
    const SRI_JSON: &str =
        include_str!("../../../../../renovate/lib/modules/datasource/cdnjs/__fixtures__/sri.json");

    // Ported: "throws for empty result" — datasource/cdnjs/index.spec.ts line 18
    #[tokio::test]
    async fn throws_for_malformed_json() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/libraries/foo"))
            .respond_with(ResponseTemplate::new(200).set_body_string("}"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "foo/bar", &http).await;
        assert!(result.is_err());
    }

    // Ported: "throws for error" — datasource/cdnjs/index.spec.ts line 28
    #[tokio::test]
    async fn throws_for_network_error() {
        let http = HttpClient::new().unwrap();
        let result = fetch_releases("http://127.0.0.1:1", "foo/bar", &http).await;
        assert!(result.is_err());
    }

    // Ported: "returns null for 404" — datasource/cdnjs/index.spec.ts line 38
    #[tokio::test]
    async fn returns_null_for_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/libraries/foo"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "foo/bar", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null for empty 200 OK" — datasource/cdnjs/index.spec.ts line 48
    #[tokio::test]
    async fn returns_null_for_empty_result() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/libraries/doesnotexist"))
            .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "doesnotexist/doesnotexist", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "throws for 401" — datasource/cdnjs/index.spec.ts line 61
    #[tokio::test]
    async fn throws_for_401() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/libraries/foo"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "foo/bar", &http).await;
        assert!(result.is_err());
    }

    // Ported: "throws for 429" — datasource/cdnjs/index.spec.ts line 71
    #[tokio::test]
    async fn throws_for_429() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/libraries/foo"))
            .respond_with(ResponseTemplate::new(429))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "foo/bar", &http).await;
        assert!(result.is_err());
    }

    // Ported: "throws for 5xx" — datasource/cdnjs/index.spec.ts line 81
    #[tokio::test]
    async fn throws_for_5xx() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/libraries/foo"))
            .respond_with(ResponseTemplate::new(502))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "foo/bar", &http).await;
        assert!(result.is_err());
    }

    // Ported: "throws for unknown error" — datasource/cdnjs/index.spec.ts line 91
    #[tokio::test]
    async fn throws_for_unknown_error() {
        let http = HttpClient::new().unwrap();
        let result = fetch_releases("http://127.0.0.1:1", "foo/bar", &http).await;
        assert!(result.is_err());
    }

    // Ported: "processes real data" — datasource/cdnjs/index.spec.ts line 101
    #[tokio::test]
    async fn processes_real_data() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/libraries/d3-force"))
            .respond_with(ResponseTemplate::new(200).set_body_string(D3_FORCE))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "d3-force/d3-force.js", &http)
            .await
            .unwrap()
            .unwrap();

        assert!(!result.releases.is_empty());
        assert_eq!(
            result.homepage.as_deref(),
            Some("https://d3js.org/d3-force/")
        );
        assert_eq!(
            result.source_url.as_deref(),
            Some("https://github.com/d3/d3-force.git")
        );
    }

    // Ported: "returs null for no result" — datasource/cdnjs/index.spec.ts line 115
    #[tokio::test]
    async fn digest_returns_null_for_empty_response() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/libraries/foo/1.2.0"))
            .and(query_param("fields", "sri"))
            .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = get_digest(&server.uri(), "foo/bar", "1.2.0", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returs null for empty sri object" — datasource/cdnjs/index.spec.ts line 131
    #[tokio::test]
    async fn digest_returns_null_for_empty_sri() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/libraries/foo/1.2.0"))
            .and(query_param("fields", "sri"))
            .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"sri":{}}"#))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = get_digest(&server.uri(), "foo/bar", "1.2.0", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returs null if file not found" — datasource/cdnjs/index.spec.ts line 147
    #[tokio::test]
    async fn digest_returns_null_if_file_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/libraries/foo/1.2.0"))
            .and(query_param("fields", "sri"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string(r#"{"sri":{"string":"hash"}}"#),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        // packageName "foo/bar" → asset "bar", but SRI has "string" not "bar"
        let result = get_digest(&server.uri(), "foo/bar", "1.2.0", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null for 404" — datasource/cdnjs/index.spec.ts line 163
    #[tokio::test]
    async fn digest_throws_for_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/libraries/foo/1.2.0"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = get_digest(&server.uri(), "foo/bar", "1.2.0", &http).await;
        assert!(result.is_err());
    }

    // Ported: "returns digest" — datasource/cdnjs/index.spec.ts line 176
    #[tokio::test]
    async fn digest_returns_hash() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/libraries/bootstrap/5.2.3"))
            .and(query_param("fields", "sri"))
            .respond_with(ResponseTemplate::new(200).set_body_string(SRI_JSON))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = get_digest(
            &server.uri(),
            "bootstrap/js/bootstrap.min.js",
            "5.2.3",
            &http,
        )
        .await
        .unwrap();
        assert_eq!(
            result.as_deref(),
            Some(
                "sha512-1/RvZTcCDEUjY/CypiMz+iqqtaoQfAITmNSJY17Myp4Ms5mdxPS5UV7iOfdZoxcGhzFbOm6sntTKJppjvuhg4g=="
            )
        );
    }
}
