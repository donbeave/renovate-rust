//! Jenkins plugins datasource.
//!
//! Renovate reference: `lib/modules/datasource/jenkins-plugins/index.ts`
//! Two-level fetch:
//!   1. `GET {registry}/current/update-center.actual.json` — plugin info + sourceUrl
//!   2. `GET {registry}/current/plugin-versions.json` — all version entries

use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY: &str = "https://updates.jenkins.io";
pub const DATASOURCE_ID: &str = "jenkins-plugins";

#[derive(Debug, Error)]
pub enum JenkinsPluginsError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

#[derive(Debug, Clone)]
pub struct JenkinsRelease {
    pub version: String,
    pub download_url: Option<String>,
    pub release_timestamp: Option<String>,
}

#[derive(Debug, Clone)]
pub struct JenkinsResult {
    pub releases: Vec<JenkinsRelease>,
    pub source_url: Option<String>,
}

// ── API types ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct InfoResponse {
    #[serde(default)]
    plugins: HashMap<String, InfoPlugin>,
}

#[derive(Debug, Deserialize)]
struct InfoPlugin {
    #[serde(rename = "scm")]
    source_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct VersionsResponse {
    #[serde(default)]
    plugins: HashMap<String, HashMap<String, VersionEntry>>,
}

#[derive(Debug, Deserialize)]
struct VersionEntry {
    url: Option<String>,
    #[serde(rename = "buildDate")]
    build_date: Option<String>,
    #[serde(rename = "releaseTimestamp")]
    release_timestamp: Option<String>,
}

/// Parse "Jan 02, 2020" → "2020-01-02T00:00:00.000Z".
fn parse_build_date(s: &str) -> Option<String> {
    NaiveDate::parse_from_str(s, "%b %d, %Y")
        .ok()
        .map(|d| format!("{}T00:00:00.000Z", d.format("%Y-%m-%d")))
}

/// Normalize ISO timestamp to exactly 3ms digits: "2020-05-13T00:11:40.00Z" → "2020-05-13T00:11:40.000Z".
fn normalize_timestamp(s: &str) -> Option<String> {
    DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.with_timezone(&Utc).format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string())
}

fn resolve_timestamp(entry: &VersionEntry) -> Option<String> {
    entry
        .release_timestamp
        .as_deref()
        .and_then(normalize_timestamp)
        .or_else(|| entry.build_date.as_deref().and_then(parse_build_date))
}

/// Fetch Jenkins plugin releases.
///
/// Package not in info → `Ok(None)`. Empty info response → `Ok(None)`.
/// HTTP errors propagate as `Err`.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<JenkinsResult>, JenkinsPluginsError> {
    let base = registry_url.trim_end_matches('/');
    let info_url = format!("{base}/current/update-center.actual.json");
    let versions_url = format!("{base}/current/plugin-versions.json");

    let info: InfoResponse = http.get_json(&info_url).await?;

    let plugin_info = match info.plugins.get(package_name) {
        Some(p) => p,
        None => return Ok(None),
    };

    let source_url = plugin_info.source_url.clone();

    let versions: VersionsResponse = http.get_json(&versions_url).await?;

    let plugin_versions = versions.plugins.get(package_name);

    let mut releases: Vec<JenkinsRelease> = match plugin_versions {
        None => vec![],
        Some(vers) => vers
            .iter()
            .map(|(version, entry)| JenkinsRelease {
                version: version.clone(),
                download_url: entry.url.clone(),
                release_timestamp: resolve_timestamp(entry),
            })
            .collect(),
    };

    // None timestamps sort first (oldest/unknown), then ascending by timestamp.
    releases.sort_by(|a, b| a.release_timestamp.cmp(&b.release_timestamp).then(a.version.cmp(&b.version)));

    Ok(Some(JenkinsResult { releases, source_url }))
}

