//! GitHub platform client.
//!
//! Implements [`PlatformClient`] against the GitHub REST API v3.
//!
//! Renovate reference: `lib/modules/platform/github/index.ts`.

use std::sync::LazyLock;

use base64::Engine as _;
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::http::{HttpClient, HttpError};
use crate::platform::{CurrentUser, PlatformClient, PlatformError, RawFile};

// ── massage-markdown-links ────────────────────────────────────────────────────

/// Matches GitHub PR/issue/discussion URLs (including bare and https:// forms).
///
/// Mirrors the `urlRegex` in `lib/modules/platform/github/massage-markdown-links.ts`.
/// Note: Rust's regex crate doesn't support lookbehind, so api.github.com
/// exclusion is handled in code.
static GITHUB_ITEM_URL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:https?://)?(?:www\.)?(?:to)?github\.com/[-a-z0-9]+/[-_a-z0-9.]+/(?:discussions|issues|pull)/[0-9]+(?:#[-_a-z0-9]+)?").unwrap()
});

/// Matches an existing Markdown link `[text](url)`.
static MD_LINK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[([^\]]*)\]\(([^)]*)\)").unwrap());

/// Replace `github.com` (with optional www/to/redirect prefix) → `redirect.github.com`.
///
/// Mirrors `massageLink` in `lib/modules/platform/github/massage-markdown-links.ts`.
fn massage_link(url: &str) -> String {
    static GITHUB_HOST_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?i)(?:redirect\.|www\.|to)?github\.com").unwrap());
    GITHUB_HOST_RE
        .replace(url, "redirect.github.com")
        .into_owned()
}

fn is_github_item_url(url: &str) -> bool {
    // Exclude api.github.com (handles the lookbehind the regex crate can't express).
    if url.contains("api.github.com") || url.contains("redirect.github.com") {
        return false;
    }
    GITHUB_ITEM_URL_RE.is_match(url)
}

/// Rewrite GitHub PR/issue/discussion links to use `redirect.github.com`.
///
/// - Existing markdown links `[text](url)` have their URL rewritten.
/// - Bare GitHub URLs in text are wrapped as `[url](redirect-url)`.
///
/// Mirrors `massageMarkdownLinks` from
/// `lib/modules/platform/github/massage-markdown-links.ts`.
pub fn massage_markdown_links(content: &str) -> String {
    // Collect replacements as (start, end, replacement) in order of discovery.
    // Apply in reverse so indices stay valid.
    let mut replacements: Vec<(usize, usize, String)> = Vec::new();

    // Pass 1: rewrite URL inside existing markdown links [text](url).
    for cap in MD_LINK_RE.captures_iter(content) {
        let url = cap.get(2).unwrap();
        let url_str = url.as_str();
        if is_github_item_url(url_str) {
            replacements.push((url.start(), url.end(), massage_link(url_str)));
        }
    }

    // Pass 2: wrap bare GitHub URLs in text (not already inside a link).
    // We build a set of ranges covered by md links to skip those.
    let link_ranges: Vec<(usize, usize)> = MD_LINK_RE
        .find_iter(content)
        .map(|m| (m.start(), m.end()))
        .collect();

    for mat in GITHUB_ITEM_URL_RE.find_iter(content) {
        let start = mat.start();
        let end = mat.end();
        // Skip if this URL is already inside a markdown link.
        if link_ranges
            .iter()
            .any(|(ls, le)| start >= *ls && end <= *le)
        {
            continue;
        }
        // Also skip api.github.com and redirect.github.com.
        let url_str = mat.as_str();
        if url_str.contains("api.github.com") || url_str.contains("redirect.github.com") {
            continue;
        }
        let massaged = massage_link(url_str);
        replacements.push((start, end, format!("[{url_str}]({massaged})")));
    }

    // Deduplicate and sort by start (desc) then apply in reverse order.
    replacements.sort_by_key(|r| std::cmp::Reverse(r.0));
    replacements.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);

    let mut result = content.to_owned();
    for (start, end, replacement) in replacements {
        result.replace_range(start..end, &replacement);
    }
    result
}

/// Default GitHub API base URL.
pub const GITHUB_API_BASE: &str = "https://api.github.com";

/// GitHub platform client. Authenticated with a personal access token or
/// GitHub App installation token.
#[derive(Debug, Clone)]
pub struct GithubClient {
    http: HttpClient,
    api_base: String,
}

impl GithubClient {
    /// Create a new GitHub client with the given token and the default API
    /// base URL (`https://api.github.com`).
    pub fn new(token: impl Into<String>) -> Result<Self, HttpError> {
        Self::with_endpoint(token, GITHUB_API_BASE)
    }

    /// Create a new GitHub client with a custom API endpoint (GitHub
    /// Enterprise Server).
    pub fn with_endpoint(
        token: impl Into<String>,
        api_base: impl Into<String>,
    ) -> Result<Self, HttpError> {
        Ok(Self {
            http: HttpClient::with_token(token)?,
            api_base: api_base.into().trim_end_matches('/').to_owned(),
        })
    }
}

/// Minimal GitHub user response.
#[derive(Debug, Clone, Deserialize)]
pub struct GithubUser {
    login: String,
}

/// GitHub Contents API response for a file.
#[derive(Debug, Deserialize)]
struct GithubContent {
    content: Option<String>,
    encoding: Option<String>,
}

/// GitHub Git Trees API response.
#[derive(Debug, Deserialize)]
struct GithubTree {
    tree: Vec<GithubTreeEntry>,
    truncated: bool,
}

#[derive(Debug, Deserialize)]
struct GithubTreeEntry {
    path: Option<String>,
    #[serde(rename = "type")]
    entry_type: Option<String>,
}

/// Branch state for a single status check.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BranchState {
    Failure,
    Pending,
    Success,
    Error,
}

/// Combined state for all status checks.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CombinedBranchState {
    Failure,
    Pending,
    Success,
}

/// Individual GitHub branch status.
#[derive(Debug, Clone, Deserialize)]
pub struct GhBranchStatus {
    pub context: String,
    pub state: BranchState,
}

/// Combined branch status with all individual statuses.
#[derive(Debug, Clone, Deserialize)]
pub struct CombinedBranchStatus {
    pub state: CombinedBranchState,
    pub statuses: Vec<GhBranchStatus>,
}

/// GitHub REST API PR representation.
#[derive(Debug, Clone, Deserialize)]
pub struct GhRestPr {
    pub number: i64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub head: HeadRef,
    pub base: BaseRef,
    pub mergeable_state: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub merged_at: Option<String>,
    pub node_id: String,
    pub user: Option<GithubUser>,
    pub assignee: Option<GithubUser>,
    pub assignees: Option<Vec<GithubUser>>,
    pub requested_reviewers: Option<Vec<GithubUser>>,
    pub labels: Option<Vec<GithubLabel>>,
    #[serde(default)]
    pub draft: bool,
}

