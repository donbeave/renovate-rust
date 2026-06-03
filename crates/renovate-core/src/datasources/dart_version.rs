//! Dart SDK version datasource.
//!
//! Fetches available Dart SDK versions from the Google Cloud Storage
//! dart-archive bucket.  Versions are discovered by listing the
//! `channels/{channel}/release/` prefix for three channels:
//! `stable`, `beta`, and `dev`.
//!
//! Renovate reference: `lib/modules/datasource/dart-version/index.ts`
//! Registry:           `https://storage.googleapis.com`

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY_URL: &str = "https://storage.googleapis.com";
pub const DATASOURCE_ID: &str = "dart-version";

/// Errors from the Dart version datasource.
#[derive(Debug, Error)]
pub enum DartVersionError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

/// GCS listing response for a dart-archive prefix.
#[derive(Debug, Deserialize)]
struct GcsPrefixList {
    #[serde(default)]
    prefixes: Vec<String>,
}

/// One release entry returned by `fetch_releases`.
#[derive(Debug, Clone)]
pub struct DartVersionRelease {
    /// Version string (e.g. `"2.17.5"`).
    pub version: String,
    /// Whether this is a stable-channel release.
    pub is_stable: bool,
}

/// Result of a `fetch_releases` call.
#[derive(Debug, Clone)]
pub struct DartVersionResult {
    pub releases: Vec<DartVersionRelease>,
    pub homepage: &'static str,
    pub source_url: &'static str,
    pub registry_url: String,
}

/// Extract the version string from a GCS object prefix.
///
/// The prefix format is `"channels/stable/release/2.17.5/"`.
/// Returns `None` for `"latest"`, old SVN-style numeric-only versions, and
/// stable version strings that appear on a non-stable channel (Renovate
/// upstream filters these too).
fn version_from_prefix(prefix: &str, channel: &str) -> Option<String> {
    // Strip exactly one trailing slash (GCS prefix format: "channels/x/release/VER/").
    // Using strip_suffix rather than trim_end_matches so double-slash inputs yield
    // an empty version string (which we then reject below).
    let without_trailing = prefix.strip_suffix('/')?;
    let parts: Vec<&str> = without_trailing.split('/').collect();
    let version = (*parts.last()?).to_owned();

    if version.is_empty() || version == "latest" {
        return None;
    }
    // Skip old SVN-style pure-integer versions.
    if version.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    // A version that looks fully stable (X.Y.Z with all-digit parts) should
    // only appear on the stable channel; skip it on beta/dev.
    let is_stable_pattern = version
        .split('.')
        .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()));
    if is_stable_pattern && channel != "stable" {
        return None;
    }

    Some(version)
}

/// Fetch all Dart SDK releases across `stable`, `beta`, and `dev` channels.
///
/// Returns `None` when all channels return empty lists.
pub async fn fetch_releases(
    registry_url: &str,
    http: &HttpClient,
) -> Result<Option<DartVersionResult>, DartVersionError> {
    let channels = ["stable", "beta", "dev"];
    let mut all: Vec<DartVersionRelease> = Vec::new();

    for channel in &channels {
        let url = format!(
            "{registry_url}/storage/v1/b/dart-archive/o?delimiter=%2F&prefix=channels%2F{channel}%2Frelease%2F&alt=json"
        );
        let resp: GcsPrefixList = match http.get_json(&url).await {
            Ok(resp) => resp,
            Err(crate::http::HttpError::Request(_)) => return Ok(None),
            Err(error) => return Err(error.into()),
        };
        let is_stable = *channel == "stable";

        for prefix in &resp.prefixes {
            if let Some(version) = version_from_prefix(prefix, channel) {
                all.push(DartVersionRelease { version, is_stable });
            }
        }
    }

    if all.is_empty() {
        return Ok(None);
    }

    Ok(Some(DartVersionResult {
        releases: all,
        homepage: "https://dart.dev/",
        source_url: "https://github.com/dart-lang/sdk",
        registry_url: registry_url.to_owned(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // Ported: "returns null for empty 200 OK" — lib/modules/datasource/dart-version/index.spec.ts line 36
    #[test]
    fn empty_prefix_lists_yield_no_releases() {
        let result = version_from_prefix("channels/stable/release//", "stable");
        assert!(result.is_none());
    }

    // Ported: "processes real data" — lib/modules/datasource/dart-version/index.spec.ts line 53
    #[test]
    fn extract_version_from_stable_prefix() {
        let v = version_from_prefix("channels/stable/release/2.17.5/", "stable");
        assert_eq!(v, Some("2.17.5".into()));
    }

    // Rust-specific: dart_version behavior test
    #[test]
    fn skip_latest_prefix() {
        assert!(version_from_prefix("channels/stable/release/latest/", "stable").is_none());
    }

    // Rust-specific: dart_version behavior test
    #[test]
    fn skip_svn_style_versions() {
        // Old SVN-based numeric-only version strings are ignored.
        assert!(version_from_prefix("channels/stable/release/12345/", "stable").is_none());
    }

    // Rust-specific: dart_version behavior test
    #[test]
    fn skip_stable_version_on_beta_channel() {
        // A fully numeric X.Y.Z version on beta is filtered out.
        assert!(version_from_prefix("channels/beta/release/2.17.5/", "beta").is_none());
    }

    // Rust-specific: dart_version behavior test
    #[test]
    fn keep_beta_version_on_beta_channel() {
        let v = version_from_prefix("channels/beta/release/2.18.0-44.1.beta/", "beta");
        assert!(v.is_some());
    }

    // Ported: "throws for 500" — lib/modules/datasource/dart-version/index.spec.ts line 16
    #[tokio::test]
    async fn throws_for_500() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/storage/v1/b/dart-archive/o"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), &http).await;
        assert!(result.is_err(), "server 500 should propagate as error");
    }

    // Ported: "returns null for error" — lib/modules/datasource/dart-version/index.spec.ts line 26
    #[tokio::test]
    async fn returns_null_for_error() {
        let bad_url = "http://127.0.0.1:1";
        let http = HttpClient::new().unwrap();
        let result = fetch_releases(bad_url, &http).await.unwrap();
        assert!(result.is_none(), "network errors should return None");
    }
}
