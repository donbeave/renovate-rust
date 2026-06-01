//! JSR (JavaScript Registry) datasource.
//!
//! Renovate reference: `lib/modules/datasource/jsr/index.ts`
//! API: `GET https://jsr.io/@scope/name/meta.json`

use std::collections::HashMap;

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const JSR_REGISTRY: &str = "https://jsr.io/";
pub const DATASOURCE_ID: &str = "jsr";

/// Timestamp used as fallback for versions without createdAt (JSR API behavior).
/// Versions published before 2025-09-18 may not have createdAt.
pub const MINIMUM_RELEASE_TIMESTAMP: &str = "2025-09-18T00:00:00.000Z";

#[derive(Debug, Error)]
pub enum JsrError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(String),
}

#[derive(Debug, Clone)]
pub struct JsrRelease {
    pub version: String,
    pub release_timestamp: String,
    pub is_deprecated: bool,
    pub is_latest: bool,
}

#[derive(Debug, Clone)]
pub struct JsrResult {
    pub releases: Vec<JsrRelease>,
    pub homepage: Option<String>,
}

// ── Package name validation ────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct JsrPackageName {
    pub scope: String,
    pub name: String,
}

/// Extract and validate a JSR package name in `@scope/name` form.
pub fn extract_jsr_package_name(package_name: &str) -> Option<JsrPackageName> {
    let without_at = package_name.strip_prefix('@')?;
    let parts: Vec<&str> = without_at.splitn(3, '/').collect();
    if parts.len() != 2 {
        return None;
    }
    let scope = parts[0];
    let name = parts[1];
    parse_jsr_scope_name(scope)?;
    parse_jsr_package_name(name)?;
    Some(JsrPackageName {
        scope: scope.to_owned(),
        name: name.to_owned(),
    })
}

fn parse_jsr_scope_name(name: &str) -> Option<&str> {
    if name.len() > 100 || name.len() < 3 {
        return None;
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return None;
    }
    Some(name)
}

fn parse_jsr_package_name(name: &str) -> Option<&str> {
    if name.starts_with('@') || name.starts_with('-') {
        return None;
    }
    if name.len() > 58 {
        return None;
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return None;
    }
    Some(name)
}

// ── API response types ─────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct JsrMeta {
    latest: Option<String>,
    #[serde(default)]
    versions: HashMap<String, JsrVersionInfo>,
}

#[derive(Debug, Deserialize)]
struct JsrVersionInfo {
    #[serde(rename = "createdAt")]
    created_at: Option<String>,
    #[serde(default)]
    yanked: bool,
}

/// Fetch JSR package releases.
///
/// Invalid package name → `Ok(None)`.
/// No versions → `Ok(None)`.
/// 4xx/network error → `Err`.
/// Parse error → `Err`.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<JsrResult>, JsrError> {
    if extract_jsr_package_name(package_name).is_none() {
        return Ok(None);
    }

    let base = registry_url.trim_end_matches('/');
    let url = format!("{}/{}/meta.json", base, package_name);

    let meta: JsrMeta = match http.get_json(&url).await {
        Ok(v) => v,
        Err(e) => return Err(JsrError::Http(e)),
    };

    if meta.versions.is_empty() {
        return Ok(None);
    }

    let mut releases: Vec<JsrRelease> = meta
        .versions
        .iter()
        .map(|(version, info)| JsrRelease {
            version: version.clone(),
            release_timestamp: info
                .created_at
                .clone()
                .unwrap_or_else(|| MINIMUM_RELEASE_TIMESTAMP.to_owned()),
            is_deprecated: info.yanked,
            is_latest: meta.latest.as_deref() == Some(version),
        })
        .collect();

    releases.sort_by(|a, b| a.version.cmp(&b.version));

    let homepage = Some(format!("https://jsr.io/{}", package_name));

    Ok(Some(JsrResult { releases, homepage }))
}

/// Summary used by pipeline.
#[derive(Debug, Clone)]
pub struct JsrUpdateSummary {
    pub latest: Option<String>,
    pub update_available: bool,
    pub versions: Vec<String>,
}