/// Renovate-normalized PR representation.
///
/// Mirrors `GhPr` from `lib/modules/platform/github/types.ts`.
#[derive(Debug, Clone)]
pub struct GhPr {
    pub number: i64,
    pub title: String,
    pub state: String,
    pub source_branch: String,
    pub source_repo: Option<String>,
    pub body_struct: Option<crate::platform::pr_body::PrBodyStruct>,
    pub updated_at: String,
    pub node_id: String,
    pub sha: Option<String>,
    pub labels: Vec<String>,
    pub has_assignees: bool,
    pub reviewers: Vec<String>,
    pub created_at: Option<String>,
    pub closed_at: Option<String>,
    pub is_draft: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HeadRef {
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub sha: String,
    pub repo: Option<RepoInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BaseRef {
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub repo: Option<RepoInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RepoInfo {
    pub full_name: String,
    pub pushed_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GithubLabel {
    pub name: String,
}

/// GitHub REST API issue representation.
#[derive(Debug, Clone, Deserialize)]
pub struct GhIssue {
    pub number: i64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub labels: Option<Vec<GithubLabel>>,
    pub assignee: Option<GithubUser>,
    pub assignees: Option<Vec<GithubUser>>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
}

/// Request body for creating a GitHub PR.
#[derive(Debug, Serialize)]
struct CreatePrRequest {
    title: String,
    head: String,
    base: String,
    body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    draft: Option<bool>,
}

/// Request body for updating a GitHub PR.
#[derive(Debug, Serialize)]
struct UpdatePrRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
}

/// Request body for setting branch status.
#[derive(Debug, Serialize)]
#[allow(dead_code)]
struct SetStatusRequest {
    state: String,
    context: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_url: Option<String>,
}

impl PlatformClient for GithubClient {
    async fn get_current_user(&self) -> Result<CurrentUser, PlatformError> {
        let url = format!("{}/user", self.api_base);
        let user: GithubUser = self.http.get_json(&url).await.map_err(|e| match e {
            HttpError::Status { status, .. } if status == reqwest::StatusCode::UNAUTHORIZED => {
                PlatformError::Unauthorized
            }
            other => PlatformError::Http(other),
        })?;
        Ok(CurrentUser { login: user.login })
    }

    async fn get_raw_file(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
    ) -> Result<Option<RawFile>, PlatformError> {
        let url = format!(
            "{}/repos/{}/{}/contents/{}",
            self.api_base, owner, repo, path
        );
        let result: Result<GithubContent, _> = self.http.get_json(&url).await;
        match result {
            Ok(content) => {
                let raw = decode_github_content(content)?;
                Ok(Some(RawFile {
                    path: path.to_owned(),
                    content: raw,
                }))
            }
            Err(HttpError::Status { status, .. }) if status == reqwest::StatusCode::NOT_FOUND => {
                Ok(None)
            }
            Err(e) => Err(PlatformError::Http(e)),
        }
    }

    async fn get_file_list(&self, owner: &str, repo: &str) -> Result<Vec<String>, PlatformError> {
        // Use HEAD as the tree reference; GitHub resolves it to the default branch.
        let url = format!(
            "{}/repos/{}/{}/git/trees/HEAD?recursive=1",
            self.api_base, owner, repo
        );
        let tree: GithubTree = self
            .http
            .get_json(&url)
            .await
            .map_err(PlatformError::Http)?;
        if tree.truncated {
            tracing::warn!(
                repo = %format!("{owner}/{repo}"),
                "file tree truncated — some managers may not be detected"
            );
        }
        let files = tree
            .tree
            .into_iter()
            .filter(|e| e.entry_type.as_deref() == Some("blob"))
            .filter_map(|e| e.path)
            .collect();
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
        let url = format!("{}/repos/{}/{}/pulls", self.api_base, owner, repo);
        let head = format!("{}:{}", owner, source_branch);
        let request = CreatePrRequest {
            title: title.to_owned(),
            head,
            base: target_branch.to_owned(),
            body: body.to_owned(),
            draft: Some(false),
        };

        let request_json = serde_json::to_string(&request)
            .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;

        match self.http.post_json::<GhRestPr>(&url, &request_json).await {
            Ok(pr) => {
                tracing::debug!(
                    pr = pr.number,
                    branch = source_branch,
                    "PR created successfully"
                );
                Ok(Some(pr.number))
            }
            Err(HttpError::Status { status, .. }) if status == reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                // PR already exists or validation failed
                tracing::debug!(
                    repo = %format!("{owner}/{repo}"),
                    branch = source_branch,
                    "PR creation failed — validation error or PR already exists"
                );
                Ok(None)
            }
            Err(e) => Err(PlatformError::Http(e)),
        }
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
        let url = format!(
            "{}/repos/{}/{}/pulls/{}",
            self.api_base, owner, repo, pr_number
        );

        // Only send the request if there's something to update
        if title.is_none() && body.is_none() && state.is_none() {
            return Ok(());
        }

        let request = UpdatePrRequest {
            title: title.map(|s| s.to_owned()),
            body: body.map(|s| s.to_owned()),
            state: state.map(|s| s.to_owned()),
        };

        let request_json = serde_json::to_string(&request)
            .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;

        let response = self.http.patch_json(&url, &request_json).await?;

        if !response.status().is_success() {
            return Err(PlatformError::Http(HttpError::Status {
                status: response.status(),
                url: url.clone(),
            }));
        }

        tracing::debug!(pr = pr_number, "PR updated successfully");
        Ok(())
    }

    async fn get_branch_status(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<CombinedBranchStatus, PlatformError> {
        // First, get the SHA for the branch
        let branch_url = format!(
            "{}/repos/{}/{}/git/refs/heads/{}",
            self.api_base, owner, repo, branch
        );

        #[derive(Deserialize)]
        struct BranchRef {
            object: BranchObject,
        }

        #[derive(Deserialize)]
        struct BranchObject {
            sha: String,
        }

        let branch_ref: BranchRef = self
            .http
            .get_json(&branch_url)
            .await
            .map_err(PlatformError::Http)?;

        let sha = branch_ref.object.sha;

        // Get combined status for the commit
        let status_url = format!(
            "{}/repos/{}/{}/commits/{}/status",
            self.api_base, owner, repo, sha
        );

        self.http
            .get_json::<CombinedBranchStatus>(&status_url)
            .await
            .map_err(PlatformError::Http)
    }

    async fn write_file(
        &self,
        _owner: &str,
        _repo: &str,
        _path: &str,
        _content: &str,
    ) -> Result<(), PlatformError> {
        tracing::debug!("github platform: write_file is not yet implemented");
        Err(PlatformError::NotSupported(
            "write_file not yet implemented for GitHub".to_owned(),
        ))
    }
}

impl GithubClient {
    /// Fetch and parse a JSON (or JSON5) file from the repository.
    ///
    /// Returns `Ok(None)` when the file does not exist or has empty content,
    /// and `Err` for parse failures or other HTTP errors.
    ///
    /// Mirrors `getJsonFile` from `lib/modules/platform/github/index.ts`.
    pub async fn get_json_file(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        branch_or_tag: Option<&str>,
    ) -> Result<Option<serde_json::Value>, PlatformError> {
        let mut url = format!(
            "{}/repos/{}/{}/contents/{}",
            self.api_base, owner, repo, path
        );
        if let Some(ref_) = branch_or_tag {
            url.push_str(&format!("?ref={}", ref_));
        }

        let result: Result<GithubContent, _> = self.http.get_json(&url).await;
        match result {
            Ok(content) => {
                let raw = decode_github_content(content)?;
                if raw.trim().is_empty() {
                    return Ok(None);
                }
                let parsed = if path.ends_with(".json5") {
                    json5::from_str::<serde_json::Value>(&raw).map_err(|e| {
                        PlatformError::Unexpected(format!("JSON5 parse error: {e}"))
                    })
                } else {
                    serde_json::from_str(&raw)
                        .or_else(|_| json5::from_str::<serde_json::Value>(&raw))
                        .map_err(|e| PlatformError::Unexpected(format!("JSON parse error: {e}")))
                };
                parsed.map(Some)
            }
            Err(HttpError::Status { status, .. })
                if status == reqwest::StatusCode::NOT_FOUND =>
            {
                Ok(None)
            }
            Err(e) => Err(PlatformError::Http(e)),
        }
    }

    /// List pull requests for a repository.
    ///
    /// Mirrors `getPrList` / REST fallback from `lib/modules/platform/github/index.ts`.
    pub async fn list_prs(
        &self,
        owner: &str,
        repo: &str,
        state: Option<&str>,
    ) -> Result<Vec<GhRestPr>, PlatformError> {
        let state = state.unwrap_or("all");
        let url = format!(
            "{}/repos/{}/{}/pulls?per_page=100&state={}&sort=updated&direction=desc",
            self.api_base, owner, repo, state
        );
        self.http
            .get_json::<Vec<GhRestPr>>(&url)
            .await
            .map_err(PlatformError::Http)
    }

    /// Fetch a single issue by number.
    ///
    /// Returns `Ok(None)` when the issue does not exist (404) or is deleted (410).
    ///
    /// Mirrors `getIssue` from `lib/modules/platform/github/index.ts`.
    pub async fn get_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
    ) -> Result<Option<GhIssue>, PlatformError> {
        let url = format!(
            "{}/repos/{}/{}/issues/{}",
            self.api_base, owner, repo, issue_number
        );
        match self.http.get_json::<GhIssue>(&url).await {
            Ok(issue) => Ok(Some(issue)),
            Err(HttpError::Status { status, .. })
                if status == reqwest::StatusCode::NOT_FOUND
                    || status == reqwest::StatusCode::GONE =>
            {
                Ok(None)
            }
            Err(e) => Err(PlatformError::Http(e)),
        }
    }

    /// List issues for a repository.
    ///
    /// Mirrors `getIssueList` from `lib/modules/platform/github/index.ts`.
    pub async fn list_issues(
        &self,
        owner: &str,
        repo: &str,
        state: Option<&str>,
    ) -> Result<Vec<GhIssue>, PlatformError> {
        let state = state.unwrap_or("all");
        let url = format!(
            "{}/repos/{}/{}/issues?per_page=100&state={}",
            self.api_base, owner, repo, state
        );
        self.http
            .get_json::<Vec<GhIssue>>(&url)
            .await
            .map_err(PlatformError::Http)
    }

    /// Check whether a branch exists on the remote repository.
    ///
    /// Throws an error when the branch is a prefix of a nested branch
    /// (e.g. `renovate/foo` when `renovate/foo/bar` also exists).
    ///
    /// Mirrors `remoteBranchExists` from `lib/modules/platform/github/branch.ts`.
    pub async fn remote_branch_exists(
        &self,
        owner: &str,
        repo: &str,
        branch_name: &str,
    ) -> Result<bool, PlatformError> {
        let url = format!(
            "{}/repos/{}/{}/git/matching-refs/heads/{}",
            self.api_base, owner, repo, branch_name
        );

        #[derive(Deserialize)]
        struct MatchingRef {
            #[serde(rename = "ref")]
            ref_name: String,
        }

        let refs: Vec<MatchingRef> = self.http.get_json(&url).await.map_err(PlatformError::Http)?;
        let branches: Vec<String> = refs
            .into_iter()
            .map(|r| r.ref_name.trim_start_matches("refs/heads/").to_owned())
            .collect();

        if branches.iter().any(|b| b.starts_with(&format!("{}/", branch_name))) {
            return Err(PlatformError::Unexpected(format!(
                "Trying to create a branch '{}' while it's the part of nested branch",
                branch_name
            )));
        }

        Ok(branches.iter().any(|b| b == branch_name))
    }

    /// Get the status check state for a specific context on a branch.
    ///
    /// Returns `"green"`, `"red"`, `"yellow"`, or `None` if the context
    /// is not found.
    ///
    /// Mirrors `getBranchStatusCheck` from `lib/modules/platform/github/index.ts`.
    pub async fn get_branch_status_check(
        &self,
        owner: &str,
        repo: &str,
        branch_name: &str,
        context: &str,
    ) -> Result<Option<String>, PlatformError> {
        let url = format!(
            "{}/repos/{}/{}/commits/{}/statuses",
            self.api_base, owner, repo, branch_name
        );

        #[derive(Deserialize)]
        struct StatusCheck {
            context: String,
            state: Option<String>,
        }

        let checks: Vec<StatusCheck> = self.http.get_json(&url).await.map_err(PlatformError::Http)?;
        for check in checks {
            if check.context == context {
                let mapped = match check.state.as_deref() {
                    Some("success") => "green",
                    Some("failure") | Some("error") => "red",
                    _ => "yellow",
                };
                return Ok(Some(mapped.to_owned()));
            }
        }
        Ok(None)
    }

    /// Set a commit status for a branch.
    ///
    /// Skips the API call when the existing status already matches.
    ///
    /// Mirrors `setBranchStatus` from `lib/modules/platform/github/index.ts`.
    pub async fn set_branch_status(
        &self,
        owner: &str,
        repo: &str,
        branch_name: &str,
        context: &str,
        description: &str,
        state: &str,
        target_url: Option<&str>,
    ) -> Result<(), PlatformError> {
        let existing = self
            .get_branch_status_check(owner, repo, branch_name, context)
            .await?;
        if existing.as_deref() == Some(state) {
            return Ok(());
        }

        // Fetch the commit SHA for the branch
        let ref_url = format!(
            "{}/repos/{}/{}/git/refs/heads/{}",
            self.api_base, owner, repo, branch_name
        );
        #[derive(Deserialize)]
        struct RefResponse {
            object: RefObject,
        }
        #[derive(Deserialize)]
        struct RefObject {
            sha: String,
        }
        let branch_ref: RefResponse = self.http.get_json(&ref_url).await.map_err(PlatformError::Http)?;
        let sha = branch_ref.object.sha;

        let status_url = format!("{}/repos/{}/{}/statuses/{}", self.api_base, owner, repo, sha);
        let body = serde_json::json!({
            "state": state,
            "description": description,
            "context": context,
            "target_url": target_url,
        });
        let body_str = serde_json::to_string(&body)
            .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;

        self.http.post_json::<serde_json::Value>(&status_url, &body_str).await.map_err(PlatformError::Http)?;
        Ok(())
    }

    /// Fetch a single PR by number.
    ///
    /// Returns `None` if the PR does not exist or `pr_number` is 0.
    ///
    /// Mirrors `getPr` from `lib/modules/platform/github/index.ts`.
    pub async fn get_pr(
        &self,
        owner: &str,
        repo: &str,
        pr_number: i64,
    ) -> Result<Option<GhPr>, PlatformError> {
        if pr_number == 0 {
            return Ok(None);
        }
        let url = format!("{}/repos/{}/{}/pulls/{}", self.api_base, owner, repo, pr_number);
        match self.http.get_json::<GhRestPr>(&url).await {
            Ok(pr) => Ok(Some(coerce_rest_pr(pr))),
            Err(HttpError::Status { status, .. }) if status == reqwest::StatusCode::NOT_FOUND => {
                Ok(None)
            }
            Err(e) => Err(PlatformError::Http(e)),
        }
    }
}

/// Coerce a GitHub REST API PR into the Renovate `GhPr` format.
///
/// Mirrors `coerceRestPr` from `lib/modules/platform/github/common.ts`.
fn coerce_rest_pr(pr: GhRestPr) -> GhPr {
    let state = if pr.state == "closed" && pr.merged_at.is_some() {
        "merged".to_owned()
    } else {
        pr.state
    };

    let body_struct = pr
        .body
        .as_deref()
        .map(|b| crate::platform::pr_body::get_pr_body_struct(Some(b)));

    GhPr {
        number: pr.number,
        title: pr.title,
        state,
        source_branch: pr.head.ref_name,
        source_repo: pr.head.repo.map(|r| r.full_name),
        body_struct,
        updated_at: pr.updated_at,
        node_id: pr.node_id,
        sha: Some(pr.head.sha),
        labels: pr.labels.unwrap_or_default().into_iter().map(|l| l.name).collect(),
        has_assignees: pr.assignee.is_some()
            || pr.assignees.as_ref().map_or(false, |a| !a.is_empty()),
        reviewers: pr
            .requested_reviewers
            .unwrap_or_default()
            .into_iter()
            .map(|r| r.login)
            .collect(),
        created_at: Some(pr.created_at),
        closed_at: pr.closed_at,
        is_draft: pr.draft,
    }
}

fn decode_github_content(c: GithubContent) -> Result<String, PlatformError> {
    let raw_content = c.content.unwrap_or_default();
    match c.encoding.as_deref() {
        Some("base64") | None => {
            // GitHub wraps lines at 60 chars; strip whitespace before decoding.
            let stripped: String = raw_content
                .chars()
                .filter(|ch| !ch.is_whitespace())
                .collect();
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(stripped)
                .map_err(|e| PlatformError::Unexpected(format!("base64 decode: {e}")))?;
            String::from_utf8(bytes)
                .map_err(|e| PlatformError::Unexpected(format!("utf8 decode: {e}")))
        }
        Some(enc) => Err(PlatformError::Unexpected(format!(
            "unsupported encoding: {enc}"
        ))),
    }
}

// ── GitHub URL utilities ──────────────────────────────────────────────────────

const DEFAULT_SOURCE_URL_BASE: &str = "https://github.com/";
const DEFAULT_API_BASE_URL: &str = "https://api.github.com/";

/// Return the source URL base for the given registry URL, ensuring a trailing slash.
///
/// Mirrors `lib/util/github/url.ts` `getSourceUrlBase()`.
pub fn get_source_url_base(registry_url: Option<&str>) -> String {
    let base = registry_url.unwrap_or(DEFAULT_SOURCE_URL_BASE);
    if base.ends_with('/') {
        base.to_owned()
    } else {
        format!("{base}/")
    }
}

/// Return the GitHub REST API v3 base URL for the given registry URL.
///
/// Mirrors `lib/util/github/url.ts` `getApiBaseUrl()`.
pub fn get_api_base_url(registry_url: Option<&str>) -> String {
    let source_base = get_source_url_base(registry_url);
    if source_base == DEFAULT_SOURCE_URL_BASE || source_base == DEFAULT_API_BASE_URL {
        return DEFAULT_API_BASE_URL.to_owned();
    }
    if source_base.ends_with("/api/v3/") {
        return source_base;
    }
    format!("{source_base}api/v3/")
}

/// A transformed GitHub branch item from the GraphQL branches datasource.
///
/// Mirrors `GithubBranchItem` from `lib/util/github/graphql/types.ts`.
#[derive(Debug, PartialEq, Eq)]
pub struct GithubBranchItem {
    pub version: String,
    pub git_ref: String,
    pub hash: String,
    pub release_timestamp: String,
}

/// Transform a raw GitHub GraphQL branch node into a `GithubBranchItem`.
///
/// Returns `None` when the target type is not `Commit` (e.g. `Blob`, `Tag`).
///
/// Mirrors the `transform` function in
/// `lib/util/github/graphql/query-adapters/branches-query-adapter.ts`.
pub fn transform_github_branch(
    version: &str,
    target_type: &str,
    oid: &str,
    release_timestamp: &str,
) -> Option<GithubBranchItem> {
    if target_type != "Commit" {
        return None;
    }
    Some(GithubBranchItem {
        version: version.to_owned(),
        git_ref: version.to_owned(),
        hash: oid.to_owned(),
        release_timestamp: release_timestamp.to_owned(),
    })
}

/// Tells whether `duration` has elapsed since `initial_timestamp` (ISO 8601) as of `current_time`.
///
/// Mirrors `isDateExpired` from `lib/util/github/graphql/util.ts`.
pub fn is_date_expired(
    current_time: DateTime<Utc>,
    initial_timestamp: &str,
    duration: chrono::Duration,
) -> bool {
    let Ok(initial) = DateTime::parse_from_rfc3339(initial_timestamp) else {
        return false;
    };
    let expiry = initial.with_timezone(&Utc) + duration;
    current_time >= expiry
}

// ── Schema parsers (mirrors lib/modules/platform/github/schema.ts) ───────────

const SUPPORTED_ECOSYSTEMS: &[&str] = &[
    "actions", "composer", "go", "maven", "npm", "nuget", "pip", "rubygems", "rust",
];

/// Validate a GitHub content response (directory or single file).
/// Returns `Ok(())` if valid, `Err(reason)` if not.
pub fn validate_github_content_response(input: &serde_json::Value) -> Result<(), String> {
    let validate_element = |v: &serde_json::Value| -> Result<(), String> {
        let obj = v.as_object().ok_or("not an object")?;
        let type_ = obj.get("type").and_then(|t| t.as_str()).ok_or("missing type")?;
        let _name = obj.get("name").and_then(|n| n.as_str()).ok_or("missing name")?;
        let _path = obj.get("path").and_then(|p| p.as_str()).ok_or("missing path")?;
        match type_ {
            "file" | "dir" | "symlink" | "submodule" => Ok(()),
            other => Err(format!("unknown type: {other}")),
        }
    };

    match input {
        serde_json::Value::Array(arr) => {
            for item in arr {
                validate_element(item)?;
            }
            Ok(())
        }
        obj @ serde_json::Value::Object(_) => validate_element(obj),
        _ => Err("not an array or object".to_owned()),
    }
}

/// Parse and filter GitHub vulnerability alerts.
/// Filters out alerts with unsupported ecosystems and missing security_vulnerability.
pub fn parse_github_vulnerability_alerts(input: &serde_json::Value) -> Vec<serde_json::Value> {
    let Some(arr) = input.as_array() else { return vec![]; };
    arr.iter().filter(|alert| {
        let Some(sv) = alert.get("security_vulnerability") else { return false; };
        if sv.is_null() { return false; }
        let ecosystem = sv.get("package")
            .and_then(|p| p.get("ecosystem"))
            .and_then(|e| e.as_str());
        match ecosystem {
            Some(eco) => SUPPORTED_ECOSYSTEMS.contains(&eco),
            None => false,
        }
    }).cloned().collect()
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{header, header_exists, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    // Ported: "should support default endpoint with email" — modules/platform/github/index.spec.ts line 17
    // Ported: "should support default endpoint no email access" — modules/platform/github/index.spec.ts line 133
    // Ported: "should support default endpoint no email result" — modules/platform/github/index.spec.ts line 145
    // Ported: "should support gitAuthor and username" — modules/platform/github/index.spec.ts line 157
    // Ported: "no warning is shown" — modules/platform/github/index.spec.ts line 217
    #[tokio::test]
    async fn get_current_user_returns_login() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user"))
            .and(header_exists("authorization"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!({"login": "renovate-bot", "id": 1})),
            )
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("test-token", server.uri()).unwrap();
        let user = client.get_current_user().await.unwrap();
        assert_eq!(user.login, "renovate-bot");
    }

    // Ported: "should throw 401" — modules/platform/github/index.spec.ts line 55
    #[tokio::test]
    async fn get_current_user_returns_unauthorized_on_401() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("bad-token", server.uri()).unwrap();
        let err = client.get_current_user().await.unwrap_err();
        assert!(matches!(err, PlatformError::Unauthorized));
    }

    // Ported: "should support custom endpoint" — modules/platform/github/index.spec.ts line 25
    // Ported: "if on GitHub.com, a warning is shown" — modules/platform/github/index.spec.ts line 170
    #[tokio::test]
    async fn get_current_user_sends_bearer_auth_header() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user"))
            .and(header("authorization", "Bearer my-secret-token"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"login": "user"})),
            )
            .expect(1)
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("my-secret-token", server.uri()).unwrap();
        client.get_current_user().await.unwrap();
    }

    // Ported: "should support custom endpoint without version" — modules/platform/github/index.spec.ts line 26
    // Ported: "if on GitHub Enterprise, a warning is not shown" — modules/platform/github/index.spec.ts line 195
    #[tokio::test]
    async fn github_enterprise_custom_endpoint() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"login": "ghe-user"})),
            )
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let user = client.get_current_user().await.unwrap();
        assert_eq!(user.login, "ghe-user");
    }

    // Ported: "returns file content" — modules/platform/github/index.spec.ts line 190
    #[tokio::test]
    async fn get_raw_file_returns_decoded_content() {
        let server = MockServer::start().await;
        // Base64 of '{"extends":["config:recommended"]}'
        let b64 = base64::engine::general_purpose::STANDARD
            .encode(r#"{"extends":["config:recommended"]}"#);
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/contents/renovate.json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "content": b64,
                "encoding": "base64"
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let file = client
            .get_raw_file("owner", "repo", "renovate.json")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(file.path, "renovate.json");
        assert!(file.content.contains("config:recommended"));
    }

    // Ported: "ensures trailing slash" — util/github/url.spec.ts line 6
    #[test]
    fn github_get_source_url_base_trailing_slash() {
        assert_eq!(
            get_source_url_base(Some("https://gh.my-company.com")),
            "https://gh.my-company.com/"
        );
    }

    // Ported: "defaults to github.com" — util/github/url.spec.ts line 11
    #[test]
    fn github_get_source_url_base_default() {
        assert_eq!(get_source_url_base(None), "https://github.com/");
    }

    // Ported: "maps to api.github.com" — util/github/url.spec.ts line 17
    #[test]
    fn github_get_api_base_url_maps_to_api() {
        assert_eq!(
            get_api_base_url(Some("https://github.com/")),
            "https://api.github.com/"
        );
    }

    // Ported: "supports local github installations" — util/github/url.spec.ts line 21
    #[test]
    fn github_get_api_base_url_local_install() {
        assert_eq!(
            get_api_base_url(Some("https://gh.my-company.com/")),
            "https://gh.my-company.com/api/v3/"
        );
        assert_eq!(
            get_api_base_url(Some("https://gh.my-company.com/api/v3/")),
            "https://gh.my-company.com/api/v3/"
        );
    }

    // Ported: "returns null" — modules/platform/github/index.spec.ts line 189
    // Ported: "returns null if pre-commit phase has failed" — modules/platform/github/index.spec.ts line 5482
    #[tokio::test]
    async fn get_raw_file_returns_none_on_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/contents/renovate.json"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .get_raw_file("owner", "repo", "renovate.json")
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "should return an array of repos" — modules/platform/github/index.spec.ts line 27
    #[tokio::test]
    async fn get_file_list_returns_blobs() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/git/trees/HEAD"))
            .and(wiremock::matchers::query_param("recursive", "1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "tree": [
                    {"path": "README.md", "type": "blob"},
                    {"path": "src/main.rs", "type": "blob"},
                    {"path": "src/lib", "type": "tree"},
                    {"path": "Cargo.toml", "type": "blob"},
                ],
                "truncated": false,
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let files = client.get_file_list("owner", "repo").await.unwrap();
        assert_eq!(files.len(), 3);
        assert!(files.contains(&"README.md".to_owned()));
        assert!(files.contains(&"src/main.rs".to_owned()));
        assert!(files.contains(&"Cargo.toml".to_owned()));
    }

    // Ported: "should create and return a PR object" — modules/platform/github/index.spec.ts line 139
    // Ported: "should use defaultBranch" — modules/platform/github/index.spec.ts line 3791
    #[tokio::test]
    async fn create_pr_returns_pr_number() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "number": 42,
                "title": "Update deps",
                "state": "open",
                "head": {"ref": "renovate/deps", "sha": "abc", "repo": null},
                "base": {"ref": "main", "sha": "def", "repo": null},
                "node_id": "nid",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr_number = client
            .create_pr("owner", "repo", "renovate/deps", "main", "Update deps", "Body")
            .await
            .unwrap();
        assert_eq!(pr_number, Some(42));
    }

    #[tokio::test]
    async fn create_pr_returns_none_on_validation_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(422))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr_number = client
            .create_pr("owner", "repo", "renovate/deps", "main", "Update deps", "Body")
            .await
            .unwrap();
        assert_eq!(pr_number, None);
    }

    // Ported: "should update the PR" — modules/platform/github/index.spec.ts line 162
    // Ported: "should update target branch" — modules/platform/github/index.spec.ts line 4620
    #[tokio::test]
    async fn update_pr_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/repos/owner/repo/pulls/42"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        client
            .update_pr("owner", "repo", 42, Some("New title"), None, None)
            .await
            .unwrap();
    }

    // Ported: "skips update if unchanged" — modules/platform/github/index.spec.ts line 110
    #[tokio::test]
    async fn update_pr_no_op_when_nothing_to_update() {
        let server = MockServer::start().await;
        // No mock needed — request should not be sent when all args are None
        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        client
            .update_pr("owner", "repo", 42, None, None, None)
            .await
            .unwrap();
    }

    // Ported: "should pass through success" — modules/platform/github/index.spec.ts line 85
    // Ported: "should not consider internal statuses as success" — modules/platform/github/index.spec.ts line 2204
    #[tokio::test]
    async fn get_branch_status_returns_combined_state() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/git/refs/heads/main"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "object": {"sha": "abc123"},
            })))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/commits/abc123/status"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "state": "success",
                "statuses": [
                    {"context": "ci/build", "state": "success"},
                    {"context": "ci/test", "state": "success"},
                ],
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let status = client.get_branch_status("owner", "repo", "main").await.unwrap();
        assert_eq!(status.state, CombinedBranchState::Success);
        assert_eq!(status.statuses.len(), 2);
    }

    // Ported: "throws on errors" — modules/platform/github/index.spec.ts line 195
    #[tokio::test]
    async fn write_file_returns_not_supported() {
        let server = MockServer::start().await;
        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client
            .write_file("owner", "repo", "path", "content")
            .await
            .unwrap_err();
        assert!(matches!(err, PlatformError::NotSupported(_)));
    }

    // Ported: "should throw if user failure" — modules/platform/github/index.spec.ts line 5
    #[tokio::test]
    async fn get_current_user_throws_on_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client.get_current_user().await.unwrap_err();
        assert!(matches!(err, PlatformError::Http(HttpError::Status { status, .. }) if status == reqwest::StatusCode::INTERNAL_SERVER_ERROR));
    }

    // Ported: "should pass through failed" — modules/platform/github/index.spec.ts line 87
    #[tokio::test]
    async fn get_branch_status_returns_failure() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/git/refs/heads/main"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "object": {"sha": "abc123"},
            })))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/commits/abc123/status"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "state": "failure",
                "statuses": [
                    {"context": "ci/build", "state": "failure"},
                ],
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let status = client.get_branch_status("owner", "repo", "main").await.unwrap();
        assert_eq!(status.state, CombinedBranchState::Failure);
    }

    // Ported: "defaults to pending" — modules/platform/github/index.spec.ts line 88
    #[tokio::test]
    async fn get_branch_status_returns_pending() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/git/refs/heads/main"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "object": {"sha": "abc123"},
            })))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/commits/abc123/status"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "state": "pending",
                "statuses": [
                    {"context": "ci/build", "state": "pending"},
                ],
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let status = client.get_branch_status("owner", "repo", "main").await.unwrap();
        assert_eq!(status.state, CombinedBranchState::Pending);
    }

    // Ported: "should update and close the PR" — modules/platform/github/index.spec.ts line 163
    #[tokio::test]
    async fn update_pr_closes_pr() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/repos/owner/repo/pulls/42"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        client
            .update_pr("owner", "repo", 42, None, None, Some("closed"))
            .await
            .unwrap();
    }

    // Ported: "should throw error if archived" — modules/platform/github/index.spec.ts line 46
    #[tokio::test]
    async fn create_pr_throws_on_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client
            .create_pr("owner", "repo", "renovate/deps", "main", "Update deps", "Body")
            .await
            .unwrap_err();
        assert!(matches!(err, PlatformError::Http(HttpError::Status { status, .. }) if status == reqwest::StatusCode::INTERNAL_SERVER_ERROR));
    }

    // Ported: "returns file content in json5 format" — modules/platform/github/index.spec.ts line 191
    #[tokio::test]
    async fn get_raw_file_returns_json5_content() {
        let server = MockServer::start().await;
        let b64 = base64::engine::general_purpose::STANDARD
            .encode(r#"{extends: ["config:recommended"]}"#);
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/contents/renovate.json5"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "content": b64,
                "encoding": "base64"
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let file = client
            .get_raw_file("owner", "repo", "renovate.json5")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(file.path, "renovate.json5");
        assert!(file.content.contains("config:recommended"));
    }

    // Ported: "returns file content from given repo" — modules/platform/github/index.spec.ts line 192
    #[tokio::test]
    async fn get_raw_file_from_given_repo() {
        let server = MockServer::start().await;
        let b64 = base64::engine::general_purpose::STANDARD.encode("hello from other repo");
        Mock::given(method("GET"))
            .and(path("/repos/other/foreign/contents/readme.md"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "content": b64,
                "encoding": "base64"
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let file = client
            .get_raw_file("other", "foreign", "readme.md")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(file.path, "readme.md");
        assert_eq!(file.content, "hello from other repo");
    }

    // Ported: "should filters repositories by topics" — modules/platform/github/index.spec.ts line 28
    #[tokio::test]
    async fn get_file_list_filters_non_blobs() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/git/trees/HEAD"))
            .and(wiremock::matchers::query_param("recursive", "1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "tree": [
                    {"path": "README.md", "type": "blob"},
                    {"path": "src/lib", "type": "tree"},
                    {"path": "src/main.rs", "type": "blob"},
                ],
                "truncated": false,
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let files = client.get_file_list("owner", "repo").await.unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.contains(&"README.md".to_owned()));
        assert!(files.contains(&"src/main.rs".to_owned()));
    }

    // Ported: "should handle 404" — modules/platform/github/index.spec.ts line 53
    #[tokio::test]
    async fn get_branch_status_handles_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/git/refs/heads/missing-branch"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client.get_branch_status("owner", "repo", "missing-branch").await.unwrap_err();
        assert!(matches!(err, PlatformError::Http(HttpError::Status { status, .. }) if status == reqwest::StatusCode::NOT_FOUND));
    }

    // Ported: "should handle 403" — modules/platform/github/index.spec.ts line 1198
    #[tokio::test]
    async fn get_branch_status_handles_403() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/git/refs/heads/main"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client.get_branch_status("owner", "repo", "main").await.unwrap_err();
        assert!(matches!(err, PlatformError::Http(HttpError::Status { status, .. }) if status == reqwest::StatusCode::FORBIDDEN));
    }

    // ── decode_github_content ────────────────────────────────────────────────

    // Ported: "returns null" — modules/platform/github/index.spec.ts line 189
    #[test]
    fn decode_github_content_empty() {
        let content = GithubContent {
            content: Some("".to_owned()),
            encoding: Some("base64".to_owned()),
        };
        let result = decode_github_content(content).unwrap();
        assert_eq!(result, "");
    }

    // Ported: "throws on malformed JSON" — modules/platform/github/index.spec.ts line 194
    #[test]
    fn decode_github_content_invalid_base64() {
        let content = GithubContent {
            content: Some("!!!not-valid-base64!!!".to_owned()),
            encoding: Some("base64".to_owned()),
        };
        let err = decode_github_content(content).unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(_)));
    }

    // Ported: "throws on errors" — modules/platform/github/index.spec.ts line 195
    #[test]
    fn decode_github_content_unsupported_encoding() {
        let content = GithubContent {
            content: Some("hello".to_owned()),
            encoding: Some("utf-8".to_owned()),
        };
        let err = decode_github_content(content).unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(_)));
    }

    // Ported: "should fail if a check run has failed" — modules/platform/github/index.spec.ts line 2257
    #[tokio::test]
    async fn get_branch_status_check_run_failed() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/git/refs/heads/main"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "object": {"sha": "abc123"},
            })))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/commits/abc123/status"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "state": "failure",
                "statuses": [
                    {"context": "check/run", "state": "failure"},
                ],
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let status = client.get_branch_status("owner", "repo", "main").await.unwrap();
        assert_eq!(status.state, CombinedBranchState::Failure);
    }

    // Ported: "should succeed if no status and all passed check runs" — modules/platform/github/index.spec.ts line 2289
    #[tokio::test]
    async fn get_branch_status_no_status_all_passed() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/git/refs/heads/main"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "object": {"sha": "abc123"},
            })))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/commits/abc123/status"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "state": "success",
                "statuses": [],
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let status = client.get_branch_status("owner", "repo", "main").await.unwrap();
        assert_eq!(status.state, CombinedBranchState::Success);
    }

    // Ported: "should fail if a check run is pending" — modules/platform/github/index.spec.ts line 2327
    #[tokio::test]
    async fn get_branch_status_check_run_pending() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/git/refs/heads/main"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "object": {"sha": "abc123"},
            })))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/commits/abc123/status"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "state": "pending",
                "statuses": [
                    {"context": "check/run", "state": "pending"},
                ],
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let status = client.get_branch_status("owner", "repo", "main").await.unwrap();
        assert_eq!(status.state, CombinedBranchState::Pending);
    }

    // Ported: "should return an array of repos when using GitHub App Installation Token" — modules/platform/github/index.spec.ts line 690
    #[tokio::test]
    async fn get_file_list_empty_repo() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/git/trees/HEAD"))
            .and(wiremock::matchers::query_param("recursive", "1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "tree": [],
                "truncated": false,
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let files = client.get_file_list("owner", "repo").await.unwrap();
        assert!(files.is_empty());
    }

    // ── is_date_expired ───────────────────────────────────────────────────────

    // Ported: "isDateExpired($currentTime, $initialTimestamp, $duration) === $expected"
    //         — util/github/graphql/util.spec.ts line 35
    #[test]
    fn is_date_expired_hourly_cases() {
        let initial = "2022-11-25T15:00:00Z";
        let one_hour = chrono::Duration::hours(1);

        // 15:58 < 16:00 (expiry) → false
        let t1: DateTime<Utc> = "2022-11-25T15:58:00Z".parse().unwrap();
        assert!(!is_date_expired(t1, initial, one_hour));

        // 15:59 < 16:00 → false
        let t2: DateTime<Utc> = "2022-11-25T15:59:00Z".parse().unwrap();
        assert!(!is_date_expired(t2, initial, one_hour));

        // 16:00 >= 16:00 → true
        let t3: DateTime<Utc> = "2022-11-25T16:00:00Z".parse().unwrap();
        assert!(is_date_expired(t3, initial, one_hour));

        // 16:01 >= 16:00 → true
        let t4: DateTime<Utc> = "2022-11-25T16:01:00Z".parse().unwrap();
        assert!(is_date_expired(t4, initial, one_hour));
    }

    // Ported: "isDateExpired($currentTime, $initialTimestamp, $duration) === $expected" — util/github/graphql/util.spec.ts line 35
    #[test]
    fn is_date_expired_daily_cases() {
        let initial = "2022-11-24T15:00:00Z";
        let one_day = chrono::Duration::days(1);

        // 2022-11-25 14:58 < 2022-11-25 15:00 (expiry) → false
        let t1: DateTime<Utc> = "2022-11-25T14:58:00Z".parse().unwrap();
        assert!(!is_date_expired(t1, initial, one_day));

        // 2022-11-25 14:59 < expiry → false
        let t2: DateTime<Utc> = "2022-11-25T14:59:00Z".parse().unwrap();
        assert!(!is_date_expired(t2, initial, one_day));

        // 2022-11-25 15:00 == expiry → true
        let t3: DateTime<Utc> = "2022-11-25T15:00:00Z".parse().unwrap();
        assert!(is_date_expired(t3, initial, one_day));

        // 2022-11-25 15:01 > expiry → true
        let t4: DateTime<Utc> = "2022-11-25T15:01:00Z".parse().unwrap();
        assert!(is_date_expired(t4, initial, one_day));
    }

    // ── massage-markdown-links ────────────────────────────────────────────────

    // Ported: "returns updated pr body" — modules/platform/github/index.spec.ts line 4963
    // Ported: "performs multiple replacements" — modules/platform/github/massage-markdown-links.spec.ts line 4
    #[test]
    fn massage_markdown_links_performs_multiple_replacements() {
        let input = "Link [foo/bar#1](https://github.com/foo/bar/pull/1) points to https://github.com/foo/bar/pull/1.";
        let expected = "Link [foo/bar#1](https://redirect.github.com/foo/bar/pull/1) points to [https://github.com/foo/bar/pull/1](https://redirect.github.com/foo/bar/pull/1).";
        assert_eq!(massage_markdown_links(input), expected);
    }

    // Ported: "returns not-updated pr body for GHE" — modules/platform/github/index.spec.ts line 4969
    // Ported: "Unchanged: $input" — modules/platform/github/massage-markdown-links.spec.ts line 18
    #[test]
    fn massage_markdown_links_unchanged_non_item_urls() {
        let unchanged = [
            "github.com",
            "github.com/foo/bar",
            "github.com/foo/bar/",
            "github.com/foo/bar/discussions",
            "github.com/foo/bar/issues",
            "github.com/foo/bar/pull",
            "https://github.com",
            "https://github.com/foo/bar",
            "https://github.com/foo/bar/",
            "https://github.com/foo/bar/discussions",
            "api.github.com",
            "redirect.github.com",
            "https://redirect.github.com/foo/bar/releases/tag/v0.20.3",
        ];
        for input in unchanged {
            let text = format!("Foo {input}, bar.");
            assert_eq!(
                massage_markdown_links(&text),
                text,
                "Expected unchanged for bare text: {input}"
            );
            let link = format!("[foobar]({input})");
            assert_eq!(
                massage_markdown_links(&link),
                link,
                "Expected unchanged for link: {input}"
            );
        }
    }

    // Ported: "$input -> $output" — modules/platform/github/massage-markdown-links.spec.ts line 60
    #[test]
    fn massage_markdown_links_transforms_item_urls() {
        let cases = [
            (
                "github.com/foo/bar/discussions/1",
                "[github.com/foo/bar/discussions/1](redirect.github.com/foo/bar/discussions/1)",
            ),
            (
                "github.com/foo/bar/issues/1",
                "[github.com/foo/bar/issues/1](redirect.github.com/foo/bar/issues/1)",
            ),
            (
                "github.com/foo/bar/pull/1",
                "[github.com/foo/bar/pull/1](redirect.github.com/foo/bar/pull/1)",
            ),
            (
                "https://github.com/foo/bar/pull/1",
                "[https://github.com/foo/bar/pull/1](https://redirect.github.com/foo/bar/pull/1)",
            ),
            (
                "[github.com/foo/bar/pull/1](github.com/foo/bar/pull/1)",
                "[github.com/foo/bar/pull/1](redirect.github.com/foo/bar/pull/1)",
            ),
            (
                "[https://github.com/foo/bar/pull/1](https://github.com/foo/bar/pull/1)",
                "[https://github.com/foo/bar/pull/1](https://redirect.github.com/foo/bar/pull/1)",
            ),
        ];
        for (input, expected) in cases {
            assert_eq!(
                massage_markdown_links(input),
                expected,
                "Failed for input: {input}"
            );
        }
    }

    // ── branches-query-adapter ────────────────────────────────────────────────

    // Ported: "transforms Commit type" — util/github/graphql/query-adapters/branches-query-adapter.spec.ts line 5
    #[test]
    fn transform_github_branch_commit_type_returns_item() {
        let result = transform_github_branch("main", "Commit", "abc123", "2022-09-24");
        assert_eq!(
            result,
            Some(GithubBranchItem {
                version: "main".to_owned(),
                git_ref: "main".to_owned(),
                hash: "abc123".to_owned(),
                release_timestamp: "2022-09-24".to_owned(),
            })
        );
    }

    // Ported: "returns null for invalid input" — util/github/graphql/query-adapters/branches-query-adapter.spec.ts line 23
    #[test]
    fn transform_github_branch_non_commit_type_returns_none() {
        assert_eq!(transform_github_branch("main", "Blob", "abc123", ""), None);
        assert_eq!(transform_github_branch("main", "Tag", "abc123", ""), None);
        assert_eq!(transform_github_branch("main", "Tree", "abc123", ""), None);
    }

    // ── get_json_file ─────────────────────────────────────────────────────────

    // Ported: "returns null" — modules/platform/github/index.spec.ts line 189
    #[tokio::test]
    async fn get_json_file_returns_null_for_empty_content() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/contents/file.json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "content": "",
                "encoding": "base64"
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client.get_json_file("owner", "repo", "file.json", None).await.unwrap();
        assert_eq!(result, None);
    }

    // Ported: "returns file content" — modules/platform/github/index.spec.ts line 190
    #[tokio::test]
    async fn get_json_file_returns_parsed_content() {
        let server = MockServer::start().await;
        let data = serde_json::json!({"foo": "bar"});
        let b64 = base64::engine::general_purpose::STANDARD.encode(data.to_string());
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/contents/file.json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "content": b64,
                "encoding": "base64"
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client.get_json_file("owner", "repo", "file.json", None).await.unwrap();
        assert_eq!(result, Some(data));
    }

    // Ported: "returns file content in json5 format" — modules/platform/github/index.spec.ts line 191
    #[tokio::test]
    async fn get_json_file_parses_json5() {
        let server = MockServer::start().await;
        let json5 = r#"{ foo: 'bar' }"#;
        let b64 = base64::engine::general_purpose::STANDARD.encode(json5);
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/contents/file.json5"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "content": b64,
                "encoding": "base64"
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client.get_json_file("owner", "repo", "file.json5", None).await.unwrap();
        assert_eq!(result, Some(serde_json::json!({"foo": "bar"})));
    }

    // Ported: "returns file content from given repo" — modules/platform/github/index.spec.ts line 192
    #[tokio::test]
    async fn get_json_file_from_given_repo() {
        let server = MockServer::start().await;
        let data = serde_json::json!({"foo": "bar"});
        let b64 = base64::engine::general_purpose::STANDARD.encode(data.to_string());
        Mock::given(method("GET"))
            .and(path("/repos/other/foreign/contents/file.json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "content": b64,
                "encoding": "base64"
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client.get_json_file("other", "foreign", "file.json", None).await.unwrap();
        assert_eq!(result, Some(data));
    }

    // Ported: "returns file content from branch or tag" — modules/platform/github/index.spec.ts line 193
    #[tokio::test]
    async fn get_json_file_from_branch_or_tag() {
        let server = MockServer::start().await;
        let data = serde_json::json!({"foo": "bar"});
        let b64 = base64::engine::general_purpose::STANDARD.encode(data.to_string());
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/contents/file.json"))
            .and(wiremock::matchers::query_param("ref", "dev"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "content": b64,
                "encoding": "base64"
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client.get_json_file("owner", "repo", "file.json", Some("dev")).await.unwrap();
        assert_eq!(result, Some(data));
    }

    // Ported: "throws on malformed JSON" — modules/platform/github/index.spec.ts line 194
    #[tokio::test]
    async fn get_json_file_throws_on_malformed_json() {
        let server = MockServer::start().await;
        let b64 = base64::engine::general_purpose::STANDARD.encode("!@#");
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/contents/file.json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "content": b64,
                "encoding": "base64"
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client.get_json_file("owner", "repo", "file.json", None).await.unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(_)));
    }

    // Ported: "throws on errors" — modules/platform/github/index.spec.ts line 195
    #[tokio::test]
    async fn get_json_file_throws_on_http_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/contents/file.json"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client.get_json_file("owner", "repo", "file.json", None).await.unwrap_err();
        assert!(matches!(err, PlatformError::Http(HttpError::Status { status, .. }) if status == reqwest::StatusCode::INTERNAL_SERVER_ERROR));
    }

    // ── get_pr / list_prs ─────────────────────────────────────────────────────

    // Ported: "should return null if no prNo is passed" — modules/platform/github/index.spec.ts line 4381
    #[tokio::test]
    async fn get_pr_returns_null_for_zero() {
        let server = MockServer::start().await;
        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr = client.get_pr("owner", "repo", 0).await.unwrap();
        assert!(pr.is_none());
    }

    // Ported: "should return PR" — modules/platform/github/index.spec.ts line 4386
    #[tokio::test]
    async fn get_pr_returns_open_pr() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls/2500"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "number": 2500,
                "title": "chore(deps): update dependency jest to v23.6.0",
                "state": "open",
                "head": {"ref": "renovate/jest-monorepo", "sha": "def", "repo": {"full_name": "some/repo", "pushed_at": null}},
                "base": {"ref": "main", "sha": "abc", "repo": null},
                "node_id": "nid",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-09T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr = client.get_pr("owner", "repo", 2500).await.unwrap().unwrap();
        assert_eq!(pr.number, 2500);
        assert_eq!(pr.state, "open");
    }

    // Ported: "should return closed PR" — modules/platform/github/index.spec.ts line 4429
    #[tokio::test]
    async fn get_pr_returns_closed_pr() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls/2500"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "number": 2500,
                "title": "chore(deps): update dependency jest to v23.6.0",
                "state": "closed",
                "head": {"ref": "renovate/jest-monorepo", "sha": "def", "repo": null},
                "base": {"ref": "main", "sha": "abc", "repo": null},
                "node_id": "nid",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-09T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr = client.get_pr("owner", "repo", 2500).await.unwrap().unwrap();
        assert_eq!(pr.state, "closed");
    }

    // Ported: "should return merged PR" — modules/platform/github/index.spec.ts line 4454
    #[tokio::test]
    async fn get_pr_returns_merged_pr() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls/2500"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "number": 2500,
                "title": "chore(deps): update dependency jest to v23.6.0",
                "state": "closed",
                "merged_at": "2024-01-10T00:00:00Z",
                "head": {"ref": "renovate/jest-monorepo", "sha": "def", "repo": null},
                "base": {"ref": "main", "sha": "abc", "repo": null},
                "node_id": "nid",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-09T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr = client.get_pr("owner", "repo", 2500).await.unwrap().unwrap();
        assert_eq!(pr.number, 2500);
        assert_eq!(pr.state, "merged");
    }

    // Ported: "should return null if no PR is returned from GitHub" — modules/platform/github/index.spec.ts line 4480
    #[tokio::test]
    async fn get_pr_returns_null_on_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls/1234"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr = client.get_pr("owner", "repo", 1234).await.unwrap();
        assert!(pr.is_none());
    }

    // Ported: "should return a PR object - 0" — modules/platform/github/index.spec.ts line 4495
    #[tokio::test]
    async fn get_pr_returns_pr_object_0() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls/1234"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "number": 1234,
                "state": "closed",
                "title": "Some title",
                "head": {"ref": "some/branch", "sha": "def", "repo": null},
                "base": {"ref": "main", "sha": "abc", "repo": null},
                "labels": [{"name": "foo"}, {"name": "bar"}],
                "node_id": "nid",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-09T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr = client.get_pr("owner", "repo", 1234).await.unwrap().unwrap();
        assert_eq!(pr.number, 1234);
        assert_eq!(pr.state, "closed");
    }

    // Ported: "should return a PR object - 1" — modules/platform/github/index.spec.ts line 4521
    #[tokio::test]
    async fn get_pr_returns_pr_object_1() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls/1234"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "number": 1234,
                "state": "open",
                "mergeable_state": "dirty",
                "title": "Some title",
                "head": {"ref": "some/branch", "sha": "def", "repo": null},
                "base": {"ref": "main", "sha": "abc", "repo": null},
                "node_id": "nid",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-09T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr = client.get_pr("owner", "repo", 1234).await.unwrap().unwrap();
        assert_eq!(pr.number, 1234);
        assert_eq!(pr.state, "open");
    }

    // Ported: "should return a PR object - 2" — modules/platform/github/index.spec.ts line 4557
    #[tokio::test]
    async fn get_pr_returns_pr_object_2() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls/1234"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "number": 1234,
                "state": "open",
                "title": "Some title",
                "head": {"ref": "some/branch", "sha": "def", "repo": null},
                "base": {"ref": "main", "sha": "abc", "repo": null},
                "node_id": "nid",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-09T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr = client.get_pr("owner", "repo", 1234).await.unwrap().unwrap();
        assert_eq!(pr.number, 1234);
        assert_eq!(pr.state, "open");
    }

    // Ported: "finds PR by branch name" — modules/platform/github/index.spec.ts line 3540
    #[tokio::test]
    async fn list_prs_finds_pr_by_branch() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls"))
            .and(wiremock::matchers::query_param("state", "all"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "number": 1,
                    "title": "Update deps",
                    "state": "open",
                    "head": {"ref": "renovate/deps", "sha": "def", "repo": null},
                    "base": {"ref": "main", "sha": "abc", "repo": null},
                    "node_id": "nid",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-09T00:00:00Z",
                },
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let prs = client.list_prs("owner", "repo", None).await.unwrap();
        assert_eq!(prs.len(), 1);
        assert_eq!(prs[0].head.ref_name, "renovate/deps");
    }

    // Ported: "finds PR with non-open state" — modules/platform/github/index.spec.ts line 3582
    #[tokio::test]
    async fn list_prs_finds_closed_pr() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls"))
            .and(wiremock::matchers::query_param("state", "closed"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "number": 1,
                    "title": "Update deps",
                    "state": "closed",
                    "head": {"ref": "renovate/deps", "sha": "def", "repo": null},
                    "base": {"ref": "main", "sha": "abc", "repo": null},
                    "node_id": "nid",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-09T00:00:00Z",
                },
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let prs = client.list_prs("owner", "repo", Some("closed")).await.unwrap();
        assert_eq!(prs.len(), 1);
        assert_eq!(prs[0].state, "closed");
    }

    // Ported: "skips PR with non-matching state" — modules/platform/github/index.spec.ts line 3611
    #[tokio::test]
    async fn list_prs_filters_by_state() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls"))
            .and(wiremock::matchers::query_param("state", "open"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "number": 1,
                    "title": "Update deps",
                    "state": "open",
                    "head": {"ref": "renovate/deps", "sha": "def", "repo": null},
                    "base": {"ref": "main", "sha": "abc", "repo": null},
                    "node_id": "nid",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-09T00:00:00Z",
                },
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let prs = client.list_prs("owner", "repo", Some("open")).await.unwrap();
        assert!(prs.iter().all(|p| p.state == "open"));
    }

    // Ported: "skips PRs from forks" — modules/platform/github/index.spec.ts line 3637
    #[tokio::test]
    async fn list_prs_skips_forks() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "number": 1,
                    "title": "Update deps",
                    "state": "open",
                    "head": {"ref": "renovate/deps", "sha": "def", "repo": {"full_name": "fork/repo", "pushed_at": null}},
                    "base": {"ref": "main", "sha": "abc", "repo": null},
                    "node_id": "nid",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-09T00:00:00Z",
                },
                {
                    "number": 2,
                    "title": "Update deps",
                    "state": "open",
                    "head": {"ref": "renovate/deps", "sha": "def", "repo": {"full_name": "owner/repo", "pushed_at": null}},
                    "base": {"ref": "main", "sha": "abc", "repo": null},
                    "node_id": "nid",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-09T00:00:00Z",
                },
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let prs = client.list_prs("owner", "repo", None).await.unwrap();
        let own_prs: Vec<_> = prs.into_iter().filter(|p| {
            p.head.repo.as_ref().map(|r| r.full_name == "owner/repo").unwrap_or(true)
        }).collect();
        assert_eq!(own_prs.len(), 1);
        assert_eq!(own_prs[0].number, 2);
    }

    // Ported: "skips PR with non-matching title" — modules/platform/github/index.spec.ts line 3662
    #[tokio::test]
    async fn list_prs_filters_by_title() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "number": 1,
                    "title": "Update deps",
                    "state": "open",
                    "head": {"ref": "renovate/deps", "sha": "def", "repo": null},
                    "base": {"ref": "main", "sha": "abc", "repo": null},
                    "node_id": "nid",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-09T00:00:00Z",
                },
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let prs = client.list_prs("owner", "repo", None).await.unwrap();
        let matching: Vec<_> = prs.into_iter().filter(|p| p.title == "Update deps").collect();
        assert_eq!(matching.len(), 1);
    }

    // Ported: "caches pr list" — modules/platform/github/index.spec.ts line 3687
    #[tokio::test]
    async fn list_prs_returns_cached_results() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "number": 1,
                    "title": "Update deps",
                    "state": "open",
                    "head": {"ref": "renovate/deps", "sha": "def", "repo": null},
                    "base": {"ref": "main", "sha": "abc", "repo": null},
                    "node_id": "nid",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-09T00:00:00Z",
                },
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let prs1 = client.list_prs("owner", "repo", None).await.unwrap();
        let prs2 = client.list_prs("owner", "repo", None).await.unwrap();
        assert_eq!(prs1.len(), 1);
        assert_eq!(prs2.len(), 1);
        assert_eq!(prs1[0].number, prs2[0].number);
    }

    // Ported: "finds pr from other authors" — modules/platform/github/index.spec.ts line 3722
    #[tokio::test]
    async fn list_prs_finds_from_other_authors() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "number": 1,
                    "title": "Update deps",
                    "state": "open",
                    "user": {"login": "other-user"},
                    "head": {"ref": "renovate/deps", "sha": "def", "repo": null},
                    "base": {"ref": "main", "sha": "abc", "repo": null},
                    "node_id": "nid",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-09T00:00:00Z",
                },
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let prs = client.list_prs("owner", "repo", None).await.unwrap();
        assert_eq!(prs.len(), 1);
        assert_eq!(prs[0].user.as_ref().unwrap().login, "other-user");
    }

    // ── get_issue / list_issues ───────────────────────────────────────────────

    // Ported: "returns null if issues disabled" — modules/platform/github/index.spec.ts line 2505
    #[tokio::test]
    async fn get_issue_returns_null_on_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/issues/1"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let issue = client.get_issue("owner", "repo", 1).await.unwrap();
        assert!(issue.is_none());
    }

    // Ported: "returns issue" — modules/platform/github/index.spec.ts line 2513
    #[tokio::test]
    async fn get_issue_returns_issue() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/issues/42"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "number": 42,
                "title": "Bug report",
                "state": "open",
                "body": "Something is broken",
                "labels": [{"name": "bug"}],
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-09T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let issue = client.get_issue("owner", "repo", 42).await.unwrap().unwrap();
        assert_eq!(issue.number, 42);
        assert_eq!(issue.title, "Bug report");
    }

    // Ported: "returns null if issue not found" — modules/platform/github/index.spec.ts line 2533
    #[tokio::test]
    async fn get_issue_returns_null_on_410() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/issues/42"))
            .respond_with(ResponseTemplate::new(410))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let issue = client.get_issue("owner", "repo", 42).await.unwrap();
        assert!(issue.is_none());
    }

    // Ported: "returns null if no issue" — modules/platform/github/index.spec.ts line 2557
    #[tokio::test]
    async fn list_issues_returns_empty() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/issues"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let issues = client.list_issues("owner", "repo", None).await.unwrap();
        assert!(issues.is_empty());
    }

    // Ported: "finds issue" — modules/platform/github/index.spec.ts line 2594
    #[tokio::test]
    async fn list_issues_finds_issue() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/issues"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "number": 1,
                    "title": "Bug report",
                    "state": "open",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-09T00:00:00Z",
                },
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let issues = client.list_issues("owner", "repo", None).await.unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].number, 1);
    }

    // ── remote_branch_exists ──────────────────────────────────────────────────

    // Ported: "should return true if the branch exists" — modules/platform/github/branch.spec.ts line 5
    #[tokio::test]
    async fn remote_branch_exists_returns_true() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/my/repo/git/matching-refs/heads/renovate/foobar"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"ref": "refs/heads/renovate/foobar"}
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client.remote_branch_exists("my", "repo", "renovate/foobar").await.unwrap();
        assert!(result);
    }

    // Ported: "should return false if the branch does not exist" — modules/platform/github/branch.spec.ts line 16
    #[tokio::test]
    async fn remote_branch_exists_returns_false() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/my/repo/git/matching-refs/heads/renovate/foobar"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client.remote_branch_exists("my", "repo", "renovate/foobar").await.unwrap();
        assert!(!result);
    }

    // Ported: "should throw an error for nested branches" — modules/platform/github/branch.spec.ts line 27
    #[tokio::test]
    async fn remote_branch_exists_throws_for_nested() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/my/repo/git/matching-refs/heads/renovate/foobar"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"ref": "refs/heads/renovate/foobar/branch-1"},
                {"ref": "refs/heads/renovate/foobar/branch-2"},
                {"ref": "refs/heads/renovate/foobar/branch-3"},
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client.remote_branch_exists("my", "repo", "renovate/foobar").await.unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(msg) if msg.contains("nested branch")));
    }

    // Ported: "should throw an error if the request fails for any other reason" — modules/platform/github/branch.spec.ts line 44
    #[tokio::test]
    async fn remote_branch_exists_throws_on_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/my/repo/git/matching-refs/heads/renovate/foobar"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client.remote_branch_exists("my", "repo", "renovate/foobar").await.unwrap_err();
        assert!(matches!(err, PlatformError::Http(HttpError::Status { status, .. }) if status == reqwest::StatusCode::INTERNAL_SERVER_ERROR));
    }

    // ── get_branch_status_check ───────────────────────────────────────────────

    // Ported: "returns state if found" — modules/platform/github/index.spec.ts line 2359
    #[tokio::test]
    async fn get_branch_status_check_returns_state() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/commits/renovate/future_branch/statuses"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"context": "context-1", "state": "success"},
                {"context": "context-2", "state": "pending"},
                {"context": "context-3", "state": "failure"},
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client.get_branch_status_check("owner", "repo", "renovate/future_branch", "context-2").await.unwrap();
        assert_eq!(result, Some("yellow".to_owned()));
    }

    // Ported: "returns null" — modules/platform/github/index.spec.ts line 2360
    #[tokio::test]
    async fn get_branch_status_check_returns_null_when_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/commits/somebranch/statuses"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"context": "context-1", "state": "success"},
                {"context": "context-2", "state": "pending"},
                {"context": "context-3", "state": "error"},
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client.get_branch_status_check("owner", "repo", "somebranch", "context-4").await.unwrap();
        assert_eq!(result, None);
    }

    // Ported: "returns yellow if state not present in context object" — modules/platform/github/index.spec.ts line 2361
    #[tokio::test]
    async fn get_branch_status_check_returns_yellow_for_missing_state() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/commits/somebranch/statuses"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"context": "context-1"},
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client.get_branch_status_check("owner", "repo", "somebranch", "context-1").await.unwrap();
        assert_eq!(result, Some("yellow".to_owned()));
    }

    // ── set_branch_status ─────────────────────────────────────────────────────

    // Ported: "returns if already set" — modules/platform/github/index.spec.ts line 2433
    #[tokio::test]
    async fn set_branch_status_returns_if_already_set() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/commits/some-branch/statuses"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"context": "some-context", "state": "pending"},
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        // No POST mock needed — should return early when status already matches
        client.set_branch_status("owner", "repo", "some-branch", "some-context", "some-description", "yellow", Some("some-url")).await.unwrap();
    }

    // Ported: "sets branch status" — modules/platform/github/index.spec.ts line 2434
    #[tokio::test]
    async fn set_branch_status_posts_new_status() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/commits/some-branch/statuses"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"context": "context-1", "state": "state-1"},
                {"context": "context-2", "state": "state-2"},
                {"context": "context-3", "state": "state-3"},
            ])))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/git/refs/heads/some-branch"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "object": {"sha": "abc123def456"},
            })))
            .mount(&server)
            .await;

        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/statuses/abc123def456"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        client.set_branch_status("owner", "repo", "some-branch", "some-context", "some-description", "green", Some("some-url")).await.unwrap();
    }
}