/// Update summary used by pipeline.
#[derive(Debug, Clone)]
pub struct JenkinsPluginUpdateSummary {
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Fetch latest plugin version (pipeline helper).
pub async fn fetch_latest(
    http: &HttpClient,
    plugin_name: &str,
    current_value: &str,
) -> Result<JenkinsPluginUpdateSummary, JenkinsPluginsError> {
    let result = fetch_releases(DEFAULT_REGISTRY, plugin_name, http).await?;
    let latest = result.and_then(|r| r.releases.into_iter().last().map(|rel| rel.version));
    let update_available = latest.as_deref().map(|l| l != current_value).unwrap_or(false);
    Ok(JenkinsPluginUpdateSummary { latest, update_available })
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    const INFO_BODY: &str = r#"{
        "plugins": {
            "foobar": {
                "name": "foobar",
                "scm": "https://source-url.example.com"
            }
        }
    }"#;

    const VERSIONS_BODY: &str = r#"{
        "plugins": {
            "foobar": {
                "1.0.0": {
                    "version": "1.0.0",
                    "url": "https://download.example.com"
                },
                "2.0.0": {
                    "version": "2.0.0",
                    "url": "https://download.example.com",
                    "buildDate": "Jan 02, 2020"
                },
                "3.0.0": {
                    "version": "3.0.0",
                    "url": "https://download.example.com",
                    "releaseTimestamp": "2020-05-13T00:11:40.00Z",
                    "requiredCore": "2.164.3"
                }
            }
        }
    }"#;

    // Ported: "returns null for a package miss" — datasource/jenkins-plugins/index.spec.ts line 57
    #[tokio::test]
    async fn returns_null_for_package_miss() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/current/update-center.actual.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(INFO_BODY))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "non-existing", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns package releases for a hit for info and releases" — datasource/jenkins-plugins/index.spec.ts line 69
    #[tokio::test]
    async fn returns_releases_for_info_and_releases_hit() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/current/update-center.actual.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(INFO_BODY))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/current/plugin-versions.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(VERSIONS_BODY))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "foobar", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.source_url.as_deref(), Some("https://source-url.example.com"));
        assert_eq!(result.releases.len(), 3);

        assert_eq!(result.releases[0].version, "1.0.0");
        assert_eq!(result.releases[0].download_url.as_deref(), Some("https://download.example.com"));
        assert!(result.releases[0].release_timestamp.is_none());

        assert_eq!(result.releases[1].version, "2.0.0");
        assert_eq!(result.releases[1].release_timestamp.as_deref(), Some("2020-01-02T00:00:00.000Z"));

        assert_eq!(result.releases[2].version, "3.0.0");
        assert_eq!(result.releases[2].release_timestamp.as_deref(), Some("2020-05-13T00:11:40.000Z"));
    }

    // Ported: "returns package releases for a hit for info and miss for releases" — datasource/jenkins-plugins/index.spec.ts line 104
    #[tokio::test]
    async fn returns_empty_releases_for_info_hit_versions_miss() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/current/update-center.actual.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(INFO_BODY))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/current/plugin-versions.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "foobar", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.source_url.as_deref(), Some("https://source-url.example.com"));
        assert!(result.releases.is_empty());
    }

    // Ported: "returns null empty response" — datasource/jenkins-plugins/index.spec.ts line 122
    #[tokio::test]
    async fn returns_null_for_empty_info_response() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/current/update-center.actual.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "foobar", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns package releases from a custom registry" — datasource/jenkins-plugins/index.spec.ts line 131
    #[tokio::test]
    async fn returns_releases_from_custom_registry() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/current/update-center.actual.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(INFO_BODY))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/current/plugin-versions.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(VERSIONS_BODY))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "foobar", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.source_url.as_deref(), Some("https://source-url.example.com"));
        assert_eq!(result.releases.len(), 3);
        assert_eq!(result.releases[0].version, "1.0.0");
        assert_eq!(result.releases[1].version, "2.0.0");
        assert_eq!(result.releases[1].release_timestamp.as_deref(), Some("2020-01-02T00:00:00.000Z"));
        assert_eq!(result.releases[2].version, "3.0.0");
        assert_eq!(result.releases[2].release_timestamp.as_deref(), Some("2020-05-13T00:11:40.000Z"));
    }
}
