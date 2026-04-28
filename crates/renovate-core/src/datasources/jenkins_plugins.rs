//! Jenkins plugins datasource.
//!
//! Fetches the latest plugin version from the Jenkins Update Center.
//!
//! Renovate reference:
//! - `lib/modules/datasource/jenkins-plugins/index.ts`
//! - API: `GET https://updates.jenkins.io/current/update-center.actual.json`
//!
//! The Update Center JSON has the shape:
//! `{"plugins": {"plugin-name": {"name": "...", "version": "1.2.3", ...}, ...}}`
//!
//! We fetch this file once and cache the result in memory for the process
//! lifetime (it is ~1.5 MB and refreshed weekly by Jenkins).

use std::collections::HashMap;
use std::sync::OnceLock;

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const UPDATE_CENTER_URL: &str = "https://updates.jenkins.io/current/update-center.actual.json";

/// Errors from fetching Jenkins plugin metadata.
#[derive(Debug, Error)]
pub enum JenkinsPluginsError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("plugin not found: {0}")]
    NotFound(String),
}

/// Update summary for a Jenkins plugin.
#[derive(Debug, Clone)]
pub struct JenkinsPluginUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

#[derive(Debug, Deserialize)]
struct UpdateCenterResponse {
    plugins: HashMap<String, PluginEntry>,
}

#[derive(Debug, Deserialize)]
struct PluginEntry {
    version: Option<String>,
}

static CACHED_VERSIONS: OnceLock<HashMap<String, String>> = OnceLock::new();

async fn load_versions(
    http: &HttpClient,
) -> Result<&'static HashMap<String, String>, JenkinsPluginsError> {
    if let Some(map) = CACHED_VERSIONS.get() {
        return Ok(map);
    }
    let resp: UpdateCenterResponse = http.get_json(UPDATE_CENTER_URL).await?;
    let map: HashMap<String, String> = resp
        .plugins
        .into_iter()
        .filter_map(|(name, entry)| entry.version.map(|v| (name, v)))
        .collect();
    // If another task already set it, return that value.
    let _ = CACHED_VERSIONS.set(map);
    Ok(CACHED_VERSIONS.get().unwrap())
}

/// Fetch the latest version for a single Jenkins plugin.
pub async fn fetch_latest(
    http: &HttpClient,
    plugin_name: &str,
    current_value: &str,
) -> Result<JenkinsPluginUpdateSummary, JenkinsPluginsError> {
    let versions = load_versions(http).await?;
    let latest = versions.get(plugin_name).cloned();
    let update_available = match &latest {
        Some(l) => l != current_value,
        None => false,
    };
    Ok(JenkinsPluginUpdateSummary {
        current_value: current_value.to_owned(),
        latest,
        update_available,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_update_center_json() {
        let json =
            r#"{"plugins": {"git": {"version": "5.2.1"}, "matrix-auth": {"version": "3.1.0"}}}"#;
        let resp: UpdateCenterResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.plugins["git"].version.as_deref(), Some("5.2.1"));
        assert_eq!(
            resp.plugins["matrix-auth"].version.as_deref(),
            Some("3.1.0")
        );
    }
}
