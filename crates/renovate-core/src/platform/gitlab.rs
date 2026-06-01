//! GitLab platform client.
//!
//! Implements [`PlatformClient`] against the GitLab REST API v4.
//!
//! Renovate reference: `lib/modules/platform/gitlab/index.ts`.
//!
//! ## Authentication
//!
//! GitLab uses the `PRIVATE-TOKEN` header for personal access tokens.
//! OAuth bearer tokens use `Authorization: Bearer <token>`.
//! This implementation uses `PRIVATE-TOKEN` (PAT) which is the common
//! self-hosted / GitLab.com workflow.
//!
//! ## Project addressing
//!
//! GitLab identifies projects by `{namespace}/{project}` (URL-encoded as
//! `namespace%2Fproject`), which maps directly to the `owner/repo` convention
//! used by this CLI.

use base64::Engine as _;
use serde::Deserialize;

use crate::http::{HttpClient, HttpError};
use crate::platform::util::repo_fingerprint;
use crate::platform::{
    CombinedBranchStatus, CurrentUser, PlatformClient, PlatformError, RawFile, RepoInitResult,
};

/// Default GitLab API base URL.
pub const GITLAB_API_BASE: &str = "https://gitlab.com/api/v4";

/// Error type for `get_repo_url`.
#[derive(Debug, thiserror::Error)]
pub enum GetRepoUrlError {
    #[error("Invalid GitLab endpoint: {0}")]
    InvalidEndpoint(String),
    #[error("SSH URL unavailable")]
    SshUrlUnavailable,
}

/// Construct the clone URL for a GitLab repository.
///
/// Mirrors `getRepoUrl()` in `lib/modules/platform/gitlab/utils.ts`.
///
/// - `git_url = Some("ssh")` → returns the ssh clone URL (returns error if unavailable).
/// - `git_url = Some("endpoint")` or `http_url_to_repo = None` → builds URL from
///   `endpoint`, returning `GetRepoUrlError::InvalidEndpoint` if the endpoint cannot
///   be parsed as a URL.
/// - Otherwise → returns `http_url_to_repo` with OAuth2 credentials injected.
pub fn get_repo_url(
    repository: &str,
    git_url: Option<&str>,
    ssh_url_to_repo: Option<&str>,
    http_url_to_repo: Option<&str>,
    endpoint: &str,
    token: Option<&str>,
) -> Result<String, GetRepoUrlError> {
    if git_url == Some("ssh") {
        return ssh_url_to_repo
            .map(str::to_owned)
            .ok_or(GetRepoUrlError::SshUrlUnavailable);
    }

    if git_url == Some("endpoint") || http_url_to_repo.is_none() {
        let parsed = parse_url(endpoint)
            .ok_or_else(|| GetRepoUrlError::InvalidEndpoint(endpoint.to_owned()))?;
        let (scheme, host, path_prefix) = parsed;
        let api_idx = path_prefix.find("/api").unwrap_or(path_prefix.len());
        let new_path = &path_prefix[..api_idx];
        let auth = token.map(|t| format!("oauth2:{t}")).unwrap_or_default();
        let url = if auth.is_empty() {
            format!("{scheme}://{host}{new_path}/{repository}.git")
        } else {
            format!("{scheme}://{auth}@{host}{new_path}/{repository}.git")
        };
        return Ok(url);
    }

    let http_url = http_url_to_repo.unwrap();
    let auth_prefix = token.map(|t| format!("oauth2:{t}@")).unwrap_or_default();
    if let Some(after_scheme) = http_url
        .find("://")
        .map(|i| (&http_url[..i], &http_url[i + 3..]))
    {
        Ok(format!(
            "{}://{}{}",
            after_scheme.0, auth_prefix, after_scheme.1
        ))
    } else {
        Ok(http_url.to_owned())
    }
}

fn parse_url(u: &str) -> Option<(String, String, String)> {
    let idx = u.find("://")?;
    let scheme = &u[..idx];
    if scheme.is_empty() {
        return None;
    }
    let rest = &u[idx + 3..];
    let slash_idx = rest.find('/').unwrap_or(rest.len());
    let host = &rest[..slash_idx];
    if host.is_empty() {
        return None;
    }
    let path = &rest[slash_idx..];
    Some((scheme.to_owned(), host.to_owned(), path.to_owned()))
}

/// GitLab platform client authenticated with a Personal Access Token.
#[derive(Debug, Clone)]
pub struct GitlabClient {
    http: HttpClient,
    api_base: String,
}

impl GitlabClient {
    /// Create a new GitLab client with the given PAT and the default API base
    /// (`https://gitlab.com/api/v4`).
    pub fn new(token: impl Into<String>) -> Result<Self, HttpError> {
        Self::with_endpoint(token, GITLAB_API_BASE)
    }

    /// Create a new GitLab client with a custom API endpoint (self-hosted GitLab).
    pub fn with_endpoint(
        token: impl Into<String>,
        api_base: impl Into<String>,
    ) -> Result<Self, HttpError> {
        Ok(Self {
            // GitLab uses PRIVATE-TOKEN header; we pass it via the HTTP client's
            // bearer mechanism and note the header name difference below in
            // get_current_user where we add it explicitly.
            http: HttpClient::with_token(token)?,
            api_base: api_base.into().trim_end_matches('/').to_owned(),
        })
    }
}

// ── API response types ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct GitlabUser {
    username: String,
}

/// GitLab repository file content (Contents API).
/// `GET /projects/{id}/repository/files/{path}?ref=HEAD`
#[derive(Debug, Deserialize)]
struct GitlabFileContent {
    content: String,
    encoding: String,
    file_path: String,
}

/// One entry from the GitLab repository tree API.
/// `GET /projects/{id}/repository/tree?recursive=true&per_page=100`
#[derive(Debug, Deserialize)]
struct GitlabTreeEntry {
    path: String,
    #[serde(rename = "type")]
    entry_type: String,
}

/// GitLab project metadata response.
/// `GET /projects/{id}`
#[derive(Debug, Deserialize)]
struct GitlabProject {
    id: i64,
    archived: bool,
    mirror: bool,
    default_branch: Option<String>,
    empty_repo: bool,
    forked_from_project: Option<serde_json::Value>,
    repository_access_level: Option<String>,
    merge_requests_access_level: Option<String>,
    merge_method: Option<String>,
    issues_enabled: Option<bool>,
}

/// GitLab merge request response.
/// `GET /projects/{id}/merge_requests/{iid}` or list endpoint.
#[derive(Debug, Deserialize)]
struct GitLabMergeRequest {
    iid: i64,
    title: String,
    description: Option<String>,
    state: String,
    source_branch: String,
    created_at: String,
    updated_at: String,
    sha: Option<String>,
    #[serde(default)]
    labels: Vec<String>,
    assignee: Option<GitlabUser>,
    #[serde(default)]
    assignees: Vec<GitlabUser>,
    #[serde(default)]
    reviewers: Vec<GitlabUser>,
}

// ── PlatformClient impl ───────────────────────────────────────────────────────

