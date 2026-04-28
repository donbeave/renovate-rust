//! Glasskube packages datasource.
//!
//! Fetches available package versions from the Glasskube package registry.
//!
//! Renovate reference: `lib/modules/datasource/glasskube-packages/index.ts`
//!
//! ## API
//!
//! `GET https://packages.dl.glasskube.dev/packages/{name}/versions.yaml`
//!
//! Response YAML:
//! ```yaml
//! versions:
//!   - version: "v1.14.2"
//! latestVersion: "v1.14.2"
//! ```

use std::sync::LazyLock;

use regex::Regex;

use crate::http::HttpClient;

pub const GLASSKUBE_REGISTRY_BASE: &str = "https://packages.dl.glasskube.dev/packages";

/// Matches `  - version: "v1.14.2"` or `  - version: v1.14.2`.
static VERSION_ENTRY_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?m)^\s+-\s+version:\s*['"]?([^\s'"]+)['"]?"#).unwrap());

/// Matches `latestVersion: "v1.14.2"`.
static LATEST_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?m)^latestVersion:\s*['"]?([^\s'"]+)['"]?"#).unwrap());

/// Update summary from the Glasskube datasource.
#[derive(Debug)]
pub struct GlasskubeUpdateSummary {
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Error from the Glasskube datasource.
#[derive(Debug, thiserror::Error)]
pub enum GlasskubeError {
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("package not found: {0}")]
    NotFound(String),
    #[error("YAML parse error: {0}")]
    Parse(String),
}

/// Fetch the latest version for a Glasskube package.
///
/// `package_name` is the simple package name (e.g. `cert-manager`).
pub async fn fetch_latest(
    http: &HttpClient,
    package_name: &str,
    current_value: &str,
) -> Result<GlasskubeUpdateSummary, GlasskubeError> {
    let url = format!("{}/{}/versions.yaml", GLASSKUBE_REGISTRY_BASE, package_name);

    let resp = http
        .get_retrying(&url)
        .await
        .map_err(|e| GlasskubeError::Http(e.to_string()))?;

    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(GlasskubeError::NotFound(package_name.to_owned()));
    }
    if !resp.status().is_success() {
        return Err(GlasskubeError::Http(format!(
            "HTTP {}",
            resp.status().as_u16()
        )));
    }

    let body = resp
        .text()
        .await
        .map_err(|e| GlasskubeError::Http(e.to_string()))?;

    // Extract `latestVersion:` first (preferred), then fall back to last version entry.
    let latest = LATEST_RE
        .captures(&body)
        .map(|c| c[1].to_owned())
        .or_else(|| {
            VERSION_ENTRY_RE
                .captures_iter(&body)
                .last()
                .map(|c| c[1].to_owned())
        });

    let update_available = latest.as_deref() != Some(current_value);

    Ok(GlasskubeUpdateSummary {
        latest,
        update_available,
    })
}
