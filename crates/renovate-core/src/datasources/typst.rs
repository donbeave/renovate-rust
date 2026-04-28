//! Typst package registry datasource.
//!
//! Fetches the Typst preview package index and returns the latest version
//! for a given package.
//!
//! Renovate reference: `lib/modules/datasource/typst/index.ts`
//!
//! ## API
//!
//! `GET https://packages.typst.org/preview/index.json`
//!
//! Returns a JSON array of `{ name, version, repository, updatedAt }` objects.
//! All entries are in the `preview` namespace.

use serde::Deserialize;

use crate::http::HttpClient;
use crate::versioning::semver_generic::semver_update_summary;

pub const TYPST_REGISTRY: &str = "https://packages.typst.org/preview/index.json";

#[derive(Debug, Deserialize)]
struct TypstEntry {
    name: String,
    version: String,
}

/// Update summary from the Typst datasource.
#[derive(Debug)]
pub struct TypstUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Error from the Typst datasource.
#[derive(Debug, thiserror::Error)]
pub enum TypstError {
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("JSON parse error: {0}")]
    Parse(String),
    #[error("package not found: {0}")]
    NotFound(String),
}

/// Fetch the latest version of `package_name` from the Typst preview registry.
///
/// `package_name` should be just the package name without namespace
/// (e.g. `cetz`, not `preview/cetz`).
pub async fn fetch_latest(
    http: &HttpClient,
    package_name: &str,
    current_value: &str,
) -> Result<TypstUpdateSummary, TypstError> {
    let body = http
        .get_raw_with_accept(TYPST_REGISTRY, "application/json")
        .await
        .map_err(|e| TypstError::Http(e.to_string()))?;

    let entries: Vec<TypstEntry> =
        serde_json::from_str(&body).map_err(|e| TypstError::Parse(e.to_string()))?;

    // Collect all versions for this package name.
    let versions: Vec<&str> = entries
        .iter()
        .filter(|e| e.name == package_name)
        .map(|e| e.version.as_str())
        .collect();

    if versions.is_empty() {
        return Err(TypstError::NotFound(package_name.to_owned()));
    }

    // Find latest by semver comparison.
    let latest = versions
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
    Ok(TypstUpdateSummary {
        current_value: current_value.to_owned(),
        latest,
        update_available: summary.update_available,
    })
}