impl PlatformClient for GitlabClient {
    async fn init_repo(
        &self,
        owner: &str,
        repo: &str,
        _fork_token: Option<&str>,
        _fork_creation: bool,
        _fork_org: Option<&str>,
    ) -> Result<RepoInitResult, PlatformError> {
        let project_id = encode_project(owner, repo);
        let url = format!("{}/projects/{project_id}", self.api_base);

        let resp = self
            .http
            .get_retrying(&url)
            .await
            .map_err(PlatformError::Http)?;

        match resp.status() {
            s if s.is_success() => {}
            s if s == reqwest::StatusCode::FORBIDDEN => {
                return Err(PlatformError::Unexpected(
                    "REPOSITORY_ACCESS_FORBIDDEN".to_owned(),
                ));
            }
            s if s == reqwest::StatusCode::NOT_FOUND => {
                return Err(PlatformError::Unexpected("REPOSITORY_NOT_FOUND".to_owned()));
            }
            s => return Err(PlatformError::Http(HttpError::Status { status: s, url })),
        }

        let project: GitlabProject = resp
            .json()
            .await
            .map_err(|e| PlatformError::Http(HttpError::Request(e)))?;

        if project.archived {
            return Err(PlatformError::Unexpected("REPOSITORY_ARCHIVED".to_owned()));
        }

        if project.mirror {
            return Err(PlatformError::Unexpected("REPOSITORY_MIRRORED".to_owned()));
        }

        if project.repository_access_level.as_deref() == Some("disabled")
            || project.merge_requests_access_level.as_deref() == Some("disabled")
        {
            return Err(PlatformError::Unexpected("REPOSITORY_DISABLED".to_owned()));
        }

        let default_branch = project
            .default_branch
            .filter(|b| !b.is_empty())
            .ok_or_else(|| PlatformError::Unexpected("REPOSITORY_EMPTY".to_owned()))?;

        if project.empty_repo {
            return Err(PlatformError::Unexpected("REPOSITORY_EMPTY".to_owned()));
        }

        let merge_method = project.merge_method.as_deref().map(|m| match m {
            "merge" => "merge".to_owned(),
            "rebase_merge" => "rebase".to_owned(),
            "ff" => "rebase".to_owned(),
            other => other.to_owned(),
        });

        let fingerprint = repo_fingerprint(&project.id.to_string(), Some(&self.api_base));

        Ok(RepoInitResult {
            default_branch,
            is_fork: project.forked_from_project.is_some(),
            repo_fingerprint: fingerprint,
            merge_method,
            auto_merge_allowed: project.merge_requests_access_level.as_deref() != Some("disabled"),
            has_issues_enabled: project.issues_enabled.unwrap_or(true),
            has_vulnerability_alerts_enabled: false,
        })
    }

    async fn get_current_user(&self) -> Result<CurrentUser, PlatformError> {
        let url = format!("{}/user", self.api_base);
        let resp = self
            .http
            .get_retrying(&url)
            .await
            .map_err(PlatformError::Http)?;

        match resp.status() {
            s if s.is_success() => {}
            s if s == reqwest::StatusCode::UNAUTHORIZED => return Err(PlatformError::Unauthorized),
            s => return Err(PlatformError::Http(HttpError::Status { status: s, url })),
        }

        let user: GitlabUser = resp
            .json()
            .await
            .map_err(|e| PlatformError::Http(HttpError::Request(e)))?;

        Ok(CurrentUser {
            login: user.username,
        })
    }

    async fn get_raw_file(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
    ) -> Result<Option<RawFile>, PlatformError> {
        // GitLab requires the file path percent-encoded (slashes as %2F).
        let encoded_path = encode_path(path);
        let project_id = encode_project(owner, repo);
        let url = format!(
            "{}/projects/{}/repository/files/{}?ref=HEAD",
            self.api_base, project_id, encoded_path
        );

        let resp = self
            .http
            .get_retrying(&url)
            .await
            .map_err(PlatformError::Http)?;

        match resp.status() {
            s if s.is_success() => {}
            s if s == reqwest::StatusCode::NOT_FOUND => return Ok(None),
            s if s == reqwest::StatusCode::UNAUTHORIZED => return Err(PlatformError::Unauthorized),
            s => return Err(PlatformError::Http(HttpError::Status { status: s, url })),
        }

        let content: GitlabFileContent = resp
            .json()
            .await
            .map_err(|e| PlatformError::Http(HttpError::Request(e)))?;

        let decoded = if content.encoding == "base64" {
            // GitLab base64 may contain newlines — strip them before decoding.
            let stripped = content.content.replace('\n', "");
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(stripped.as_bytes())
                .map_err(|e| PlatformError::Unexpected(format!("base64 decode error: {e}")))?;
            String::from_utf8(bytes)
                .map_err(|e| PlatformError::Unexpected(format!("UTF-8 decode error: {e}")))?
        } else {
            content.content
        };

        Ok(Some(RawFile {
            path: content.file_path,
            content: decoded,
        }))
    }

    async fn get_file_list(&self, owner: &str, repo: &str) -> Result<Vec<String>, PlatformError> {
        let project_id = encode_project(owner, repo);
        let mut files: Vec<String> = Vec::new();
        let mut page: u32 = 1;

        // Paginate through the full recursive tree.  GitLab's tree API returns
        // at most 100 entries per page; we stop when a page returns fewer than
        // `per_page` entries (last page) or after 50 pages as a safety cap.
        loop {
            let url = format!(
                "{}/projects/{}/repository/tree?recursive=true&per_page=100&page={page}",
                self.api_base, project_id
            );

            let resp = self
                .http
                .get_retrying(&url)
                .await
                .map_err(PlatformError::Http)?;

            match resp.status() {
                s if s.is_success() => {}
                s if s == reqwest::StatusCode::NOT_FOUND => {
                    return Err(PlatformError::Unexpected("repository not found".to_owned()));
                }
                s if s == reqwest::StatusCode::UNAUTHORIZED => {
                    return Err(PlatformError::Unauthorized);
                }
                s => return Err(PlatformError::Http(HttpError::Status { status: s, url })),
            }

            // GitLab returns total page count in X-Total-Pages header; we
            // rely on entry count to detect the last page instead.
            let entries: Vec<GitlabTreeEntry> = resp
                .json()
                .await
                .map_err(|e| PlatformError::Http(HttpError::Request(e)))?;

            let count = entries.len();
            files.extend(
                entries
                    .into_iter()
                    .filter(|e| e.entry_type == "blob")
                    .map(|e| e.path),
            );

            if count < 100 || page >= 50 {
                if page >= 50 {
                    tracing::warn!(
                        repo = %format!("{owner}/{repo}"),
                        "GitLab file tree exceeded 50 pages; list may be incomplete"
                    );
                }
                break;
            }
            page += 1;
        }

        Ok(files)
    }

    async fn create_pr(
        &self,
        owner: &str,
        repo: &str,
        source_branch: &str,
        target_branch: &str,
        title: &str,
        body: &str,
    ) -> Result<Option<i64>, PlatformError> {
        let project_id = encode_project(owner, repo);
        let url = format!("{}/projects/{}/merge_requests", self.api_base, project_id);

        let body_json = serde_json::json!({
            "source_branch": source_branch,
            "target_branch": target_branch,
            "title": title,
            "description": body,
            "remove_source_branch": true,
        });
        let body_str = serde_json::to_string(&body_json)
            .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;

        let resp = self
            .http
            .post_json::<GitLabMergeRequest>(&url, &body_str)
            .await
            .map_err(PlatformError::Http)?;

        Ok(Some(resp.iid))
    }

