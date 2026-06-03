//! Bazel Central Registry datasource.
//!
//! Renovate reference: `lib/modules/datasource/bazel/index.ts`
//! API: `GET {registry}/modules/{name}/metadata.json`
//! Local: `file://{registry}/modules/{name}/metadata.json`

use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY: &str =
    "https://raw.githubusercontent.com/bazelbuild/bazel-central-registry/main";
pub const DATASOURCE_ID: &str = "bazel";

#[derive(Debug, Error)]
pub enum BazelError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct BazelRelease {
    pub version: String,
    pub is_deprecated: bool,
}

#[derive(Debug, Clone)]
pub struct BazelResult {
    pub releases: Vec<BazelRelease>,
    pub source_url: Option<String>,
    pub registry_url: String,
}

// ── API types ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct BazelMetadata {
    #[serde(default)]
    versions: Vec<String>,
    #[serde(default)]
    yanked_versions: HashMap<String, String>,
    homepage: Option<String>,
}

fn parse_metadata(meta: BazelMetadata, registry_url: &str) -> Option<BazelResult> {
    if meta.versions.is_empty() {
        return None;
    }

    let mut releases: Vec<BazelRelease> = meta
        .versions
        .iter()
        .map(|v| BazelRelease {
            version: v.clone(),
            is_deprecated: meta.yanked_versions.contains_key(v),
        })
        .collect();

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

    Some(BazelResult {
        releases,
        source_url: meta.homepage,
        registry_url: registry_url.to_owned(),
    })
}

/// Fetch Bazel Central Registry module releases.
///
/// Supports both HTTP and `file://` registry URLs.
/// 404 → `Ok(None)`. 5xx / network errors → `Err(...)`. Empty versions → `Ok(None)`.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<BazelResult>, BazelError> {
    if registry_url.starts_with("file://") {
        return fetch_releases_local(registry_url, package_name).await;
    }
    fetch_releases_http(registry_url, package_name, http).await
}

