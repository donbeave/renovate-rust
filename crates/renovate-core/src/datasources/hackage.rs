//! Hackage datasource for Haskell packages.
//!
//! Fetches package versions from the Hackage registry.
//!
//! Renovate reference:
//! - `lib/modules/datasource/hackage/index.ts`
//! - API: `GET https://hackage.haskell.org/package/{name}.json`
//!
//! The response is a JSON object: `{"1.0.0": "normal", "0.9.0": "deprecated"}`.

use std::collections::HashMap;

use thiserror::Error;

use crate::http::HttpClient;

const HACKAGE_BASE: &str = "https://hackage.haskell.org";

/// Errors from fetching Hackage package metadata.
#[derive(Debug, Error)]
pub enum HackageError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("package not found: {0}")]
    NotFound(String),
}

/// Update summary for a Hackage dep.
#[derive(Debug, Clone)]
pub struct HackageUpdateSummary {
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Fetch the latest non-deprecated version of a Hackage package.
pub async fn fetch_latest(
    http: &HttpClient,
    package_name: &str,
) -> Result<HackageUpdateSummary, HackageError> {
    let encoded = urlencoding(package_name);
    let url = format!("{HACKAGE_BASE}/package/{encoded}.json");
    let versions: HashMap<String, String> = http.get_json(&url).await?;

    // Filter out deprecated versions; find latest by version ordering.
    let mut valid: Vec<String> = versions
        .into_iter()
        .filter(|(_, status)| status != "deprecated")
        .map(|(v, _)| v)
        .collect();

    valid.sort_by(|a, b| cmp_pvp(a, b));
    let latest = valid.pop();

    Ok(HackageUpdateSummary {
        update_available: false, // caller compares
        latest,
    })
}

/// Simple URL encoding for package names (handles `+` in names).
fn urlencoding(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' => c.to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}

/// Compare two PVP version strings numerically.
fn cmp_pvp(a: &str, b: &str) -> std::cmp::Ordering {
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
    fn cmp_pvp_ordering() {
        use std::cmp::Ordering::*;
        assert_eq!(cmp_pvp("2.0.0", "1.9.9"), Greater);
        assert_eq!(cmp_pvp("1.0.0", "1.0.0"), Equal);
        assert_eq!(cmp_pvp("4.7.0.0", "4.7.0.1"), Less);
    }
}
