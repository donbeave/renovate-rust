//! Devbox package datasource.
//!
//! Fetches available versions for Nix packages via the Devbox search API:
//! `https://search.devbox.sh/v2/pkg?name=<package>`
//!
//! Renovate reference: `lib/modules/datasource/devbox/index.ts`

use serde::Deserialize;

use crate::http::HttpClient;

pub const DEVBOX_API_BASE: &str = "https://search.devbox.sh/v2";

#[derive(Debug, Deserialize)]
struct DevboxResponse {
    releases: Vec<DevboxRelease>,
}

#[derive(Debug, Deserialize)]
struct DevboxRelease {
    version: String,
}

/// Summary of a devbox package version lookup.
#[derive(Debug, Clone)]
pub struct DevboxUpdateSummary {
    pub update_available: bool,
    pub current_version: String,
    pub latest: Option<String>,
}

/// Errors from the devbox datasource.
#[derive(Debug, thiserror::Error)]
pub enum DevboxError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("no releases found for package")]
    NoReleases,
}

/// Fetch the latest version for a devbox package and compare with `current`.
pub async fn fetch_latest(
    http: &HttpClient,
    package: &str,
    current_version: &str,
) -> Result<DevboxUpdateSummary, DevboxError> {
    // Encode the package name for URL safety (handles spaces, slashes, etc.)
    let encoded: String = package
        .bytes()
        .flat_map(|b| {
            if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.') {
                vec![b as char]
            } else {
                format!("%{b:02X}").chars().collect::<Vec<_>>()
            }
        })
        .collect();
    let url = format!("{DEVBOX_API_BASE}/pkg?name={encoded}");
    let resp: DevboxResponse = http.get_json(&url).await.map_err(DevboxError::Http)?;

    if resp.releases.is_empty() {
        return Err(DevboxError::NoReleases);
    }

    // Releases are returned in descending order — first is latest.
    let latest = resp.releases.into_iter().next().map(|r| r.version);
    let update_available = latest
        .as_deref()
        .map(|l| l != current_version)
        .unwrap_or(false);

    Ok(DevboxUpdateSummary {
        update_available,
        current_version: current_version.to_owned(),
        latest,
    })
}
