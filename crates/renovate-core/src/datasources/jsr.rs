//! JSR (JavaScript Registry) datasource.
//!
//! Fetches available versions for a JSR package from `https://jsr.io`.
//!
//! Renovate reference:
//! - `lib/modules/datasource/jsr/index.ts`
//! - `lib/modules/datasource/jsr/schema.ts`
//! - Default registry: `https://jsr.io/`
//!
//! ## API format
//!
//! `GET https://jsr.io/@scope/name/meta.json`
//!
//! ```json
//! {
//!   "latest": "1.2.3",
//!   "versions": {
//!     "1.0.0": {},
//!     "1.2.3": { "yanked": false }
//!   }
//! }
//! ```

use std::collections::HashMap;

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

/// Default JSR registry base.
pub const JSR_REGISTRY: &str = "https://jsr.io/";

/// Errors from JSR datasource lookups.
#[derive(Debug, Error)]
pub enum JsrError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(reqwest::Error),
    #[error("invalid package name '{0}': must be @scope/name")]
    InvalidPackageName(String),
    #[error("package not found")]
    NotFound,
}

#[derive(Debug, Deserialize)]
struct JsrMeta {
    latest: Option<String>,
    #[serde(default)]
    versions: HashMap<String, JsrVersionMeta>,
}

#[derive(Debug, Deserialize)]
struct JsrVersionMeta {
    #[serde(default)]
    yanked: bool,
}

/// Summary of a JSR package version lookup.
#[derive(Debug, Clone)]
pub struct JsrUpdateSummary {
    /// Latest stable (non-yanked) version.
    pub latest: Option<String>,
    /// Whether `latest` != `current_value`.
    pub update_available: bool,
    /// All non-yanked versions.
    pub versions: Vec<String>,
}

/// Fetch the latest version of a JSR package.
///
/// `package_name` must be in `@scope/name` form.
pub async fn fetch_latest(
    package_name: &str,
    current_value: &str,
    http: &HttpClient,
) -> Result<JsrUpdateSummary, JsrError> {
    validate_package_name(package_name)?;

    let url = format!("{JSR_REGISTRY}{package_name}/meta.json");
    let resp = http.get_retrying(&url).await?;
    if resp.status().as_u16() == 404 {
        return Err(JsrError::NotFound);
    }
    if !resp.status().is_success() {
        return Err(JsrError::NotFound);
    }

    let meta: JsrMeta = resp.json().await.map_err(JsrError::Json)?;

    let mut versions: Vec<String> = meta
        .versions
        .iter()
        .filter(|(_, v)| !v.yanked)
        .map(|(k, _)| k.clone())
        .collect();
    versions.sort(); // semver-ish sort; good enough for display

    let latest = meta
        .latest
        .filter(|v| meta.versions.get(v.as_str()).is_none_or(|mv| !mv.yanked));

    let update_available = latest
        .as_deref()
        .is_some_and(|l| l != current_value && !current_value.is_empty());

    Ok(JsrUpdateSummary {
        latest,
        update_available,
        versions,
    })
}

/// Validate that `package_name` is in `@scope/name` form.
fn validate_package_name(package_name: &str) -> Result<(), JsrError> {
    if !package_name.starts_with('@') || !package_name.contains('/') {
        return Err(JsrError::InvalidPackageName(package_name.to_owned()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_package_name_rejects_unscoped() {
        assert!(validate_package_name("bare-package").is_err());
    }

    #[test]
    fn validate_package_name_accepts_scoped() {
        assert!(validate_package_name("@std/path").is_ok());
    }
}