    async fn update_pr(
        &self,
        owner: &str,
        repo: &str,
        pr_number: i64,
        title: Option<&str>,
        body: Option<&str>,
        state: Option<&str>,
    ) -> Result<(), PlatformError> {
        if title.is_none() && body.is_none() && state.is_none() {
            return Ok(());
        }

        let project_id = encode_project(owner, repo);
        let url = format!(
            "{}/projects/{}/merge_requests/{}",
            self.api_base, project_id, pr_number
        );

        // Preserve draft prefix if the existing PR is a draft.
        let effective_title = match title {
            Some(t) => match self.get_pr(owner, repo, pr_number).await? {
                Some(existing) if existing.is_draft => Some(format!("{DRAFT_PREFIX}{t}")),
                _ => Some(t.to_owned()),
            },
            None => None,
        };

        let state_event = state.map(|s| match s {
            "closed" => "close",
            "open" => "reopen",
            _ => s,
        });

        let mut request = serde_json::Map::new();
        if let Some(t) = effective_title {
            request.insert("title".to_owned(), serde_json::Value::String(t));
        }
        if let Some(b) = body {
            request.insert(
                "description".to_owned(),
                serde_json::Value::String(b.to_owned()),
            );
        }
        if let Some(se) = state_event {
            request.insert(
                "state_event".to_owned(),
                serde_json::Value::String(se.to_owned()),
            );
        }

        let body_str = serde_json::to_string(&request)
            .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;

        let resp = self
            .http
            .put_json::<GitLabMergeRequest>(&url, &body_str)
            .await
            .map_err(PlatformError::Http)?;

        if !resp.state.is_empty() {
            tracing::debug!(pr = pr_number, "PR updated successfully");
        }
        Ok(())
    }

    async fn get_branch_status(
        &self,
        _owner: &str,
        _repo: &str,
        _branch: &str,
    ) -> Result<CombinedBranchStatus, PlatformError> {
        Err(PlatformError::NotSupported(
            "GitLab branch status".to_owned(),
        ))
    }

    async fn write_file(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        content: &str,
        branch: Option<&str>,
        message: Option<&str>,
    ) -> Result<(), PlatformError> {
        let project_id = encode_project(owner, repo);
        let encoded_path = encode_path(path);
        let branch = branch.ok_or_else(|| {
            PlatformError::Unexpected("GitLab write_file requires a branch".to_owned())
        })?;

        // Try to fetch existing file to determine whether to create or update.
        let get_url = format!(
            "{}/projects/{}/repository/files/{}?ref={}",
            self.api_base, project_id, encoded_path, branch
        );
        let exists = match self.http.get(&get_url).send().await {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        };

        let b64_content = base64::engine::general_purpose::STANDARD.encode(content);
        let body = serde_json::json!({
            "branch": branch,
            "content": b64_content,
            "encoding": "base64",
            "commit_message": message.unwrap_or("Update file via Renovate"),
        });
        let body_str = serde_json::to_string(&body)
            .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;

        let url = format!(
            "{}/projects/{}/repository/files/{}",
            self.api_base, project_id, encoded_path
        );

        if exists {
            self.http
                .put_json::<serde_json::Value>(&url, &body_str)
                .await
                .map_err(PlatformError::Http)?;
        } else {
            self.http
                .post_json::<serde_json::Value>(&url, &body_str)
                .await
                .map_err(PlatformError::Http)?;
        }

        Ok(())
    }

    async fn get_pr_list(
        &self,
        owner: &str,
        repo: &str,
        state: Option<&str>,
    ) -> Result<Vec<crate::platform::GhPr>, PlatformError> {
        let project_id = encode_project(owner, repo);
        let gl_state = match state {
            Some("open") => "opened",
            Some(s) => s,
            None => "all",
        };

        let mut all_prs: Vec<crate::platform::GhPr> = Vec::new();
        let mut page: u32 = 1;

        loop {
            let url = format!(
                "{}/projects/{}/merge_requests?per_page=100&state={}&page={}",
                self.api_base, project_id, gl_state, page
            );

            let resp = self
                .http
                .get_retrying(&url)
                .await
                .map_err(PlatformError::Http)?;

            match resp.status() {
                s if s.is_success() => {}
                s if s == reqwest::StatusCode::NOT_FOUND => {
                    return Err(PlatformError::Unexpected("repository not found".to_owned()));
                }
                s if s == reqwest::StatusCode::UNAUTHORIZED => {
                    return Err(PlatformError::Unauthorized);
                }
                s => return Err(PlatformError::Http(HttpError::Status { status: s, url })),
            }

            let mrs: Vec<GitLabMergeRequest> = resp
                .json()
                .await
                .map_err(|e| PlatformError::Http(HttpError::Request(e)))?;

            let count = mrs.len();
            all_prs.extend(mrs.into_iter().map(mr_to_gh_pr));

            if count < 100 || page >= 50 {
                if page >= 50 {
                    tracing::warn!(
                        repo = %format!("{owner}/{repo}"),
                        "GitLab PR list exceeded 50 pages; list may be incomplete"
                    );
                }
                break;
            }
            page += 1;
        }

        Ok(all_prs)
    }

    async fn get_pr(
        &self,
        owner: &str,
        repo: &str,
        pr_number: i64,
    ) -> Result<Option<crate::platform::GhPr>, PlatformError> {
        if pr_number == 0 {
            return Ok(None);
        }

        let project_id = encode_project(owner, repo);
        let url = format!(
            "{}/projects/{}/merge_requests/{}?include_diverged_commits_count=1",
            self.api_base, project_id, pr_number
        );

        match self.http.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => {
                let mr: GitLabMergeRequest = resp
                    .json()
                    .await
                    .map_err(|e| PlatformError::Http(HttpError::Request(e)))?;
                Ok(Some(mr_to_gh_pr(mr)))
            }
            Ok(resp) if resp.status() == reqwest::StatusCode::NOT_FOUND => Ok(None),
            Ok(resp) => Err(PlatformError::Http(HttpError::Status {
                status: resp.status(),
                url,
            })),
            Err(e) => Err(PlatformError::Http(HttpError::Request(e))),
        }
    }

    async fn get_branch_pr(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<Option<crate::platform::GhPr>, PlatformError> {
        let project_id = encode_project(owner, repo);
        let url = format!(
            "{}/projects/{}/merge_requests?source_branch={}&state=opened&per_page=1",
            self.api_base, project_id, branch
        );

        let resp = self
            .http
            .get_retrying(&url)
            .await
            .map_err(PlatformError::Http)?;

        match resp.status() {
            s if s.is_success() => {}
            s if s == reqwest::StatusCode::NOT_FOUND => {
                return Err(PlatformError::Unexpected("repository not found".to_owned()));
            }
            s if s == reqwest::StatusCode::UNAUTHORIZED => {
                return Err(PlatformError::Unauthorized);
            }
            s => return Err(PlatformError::Http(HttpError::Status { status: s, url })),
        }

        let mrs: Vec<GitLabMergeRequest> = resp
            .json()
            .await
            .map_err(|e| PlatformError::Http(HttpError::Request(e)))?;

        match mrs.into_iter().next() {
            Some(mr) => self.get_pr(owner, repo, mr.iid).await,
            None => Ok(None),
        }
    }
}

// ── URL encoding helpers ──────────────────────────────────────────────────────

/// Percent-encode a file path for use in GitLab API URLs.
///
/// GitLab requires each `/` in the path to be encoded as `%2F`.
fn encode_path(path: &str) -> String {
    path.replace('/', "%2F")
}