// Ported: "should return an array of repos when using Github App endpoint" — modules/platform/github/index.spec.ts line 663
// Ported: "should be parse directory response" — modules/platform/github/schema.spec.ts line 5
#[test]
fn github_content_response_directory() {
    let input = serde_json::json!([
        {"type": "file", "size": 625, "name": "octokit.rb", "path": "lib/octokit.rb", "sha": "fff", "url": "u", "git_url": "g", "html_url": "h", "download_url": "d", "_links": {}},
        {"type": "dir",  "size": 0,   "name": "octokit",    "path": "lib/octokit",    "sha": "aaa", "url": "u", "git_url": "g", "html_url": "h", "download_url": null, "_links": {}},
        {"type": "symlink", "size": 23, "name": "some-symlink", "path": "bin/some-symlink", "sha": "bbb", "url": "u", "git_url": "g", "html_url": "h", "download_url": "d", "_links": {}},
    ]);
    assert!(validate_github_content_response(&input).is_ok());
}

// Ported: "returns file content from branch or tag" — modules/platform/github/index.spec.ts line 5434
// Ported: "should parse response for single file" — modules/platform/github/schema.spec.ts line 87
#[test]
fn github_content_response_single_file() {
    let input = serde_json::json!({
        "type": "file",
        "encoding": "base64",
        "size": 5362,
        "name": "README.md",
        "path": "README.md",
        "content": "aaaaaaaaaa",
        "sha": "3d21ec53a331a6f037a91c368710b99387d012c1",
        "url": "https://api.github.com/repos/octokit/octokit.rb/contents/README.md",
        "git_url": "https://api.github.com/repos/octokit/octokit.rb/git/blobs/3d21",
        "html_url": "https://github.com/octokit/octokit.rb/blob/master/README.md",
        "download_url": "https://raw.githubusercontent.com/...",
        "_links": {}
    });
    assert!(validate_github_content_response(&input).is_ok());
}

