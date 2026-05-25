//! Unity3D Packages datasource.
//!
//! Fetches package releases from the Unity Package Manager registry.
//!
//! Renovate reference: `lib/modules/datasource/unity3d-packages/index.ts`
//! API: `GET <registryUrl>/<packageName>`

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY_URL: &str = "https://packages.unity.com";
pub const DATASOURCE_ID: &str = "unity3d-packages";

/// Errors from the unity3d-packages datasource.
#[derive(Debug, Error)]
pub enum Unity3dPackagesError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

#[derive(Debug, Deserialize)]
struct Upm {
    changelog: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Repository {
    url: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PackageVersion {
    version: String,
    documentation_url: Option<String>,
    repository: Option<Repository>,
    #[serde(rename = "_upm")]
    upm: Option<Upm>,
}

#[derive(Debug, Deserialize)]
struct PackageReleasesJson {
    versions: serde_json::Map<String, serde_json::Value>,
    time: serde_json::Map<String, serde_json::Value>,
}

/// One Unity3D package release.
#[derive(Debug, Clone, PartialEq)]
pub struct Unity3dPackageRelease {
    pub version: String,
    pub release_timestamp: Option<String>,
    pub changelog_content: Option<String>,
    pub changelog_url: Option<String>,
    pub is_stable: bool,
    pub registry_url: String,
}

/// Result of a `fetch_releases` call.
#[derive(Debug, Clone)]
pub struct Unity3dPackagesResult {
    pub registry_url: String,
    pub homepage: Option<String>,
    pub source_url: Option<String>,
    pub releases: Vec<Unity3dPackageRelease>,
}

/// Check if a Unity3D package version is stable.
///
/// Unstable labels: `exp.*`, `pre.*`, `preview.*`
pub fn is_stable_version(version: &str) -> bool {
    let re = regex::Regex::new(r"^\d+\.\d+\.\d+-?(.*)").unwrap();
    if let Some(caps) = re.captures(version) {
        let label = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let unstable = regex::Regex::new(r"^(exp\.|pre\.|preview\.)").unwrap();
        !unstable.is_match(label)
    } else {
        true
    }
}

/// Derive the changelog URL from a documentation URL.
///
/// Replaces `manual/index.html` with `changelog/CHANGELOG.html`.
fn changelog_url_from_docs(docs_url: &str) -> Option<String> {
    if docs_url.contains("manual/index.html") {
        Some(docs_url.replace("manual/index.html", "changelog/CHANGELOG.html"))
    } else {
        None
    }
}

/// Fetch Unity3D package releases.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Unity3dPackagesResult, Unity3dPackagesError> {
    let url = format!(
        "{}/{}",
        registry_url.trim_end_matches('/'),
        package_name
    );

    let body: PackageReleasesJson = http.get_json(&url).await?;

    let using_default_registry = registry_url.trim_end_matches('/') == DEFAULT_REGISTRY_URL.trim_end_matches('/');

    let mut releases = Vec::new();
    let mut homepage: Option<String> = None;
    let mut source_url: Option<String> = None;
    let mut first = true;

    for (_key, val) in &body.versions {
        let pkg_ver: PackageVersion = match serde_json::from_value(val.clone()) {
            Ok(v) => v,
            Err(_) => continue,
        };

        if first {
            homepage = pkg_ver.documentation_url.clone();
            source_url = pkg_ver
                .repository
                .as_ref()
                .and_then(|r| r.url.clone());
            first = false;
        }

        let release_timestamp = body
            .time
            .get(&pkg_ver.version)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let changelog_content = pkg_ver
            .upm
            .as_ref()
            .and_then(|u| u.changelog.clone())
            .filter(|s| !s.is_empty());

        let changelog_url = if using_default_registry {
            pkg_ver
                .documentation_url
                .as_deref()
                .and_then(changelog_url_from_docs)
        } else {
            None
        };

        releases.push(Unity3dPackageRelease {
            is_stable: is_stable_version(&pkg_ver.version),
            registry_url: registry_url.trim_end_matches('/').to_string(),
            release_timestamp,
            changelog_content,
            changelog_url,
            version: pkg_ver.version,
        });
    }

    Ok(Unity3dPackagesResult {
        registry_url: registry_url.trim_end_matches('/').to_string(),
        homepage,
        source_url,
        releases,
    })
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    // Ported: "package with no versions" — unity3d-packages/index.spec.ts line 6
    #[tokio::test]
    async fn package_with_no_versions() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/com.unity.xr.openxr"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"{ "versions": {}, "time": {} }"#,
            ))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "com.unity.xr.openxr", &http)
            .await
            .unwrap();

        assert_eq!(result.homepage, None);
        assert_eq!(result.registry_url, server.uri().trim_end_matches('/'));
        assert!(result.releases.is_empty());
    }

    // Ported: "package with no documentationUrl" — unity3d-packages/index.spec.ts line 31
    #[tokio::test]
    async fn package_with_no_documentation_url() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/com.unity.xr.openxr"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "versions": {
                    "1.14.2": { "version": "1.14.2" }
                },
                "time": {
                    "1.14.2": "2025-03-27T10:54:45.412Z"
                }
            })))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "com.unity.xr.openxr", &http)
            .await
            .unwrap();

        assert_eq!(result.releases.len(), 1);
        let r = &result.releases[0];
        assert_eq!(r.version, "1.14.2");
        assert_eq!(r.release_timestamp.as_deref(), Some("2025-03-27T10:54:45.412Z"));
        assert!(r.is_stable);
        assert_eq!(r.changelog_url, None);
        assert_eq!(result.registry_url, server.uri().trim_end_matches('/'));
    }

    // Ported: "package from a custom registry" — unity3d-packages/index.spec.ts line 70
    #[tokio::test]
    async fn package_from_custom_registry() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/com.unity.xr.openxr"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "versions": {
                    "1.14.2": {
                        "documentationUrl": "https://docs.unity3d.com/Packages/com.unity.xr.openxr@1.14/manual/index.html",
                        "version": "1.14.2"
                    }
                },
                "time": {
                    "1.14.2": "2025-03-27T10:54:45.412Z"
                }
            })))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "com.unity.xr.openxr", &http)
            .await
            .unwrap();

        // Custom registry: changelogUrl should be None (not using default registry)
        assert_eq!(result.releases[0].changelog_url, None);
        assert_eq!(
            result.homepage.as_deref(),
            Some("https://docs.unity3d.com/Packages/com.unity.xr.openxr@1.14/manual/index.html")
        );
    }

    // Ported: "package with changelog content and url" — unity3d-packages/index.spec.ts line 112
    #[tokio::test]
    async fn package_with_changelog_content_and_url() {
        // TypeScript test uses DEFAULT_REGISTRY_URL (https://packages.unity.com).
        // We test the same behavior via a mock server treated as the default registry:
        // fetch_releases compares registry_url to DEFAULT_REGISTRY_URL for changelog derivation.
        // Since we can't point the mock server at the real host, we test the two aspects separately:
        // 1. HTTP + parsing behavior (via mock at custom URL, so changelogUrl = None)
        // 2. changelog_url_from_docs and is_stable_version logic (unit tests)

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/com.unity.xr.openxr"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "versions": {
                    "1.14.3": {
                        "_upm": { "changelog": "### Fixed\n\n* Fixed Multiview Render Regions feature regression." },
                        "documentationUrl": "https://docs.unity3d.com/Packages/com.unity.xr.openxr@1.14/manual/index.html",
                        "version": "1.14.3"
                    },
                    "1.12.0-exp.1": {
                        "_upm": {},
                        "documentationUrl": "https://docs.unity3d.com/Packages/com.unity.xr.openxr@1.12/manual/index.html",
                        "version": "1.12.0-exp.1"
                    },
                    "1.0.0-pre.1": { "version": "1.0.0-pre.1" },
                    "0.1.2-preview.2": { "version": "0.1.2-preview.2" }
                },
                "time": {
                    "1.14.3": "2025-04-18T18:06:12.036Z",
                    "1.12.0-exp.1": "2024-07-03T15:24:28.000Z",
                    "1.0.0-pre.1": "2021-02-11T19:26:19.000Z",
                    "0.1.2-preview.2": "2021-01-05T17:57:41.000Z"
                }
            })))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "com.unity.xr.openxr", &http)
            .await
            .unwrap();

        assert_eq!(result.releases.len(), 4);

        // Stability checks
        assert!(result.releases.iter().find(|r| r.version == "1.14.3").unwrap().is_stable);
        assert!(!result.releases.iter().find(|r| r.version == "1.12.0-exp.1").unwrap().is_stable);
        assert!(!result.releases.iter().find(|r| r.version == "1.0.0-pre.1").unwrap().is_stable);
        assert!(!result.releases.iter().find(|r| r.version == "0.1.2-preview.2").unwrap().is_stable);

        // Timestamps
        let r143 = result.releases.iter().find(|r| r.version == "1.14.3").unwrap();
        assert_eq!(r143.release_timestamp.as_deref(), Some("2025-04-18T18:06:12.036Z"));

        // Changelog content
        assert_eq!(
            r143.changelog_content.as_deref(),
            Some("### Fixed\n\n* Fixed Multiview Render Regions feature regression.")
        );
        assert_eq!(
            result.releases.iter().find(|r| r.version == "1.12.0-exp.1").unwrap().changelog_content,
            None
        );

        // changelogUrl derivation (unit test for the function)
        assert_eq!(
            changelog_url_from_docs("https://docs.unity3d.com/Packages/com.unity.xr.openxr@1.14/manual/index.html"),
            Some("https://docs.unity3d.com/Packages/com.unity.xr.openxr@1.14/changelog/CHANGELOG.html".to_string())
        );
        assert_eq!(
            changelog_url_from_docs("https://docs.unity3d.com/Packages/com.unity.xr.openxr@1.12/manual/index.html"),
            Some("https://docs.unity3d.com/Packages/com.unity.xr.openxr@1.12/changelog/CHANGELOG.html".to_string())
        );

        // homepage = first version's documentationUrl
        assert_eq!(
            result.homepage.as_deref(),
            Some("https://docs.unity3d.com/Packages/com.unity.xr.openxr@1.14/manual/index.html")
        );
    }

    // Ported: "package with repository" — unity3d-packages/index.spec.ts line 200
    #[tokio::test]
    async fn package_with_repository() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/com.unity.xr.openxr"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "versions": {
                    "1.14.3": {
                        "repository": { "url": "https://github.cds.internal.unity3d.com/unity/xr.sdk.openxr.git" },
                        "version": "1.14.3"
                    },
                    "1.12.0-exp.1": { "version": "1.12.0-exp.1" }
                },
                "time": {
                    "1.14.3": "2025-04-18T18:06:12.036Z",
                    "1.12.0-exp.1": "2024-07-03T15:24:28.000Z"
                }
            })))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "com.unity.xr.openxr", &http)
            .await
            .unwrap();

        assert_eq!(result.releases.len(), 2);
        assert_eq!(
            result.source_url.as_deref(),
            Some("https://github.cds.internal.unity3d.com/unity/xr.sdk.openxr.git")
        );
        let r143 = result.releases.iter().find(|r| r.version == "1.14.3").unwrap();
        assert!(r143.is_stable);
        assert_eq!(r143.release_timestamp.as_deref(), Some("2025-04-18T18:06:12.036Z"));

        let r112 = result.releases.iter().find(|r| r.version == "1.12.0-exp.1").unwrap();
        assert!(!r112.is_stable);
        assert_eq!(r112.release_timestamp.as_deref(), Some("2024-07-03T15:24:28.000Z"));
    }
}
