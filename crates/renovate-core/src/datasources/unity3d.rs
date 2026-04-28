//! Unity3D Editor version datasource.
//!
//! Fetches available Unity Editor versions from the Unity Releases API.
//!
//! Renovate reference: `lib/modules/datasource/unity3d/index.ts`
//!
//! ## API
//!
//! `GET https://services.api.unity.com/unity/editor/release/v1/releases?stream=LTS&limit=25`
//!
//! Response: `{ "total": N, "results": [{ "version": "2022.3.10f1", "shortRevision": "ff3792e53c62", ... }] }`

use serde::Deserialize;

use crate::http::HttpClient;

pub const UNITY_API_BASE: &str = "https://services.api.unity.com/unity/editor/release/v1/releases";
pub const UNITY_LTS_STREAM: &str =
    "https://services.api.unity.com/unity/editor/release/v1/releases?stream=LTS";

const PAGE_LIMIT: usize = 25;

#[derive(Debug, Deserialize)]
struct UnityRelease {
    version: String,
    #[serde(rename = "shortRevision")]
    short_revision: String,
}

#[derive(Debug, Deserialize)]
struct UnityReleasePage {
    #[allow(dead_code)]
    total: u64,
    results: Vec<UnityRelease>,
}

/// Update summary from the Unity3D datasource.
#[derive(Debug)]
pub struct Unity3dUpdateSummary {
    pub latest: Option<String>,
    pub latest_with_revision: Option<String>,
    pub update_available: bool,
}

/// Error from the Unity3D datasource.
#[derive(Debug, thiserror::Error)]
pub enum Unity3dError {
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("JSON parse error: {0}")]
    Parse(String),
}

/// Fetch the latest Unity LTS Editor version.
///
/// `with_revision` controls whether the returned `latest` string includes the
/// short hash: `"2022.3.10f1 (ff3792e53c62)"` vs `"2022.3.10f1"`.
pub async fn fetch_latest_lts(
    http: &HttpClient,
    with_revision: bool,
) -> Result<Unity3dUpdateSummary, Unity3dError> {
    // Fetch first page only — latest is at offset 0.
    let url = format!("{UNITY_LTS_STREAM}&limit={PAGE_LIMIT}&offset=0");
    let resp = http
        .get_retrying(&url)
        .await
        .map_err(|e| Unity3dError::Http(e.to_string()))?;
    let body = resp
        .text()
        .await
        .map_err(|e| Unity3dError::Http(e.to_string()))?;
    let page: UnityReleasePage =
        serde_json::from_str(&body).map_err(|e| Unity3dError::Parse(e.to_string()))?;

    let latest = page.results.first().map(|r| {
        if with_revision {
            format!("{} ({})", r.version, r.short_revision)
        } else {
            r.version.clone()
        }
    });

    Ok(Unity3dUpdateSummary {
        latest_with_revision: page
            .results
            .first()
            .map(|r| format!("{} ({})", r.version, r.short_revision)),
        latest: page.results.first().map(|r| r.version.clone()),
        update_available: latest.is_some(),
    })
}
