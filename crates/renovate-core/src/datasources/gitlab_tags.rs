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

#[derive(Debug, Deserialize)]
struct GitlabTagFull {
    name: String,
    commit: Option<GitlabTagCommit>,
}

#[derive(Debug, Deserialize)]
struct GitlabTagCommit {
    created_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GitlabCommitResponse {
    id: Option<String>,
}

pub const DEFAULT_REGISTRY_URL: &str = "https://gitlab.com";

/// A single release entry from `fetch_releases`.
#[derive(Debug, Clone, PartialEq)]
pub struct GitlabTagRelease {
    pub version: String,
    pub git_ref: String,
    pub release_timestamp: Option<String>,
}

/// Result of `fetch_releases`.
#[derive(Debug, Clone)]
pub struct GitlabTagsReleasesResult {
    pub releases: Vec<GitlabTagRelease>,
    pub source_url: Option<String>,
}

/// Strip `/api/v4` suffix from a registry URL to get the GitLab instance host.
///
/// Mirrors `getDepHost` from `gitlab-tags/util.ts`.
pub fn get_dep_host(registry_url: &str) -> String {
    let trimmed = registry_url.trim_end_matches('/');
    if let Some(stripped) = trimmed.strip_suffix("/api/v4") {
        stripped.to_owned()
    } else {
        trimmed.to_owned()
    }
}

/// Build the source URL for a GitLab repository.
///
/// Mirrors `getSourceUrl` from `gitlab-tags/util.ts`.
pub fn get_source_url(package_name: &str, registry_url: Option<&str>) -> String {
    let dep_host = get_dep_host(registry_url.unwrap_or(DEFAULT_REGISTRY_URL));
    format!("{dep_host}/{package_name}")
}

/// Fetch all tags for `package_name` from a GitLab instance.
///
/// `registry_url` may be `https://gitlab.com`, `https://host/api/v4/`, or
/// `https://host/subpath` (no `/api/v4` suffix).
///
/// Returns `Ok(None)` for 4xx. Returns `Err` for 5xx or network errors.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<GitlabTagsReleasesResult>, GitlabTagsError> {
    let dep_host = get_dep_host(registry_url);
    let encoded = package_name.replace('/', "%2F");
    let url = format!("{dep_host}/api/v4/projects/{encoded}/repository/tags?per_page=100");

    let resp = http.get_retrying(&url).await?;
    if !resp.status().is_success() {
        return Ok(None);
    }

    let tags: Vec<GitlabTagFull> = resp.json().await.map_err(GitlabTagsError::Json)?;

    let source_url = Some(format!("{dep_host}/{package_name}"));

    let releases = tags
        .into_iter()
        .map(|t| GitlabTagRelease {
            git_ref: t.name.clone(),
            release_timestamp: t.commit.and_then(|c| c.created_at),
            version: t.name,
        })
        .collect();

    Ok(Some(GitlabTagsReleasesResult { releases, source_url }))
}

