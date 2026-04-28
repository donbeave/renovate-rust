//! NuGet datasource.
//!
//! Fetches available versions from the NuGet flat-container API.
//!
//! Renovate reference:
//! - `lib/modules/datasource/nuget/index.ts`
//! - API: `https://api.nuget.org/v3-flatcontainer/{id}/index.json`

use std::collections::HashMap;
use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::HttpClient;

pub const NUGET_API: &str = "https://api.nuget.org/v3-flatcontainer";

/// Errors from fetching NuGet metadata.
#[derive(Debug, Error)]
pub enum NuGetError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
}

/// Input for a single NuGet package lookup.
#[derive(Debug, Clone)]
pub struct NuGetDepInput {
    pub package_id: String,
    pub current_value: String,
}

/// Update summary for a NuGet dependency.
#[derive(Debug, Clone)]
pub struct NuGetUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Per-dependency result from `fetch_updates_concurrent`.
#[derive(Debug)]
pub struct NuGetUpdateResult {
    pub package_id: String,
    pub summary: Result<NuGetUpdateSummary, NuGetError>,
}

#[derive(Debug, Deserialize)]
struct FlatContainerIndex {
    versions: Vec<String>,
}

/// Fetch the latest stable version of a NuGet package.
///
/// The NuGet flat-container API requires the package ID to be lowercase.
pub async fn fetch_latest(
    package_id: &str,
    http: &HttpClient,
    api_base: &str,
) -> Result<Option<String>, NuGetError> {
    let lower = package_id.to_ascii_lowercase();
    let url = format!("{api_base}/{lower}/index.json");

    let resp = http.get_retrying(&url).await?;
    if resp.status().as_u16() == 404 {
        return Ok(None);
    }
    if !resp.status().is_success() {
        return Ok(None);
    }

    let index: FlatContainerIndex = resp.json().await.map_err(NuGetError::Json)?;

    // Versions are in ascending order; return the last stable one.
    let latest = index
        .versions
        .iter()
        .rev()
        .find(|v| is_stable(v))
        .map(|v| v.to_owned());

    Ok(latest)
}

/// Fetch update summaries for multiple NuGet packages concurrently.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[NuGetDepInput],
    api_base: &str,
    concurrency: usize,
) -> Vec<NuGetUpdateResult> {
    if deps.is_empty() {
        return Vec::new();
    }

    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<NuGetUpdateResult> = JoinSet::new();

    for dep in deps {
        let http = http.clone();
        let dep = dep.clone();
        let sem = Arc::clone(&sem);
        let api_base = api_base.to_owned();

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_update_summary(&dep, &http, &api_base).await;
            NuGetUpdateResult {
                package_id: dep.package_id.clone(),
                summary: result,
            }
        });
    }

    let mut results = Vec::with_capacity(deps.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok(r) => results.push(r),
            Err(join_err) => tracing::error!(%join_err, "nuget lookup task panicked"),
        }
    }
    results
}

async fn fetch_update_summary(
    dep: &NuGetDepInput,
    http: &HttpClient,
    api_base: &str,
) -> Result<NuGetUpdateSummary, NuGetError> {
    let latest = fetch_latest(&dep.package_id, http, api_base).await?;
    let s = crate::versioning::nuget::nuget_update_summary(&dep.current_value, latest.as_deref());
    Ok(NuGetUpdateSummary {
        current_value: s.current_value,
        latest: s.latest,
        update_available: s.update_available,
    })
}

/// Cached NuGet latest version entry.
pub type NuGetLatestEntry = Option<String>;

/// Fetch the latest stable version for a batch of unique NuGet package IDs.
pub async fn fetch_latest_batch(
    http: &HttpClient,
    package_ids: &[String],
    api_base: &str,
    concurrency: usize,
) -> HashMap<String, NuGetLatestEntry> {
    if package_ids.is_empty() {
        return HashMap::new();
    }

    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<(String, NuGetLatestEntry)> = JoinSet::new();

    for id in package_ids {
        let http = http.clone();
        let id = id.clone();
        let api_base = api_base.to_owned();
        let sem = Arc::clone(&sem);

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let latest = fetch_latest(&id, &http, &api_base).await.ok().flatten();
            (id, latest)
        });
    }

    let mut cache = HashMap::with_capacity(package_ids.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok((id, latest)) => {
                cache.insert(id, latest);
            }
            Err(join_err) => tracing::error!(%join_err, "nuget batch fetch task panicked"),
        }
    }
    cache
}

/// Compute a `NuGetUpdateSummary` from a pre-fetched latest version entry.
pub fn summary_from_cache(current_value: &str, latest: NuGetLatestEntry) -> NuGetUpdateSummary {
    let s = crate::versioning::nuget::nuget_update_summary(current_value, latest.as_deref());
    NuGetUpdateSummary {
        current_value: s.current_value,
        latest: s.latest,
        update_available: s.update_available,
    }
}

/// Returns `true` when the version string looks like a stable NuGet release.
///
/// Pre-release versions contain a `-` after the numeric version part.
fn is_stable(version: &str) -> bool {
    // NuGet pre-release versions have a hyphen-separated label: `1.2.3-preview1`
    !version.contains('-')
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    #[test]
    fn is_stable_accepts_release() {
        assert!(is_stable("13.0.3"));
        assert!(is_stable("3.1.1"));
        assert!(is_stable("4.0.0"));
    }

    #[test]
    fn is_stable_rejects_prerelease() {
        assert!(!is_stable("13.0.3-preview1"));
        assert!(!is_stable("1.0.0-alpha"));
        assert!(!is_stable("1.0.0-rc.1"));
    }

    #[tokio::test]
    async fn fetch_latest_returns_latest_stable() {
        let server = MockServer::start().await;
        let body = serde_json::json!({
            "versions": ["12.0.0", "13.0.0", "13.0.1-preview1", "13.0.3"]
        })
        .to_string();
        Mock::given(method("GET"))
            .and(path("/newtonsoft.json/index.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("Newtonsoft.Json", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, Some("13.0.3".to_owned()));
    }

    #[tokio::test]
    async fn fetch_latest_404_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/missing.package/index.json"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("Missing.Package", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn package_id_lowercased_in_url() {
        let server = MockServer::start().await;
        // URL should use lowercase even if package_id has capitals.
        Mock::given(method("GET"))
            .and(path("/microsoft.aspnetcore.mvc/index.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"versions":["1.1.3"]}"#))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("Microsoft.AspNetCore.Mvc", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, Some("1.1.3".to_owned()));
    }
}
