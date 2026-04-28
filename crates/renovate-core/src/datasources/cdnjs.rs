//! CDNJS datasource.
//!
//! Fetches available versions for a library from the cdnjs API.
//!
//! Renovate reference: `lib/modules/datasource/cdnjs/index.ts`
//!
//! ## API format
//!
//! GET `https://api.cdnjs.com/libraries/{library}?fields=versions`
//!
//! Response: `{ "versions": ["1.0.0", "1.1.0", "2.0.0"] }`

use std::sync::LazyLock;

use regex::Regex;
use serde::Deserialize;

use crate::http::HttpClient;
use crate::versioning::semver_generic::semver_update_summary;

pub const CDNJS_API: &str = "https://api.cdnjs.com/";

#[derive(Debug, Deserialize)]
struct CdnjsResponse {
    versions: Vec<String>,
}

/// Update summary returned by the cdnjs datasource.
#[derive(Debug)]
pub struct CdnjsUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Error returned by the cdnjs datasource.
#[derive(Debug, thiserror::Error)]
pub enum CdnjsError {
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("library not found: {0}")]
    NotFound(String),
    #[error("JSON parse error: {0}")]
    Parse(String),
}

// Strips a leading `v` from semver tags like `v3.6.0` → `3.6.0` for comparison.
static V_PREFIX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^v(\d)").unwrap());

/// Fetch the latest version for `library` from the cdnjs API.
///
/// `library` is just the bare library name (e.g. `jquery`, not `jquery/jquery.min.js`).
pub async fn fetch_latest(
    http: &HttpClient,
    library: &str,
    current_value: &str,
) -> Result<CdnjsUpdateSummary, CdnjsError> {
    let url = format!("{}libraries/{}?fields=versions", CDNJS_API, library);

    let body = http
        .get_raw_with_accept(&url, "application/json")
        .await
        .map_err(|e| CdnjsError::Http(e.to_string()))?;

    let response: CdnjsResponse =
        serde_json::from_str(&body).map_err(|e| CdnjsError::Parse(e.to_string()))?;

    if response.versions.is_empty() {
        return Err(CdnjsError::NotFound(library.to_owned()));
    }

    // Find the highest semver in the versions list.
    let latest = response
        .versions
        .iter()
        .max_by(|a, b| {
            let a_clean = V_PREFIX.replace(a, "$1").into_owned();
            let b_clean = V_PREFIX.replace(b, "$1").into_owned();
            let s = semver_update_summary(&a_clean, Some(&b_clean));
            if s.update_available {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        })
        .cloned();

    let summary = semver_update_summary(current_value, latest.as_deref());
    Ok(CdnjsUpdateSummary {
        current_value: current_value.to_owned(),
        latest,
        update_available: summary.update_available,
    })
}
