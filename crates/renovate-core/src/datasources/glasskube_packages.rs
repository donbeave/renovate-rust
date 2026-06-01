//! Glasskube packages datasource.
//!
//! Renovate reference: `lib/modules/datasource/glasskube-packages/index.ts`
//! Two-level fetch: versions.yaml → package.yaml for manifest references.

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const GLASSKUBE_REGISTRY_BASE: &str = "https://packages.dl.glasskube.dev/packages";
pub const DATASOURCE_ID: &str = "glasskube-packages";

#[derive(Debug, Error)]
pub enum GlasskubeError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

#[derive(Debug, Clone)]
pub struct GlasskubeRelease {
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct GlasskubeResult {
    pub releases: Vec<GlasskubeRelease>,
    pub tags: Option<std::collections::HashMap<String, String>>,
    pub source_url: Option<String>,
    pub homepage: Option<String>,
}

// ── YAML response types ────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct VersionsYaml {
    #[serde(rename = "latestVersion")]
    latest_version: String,
    versions: Vec<VersionEntry>,
}

#[derive(Debug, Deserialize)]
struct VersionEntry {
    version: String,
}

#[derive(Debug, Deserialize)]
struct PackageManifest {
    #[serde(default)]
    references: Vec<Reference>,
}

#[derive(Debug, Deserialize)]
struct Reference {
    label: String,
    url: String,
}

fn parse_yaml<T: serde::de::DeserializeOwned>(text: &str) -> Option<T> {
    serde_yaml::from_str(text).ok()
}

/// Fetch Glasskube package releases (versions.yaml + package.yaml manifest).
///
/// 5xx → `Err`. Empty/missing response → `Ok(None)`. 4xx → `Ok(None)`.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<GlasskubeResult>, GlasskubeError> {
    let base = registry_url.trim_end_matches('/');
    let versions_url = format!("{}/{}/versions.yaml", base, package_name);

    let versions_text = match http
        .get_raw_with_accept(&versions_url, "application/yaml")
        .await
    {
        Ok(v) => v,
        Err(crate::http::HttpError::Status { status, .. }) if status.is_client_error() => {
            return Ok(None);
        }
        Err(crate::http::HttpError::Request(_)) => return Ok(None),
        Err(e) => return Err(GlasskubeError::Http(e)),
    };

    if versions_text.trim().is_empty() {
        return Ok(None);
    }

    let Some(versions) = parse_yaml::<VersionsYaml>(&versions_text) else {
        return Ok(None);
    };

    let releases: Vec<GlasskubeRelease> = versions
        .versions
        .into_iter()
        .map(|v| GlasskubeRelease { version: v.version })
        .collect();

    let mut tags = std::collections::HashMap::new();
    tags.insert("latest".to_owned(), versions.latest_version.clone());

    let manifest_url = format!(
        "{}/{}/{}/package.yaml",
        base, package_name, versions.latest_version
    );

    let manifest_text = match http
        .get_raw_with_accept(&manifest_url, "application/yaml")
        .await
    {
        Ok(v) => v,
        Err(crate::http::HttpError::Status { status, .. }) if status.is_client_error() => {
            return Ok(None);
        }
        Err(crate::http::HttpError::Request(_)) => return Ok(None),
        Err(e) => return Err(GlasskubeError::Http(e)),
    };

    if manifest_text.trim().is_empty() {
        return Ok(None);
    }

    let Some(manifest) = parse_yaml::<PackageManifest>(&manifest_text) else {
        return Ok(None);
    };

    let mut source_url = None;
    let mut homepage = None;
    for reference in &manifest.references {
        match reference.label.to_lowercase().as_str() {
            "github" => source_url = Some(reference.url.clone()),
            "website" => homepage = Some(reference.url.clone()),
            _ => {}
        }
    }

    Ok(Some(GlasskubeResult {
        releases,
        tags: Some(tags),
        source_url,
        homepage,
    }))
}