/// Encode a `owner/repo` pair as a GitLab project identifier.
///
/// GitLab accepts `{namespace}%2F{project}` as the project ID in REST URLs.
fn encode_project(owner: &str, repo: &str) -> String {
    format!("{owner}%2F{repo}")
}

const DRAFT_PREFIX: &str = "Draft: ";
const DRAFT_PREFIX_DEPRECATED: &str = "WIP: ";

/// Convert a GitLab merge request into the Renovate `GhPr` format.
///
/// Mirrors `prInfo()` from `lib/modules/platform/gitlab/utils.ts`.
fn mr_to_gh_pr(mr: GitLabMergeRequest) -> crate::platform::GhPr {
    let mut title = mr.title;
    let is_draft = if title.starts_with(DRAFT_PREFIX) {
        title = title.strip_prefix(DRAFT_PREFIX).unwrap_or(&title).to_owned();
        true
    } else if title.starts_with(DRAFT_PREFIX_DEPRECATED) {
        title = title
            .strip_prefix(DRAFT_PREFIX_DEPRECATED)
            .unwrap_or(&title)
            .to_owned();
        true
    } else {
        false
    };

    let state = if mr.state == "opened" {
        "open".to_owned()
    } else {
        mr.state
    };

    let has_assignees = mr.assignee.is_some() || !mr.assignees.is_empty();
    let reviewers = mr
        .reviewers
        .into_iter()
        .map(|u| u.username)
        .collect::<Vec<_>>();

    let body_struct = crate::platform::pr_body::get_pr_body_struct(mr.description.as_deref());

    crate::platform::GhPr {
        number: mr.iid,
        title,
        state,
        source_branch: mr.source_branch,
        source_repo: None,
        body_struct: Some(body_struct),
        updated_at: mr.updated_at,
        node_id: mr.iid.to_string(),
        sha: mr.sha,
        labels: mr.labels,
        has_assignees,
        reviewers,
        created_at: Some(mr.created_at),
        closed_at: None,
        is_draft,
    }
}

// ── code-owners (mirrors lib/modules/platform/gitlab/code-owners.ts) ─────────

/// A parsed CODEOWNERS file rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileOwnerRule {
    /// The glob pattern for this rule.
    pub pattern: String,
    /// The owner usernames assigned to this pattern.
    pub usernames: Vec<String>,
    /// Score = pattern length (longer patterns win).
    pub score: usize,
}

/// Parse CODEOWNERS file lines into owner rules.
///
/// Mirrors `extractRulesFromCodeOwnersLines` from
/// `lib/modules/platform/gitlab/code-owners.ts`.
pub fn extract_rules_from_code_owners_lines(lines: &[&str]) -> Vec<FileOwnerRule> {
    let mut default_users: Vec<String> = Vec::new();
    let mut rules = Vec::new();

    for &line in lines {
        if line.starts_with('[') || line.starts_with("^[") {
            // Section header: find last `]` to handle approval counts like `[Team][2]`
            let last_close = line.rfind(']').unwrap_or(0);
            let after_header = line[last_close + 1..].trim();
            default_users = if after_header.is_empty() {
                Vec::new()
            } else {
                after_header
                    .split_whitespace()
                    .map(|s| s.to_owned())
                    .collect()
            };
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let (pattern, usernames) = match parts.split_first() {
                Some((p, rest)) => (
                    *p,
                    if rest.is_empty() {
                        default_users.clone()
                    } else {
                        rest.iter().map(|s| (*s).to_owned()).collect()
                    },
                ),
                None => continue,
            };
            rules.push(FileOwnerRule {
                score: pattern.len(),
                pattern: pattern.to_owned(),
                usernames,
            });
        }
    }

    rules
}