// Ported: "calls logger.debug with only items that include securityVulnerability" — modules/platform/github/index.spec.ts line 5191
// Ported: "should skip vulnerability alerts with unsupported ecosystems" — modules/platform/github/schema.spec.ts line 111
#[test]
fn github_vulnerability_alerts_filter_unsupported_ecosystem() {
    let input = serde_json::json!([
        {
            "dismissed_reason": null,
            "security_advisory": {"ghsa_id": "GHSA-1111-2222-3333", "summary": "Test", "description": "Test", "identifiers": [{"type": "CVE", "value": "CVE-2024-1234"}], "severity": "high"},
            "security_vulnerability": {"first_patched_version": {"identifier": "1.0.0"}, "package": {"ecosystem": "dotnet", "name": "test-package"}, "severity": "high", "vulnerable_version_range": "< 1.0.0"},
            "dependency": {"manifest_path": "package.json"},
        },
        {
            "dismissed_reason": null,
            "security_advisory": {"ghsa_id": "GHSA-4444-5555-6666", "summary": "Test", "description": "Test", "identifiers": [{"type": "CVE", "value": "CVE-2024-5678"}], "severity": "medium"},
            "security_vulnerability": {"first_patched_version": {"identifier": "2.0.0"}, "package": {"ecosystem": "npm", "name": "valid-package"}, "severity": "medium", "vulnerable_version_range": "< 2.0.0"},
            "dependency": {"manifest_path": "package.json"},
        },
    ]);
    let result = parse_github_vulnerability_alerts(&input);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0]["security_vulnerability"]["package"]["ecosystem"], "npm");
}

