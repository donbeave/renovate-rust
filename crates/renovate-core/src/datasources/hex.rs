//! Hex.pm datasource for Elixir/Erlang packages.
//!
//! Fetches the latest stable version of a Hex.pm package using the public
//! REST API.
//!
//! Renovate reference:
//! - `lib/modules/datasource/hex/index.ts` — `HexDatasource`
//! - API: `GET https://hex.pm/api/packages/{name}`
//!
//! The `/api/packages/{name}` endpoint returns a JSON object with
//! `latest_stable_version` and `latest_version` fields. We use
//! `latest_stable_version` to avoid pre-release versions.

use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::HttpClient;

pub const HEX_API: &str = "https://hex.pm/api";

/// Errors from fetching Hex.pm metadata.
#[derive(Debug, Error)]
pub enum HexError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
}

/// Input for a single Hex.pm package lookup.
#[derive(Debug, Clone)]
pub struct HexDepInput {
    pub name: String,
    pub current_value: String,
}

/// Update summary for a Hex.pm dependency.
#[derive(Debug, Clone)]
pub struct HexUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Per-dependency result from `fetch_updates_concurrent`.
#[derive(Debug)]
pub struct HexUpdateResult {
    pub name: String,
    pub summary: Result<HexUpdateSummary, HexError>,
}

#[derive(Debug, Deserialize)]
struct HexPackage {
    latest_stable_version: Option<String>,
}

/// Fetch the latest stable version of a Hex.pm package.
pub async fn fetch_latest(
    package_name: &str,
    http: &HttpClient,
    api_base: &str,
) -> Result<Option<String>, HexError> {
    let url = format!("{api_base}/packages/{package_name}");

    let resp = http.get_retrying(&url).await?;
    if resp.status().as_u16() == 404 {
        return Ok(None);
    }
    if !resp.status().is_success() {
        return Ok(None);
    }

    let pkg: HexPackage = resp.json().await.map_err(HexError::Json)?;
    Ok(pkg.latest_stable_version)
}

/// Fetch update summaries for multiple Hex.pm packages concurrently.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[HexDepInput],
    api_base: &str,
    concurrency: usize,
) -> Vec<HexUpdateResult> {
    if deps.is_empty() {
        return Vec::new();
    }

    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<HexUpdateResult> = JoinSet::new();

    for dep in deps {
        let http = http.clone();
        let dep = dep.clone();
        let sem = Arc::clone(&sem);
        let api_base = api_base.to_owned();

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_update_summary(&dep, &http, &api_base).await;
            HexUpdateResult {
                name: dep.name.clone(),
                summary: result,
            }
        });
    }

    let mut results = Vec::with_capacity(deps.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok(r) => results.push(r),
            Err(join_err) => tracing::error!(%join_err, "hex.pm lookup task panicked"),
        }
    }
    results
}

async fn fetch_update_summary(
    dep: &HexDepInput,
    http: &HttpClient,
    api_base: &str,
) -> Result<HexUpdateSummary, HexError> {
    let latest = fetch_latest(&dep.name, http, api_base).await?;
    let s = crate::versioning::semver_generic::semver_update_summary(
        &dep.current_value,
        latest.as_deref(),
    );
    Ok(HexUpdateSummary {
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

    fn pkg_json(name: &str, stable: &str) -> String {
        serde_json::json!({
            "name": name,
            "latest_stable_version": stable,
            "latest_version": stable,
        })
        .to_string()
    }

    #[tokio::test]
    async fn fetch_latest_returns_stable_version() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/packages/phoenix"))
            .respond_with(ResponseTemplate::new(200).set_body_string(pkg_json("phoenix", "1.7.3")))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("phoenix", &http, &server.uri()).await.unwrap();
        assert_eq!(result, Some("1.7.3".to_owned()));
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

    #[tokio::test]
    async fn concurrent_fetch_returns_all() {
        let server = MockServer::start().await;
        for (name, ver) in [("phoenix", "1.7.3"), ("ecto", "3.10.1")] {
            Mock::given(method("GET"))
                .and(path(format!("/packages/{name}")))
                .respond_with(ResponseTemplate::new(200).set_body_string(pkg_json(name, ver)))
                .mount(&server)
                .await;
        }

        let http = HttpClient::new().unwrap();
        let deps = vec![
            HexDepInput {
                name: "phoenix".to_owned(),
                current_value: "~> 1.7.0".to_owned(),
            },
            HexDepInput {
                name: "ecto".to_owned(),
                current_value: "~> 3.10".to_owned(),
            },
        ];
        let results = fetch_updates_concurrent(&http, &deps, &server.uri(), 4).await;
        assert_eq!(results.len(), 2);

        let phoenix = results.iter().find(|r| r.name == "phoenix").unwrap();
        let s = phoenix.summary.as_ref().unwrap();
        assert_eq!(s.latest.as_deref(), Some("1.7.3"));
    }
}
