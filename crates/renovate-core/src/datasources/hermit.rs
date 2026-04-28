//! Hermit package manager datasource.
//!
//! Fetches available versions for a named package from the Hermit package
//! index hosted at `cashapp/hermit-packages` on GitHub.
//!
//! Renovate reference:
//! - `lib/modules/datasource/hermit/index.ts`
//!
//! ## Protocol
//!
//! 1. Fetch the GitHub release tagged `index` from the registry repository.
//! 2. Download the `index.json` asset attached to that release.
//! 3. Find the entry matching `packageName`.
//! 4. Return `Versions` (semver strings) and `Channels` (channel names).

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

/// Default Hermit package registry.
pub const DEFAULT_REGISTRY: &str = "https://github.com/cashapp/hermit-packages";

/// GitHub API base used to look up release assets.
const GH_API_BASE: &str = "https://api.github.com";

/// Errors from Hermit datasource lookups.
#[derive(Debug, Error)]
pub enum HermitError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(reqwest::Error),
    #[error("Registry URL is not a GitHub URL: {0}")]
    NotGitHub(String),
    #[error("release index not found at {0}")]
    NoIndexRelease(String),
    #[error("index.json asset not found in release")]
    NoIndexAsset,
    #[error("package '{0}' not found in Hermit index")]
    NotFound(String),
}

/// One entry from `index.json`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct HermitSearchResult {
    name: String,
    #[serde(default)]
    versions: Vec<String>,
    #[serde(default)]
    channels: Vec<String>,
    repository: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

/// Summary of available Hermit package versions.
#[derive(Debug, Clone)]
pub struct HermitUpdateSummary {
    /// All versioned releases, oldest first.
    pub versions: Vec<String>,
    /// Named channels (e.g. `stable`, `latest`).
    pub channels: Vec<String>,
    /// Latest version (last in the versions list).
    pub latest: Option<String>,
    /// Source repo URL for the package.
    pub source_url: Option<String>,
    /// Whether any version is newer than `current_value`.
    pub update_available: bool,
}

/// Fetch versions for a Hermit package from the given registry.
pub async fn fetch_latest(
    package_name: &str,
    current_value: &str,
    registry_url: &str,
    http: &HttpClient,
) -> Result<HermitUpdateSummary, HermitError> {
    // Parse owner/repo from registry URL.
    let (owner, repo) = parse_github_repo(registry_url)
        .ok_or_else(|| HermitError::NotGitHub(registry_url.to_owned()))?;

    // Fetch the `index` release from GitHub API.
    let release_url = format!("{GH_API_BASE}/repos/{owner}/{repo}/releases/tags/index");
    let release_resp = http.get_retrying(&release_url).await?;
    if !release_resp.status().is_success() {
        return Err(HermitError::NoIndexRelease(release_url));
    }
    let release: GithubRelease = release_resp.json().await.map_err(HermitError::Json)?;

    // Find the `index.json` asset.
    let asset = release
        .assets
        .iter()
        .find(|a| a.name == "index.json")
        .ok_or(HermitError::NoIndexAsset)?;

    // Download the index.
    let index_resp = http.get_retrying(&asset.browser_download_url).await?;
    let entries: Vec<HermitSearchResult> = index_resp.json().await.map_err(HermitError::Json)?;

    // Find the entry for our package.
    let entry = entries
        .iter()
        .find(|e| e.name.eq_ignore_ascii_case(package_name))
        .ok_or_else(|| HermitError::NotFound(package_name.to_owned()))?;

    let latest = entry.versions.last().cloned();
    let update_available = latest
        .as_deref()
        .is_some_and(|l| l != current_value && !current_value.is_empty());

    Ok(HermitUpdateSummary {
        versions: entry.versions.clone(),
        channels: entry.channels.clone(),
        latest,
        source_url: entry.repository.clone(),
        update_available,
    })
}

/// Parse `owner` and `repo` from a `https://github.com/{owner}/{repo}` URL.
fn parse_github_repo(url: &str) -> Option<(&str, &str)> {
    let path = url.strip_prefix("https://github.com/")?;
    let mut parts = path.trim_end_matches('/').splitn(2, '/');
    let owner = parts.next()?;
    let repo = parts.next()?;
    Some((owner, repo))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_github_repo_works() {
        assert_eq!(
            parse_github_repo("https://github.com/cashapp/hermit-packages"),
            Some(("cashapp", "hermit-packages"))
        );
        assert_eq!(
            parse_github_repo("https://github.com/cashapp/hermit-packages/"),
            Some(("cashapp", "hermit-packages"))
        );
        assert_eq!(parse_github_repo("https://gitlab.com/foo/bar"), None);
    }
}
