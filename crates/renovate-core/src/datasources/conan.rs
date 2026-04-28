//! Conan Center datasource.
//!
//! Fetches package versions from the Conan Center Index repository on GitHub.
//!
//! Renovate reference:
//! - `lib/modules/datasource/conan/index.ts` — `getConanCenterReleases`
//! - API: `GET https://api.github.com/repos/conan-io/conan-center-index/contents/recipes/{name}/config.yml`
//!   with `Accept: application/vnd.github.v3.raw` to get the raw YAML.
//!
//! ## `config.yml` format
//!
//! ```yaml
//! versions:
//!   "1.2.11":
//!     folder: all
//!   "1.3.0":
//!     folder: all
//! ```
//!
//! We parse the version keys with a regex — no YAML parser dependency needed.

use std::sync::LazyLock;

use regex::Regex;
use thiserror::Error;

use crate::http::HttpClient;

const CONFIG_URL_BASE: &str =
    "https://api.github.com/repos/conan-io/conan-center-index/contents/recipes";

/// Errors from fetching Conan Center metadata.
#[derive(Debug, Error)]
pub enum ConanError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("package not found: {0}")]
    NotFound(String),
}

/// Update summary for a Conan package.
#[derive(Debug, Clone)]
pub struct ConanUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Matches a version key line in `config.yml` of the form `  "1.2.3":` or `  1.2.3:`.
static VERSION_LINE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?m)^\s+"?(\d[^":\s]*)"?\s*:"#).unwrap());

/// Fetch the latest version for a Conan Center package.
///
/// Requires a GitHub-authenticated HTTP client (passes token via bearer auth).
pub async fn fetch_latest(
    http: &HttpClient,
    package_name: &str,
    current_value: &str,
) -> Result<ConanUpdateSummary, ConanError> {
    let url = format!("{CONFIG_URL_BASE}/{package_name}/config.yml");
    let yaml: String = http
        .get_raw_with_accept(&url, "application/vnd.github.v3.raw")
        .await?;

    let versions: Vec<String> = VERSION_LINE
        .captures_iter(&yaml)
        .map(|c| c[1].to_owned())
        .collect();

    if versions.is_empty() {
        return Err(ConanError::NotFound(package_name.to_owned()));
    }

    let latest = versions.iter().max_by(|a, b| cmp_semver(a, b)).cloned();

    let update_available = match &latest {
        Some(l) => l != current_value,
        None => false,
    };

    Ok(ConanUpdateSummary {
        current_value: current_value.to_owned(),
        latest,
        update_available,
    })
}

/// Simple numeric component comparison for version strings.
fn cmp_semver(a: &str, b: &str) -> std::cmp::Ordering {
    let parts = |s: &str| -> Vec<u64> {
        s.split('.')
            .map(|p| p.parse::<u64>().unwrap_or(0))
            .collect()
    };
    let av = parts(a);
    let bv = parts(b);
    for i in 0..av.len().max(bv.len()) {
        let ai = av.get(i).copied().unwrap_or(0);
        let bi = bv.get(i).copied().unwrap_or(0);
        match ai.cmp(&bi) {
            std::cmp::Ordering::Equal => continue,
            other => return other,
        }
    }
    std::cmp::Ordering::Equal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_config_yml() {
        let yaml = "versions:\n  \"1.2.11\":\n    folder: all\n  \"1.3.0\":\n    folder: all\n";
        let versions: Vec<String> = VERSION_LINE
            .captures_iter(yaml)
            .map(|c| c[1].to_owned())
            .collect();
        assert_eq!(versions, vec!["1.2.11", "1.3.0"]);
    }

    #[test]
    fn cmp_semver_basic() {
        use std::cmp::Ordering::*;
        assert_eq!(cmp_semver("1.3.0", "1.2.11"), Greater);
        assert_eq!(cmp_semver("1.2.11", "1.3.0"), Less);
        assert_eq!(cmp_semver("1.0.0", "1.0.0"), Equal);
    }
}
