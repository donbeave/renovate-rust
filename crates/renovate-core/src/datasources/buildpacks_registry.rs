//! Cloud Native Buildpacks Registry datasource.
//!
//! Fetches available versions for buildpacks from the CNB registry.
//!
//! Renovate reference: `lib/modules/datasource/buildpacks-registry/index.ts`
//!
//! ## API
//!
//! `GET https://registry.buildpacks.io/api/v1/buildpacks/{namespace}/{name}`
//!
//! Response: `{ "versions": [{ "version": "1.0.0" }, ...], "latest": { "homepage": "..." } }`

use serde::Deserialize;

use crate::http::HttpClient;

pub const BUILDPACKS_REGISTRY_BASE: &str = "https://registry.buildpacks.io";

#[derive(Debug, Deserialize)]
struct RegistryVersion {
    version: String,
}

#[derive(Debug, Deserialize)]
struct RegistryResponse {
    #[serde(default)]
    versions: Vec<RegistryVersion>,
}

/// Update summary from the BuildpacksRegistry datasource.
#[derive(Debug)]
pub struct BuildpacksUpdateSummary {
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Error from the BuildpacksRegistry datasource.
#[derive(Debug, thiserror::Error)]
pub enum BuildpacksError {
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("package not found: {0}")]
    NotFound(String),
    #[error("JSON parse error: {0}")]
    Parse(String),
}

/// Fetch the latest version for a buildpack.
///
/// `package_name` is `"namespace/name"`, e.g. `"heroku/nodejs"`.
pub async fn fetch_latest(
    http: &HttpClient,
    package_name: &str,
    current_value: &str,
) -> Result<BuildpacksUpdateSummary, BuildpacksError> {
    let url = format!(
        "{}/api/v1/buildpacks/{}",
        BUILDPACKS_REGISTRY_BASE, package_name
    );

    let resp = http
        .get_retrying(&url)
        .await
        .map_err(|e| BuildpacksError::Http(e.to_string()))?;

    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(BuildpacksError::NotFound(package_name.to_owned()));
    }
    if !resp.status().is_success() {
        return Err(BuildpacksError::Http(format!(
            "HTTP {}",
            resp.status().as_u16()
        )));
    }

    let body = resp
        .text()
        .await
        .map_err(|e| BuildpacksError::Http(e.to_string()))?;
    let response: RegistryResponse =
        serde_json::from_str(&body).map_err(|e| BuildpacksError::Parse(e.to_string()))?;

    if response.versions.is_empty() {
        return Err(BuildpacksError::NotFound(package_name.to_owned()));
    }

    // Versions are returned newest-first by the registry.
    let latest = response.versions.first().map(|v| v.version.clone());
    let update_available = latest.as_deref() != Some(current_value);

    Ok(BuildpacksUpdateSummary {
        latest,
        update_available,
    })
}
