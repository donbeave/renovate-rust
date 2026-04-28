//! Bitrise step library datasource.
//!
//! Fetches available versions for Bitrise CI steps from the
//! Bitrise steplib GitHub repository index.
//!
//! Renovate reference: `lib/modules/datasource/bitrise/index.ts`
//!
//! ## API
//!
//! 1. `GET https://api.github.com/repos/{owner}/{repo}/releases/tags/index`
//!    → finds the `index.json` asset's `browser_download_url`
//! 2. `GET {browser_download_url}` → array of `{ "Name": str, "Versions": [...], "Channels": [...] }`
//!
//! The default steplib is `https://github.com/bitrise-io/bitrise-steplib.git`.

use std::collections::HashMap;
use std::sync::Mutex;

use serde::Deserialize;

use crate::http::HttpClient;

pub const DEFAULT_STEPLIB_URL: &str = "https://github.com/bitrise-io/bitrise-steplib.git";

const GITHUB_API: &str = "https://api.github.com";

#[derive(Debug, Deserialize)]
struct GithubReleaseAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    assets: Vec<GithubReleaseAsset>,
}

#[derive(Debug, Deserialize)]
struct BitriseStepEntry {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Versions", default)]
    versions: Vec<String>,
    #[serde(rename = "Channels", default)]
    channels: Vec<String>,
}

/// Process-level per-registry cache: registry_url → step-name → versions.
static INDEX_CACHE: Mutex<Option<HashMap<String, Vec<String>>>> = Mutex::new(None);

/// Update summary from the Bitrise datasource.
#[derive(Debug)]
pub struct BitriseUpdateSummary {
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Error from the Bitrise datasource.
#[derive(Debug, thiserror::Error)]
pub enum BitriseError {
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("step not found: {0}")]
    NotFound(String),
    #[error("JSON parse error: {0}")]
    Parse(String),
    #[error("invalid registry URL (expected github.com/owner/repo): {0}")]
    InvalidRegistry(String),
}

/// Fetch the latest version for a Bitrise step from the given steplib registry.
///
/// `registry_url` is a GitHub URL like `https://github.com/bitrise-io/bitrise-steplib.git`.
pub async fn fetch_latest(
    http: &HttpClient,
    step_name: &str,
    current_value: &str,
    registry_url: &str,
) -> Result<BitriseUpdateSummary, BitriseError> {
    let index = get_or_fetch_index(http, registry_url).await?;

    let versions = match index.get(step_name) {
        Some(v) if !v.is_empty() => v,
        _ => return Err(BitriseError::NotFound(step_name.to_owned())),
    };

    let latest = versions.last().cloned();
    let update_available = latest.as_deref() != Some(current_value);

    Ok(BitriseUpdateSummary {
        latest,
        update_available,
    })
}

async fn get_or_fetch_index(
    http: &HttpClient,
    registry_url: &str,
) -> Result<HashMap<String, Vec<String>>, BitriseError> {
    // Check cache (only caches the default registry to keep it simple).
    if registry_url == DEFAULT_STEPLIB_URL {
        if let Ok(guard) = INDEX_CACHE.lock() {
            if let Some(ref cached) = *guard {
                return Ok(cached.clone());
            }
        }
    }

    let (owner, repo) = parse_github_url(registry_url)?;
    let repo_clean = repo.trim_end_matches(".git");

    let release_url = format!(
        "{}/repos/{}/{}/releases/tags/index",
        GITHUB_API, owner, repo_clean
    );

    let body = http
        .get_retrying(&release_url)
        .await
        .map_err(|e| BitriseError::Http(e.to_string()))?
        .text()
        .await
        .map_err(|e| BitriseError::Http(e.to_string()))?;

    let release: GithubRelease =
        serde_json::from_str(&body).map_err(|e| BitriseError::Parse(e.to_string()))?;

    let asset_url = release
        .assets
        .iter()
        .find(|a| a.name == "index.json")
        .map(|a| a.browser_download_url.clone())
        .ok_or_else(|| BitriseError::Http("index.json asset not found in release".to_owned()))?;

    let index_body = http
        .get_retrying(&asset_url)
        .await
        .map_err(|e| BitriseError::Http(e.to_string()))?
        .text()
        .await
        .map_err(|e| BitriseError::Http(e.to_string()))?;

    let entries: Vec<BitriseStepEntry> =
        serde_json::from_str(&index_body).map_err(|e| BitriseError::Parse(e.to_string()))?;

    let mut index: HashMap<String, Vec<String>> = HashMap::new();
    for entry in entries {
        let mut versions = entry.versions.clone();
        versions.extend(entry.channels.iter().cloned());
        versions.sort_by(|a, b| {
            let av = semver::Version::parse(a);
            let bv = semver::Version::parse(b);
            match (av, bv) {
                (Ok(a), Ok(b)) => a.cmp(&b),
                _ => a.cmp(b),
            }
        });
        index.insert(entry.name, versions);
    }

    if registry_url == DEFAULT_STEPLIB_URL {
        if let Ok(mut guard) = INDEX_CACHE.lock() {
            *guard = Some(index.clone());
        }
    }

    Ok(index)
}

fn parse_github_url(url: &str) -> Result<(String, String), BitriseError> {
    let url = url.trim_end_matches('/');
    let stripped = url
        .strip_prefix("https://github.com/")
        .or_else(|| url.strip_prefix("http://github.com/"))
        .ok_or_else(|| BitriseError::InvalidRegistry(url.to_owned()))?;

    let parts: Vec<&str> = stripped.splitn(2, '/').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return Err(BitriseError::InvalidRegistry(url.to_owned()));
    }

    Ok((parts[0].to_owned(), parts[1].to_owned()))
}