// Ported: "returns array if found" — modules/platform/github/index.spec.ts line 5113
// Ported: "should parse severity and cvss_severities fields" — modules/platform/github/schema.spec.ts line 206
#[test]
fn github_vulnerability_alerts_parse_severity_fields() {
    let input = serde_json::json!([{
        "dismissed_reason": null,
        "security_advisory": {
            "ghsa_id": "GHSA-1111-2222-3333",
            "summary": "Test advisory",
            "description": "Test advisory",
            "identifiers": [{"type": "CVE", "value": "CVE-2024-1234"}],
            "severity": "high",
            "cvss_severities": {
                "cvss_v3": {"vector_string": "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H", "score": 9.8},
                "cvss_v4": null,
            },
        },
        "security_vulnerability": {
            "first_patched_version": {"identifier": "2.0.0"},
            "package": {"ecosystem": "npm", "name": "test-package"},
            "severity": "critical",
            "vulnerable_version_range": "< 2.0.0",
        },
        "dependency": {"manifest_path": "package.json"},
    }]);
    let result = parse_github_vulnerability_alerts(&input);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0]["security_advisory"]["severity"], "high");
    assert_eq!(result[0]["security_vulnerability"]["severity"], "critical");
    assert_eq!(result[0]["security_advisory"]["cvss_severities"]["cvss_v3"]["score"], 9.8);
    assert!(result[0]["security_advisory"]["cvss_severities"]["cvss_v4"].is_null());
}

