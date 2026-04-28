//! GitLab tags datasource.
//!
//! Fetches repository tags from the GitLab REST API and returns the latest
//! version-like tag for a given `namespace/project` slug.
//!
//! Renovate reference:
//! - `lib/modules/datasource/gitlab-tags/index.ts` — `GitlabTagsDatasource`
//! - API: `GET {host}/api/v4/projects/{url_encoded_path}/repository/tags?per_page=100`
//!
//! The `{url_encoded_path}` encodes slashes as `%2F`, so
//! `myorg/myrepo` → `myorg%2Fmyrepo`.

use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::HttpClient;

pub const GITLAB_API: &str = "https://gitlab.com";

/// Errors from fetching GitLab tags.
#[derive(Debug, Error)]
pub enum GitlabTagsError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
}

/// Input for a single GitLab tags lookup.
#[derive(Debug, Clone)]
pub struct GitlabTagsDepInput {
    /// `namespace/project` slug (e.g. `myorg/myrepo`).
    pub dep_name: String,
    /// Currently pinned tag (e.g. `"1.0.0"`).
    pub current_value: String,
}

/// Update summary for a GitLab tags dependency.
#[derive(Debug, Clone)]
pub struct GitlabTagsUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Per-dependency result from `fetch_updates_concurrent`.
#[derive(Debug)]
pub struct GitlabTagsUpdateResult {
    pub dep_name: String,
    pub summary: Result<GitlabTagsUpdateSummary, GitlabTagsError>,
}

#[derive(Debug, Deserialize)]
struct GitlabTag {
    name: String,
}

/// Fetch all version-like tags for `namespace/project` and return the latest.
///
/// `api_base` defaults to `https://gitlab.com`; pass a custom value for
/// GitLab self-hosted instances.
pub async fn fetch_latest_tag(
    owner_repo: &str,
    http: &HttpClient,
    api_base: &str,
) -> Result<Option<String>, GitlabTagsError> {
    let encoded = owner_repo.replace('/', "%2F");
    let url = format!("{api_base}/api/v4/projects/{encoded}/repository/tags?per_page=100");

    let resp = http.get_retrying(&url).await?;
    if resp.status().as_u16() == 404 || resp.status().as_u16() == 403 {
        return Ok(None);
    }
    if !resp.status().is_success() {
        return Ok(None);
    }

    let tags: Vec<GitlabTag> = resp.json().await.map_err(GitlabTagsError::Json)?;

    // Return the first tag that looks like a version (starts with `v` or a digit).
    let latest = tags
        .into_iter()
        .find(|t| {
            let n = t.name.as_str();
            n.starts_with('v')
                && n.len() > 1
                && n.chars().nth(1).is_some_and(|c| c.is_ascii_digit())
                || n.chars().next().is_some_and(|c| c.is_ascii_digit())
        })
        .map(|t| t.name.trim_start_matches('v').to_owned());

    Ok(latest)
}

/// Fetch update summaries for multiple GitLab deps concurrently.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[GitlabTagsDepInput],
    api_base: &str,
    concurrency: usize,
) -> Vec<GitlabTagsUpdateResult> {
    if deps.is_empty() {
        return Vec::new();
    }

    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<GitlabTagsUpdateResult> = JoinSet::new();

    for dep in deps {
        let http = http.clone();
        let dep = dep.clone();
        let sem = Arc::clone(&sem);
        let api_base = api_base.to_owned();

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_update_summary(&dep, &http, &api_base).await;
            GitlabTagsUpdateResult {
                dep_name: dep.dep_name.clone(),
                summary: result,
            }
        });
    }

    let mut results = Vec::with_capacity(deps.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok(r) => results.push(r),
            Err(join_err) => tracing::error!(%join_err, "gitlab tags lookup task panicked"),
        }
    }
    results
}

async fn fetch_update_summary(
    dep: &GitlabTagsDepInput,
    http: &HttpClient,
    api_base: &str,
) -> Result<GitlabTagsUpdateSummary, GitlabTagsError> {
    let latest = fetch_latest_tag(&dep.dep_name, http, api_base).await?;
    let current_stripped = dep.current_value.trim_start_matches('v');
    let update_available = latest
        .as_deref()
        .is_some_and(|l| !current_stripped.is_empty() && l != current_stripped);
    Ok(GitlabTagsUpdateSummary {
        current_value: dep.current_value.clone(),
        latest,
        update_available,
    })
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    fn tags_json(tags: &[&str]) -> String {
        let items: Vec<serde_json::Value> = tags
            .iter()
            .map(|t| serde_json::json!({"name": t, "commit": {"id": "abc123"}}))
            .collect();
        serde_json::to_string(&items).unwrap()
    }

    #[tokio::test]
    async fn fetch_latest_returns_first_version_tag() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v4/projects/myorg%2Fmyrepo/repository/tags"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(tags_json(&["v2.1.0", "v2.0.0", "v1.9.0"])),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest_tag("myorg/myrepo", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, Some("2.1.0".to_owned()));
    }

    #[tokio::test]
    async fn fetch_latest_skips_non_version_tags() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v4/projects/myorg%2Fmyrepo/repository/tags"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(tags_json(&["latest", "stable", "v1.0.0"])),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest_tag("myorg/myrepo", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, Some("1.0.0".to_owned()));
    }

    #[tokio::test]
    async fn fetch_latest_404_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v4/projects/myorg%2Fnonexistent/repository/tags"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest_tag("myorg/nonexistent", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn update_available_when_newer() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v4/projects/myorg%2Fmyrepo/repository/tags"))
            .respond_with(ResponseTemplate::new(200).set_body_string(tags_json(&["v2.0.0"])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let deps = vec![GitlabTagsDepInput {
            dep_name: "myorg/myrepo".to_owned(),
            current_value: "1.0.0".to_owned(),
        }];
        let results = fetch_updates_concurrent(&http, &deps, &server.uri(), 4).await;
        assert_eq!(results.len(), 1);
        let s = results[0].summary.as_ref().unwrap();
        assert!(s.update_available);
        assert_eq!(s.latest.as_deref(), Some("2.0.0"));
    }
}
