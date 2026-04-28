//! Bazel Central Registry datasource.
//!
//! Fetches module metadata from the Bazel Central Registry on GitHub.
//!
//! Renovate reference: `lib/modules/datasource/bazel/index.ts`
//!
//! ## API
//!
//! `GET https://raw.githubusercontent.com/bazelbuild/bazel-central-registry/main/modules/{name}/metadata.json`
//!
//! Response: `{ "versions": ["0.1.0", "0.2.0", ...], "yanked_versions": {...} }`

use serde::Deserialize;
use std::collections::HashMap;

use crate::http::HttpClient;
use crate::versioning::semver_generic::semver_update_summary;

pub const BAZEL_CENTRAL_REGISTRY: &str =
    "https://raw.githubusercontent.com/bazelbuild/bazel-central-registry/main";

#[derive(Debug, Deserialize)]
struct BazelMetadata {
    versions: Vec<String>,
    #[serde(default)]
    yanked_versions: HashMap<String, String>,
}

/// Update summary from the Bazel Central Registry.
#[derive(Debug)]
pub struct BazelUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Error from the Bazel datasource.
#[derive(Debug, thiserror::Error)]
pub enum BazelError {
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("module not found: {0}")]
    NotFound(String),
    #[error("JSON parse error: {0}")]
    Parse(String),
}

/// Fetch the latest version of `module_name` from the Bazel Central Registry.
pub async fn fetch_latest(
    http: &HttpClient,
    module_name: &str,
    current_value: &str,
) -> Result<BazelUpdateSummary, BazelError> {
    let url = format!(
        "{}/modules/{}/metadata.json",
        BAZEL_CENTRAL_REGISTRY, module_name
    );

    let body = http
        .get_raw_with_accept(&url, "application/json")
        .await
        .map_err(|e| BazelError::Http(e.to_string()))?;

    let meta: BazelMetadata =
        serde_json::from_str(&body).map_err(|e| BazelError::Parse(e.to_string()))?;

    if meta.versions.is_empty() {
        return Err(BazelError::NotFound(module_name.to_owned()));
    }

    // Exclude yanked versions.
    let available: Vec<&str> = meta
        .versions
        .iter()
        .filter(|v| !meta.yanked_versions.contains_key(*v))
        .map(|v| v.as_str())
        .collect();

    if available.is_empty() {
        return Err(BazelError::NotFound(module_name.to_owned()));
    }

    // Find latest by semver comparison.
    let latest = available
        .iter()
        .copied()
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
    Ok(BazelUpdateSummary {
        current_value: current_value.to_owned(),
        latest,
        update_available: summary.update_available,
    })
}