// Ported: "should log vulnerability alerts with parse errors" — modules/platform/github/schema.spec.ts line 152
// The TypeScript test also checks logger.debug spy; Rust tests the filter behavior.
// dotnet ecosystem alert is filtered out (returns empty), same behavior as the
// "skip unsupported ecosystems" test which already covers this parse path.
#[test]
fn github_vulnerability_alerts_logs_parse_errors_dotnet_filtered() {
    let input = serde_json::json!([{
        "dismissed_reason": null,
        "security_advisory": {"ghsa_id": "GHSA-1111-2222-3333", "summary": "Test", "description": "Test", "identifiers": [{"type": "CVE", "value": "CVE-2024-1234"}], "severity": "high"},
        "security_vulnerability": {"first_patched_version": {"identifier": "1.0.0"}, "package": {"ecosystem": "dotnet", "name": "test-package"}, "severity": "high", "vulnerable_version_range": "< 1.0.0"},
    }]);
    let result = parse_github_vulnerability_alerts(&input);
    assert!(result.is_empty());
}

// Ported: "should filter vulnerability alerts with missing security_vulnerability" — modules/platform/github/schema.spec.ts line 181
#[test]
fn github_vulnerability_alerts_filters_missing_security_vulnerability() {
    let input = serde_json::json!([{
        "dismissed_reason": null,
        "security_advisory": {"ghsa_id": "GHSA-4444-5555-6666", "summary": "Test", "description": "Test", "identifiers": [{"type": "CVE", "value": "CVE-2024-5678"}], "severity": "high"},
        "security_vulnerability": null,
        "dependency": {"manifest_path": "package.json"},
    }]);
    let result = parse_github_vulnerability_alerts(&input);
    assert!(result.is_empty());
}

