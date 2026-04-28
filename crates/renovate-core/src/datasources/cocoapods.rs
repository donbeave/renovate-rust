//! CocoaPods trunk datasource.
//!
//! Fetches pod versions from the CocoaPods trunk registry REST API.
//!
//! Renovate reference:
//! - `lib/modules/datasource/pod/index.ts` — `PodDatasource`
//! - API: `GET https://trunk.cocoapods.org/api/v1/pods/{name}`
//!
//! The trunk API response includes a `versions` array (newest first):
//! `{"versions":[{"name":"5.6.4","created_at":"..."},{"name":"5.6.3",...}]}`

use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::HttpClient;

pub const TRUNK_API: &str = "https://trunk.cocoapods.org/api/v1";

/// Errors from fetching CocoaPods trunk metadata.
#[derive(Debug, Error)]
pub enum CocoapodsError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
}

/// Input for a single pod lookup.
#[derive(Debug, Clone)]
pub struct PodDepInput {
    /// Pod name (may include subspec like `Firebase/Analytics`; trunk uses base name).
    pub name: String,
    pub current_value: String,
}

/// Update summary for a pod dependency.
#[derive(Debug, Clone)]
pub struct PodUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Per-dependency result from `fetch_updates_concurrent`.
#[derive(Debug)]
pub struct PodUpdateResult {
    pub name: String,
    pub summary: Result<PodUpdateSummary, CocoapodsError>,
}

#[derive(Debug, Deserialize)]
struct TrunkResponse {
    versions: Vec<TrunkVersion>,
}

#[derive(Debug, Deserialize)]
struct TrunkVersion {
    name: String,
}

/// Fetch the latest stable version of a pod.
///
/// For subspecs like `Firebase/Analytics`, uses the base name `Firebase`.
pub async fn fetch_latest(
    pod_name: &str,
    http: &HttpClient,
    api_base: &str,
) -> Result<Option<String>, CocoapodsError> {
    // Trunk uses base pod name, not subspec.
    let base_name = pod_name.split('/').next().unwrap_or(pod_name);
    let url = format!("{api_base}/pods/{base_name}");

    let resp = http.get_retrying(&url).await?;
    if resp.status().as_u16() == 404 {
        return Ok(None);
    }
    if !resp.status().is_success() {
        return Ok(None);
    }

    let body: TrunkResponse = resp.json().await.map_err(CocoapodsError::Json)?;

    // Versions are newest first; skip pre-releases (containing `-`).
    let latest = body
        .versions
        .into_iter()
        .find(|v| !v.name.contains('-'))
        .map(|v| v.name);

    Ok(latest)
}

/// Fetch update summaries for multiple pods concurrently.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[PodDepInput],
    api_base: &str,
    concurrency: usize,
) -> Vec<PodUpdateResult> {
    if deps.is_empty() {
        return Vec::new();
    }

    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<PodUpdateResult> = JoinSet::new();

    for dep in deps {
        let http = http.clone();
        let dep = dep.clone();
        let sem = Arc::clone(&sem);
        let api_base = api_base.to_owned();

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_update_summary(&dep, &http, &api_base).await;
            PodUpdateResult {
                name: dep.name.clone(),
                summary: result,
            }
        });
    }

    let mut results = Vec::with_capacity(deps.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok(r) => results.push(r),
            Err(join_err) => tracing::error!(%join_err, "cocoapods lookup task panicked"),
        }
    }
    results
}

async fn fetch_update_summary(
    dep: &PodDepInput,
    http: &HttpClient,
    api_base: &str,
) -> Result<PodUpdateSummary, CocoapodsError> {
    let latest = fetch_latest(&dep.name, http, api_base).await?;
    // Strip constraint operators to get the lower bound.
    let lower = lower_bound(&dep.current_value);
    let update_available = latest
        .as_deref()
        .is_some_and(|l| !lower.is_empty() && l != lower);
    Ok(PodUpdateSummary {
        current_value: dep.current_value.clone(),
        latest,
        update_available,
    })
}

fn lower_bound(constraint: &str) -> &str {
    constraint
        .trim()
        .trim_start_matches(['~', '>', '<', '=', '!', ' '])
        .split(',')
        .next()
        .unwrap_or("")
        .trim()
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    fn trunk_json(versions: &[&str]) -> String {
        let vers: Vec<serde_json::Value> = versions
            .iter()
            .map(|v| serde_json::json!({"name": v, "created_at": "2023-01-01"}))
            .collect();
        serde_json::json!({"name": "MyPod", "versions": vers}).to_string()
    }

    #[tokio::test]
    async fn fetch_latest_returns_newest_stable() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/pods/Alamofire"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(trunk_json(&["5.6.4", "5.6.3", "5.6.2"])),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("Alamofire", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, Some("5.6.4".to_owned()));
    }

    #[tokio::test]
    async fn fetch_latest_skips_prerelease() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/pods/MyLib"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string(trunk_json(&["2.0.0-beta1", "1.9.0"])),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("MyLib", &http, &server.uri()).await.unwrap();
        assert_eq!(result, Some("1.9.0".to_owned()));
    }

    #[tokio::test]
    async fn subspec_uses_base_name() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/pods/Firebase"))
            .respond_with(ResponseTemplate::new(200).set_body_string(trunk_json(&["10.5.0"])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("Firebase/Analytics", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, Some("10.5.0".to_owned()));
    }

    #[tokio::test]
    async fn fetch_latest_404_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/pods/NonExistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("NonExistent", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn lower_bound_pessimistic() {
        assert_eq!(lower_bound("~> 5.6"), "5.6");
        assert_eq!(lower_bound(">= 1.0.0"), "1.0.0");
        assert_eq!(lower_bound("5.6.4"), "5.6.4");
    }
}