async fn fetch_releases_http(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<BazelResult>, BazelError> {
    let base = registry_url.trim_end_matches('/');
    let url = format!("{base}/modules/{package_name}/metadata.json");

    let text = match http.get_raw_with_accept(&url, "application/json").await {
        Ok(v) => v,
        Err(crate::http::HttpError::Status { status, .. }) if status.as_u16() == 404 => {
            return Ok(None);
        }
        Err(e) => return Err(BazelError::Http(e)),
    };

    let meta: BazelMetadata = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    Ok(parse_metadata(meta, base))
}

async fn fetch_releases_local(
    registry_url: &str,
    package_name: &str,
) -> Result<Option<BazelResult>, BazelError> {
    let local_path = registry_url.strip_prefix("file://").unwrap_or(registry_url);
    let file_path = Path::new(local_path)
        .join("modules")
        .join(package_name)
        .join("metadata.json");

    if !file_path.exists() {
        return Ok(None);
    }

    let text = tokio::fs::read_to_string(&file_path).await?;
    let meta: BazelMetadata = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    Ok(parse_metadata(meta, registry_url))
}

/// Update summary used by pipeline.
#[derive(Debug)]
pub struct BazelUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Fetch latest version (pipeline helper).
pub async fn fetch_latest(
    http: &HttpClient,
    module_name: &str,
    current_value: &str,
) -> Result<BazelUpdateSummary, BazelError> {
    let result = fetch_releases(DEFAULT_REGISTRY, module_name, http).await?;
    let latest = result.and_then(|r| {
        r.releases
            .iter()
            .rev()
            .find(|rel| !rel.is_deprecated)
            .map(|rel| rel.version.clone())
    });
    let update_available = latest
        .as_deref()
        .map(|l| l != current_value)
        .unwrap_or(false);
    Ok(BazelUpdateSummary {
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

    const METADATA_NO_YANKED: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/bazel/__fixtures__/metadata-no-yanked-versions.json"
    );
    const METADATA_WITH_YANKED: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/bazel/__fixtures__/metadata-with-yanked-versions.json"
    );

    // Ported: "throws for error" — lib/modules/datasource/bazel/index.spec.ts line 26
    #[tokio::test]
    async fn throws_for_network_error() {
        // Use an unreachable address to simulate a network error.
        let http = HttpClient::new().unwrap();
        let result = fetch_releases("http://127.0.0.1:1", "rules_foo", &http).await;
        assert!(result.is_err());
    }

    // Ported: "returns null for 404" — lib/modules/datasource/bazel/index.spec.ts line 33
    #[tokio::test]
    async fn returns_null_for_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/modules/rules_foo/metadata.json"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "rules_foo", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null for empty result" — lib/modules/datasource/bazel/index.spec.ts line 38
    #[tokio::test]
    async fn returns_null_for_empty_result() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/modules/rules_foo/metadata.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "rules_foo", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null for empty 200 OK" — lib/modules/datasource/bazel/index.spec.ts line 43
    #[tokio::test]
    async fn returns_null_for_empty_versions() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/modules/rules_foo/metadata.json"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(r#"{"versions":[],"yanked_versions":{}}"#),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "rules_foo", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "throws for 5xx" — lib/modules/datasource/bazel/index.spec.ts line 51
    #[tokio::test]
    async fn throws_for_5xx() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/modules/rules_foo/metadata.json"))
            .respond_with(ResponseTemplate::new(502))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "rules_foo", &http).await;
        assert!(result.is_err());
    }

    // Ported: "metadata without yanked versions" — lib/modules/datasource/bazel/index.spec.ts line 58
    #[tokio::test]
    async fn metadata_without_yanked_versions() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/modules/rules_foo/metadata.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(METADATA_NO_YANKED))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "rules_foo", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 4);
        assert_eq!(result.releases[0].version, "0.14.8");
        assert!(!result.releases[0].is_deprecated);
        assert_eq!(result.releases[1].version, "0.14.9");
        assert_eq!(result.releases[2].version, "0.15.0");
        assert!(!result.releases[2].is_deprecated);
        assert_eq!(result.releases[3].version, "0.16.0");
        assert_eq!(
            result.source_url.as_deref(),
            Some("https://github.com/foo/bar")
        );
    }

    // Ported: "metadata with yanked versions" — lib/modules/datasource/bazel/index.spec.ts line 77
    #[tokio::test]
    async fn metadata_with_yanked_versions() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/modules/rules_foo/metadata.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(METADATA_WITH_YANKED))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "rules_foo", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 4);
        assert_eq!(result.releases[0].version, "0.14.8");
        assert!(!result.releases[0].is_deprecated);
        assert_eq!(result.releases[2].version, "0.15.0");
        assert!(result.releases[2].is_deprecated);
        assert_eq!(result.releases[3].version, "0.16.0");
        assert!(!result.releases[3].is_deprecated);
        assert!(result.source_url.is_none());
    }

    // Ported: "should handle local file correctly" — lib/modules/datasource/bazel/index.spec.ts line 106
    #[tokio::test]
    async fn local_file_handles_correctly() {
        let dir = tempfile::tempdir().unwrap();
        let modules_dir = dir.path().join("modules").join("rules_foo");
        tokio::fs::create_dir_all(&modules_dir).await.unwrap();
        let metadata = serde_json::json!({
            "versions": ["0.14.8", "0.14.9", "0.15.0", "0.16.0"],
            "yanked_versions": {"0.15.0": "yanked for security reasons"},
            "homepage": "https://github.com/foo/bar"
        });
        tokio::fs::write(
            modules_dir.join("metadata.json"),
            serde_json::to_string(&metadata).unwrap(),
        )
        .await
        .unwrap();

        let http = HttpClient::new().unwrap();
        let registry_url = format!("file://{}", dir.path().display());
        let result = fetch_releases(&registry_url, "rules_foo", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.registry_url, registry_url);
        assert_eq!(result.releases.len(), 4);
        assert_eq!(result.releases[0].version, "0.14.8");
        assert!(!result.releases[0].is_deprecated);
        assert_eq!(result.releases[1].version, "0.14.9");
        assert_eq!(result.releases[2].version, "0.15.0");
        assert!(result.releases[2].is_deprecated);
        assert_eq!(result.releases[3].version, "0.16.0");
        assert_eq!(
            result.source_url.as_deref(),
            Some("https://github.com/foo/bar")
        );
    }

    // Ported: "should return null for invalid file path" — lib/modules/datasource/bazel/index.spec.ts line 135
    #[tokio::test]
    async fn local_file_returns_null_for_invalid_path() {
        let http = HttpClient::new().unwrap();
        let result = fetch_releases("file:///nonexistent/path", "rules_foo", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "should return null for empty file content" — lib/modules/datasource/bazel/index.spec.ts line 146
    #[tokio::test]
    async fn local_file_returns_null_for_empty_content() {
        let dir = tempfile::tempdir().unwrap();
        let modules_dir = dir.path().join("modules").join("rules_foo");
        tokio::fs::create_dir_all(&modules_dir).await.unwrap();
        tokio::fs::write(modules_dir.join("metadata.json"), "")
            .await
            .unwrap();

        let http = HttpClient::new().unwrap();
        let registry_url = format!("file://{}", dir.path().display());
        let result = fetch_releases(&registry_url, "rules_foo", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "parses metadata" — lib/modules/datasource/bazel/schema.spec.ts line 6
    #[test]
    fn parses_bazel_metadata_schema() {
        let json = r#"{
            "versions": ["1.0.0", "1.1.0", "1.2.0", "2.0.0"],
            "yanked_versions": {"1.1.0": "has a bug"},
            "homepage": "https://github.com/bazelbuild/rules_foo"
        }"#;
        let meta: BazelMetadata = serde_json::from_str(json).unwrap();
        assert_eq!(meta.versions.len(), 4);
        assert_eq!(meta.yanked_versions.len(), 1);
        assert!(meta.yanked_versions.contains_key("1.1.0"));
        assert_eq!(
            meta.homepage.as_deref(),
            Some("https://github.com/bazelbuild/rules_foo")
        );
    }
}