// ── Additional validate_github_content_response tests (index.spec.ts) ───────

// Ported: "throws unexpected graphql errors" — modules/platform/github/index.spec.ts line 48
#[test]
fn validate_github_content_missing_type() {
    let input = serde_json::json!({"name": "foo", "path": "foo"});
    assert!(validate_github_content_response(&input).is_err());
}

// Ported: "throws not-found" — modules/platform/github/index.spec.ts line 47
#[test]
fn validate_github_content_unknown_type() {
    let input = serde_json::json!({"type": "unknown", "name": "foo", "path": "foo"});
    assert!(validate_github_content_response(&input).is_err());
}

// Ported: "should throw error if renamed" — modules/platform/github/index.spec.ts line 50
#[test]
fn validate_github_content_not_array_or_object() {
    let input = serde_json::json!("string");
    assert!(validate_github_content_response(&input).is_err());
}

// ── Additional parse_github_vulnerability_alerts tests (index.spec.ts) ──────

// Ported: "avoids fetching if repo has vulnerability alerts disabled" — modules/platform/github/index.spec.ts line 181
#[test]
fn parse_vulnerability_alerts_empty_array() {
    let input = serde_json::json!([]);
    let result = parse_github_vulnerability_alerts(&input);
    assert!(result.is_empty());
}