/// Fetch the commit digest for `package_name`.
///
/// - `new_value = None` → latest commit hash (`commits?per_page=1`)
/// - `new_value = Some(branch)` → specific branch/ref hash
///
/// Returns `None` on 4xx, empty results, or any error.
pub async fn fetch_digest(
    registry_url: &str,
    package_name: &str,
    new_value: Option<&str>,
    http: &HttpClient,
) -> Option<String> {
    let dep_host = get_dep_host(registry_url);
    let encoded = package_name.replace('/', "%2F");

    if let Some(branch) = new_value {
        let url =
            format!("{dep_host}/api/v4/projects/{encoded}/repository/commits/{branch}");
        let resp = http.get_retrying(&url).await.ok()?;
        if !resp.status().is_success() {
            return None;
        }
        let commit: GitlabCommitResponse = resp.json().await.ok()?;
        commit.id.filter(|id| !id.is_empty())
    } else {
        let url = format!("{dep_host}/api/v4/projects/{encoded}/repository/commits?per_page=1");
        let resp = http.get_retrying(&url).await.ok()?;
        if !resp.status().is_success() {
            return None;
        }
        let commits: Vec<GitlabCommitResponse> = resp.json().await.ok()?;
        commits.into_iter().next()?.id.filter(|id| !id.is_empty())
    }
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

    // Ported: "returns null for 404" — lib/modules/datasource/pod/index.spec.ts line 60
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

    // Ported: "returns tags from custom registry" — datasource/gitlab-tags/index.spec.ts line 9
    #[tokio::test]
    async fn returns_tags_from_custom_registry() {
        let server = MockServer::start().await;
        let body = serde_json::json!([
            {"name": "v1.0.0", "commit": {"created_at": "2020-03-04T12:01:37.000-06:00"}},
            {"name": "v1.1.0", "commit": {}},
            {"name": "v1.1.1"}
        ]);
        Mock::given(method("GET"))
            .and(path("/api/v4/projects/some%2Fdep2/repository/tags"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        // registry_url = "{server}/api/v4/" → dep_host = "{server}"
        let registry_url = format!("{}/api/v4/", server.uri());
        let result = fetch_releases(&registry_url, "some/dep2", &http).await.unwrap().unwrap();

        assert_eq!(result.releases.len(), 3);
        assert_eq!(result.releases[0].version, "v1.0.0");
        assert_eq!(result.releases[0].git_ref, "v1.0.0");
        assert!(result.releases[0].release_timestamp.is_some());
        assert!(result.releases[1].release_timestamp.is_none());
        assert!(result.releases[2].release_timestamp.is_none());
    }

    // Ported: "returns tags from custom registry in sub path" — datasource/gitlab-tags/index.spec.ts line 38
    #[tokio::test]
    async fn returns_tags_from_custom_registry_in_sub_path() {
        let server = MockServer::start().await;
        let body = serde_json::json!([
            {"name": "v1.0.0", "commit": {"created_at": "2020-03-04T12:01:37.000-06:00"}},
            {"name": "v1.1.0", "commit": {}},
            {"name": "v1.1.1"}
        ]);
        Mock::given(method("GET"))
            .and(path("/gitlab/api/v4/projects/some%2Fdep2/repository/tags"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        // registry_url = "{server}/gitlab" → dep_host = "{server}/gitlab"
        let registry_url = format!("{}/gitlab", server.uri());
        let result = fetch_releases(&registry_url, "some/dep2", &http).await.unwrap().unwrap();

        assert_eq!(result.releases.len(), 3);
    }

    // Ported: "returns tags with default registry" — datasource/gitlab-tags/index.spec.ts line 67
    #[tokio::test]
    async fn returns_tags_with_default_registry() {
        let server = MockServer::start().await;
        let body = serde_json::json!([
            {"name": "v1.0.0"},
            {"name": "v1.1.0"}
        ]);
        Mock::given(method("GET"))
            .and(path("/api/v4/projects/some%2Fdep2/repository/tags"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "some/dep2", &http).await.unwrap().unwrap();

        assert_eq!(result.releases.len(), 2);
    }

    // Ported: "returns commits from gitlab installation" — datasource/gitlab-tags/index.spec.ts line 83
    #[tokio::test]
    async fn returns_commits_from_gitlab_installation() {
        let server = MockServer::start().await;
        let digest = "abcd00001234";
        let body = serde_json::json!([{"id": digest}]);
        Mock::given(method("GET"))
            .and(path("/api/v4/projects/some%2Fdep2/repository/commits"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let registry_url = format!("{}/api/v4/", server.uri());
        let result = fetch_digest(&registry_url, "some/dep2", None, &http).await;

        assert_eq!(result.as_deref(), Some(digest));
    }

    // Ported: "returns commits from gitlab installation for a specific branch" — datasource/gitlab-tags/index.spec.ts line 102
    #[tokio::test]
    async fn returns_commits_for_specific_branch() {
        let server = MockServer::start().await;
        let digest = "abcd00001234";
        let body = serde_json::json!({"id": digest});
        Mock::given(method("GET"))
            .and(path("/api/v4/projects/some%2Fdep2/repository/commits/branch"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let registry_url = format!("{}/api/v4/", server.uri());
        let result = fetch_digest(&registry_url, "some/dep2", Some("branch"), &http).await;

        assert_eq!(result.as_deref(), Some(digest));
    }

    // Ported: "returns null from gitlab installation with no commits" — datasource/gitlab-tags/index.spec.ts line 122
    #[tokio::test]
    async fn returns_null_with_no_commits() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v4/projects/some%2Fdep2/repository/commits"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&serde_json::json!([])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let registry_url = format!("{}/api/v4/", server.uri());
        let result = fetch_digest(&registry_url, "some/dep2", None, &http).await;

        assert!(result.is_none());
    }

    // Ported: "returns null from gitlab installation with unknown branch" — datasource/gitlab-tags/index.spec.ts line 135
    #[tokio::test]
    async fn returns_null_for_unknown_branch() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v4/projects/some%2Fdep2/repository/commits/unknown-branch"))
            .respond_with(ResponseTemplate::new(404).set_body_string("}"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let registry_url = format!("{}/api/v4/", server.uri());
        let result = fetch_digest(&registry_url, "some/dep2", Some("unknown-branch"), &http).await;

        assert!(result.is_none());
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

    // Ported: "works" — datasource/gitlab-tags/util.spec.ts line 5
    #[test]
    fn get_dep_host_works() {
        assert_eq!(get_dep_host("https://gitlab.com"), "https://gitlab.com");
        assert_eq!(
            get_dep_host("https://gitlab.domain.test/api/v4"),
            "https://gitlab.domain.test"
        );
        assert_eq!(
            get_dep_host("https://domain.test/gitlab/api/v4"),
            "https://domain.test/gitlab"
        );
    }

    // Ported: "works" — datasource/gitlab-tags/util.spec.ts line 17
    #[test]
    fn get_source_url_works() {
        assert_eq!(
            get_source_url("some/repo", None),
            "https://gitlab.com/some/repo"
        );
        assert_eq!(
            get_source_url("some/repo", Some("https://gitlab.domain.test/api/v4")),
            "https://gitlab.domain.test/some/repo"
        );
    }
}
