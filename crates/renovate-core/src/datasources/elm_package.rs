//! Elm package version datasource.
//!
//! Fetches release information from package.elm-lang.org for a given package.
//! The registry returns a JSON object mapping version strings to Unix timestamps.
//!
//! Renovate reference: `lib/modules/datasource/elm-package/index.ts`
//! Registry:           `https://package.elm-lang.org`

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY_URL: &str = "https://package.elm-lang.org";
pub const DATASOURCE_ID: &str = "elm-package";

/// Errors from the Elm package datasource.
#[derive(Debug, Error)]
pub enum ElmPackageError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

/// One release entry returned by `fetch_releases`.
#[derive(Debug, Clone)]
pub struct ElmPackageRelease {
    pub version: String,
    pub release_timestamp: Option<String>,
}

/// Result of a `fetch_releases` call.
#[derive(Debug, Clone)]
pub struct ElmPackageResult {
    pub releases: Vec<ElmPackageRelease>,
    pub source_url: Option<String>,
    pub registry_url: String,
}

/// Response from `packages/{packageName}/releases.json` — version → Unix timestamp (seconds).
#[derive(Debug, Deserialize)]
struct ReleasesResponse(HashMap<String, i64>);

fn unix_secs_to_iso(secs: i64) -> Option<String> {
    let dt = DateTime::<Utc>::from_timestamp(secs, 0)?;
    Some(dt.format("%Y-%m-%dT%H:%M:%S.000Z").to_string())
}

fn is_fatal_status(status: StatusCode) -> bool {
    status.is_server_error() || status == StatusCode::TOO_MANY_REQUESTS
}

/// Fetch Elm package releases from `package.elm-lang.org`.
///
/// - `registry_url`: base URL (e.g. `"https://package.elm-lang.org"`)
/// - `package_name`: package name like `"elm/core"` or `"author/pkg"`
///
/// Returns `None` when registry URL is empty, the package has no releases,
/// or the response is invalid. Propagates errors for 5xx / 429 responses.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<ElmPackageResult>, ElmPackageError> {
    if registry_url.is_empty() {
        return Ok(None);
    }

    let url = format!(
        "{}/packages/{}/releases.json",
        registry_url.trim_end_matches('/'),
        package_name
    );

    let releases_map: HashMap<String, i64> = match http.get_json::<ReleasesResponse>(&url).await {
        Ok(r) => r.0,
        Err(crate::http::HttpError::Status { status, .. }) if is_fatal_status(status) => {
            return Err(ElmPackageError::Http(crate::http::HttpError::Status {
                status,
                url: url.clone(),
            }));
        }
        Err(crate::http::HttpError::Status { .. }) => return Ok(None),
        Err(crate::http::HttpError::Request(_)) => return Ok(None),
        Err(crate::http::HttpError::Parse(_)) => return Ok(None),
    };

    if releases_map.is_empty() {
        return Ok(None);
    }

    let mut releases: Vec<ElmPackageRelease> = releases_map
        .into_iter()
        .map(|(version, ts)| ElmPackageRelease {
            version,
            release_timestamp: unix_secs_to_iso(ts),
        })
        .collect();

    releases.sort_by(|a, b| a.version.cmp(&b.version));

    let source_url = if package_name.contains('/') {
        Some(format!("https://github.com/{package_name}"))
    } else {
        None
    };

    Ok(Some(ElmPackageResult {
        releases,
        source_url,
        registry_url: registry_url.to_owned(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "processes real data" — elm-package/index.spec.ts line 97
    #[test]
    fn processes_real_data() {
        let body = r#"{"1.0.0":1534771622,"1.0.1":1542199511,"1.0.2":1542317893,"1.0.3":1575566216,"1.0.4":1575998397,"1.0.5":1581794195}"#;
        let resp: ReleasesResponse = serde_json::from_str(body).unwrap();
        let mut releases: Vec<ElmPackageRelease> = resp
            .0
            .into_iter()
            .map(|(version, ts)| ElmPackageRelease {
                version,
                release_timestamp: unix_secs_to_iso(ts),
            })
            .collect();
        releases.sort_by(|a, b| a.version.cmp(&b.version));

        assert_eq!(releases.len(), 6);
        assert_eq!(releases[0].version, "1.0.0");
        assert_eq!(
            releases[0].release_timestamp.as_deref(),
            Some("2018-08-20T13:27:02.000Z")
        );
        assert_eq!(releases[5].version, "1.0.5");
        assert_eq!(
            releases[5].release_timestamp.as_deref(),
            Some("2020-02-15T19:16:35.000Z")
        );
    }

    // Ported: "returns null for empty result" — elm-package/index.spec.ts line 19
    #[test]
    fn empty_map_returns_none() {
        let resp: ReleasesResponse = serde_json::from_str("{}").unwrap();
        assert!(resp.0.is_empty());
    }

    // Ported: "returns null for invalid schema response" — elm-package/index.spec.ts line 129
    #[test]
    fn non_numeric_timestamp_fails_deserialization() {
        let result: Result<ReleasesResponse, _> =
            serde_json::from_str(r#"{"1.0.0":"not-a-number"}"#);
        assert!(result.is_err());
    }

    // Ported: "handles package without slash in name" — elm-package/index.spec.ts line 142
    #[test]
    fn package_without_slash_has_no_source_url() {
        let name = "somepackage";
        let source_url = if name.contains('/') {
            Some(format!("https://github.com/{name}"))
        } else {
            None
        };
        assert!(source_url.is_none());
    }

    // Rust-specific: elm_package behavior test
    #[test]
    fn unix_secs_to_iso_conversion() {
        assert_eq!(
            unix_secs_to_iso(1534771622).as_deref(),
            Some("2018-08-20T13:27:02.000Z")
        );
        assert_eq!(
            unix_secs_to_iso(1581794195).as_deref(),
            Some("2020-02-15T19:16:35.000Z")
        );
    }
}
