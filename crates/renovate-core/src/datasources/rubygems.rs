//! RubyGems datasource.
//!
//! Fetches available gem versions from the RubyGems REST API.
//!
//! Renovate reference:
//! - `lib/modules/datasource/rubygems/index.ts` — `RubygemsDatasource`
//! - API: `GET https://rubygems.org/api/v1/versions/{gemname}.json`
//!
//! The `/api/v1/versions/{gem}.json` endpoint returns an array of version
//! objects, newest first, each with a `"number"` field and `"prerelease"` bool.
//! We pick the first entry where `prerelease == false` as the latest stable
//! version.

use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::HttpClient;

pub const RUBYGEMS_API: &str = "https://rubygems.org/api/v1";

/// Errors from fetching RubyGems metadata.
#[derive(Debug, Error)]
pub enum RubyGemsError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
}

/// Input for a single gem lookup.
#[derive(Debug, Clone)]
pub struct GemDepInput {
    pub name: String,
    pub current_value: String,
}

/// Update summary for a gem dependency.
#[derive(Debug, Clone)]
pub struct GemUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
    /// ISO 8601 timestamp when the latest stable version was published.
    /// Used for `minimumReleaseAge` evaluation.
    pub release_timestamp: Option<String>,
}

/// Per-dependency result from `fetch_updates_concurrent`.
#[derive(Debug)]
pub struct GemUpdateResult {
    pub name: String,
    pub summary: Result<GemUpdateSummary, RubyGemsError>,
}

#[derive(Debug, Deserialize)]
struct GemVersion {
    number: String,
    prerelease: bool,
    /// ISO 8601 creation timestamp, e.g. `"2024-01-15T10:30:00.000Z"`.
    /// Available from `/api/v1/versions/{gem}.json`.
    created_at: Option<String>,
}

/// Fetch the latest stable version of a gem and its release timestamp.
pub async fn fetch_latest(
    gem_name: &str,
    http: &HttpClient,
    api_base: &str,
) -> Result<Option<(String, Option<String>)>, RubyGemsError> {
    let url = format!("{api_base}/versions/{gem_name}.json");

    let resp = http.get_retrying(&url).await?;
    if resp.status().as_u16() == 404 {
        return Ok(None);
    }
    if !resp.status().is_success() {
        return Ok(None);
    }

    let versions: Vec<GemVersion> = resp.json().await.map_err(RubyGemsError::Json)?;

    // Versions are newest-first; return first stable entry.
    let latest = versions
        .into_iter()
        .find(|v| !v.prerelease)
        .map(|v| (v.number, v.created_at));

    Ok(latest)
}

/// Fetch update summaries for multiple gems concurrently.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[GemDepInput],
    api_base: &str,
    concurrency: usize,
) -> Vec<GemUpdateResult> {
    if deps.is_empty() {
        return Vec::new();
    }

    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<GemUpdateResult> = JoinSet::new();

    for dep in deps {
        let http = http.clone();
        let dep = dep.clone();
        let sem = Arc::clone(&sem);
        let api_base = api_base.to_owned();

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_update_summary(&dep, &http, &api_base).await;
            GemUpdateResult {
                name: dep.name.clone(),
                summary: result,
            }
        });
    }

    let mut results = Vec::with_capacity(deps.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok(r) => results.push(r),
            Err(join_err) => tracing::error!(%join_err, "rubygems lookup task panicked"),
        }
    }
    results
}

async fn fetch_update_summary(
    dep: &GemDepInput,
    http: &HttpClient,
    api_base: &str,
) -> Result<GemUpdateSummary, RubyGemsError> {
    let result = fetch_latest(&dep.name, http, api_base).await?;
    let (latest_version, release_timestamp) = result
        .map(|(v, ts)| (Some(v), ts))
        .unwrap_or((None, None));
    let s = crate::versioning::semver_generic::semver_update_summary(
        &dep.current_value,
        latest_version.as_deref(),
    );
    Ok(GemUpdateSummary {
        current_value: s.current_value,
        latest: s.latest,
        update_available: s.update_available,
        release_timestamp,
    })
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    fn versions_json(versions: &[(&str, bool)]) -> String {
        let items: Vec<String> = versions
            .iter()
            .map(|(v, pre)| {
                serde_json::json!({"number": v, "prerelease": pre, "platform": "ruby"}).to_string()
            })
            .collect();
        format!("[{}]", items.join(","))
    }

    #[tokio::test]
    async fn fetch_latest_returns_newest_stable() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/versions/rails.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(versions_json(&[
                ("7.1.0.rc1", true),
                ("7.0.8", false),
                ("7.0.7", false),
            ])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("rails", &http, &server.uri()).await.unwrap();
        assert_eq!(result.map(|(v, _)| v), Some("7.0.8".to_owned()));
    }

    #[tokio::test]
    async fn fetch_latest_skips_prerelease() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/versions/mylib.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(versions_json(&[
                ("2.0.0.alpha1", true),
                ("2.0.0.beta2", true),
            ])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("mylib", &http, &server.uri()).await.unwrap();
        assert_eq!(result, None); // no stable version available
    }

    #[tokio::test]
    async fn fetch_latest_404_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/versions/nonexistent.json"))
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

        for (gem, ver) in [("rails", "7.0.8"), ("devise", "4.9.3")] {
            Mock::given(method("GET"))
                .and(path(format!("/versions/{gem}.json")))
                .respond_with(
                    ResponseTemplate::new(200).set_body_string(versions_json(&[(ver, false)])),
                )
                .mount(&server)
                .await;
        }

        let http = HttpClient::new().unwrap();
        let deps = vec![
            GemDepInput {
                name: "rails".to_owned(),
                current_value: "~> 7.0.4".to_owned(),
            },
            GemDepInput {
                name: "devise".to_owned(),
                current_value: "~> 4.9".to_owned(),
            },
        ];

        let results = fetch_updates_concurrent(&http, &deps, &server.uri(), 4).await;
        assert_eq!(results.len(), 2);

        let rails = results.iter().find(|r| r.name == "rails").unwrap();
        let s = rails.summary.as_ref().unwrap();
        assert_eq!(s.latest.as_deref(), Some("7.0.8"));
        assert!(s.update_available);
    }
}
