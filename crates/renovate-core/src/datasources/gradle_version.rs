//! Gradle Version datasource.
//!
//! Fetches Gradle release information from:
//! `https://services.gradle.org/versions/all`
//!
//! Returns the current Gradle version (latest stable, non-snapshot, non-nightly).
//!
//! Renovate reference: `lib/modules/datasource/gradle-version/index.ts`

use serde::Deserialize;

use crate::http::HttpClient;

pub const GRADLE_VERSIONS_URL: &str = "https://services.gradle.org/versions/all";

#[derive(Debug, Deserialize)]
struct GradleRelease {
    version: String,
    #[serde(default)]
    snapshot: bool,
    #[serde(default)]
    nightly: bool,
    #[serde(default)]
    broken: bool,
}

/// Result of a Gradle version lookup.
#[derive(Debug, Clone)]
pub struct GradleVersionSummary {
    /// Whether a newer stable version exists.
    pub update_available: bool,
    /// Current version (from the wrapper properties file).
    pub current_version: String,
    /// Latest stable Gradle version, if found.
    pub latest: Option<String>,
}

/// Fetch the latest stable Gradle version and compare with `current_version`.
pub async fn fetch_latest(
    http: &HttpClient,
    current_version: &str,
) -> Result<GradleVersionSummary, GradleVersionError> {
    let releases: Vec<GradleRelease> = http
        .get_json(GRADLE_VERSIONS_URL)
        .await
        .map_err(GradleVersionError::Http)?;

    // Filter to stable releases only (no snapshot, no nightly, no broken).
    let mut stable: Vec<String> = releases
        .into_iter()
        .filter(|r| !r.snapshot && !r.nightly && !r.broken)
        .map(|r| r.version)
        .collect();

    if stable.is_empty() {
        return Err(GradleVersionError::NoStableRelease);
    }

    // Sort descending by version (simple lexicographic is fine for `X.Y.Z` semver-like).
    stable.sort_by(|a, b| cmp_gradle_version(b, a));

    let latest = stable.into_iter().next();
    let update_available = latest
        .as_deref()
        .map(|l| l != current_version)
        .unwrap_or(false);

    Ok(GradleVersionSummary {
        update_available,
        current_version: current_version.to_owned(),
        latest,
    })
}

/// Compare Gradle version strings numerically (e.g. `"8.4"` vs `"8.10"`).
fn cmp_gradle_version(a: &str, b: &str) -> std::cmp::Ordering {
    let parse = |s: &str| -> Vec<u32> { s.split('.').filter_map(|p| p.parse().ok()).collect() };
    parse(a).cmp(&parse(b))
}

/// Errors from the Gradle version datasource.
#[derive(Debug, thiserror::Error)]
pub enum GradleVersionError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("no stable Gradle release found")]
    NoStableRelease,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_sort_correctness() {
        let mut versions = vec!["8.4".to_owned(), "8.10".to_owned(), "7.6.1".to_owned()];
        versions.sort_by(|a, b| cmp_gradle_version(b, a));
        assert_eq!(versions, vec!["8.10", "8.4", "7.6.1"]);
    }
}
