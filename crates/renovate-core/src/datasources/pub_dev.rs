//! Dart pub.dev datasource.
//!
//! Fetches the latest version of a Dart package from the pub.dev API.
//!
//! Renovate reference:
//! - `lib/modules/datasource/dart/index.ts` — `DartDatasource`
//! - API: `https://pub.dev/api/packages/{name}`

use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::HttpClient;

pub const PUB_DEV_API: &str = "https://pub.dev/api";

/// Errors from fetching pub.dev metadata.
#[derive(Debug, Error)]
pub enum PubError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
}

/// Input for a single pub.dev package lookup.
#[derive(Debug, Clone)]
pub struct PubDepInput {
    pub name: String,
    pub current_value: String,
}

/// Update summary for a pub dependency.
#[derive(Debug, Clone)]
pub struct PubUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Per-dependency result from `fetch_updates_concurrent`.
#[derive(Debug)]
pub struct PubUpdateResult {
    pub name: String,
    pub summary: Result<PubUpdateSummary, PubError>,
}

#[derive(Debug, Deserialize)]
struct PubPackage {
    latest: PubVersion,
}

#[derive(Debug, Deserialize)]
struct PubVersion {
    version: String,
}

/// Fetch the latest stable version of a pub.dev package.
pub async fn fetch_latest(
    package_name: &str,
    http: &HttpClient,
    api_base: &str,
) -> Result<Option<String>, PubError> {
    let url = format!("{api_base}/packages/{package_name}");

    let resp = http.get_retrying(&url).await?;
    if resp.status().as_u16() == 404 {
        return Ok(None);
    }
    if !resp.status().is_success() {
        return Ok(None);
    }

    let pkg: PubPackage = resp.json().await.map_err(PubError::Json)?;
    Ok(Some(pkg.latest.version))
}

/// Fetch update summaries for multiple pub packages concurrently.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[PubDepInput],
    api_base: &str,
    concurrency: usize,
) -> Vec<PubUpdateResult> {
    if deps.is_empty() {
        return Vec::new();
    }

    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<PubUpdateResult> = JoinSet::new();

    for dep in deps {
        let http = http.clone();
        let dep = dep.clone();
        let sem = Arc::clone(&sem);
        let api_base = api_base.to_owned();

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_update_summary(&dep, &http, &api_base).await;
            PubUpdateResult {
                name: dep.name.clone(),
                summary: result,
            }
        });
    }

    let mut results = Vec::with_capacity(deps.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok(r) => results.push(r),
            Err(join_err) => tracing::error!(%join_err, "pub.dev lookup task panicked"),
        }
    }
    results
}

async fn fetch_update_summary(
    dep: &PubDepInput,
    http: &HttpClient,
    api_base: &str,
) -> Result<PubUpdateSummary, PubError> {
    let latest = fetch_latest(&dep.name, http, api_base).await?;
    let s = crate::versioning::semver_generic::semver_update_summary(
        &dep.current_value,
        latest.as_deref(),
    );
    Ok(PubUpdateSummary {
        current_value: s.current_value,
        latest: s.latest,
        update_available: s.update_available,
    })
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    fn http_pkg_json(name: &str, version: &str) -> String {
        serde_json::json!({
            "name": name,
            "latest": {"version": version, "pubspec": {}},
            "versions": [{"version": version}]
        })
        .to_string()
    }

    #[tokio::test]
    async fn fetch_latest_returns_version() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/packages/http"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string(http_pkg_json("http", "0.13.6")),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("http", &http, &server.uri()).await.unwrap();
        assert_eq!(result, Some("0.13.6".to_owned()));
    }

    #[tokio::test]
    async fn fetch_latest_404_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/packages/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("nonexistent", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, None);
    }
}