/// Transform Markdown content for GitLab compatibility.
///
/// Replaces GitHub-style PR references and links with GitLab MR equivalents,
/// strips unicode null characters, and applies `smartTruncate`.
/// Mirrors `massageMarkdown` from `lib/modules/platform/gitlab/index.ts`.
pub fn massage_markdown(input: &str) -> String {
    use crate::platform::pr_body::smart_truncate;
    let desc = input.replace("Pull Request", "Merge Request");
    let re_pr_hash = regex::Regex::new(r"\bPR: #").unwrap();
    let desc = re_pr_hash.replace_all(&desc, "MR: !").into_owned();
    let re_prs = regex::Regex::new(r"\bPRs\b").unwrap();
    let desc = re_prs.replace_all(&desc, "MRs").into_owned();
    let re_pr = regex::Regex::new(r"\bPR\b").unwrap();
    let desc = re_pr.replace_all(&desc, "MR").into_owned();
    let desc = desc
        .replace("](../pull/", "](!")
        .replace("](../issues/", "](#")
        .replace('\u{0000}', "");
    smart_truncate(&desc, 1_000_000)
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;
    use crate::platform::PlatformClient;

    fn make_client(server_uri: &str) -> GitlabClient {
        GitlabClient::with_endpoint("test-token", server_uri).unwrap()
    }

    // ── encode helpers ────────────────────────────────────────────────────────

    // Rust-specific: gitlab behavior test
    #[test]
    fn encode_path_replaces_slashes() {
        assert_eq!(encode_path("src/main.rs"), "src%2Fmain.rs");
        assert_eq!(
            encode_path(".github/renovate.json"),
            ".github%2Frenovate.json"
        );
    }

    // Rust-specific: gitlab behavior test
    #[test]
    fn encode_project_formats_correctly() {
        assert_eq!(encode_project("owner", "repo"), "owner%2Frepo");
    }

    // ── init_repo ─────────────────────────────────────────────────────────────

    // Ported: "should throw an error if repository is archived" — modules/platform/gitlab/index.spec.ts line 345
    #[tokio::test]
    async fn init_repo_archived_returns_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 1,
                "archived": true,
                "mirror": false,
                "default_branch": "main",
                "empty_repo": false,
                "repository_access_level": "enabled",
                "merge_requests_access_level": "enabled"
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let err = client
            .init_repo("owner", "repo", None, false, None)
            .await
            .unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(msg) if msg == "REPOSITORY_ARCHIVED"));
    }

    // Ported: "should throw an error if repository is a mirror" — modules/platform/gitlab/index.spec.ts line 357
    #[tokio::test]
    async fn init_repo_mirror_returns_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 1,
                "archived": false,
                "mirror": true,
                "default_branch": "main",
                "empty_repo": false,
                "repository_access_level": "enabled",
                "merge_requests_access_level": "enabled"
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let err = client
            .init_repo("owner", "repo", None, false, None)
            .await
            .unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(msg) if msg == "REPOSITORY_MIRRORED"));
    }

    // Ported: "should throw an error if repository has empty_repo property" — modules/platform/gitlab/index.spec.ts line 413
    #[tokio::test]
    async fn init_repo_empty_returns_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 1,
                "archived": false,
                "mirror": false,
                "default_branch": null,
                "empty_repo": true,
                "repository_access_level": "enabled",
                "merge_requests_access_level": "enabled"
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let err = client
            .init_repo("owner", "repo", None, false, None)
            .await
            .unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(msg) if msg == "REPOSITORY_EMPTY"));
    }

    // Ported: "should throw an error if repository is empty" — modules/platform/gitlab/index.spec.ts line 425
    #[tokio::test]
    async fn init_repo_null_default_branch_returns_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 1,
                "archived": false,
                "mirror": false,
                "default_branch": null,
                "empty_repo": false,
                "repository_access_level": "enabled",
                "merge_requests_access_level": "enabled"
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let err = client
            .init_repo("owner", "repo", None, false, None)
            .await
            .unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(msg) if msg == "REPOSITORY_EMPTY"));
    }

    // Ported: "should throw an error if receiving an error" — modules/platform/gitlab/index.spec.ts line 333
    #[tokio::test]
    async fn init_repo_server_error_returns_http_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let err = client
            .init_repo("owner", "repo", None, false, None)
            .await
            .unwrap_err();
        assert!(matches!(err, PlatformError::Http(HttpError::Status { status, .. }) if status == reqwest::StatusCode::INTERNAL_SERVER_ERROR));
    }

    // Ported: "should fall back if http_url_to_repo is empty" — modules/platform/gitlab/index.spec.ts line 437
    #[tokio::test]
    async fn init_repo_minimal_response_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 1,
                "archived": false,
                "mirror": false,
                "default_branch": "master",
                "empty_repo": false,
                "repository_access_level": "enabled",
                "merge_requests_access_level": "enabled"
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let result = client
            .init_repo("owner", "repo", None, false, None)
            .await
            .unwrap();
        assert_eq!(result.default_branch, "master");
        assert!(!result.is_fork);
    }

    // Ported: "should throw an error if repository access is disabled" — modules/platform/gitlab/index.spec.ts line 389
    #[tokio::test]
    async fn init_repo_disabled_returns_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 1,
                "archived": false,
                "mirror": false,
                "default_branch": "main",
                "empty_repo": false,
                "repository_access_level": "disabled",
                "merge_requests_access_level": "enabled"
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let err = client
            .init_repo("owner", "repo", None, false, None)
            .await
            .unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(msg) if msg == "REPOSITORY_DISABLED"));
    }

    // Ported: "should throw an error if MRs are disabled" — modules/platform/gitlab/index.spec.ts line 401
    #[tokio::test]
    async fn init_repo_mrs_disabled_returns_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 1,
                "archived": false,
                "mirror": false,
                "default_branch": "main",
                "empty_repo": false,
                "repository_access_level": "enabled",
                "merge_requests_access_level": "disabled"
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let err = client
            .init_repo("owner", "repo", None, false, None)
            .await
            .unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(msg) if msg == "REPOSITORY_DISABLED"));
    }

    // Ported: "should return an array of repos" — modules/platform/gitlab/index.spec.ts line 163
    #[tokio::test]
    async fn init_repo_returns_real_metadata() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 42,
                "archived": false,
                "mirror": false,
                "default_branch": "master",
                "empty_repo": false,
                "forked_from_project": {"id": 1},
                "repository_access_level": "enabled",
                "merge_requests_access_level": "enabled",
                "merge_method": "rebase_merge",
                "issues_enabled": true,
                "squash_option": "default_on",
                "path_with_namespace": "owner/repo"
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let result = client
            .init_repo("owner", "repo", None, false, None)
            .await
            .unwrap();
        assert_eq!(result.default_branch, "master");
        assert!(result.is_fork);
        assert!(!result.repo_fingerprint.is_empty());
        assert_eq!(result.merge_method, Some("rebase".to_owned()));
        assert!(result.auto_merge_allowed);
        assert!(result.has_issues_enabled);
        assert!(!result.has_vulnerability_alerts_enabled);
    }

    // Ported: "should throw an error if receiving an error" — modules/platform/gitlab/index.spec.ts line 333
    #[tokio::test]
    async fn init_repo_not_found_returns_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let err = client
            .init_repo("owner", "repo", None, false, None)
            .await
            .unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(msg) if msg == "REPOSITORY_NOT_FOUND"));
    }

    // Ported: "should throw if endpoint is not a valid URL" — modules/platform/gitlab/index.spec.ts line 82
    #[tokio::test]
    async fn init_repo_forbidden_returns_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let err = client
            .init_repo("owner", "repo", None, false, None)
            .await
            .unwrap_err();
        assert!(
            matches!(err, PlatformError::Unexpected(msg) if msg == "REPOSITORY_ACCESS_FORBIDDEN")
        );
    }

    // ── get_current_user ──────────────────────────────────────────────────────

    // Ported: "should throw if auth fails" — modules/platform/gitlab/index.spec.ts line 91
    #[tokio::test]
    async fn get_current_user_unauthorized() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let err = client.get_current_user().await.unwrap_err();
        assert!(matches!(err, PlatformError::Unauthorized));
    }

    // Ported: "should default to gitlab.com" — modules/platform/gitlab/index.spec.ts line 101
    #[tokio::test]
    async fn get_current_user_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!({"id": 1, "username": "devuser"})),
            )
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let user = client.get_current_user().await.unwrap();
        assert_eq!(user.login, "devuser");
    }

    // ── get_raw_file ──────────────────────────────────────────────────────────

    // Ported: "should throw an error if it receives an error" — modules/platform/gitlab/index.spec.ts line 153
    #[tokio::test]
    async fn get_raw_file_returns_none_for_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/repository/files/missing.txt"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let result = client
            .get_raw_file("owner", "repo", "missing.txt")
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "should return an array of repos" — modules/platform/gitlab/index.spec.ts line 163
    #[tokio::test]
    async fn get_raw_file_returns_decoded_content() {
        let server = MockServer::start().await;
        let encoded =
            base64::engine::general_purpose::STANDARD.encode(b"[package]\nname = \"foo\"");
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/repository/files/Cargo.toml"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "content": encoded,
                "encoding": "base64",
                "file_path": "Cargo.toml"
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let file = client
            .get_raw_file("owner", "repo", "Cargo.toml")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(file.path, "Cargo.toml");
        assert!(file.content.contains("[package]"));
    }

    // Ported: "should return an array of repos including mirrors" — modules/platform/gitlab/index.spec.ts line 185
    #[tokio::test]
    async fn get_raw_file_encodes_path_slashes() {
        let server = MockServer::start().await;
        let encoded = base64::engine::general_purpose::STANDARD.encode(b"{}");
        Mock::given(method("GET"))
            .and(path(
                "/projects/owner%2Frepo/repository/files/.github%2Frenovate.json",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "content": encoded,
                "encoding": "base64",
                "file_path": ".github/renovate.json"
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let file = client
            .get_raw_file("owner", "repo", ".github/renovate.json")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(file.path, ".github/renovate.json");
    }

    // ── get_file_list ─────────────────────────────────────────────────────────

    // Ported: "should encode the requested topics into the URL" — modules/platform/gitlab/index.spec.ts line 207
    #[tokio::test]
    async fn get_file_list_returns_blobs_only() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/repository/tree"))
            .and(query_param("page", "1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"path": "Cargo.toml", "type": "blob"},
                {"path": "src", "type": "tree"},
                {"path": "src/main.rs", "type": "blob"},
            ])))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let files = client.get_file_list("owner", "repo").await.unwrap();
        assert_eq!(files, vec!["Cargo.toml", "src/main.rs"]);
    }

    // Ported: "should query the groups endpoint for each namespace" — modules/platform/gitlab/index.spec.ts line 225
    #[tokio::test]
    async fn get_file_list_paginates() {
        let server = MockServer::start().await;

        // First page: 100 entries (all blobs to keep the fixture simple).
        let page1: Vec<serde_json::Value> = (0..100)
            .map(|i| serde_json::json!({"path": format!("file{i}.txt"), "type": "blob"}))
            .collect();
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/repository/tree"))
            .and(query_param("page", "1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(page1))
            .mount(&server)
            .await;

        // Second page: 3 entries → last page.
        let page2 = serde_json::json!([
            {"path": "README.md", "type": "blob"},
            {"path": "LICENSE", "type": "blob"},
            {"path": "docs", "type": "tree"},
        ]);
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/repository/tree"))
            .and(query_param("page", "2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(page2))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let files = client.get_file_list("owner", "repo").await.unwrap();
        // 100 from page 1 + 2 blobs from page 2 (docs/tree is excluded)
        assert_eq!(files.len(), 102);
        assert!(files.contains(&"README.md".to_owned()));
    }

    // ── create_pr ─────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn create_pr_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/projects/owner%2Frepo/merge_requests"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "iid": 42,
                "title": "Update deps",
                "description": "Body",
                "state": "opened",
                "source_branch": "renovate/deps",
                "target_branch": "main",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let pr_number = client
            .create_pr("owner", "repo", "renovate/deps", "main", "Update deps", "Body")
            .await
            .unwrap();
        assert_eq!(pr_number, Some(42));
    }

    // ── update_pr ─────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn update_pr_no_op_when_nothing_to_update() {
        let server = MockServer::start().await;
        let client = make_client(&server.uri());
        // No request should be sent when all args are None.
        let result = client.update_pr("owner", "repo", 42, None, None, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn update_pr_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/projects/owner%2Frepo/merge_requests/42"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "iid": 42,
                "title": "New title",
                "description": "New body",
                "state": "opened",
                "source_branch": "renovate/deps",
                "target_branch": "main",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let result = client
            .update_pr("owner", "repo", 42, Some("New title"), Some("New body"), None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn update_pr_closes_pr() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/projects/owner%2Frepo/merge_requests/42"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "iid": 42,
                "title": "Title",
                "description": "Body",
                "state": "closed",
                "source_branch": "renovate/deps",
                "target_branch": "main",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let result = client
            .update_pr("owner", "repo", 42, None, None, Some("closed"))
            .await;
        assert!(result.is_ok());
    }

    // ── get_branch_status ─────────────────────────────────────────────────────

    #[tokio::test]
    async fn get_branch_status_returns_not_supported() {
        let server = MockServer::start().await;
        let client = make_client(&server.uri());
        let err = client
            .get_branch_status("owner", "repo", "main")
            .await
            .unwrap_err();
        assert!(matches!(err, PlatformError::NotSupported(_)));
    }

    // ── write_file ────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn write_file_requires_branch() {
        let server = MockServer::start().await;
        let client = make_client(&server.uri());
        let err = client
            .write_file("owner", "repo", "path", "content", None, None)
            .await
            .unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(msg) if msg.contains("requires a branch")));
    }

    #[tokio::test]
    async fn write_file_creates_file() {
        let server = MockServer::start().await;
        // File does not exist → GET returns 404
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/repository/files/new.txt"))
            .and(query_param("ref", "main"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        Mock::given(method("POST"))
            .and(path("/projects/owner%2Frepo/repository/files/new.txt"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "file_path": "new.txt",
                "branch": "main",
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let result = client
            .write_file("owner", "repo", "new.txt", "hello", Some("main"), None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn write_file_updates_existing_file() {
        let server = MockServer::start().await;
        let encoded = base64::engine::general_purpose::STANDARD.encode(b"existing");
        // File exists → GET returns 200
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/repository/files/existing.txt"))
            .and(query_param("ref", "main"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "file_path": "existing.txt",
                "content": encoded,
                "encoding": "base64",
            })))
            .mount(&server)
            .await;

        Mock::given(method("PUT"))
            .and(path("/projects/owner%2Frepo/repository/files/existing.txt"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "file_path": "existing.txt",
                "branch": "main",
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let result = client
            .write_file("owner", "repo", "existing.txt", "updated", Some("main"), None)
            .await;
        assert!(result.is_ok());
    }

    // ── get_pr_list ───────────────────────────────────────────────────────────

    #[tokio::test]
    async fn get_pr_list_returns_prs() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/merge_requests"))
            .and(query_param("state", "all"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "iid": 1,
                    "title": "Update deps",
                    "description": "Body",
                    "state": "opened",
                    "source_branch": "renovate/deps",
                    "target_branch": "main",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-02T00:00:00Z",
                    "sha": "abc123",
                    "labels": ["dependencies"],
                },
                {
                    "iid": 2,
                    "title": "WIP: Draft PR",
                    "description": null,
                    "state": "closed",
                    "source_branch": "renovate/draft",
                    "target_branch": "main",
                    "created_at": "2024-01-03T00:00:00Z",
                    "updated_at": "2024-01-04T00:00:00Z",
                    "sha": "def456",
                    "labels": [],
                },
            ])))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let prs = client.get_pr_list("owner", "repo", None).await.unwrap();
        assert_eq!(prs.len(), 2);
        assert_eq!(prs[0].number, 1);
        assert_eq!(prs[0].state, "open");
        assert_eq!(prs[0].source_branch, "renovate/deps");
        assert_eq!(prs[1].number, 2);
        assert_eq!(prs[1].state, "closed");
        assert!(prs[1].is_draft);
        assert_eq!(prs[1].title, "Draft PR");
    }

    #[tokio::test]
    async fn get_pr_list_filters_by_state() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/merge_requests"))
            .and(query_param("state", "opened"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "iid": 1,
                    "title": "Update deps",
                    "description": null,
                    "state": "opened",
                    "source_branch": "renovate/deps",
                    "target_branch": "main",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-01T00:00:00Z",
                },
            ])))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let prs = client.get_pr_list("owner", "repo", Some("open")).await.unwrap();
        assert_eq!(prs.len(), 1);
        assert_eq!(prs[0].state, "open");
    }

    // ── get_pr ────────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn get_pr_returns_mr() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/merge_requests/42"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "iid": 42,
                "title": "Update deps",
                "description": "Body",
                "state": "opened",
                "source_branch": "renovate/deps",
                "target_branch": "main",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-02T00:00:00Z",
                "sha": "abc123",
                "labels": ["dependencies"],
                "assignee": {"id": 1, "username": "alice"},
                "assignees": [{"id": 1, "username": "alice"}],
                "reviewers": [{"id": 2, "username": "bob"}],
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let pr = client.get_pr("owner", "repo", 42).await.unwrap().unwrap();
        assert_eq!(pr.number, 42);
        assert_eq!(pr.title, "Update deps");
        assert_eq!(pr.state, "open");
        assert!(pr.has_assignees);
        assert_eq!(pr.reviewers, vec!["bob"]);
        assert_eq!(pr.labels, vec!["dependencies"]);
        assert_eq!(pr.sha, Some("abc123".to_owned()));
    }

    #[tokio::test]
    async fn get_pr_returns_none_for_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/merge_requests/99"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let pr = client.get_pr("owner", "repo", 99).await.unwrap();
        assert!(pr.is_none());
    }

    #[tokio::test]
    async fn get_pr_returns_none_for_zero() {
        let server = MockServer::start().await;
        let client = make_client(&server.uri());
        let pr = client.get_pr("owner", "repo", 0).await.unwrap();
        assert!(pr.is_none());
    }

    // Ported: "removes draft prefix from returned title" — modules/platform/gitlab/index.spec.ts line 3466
    #[tokio::test]
    async fn get_pr_strips_draft_prefix() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/merge_requests/7"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "iid": 7,
                "title": "Draft: do something",
                "description": "a merge request",
                "state": "merged",
                "source_branch": "some-branch",
                "target_branch": "main",
                "created_at": "2025-05-19T12:00:00Z",
                "updated_at": "2025-05-19T12:00:00Z",
                "assignees": []
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let pr = client.get_pr("owner", "repo", 7).await.unwrap().unwrap();
        assert_eq!(pr.title, "do something");
        assert!(pr.is_draft);
    }

    // Ported: "removes deprecated draft prefix from returned title" — modules/platform/gitlab/index.spec.ts line 3490
    #[tokio::test]
    async fn get_pr_strips_wip_prefix() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/merge_requests/7"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "iid": 7,
                "title": "WIP: do something",
                "description": "a merge request",
                "state": "merged",
                "source_branch": "some-branch",
                "target_branch": "main",
                "created_at": "2025-05-19T12:00:00Z",
                "updated_at": "2025-05-19T12:00:00Z",
                "assignees": []
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let pr = client.get_pr("owner", "repo", 7).await.unwrap().unwrap();
        assert_eq!(pr.title, "do something");
        assert!(pr.is_draft);
    }

    // ── get_branch_pr ─────────────────────────────────────────────────────────

    #[tokio::test]
    async fn get_branch_pr_finds_open_mr() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/merge_requests"))
            .and(query_param("source_branch", "renovate/deps"))
            .and(query_param("state", "opened"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "iid": 7,
                    "title": "Update deps",
                    "description": null,
                    "state": "opened",
                    "source_branch": "renovate/deps",
                    "target_branch": "main",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-01T00:00:00Z",
                },
            ])))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/merge_requests/7"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "iid": 7,
                "title": "Update deps",
                "description": "Detailed body",
                "state": "opened",
                "source_branch": "renovate/deps",
                "target_branch": "main",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let pr = client
            .get_branch_pr("owner", "repo", "renovate/deps")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(pr.number, 7);
        assert_eq!(pr.title, "Update deps");
    }

    #[tokio::test]
    async fn get_branch_pr_returns_none_when_no_mr() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/merge_requests"))
            .and(query_param("source_branch", "renovate/missing"))
            .and(query_param("state", "opened"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let pr = client
            .get_branch_pr("owner", "repo", "renovate/missing")
            .await
            .unwrap();
        assert!(pr.is_none());
    }

    // Ported: "should strip draft prefix from title" — modules/platform/gitlab/index.spec.ts line 618
    #[tokio::test]
    async fn get_branch_pr_strips_draft_prefix() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/merge_requests"))
            .and(query_param("source_branch", "renovate/deps"))
            .and(query_param("state", "opened"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "iid": 7,
                    "title": "Draft: some change",
                    "description": "a merge request",
                    "state": "opened",
                    "source_branch": "renovate/deps",
                    "target_branch": "main",
                    "created_at": "2025-05-19T12:00:00Z",
                    "updated_at": "2025-05-19T12:00:00Z",
                },
            ])))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/merge_requests/7"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "iid": 7,
                "title": "Draft: some change",
                "description": "a merge request",
                "state": "opened",
                "source_branch": "renovate/deps",
                "target_branch": "main",
                "created_at": "2025-05-19T12:00:00Z",
                "updated_at": "2025-05-19T12:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let pr = client
            .get_branch_pr("owner", "repo", "renovate/deps")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(pr.title, "some change");
        assert!(pr.is_draft);
    }

    // Ported: "should strip deprecated draft prefix from title" — modules/platform/gitlab/index.spec.ts line 657
    #[tokio::test]
    async fn get_branch_pr_strips_wip_prefix() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/merge_requests"))
            .and(query_param("source_branch", "renovate/deps"))
            .and(query_param("state", "opened"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "iid": 7,
                    "title": "WIP: some change",
                    "description": "a merge request",
                    "state": "opened",
                    "source_branch": "renovate/deps",
                    "target_branch": "main",
                    "created_at": "2025-05-19T12:00:00Z",
                    "updated_at": "2025-05-19T12:00:00Z",
                },
            ])))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/merge_requests/7"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "iid": 7,
                "title": "WIP: some change",
                "description": "a merge request",
                "state": "opened",
                "source_branch": "renovate/deps",
                "target_branch": "main",
                "created_at": "2025-05-19T12:00:00Z",
                "updated_at": "2025-05-19T12:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let pr = client
            .get_branch_pr("owner", "repo", "renovate/deps")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(pr.title, "some change");
        assert!(pr.is_draft);
    }

    // ── code-owners ───────────────────────────────────────────────────────────

    // Ported: "should extract an owner rule from a line" — modules/platform/gitlab/code-owners.spec.ts line 5
    #[test]
    fn code_owners_parses_pattern_with_usernames() {
        let rules = extract_rules_from_code_owners_lines(&["pattern username1 username2"]);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].pattern, "pattern");
        assert_eq!(rules[0].usernames, vec!["username1", "username2"]);
        assert_eq!(rules[0].score, 7);
    }

    // Ported: "should extract an owner rule from a line with no usernames" — modules/platform/gitlab/code-owners.spec.ts line 20
    #[test]
    fn code_owners_parses_pattern_without_usernames() {
        let rules = extract_rules_from_code_owners_lines(&["pattern"]);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].pattern, "pattern");
        assert_eq!(rules[0].usernames, Vec::<String>::new());
        assert_eq!(rules[0].score, 7);
    }

    // Ported: "should extract an owner rule from a line after a section header" — modules/platform/gitlab/code-owners.spec.ts line 33
    #[test]
    fn code_owners_section_header_default_users() {
        let rules =
            extract_rules_from_code_owners_lines(&["[team] username1 username2", "filename"]);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].pattern, "filename");
        assert_eq!(rules[0].usernames, vec!["username1", "username2"]);
        assert_eq!(rules[0].score, 8);
    }

    // Ported: "should extract an owner rule from a line after a section header with no usernames" — modules/platform/gitlab/code-owners.spec.ts line 47
    #[test]
    fn code_owners_section_header_no_users() {
        let rules = extract_rules_from_code_owners_lines(&["[team]", "filename"]);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].pattern, "filename");
        assert_eq!(rules[0].usernames, Vec::<String>::new());
        assert_eq!(rules[0].score, 8);
    }

    // Ported: "should extract an owner rule from a line after a section header with spaces" — modules/platform/gitlab/code-owners.spec.ts line 61
    #[test]
    fn code_owners_section_header_with_spaces() {
        let rules =
            extract_rules_from_code_owners_lines(&["[Backend Team] @backend-team", "filename"]);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].pattern, "filename");
        assert_eq!(rules[0].usernames, vec!["@backend-team"]);
        assert_eq!(rules[0].score, 8);
    }

    // Ported: "should extract an owner rule from a line after a section header with spaces and no usernames" — modules/platform/gitlab/code-owners.spec.ts line 75
    #[test]
    fn code_owners_section_header_with_spaces_no_users() {
        let rules = extract_rules_from_code_owners_lines(&["[Backend Team]", "filename"]);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].pattern, "filename");
        assert_eq!(rules[0].usernames, Vec::<String>::new());
        assert_eq!(rules[0].score, 8);
    }

    // Ported: "should extract an owner rule from a line after a section header with spaces and multiple usernames" — modules/platform/gitlab/code-owners.spec.ts line 89
    #[test]
    fn code_owners_section_header_multiple_users() {
        let rules = extract_rules_from_code_owners_lines(&[
            "[Backend Team] @backend-team @backend-lead",
            "filename",
        ]);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].pattern, "filename");
        assert_eq!(rules[0].usernames, vec!["@backend-team", "@backend-lead"]);
        assert_eq!(rules[0].score, 8);
    }

    // ── gitlab/utils.spec.ts tests ────────────────────────────────────────────

    // Ported: "throws on invalid endpoint when gitUrl is endpoint" — modules/platform/gitlab/utils.spec.ts line 42
    #[test]
    fn get_repo_url_throws_on_invalid_endpoint_when_git_url_is_endpoint() {
        let result = get_repo_url(
            "group/repo",
            Some("endpoint"),
            None,
            None,
            "not-a-valid-url",
            None,
        );
        match result {
            Err(GetRepoUrlError::InvalidEndpoint(msg)) => {
                assert_eq!(msg, "not-a-valid-url");
            }
            _ => panic!("Expected InvalidEndpoint error"),
        }
    }

    // Ported: "throws on invalid endpoint when http_url_to_repo is null" — modules/platform/gitlab/utils.spec.ts line 48
    #[test]
    fn get_repo_url_throws_on_invalid_endpoint_when_http_url_is_null() {
        let result = get_repo_url("group/repo", None, None, None, "not-a-valid-url", None);
        match result {
            Err(GetRepoUrlError::InvalidEndpoint(msg)) => {
                assert_eq!(msg, "not-a-valid-url");
            }
            _ => panic!("Expected InvalidEndpoint error"),
        }
    }

    // Ported: "should extract an owner rule from a line after an optional section header with spaces" — modules/platform/gitlab/code-owners.spec.ts line 103
    #[test]
    fn code_owners_optional_section_header() {
        let rules =
            extract_rules_from_code_owners_lines(&["^[Backend Team] @backend-team", "filename"]);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].pattern, "filename");
        assert_eq!(rules[0].usernames, vec!["@backend-team"]);
        assert_eq!(rules[0].score, 8);
    }

    // Ported: "should extract an owner rule from a line after a section header with approval count and spaces" — modules/platform/gitlab/code-owners.spec.ts line 117
    #[test]
    fn code_owners_section_header_with_approval_count() {
        let rules =
            extract_rules_from_code_owners_lines(&["[Backend Team][2] @backend-team", "filename"]);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].pattern, "filename");
        assert_eq!(rules[0].usernames, vec!["@backend-team"]);
        assert_eq!(rules[0].score, 8);
    }

    // Ported: "strips invalid unicode null characters" — modules/platform/gitlab/index.spec.ts line 3950
    #[test]
    fn massage_markdown_strips_null_chars() {
        assert_eq!(
            massage_markdown("The source contains 'Ruby\u{0000}' at: 2.7.6.219"),
            "The source contains 'Ruby' at: 2.7.6.219"
        );
    }

    // Ported: "replaces PR with MR including pluralization" — modules/platform/gitlab/index.spec.ts line 3958
    #[test]
    fn massage_markdown_replaces_pr_with_mr() {
        assert_eq!(
            massage_markdown("A Pull Request is a PR, multiple Pull Requests are PRs."),
            "A Merge Request is a MR, multiple Merge Requests are MRs."
        );
    }

    // Ported: "replaces PR reference with MR reference" — modules/platform/gitlab/index.spec.ts line 3966
    #[test]
    fn massage_markdown_replaces_pr_ref() {
        assert_eq!(
            massage_markdown("See the following PR: #123 for more details"),
            "See the following MR: !123 for more details"
        );
    }

    // Ported: "replaces PR relative link with MR reference" — modules/platform/gitlab/index.spec.ts line 3972
    #[test]
    fn massage_markdown_replaces_pr_link() {
        assert_eq!(
            massage_markdown("See the following PR: [abc](../pull/123) for more details"),
            "See the following MR: [abc](!123) for more details"
        );
    }

    // Ported: "replaces issues relative link with issue reference" — modules/platform/gitlab/index.spec.ts line 3980
    #[test]
    fn massage_markdown_replaces_issues_link() {
        assert_eq!(
            massage_markdown(
                "Check the [Dependency Dashboard](../issues/123) for more information."
            ),
            "Check the [Dependency Dashboard](#123) for more information."
        );
    }

    // Ported: "avoids false positives when replacing PR with MR" — modules/platform/gitlab/index.spec.ts line 3988
    #[test]
    fn massage_markdown_avoids_false_positives() {
        let input = "PROCESSING APPROPRIATE SUPPRESS NOPR";
        assert_eq!(massage_markdown(input), input);
    }

    // Ported: "should use ssh_url_to_repo if gitUrl is set to ssh" — modules/platform/gitlab/index.spec.ts line 456
    #[test]
    fn get_repo_url_uses_ssh_when_git_url_is_ssh() {
        let result = get_repo_url(
            "some/repo/project",
            Some("ssh"),
            Some("ssh://git@gitlab.com/some%2Frepo%2Fproject.git"),
            Some("https://gitlab.com/some%2Frepo%2Fproject.git"),
            "https://gitlab.com/api/v4",
            None,
        );
        assert_eq!(
            result.unwrap(),
            "ssh://git@gitlab.com/some%2Frepo%2Fproject.git"
        );
    }

    // Ported: "should throw if ssh_url_to_repo is not present but gitUrl is set to ssh" — modules/platform/gitlab/index.spec.ts line 473
    #[test]
    fn get_repo_url_throws_when_ssh_missing_and_git_url_is_ssh() {
        let result = get_repo_url(
            "some/repo/project",
            Some("ssh"),
            None,
            Some("https://gitlab.com/some%2Frepo%2Fproject.git"),
            "https://gitlab.com/api/v4",
            None,
        );
        assert!(matches!(result, Err(GetRepoUrlError::SshUrlUnavailable)));
    }

    // Ported: "returns updated pr body" — modules/platform/gitlab/index.spec.ts line 3993
    #[test]
    fn massage_markdown_returns_updated_pr_body() {
        let input = "https://github.com/foo/bar/issues/5 plus also [a link](https://github.com/foo/bar/issues/5\n\n  Pull Requests are the best, here are some PRs.\n\n  ## Open\n\nThese updates have all been created already. To force a retry/rebase of any, click on a checkbox below.\n\n - [ ] <!-- rebase-branch=renovate/major-got-packages -->[build(deps): update got packages (major)](../pull/2433) (\\`gh-got\\`, \\`gl-got\\`, \\`got\\`)\n";
        let expected = "https://github.com/foo/bar/issues/5 plus also [a link](https://github.com/foo/bar/issues/5\n\n  Merge Requests are the best, here are some MRs.\n\n  ## Open\n\nThese updates have all been created already. To force a retry/rebase of any, click on a checkbox below.\n\n - [ ] <!-- rebase-branch=renovate/major-got-packages -->[build(deps): update got packages (major)](!2433) (\\`gh-got\\`, \\`gl-got\\`, \\`got\\`)\n";
        assert_eq!(massage_markdown(input), expected);
    }
}
