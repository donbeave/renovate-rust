//! Conda / Anaconda package datasource.
//!
//! Queries the Anaconda.org API to fetch available versions for a conda
//! package in a given channel (e.g. `conda-forge`).
//!
//! Renovate reference:
//! - `lib/modules/datasource/conda/index.ts`
//! - Default registry: `https://api.anaconda.org/package/`
//!
//! ## Request format
//!
//! `GET https://api.anaconda.org/package/{channel}/{package}`
//!
//! ## Response (relevant fields)
//!
//! ```json
//! { "versions": ["24.1.0", "24.5.0", "24.6.1"], ... }
//! ```

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

/// Default Anaconda registry base.
pub const ANACONDA_REGISTRY: &str = "https://api.anaconda.org/package/";
/// Default conda channel used when none is specified.
pub const DEFAULT_CHANNEL: &str = "conda-forge";

/// Errors from conda datasource lookups.
#[derive(Debug, Error)]
pub enum CondaError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(reqwest::Error),
    #[error("package not found")]
    NotFound,
}

#[derive(Debug, Deserialize)]
struct AnacondaPackage {
    versions: Vec<String>,
}

/// Summary of a conda package version lookup.
#[derive(Debug, Clone)]
pub struct CondaUpdateSummary {
    /// All available versions, oldest first.
    pub versions: Vec<String>,
    /// The latest available version (last in the list).
    pub latest: Option<String>,
    /// True when `latest` != `current_value`.
    pub update_available: bool,
}

/// Fetch available conda package versions from the Anaconda registry.
///
/// `package_name` may include a channel prefix (`channel::package`) or just
/// the package name (uses `DEFAULT_CHANNEL`).
pub async fn fetch_latest(
    package_name: &str,
    current_value: &str,
    http: &HttpClient,
) -> Result<CondaUpdateSummary, CondaError> {
    let (channel, name) = parse_channel_package(package_name);
    let url = format!("{ANACONDA_REGISTRY}{channel}/{name}");

    let resp = http.get_retrying(&url).await?;
    if resp.status().as_u16() == 404 {
        return Err(CondaError::NotFound);
    }
    if !resp.status().is_success() {
        return Err(CondaError::NotFound);
    }

    let pkg: AnacondaPackage = resp.json().await.map_err(CondaError::Json)?;

    let latest = pkg.versions.last().cloned();
    let update_available = latest
        .as_deref()
        .is_some_and(|l| l != current_value && !current_value.is_empty());

    Ok(CondaUpdateSummary {
        versions: pkg.versions,
        latest,
        update_available,
    })
}

/// Parse `channel::package` or just `package` (default channel).
fn parse_channel_package(package_name: &str) -> (&str, &str) {
    if let Some((channel, pkg)) = package_name.split_once("::") {
        (channel, pkg)
    } else {
        (DEFAULT_CHANNEL, package_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_channel_package_default() {
        assert_eq!(parse_channel_package("numpy"), (DEFAULT_CHANNEL, "numpy"));
    }

    #[test]
    fn parse_channel_package_explicit() {
        assert_eq!(parse_channel_package("bioconda::bwa"), ("bioconda", "bwa"));
    }
}
