//! GitHub tags datasource.
//!
//! Fetches the list of tags from the GitHub REST API and returns the latest
//! version-like tag for a given `owner/repo`.  Used to look up GitHub Actions
//! versions (e.g. `actions/checkout@v4` → latest `v4.x.y` tag).
//!
//! Renovate reference:
//! - `lib/modules/datasource/github-tags/index.ts`
//! - `lib/modules/datasource/github-releases/index.ts`

use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::HttpClient;

const GITHUB_API_BASE: &str = "https://api.github.com";

/// Errors from fetching GitHub tags.
#[derive(Debug, Error)]
pub enum GithubTagsError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
}

/// Input for a single GitHub Actions dependency lookup.
#[derive(Debug, Clone)]
pub struct GithubActionsDepInput {
    /// `owner/repo` (lookup key for the tags API).
    pub dep_name: String,
    /// Currently pinned tag (e.g. `"v4"`).
    pub current_value: String,
}

/// Update summary for a GitHub Actions dependency.
#[derive(Debug, Clone)]
pub struct GithubActionsUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Per-dependency result from `fetch_updates_concurrent`.
#[derive(Debug)]
pub struct GithubActionsUpdateResult {
    pub dep_name: String,
    pub summary: Result<GithubActionsUpdateSummary, GithubTagsError>,
}

#[derive(Debug, Deserialize)]
struct GithubTag {
    name: String,
}

/// Fetch all version-like tags for `owner/repo` and return the latest one.
///
/// `api_base` defaults to `https://api.github.com`; pass a custom value for
/// GitHub Enterprise Server endpoints.
pub async fn fetch_latest_tag(
    owner_repo: &str,
    http: &HttpClient,
    api_base: &str,
) -> Result<Option<String>, GithubTagsError> {
    let url = format!("{api_base}/repos/{owner_repo}/tags?per_page=100");
    let resp = http.get_retrying(&url).await?;
    if resp.status().as_u16() == 404 || resp.status().as_u16() == 403 {
        return Ok(None);
    }
    if !resp.status().is_success() {
        return Ok(None);
    }

    let tags: Vec<GithubTag> = resp.json().await.map_err(GithubTagsError::Json)?;

    // Return the first tag that looks like a version (starts with v or digit).
    let latest = tags
        .into_iter()
        .map(|t| t.name)
        .find(|n| n.starts_with('v') || n.chars().next().is_some_and(|c| c.is_ascii_digit()));

    Ok(latest)
}

/// Fetch update summaries for multiple GitHub Actions dependencies concurrently.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[GithubActionsDepInput],
    api_base: &str,
    concurrency: usize,
) -> Vec<GithubActionsUpdateResult> {
    if deps.is_empty() {
        return Vec::new();
    }

    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<GithubActionsUpdateResult> = JoinSet::new();

    for dep in deps {
        let http = http.clone();
        let dep = dep.clone();
        let sem = Arc::clone(&sem);
        let api_base = api_base.to_owned();

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_update_summary(&dep, &http, &api_base).await;
            GithubActionsUpdateResult {
                dep_name: dep.dep_name.clone(),
                summary: result,
            }
        });
    }

    let mut results = Vec::with_capacity(deps.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok(r) => results.push(r),
            Err(join_err) => {
                tracing::error!(%join_err, "github-tags lookup task panicked")
            }
        }
    }
    results
}

async fn fetch_update_summary(
    dep: &GithubActionsDepInput,
    http: &HttpClient,
    api_base: &str,
) -> Result<GithubActionsUpdateSummary, GithubTagsError> {
    let latest = fetch_latest_tag(&dep.dep_name, http, api_base).await?;
    let update_available = latest
        .as_deref()
        .is_some_and(|l| l != dep.current_value && !dep.current_value.is_empty());
    Ok(GithubActionsUpdateSummary {
        current_value: dep.current_value.clone(),
        latest,
        update_available,
    })
}

/// Derive the GitHub API base URL from a platform endpoint string.
///
/// - `https://api.github.com` → `https://api.github.com`
/// - `https://github.example.com/api/v3` → `https://github.example.com/api/v3`
/// - `None` (unset) → `https://api.github.com`
pub fn api_base_from_endpoint(endpoint: Option<&str>) -> &str {
    match endpoint {
        None | Some("") => GITHUB_API_BASE,
        Some(ep) => ep.trim_end_matches('/'),
    }
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    fn make_http(token: &str) -> HttpClient {
        HttpClient::with_token(token).unwrap()
    }

    fn tags_json() -> &'static str {
        r#"[
            {"name":"v4.1.0","zipball_url":"","tarball_url":"","commit":{}},
            {"name":"v4.0.0","zipball_url":"","tarball_url":"","commit":{}},
            {"name":"v3.6.0","zipball_url":"","tarball_url":"","commit":{}}
        ]"#
    }

    #[tokio::test]
    async fn fetch_latest_tag_returns_first_version_like() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/actions/checkout/tags"))
            .respond_with(ResponseTemplate::new(200).set_body_string(tags_json()))
            .mount(&server)
            .await;

        let http = make_http("token");
        let result = fetch_latest_tag("actions/checkout", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, Some("v4.1.0".to_owned()));
    }

    #[tokio::test]
    async fn fetch_latest_tag_404_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/missing/repo/tags"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = make_http("token");
        let result = fetch_latest_tag("missing/repo", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn fetch_latest_tag_empty_list_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/actions/empty/tags"))
            .respond_with(ResponseTemplate::new(200).set_body_string("[]"))
            .mount(&server)
            .await;

        let http = make_http("token");
        let result = fetch_latest_tag("actions/empty", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn api_base_from_none() {
        assert_eq!(api_base_from_endpoint(None), GITHUB_API_BASE);
    }

    #[test]
    fn api_base_from_custom() {
        assert_eq!(
            api_base_from_endpoint(Some("https://github.example.com/api/v3")),
            "https://github.example.com/api/v3"
        );
    }

    #[test]
    fn api_base_trims_trailing_slash() {
        assert_eq!(
            api_base_from_endpoint(Some("https://api.github.com/")),
            "https://api.github.com"
        );
    }
}