/// Fetch the latest non-yanked version (pipeline helper).
pub async fn fetch_latest(
    package_name: &str,
    current_value: &str,
    http: &HttpClient,
) -> Result<JsrUpdateSummary, JsrError> {
    let result = fetch_releases(JSR_REGISTRY, package_name, http).await?;
    match result {
        None => Ok(JsrUpdateSummary {
            latest: None,
            update_available: false,
            versions: vec![],
        }),
        Some(r) => {
            let latest = r
                .releases
                .iter()
                .find(|rel| rel.is_latest)
                .map(|rel| rel.version.clone());
            let versions: Vec<String> = r
                .releases
                .iter()
                .filter(|rel| !rel.is_deprecated)
                .map(|rel| rel.version.clone())
                .collect();
            let update_available = latest
                .as_deref()
                .map(|l| l != current_value)
                .unwrap_or(false);
            Ok(JsrUpdateSummary {
                latest,
                update_available,
                versions,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    // ── util tests ────────────────────────────────────────────────────────────

    // Ported: "should extract package name" — datasource/jsr/util.spec.ts line 4
    #[test]
    fn extract_package_name_valid() {
        let result = extract_jsr_package_name("@scope/package-name").unwrap();
        assert_eq!(result.scope, "scope");
        assert_eq!(result.name, "package-name");
    }

    // Ported: "should return null for invalid name" — datasource/jsr/util.spec.ts line 12
    #[test]
    fn extract_null_for_invalid_path() {
        assert!(extract_jsr_package_name("@invalid/package/name").is_none());
    }

    // Ported: "should return null for below scope min length" — datasource/jsr/util.spec.ts line 17
    #[test]
    fn extract_null_for_short_scope() {
        assert!(extract_jsr_package_name("@sc/packagename").is_none());
    }

    // Ported: "should return null for exceed scope max length" — datasource/jsr/util.spec.ts line 22
    #[test]
    fn extract_null_for_long_scope() {
        let long_scope = format!("@{}/packagename", "a".repeat(101));
        assert!(extract_jsr_package_name(&long_scope).is_none());
    }

    // Ported: "should return null for invalid scope name" — datasource/jsr/util.spec.ts line 27
    #[test]
    fn extract_null_for_non_ascii_scope() {
        assert!(extract_jsr_package_name("@🦕🦕🦕/package-name").is_none());
    }

    // Ported: "should return null for invalid package name starting with @" — datasource/jsr/util.spec.ts line 32
    #[test]
    fn extract_null_for_package_starting_with_at() {
        assert!(extract_jsr_package_name("@scope/@package-name").is_none());
    }

    // Ported: "should return null for exceed package max length" — datasource/jsr/util.spec.ts line 37
    #[test]
    fn extract_null_for_long_package_name() {
        let long_name = format!("@scope/{}", "a".repeat(59));
        assert!(extract_jsr_package_name(&long_name).is_none());
    }

    // Ported: "should return null for invalid package name" — datasource/jsr/util.spec.ts line 42
    #[test]
    fn extract_null_for_uppercase_package() {
        assert!(extract_jsr_package_name("@scope/PACKAGE-NAME").is_none());
    }

    // Ported: "should return null for invalid package name starting with -" — datasource/jsr/util.spec.ts line 47
    #[test]
    fn extract_null_for_package_starting_with_dash() {
        assert!(extract_jsr_package_name("@scope/-package-name").is_none());
    }

    // ── index tests ───────────────────────────────────────────────────────────

    // Ported: "should return null for invalid package name" — datasource/jsr/index.spec.ts line 24
    #[tokio::test]
    async fn returns_null_for_invalid_package_name() {
        let http = HttpClient::new().unwrap();
        let result = fetch_releases("https://jsr.io/", "invalid", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "should return null for no versions" — datasource/jsr/index.spec.ts line 32
    #[tokio::test]
    async fn returns_null_for_no_versions() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/@scope/package-name/meta.json"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string(r#"{"latest":"0.0.2","versions":{}}"#),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "@scope/package-name", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "should fetch package info from jsr" — datasource/jsr/index.spec.ts line 46
    #[tokio::test]
    async fn fetches_package_info() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/@scope/package-name/meta.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"{"latest":"0.0.2","versions":{"0.0.1":{},"0.0.2":{"createdAt":"2025-11-15T00:00:00.000Z"}}}"#,
            ))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "@scope/package-name", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            result.homepage.as_deref(),
            Some("https://jsr.io/@scope/package-name")
        );
        assert_eq!(result.releases.len(), 2);

        // releases sorted by version: 0.0.1, 0.0.2
        let r0 = &result.releases[0];
        assert_eq!(r0.version, "0.0.1");
        assert_eq!(r0.release_timestamp, MINIMUM_RELEASE_TIMESTAMP);
        assert!(!r0.is_latest);

        let r1 = &result.releases[1];
        assert_eq!(r1.version, "0.0.2");
        assert_eq!(r1.release_timestamp, "2025-11-15T00:00:00.000Z");
        assert!(r1.is_latest);
    }

    // Ported: "contains yanked versions" — datasource/jsr/index.spec.ts line 74
    #[tokio::test]
    async fn contains_yanked_versions() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/@scope/package-name/meta.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"{"latest":"0.0.2","versions":{"0.0.1":{"yanked":true},"0.0.2":{"createdAt":"2025-11-15T00:00:00.000Z"}}}"#,
            ))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "@scope/package-name", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 2);
        let r0 = &result.releases[0];
        assert_eq!(r0.version, "0.0.1");
        assert!(r0.is_deprecated);

        let r1 = &result.releases[1];
        assert_eq!(r1.version, "0.0.2");
        assert!(!r1.is_deprecated);
        assert!(r1.is_latest);
    }

    // Ported: "should return null if lookup fails" — datasource/jsr/index.spec.ts line 102
    #[tokio::test]
    async fn throws_for_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/@scope/package-name/meta.json"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result =
            fetch_releases(&format!("{}/", server.uri()), "@scope/package-name", &http).await;
        assert!(result.is_err());
    }

    // Ported: "should throw error for unparseable" — datasource/jsr/index.spec.ts line 115
    #[tokio::test]
    async fn throws_for_unparseable() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/@scope/package-name/meta.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string("oops"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result =
            fetch_releases(&format!("{}/", server.uri()), "@scope/package-name", &http).await;
        assert!(result.is_err());
    }
}
