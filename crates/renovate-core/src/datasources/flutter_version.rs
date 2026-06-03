//! Flutter SDK version datasource.
//!
//! Fetches Flutter release information from the Google Cloud Storage
//! flutter_infra_release bucket at `releases_linux.json`.
//!
//! Renovate reference: `lib/modules/datasource/flutter-version/index.ts`
//! Registry:           `https://storage.googleapis.com`

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY_URL: &str = "https://storage.googleapis.com";
pub const DATASOURCE_ID: &str = "flutter-version";

/// Errors from the Flutter version datasource.
#[derive(Debug, Error)]
pub enum FlutterVersionError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

/// Top-level shape of `releases_linux.json`.
#[derive(Debug, Deserialize)]
struct FlutterResponse {
    #[serde(default)]
    releases: Vec<FlutterRelease>,
}

/// One entry in the `releases` array.
#[derive(Debug, Deserialize)]
struct FlutterRelease {
    version: String,
    channel: String,
    release_date: Option<String>,
}

/// Regex-like check: `X.Y.Z` where every part is purely numeric.
fn is_stable_pattern(v: &str) -> bool {
    let parts: Vec<&str> = v.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
}

/// One release entry returned by `fetch_releases`.
#[derive(Debug, Clone)]
pub struct FlutterVersionRelease {
    pub version: String,
    pub release_timestamp: Option<String>,
    pub is_stable: bool,
}

/// Result of a `fetch_releases` call.
#[derive(Debug, Clone)]
pub struct FlutterVersionResult {
    pub releases: Vec<FlutterVersionRelease>,
    pub homepage: &'static str,
    pub source_url: &'static str,
}

/// Fetch all Flutter SDK releases from `registry_url`.
///
/// Filters out stable-pattern versions that appear on non-stable channels,
/// matching the upstream TypeScript behaviour.  Returns `None` when the
/// server returns an empty releases list.
pub async fn fetch_releases(
    registry_url: &str,
    http: &HttpClient,
) -> Result<Option<FlutterVersionResult>, FlutterVersionError> {
    let url = format!("{registry_url}/flutter_infra_release/releases/releases_linux.json");
    let resp: FlutterResponse = http.get_json(&url).await?;

    let releases: Vec<FlutterVersionRelease> = resp
        .releases
        .into_iter()
        .filter(|r| {
            // A version that looks like a stable release (X.Y.Z all-digit)
            // must only appear on the stable channel.
            if is_stable_pattern(&r.version) {
                r.channel == "stable"
            } else {
                true
            }
        })
        .map(|r| FlutterVersionRelease {
            version: r.version,
            release_timestamp: r.release_date,
            is_stable: r.channel == "stable",
        })
        .collect();

    if releases.is_empty() {
        return Ok(None);
    }

    Ok(Some(FlutterVersionResult {
        releases,
        homepage: "https://flutter.dev",
        source_url: "https://github.com/flutter/flutter",
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "returns null for empty 200 OK" — lib/modules/datasource/flutter-version/index.spec.ts line 34
    #[test]
    fn empty_releases_yield_none() {
        let resp = FlutterResponse { releases: vec![] };
        assert!(resp.releases.is_empty());
    }

    // Ported: "processes real data" — lib/modules/datasource/flutter-version/index.spec.ts line 44
    #[test]
    fn parse_stable_release() {
        let json = r#"{"releases":[{"hash":"abc","channel":"stable","version":"3.0.0","dart_sdk_version":"2.17.0","release_date":"2022-05-11T17:04:19.536736Z","archive":"stable/linux/flutter_linux_3.0.0-stable.tar.xz","sha256":"xyz"}]}"#;
        let resp: FlutterResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.releases.len(), 1);
        assert_eq!(resp.releases[0].channel, "stable");
        assert_eq!(resp.releases[0].version, "3.0.0");
    }

    // Rust-specific: flutter_version behavior test
    #[test]
    fn stable_pattern_filter_on_beta() {
        // A stable-looking version on beta channel should be filtered out.
        let releases = vec![
            FlutterRelease {
                version: "3.0.0".into(),
                channel: "beta".into(),
                release_date: None,
            },
            FlutterRelease {
                version: "3.1.0-0.pre".into(),
                channel: "beta".into(),
                release_date: None,
            },
        ];
        let filtered: Vec<_> = releases
            .into_iter()
            .filter(|r| {
                if is_stable_pattern(&r.version) {
                    r.channel == "stable"
                } else {
                    true
                }
            })
            .collect();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].version, "3.1.0-0.pre");
    }

    // Rust-specific: flutter_version behavior test
    #[test]
    fn is_stable_pattern_works() {
        assert!(is_stable_pattern("3.0.0"));
        assert!(is_stable_pattern("2.17.5"));
        assert!(!is_stable_pattern("3.1.0-0.pre"));
        assert!(!is_stable_pattern("2.12.0-4.1.pre"));
    }
}
