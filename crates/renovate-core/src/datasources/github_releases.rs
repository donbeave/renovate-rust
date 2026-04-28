//! GitHub Releases datasource.
//!
//! Fetches the latest stable release of a GitHub repository using the
//! `/repos/{owner}/{repo}/releases` REST API.  Unlike the tags API, releases
//! carry explicit `prerelease` and `draft` flags so stability filtering is
//! precise.
//!
//! Renovate reference:
//! - `lib/modules/datasource/github-releases/index.ts`
//! - API: `GET https://api.github.com/repos/{owner}/{repo}/releases?per_page=100`

use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::HttpClient;

pub const GITHUB_API: &str = "https://api.github.com";

/// Errors from fetching GitHub releases.
#[derive(Debug, Error)]
pub enum GithubReleasesError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
}

/// Input for a single GitHub release lookup.
#[derive(Debug, Clone)]
pub struct GithubReleasesDepInput {
    /// `owner/repo` string (e.g. `"hashicorp/terraform"`).
    pub dep_name: String,
    /// Currently pinned version (e.g. `"1.6.0"`).
    pub current_value: String,
}

/// Update summary for a GitHub release dependency.
#[derive(Debug, Clone)]
pub struct GithubReleasesUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Per-dependency result from `fetch_updates_concurrent`.
#[derive(Debug)]
pub struct GithubReleasesUpdateResult {
    pub dep_name: String,
    pub summary: Result<GithubReleasesUpdateSummary, GithubReleasesError>,
}

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
    prerelease: bool,
    draft: bool,
}

/// Fetch the latest stable (non-prerelease, non-draft) release tag for `owner/repo`.
pub async fn fetch_latest_release(
    owner_repo: &str,
    http: &HttpClient,
    api_base: &str,
) -> Result<Option<String>, GithubReleasesError> {
    let url = format!("{api_base}/repos/{owner_repo}/releases?per_page=100");

    let resp = http.get_retrying(&url).await?;
    if resp.status().as_u16() == 404 {
        return Ok(None);
    }
    if !resp.status().is_success() {
        return Ok(None);
    }

    let releases: Vec<Release> = resp.json().await.map_err(GithubReleasesError::Json)?;

    // Releases are newest-first; return the first stable one.
    let latest = releases
        .into_iter()
        .find(|r| !r.prerelease && !r.draft)
        .map(|r| r.tag_name);

    Ok(latest)
}

/// Fetch update summaries for multiple GitHub release dependencies concurrently.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[GithubReleasesDepInput],
    api_base: &str,
    concurrency: usize,
) -> Vec<GithubReleasesUpdateResult> {
    if deps.is_empty() {
        return Vec::new();
    }

    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<GithubReleasesUpdateResult> = JoinSet::new();

    for dep in deps {
        let http = http.clone();
        let dep = dep.clone();
        let sem = Arc::clone(&sem);
        let api_base = api_base.to_owned();

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_update_summary(&dep, &http, &api_base).await;
            GithubReleasesUpdateResult {
                dep_name: dep.dep_name.clone(),
                summary: result,
            }
        });
    }

    let mut results = Vec::with_capacity(deps.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok(r) => results.push(r),
            Err(join_err) => tracing::error!(%join_err, "github-releases lookup task panicked"),
        }
    }
    results
}

async fn fetch_update_summary(
    dep: &GithubReleasesDepInput,
    http: &HttpClient,
    api_base: &str,
) -> Result<GithubReleasesUpdateSummary, GithubReleasesError> {
    let latest_tag = fetch_latest_release(&dep.dep_name, http, api_base).await?;
    let s = crate::versioning::semver_generic::semver_update_summary(
        &dep.current_value,
        latest_tag.as_deref(),
    );
    Ok(GithubReleasesUpdateSummary {
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

    fn releases_json(releases: &[(&str, bool, bool)]) -> String {
        let items: Vec<serde_json::Value> = releases
            .iter()
            .map(|(tag, pre, draft)| {
                serde_json::json!({
                    "tag_name": tag,
                    "prerelease": pre,
                    "draft": draft,
                    "name": tag,
                })
            })
            .collect();
        serde_json::Value::Array(items).to_string()
    }

    #[tokio::test]
    async fn fetch_latest_returns_first_stable() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/hashicorp/terraform/releases"))
            .respond_with(ResponseTemplate::new(200).set_body_string(releases_json(&[
                ("v1.7.0-rc1", true, false),
                ("v1.6.6", false, false),
                ("v1.6.5", false, false),
            ])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest_release("hashicorp/terraform", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, Some("v1.6.6".to_owned()));
    }

    #[tokio::test]
    async fn skips_drafts() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/releases"))
            .respond_with(ResponseTemplate::new(200).set_body_string(releases_json(&[
                ("v2.0.0", false, true),
                ("v1.9.0", false, false),
            ])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest_release("owner/repo", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, Some("v1.9.0".to_owned()));
    }

    #[tokio::test]
    async fn fetch_latest_404_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/missing/repo/releases"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest_release("missing/repo", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, None);
    }
}
