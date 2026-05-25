//! Cloud Native Buildpacks Registry datasource.
//!
//! Renovate reference: `lib/modules/datasource/buildpacks-registry/index.ts`
//! API: `GET {registry}/api/v1/buildpacks/{namespace}/{name}`

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY: &str = "https://registry.buildpacks.io";
pub const DATASOURCE_ID: &str = "buildpacks-registry";

#[derive(Debug, Error)]
pub enum BuildpacksError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

#[derive(Debug, Clone)]
pub struct BuildpacksRelease {
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct BuildpacksResult {
    pub releases: Vec<BuildpacksRelease>,
    pub source_url: Option<String>,
    pub registry_url: String,
}

// ── API types ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct ApiLatest {
    homepage: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiVersion {
    version: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    latest: Option<ApiLatest>,
    #[serde(default)]
    versions: Vec<ApiVersion>,
}

/// Fetch CNB Registry buildpack releases.
///
/// All HTTP errors / empty versions / parse errors → `Ok(None)`.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<BuildpacksResult>, BuildpacksError> {
    let base = registry_url.trim_end_matches('/');
    let url = format!("{base}/api/v1/buildpacks/{package_name}");

    let text = match http.get_raw_with_accept(&url, "application/json").await {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    let api: ApiResponse = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    if api.versions.is_empty() {
        return Ok(None);
    }

    let mut releases: Vec<BuildpacksRelease> =
        api.versions.into_iter().map(|v| BuildpacksRelease { version: v.version }).collect();

    // Sort ascending by semver (matches Renovate's sortAndRemoveDuplicates).
    releases.sort_by(|a, b| {
        let av = semver::Version::parse(&a.version).ok();
        let bv = semver::Version::parse(&b.version).ok();
        match (av, bv) {
            (Some(av), Some(bv)) => av.cmp(&bv),
            (Some(_), None) => std::cmp::Ordering::Greater,
            (None, Some(_)) => std::cmp::Ordering::Less,
            (None, None) => a.version.cmp(&b.version),
        }
    });

    let source_url = api.latest.and_then(|l| l.homepage);

    Ok(Some(BuildpacksResult { releases, source_url, registry_url: base.to_string() }))
}

/// Update summary used by pipeline.
#[derive(Debug)]
pub struct BuildpacksUpdateSummary {
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Fetch latest version (pipeline helper).
pub async fn fetch_latest(
    http: &HttpClient,
    package_name: &str,
    current_value: &str,
) -> Result<BuildpacksUpdateSummary, BuildpacksError> {
    let result = fetch_releases(DEFAULT_REGISTRY, package_name, http).await?;
    let latest = result.and_then(|r| r.releases.last().map(|rel| rel.version.clone()));
    let update_available = latest.as_deref().map(|l| l != current_value).unwrap_or(false);
    Ok(BuildpacksUpdateSummary { latest, update_available })
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    const REAL_DATA: &str = r#"{
        "latest": {
            "version": "0.17.1",
            "namespace": "heroku",
            "name": "python",
            "description": "Heroku's buildpack for Python applications.",
            "homepage": "https://github.com/heroku/buildpacks-python",
            "licenses": ["BSD-3-Clause"],
            "stacks": ["*"],
            "id": "75946bf8-3f6a-4af0-a757-614bebfdfcd6"
        },
        "versions": [
            {
                "version": "0.17.1",
                "_link": "https://registry.buildpacks.io//api/v1/buildpacks/heroku/python/0.17.1"
            },
            {
                "version": "0.17.0",
                "_link": "https://registry.buildpacks.io//api/v1/buildpacks/heroku/python/0.17.0"
            }
        ]
    }"#;

    // Ported: "processes real data" — datasource/buildpacks-registry/index.spec.ts line 9
    #[tokio::test]
    async fn processes_real_data() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/buildpacks/heroku/python"))
            .respond_with(ResponseTemplate::new(200).set_body_string(REAL_DATA))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "heroku/python", &http).await.unwrap().unwrap();

        assert_eq!(result.releases.len(), 2);
        // Sorted ascending
        assert_eq!(result.releases[0].version, "0.17.0");
        assert_eq!(result.releases[1].version, "0.17.1");
        assert_eq!(result.source_url.as_deref(), Some("https://github.com/heroku/buildpacks-python"));
    }

    // Ported: "returns null on empty result" — datasource/buildpacks-registry/index.spec.ts line 48
    #[tokio::test]
    async fn returns_null_on_empty_result() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/buildpacks/heroku/empty"))
            .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "heroku/empty", &http).await.unwrap();
        assert!(result.is_none());
    }

    // Ported: "handles not found" — datasource/buildpacks-registry/index.spec.ts line 57
    #[tokio::test]
    async fn handles_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/buildpacks/heroku/notexisting"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "heroku/notexisting", &http).await.unwrap();
        assert!(result.is_none());
    }
}