/// Update summary from the Glasskube datasource (used by pipeline).
#[derive(Debug)]
pub struct GlasskubeUpdateSummary {
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Fetch the latest version for a Glasskube package (used by CLI pipeline).
pub async fn fetch_latest(
    http: &HttpClient,
    package_name: &str,
    current_value: &str,
) -> Result<GlasskubeUpdateSummary, GlasskubeError> {
    let result = fetch_releases(GLASSKUBE_REGISTRY_BASE, package_name, http).await?;
    let latest = result.and_then(|r| r.tags.and_then(|t| t.get("latest").cloned()));
    let update_available = latest
        .as_deref()
        .map(|l| l != current_value)
        .unwrap_or(false);
    Ok(GlasskubeUpdateSummary {
        latest,
        update_available,
    })
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    const VERSIONS_YAML: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/glasskube-packages/__fixtures__/versions.yaml"
    );
    const PACKAGE_YAML: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/glasskube-packages/__fixtures__/package.yaml"
    );
    const PACKAGE_NO_REFS_YAML: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/glasskube-packages/__fixtures__/package_no_references.yaml"
    );

    // Ported: "should handle error response on versions request" — datasource/glasskube-packages/index.spec.ts line 27
    #[tokio::test]
    async fn error_on_versions_request() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/cloudnative-pg/versions.yaml"))
            .respond_with(ResponseTemplate::new(500).set_body_string("internal server error"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "cloudnative-pg", &http).await;
        assert!(result.is_err());
    }

    // Ported: "should handle empty response on versions request" — datasource/glasskube-packages/index.spec.ts line 41
    #[tokio::test]
    async fn empty_versions_response() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/cloudnative-pg/versions.yaml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(""))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "cloudnative-pg", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "should handle error response on manifest request" — datasource/glasskube-packages/index.spec.ts line 54
    #[tokio::test]
    async fn error_on_manifest_request() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/cloudnative-pg/versions.yaml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(VERSIONS_YAML))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/cloudnative-pg/v1.23.1+1/package.yaml"))
            .respond_with(ResponseTemplate::new(500).set_body_string("internal server error"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "cloudnative-pg", &http).await;
        assert!(result.is_err());
    }

    // Ported: "should handle empty response on manifest request" — datasource/glasskube-packages/index.spec.ts line 72
    #[tokio::test]
    async fn empty_manifest_response() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/cloudnative-pg/versions.yaml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(VERSIONS_YAML))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/cloudnative-pg/v1.23.1+1/package.yaml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(""))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "cloudnative-pg", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "should handle package manifest without references" — datasource/glasskube-packages/index.spec.ts line 89
    #[tokio::test]
    async fn manifest_without_references() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/cloudnative-pg/versions.yaml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(VERSIONS_YAML))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/cloudnative-pg/v1.23.1+1/package.yaml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(PACKAGE_NO_REFS_YAML))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "cloudnative-pg", &http)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(result.releases.len(), 2);
        assert_eq!(result.releases[0].version, "v1.22.0+1");
        assert_eq!(result.releases[1].version, "v1.23.1+1");
        assert_eq!(
            result
                .tags
                .as_ref()
                .and_then(|t| t.get("latest"))
                .map(|s| s.as_str()),
            Some("v1.23.1+1")
        );
        assert!(result.source_url.is_none());
        assert!(result.homepage.is_none());
    }

    // Ported: "should handle package manifest with references and default url" — datasource/glasskube-packages/index.spec.ts line 110
    #[tokio::test]
    async fn manifest_with_references_default_url() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/cloudnative-pg/versions.yaml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(VERSIONS_YAML))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/cloudnative-pg/v1.23.1+1/package.yaml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(PACKAGE_YAML))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "cloudnative-pg", &http)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            result.source_url.as_deref(),
            Some("https://github.com/cloudnative-pg/cloudnative-pg")
        );
        assert_eq!(
            result.homepage.as_deref(),
            Some("https://cloudnative-pg.io/")
        );
        assert_eq!(result.releases.len(), 2);
        assert_eq!(
            result
                .tags
                .as_ref()
                .and_then(|t| t.get("latest"))
                .map(|s| s.as_str()),
            Some("v1.23.1+1")
        );
    }

    // Ported: "should handle package manifest with references and custom url" — datasource/glasskube-packages/index.spec.ts line 132
    #[tokio::test]
    async fn manifest_with_references_custom_url() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/cloudnative-pg/versions.yaml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(VERSIONS_YAML))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/cloudnative-pg/v1.23.1+1/package.yaml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(PACKAGE_YAML))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "cloudnative-pg", &http)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            result.source_url.as_deref(),
            Some("https://github.com/cloudnative-pg/cloudnative-pg")
        );
        assert_eq!(
            result.homepage.as_deref(),
            Some("https://cloudnative-pg.io/")
        );
    }
}
