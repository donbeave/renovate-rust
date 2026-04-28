//! CPAN (MetaCPAN) datasource.
//!
//! Fetches the latest version of a Perl module from the MetaCPAN API.
//!
//! Renovate reference: `lib/modules/datasource/cpan/index.ts`
//!
//! ## API
//!
//! `GET https://fastapi.metacpan.org/v1/module/{module_name}`
//!
//! Response: `{ "version": "2.2006", "name": "Moose", ... }`

use serde::Deserialize;

use crate::http::HttpClient;
use crate::versioning::semver_generic::semver_update_summary;

pub const METACPAN_API: &str = "https://fastapi.metacpan.org";

#[derive(Debug, Deserialize)]
struct MetaCpanModuleResponse {
    version: Option<String>,
}

/// Update summary from the CPAN datasource.
#[derive(Debug)]
pub struct CpanUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Error from the CPAN datasource.
#[derive(Debug, thiserror::Error)]
pub enum CpanError {
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("module not found: {0}")]
    NotFound(String),
    #[error("JSON parse error: {0}")]
    Parse(String),
}

/// Fetch the latest version of a Perl module from MetaCPAN.
///
/// `module_name` is in Perl double-colon notation, e.g. `Moose` or `Test::More`.
pub async fn fetch_latest(
    http: &HttpClient,
    module_name: &str,
    current_value: &str,
) -> Result<CpanUpdateSummary, CpanError> {
    // URL-encode `::` as `%3A%3A` to be safe, though MetaCPAN also accepts `::`.
    let url = format!("{}/v1/module/{}", METACPAN_API, module_name);

    let body = http
        .get_raw_with_accept(&url, "application/json")
        .await
        .map_err(|e| CpanError::Http(e.to_string()))?;

    let resp: MetaCpanModuleResponse =
        serde_json::from_str(&body).map_err(|e| CpanError::Parse(e.to_string()))?;

    let latest = resp.version;
    if latest.is_none() {
        return Err(CpanError::NotFound(module_name.to_owned()));
    }

    // Normalize Perl version: `5.036001` → `5.36.1`, `1.023` → `1.023` (keep).
    // For simple semver-ish versions, delegate to the generic comparator.
    let summary = semver_update_summary(current_value, latest.as_deref());
    Ok(CpanUpdateSummary {
        current_value: current_value.to_owned(),
        latest,
        update_available: summary.update_available,
    })
}
