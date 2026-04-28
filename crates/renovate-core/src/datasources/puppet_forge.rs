//! Puppet Forge datasource.
//!
//! Fetches module release metadata from the Puppet Forge REST API.
//!
//! Renovate reference: `lib/modules/datasource/puppet-forge/index.ts`
//!
//! ## API
//!
//! `GET https://forgeapi.puppet.com/v3/modules/{author}-{name}?exclude_fields=current_release`
//!
//! Response includes a `releases` array of `{ version: "x.y.z" }` objects.

use serde::Deserialize;

use crate::http::HttpClient;
use crate::versioning::semver_generic::semver_update_summary;

pub const PUPPET_FORGE_BASE: &str = "https://forgeapi.puppet.com";

#[derive(Debug, Deserialize)]
struct ForgeRelease {
    version: String,
}

#[derive(Debug, Deserialize)]
struct ForgeModule {
    releases: Vec<ForgeRelease>,
}

/// Update summary from the Puppet Forge datasource.
#[derive(Debug)]
pub struct PuppetForgeUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Error from the Puppet Forge datasource.
#[derive(Debug, thiserror::Error)]
pub enum PuppetForgeError {
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("module not found: {0}")]
    NotFound(String),
    #[error("JSON parse error: {0}")]
    Parse(String),
}

/// Fetch the latest version of `module_name` from the Puppet Forge.
///
/// `module_name` should be `author/name` or `author-name` format.
/// `registry_url` defaults to `PUPPET_FORGE_BASE` when empty.
pub async fn fetch_latest(
    http: &HttpClient,
    module_name: &str,
    current_value: &str,
    registry_url: &str,
) -> Result<PuppetForgeUpdateSummary, PuppetForgeError> {
    let base = if registry_url.is_empty() {
        PUPPET_FORGE_BASE
    } else {
        registry_url
    };

    // Normalize author/name → author-name for the API slug
    let slug = module_name.replace('/', "-");
    let url = format!(
        "{}/v3/modules/{}?exclude_fields=current_release",
        base, slug
    );

    let body = http
        .get_raw_with_accept(&url, "application/json")
        .await
        .map_err(|e| PuppetForgeError::Http(e.to_string()))?;

    let module: ForgeModule =
        serde_json::from_str(&body).map_err(|e| PuppetForgeError::Parse(e.to_string()))?;

    if module.releases.is_empty() {
        return Err(PuppetForgeError::NotFound(module_name.to_owned()));
    }

    // Find the latest semver release.
    let latest = module
        .releases
        .iter()
        .map(|r| r.version.as_str())
        .max_by(|a, b| {
            let s = semver_update_summary(a, Some(b));
            if s.update_available {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        })
        .map(|s| s.to_owned());

    let summary = semver_update_summary(current_value, latest.as_deref());
    Ok(PuppetForgeUpdateSummary {
        current_value: current_value.to_owned(),
        latest,
        update_available: summary.update_available,
    })
}