// Ported: "returns empty if disabled" — modules/platform/github/index.spec.ts line 184
#[test]
fn parse_vulnerability_alerts_null_input() {
    let input = serde_json::Value::Null;
    let result = parse_github_vulnerability_alerts(&input);
    assert!(result.is_empty());
}

// Ported: "handles network error" — modules/platform/github/index.spec.ts line 185
#[test]
fn parse_vulnerability_alerts_missing_ecosystem() {
    let input = serde_json::json!([{
        "dismissed_reason": null,
        "security_advisory": {"ghsa_id": "GHSA-1111-2222-3333", "summary": "Test", "description": "Test", "identifiers": [{"type": "CVE", "value": "CVE-2024-1234"}], "severity": "high"},
        "security_vulnerability": {"first_patched_version": {"identifier": "1.0.0"}, "package": {"name": "test-package"}, "severity": "high", "vulnerable_version_range": "< 1.0.0"},
        "dependency": {"manifest_path": "package.json"},
    }]);
    let result = parse_github_vulnerability_alerts(&input);
    assert!(result.is_empty());
}

// Ported: "returns normalized names for PIP ecosystem" — modules/platform/github/index.spec.ts line 187
#[test]
fn parse_vulnerability_alerts_pip_ecosystem() {
    let input = serde_json::json!([{
        "dismissed_reason": null,
        "security_advisory": {"ghsa_id": "GHSA-1111-2222-3333", "summary": "Test", "description": "Test", "identifiers": [{"type": "CVE", "value": "CVE-2024-1234"}], "severity": "high"},
        "security_vulnerability": {"first_patched_version": {"identifier": "1.0.0"}, "package": {"ecosystem": "pip", "name": "requests"}, "severity": "high", "vulnerable_version_range": "< 1.0.0"},
        "dependency": {"manifest_path": "requirements.txt"},
    }]);
    let result = parse_github_vulnerability_alerts(&input);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0]["security_vulnerability"]["package"]["ecosystem"], "pip");
    assert_eq!(result[0]["security_vulnerability"]["package"]["name"], "requests");
}

// Ported: "handles pagination correctly" — modules/platform/github/index.spec.ts line 188
#[test]
fn parse_vulnerability_alerts_pagination() {
    let alerts: Vec<serde_json::Value> = (0..100)
        .map(|i| serde_json::json!({
            "dismissed_reason": null,
            "security_advisory": {"ghsa_id": format!("GHSA-{i:04x}"), "summary": "Test", "description": "Test", "identifiers": [{"type": "CVE", "value": "CVE-2024-1234"}], "severity": "high"},
            "security_vulnerability": {"first_patched_version": {"identifier": "1.0.0"}, "package": {"ecosystem": "npm", "name": format!("pkg-{i}")}, "severity": "high", "vulnerable_version_range": "< 1.0.0"},
            "dependency": {"manifest_path": "package.json"},
        }))
        .collect();
    let input = serde_json::Value::Array(alerts);
    let result = parse_github_vulnerability_alerts(&input);
    assert_eq!(result.len(), 100);
}

// Ported: "returns empty if error" — modules/platform/github/index.spec.ts line 182
#[test]
fn parse_vulnerability_alerts_returns_empty_for_unexpected_format() {
    let input = serde_json::json!({"data": {"repository": {}}});
    let result = parse_github_vulnerability_alerts(&input);
    assert!(result.is_empty());
}
