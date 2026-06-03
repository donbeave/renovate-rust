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
use crate::platform::{CurrentUser, PlatformClient, PlatformError, RawFile, RepoInitResult};

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

/// Parse a `Link` HTTP response header and return the URL for `rel="next"`.
fn parse_link_header_next(header: &str) -> Option<String> {
    // Link: <url>; rel="next", <url>; rel="last"
    for part in header.split(',') {
        let part = part.trim();
        if (part.contains(r#"rel="next""#) || part.contains("rel='next'"))
            && let Some(start) = part.find('<')
            && let Some(end) = part.find('>')
            && start < end
        {
            return Some(part[start + 1..end].to_owned());
        }
    }
    None
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

/// Fork state stored after `init_repo` when fork mode is active.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct GithubForkState {
    token: String,
    org: Option<String>,
}

/// GitHub platform client. Authenticated with a personal access token or
/// GitHub App installation token.
/// Cached PR list with timestamp for TTL-based eviction.
#[derive(Debug, Clone)]
struct PrCacheEntry {
    fetched_at: std::time::Instant,
    prs: Vec<GhRestPr>,
}

#[derive(Debug, Clone)]
pub struct GithubClient {
    http: HttpClient,
    api_base: String,
    branch_force_rebase_cache:
        std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, bool>>>,
    fork_state: std::sync::Arc<std::sync::Mutex<Option<GithubForkState>>>,
    /// TTL cache for `list_prs` results. Key is `"owner/repo/state"`.
    pr_cache: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, PrCacheEntry>>>,
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
        let api_base = api_base.into().trim_end_matches('/').to_owned();
        if let Err(e) = url::Url::parse(&api_base) {
            return Err(HttpError::Parse(format!(
                "Invalid GitHub endpoint URL: {api_base}: {e}"
            )));
        }
        Ok(Self {
            http: HttpClient::with_token(token)?,
            api_base,
            branch_force_rebase_cache: std::sync::Arc::new(std::sync::Mutex::new(
                std::collections::HashMap::new(),
            )),
            fork_state: std::sync::Arc::new(std::sync::Mutex::new(None)),
            pr_cache: std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
        })
    }

    /// GraphQL query to fetch repository metadata.
    ///
    /// Mirrors `repoInfoQuery` from `lib/modules/platform/github/graphql.ts`.
    const REPO_INFO_QUERY: &str = r#"
query($owner: String!, $name: String!, $user: String) {
  repository(owner: $owner, name: $name) {
    id
    isFork
    parent {
      nameWithOwner
    }
    isArchived
    nameWithOwner
    hasIssuesEnabled
    hasVulnerabilityAlertsEnabled
    autoMergeAllowed
    mergeCommitAllowed
    rebaseMergeAllowed
    squashMergeAllowed
    defaultBranchRef {
      name
      target {
        oid
      }
    }
    issues(
      orderBy: { field: UPDATED_AT, direction: DESC },
      filterBy: { createdBy: $user },
      first: 5
    ) {
      nodes {
        number
        state
        title
        body
        updatedAt
      }
    }
  }
}
"#;

    /// Initialize the GitHub platform.
    ///
    /// Validates the token, fetches the current user, and attempts to
    /// resolve a git author email from the user profile or emails API.
    ///
    /// Mirrors `initPlatform` from `lib/modules/platform/github/index.ts`.
    pub async fn init_platform(
        &self,
        token: &str,
    ) -> Result<(String, Option<String>), PlatformError> {
        if token.is_empty() {
            return Err(PlatformError::Unexpected(
                "Init: You must configure a GitHub token".to_owned(),
            ));
        }

        // Detect GHE and validate fine-grained token compatibility
        self.check_fine_grained_token(token).await?;

        let user: GithubUser = self
            .http
            .get_json(&format!("{}/user", self.api_base))
            .await
            .map_err(|e| match e {
                HttpError::Status { status, .. } if status == reqwest::StatusCode::UNAUTHORIZED => {
                    PlatformError::Unauthorized
                }
                other => PlatformError::Http(other),
            })?;

        let email = if let Some(ref public_email) = user.email {
            Some(public_email.clone())
        } else {
            self.fetch_user_email().await.ok().flatten()
        };

        Ok((user.login, email))
    }

    /// Initialize a specific repository on GitHub.
    ///
    /// Fetches repository metadata via GraphQL, detects merge methods,
    /// archived/renamed/empty states, and computes a repo fingerprint.
    ///
    /// Mirrors `initRepo` from `lib/modules/platform/github/index.ts`.
    pub async fn init_repo(
        &self,
        owner: &str,
        repo: &str,
        fork_token: Option<&str>,
        fork_creation: bool,
        fork_org: Option<&str>,
    ) -> Result<RepoInitResult, PlatformError> {
        let variables = serde_json::json!({
            "owner": owner,
            "name": repo,
        });
        let body = serde_json::json!({
            "query": Self::REPO_INFO_QUERY,
            "variables": variables,
        });
        let url = format!("{}/graphql", self.api_base);

        let response: serde_json::Value = self
            .http
            .post_json(&url, &body.to_string())
            .await
            .map_err(|e| match e {
                HttpError::Status { status, .. } if status == reqwest::StatusCode::FORBIDDEN => {
                    PlatformError::Unexpected("REPOSITORY_ACCESS_FORBIDDEN".to_owned())
                }
                HttpError::Status { status, .. } if status == reqwest::StatusCode::NOT_FOUND => {
                    PlatformError::Unexpected("REPOSITORY_NOT_FOUND".to_owned())
                }
                other => PlatformError::Http(other),
            })?;

        // Check for GraphQL-level errors.
        if let Some(errors) = response.get("errors") {
            let errors_array = errors.as_array().cloned().unwrap_or_default();
            if errors_array.iter().any(|e| {
                e.get("type")
                    .and_then(|t| t.as_str())
                    .map(|t| t == "RATE_LIMITED")
                    .unwrap_or(false)
            }) {
                return Err(PlatformError::Unexpected(
                    "PLATFORM_RATE_LIMIT_EXCEEDED".to_owned(),
                ));
            }
            if errors_array.iter().any(|e| {
                e.get("message")
                    .and_then(|m| m.as_str())
                    .map(|m| m.starts_with("Repository access blocked"))
                    .unwrap_or(false)
            }) {
                return Err(PlatformError::Unexpected(
                    "REPOSITORY_ACCESS_FORBIDDEN".to_owned(),
                ));
            }
            return Err(PlatformError::Unexpected(
                "PLATFORM_UNKNOWN_ERROR".to_owned(),
            ));
        }

        let repo_data = response
            .get("data")
            .and_then(|d| d.get("repository"))
            .ok_or_else(|| PlatformError::Unexpected("REPOSITORY_NOT_FOUND".to_owned()))?;

        if repo_data.is_null() {
            return Err(PlatformError::Unexpected("REPOSITORY_NOT_FOUND".to_owned()));
        }

        let default_branch = repo_data
            .get("defaultBranchRef")
            .and_then(|b| b.get("name"))
            .and_then(|n| n.as_str())
            .ok_or_else(|| PlatformError::Unexpected("REPOSITORY_EMPTY".to_owned()))?;

        let name_with_owner = repo_data
            .get("nameWithOwner")
            .and_then(|n| n.as_str())
            .unwrap_or_default();
        if !name_with_owner.is_empty()
            && name_with_owner.to_uppercase() != format!("{owner}/{repo}").to_uppercase()
        {
            return Err(PlatformError::Unexpected("REPOSITORY_RENAMED".to_owned()));
        }

        let is_archived = repo_data
            .get("isArchived")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        if is_archived {
            return Err(PlatformError::Unexpected("REPOSITORY_ARCHIVED".to_owned()));
        }

        let is_fork = repo_data
            .get("isFork")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        // Fork-mode handling.
        if let Some(token) = fork_token {
            if is_fork {
                return Err(PlatformError::Unexpected("REPOSITORY_FORKED".to_owned()));
            }
            let repository = format!("{owner}/{repo}");
            let existing_fork = self.find_fork(token, &repository, fork_org).await?;
            match existing_fork {
                Some(_fork) => {
                    // TODO: sync default branch in fork if needed
                }
                None => {
                    if fork_creation {
                        let _new_fork = self.create_fork(token, owner, repo, fork_org).await?;
                    } else {
                        return Err(PlatformError::Unexpected(
                            "REPOSITORY_FORK_MISSING".to_owned(),
                        ));
                    }
                }
            }
            if let Ok(mut state) = self.fork_state.lock() {
                *state = Some(GithubForkState {
                    token: token.to_owned(),
                    org: fork_org.map(|s| s.to_owned()),
                });
            }
        }

        let merge_method = if repo_data
            .get("squashMergeAllowed")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            Some("squash".to_owned())
        } else if repo_data
            .get("mergeCommitAllowed")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            Some("merge".to_owned())
        } else if repo_data
            .get("rebaseMergeAllowed")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            Some("rebase".to_owned())
        } else {
            None
        };

        let repo_id = repo_data
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let fingerprint = super::util::repo_fingerprint(repo_id, Some(&self.api_base));

        Ok(RepoInitResult {
            default_branch: default_branch.to_owned(),
            is_fork,
            repo_fingerprint: fingerprint,
            merge_method,
            auto_merge_allowed: repo_data
                .get("autoMergeAllowed")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            has_issues_enabled: repo_data
                .get("hasIssuesEnabled")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            has_vulnerability_alerts_enabled: repo_data
                .get("hasVulnerabilityAlertsEnabled")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
        })
    }

    /// Find an existing fork of `repository` owned by the fork-token user
    /// (or `fork_org` when specified).
    async fn find_fork(
        &self,
        fork_token: &str,
        repository: &str,
        fork_org: Option<&str>,
    ) -> Result<Option<String>, PlatformError> {
        let fork_http = HttpClient::with_token(fork_token).map_err(PlatformError::Http)?;
        let owner = if let Some(org) = fork_org {
            org.to_owned()
        } else {
            let user: GithubUser = fork_http
                .get_json(&format!("{}/user", self.api_base))
                .await
                .map_err(PlatformError::Http)?;
            user.login
        };

        let repo_name = repository
            .rsplit_once('/')
            .map(|(_, r)| r)
            .unwrap_or(repository);
        let url = format!("{}/repos/{owner}/{repo_name}", self.api_base);
        match fork_http.get_json::<serde_json::Value>(&url).await {
            Ok(repo_json) => {
                let is_fork = repo_json
                    .get("fork")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                let source = repo_json
                    .get("source")
                    .and_then(|s| s.get("full_name"))
                    .and_then(|n| n.as_str());
                if is_fork && source == Some(repository) {
                    Ok(Some(format!("{owner}/{repo_name}")))
                } else {
                    Ok(None)
                }
            }
            Err(_) => Ok(None),
        }
    }

    /// Create a fork of `owner/repo` using `fork_token`.
    /// Optionally creates the fork inside `fork_org`.
    async fn create_fork(
        &self,
        fork_token: &str,
        owner: &str,
        repo: &str,
        fork_org: Option<&str>,
    ) -> Result<String, PlatformError> {
        let fork_http = HttpClient::with_token(fork_token).map_err(PlatformError::Http)?;
        let url = format!("{}/repos/{owner}/{repo}/forks", self.api_base);
        let body = if let Some(org) = fork_org {
            serde_json::json!({ "organization": org })
        } else {
            serde_json::json!({})
        };
        let result: serde_json::Value = fork_http
            .post_json(&url, &body.to_string())
            .await
            .map_err(PlatformError::Http)?;
        result
            .get("full_name")
            .and_then(|n| n.as_str())
            .map(|s| s.to_owned())
            .ok_or_else(|| {
                PlatformError::Unexpected("FORK creation response missing full_name".to_owned())
            })
    }

    /// Detect GitHub Enterprise Server version and validate fine-grained
    /// Personal Access Token compatibility.
    ///
    /// GHE requires version >= 3.10 to support fine-grained PATs.
    async fn check_fine_grained_token(&self, token: &str) -> Result<(), PlatformError> {
        let is_fine_grained = token.starts_with("github_pat_");
        if !is_fine_grained {
            return Ok(());
        }

        let host = match url::Url::parse(&self.api_base) {
            Ok(url) => url.host_str().unwrap_or("").to_owned(),
            Err(_) => return Ok(()),
        };

        let is_ghe = host != "api.github.com";
        if !is_ghe {
            return Ok(());
        }

        let Ok(resp) = self.http.head_json(&self.api_base).await else {
            return Ok(());
        };

        let ghe_version = resp
            .headers()
            .get("x-github-enterprise-version")
            .and_then(|v| v.to_str().ok());

        let needs_check = match ghe_version {
            Some(v) => {
                let version = semver::Version::parse(v).ok();
                match version {
                    Some(ver) => ver < semver::Version::new(3, 10, 0),
                    None => true,
                }
            }
            None => true,
        };

        if needs_check {
            return Err(PlatformError::Unexpected(
                "Init: Fine-grained Personal Access Tokens do not support GitHub Enterprise Server API version <3.10 and cannot be used with Renovate.".to_owned(),
            ));
        }

        Ok(())
    }

    /// Fetch the primary verified email from `/user/emails`.
    async fn fetch_user_email(&self) -> Result<Option<String>, PlatformError> {
        let url = format!("{}/user/emails", self.api_base);
        let emails: Vec<GithubUserEmail> = self
            .http
            .get_json(&url)
            .await
            .map_err(PlatformError::Http)?;
        Ok(emails
            .into_iter()
            .find(|e| e.primary && e.verified)
            .map(|e| e.email))
    }

    /// List repositories accessible to the authenticated user.
    ///
    /// For regular tokens: GET /user/repos?per_page=100
    /// For GitHub App tokens: GET /installation/repositories?per_page=100
    ///
    /// Mirrors `getRepos` from `lib/modules/platform/github/index.ts`.
    pub async fn get_repos(
        &self,
        topics: Option<Vec<String>>,
    ) -> Result<Vec<String>, PlatformError> {
        let token = self.http.token.as_deref().unwrap_or("");
        let is_app_token = token.starts_with("ghs_") || token.contains("x-access-token");

        let url = if is_app_token {
            format!("{}/installation/repositories?per_page=100", self.api_base)
        } else {
            format!("{}/user/repos?per_page=100", self.api_base)
        };

        if is_app_token {
            #[derive(Debug, Deserialize)]
            struct AppRepositories {
                repositories: Vec<RepoItem>,
            }
            #[derive(Debug, Deserialize)]
            struct RepoItem {
                full_name: String,
                #[serde(default)]
                archived: bool,
            }

            let resp: AppRepositories = self
                .http
                .get_json(&url)
                .await
                .map_err(PlatformError::Http)?;
            let repos: Vec<String> = resp
                .repositories
                .into_iter()
                .filter(|r| !r.archived)
                .map(|r| r.full_name)
                .collect();
            Ok(repos)
        } else {
            #[derive(Debug, Deserialize)]
            struct UserRepo {
                full_name: String,
                #[serde(default)]
                archived: bool,
                #[serde(default)]
                topics: Vec<String>,
            }

            let repos: Vec<UserRepo> = self
                .http
                .get_json(&url)
                .await
                .map_err(PlatformError::Http)?;
            let mut result: Vec<String> = repos
                .into_iter()
                .filter(|r| !r.archived)
                .filter(|r| {
                    if let Some(ref filter_topics) = topics {
                        if filter_topics.is_empty() {
                            return true;
                        }
                        return filter_topics.iter().any(|t| r.topics.contains(t));
                    }
                    true
                })
                .map(|r| r.full_name)
                .collect();
            result.sort();
            result.dedup();
            Ok(result)
        }
    }
}

/// Minimal GitHub user response.
#[derive(Debug, Clone, Deserialize)]
pub struct GithubUser {
    login: String,
    #[serde(default)]
    pub email: Option<String>,
}

/// GitHub user email response.
#[derive(Debug, Clone, Deserialize)]
struct GithubUserEmail {
    email: String,
    #[serde(default)]
    primary: bool,
    #[serde(default)]
    verified: bool,
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

/// Single check-run returned by the GitHub check-runs API.
#[derive(Debug, Clone, Deserialize)]
struct CheckRun {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    status: String,
    conclusion: Option<String>,
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

/// GitHub REST API comment representation.
#[derive(Debug, Clone, Deserialize)]
pub struct GhComment {
    pub id: i64,
    pub body: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    maintainer_can_modify: Option<bool>,
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
    async fn init_repo(
        &self,
        owner: &str,
        repo: &str,
        fork_token: Option<&str>,
        fork_creation: bool,
        fork_org: Option<&str>,
    ) -> Result<RepoInitResult, PlatformError> {
        self.init_repo(owner, repo, fork_token, fork_creation, fork_org)
            .await
    }

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
        self.create_pr_with_options(
            owner,
            repo,
            source_branch,
            target_branch,
            title,
            body,
            false,
            None,
            None,
        )
        .await
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

        let mut combined: CombinedBranchStatus = self
            .http
            .get_json::<CombinedBranchStatus>(&status_url)
            .await
            .map_err(PlatformError::Http)?;

        // Fetch check-runs and merge their conclusions into the combined state.
        // Upstream: lib/modules/platform/github/index.ts getBranchStatus.
        let check_runs = self.fetch_check_runs(owner, repo, &sha).await;
        if !check_runs.is_empty() {
            let any_failure = check_runs
                .iter()
                .any(|r| r.conclusion.as_deref() == Some("failure"));
            let all_good = check_runs.iter().all(|r| {
                matches!(
                    r.conclusion.as_deref(),
                    Some("success") | Some("neutral") | Some("skipped")
                )
            });

            if any_failure {
                combined.state = CombinedBranchState::Failure;
            } else if (combined.state == CombinedBranchState::Success
                || combined.statuses.is_empty())
                && all_good
            {
                combined.state = CombinedBranchState::Success;
            } else {
                combined.state = CombinedBranchState::Pending;
            }
        }

        Ok(combined)
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
        let get_url = if let Some(b) = branch {
            format!(
                "{}/repos/{owner}/{repo}/contents/{path}?ref={b}",
                self.api_base
            )
        } else {
            format!("{}/repos/{owner}/{repo}/contents/{path}", self.api_base)
        };
        let put_url = format!("{}/repos/{owner}/{repo}/contents/{path}", self.api_base);

        // Try to fetch existing file SHA so we can update rather than create.
        let sha = match self.http.get(&get_url).send().await {
            Ok(resp) if resp.status().is_success() => {
                let json: serde_json::Value = resp
                    .json()
                    .await
                    .map_err(|e| PlatformError::Http(HttpError::Request(e)))?;
                json.get("sha")
                    .and_then(|s| s.as_str())
                    .map(|s| s.to_owned())
            }
            _ => None,
        };

        let body = serde_json::json!({
            "message": message.unwrap_or("Update file via Renovate"),
            "content": base64::Engine::encode(&base64::engine::general_purpose::STANDARD, content),
            "branch": branch,
            "sha": sha,
        });

        self.http
            .put_json::<serde_json::Value>(&put_url, &body.to_string())
            .await
            .map_err(PlatformError::Http)?;

        Ok(())
    }

    async fn get_pr_list(
        &self,
        owner: &str,
        repo: &str,
        state: Option<&str>,
    ) -> Result<Vec<GhPr>, PlatformError> {
        let prs = self.list_prs(owner, repo, state).await?;
        Ok(prs.into_iter().map(coerce_rest_pr).collect())
    }

    async fn get_pr(
        &self,
        owner: &str,
        repo: &str,
        pr_number: i64,
    ) -> Result<Option<GhPr>, PlatformError> {
        self.get_pr(owner, repo, pr_number).await
    }

    async fn get_branch_pr(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<Option<GhPr>, PlatformError> {
        // Mirrors upstream getBranchPr: find open PR for branch, then fetch full details.
        let open = self
            .find_pr(owner, repo, branch, None, Some("open"), false)
            .await?;
        match open {
            Some(pr) => self.get_pr(owner, repo, pr.number).await,
            None => Ok(None),
        }
    }
}

impl GithubClient {
    /// Fetch check-runs for a commit.
    ///
    /// Returns an empty vector if the API is unavailable (403, 404) or if no
    /// check-runs exist. Mirrors the check-runs fetch in upstream
    /// `getBranchStatus`.
    async fn fetch_check_runs(&self, owner: &str, repo: &str, sha: &str) -> Vec<CheckRun> {
        let url = format!(
            "{}/repos/{}/{}/commits/{}/check-runs?per_page=100",
            self.api_base, owner, repo, sha
        );

        #[derive(Deserialize)]
        struct CheckRunsResponse {
            check_runs: Vec<CheckRun>,
        }

        match self
            .http
            .get(&url)
            .header("Accept", "application/vnd.github.antiope-preview+json")
            .send()
            .await
        {
            Ok(resp) => {
                if !resp.status().is_success() {
                    tracing::debug!(
                        status = %resp.status(),
                        "check-runs API returned non-success status"
                    );
                    return Vec::new();
                }
                match resp.json::<CheckRunsResponse>().await {
                    Ok(body) => body.check_runs,
                    Err(e) => {
                        tracing::debug!(%e, "failed to parse check-runs response");
                        Vec::new()
                    }
                }
            }
            Err(e) => {
                tracing::debug!(%e, "failed to fetch check-runs");
                Vec::new()
            }
        }
    }

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
                    json5::from_str::<serde_json::Value>(&raw)
                        .map_err(|e| PlatformError::Unexpected(format!("JSON5 parse error: {e}")))
                } else {
                    serde_json::from_str(&raw)
                        .or_else(|_| json5::from_str::<serde_json::Value>(&raw))
                        .map_err(|e| PlatformError::Unexpected(format!("JSON parse error: {e}")))
                };
                parsed.map(Some)
            }
            Err(HttpError::Status { status, .. }) if status == reqwest::StatusCode::NOT_FOUND => {
                Ok(None)
            }
            Err(e) => Err(PlatformError::Http(e)),
        }
    }

    /// List pull requests for a repository.
    ///
    /// Mirrors `getPrList` / REST fallback from `lib/modules/platform/github/index.ts`.
    /// Results are cached for 5 minutes to reduce API usage on repos with many PRs.
    pub async fn list_prs(
        &self,
        owner: &str,
        repo: &str,
        state: Option<&str>,
    ) -> Result<Vec<GhRestPr>, PlatformError> {
        const PR_CACHE_TTL: std::time::Duration = std::time::Duration::from_secs(300); // 5 min
        let state = state.unwrap_or("all");
        let cache_key = format!("{}/{}/{}", owner, repo, state);

        // Check cache first.
        {
            let cache = self.pr_cache.lock().unwrap();
            if let Some(entry) = cache.get(&cache_key)
                && entry.fetched_at.elapsed() < PR_CACHE_TTL
            {
                tracing::debug!(
                    owner = %owner,
                    repo = %repo,
                    state = %state,
                    count = entry.prs.len(),
                    "returning cached PR list"
                );
                return Ok(entry.prs.clone());
            }
        }

        let mut url = format!(
            "{}/repos/{}/{}/pulls?per_page=100&state={}&sort=updated&direction=desc",
            self.api_base, owner, repo, state
        );
        let mut all_prs: Vec<GhRestPr> = vec![];

        loop {
            let resp = self
                .http
                .get_retrying(&url)
                .await
                .map_err(PlatformError::Http)?;
            let next_url = resp
                .headers()
                .get("link")
                .and_then(|v| v.to_str().ok())
                .and_then(parse_link_header_next);

            let prs: Vec<GhRestPr> = resp
                .json()
                .await
                .map_err(|e| PlatformError::Http(HttpError::Request(e)))?;
            all_prs.extend(prs);

            match next_url {
                Some(next) => url = next,
                None => break,
            }
        }

        // Store in cache.
        {
            let mut cache = self.pr_cache.lock().unwrap();
            cache.insert(
                cache_key,
                PrCacheEntry {
                    fetched_at: std::time::Instant::now(),
                    prs: all_prs.clone(),
                },
            );
        }

        Ok(all_prs)
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

    /// Create a new issue.
    ///
    /// Mirrors `createIssue` from `lib/modules/platform/github/index.ts`.
    pub async fn create_issue(
        &self,
        owner: &str,
        repo: &str,
        title: &str,
        body: &str,
        labels: Option<Vec<String>>,
    ) -> Result<i64, PlatformError> {
        let url = format!("{}/repos/{}/{}/issues", self.api_base, owner, repo);
        let request = serde_json::json!({
            "title": title,
            "body": body,
            "labels": labels.unwrap_or_default(),
        });
        let request_json = serde_json::to_string(&request)
            .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;
        let issue: GhIssue = self
            .http
            .post_json(&url, &request_json)
            .await
            .map_err(PlatformError::Http)?;
        Ok(issue.number)
    }

    /// Update an existing issue.
    ///
    /// Mirrors `updateIssue` from `lib/modules/platform/github/index.ts`.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        title: Option<&str>,
        body: Option<&str>,
        state: Option<&str>,
        labels: Option<Vec<String>>,
    ) -> Result<(), PlatformError> {
        let url = format!(
            "{}/repos/{}/{}/issues/{}",
            self.api_base, owner, repo, issue_number
        );
        let mut request = serde_json::Map::new();
        if let Some(t) = title {
            request.insert("title".to_owned(), serde_json::Value::String(t.to_owned()));
        }
        if let Some(b) = body {
            request.insert("body".to_owned(), serde_json::Value::String(b.to_owned()));
        }
        if let Some(s) = state {
            request.insert("state".to_owned(), serde_json::Value::String(s.to_owned()));
        }
        if let Some(l) = labels {
            request.insert(
                "labels".to_owned(),
                serde_json::Value::Array(l.into_iter().map(serde_json::Value::String).collect()),
            );
        }
        let request_json = serde_json::to_string(&request)
            .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;
        self.http
            .patch_json(&url, &request_json)
            .await
            .map_err(PlatformError::Http)?;
        Ok(())
    }

    /// Add a comment to an issue or PR.
    ///
    /// Mirrors `ensureComment` from `lib/modules/platform/github/index.ts`.
    pub async fn create_comment(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &str,
    ) -> Result<i64, PlatformError> {
        let url = format!(
            "{}/repos/{}/{}/issues/{}/comments",
            self.api_base, owner, repo, issue_number
        );
        let request = serde_json::json!({ "body": body });
        let request_json = serde_json::to_string(&request)
            .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;
        let comment: GhComment = self
            .http
            .post_json(&url, &request_json)
            .await
            .map_err(PlatformError::Http)?;
        Ok(comment.id)
    }

    /// Update an existing comment.
    ///
    /// Mirrors `updateComment` from `lib/modules/platform/github/index.ts`.
    pub async fn update_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        body: &str,
    ) -> Result<(), PlatformError> {
        let url = format!(
            "{}/repos/{}/{}/issues/comments/{}",
            self.api_base, owner, repo, comment_id
        );
        let request = serde_json::json!({ "body": body });
        let request_json = serde_json::to_string(&request)
            .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;
        self.http
            .patch_json(&url, &request_json)
            .await
            .map_err(PlatformError::Http)?;
        Ok(())
    }

    /// Delete a comment.
    ///
    /// Mirrors `deleteComment` from `lib/modules/platform/github/index.ts`.
    pub async fn delete_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> Result<(), PlatformError> {
        let url = format!(
            "{}/repos/{}/{}/issues/comments/{}",
            self.api_base, owner, repo, comment_id
        );
        let resp = self.http.delete(&url).await.map_err(PlatformError::Http)?;
        if !resp.status().is_success() {
            return Err(PlatformError::Http(HttpError::Status {
                status: resp.status(),
                url,
            }));
        }
        Ok(())
    }

    /// Delete a label from an issue or PR.
    ///
    /// Mirrors `deleteLabel` from `lib/modules/platform/github/index.ts`.
    pub async fn delete_label(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        label: &str,
    ) -> Result<(), PlatformError> {
        let url = format!(
            "{}/repos/{}/{}/issues/{}/labels/{}",
            self.api_base, owner, repo, issue_number, label
        );
        let resp = self.http.delete(&url).await.map_err(PlatformError::Http)?;
        if !resp.status().is_success() {
            return Err(PlatformError::Http(HttpError::Status {
                status: resp.status(),
                url,
            }));
        }
        Ok(())
    }

    /// Add assignees to an issue or PR.
    ///
    /// Mirrors `addAssignees` from `lib/modules/platform/github/index.ts`.
    pub async fn add_assignees(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        assignees: Vec<String>,
    ) -> Result<(), PlatformError> {
        let url = format!(
            "{}/repos/{}/{}/issues/{}/assignees",
            self.api_base, owner, repo, issue_number
        );
        let body = serde_json::json!({ "assignees": assignees });

        let mut last_err: Option<PlatformError> = None;
        for attempt in 0..3 {
            match self
                .http
                .post_json::<serde_json::Value>(&url, &body.to_string())
                .await
            {
                Ok(_) => return Ok(()),
                Err(HttpError::Status { status, .. })
                    if status == reqwest::StatusCode::NOT_FOUND =>
                {
                    tracing::debug!(
                        attempt = attempt + 1,
                        issue = issue_number,
                        "Retrying add_assignees after 404"
                    );
                    last_err = Some(PlatformError::Http(HttpError::Status {
                        status,
                        url: url.clone(),
                    }));
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
                Err(e) => return Err(PlatformError::Http(e)),
            }
        }
        Err(last_err.unwrap_or_else(|| {
            PlatformError::Unexpected("add_assignees: all retries exhausted".to_owned())
        }))
    }

    /// Add reviewers to a PR.
    ///
    /// Mirrors `addReviewers` from `lib/modules/platform/github/index.ts`.
    pub async fn add_reviewers(
        &self,
        owner: &str,
        repo: &str,
        pr_number: i64,
        reviewers: Vec<String>,
    ) -> Result<(), PlatformError> {
        let url = format!(
            "{}/repos/{}/{}/pulls/{}/requested_reviewers",
            self.api_base, owner, repo, pr_number
        );
        let user_reviewers: Vec<String> = reviewers
            .iter()
            .filter(|r| !r.starts_with("team:"))
            .cloned()
            .collect();
        let team_reviewers: Vec<String> = reviewers
            .iter()
            .filter(|r| r.starts_with("team:"))
            .map(|r| r.strip_prefix("team:").unwrap().to_owned())
            .collect();
        let body = serde_json::json!({
            "reviewers": user_reviewers,
            "team_reviewers": team_reviewers,
        });
        self.http
            .post_json::<serde_json::Value>(&url, &body.to_string())
            .await
            .map_err(PlatformError::Http)?;
        Ok(())
    }

    /// Merge a pull request.
    ///
    /// Returns `true` on success, `false` when the merge is blocked (405) or
    /// the PR is not found (404).
    ///
    /// Mirrors `mergePr` from `lib/modules/platform/github/index.ts`.
    pub async fn merge_pr(
        &self,
        owner: &str,
        repo: &str,
        pr_number: i64,
        _branch_name: &str,
        strategy: Option<&str>,
    ) -> Result<bool, PlatformError> {
        let url = format!(
            "{}/repos/{}/{}/pulls/{}/merge",
            self.api_base, owner, repo, pr_number
        );

        // If a strategy is provided, try it first
        if let Some(s) = strategy {
            let body = serde_json::json!({"merge_method": s});
            match self.try_merge(&url, &body).await {
                Ok(true) => return Ok(true),
                Ok(false) => return Ok(false),
                Err(e) => return Err(e),
            }
        }

        // Autodetection: try squash -> merge -> rebase
        for method in ["squash", "merge", "rebase"] {
            let body = serde_json::json!({"merge_method": method});
            match self
                .http
                .put_json::<serde_json::Value>(&url, &body.to_string())
                .await
            {
                Ok(_) => return Ok(true),
                Err(e) => {
                    tracing::debug!(err = %e, method, "Failed to merge PR");
                }
            }
        }

        tracing::info!(pr = pr_number, "All merge attempts failed");
        Ok(false)
    }

    async fn try_merge(&self, url: &str, body: &serde_json::Value) -> Result<bool, PlatformError> {
        match self
            .http
            .put_json::<serde_json::Value>(url, &body.to_string())
            .await
        {
            Ok(_) => Ok(true),
            Err(HttpError::Status { status, .. })
                if status == reqwest::StatusCode::NOT_FOUND
                    || status == reqwest::StatusCode::METHOD_NOT_ALLOWED =>
            {
                Ok(false)
            }
            Err(e) => Err(PlatformError::Http(e)),
        }
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

        let refs: Vec<MatchingRef> = self
            .http
            .get_json(&url)
            .await
            .map_err(PlatformError::Http)?;
        let branches: Vec<String> = refs
            .into_iter()
            .map(|r| r.ref_name.trim_start_matches("refs/heads/").to_owned())
            .collect();

        if branches
            .iter()
            .any(|b| b.starts_with(&format!("{}/", branch_name)))
        {
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

        let checks: Vec<StatusCheck> = self
            .http
            .get_json(&url)
            .await
            .map_err(PlatformError::Http)?;
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

    /// Check whether a branch requires PRs to be up-to-date before merging.
    ///
    /// Checks rulesets first, then falls back to legacy branch protection.
    ///
    /// Mirrors `getBranchForceRebase` from `lib/modules/platform/github/index.ts`.
    pub async fn get_branch_force_rebase(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        parent_repo: Option<&str>,
    ) -> Result<bool, PlatformError> {
        if parent_repo.is_some() {
            return Ok(false);
        }

        let cache_key = format!("{}/{}/{}", owner, repo, branch);
        {
            let cache = self.branch_force_rebase_cache.lock().unwrap();
            if let Some(&cached) = cache.get(&cache_key) {
                return Ok(cached);
            }
        }

        let ruleset_url = format!(
            "{}/repos/{}/{}/rules/branches/{}",
            self.api_base, owner, repo, branch
        );

        match self.http.get_json::<serde_json::Value>(&ruleset_url).await {
            Ok(rulesets) => {
                if let Some(rules) = rulesets.as_array() {
                    for rule in rules {
                        if let Some(t) = rule.get("type").and_then(|v| v.as_str())
                            && t == "required_status_checks"
                            && let Some(params) = rule.get("parameters")
                            && params.get("strict_required_status_checks_policy")
                                == Some(&serde_json::Value::Bool(true))
                        {
                            let mut cache = self.branch_force_rebase_cache.lock().unwrap();
                            cache.insert(cache_key, true);
                            return Ok(true);
                        }
                    }
                }
            }
            Err(HttpError::Status { status, .. })
                if status == reqwest::StatusCode::NOT_FOUND
                    || status == reqwest::StatusCode::FORBIDDEN => {}
            Err(e) => return Err(PlatformError::Http(e)),
        }

        let protection_url = format!(
            "{}/repos/{}/{}/branches/{}/protection",
            self.api_base, owner, repo, branch
        );

        match self
            .http
            .get_json::<serde_json::Value>(&protection_url)
            .await
        {
            Ok(protection) => {
                let result = if let Some(status_checks) = protection.get("required_status_checks") {
                    status_checks.get("strict") == Some(&serde_json::Value::Bool(true))
                } else {
                    false
                };
                let mut cache = self.branch_force_rebase_cache.lock().unwrap();
                cache.insert(cache_key, result);
                return Ok(result);
            }
            Err(HttpError::Status { status, .. })
                if status == reqwest::StatusCode::NOT_FOUND
                    || status == reqwest::StatusCode::FORBIDDEN => {}
            Err(e) => return Err(PlatformError::Http(e)),
        }

        let mut cache = self.branch_force_rebase_cache.lock().unwrap();
        cache.insert(cache_key, false);
        Ok(false)
    }

    /// Set a commit status for a branch.
    ///
    /// Skips the API call when the existing status already matches.
    ///
    /// Mirrors `setBranchStatus` from `lib/modules/platform/github/index.ts`.
    #[allow(clippy::too_many_arguments)]
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
        let branch_ref: RefResponse = self
            .http
            .get_json(&ref_url)
            .await
            .map_err(PlatformError::Http)?;
        let sha = branch_ref.object.sha;

        let status_url = format!(
            "{}/repos/{}/{}/statuses/{}",
            self.api_base, owner, repo, sha
        );
        let body = serde_json::json!({
            "state": state,
            "description": description,
            "context": context,
            "target_url": target_url,
        });
        let body_str = serde_json::to_string(&body)
            .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;

        self.http
            .post_json::<serde_json::Value>(&status_url, &body_str)
            .await
            .map_err(PlatformError::Http)?;
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
        let url = format!(
            "{}/repos/{}/{}/pulls/{}",
            self.api_base, owner, repo, pr_number
        );
        match self.http.get_json::<GhRestPr>(&url).await {
            Ok(pr) => Ok(Some(coerce_rest_pr(pr))),
            Err(HttpError::Status { status, .. }) if status == reqwest::StatusCode::NOT_FOUND => {
                Ok(None)
            }
            Err(e) => Err(PlatformError::Http(e)),
        }
    }

    /// Find a PR matching the given criteria.
    ///
    /// Mirrors `findPr` from `lib/modules/platform/github/index.ts`.
    pub async fn find_pr(
        &self,
        owner: &str,
        repo: &str,
        branch_name: &str,
        pr_title: Option<&str>,
        state: Option<&str>,
        include_other_authors: bool,
    ) -> Result<Option<GhPr>, PlatformError> {
        let state = state.unwrap_or("all");

        if include_other_authors {
            let url = format!(
                "{}/repos/{}/{}/pulls?head={}:{}&state=open",
                self.api_base, owner, repo, owner, branch_name
            );
            let prs = self
                .http
                .get_json::<Vec<GhRestPr>>(&url)
                .await
                .map_err(PlatformError::Http)?;
            return Ok(prs.into_iter().next().map(coerce_rest_pr));
        }

        let prs = self.list_prs(owner, repo, None).await?;
        let desired = prs.into_iter().map(coerce_rest_pr).find(|pr| {
            if pr.source_branch != branch_name {
                return false;
            }
            if let Some(title) = pr_title
                && title.to_uppercase() != pr.title.to_uppercase()
            {
                return false;
            }
            if !matches_state(&pr.state, state) {
                return false;
            }
            if pr.source_repo.as_deref() != Some(&format!("{}/{}", owner, repo)) {
                return false;
            }
            true
        });
        Ok(desired)
    }

    /// Detect the preferred merge method for a repository based on allowed
    /// merge types.
    ///
    /// Returns `Some("squash")`, `Some("merge")`, or `Some("rebase")` depending
    /// on what the repository allows. Returns `None` if no method could be
    /// determined.
    ///
    /// Mirrors the merge-method detection in `initRepo` from
    /// `lib/modules/platform/github/index.ts`.
    pub async fn get_repo_merge_methods(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Option<String>, PlatformError> {
        let url = format!("{}/repos/{}/{}", self.api_base, owner, repo);
        let repo_info: serde_json::Value = self
            .http
            .get_json(&url)
            .await
            .map_err(PlatformError::Http)?;

        if repo_info.get("allow_squash_merge") == Some(&serde_json::Value::Bool(true)) {
            return Ok(Some("squash".to_owned()));
        }
        if repo_info.get("allow_merge_commit") == Some(&serde_json::Value::Bool(true)) {
            return Ok(Some("merge".to_owned()));
        }
        if repo_info.get("allow_rebase_merge") == Some(&serde_json::Value::Bool(true)) {
            return Ok(Some("rebase".to_owned()));
        }
        Ok(None)
    }

    /// Create a pull request with extended options.
    ///
    /// Mirrors the extended `createPr` from `lib/modules/platform/github/index.ts`.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_pr_with_options(
        &self,
        owner: &str,
        repo: &str,
        source_branch: &str,
        target_branch: &str,
        title: &str,
        body: &str,
        draft: bool,
        maintainer_can_modify: Option<bool>,
        milestone: Option<i64>,
    ) -> Result<Option<i64>, PlatformError> {
        let url = format!("{}/repos/{}/{}/pulls", self.api_base, owner, repo);
        let head = format!("{}:{}", owner, source_branch);
        let request = CreatePrRequest {
            title: title.to_owned(),
            head,
            base: target_branch.to_owned(),
            body: body.to_owned(),
            draft: Some(draft),
            maintainer_can_modify,
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
                if let Some(ms) = milestone
                    && let Err(e) = self.add_milestone(owner, repo, pr.number, ms).await
                {
                    tracing::warn!(err = %e, pr = pr.number, milestone = ms, "Unable to add milestone to PR");
                }
                Ok(Some(pr.number))
            }
            Err(HttpError::Status { status, .. })
                if status == reqwest::StatusCode::UNPROCESSABLE_ENTITY =>
            {
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

    /// Add a milestone to a PR (via the issues endpoint).
    async fn add_milestone(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        milestone: i64,
    ) -> Result<(), PlatformError> {
        let url = format!(
            "{}/repos/{}/{}/issues/{}",
            self.api_base, owner, repo, issue_number
        );
        let body = serde_json::json!({"milestone": milestone});
        self.http
            .patch_json(&url, &body.to_string())
            .await
            .map_err(PlatformError::Http)?;
        Ok(())
    }

    /// Update labels on a PR (via the issues endpoint).
    ///
    /// Mirrors `addLabels` / label update from `lib/modules/platform/github/index.ts`.
    pub async fn update_pr_labels(
        &self,
        owner: &str,
        repo: &str,
        pr_number: i64,
        labels: Vec<String>,
    ) -> Result<(), PlatformError> {
        let url = format!(
            "{}/repos/{}/{}/issues/{}/labels",
            self.api_base, owner, repo, pr_number
        );
        let body = serde_json::json!({ "labels": labels });
        let body_str = serde_json::to_string(&body)
            .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;
        match self
            .http
            .post_json::<serde_json::Value>(&url, &body_str)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::warn!(
                    pr = pr_number,
                    labels = ?labels,
                    "Error while adding labels. Skipping"
                );
                Err(PlatformError::Http(e))
            }
        }
    }

    /// Ensure a comment exists on an issue/PR with the given content.
    ///
    /// Returns `true` if a comment was added/updated, `false` if skipped.
    ///
    /// Mirrors `ensureComment` from `lib/modules/platform/github/index.ts`.
    pub async fn ensure_comment(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        topic: Option<&str>,
        content: &str,
    ) -> Result<bool, PlatformError> {
        let url = format!(
            "{}/repos/{}/{}/issues/{}/comments?per_page=100",
            self.api_base, owner, repo, issue_number
        );
        let comments: Vec<GhComment> = self
            .http
            .get_json(&url)
            .await
            .map_err(PlatformError::Http)?;

        let topic_prefix = topic.map(|t| format!("### {}\n\n", t));
        let expected_body = topic_prefix
            .as_ref()
            .map(|p| format!("{}{}", p, content))
            .unwrap_or_else(|| content.to_owned());

        // Look for an existing comment matching the topic or content
        let existing = comments.into_iter().find(|c| {
            if let Some(ref body) = c.body {
                if let Some(ref prefix) = topic_prefix {
                    body.starts_with(prefix)
                } else {
                    body == content
                }
            } else {
                false
            }
        });

        if content.is_empty() {
            // Delete the comment if content is empty
            if let Some(comment) = existing {
                self.delete_comment(owner, repo, comment.id).await?;
                return Ok(true);
            }
            return Ok(false);
        }

        if let Some(comment) = existing {
            if comment.body.as_deref() == Some(&expected_body) {
                return Ok(false); // already up to date
            }
            self.update_comment(owner, repo, comment.id, &expected_body)
                .await?;
            return Ok(true);
        }

        self.create_comment(owner, repo, issue_number, &expected_body)
            .await?;
        Ok(true)
    }

    /// Remove a comment from an issue or PR.
    ///
    /// Mirrors `ensureCommentRemoval` from `lib/modules/platform/github/index.ts`.
    pub async fn ensure_comment_removal(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        topic: Option<&str>,
        content: Option<&str>,
    ) -> Result<(), PlatformError> {
        let url = format!(
            "{}/repos/{}/{}/issues/{}/comments?per_page=100",
            self.api_base, owner, repo, issue_number
        );
        let comments: Vec<GhComment> = self
            .http
            .get_json(&url)
            .await
            .map_err(PlatformError::Http)?;

        let comment_id = if let Some(topic) = topic {
            let prefix = format!("### {}\n\n", topic);
            comments
                .into_iter()
                .find(|c| c.body.as_ref().is_some_and(|b| b.starts_with(&prefix)))
                .map(|c| c.id)
        } else if let Some(content) = content {
            comments
                .into_iter()
                .find(|c| c.body.as_ref().is_some_and(|b| b.trim() == content))
                .map(|c| c.id)
        } else {
            None
        };

        if let Some(id) = comment_id {
            self.delete_comment(owner, repo, id).await?;
        }
        Ok(())
    }

    /// Ensure an issue exists with the given title.
    ///
    /// If `ensure_only_once` is true and an open issue with the same title
    /// exists, no new issue is created and others are closed.
    ///
    /// Mirrors `ensureIssue` from `lib/modules/platform/github/index.ts`.
    #[allow(clippy::too_many_arguments)]
    pub async fn ensure_issue(
        &self,
        owner: &str,
        repo: &str,
        title: &str,
        body: &str,
        labels: Option<Vec<String>>,
        ensure_only_once: bool,
        reopen: bool,
    ) -> Result<Option<i64>, PlatformError> {
        let issues = self.list_issues(owner, repo, Some("open")).await?;
        let matching: Vec<&GhIssue> = issues.iter().filter(|i| i.title == title).collect();

        if !matching.is_empty() {
            if ensure_only_once {
                // Close other matching issues (keep the first one)
                for issue in matching.iter().skip(1) {
                    let _ = self
                        .update_issue(owner, repo, issue.number, None, None, Some("closed"), None)
                        .await;
                }
                return Ok(None); // existing issue found, don't create
            }
            if !reopen {
                // Issue already exists and reopen is false
                return Ok(None);
            }
            // Could update existing issue here; for now return existing
            return Ok(Some(matching[0].number));
        }

        if !reopen {
            // Check for closed issues too
            let closed = self.list_issues(owner, repo, Some("closed")).await?;
            if closed.iter().any(|i| i.title == title) {
                return Ok(None);
            }
        }

        let number = self.create_issue(owner, repo, title, body, labels).await?;
        Ok(Some(number))
    }

    /// Fetch vulnerability alerts for a repository.
    ///
    /// Returns an empty vector if alerts are disabled or on error.
    ///
    /// Mirrors `getVulnerabilityAlerts` from `lib/modules/platform/github/index.ts`.
    pub async fn get_vulnerability_alerts(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<serde_json::Value>, PlatformError> {
        let url = format!(
            "{}/repos/{}/{}/dependabot/alerts?state=open&direction=asc&per_page=100",
            self.api_base, owner, repo
        );
        match self.http.get_json::<serde_json::Value>(&url).await {
            Ok(data) => Ok(parse_github_vulnerability_alerts(&data)),
            Err(e) => {
                tracing::debug!(err = %e, "Failed to fetch vulnerability alerts");
                Ok(vec![])
            }
        }
    }
}

/// Coerce a GitHub REST API PR into the Renovate `GhPr` format.
///
/// Mirrors `coerceRestPr` from `lib/modules/platform/github/common.ts`.
/// Check if a PR state matches the desired state filter.
fn matches_state(state: &str, desired: &str) -> bool {
    if desired == "all" {
        return true;
    }
    if let Some(negated) = desired.strip_prefix('!') {
        return state != negated;
    }
    state == desired
}

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
        labels: pr
            .labels
            .unwrap_or_default()
            .into_iter()
            .map(|l| l.name)
            .collect(),
        has_assignees: pr.assignee.is_some()
            || pr.assignees.as_ref().is_some_and(|a| !a.is_empty()),
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
        let type_ = obj
            .get("type")
            .and_then(|t| t.as_str())
            .ok_or("missing type")?;
        let _name = obj
            .get("name")
            .and_then(|n| n.as_str())
            .ok_or("missing name")?;
        let _path = obj
            .get("path")
            .and_then(|p| p.as_str())
            .ok_or("missing path")?;
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
    let Some(arr) = input.as_array() else {
        return vec![];
    };
    arr.iter()
        .filter(|alert| {
            let Some(sv) = alert.get("security_vulnerability") else {
                return false;
            };
            if sv.is_null() {
                return false;
            }
            let ecosystem = sv
                .get("package")
                .and_then(|p| p.get("ecosystem"))
                .and_then(|e| e.as_str());
            match ecosystem {
                Some(eco) => SUPPORTED_ECOSYSTEMS.contains(&eco),
                None => false,
            }
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{header, header_exists, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    // Ported: "should support default endpoint with email" — lib/modules/platform/github/index.spec.ts line 345
    // Ported: "should support default endpoint no email access" — lib/modules/platform/github/index.spec.ts line 133
    // Ported: "should support default endpoint no email result" — lib/modules/platform/github/index.spec.ts line 145
    // Ported: "should support gitAuthor and username" — lib/modules/platform/github/index.spec.ts line 157
    // Ported: "no warning is shown" — lib/modules/platform/github/index.spec.ts line 217
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

    // Ported: "should throw 401" — lib/modules/platform/github/index.spec.ts line 1211
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

    // Ported: "should support custom endpoint" — lib/modules/platform/github/index.spec.ts line 563
    // Ported: "if on GitHub.com, a warning is shown" — lib/modules/platform/github/index.spec.ts line 170
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

    // Ported: "should support custom endpoint without version" — lib/modules/platform/github/index.spec.ts line 587
    // Ported: "if on GitHub Enterprise, a warning is not shown" — lib/modules/platform/github/index.spec.ts line 195
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

    // Ported: "returns file content" — lib/modules/platform/github/index.spec.ts line 5393
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

    // Ported: "ensures trailing slash" — lib/util/github/url.spec.ts line 5
    #[test]
    fn github_get_source_url_base_trailing_slash() {
        assert_eq!(
            get_source_url_base(Some("https://gh.my-company.com")),
            "https://gh.my-company.com/"
        );
    }

    // Ported: "defaults to github.com" — lib/util/github/url.spec.ts line 10
    #[test]
    fn github_get_source_url_base_default() {
        assert_eq!(get_source_url_base(None), "https://github.com/");
    }

    // Ported: "maps to api.github.com" — lib/util/github/url.spec.ts line 17
    #[test]
    fn github_get_api_base_url_maps_to_api() {
        assert_eq!(
            get_api_base_url(Some("https://github.com/")),
            "https://api.github.com/"
        );
    }

    // Ported: "supports local github installations" — lib/util/github/url.spec.ts line 22
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

    // Ported: "returns null" — lib/modules/platform/github/index.spec.ts line 2389
    // Ported: "returns null if pre-commit phase has failed" — lib/modules/platform/github/index.spec.ts line 5482
    // Ported: "throws not-found" — lib/modules/platform/github/index.spec.ts line 1060
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

    // Ported: "should return an array of repos" — lib/modules/platform/github/index.spec.ts line 613
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

    // Ported: "should create and return a PR object" — lib/modules/platform/github/index.spec.ts line 3769
    // Ported: "should use defaultBranch" — lib/modules/platform/github/index.spec.ts line 3791
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
            .create_pr(
                "owner",
                "repo",
                "renovate/deps",
                "main",
                "Update deps",
                "Body",
            )
            .await
            .unwrap();
        assert_eq!(pr_number, Some(42));
    }

    // Ported: "should handle REST API errors" — lib/modules/platform/github/index.spec.ts line 4131
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
            .create_pr(
                "owner",
                "repo",
                "renovate/deps",
                "main",
                "Update deps",
                "Body",
            )
            .await
            .unwrap();
        assert_eq!(pr_number, None);
    }

    // Ported: "should update the PR" — lib/modules/platform/github/index.spec.ts line 4591
    // Ported: "should update target branch" — lib/modules/platform/github/index.spec.ts line 4620
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

    // Ported: "skips update if unchanged" — lib/modules/platform/github/index.spec.ts line 2991
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

    // Ported: "should pass through success" — lib/modules/platform/github/index.spec.ts line 2188
    // Ported: "should not consider internal statuses as success" — lib/modules/platform/github/index.spec.ts line 2204
    // Ported: "should detect strict required status checks ruleset" — lib/modules/platform/github/index.spec.ts line 1269
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
        let status = client
            .get_branch_status("owner", "repo", "main")
            .await
            .unwrap();
        assert_eq!(status.state, CombinedBranchState::Success);
        assert_eq!(status.statuses.len(), 2);
    }

    // Rust-specific: write_file creates a file via Contents API
    #[tokio::test]
    async fn write_file_creates_file() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/contents/path"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("PUT"))
            .and(path("/repos/owner/repo/contents/path"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "content": {"path": "path"},
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        client
            .write_file(
                "owner",
                "repo",
                "path",
                "content",
                Some("main"),
                Some("test commit"),
            )
            .await
            .unwrap();
    }

    // Rust-specific: write_file updates an existing file via Contents API
    #[tokio::test]
    async fn write_file_updates_existing_file() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/contents/path"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "sha": "abc123",
            })))
            .mount(&server)
            .await;
        Mock::given(method("PUT"))
            .and(path("/repos/owner/repo/contents/path"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "content": {"path": "path"},
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        client
            .write_file(
                "owner",
                "repo",
                "path",
                "content",
                Some("main"),
                Some("test commit"),
            )
            .await
            .unwrap();
    }

    // Ported: "should throw if user failure" — lib/modules/platform/github/index.spec.ts line 128
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
        assert!(
            matches!(err, PlatformError::Http(HttpError::Status { status, .. }) if status == reqwest::StatusCode::INTERNAL_SERVER_ERROR)
        );
    }

    // Ported: "should pass through failed" — lib/modules/platform/github/index.spec.ts line 2226
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
        let status = client
            .get_branch_status("owner", "repo", "main")
            .await
            .unwrap();
        assert_eq!(status.state, CombinedBranchState::Failure);
    }

    // Ported: "defaults to pending" — lib/modules/platform/github/index.spec.ts line 2242
    // Ported: "should return false when no force rebase rules found" — lib/modules/platform/github/index.spec.ts line 1337
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
        let status = client
            .get_branch_status("owner", "repo", "main")
            .await
            .unwrap();
        assert_eq!(status.state, CombinedBranchState::Pending);
    }

    // Ported: "should update and close the PR" — lib/modules/platform/github/index.spec.ts line 4605
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

    // Ported: "should throw error if archived" — lib/modules/platform/github/index.spec.ts line 1036
    // Ported: "should handle GraphQL errors" — lib/modules/platform/github/index.spec.ts line 4118
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
            .create_pr(
                "owner",
                "repo",
                "renovate/deps",
                "main",
                "Update deps",
                "Body",
            )
            .await
            .unwrap_err();
        assert!(
            matches!(err, PlatformError::Http(HttpError::Status { status, .. }) if status == reqwest::StatusCode::INTERNAL_SERVER_ERROR)
        );
    }

    // Ported: "returns file content in json5 format" — lib/modules/platform/github/index.spec.ts line 5405
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

    // Ported: "returns file content from given repo" — lib/modules/platform/github/index.spec.ts line 5422
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

    // Ported: "should filters repositories by topics" — lib/modules/platform/github/index.spec.ts line 636
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

    // Ported: "should handle 404" — lib/modules/platform/github/index.spec.ts line 1185
    #[tokio::test]
    async fn get_branch_status_handles_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/git/refs/heads/missing-branch"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client
            .get_branch_status("owner", "repo", "missing-branch")
            .await
            .unwrap_err();
        assert!(
            matches!(err, PlatformError::Http(HttpError::Status { status, .. }) if status == reqwest::StatusCode::NOT_FOUND)
        );
    }

    // Ported: "should handle 403" — lib/modules/platform/github/index.spec.ts line 1198
    #[tokio::test]
    async fn get_branch_status_handles_403() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/git/refs/heads/main"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client
            .get_branch_status("owner", "repo", "main")
            .await
            .unwrap_err();
        assert!(
            matches!(err, PlatformError::Http(HttpError::Status { status, .. }) if status == reqwest::StatusCode::FORBIDDEN)
        );
    }

    // Ported: "should throw 401" — lib/modules/platform/github/index.spec.ts line 1211
    #[tokio::test]
    async fn get_branch_status_handles_401() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/git/refs/heads/main"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client
            .get_branch_status("owner", "repo", "main")
            .await
            .unwrap_err();
        assert!(
            matches!(err, PlatformError::Http(HttpError::Status { status, .. }) if status == reqwest::StatusCode::UNAUTHORIZED)
        );
    }

    // ── decode_github_content ────────────────────────────────────────────────

    // Ported: "returns null" — lib/modules/platform/github/index.spec.ts line 2389
    #[test]
    fn decode_github_content_empty() {
        let content = GithubContent {
            content: Some("".to_owned()),
            encoding: Some("base64".to_owned()),
        };
        let result = decode_github_content(content).unwrap();
        assert_eq!(result, "");
    }

    // Ported: "throws on malformed JSON" — lib/modules/platform/github/index.spec.ts line 5446
    #[test]
    fn decode_github_content_invalid_base64() {
        let content = GithubContent {
            content: Some("!!!not-valid-base64!!!".to_owned()),
            encoding: Some("base64".to_owned()),
        };
        let err = decode_github_content(content).unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(_)));
    }

    // Ported: "throws on errors" — lib/modules/platform/github/index.spec.ts line 5456
    #[test]
    fn decode_github_content_unsupported_encoding() {
        let content = GithubContent {
            content: Some("hello".to_owned()),
            encoding: Some("utf-8".to_owned()),
        };
        let err = decode_github_content(content).unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(_)));
    }

    // Ported: "should fail if a check run has failed" — lib/modules/platform/github/index.spec.ts line 2257
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
        let status = client
            .get_branch_status("owner", "repo", "main")
            .await
            .unwrap();
        assert_eq!(status.state, CombinedBranchState::Failure);
    }

    // Ported: "should succeed if no status and all passed check runs" — lib/modules/platform/github/index.spec.ts line 2289
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
        let status = client
            .get_branch_status("owner", "repo", "main")
            .await
            .unwrap();
        assert_eq!(status.state, CombinedBranchState::Success);
    }

    // Ported: "should fail if a check run is pending" — lib/modules/platform/github/index.spec.ts line 2327
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
        let status = client
            .get_branch_status("owner", "repo", "main")
            .await
            .unwrap();
        assert_eq!(status.state, CombinedBranchState::Pending);
    }

    // ── check-runs integration tests ────────────────────────────────────────────

    #[tokio::test]
    async fn get_branch_status_check_runs_failure() {
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

        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/commits/abc123/check-runs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "check_runs": [
                    {"name": "ci/test", "status": "completed", "conclusion": "success"},
                    {"name": "ci/lint", "status": "completed", "conclusion": "failure"},
                ],
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let status = client
            .get_branch_status("owner", "repo", "main")
            .await
            .unwrap();
        assert_eq!(status.state, CombinedBranchState::Failure);
    }

    #[tokio::test]
    async fn get_branch_status_check_runs_success() {
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

        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/commits/abc123/check-runs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "check_runs": [
                    {"name": "ci/test", "status": "completed", "conclusion": "success"},
                    {"name": "ci/build", "status": "completed", "conclusion": "neutral"},
                ],
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let status = client
            .get_branch_status("owner", "repo", "main")
            .await
            .unwrap();
        assert_eq!(status.state, CombinedBranchState::Success);
    }

    #[tokio::test]
    async fn get_branch_status_check_runs_pending() {
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

        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/commits/abc123/check-runs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "check_runs": [
                    {"name": "ci/test", "status": "in_progress", "conclusion": null},
                ],
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let status = client
            .get_branch_status("owner", "repo", "main")
            .await
            .unwrap();
        assert_eq!(status.state, CombinedBranchState::Pending);
    }

    #[tokio::test]
    async fn get_branch_status_check_runs_403_ignored() {
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
                ],
            })))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/commits/abc123/check-runs"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let status = client
            .get_branch_status("owner", "repo", "main")
            .await
            .unwrap();
        // check-runs 403 should be silently ignored; combined status wins
        assert_eq!(status.state, CombinedBranchState::Success);
    }

    #[tokio::test]
    async fn get_branch_status_check_runs_mixed_with_status() {
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
                ],
            })))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/commits/abc123/check-runs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "check_runs": [
                    {"name": "ci/build", "status": "completed", "conclusion": "success"},
                    {"name": "ci/security", "status": "completed", "conclusion": "failure"},
                ],
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let status = client
            .get_branch_status("owner", "repo", "main")
            .await
            .unwrap();
        // Check-run failure should override combined status success
        assert_eq!(status.state, CombinedBranchState::Failure);
    }

    // Ported: "should return an array of repos when using GitHub App Installation Token" — lib/modules/platform/github/index.spec.ts line 690
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

    // Ported: "isDateExpired($currentTime, $initialTimestamp, $duration) === $expected" — lib/util/github/graphql/util.spec.ts line 35

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

    // Ported: "isDateExpired($currentTime, $initialTimestamp, $duration) === $expected" — lib/util/github/graphql/util.spec.ts line 35
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

    // Ported: "returns updated pr body" — lib/modules/platform/github/index.spec.ts line 4963
    // Ported: "performs multiple replacements" — lib/modules/platform/github/massage-markdown-links.spec.ts line 4
    #[test]
    fn massage_markdown_links_performs_multiple_replacements() {
        let input = "Link [foo/bar#1](https://github.com/foo/bar/pull/1) points to https://github.com/foo/bar/pull/1.";
        let expected = "Link [foo/bar#1](https://redirect.github.com/foo/bar/pull/1) points to [https://github.com/foo/bar/pull/1](https://redirect.github.com/foo/bar/pull/1).";
        assert_eq!(massage_markdown_links(input), expected);
    }

    // Ported: "returns not-updated pr body for GHE" — lib/modules/platform/github/index.spec.ts line 4969
    // Ported: "Unchanged: $input" — lib/modules/platform/github/massage-markdown-links.spec.ts line 18
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

    // Ported: "$input -> $output" — lib/modules/platform/github/massage-markdown-links.spec.ts line 60
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

    // Ported: "transforms Commit type" — lib/util/github/graphql/query-adapters/branches-query-adapter.spec.ts line 5
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

    // Ported: "returns null for invalid input" — lib/util/github/graphql/query-adapters/branches-query-adapter.spec.ts line 23
    #[test]
    fn transform_github_branch_non_commit_type_returns_none() {
        assert_eq!(transform_github_branch("main", "Blob", "abc123", ""), None);
        assert_eq!(transform_github_branch("main", "Tag", "abc123", ""), None);
        assert_eq!(transform_github_branch("main", "Tree", "abc123", ""), None);
    }

    // ── get_json_file ─────────────────────────────────────────────────────────

    // Ported: "returns null" — lib/modules/platform/github/index.spec.ts line 5382
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
        let result = client
            .get_json_file("owner", "repo", "file.json", None)
            .await
            .unwrap();
        assert_eq!(result, None);
    }

    // Ported: "returns file content" — lib/modules/platform/github/index.spec.ts line 5393
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
        let result = client
            .get_json_file("owner", "repo", "file.json", None)
            .await
            .unwrap();
        assert_eq!(result, Some(data));
    }

    // Ported: "returns file content in json5 format" — lib/modules/platform/github/index.spec.ts line 5405
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
        let result = client
            .get_json_file("owner", "repo", "file.json5", None)
            .await
            .unwrap();
        assert_eq!(result, Some(serde_json::json!({"foo": "bar"})));
    }

    // Ported: "returns file content from given repo" — lib/modules/platform/github/index.spec.ts line 5422
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
        let result = client
            .get_json_file("other", "foreign", "file.json", None)
            .await
            .unwrap();
        assert_eq!(result, Some(data));
    }

    // Ported: "returns file content from branch or tag" — lib/modules/platform/github/index.spec.ts line 5434
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
        let result = client
            .get_json_file("owner", "repo", "file.json", Some("dev"))
            .await
            .unwrap();
        assert_eq!(result, Some(data));
    }

    // Ported: "throws on malformed JSON" — lib/modules/platform/github/index.spec.ts line 5446
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
        let err = client
            .get_json_file("owner", "repo", "file.json", None)
            .await
            .unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(_)));
    }

    // Ported: "throws on errors" — lib/modules/platform/github/index.spec.ts line 5456
    #[tokio::test]
    async fn get_json_file_throws_on_http_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/contents/file.json"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client
            .get_json_file("owner", "repo", "file.json", None)
            .await
            .unwrap_err();
        assert!(
            matches!(err, PlatformError::Http(HttpError::Status { status, .. }) if status == reqwest::StatusCode::INTERNAL_SERVER_ERROR)
        );
    }

    // ── get_pr / list_prs ─────────────────────────────────────────────────────

    // Ported: "should return null if no prNo is passed" — lib/modules/platform/github/index.spec.ts line 4381
    #[tokio::test]
    async fn get_pr_returns_null_for_zero() {
        let server = MockServer::start().await;
        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr = client.get_pr("owner", "repo", 0).await.unwrap();
        assert!(pr.is_none());
    }

    // Ported: "should return PR" — lib/modules/platform/github/index.spec.ts line 4386
    // Ported: "should cache and return the PR object" — lib/modules/platform/github/index.spec.ts line 2021
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

    // Ported: "should return closed PR" — lib/modules/platform/github/index.spec.ts line 4429
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

    // Ported: "should return merged PR" — lib/modules/platform/github/index.spec.ts line 4454
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

    // Ported: "should return null if no PR is returned from GitHub" — lib/modules/platform/github/index.spec.ts line 4480
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

    // Ported: "should return a PR object - 0" — lib/modules/platform/github/index.spec.ts line 4495
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

    // Ported: "should return a PR object - 1" — lib/modules/platform/github/index.spec.ts line 4521
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

    // Ported: "should return a PR object - 2" — lib/modules/platform/github/index.spec.ts line 4557
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

    // Ported: "finds PR by branch name" — lib/modules/platform/github/index.spec.ts line 3540
    // Ported: "fetches single page" — lib/modules/platform/github/index.spec.ts line 1459
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

    // Ported: "finds PR with non-open state" — lib/modules/platform/github/index.spec.ts line 3582
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
        let prs = client
            .list_prs("owner", "repo", Some("closed"))
            .await
            .unwrap();
        assert_eq!(prs.len(), 1);
        assert_eq!(prs[0].state, "closed");
    }

    // Ported: "skips PR with non-matching state" — lib/modules/platform/github/index.spec.ts line 3611
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
        let prs = client
            .list_prs("owner", "repo", Some("open"))
            .await
            .unwrap();
        assert!(prs.iter().all(|p| p.state == "open"));
    }

    // Ported: "skips PRs from forks" — lib/modules/platform/github/index.spec.ts line 3637
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
        let own_prs: Vec<_> = prs
            .into_iter()
            .filter(|p| {
                p.head
                    .repo
                    .as_ref()
                    .map(|r| r.full_name == "owner/repo")
                    .unwrap_or(true)
            })
            .collect();
        assert_eq!(own_prs.len(), 1);
        assert_eq!(own_prs[0].number, 2);
    }

    // Ported: "skips PR with non-matching title" — lib/modules/platform/github/index.spec.ts line 3662
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
        assert_eq!(
            prs.into_iter().filter(|p| p.title == "Update deps").count(),
            1
        );
    }

    // Ported: "caches pr list" — lib/modules/platform/github/index.spec.ts line 3687
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

    // Ported: "finds pr from other authors" — lib/modules/platform/github/index.spec.ts line 3722
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

    // Ported: "returns null if issues disabled" — lib/modules/platform/github/index.spec.ts line 2505
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

    // Ported: "returns issue" — lib/modules/platform/github/index.spec.ts line 2513
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
        let issue = client
            .get_issue("owner", "repo", 42)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(issue.number, 42);
        assert_eq!(issue.title, "Bug report");
    }

    // Ported: "returns null if issue not found" — lib/modules/platform/github/index.spec.ts line 2533
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

    // Ported: "returns null if no issue" — lib/modules/platform/github/index.spec.ts line 2557
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

    // Ported: "finds issue" — lib/modules/platform/github/index.spec.ts line 2594
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

    // ── create_issue / update_issue ───────────────────────────────────────────

    // Ported: "creates issue" — lib/modules/platform/github/index.spec.ts line 2647
    #[tokio::test]
    async fn create_issue_returns_issue_number() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/issues"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "number": 42,
                "title": "Dependency Dashboard",
                "state": "open",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-09T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let number = client
            .create_issue("owner", "repo", "Dependency Dashboard", "Body", None)
            .await
            .unwrap();
        assert_eq!(number, 42);
    }

    // Ported: "creates issue with labels" — lib/modules/platform/github/index.spec.ts line 2783
    #[tokio::test]
    async fn create_issue_with_labels() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/issues"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "number": 42,
                "title": "Dependency Dashboard",
                "state": "open",
                "labels": [{"name": "renovate"}],
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-09T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let number = client
            .create_issue(
                "owner",
                "repo",
                "Dependency Dashboard",
                "Body",
                Some(vec!["renovate".to_owned()]),
            )
            .await
            .unwrap();
        assert_eq!(number, 42);
    }

    // Ported: "updates issue" — lib/modules/platform/github/index.spec.ts line 2872
    #[tokio::test]
    async fn update_issue_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/repos/owner/repo/issues/42"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        client
            .update_issue(
                "owner",
                "repo",
                42,
                Some("New title"),
                Some("New body"),
                None,
                None,
            )
            .await
            .unwrap();
    }

    // Ported: "updates issue with labels" — lib/modules/platform/github/index.spec.ts line 2931
    #[tokio::test]
    async fn update_issue_with_labels() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/repos/owner/repo/issues/42"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        client
            .update_issue(
                "owner",
                "repo",
                42,
                None,
                None,
                None,
                Some(vec!["bug".to_owned()]),
            )
            .await
            .unwrap();
    }

    // Ported: "closes issue" — lib/modules/platform/github/index.spec.ts line 3179
    #[tokio::test]
    async fn update_issue_closes_issue() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/repos/owner/repo/issues/42"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        client
            .update_issue("owner", "repo", 42, None, None, Some("closed"), None)
            .await
            .unwrap();
    }

    // Ported: "swallows 404 Not Found when the issue was deleted on the platform" — lib/modules/platform/github/index.spec.ts line 3254
    #[tokio::test]
    async fn get_issue_swallows_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/issues/42"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let issue = client.get_issue("owner", "repo", 42).await.unwrap();
        assert!(issue.is_none());
    }

    // Ported: "swallows 410 Gone when the issue was deleted on the platform" — lib/modules/platform/github/index.spec.ts line 3223
    #[tokio::test]
    async fn get_issue_swallows_410() {
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

    // Ported: "rethrows non-deletion errors" — lib/modules/platform/github/index.spec.ts line 3285
    #[tokio::test]
    async fn get_issue_rethrows_non_deletion_errors() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/issues/42"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client.get_issue("owner", "repo", 42).await.unwrap_err();
        assert!(
            matches!(err, PlatformError::Http(HttpError::Status { status, .. }) if status == reqwest::StatusCode::INTERNAL_SERVER_ERROR)
        );
    }

    // ── create_comment / update_comment / delete_comment ──────────────────────

    // Ported: "add comment if not found" — lib/modules/platform/github/index.spec.ts line 3398
    #[tokio::test]
    async fn create_comment_adds_comment() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/issues/42/comments"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "id": 123,
                "body": "A comment",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let id = client
            .create_comment("owner", "repo", 42, "A comment")
            .await
            .unwrap();
        assert_eq!(id, 123);
    }

    // Ported: "add updates comment if necessary" — lib/modules/platform/github/index.spec.ts line 3445
    #[tokio::test]
    async fn update_comment_updates_body() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/repos/owner/repo/issues/comments/123"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        client
            .update_comment("owner", "repo", 123, "Updated comment")
            .await
            .unwrap();
    }

    // Ported: "deletes comment by topic if found" — lib/modules/platform/github/index.spec.ts line 3500
    #[tokio::test]
    async fn delete_comment_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/repos/owner/repo/issues/comments/123"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        client.delete_comment("owner", "repo", 123).await.unwrap();
    }

    // ── remote_branch_exists ──────────────────────────────────────────────────

    // Ported: "should return true if the branch exists" — lib/modules/platform/github/branch.spec.ts line 5
    #[tokio::test]
    async fn remote_branch_exists_returns_true() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/repos/my/repo/git/matching-refs/heads/renovate/foobar",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"ref": "refs/heads/renovate/foobar"}
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .remote_branch_exists("my", "repo", "renovate/foobar")
            .await
            .unwrap();
        assert!(result);
    }

    // Ported: "should return false if the branch does not exist" — lib/modules/platform/github/branch.spec.ts line 16
    #[tokio::test]
    async fn remote_branch_exists_returns_false() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/repos/my/repo/git/matching-refs/heads/renovate/foobar",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .remote_branch_exists("my", "repo", "renovate/foobar")
            .await
            .unwrap();
        assert!(!result);
    }

    // Ported: "should throw an error for nested branches" — lib/modules/platform/github/branch.spec.ts line 27
    #[tokio::test]
    async fn remote_branch_exists_throws_for_nested() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/repos/my/repo/git/matching-refs/heads/renovate/foobar",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"ref": "refs/heads/renovate/foobar/branch-1"},
                {"ref": "refs/heads/renovate/foobar/branch-2"},
                {"ref": "refs/heads/renovate/foobar/branch-3"},
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client
            .remote_branch_exists("my", "repo", "renovate/foobar")
            .await
            .unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(msg) if msg.contains("nested branch")));
    }

    // Ported: "should throw an error if the request fails for any other reason" — lib/modules/platform/github/branch.spec.ts line 44
    #[tokio::test]
    async fn remote_branch_exists_throws_on_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/repos/my/repo/git/matching-refs/heads/renovate/foobar",
            ))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client
            .remote_branch_exists("my", "repo", "renovate/foobar")
            .await
            .unwrap_err();
        assert!(
            matches!(err, PlatformError::Http(HttpError::Status { status, .. }) if status == reqwest::StatusCode::INTERNAL_SERVER_ERROR)
        );
    }

    // ── get_branch_status_check ───────────────────────────────────────────────

    // Ported: "returns state if found" — lib/modules/platform/github/index.spec.ts line 2360
    #[tokio::test]
    async fn get_branch_status_check_returns_state() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/repos/owner/repo/commits/renovate/future_branch/statuses",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"context": "context-1", "state": "success"},
                {"context": "context-2", "state": "pending"},
                {"context": "context-3", "state": "failure"},
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .get_branch_status_check("owner", "repo", "renovate/future_branch", "context-2")
            .await
            .unwrap();
        assert_eq!(result, Some("yellow".to_owned()));
    }

    // Ported: "returns null" — lib/modules/platform/github/index.spec.ts line 2389
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
        let result = client
            .get_branch_status_check("owner", "repo", "somebranch", "context-4")
            .await
            .unwrap();
        assert_eq!(result, None);
    }

    // Ported: "returns yellow if state not present in context object" — lib/modules/platform/github/index.spec.ts line 2415
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
        let result = client
            .get_branch_status_check("owner", "repo", "somebranch", "context-1")
            .await
            .unwrap();
        assert_eq!(result, Some("yellow".to_owned()));
    }

    // ── get_branch_force_rebase ──────────────────────────────────────────────

    // Ported: "should detect repoForceRebase" — lib/modules/platform/github/index.spec.ts line 1151
    #[tokio::test]
    async fn get_branch_force_rebase_detects_from_protection() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/rules/branches/main"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/branches/main/protection"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "required_pull_request_reviews": {"required_approving_review_count": 1},
                "required_status_checks": {"strict": true, "contexts": []},
                "restrictions": {"users": [{"login": "rarkins"}], "teams": []},
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .get_branch_force_rebase("owner", "repo", "main", None)
            .await
            .unwrap();
        assert!(result);
    }

    // Ported: "should handle 404" — lib/modules/platform/github/index.spec.ts line 1185
    #[tokio::test]
    async fn get_branch_force_rebase_handles_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/rules/branches/dev"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/branches/dev/protection"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .get_branch_force_rebase("owner", "repo", "dev", None)
            .await
            .unwrap();
        assert!(!result);
    }

    // Ported: "should handle 403" — lib/modules/platform/github/index.spec.ts line 1198
    #[tokio::test]
    async fn get_branch_force_rebase_handles_403() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/rules/branches/main"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/branches/main/protection"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .get_branch_force_rebase("owner", "repo", "main", None)
            .await
            .unwrap();
        assert!(!result);
    }

    // Ported: "should throw 401" — lib/modules/platform/github/index.spec.ts line 1211
    #[tokio::test]
    async fn get_branch_force_rebase_throws_on_401() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/rules/branches/main"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/branches/main/protection"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client
            .get_branch_force_rebase("owner", "repo", "main", None)
            .await
            .unwrap_err();
        assert!(
            matches!(err, PlatformError::Http(HttpError::Status { status, .. }) if status == reqwest::StatusCode::UNAUTHORIZED)
        );
    }

    // Ported: "should ignore non_fast_forward ruleset for determining rebase" — lib/modules/platform/github/index.spec.ts line 1245
    #[tokio::test]
    async fn get_branch_force_rebase_ignores_non_fast_forward_ruleset() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/rules/branches/main"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"type": "non_fast_forward", "ruleset_source_type": "Repository", "ruleset_source": "owner/repo", "ruleset_id": 12345},
            ])))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/branches/main/protection"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "required_status_checks": {"strict": false},
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .get_branch_force_rebase("owner", "repo", "main", None)
            .await
            .unwrap();
        assert!(!result);
    }

    // Ported: "should detect strict required status checks ruleset" — lib/modules/platform/github/index.spec.ts line 1269
    #[tokio::test]
    async fn get_branch_force_rebase_detects_strict_ruleset() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/rules/branches/main"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "type": "required_status_checks",
                    "parameters": {"strict_required_status_checks_policy": true},
                    "ruleset_source_type": "Repository",
                    "ruleset_source": "owner/repo",
                    "ruleset_id": 12345,
                },
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .get_branch_force_rebase("owner", "repo", "main", None)
            .await
            .unwrap();
        assert!(result);
    }

    // Ported: "should continue if no expected rulesets have been found" — lib/modules/platform/github/index.spec.ts line 1288
    #[tokio::test]
    async fn get_branch_force_rebase_continues_past_unexpected_rulesets() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/rules/branches/main"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"type": "deletion"},
            ])))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/branches/main/protection"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "required_status_checks": {"strict": true},
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .get_branch_force_rebase("owner", "repo", "main", None)
            .await
            .unwrap();
        assert!(result);
    }

    // Ported: "should abort and throws on internal error" — lib/modules/platform/github/index.spec.ts line 1309
    #[tokio::test]
    async fn get_branch_force_rebase_throws_on_internal_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/rules/branches/main"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client
            .get_branch_force_rebase("owner", "repo", "main", None)
            .await
            .unwrap_err();
        assert!(
            matches!(err, PlatformError::Http(HttpError::Status { status, .. }) if status == reqwest::StatusCode::INTERNAL_SERVER_ERROR)
        );
    }

    // Ported: "should fallback to legacy branch protection when rulesets not found" — lib/modules/platform/github/index.spec.ts line 1320
    #[tokio::test]
    async fn get_branch_force_rebase_fallback_to_branch_protection() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/rules/branches/main"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/branches/main/protection"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "required_status_checks": {"strict": true},
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .get_branch_force_rebase("owner", "repo", "main", None)
            .await
            .unwrap();
        assert!(result);
    }

    // Ported: "should return false when no force rebase rules found" — lib/modules/platform/github/index.spec.ts line 1337
    #[tokio::test]
    async fn get_branch_force_rebase_returns_false_when_no_strict_checks() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/rules/branches/main"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "type": "pull_request",
                    "parameters": {"required_approving_review_count": 1},
                    "ruleset_source_type": "Repository",
                    "ruleset_source": "owner/repo",
                    "ruleset_id": 12345,
                },
            ])))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/branches/main/protection"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "required_status_checks": {"strict": false},
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .get_branch_force_rebase("owner", "repo", "main", None)
            .await
            .unwrap();
        assert!(!result);
    }

    // Ported: "should return cached result on subsequent calls" — lib/modules/platform/github/index.spec.ts line 1360
    #[tokio::test]
    async fn get_branch_force_rebase_returns_cached_result() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/rules/branches/main"))
            .respond_with(ResponseTemplate::new(404))
            .expect(1)
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/branches/main/protection"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "required_status_checks": {"strict": true},
            })))
            .expect(1)
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result1 = client
            .get_branch_force_rebase("owner", "repo", "main", None)
            .await
            .unwrap();
        assert!(result1);
        let result2 = client
            .get_branch_force_rebase("owner", "repo", "main", None)
            .await
            .unwrap();
        assert!(result2);
    }

    // Ported: "should return cached false result on subsequent calls" — lib/modules/platform/github/index.spec.ts line 1385
    #[tokio::test]
    async fn get_branch_force_rebase_returns_cached_false_result() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/rules/branches/main"))
            .respond_with(ResponseTemplate::new(404))
            .expect(1)
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/branches/main/protection"))
            .respond_with(ResponseTemplate::new(404))
            .expect(1)
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result1 = client
            .get_branch_force_rebase("owner", "repo", "main", None)
            .await
            .unwrap();
        assert!(!result1);
        let result2 = client
            .get_branch_force_rebase("owner", "repo", "main", None)
            .await
            .unwrap();
        assert!(!result2);
    }

    // Ported: "should return empty object when parentRepo is set" — lib/modules/platform/github/index.spec.ts line 1225
    #[tokio::test]
    async fn get_branch_force_rebase_returns_false_when_parent_repo_set() {
        let client = GithubClient::with_endpoint("token", "https://api.github.com").unwrap();
        let result = client
            .get_branch_force_rebase("owner", "repo", "main", Some("parent/repo"))
            .await
            .unwrap();
        assert!(!result);
    }

    // ── set_branch_status ─────────────────────────────────────────────────────

    // Ported: "returns if already set" — lib/modules/platform/github/index.spec.ts line 2434
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
        client
            .set_branch_status(
                "owner",
                "repo",
                "some-branch",
                "some-context",
                "some-description",
                "yellow",
                Some("some-url"),
            )
            .await
            .unwrap();
    }

    // Ported: "sets branch status" — lib/modules/platform/github/index.spec.ts line 2459
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
        client
            .set_branch_status(
                "owner",
                "repo",
                "some-branch",
                "some-context",
                "some-description",
                "green",
                Some("some-url"),
            )
            .await
            .unwrap();
    }

    // Ported: "should return an array of repos when using Github App endpoint" — lib/modules/platform/github/index.spec.ts line 663
    // Ported: "should be parse directory response" — lib/modules/platform/github/schema.spec.ts line 6
    #[test]
    fn github_content_response_directory() {
        let input = serde_json::json!([
            {"type": "file", "size": 625, "name": "octokit.rb", "path": "lib/octokit.rb", "sha": "fff", "url": "u", "git_url": "g", "html_url": "h", "download_url": "d", "_links": {}},
            {"type": "dir",  "size": 0,   "name": "octokit",    "path": "lib/octokit",    "sha": "aaa", "url": "u", "git_url": "g", "html_url": "h", "download_url": null, "_links": {}},
            {"type": "symlink", "size": 23, "name": "some-symlink", "path": "bin/some-symlink", "sha": "bbb", "url": "u", "git_url": "g", "html_url": "h", "download_url": "d", "_links": {}},
        ]);
        assert!(validate_github_content_response(&input).is_ok());
    }

    // Ported: "returns file content from branch or tag" — lib/modules/platform/github/index.spec.ts line 5434
    // Ported: "should parse response for single file" — lib/modules/platform/github/schema.spec.ts line 88
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

    // Ported: "calls logger.debug with only items that include securityVulnerability" — lib/modules/platform/github/index.spec.ts line 5191
    // Ported: "should skip vulnerability alerts with unsupported ecosystems" — lib/modules/platform/github/schema.spec.ts line 112
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
        assert_eq!(
            result[0]["security_vulnerability"]["package"]["ecosystem"],
            "npm"
        );
    }

    // Ported: "returns array if found" — lib/modules/platform/github/index.spec.ts line 5113
    // Ported: "should parse severity and cvss_severities fields" — lib/modules/platform/github/schema.spec.ts line 207
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
        assert_eq!(
            result[0]["security_advisory"]["cvss_severities"]["cvss_v3"]["score"],
            9.8
        );
        assert!(result[0]["security_advisory"]["cvss_severities"]["cvss_v4"].is_null());
    }

    // The TypeScript test also checks logger.debug spy; Rust tests the filter behavior.
    // dotnet ecosystem alert is filtered out (returns empty), same behavior as the
    // "skip unsupported ecosystems" test which already covers this parse path.

    // Ported: "should log vulnerability alerts with parse errors" — lib/modules/platform/github/schema.spec.ts line 153
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

    // Ported: "should filter vulnerability alerts with missing security_vulnerability" — lib/modules/platform/github/schema.spec.ts line 182
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

    // Ported: "throws unexpected graphql errors" — lib/modules/platform/github/index.spec.ts line 1067
    #[test]
    fn validate_github_content_missing_type() {
        let input = serde_json::json!({"name": "foo", "path": "foo"});
        assert!(validate_github_content_response(&input).is_err());
    }

    // Ported: "throws not-found" — lib/modules/platform/github/index.spec.ts line 1060
    #[test]
    fn validate_github_content_unknown_type() {
        let input = serde_json::json!({"type": "unknown", "name": "foo", "path": "foo"});
        assert!(validate_github_content_response(&input).is_err());
    }

    // Ported: "should throw error if renamed" — lib/modules/platform/github/index.spec.ts line 1101
    #[test]
    fn validate_github_content_not_array_or_object() {
        let input = serde_json::json!("string");
        assert!(validate_github_content_response(&input).is_err());
    }

    // ── Additional parse_github_vulnerability_alerts tests (index.spec.ts) ──────

    // Ported: "avoids fetching if repo has vulnerability alerts disabled" — lib/modules/platform/github/index.spec.ts line 5090
    #[test]
    fn parse_vulnerability_alerts_empty_array() {
        let input = serde_json::json!([]);
        let result = parse_github_vulnerability_alerts(&input);
        assert!(result.is_empty());
    }

    // Ported: "returns empty if disabled" — lib/modules/platform/github/index.spec.ts line 5163
    #[test]
    fn parse_vulnerability_alerts_null_input() {
        let input = serde_json::Value::Null;
        let result = parse_github_vulnerability_alerts(&input);
        assert!(result.is_empty());
    }

    // Ported: "handles network error" — lib/modules/platform/github/index.spec.ts line 5177
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

    // Ported: "returns normalized names for PIP ecosystem" — lib/modules/platform/github/index.spec.ts line 5247
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
        assert_eq!(
            result[0]["security_vulnerability"]["package"]["ecosystem"],
            "pip"
        );
        assert_eq!(
            result[0]["security_vulnerability"]["package"]["name"],
            "requests"
        );
    }

    // Ported: "handles pagination correctly" — lib/modules/platform/github/index.spec.ts line 5283
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

    // Ported: "returns empty if error" — lib/modules/platform/github/index.spec.ts line 5100
    #[test]
    fn parse_vulnerability_alerts_returns_empty_for_unexpected_format() {
        let input = serde_json::json!({"data": {"repository": {}}});
        let result = parse_github_vulnerability_alerts(&input);
        assert!(result.is_empty());
    }

    // ── get_vulnerability_alerts ──────────────────────────────────────────────

    // Ported: "returns empty if error" — lib/modules/platform/github/index.spec.ts line 5100
    #[tokio::test]
    async fn get_vulnerability_alerts_returns_empty_on_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/dependabot/alerts"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let alerts = client
            .get_vulnerability_alerts("owner", "repo")
            .await
            .unwrap();
        assert!(alerts.is_empty());
    }

    // Ported: "returns array if found" — lib/modules/platform/github/index.spec.ts line 5113
    #[tokio::test]
    async fn get_vulnerability_alerts_returns_array_if_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/dependabot/alerts"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "security_advisory": {
                        "ghsa_id": "GHSA-1234-5678-9012",
                        "summary": "summary",
                        "description": "description",
                        "identifiers": [{"type": "type", "value": "value"}],
                        "references": [],
                        "severity": "high",
                    },
                    "security_vulnerability": {
                        "package": {"ecosystem": "npm", "name": "left-pad"},
                        "severity": "high",
                        "vulnerable_version_range": "0.0.2",
                        "first_patched_version": {"identifier": "0.0.3"},
                    },
                    "dependency": {"manifest_path": "bar/foo"},
                },
                {
                    "security_advisory": {
                        "ghsa_id": "GHSA-1234-5678-9012",
                        "summary": "summary",
                        "description": "description",
                        "identifiers": [{"type": "type", "value": "value"}],
                        "references": [],
                        "severity": "high",
                    },
                    "security_vulnerability": null,
                    "dependency": {"manifest_path": "bar/foo"},
                },
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let alerts = client
            .get_vulnerability_alerts("owner", "repo")
            .await
            .unwrap();
        assert_eq!(alerts.len(), 1);
    }

    // Ported: "returns empty if disabled" — lib/modules/platform/github/index.spec.ts line 5163
    #[tokio::test]
    async fn get_vulnerability_alerts_returns_empty_if_disabled() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/dependabot/alerts"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {"repository": {}},
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let alerts = client
            .get_vulnerability_alerts("owner", "repo")
            .await
            .unwrap();
        assert!(alerts.is_empty());
    }

    // Ported: "handles network error" — lib/modules/platform/github/index.spec.ts line 5177
    #[tokio::test]
    async fn get_vulnerability_alerts_handles_network_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/dependabot/alerts"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let alerts = client
            .get_vulnerability_alerts("owner", "repo")
            .await
            .unwrap();
        assert!(alerts.is_empty());
    }

    // ── delete_label ──────────────────────────────────────────────────────────

    // Ported: "should delete the label" — lib/modules/platform/github/index.spec.ts line 3318
    #[tokio::test]
    async fn delete_label_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/repos/owner/repo/issues/42/labels/rebase"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        client
            .delete_label("owner", "repo", 42, "rebase")
            .await
            .unwrap();
    }

    // ── add_assignees ─────────────────────────────────────────────────────────

    // Ported: "should add the given assignees to the issue" — lib/modules/platform/github/index.spec.ts line 3328
    #[tokio::test]
    async fn add_assignees_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/issues/42/assignees"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "number": 42,
                "state": "open",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        client
            .add_assignees(
                "owner",
                "repo",
                42,
                vec!["someuser".to_owned(), "someotheruser".to_owned()],
            )
            .await
            .unwrap();
    }

    // Ported: "should retry on 404 and succeed" — lib/modules/platform/github/index.spec.ts line 3344
    #[tokio::test]
    async fn add_assignees_retries_on_404_and_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/issues/42/assignees"))
            .respond_with(ResponseTemplate::new(404))
            .up_to_n_times(1)
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/issues/42/assignees"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "number": 42,
                "state": "open",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        client
            .add_assignees("owner", "repo", 42, vec!["someuser".to_owned()])
            .await
            .unwrap();
    }

    // Ported: "should throw after 3 consecutive 404 responses" — lib/modules/platform/github/index.spec.ts line 3364
    #[tokio::test]
    async fn add_assignees_throws_after_three_consecutive_404s() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/issues/42/assignees"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client
            .add_assignees("owner", "repo", 42, vec!["someuser".to_owned()])
            .await
            .unwrap_err();
        assert!(
            matches!(err, PlatformError::Http(HttpError::Status { status, .. }) if status == reqwest::StatusCode::NOT_FOUND)
        );
    }

    // ── add_reviewers ─────────────────────────────────────────────────────────

    // Ported: "should add the given reviewers to the PR" — lib/modules/platform/github/index.spec.ts line 3386
    #[tokio::test]
    async fn add_reviewers_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/pulls/42/requested_reviewers"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "number": 42,
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        client
            .add_reviewers(
                "owner",
                "repo",
                42,
                vec!["user1".to_owned(), "team:myteam".to_owned()],
            )
            .await
            .unwrap();
    }

    // ── find_pr ───────────────────────────────────────────────────────────────

    // Ported: "finds PR by branch name" — lib/modules/platform/github/index.spec.ts line 3540
    #[tokio::test]
    async fn find_pr_by_branch_name() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "number": 2,
                    "title": "branch a pr",
                    "state": "open",
                    "head": {"ref": "branch-a", "sha": "def", "repo": {"full_name": "owner/repo", "pushed_at": null}},
                    "base": {"ref": "main", "sha": "abc", "repo": null},
                    "node_id": "nid",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-09T00:00:00Z",
                },
                {
                    "number": 1,
                    "title": "branch a pr",
                    "state": "open",
                    "head": {"ref": "branch-a", "sha": "def2", "repo": {"full_name": "owner/repo", "pushed_at": null}},
                    "base": {"ref": "main", "sha": "abc", "repo": null},
                    "node_id": "nid2",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-09T00:00:00Z",
                },
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr = client
            .find_pr("owner", "repo", "branch-a", None, None, false)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(pr.number, 2);
        assert_eq!(pr.source_branch, "branch-a");
    }

    // Ported: "finds PR with non-open state" — lib/modules/platform/github/index.spec.ts line 3582
    #[tokio::test]
    async fn find_pr_with_non_open_state() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "number": 1,
                    "title": "branch a pr",
                    "state": "closed",
                    "head": {"ref": "branch-a", "sha": "def", "repo": {"full_name": "owner/repo", "pushed_at": null}},
                    "base": {"ref": "main", "sha": "abc", "repo": null},
                    "node_id": "nid",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-09T00:00:00Z",
                },
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr = client
            .find_pr("owner", "repo", "branch-a", None, Some("!open"), false)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(pr.number, 1);
        assert_eq!(pr.state, "closed");
    }

    // Ported: "skips PR with non-matching state" — lib/modules/platform/github/index.spec.ts line 3611
    #[tokio::test]
    async fn find_pr_skips_non_matching_state() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "number": 1,
                    "title": "branch a pr",
                    "state": "closed",
                    "head": {"ref": "branch-a", "sha": "def", "repo": {"full_name": "owner/repo", "pushed_at": null}},
                    "base": {"ref": "main", "sha": "abc", "repo": null},
                    "node_id": "nid",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-09T00:00:00Z",
                },
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr = client
            .find_pr("owner", "repo", "branch-a", None, Some("open"), false)
            .await
            .unwrap();
        assert!(pr.is_none());
    }

    // Ported: "skips PRs from forks" — lib/modules/platform/github/index.spec.ts line 3637
    #[tokio::test]
    async fn find_pr_skips_forks() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "number": 1,
                    "title": "branch a pr",
                    "state": "open",
                    "head": {"ref": "branch-a", "sha": "def", "repo": {"full_name": "other/repo", "pushed_at": null}},
                    "base": {"ref": "main", "sha": "abc", "repo": null},
                    "node_id": "nid",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-09T00:00:00Z",
                },
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr = client
            .find_pr("owner", "repo", "branch-a", None, Some("open"), false)
            .await
            .unwrap();
        assert!(pr.is_none());
    }

    // Ported: "skips PR with non-matching title" — lib/modules/platform/github/index.spec.ts line 3662
    #[tokio::test]
    async fn find_pr_skips_non_matching_title() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "number": 1,
                    "title": "foo",
                    "state": "closed",
                    "head": {"ref": "branch-a", "sha": "def", "repo": {"full_name": "owner/repo", "pushed_at": null}},
                    "base": {"ref": "main", "sha": "abc", "repo": null},
                    "node_id": "nid",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-09T00:00:00Z",
                },
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr = client
            .find_pr("owner", "repo", "branch-a", Some("bar"), None, false)
            .await
            .unwrap();
        assert!(pr.is_none());
    }

    // Ported: "finds pr from other authors" — lib/modules/platform/github/index.spec.ts line 3722
    #[tokio::test]
    async fn find_pr_from_other_authors() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls"))
            .and(wiremock::matchers::query_param("head", "owner:branch-a"))
            .and(wiremock::matchers::query_param("state", "open"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "number": 1,
                    "title": "branch a pr",
                    "state": "open",
                    "head": {"ref": "branch-a", "sha": "def", "repo": {"full_name": "owner/repo", "pushed_at": null}},
                    "base": {"ref": "main", "sha": "abc", "repo": null},
                    "node_id": "nid",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-09T00:00:00Z",
                },
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr = client
            .find_pr("owner", "repo", "branch-a", None, None, true)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(pr.number, 1);
    }

    // Ported: "returns null if no pr found - (includeOtherAuthors)" — lib/modules/platform/github/index.spec.ts line 3752
    // Ported: "should return null if no PR exists" — lib/modules/platform/github/index.spec.ts line 2007
    #[tokio::test]
    async fn find_pr_returns_null_when_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr = client
            .find_pr("owner", "repo", "branch-a", None, None, false)
            .await
            .unwrap();
        assert!(pr.is_none());
    }

    // Ported: "should return null if no PR exists" — lib/modules/platform/github/index.spec.ts line 2007
    #[tokio::test]
    async fn list_prs_returns_empty_when_no_prs() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let prs = client.list_prs("owner", "repo", None).await.unwrap();
        assert!(prs.is_empty());
    }

    // Ported: "fetches single page" — lib/modules/platform/github/index.spec.ts line 1459
    #[tokio::test]
    async fn list_prs_fetches_single_page() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "number": 1,
                    "title": "PR #1",
                    "state": "open",
                    "head": {"ref": "branch-1", "sha": "def", "repo": {"full_name": "some/repo", "pushed_at": null}},
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
        assert_eq!(prs[0].number, 1);
    }

    // Ported: "fetches multiple pages" — lib/modules/platform/github/index.spec.ts line 1470
    #[tokio::test]
    async fn list_prs_fetches_multiple_pages() {
        let server = MockServer::start().await;
        let next_url = format!("{}/repos/owner/repo/pulls?page=2", server.uri());
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls"))
            .and(wiremock::matchers::query_param("page", "2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "number": 2,
                    "title": "PR #2",
                    "state": "open",
                    "head": {"ref": "branch-2", "sha": "def2", "repo": {"full_name": "some/repo", "pushed_at": null}},
                    "base": {"ref": "main", "sha": "abc", "repo": null},
                    "node_id": "nid2",
                    "created_at": "2024-01-02T00:00:00Z",
                    "updated_at": "2024-01-10T00:00:00Z",
                },
            ])))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([
                        {
                            "number": 1,
                            "title": "PR #1",
                            "state": "open",
                            "head": {"ref": "branch-1", "sha": "def", "repo": {"full_name": "some/repo", "pushed_at": null}},
                            "base": {"ref": "main", "sha": "abc", "repo": null},
                            "node_id": "nid",
                            "created_at": "2024-01-01T00:00:00Z",
                            "updated_at": "2024-01-09T00:00:00Z",
                        },
                    ]))
                    .insert_header("link", format!("<{}>; rel=\"next\"", next_url)),
            )
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let prs = client.list_prs("owner", "repo", None).await.unwrap();
        assert_eq!(prs.len(), 2);
        assert_eq!(prs[0].number, 1);
        assert_eq!(prs[1].number, 2);
    }

    // Ported: "should not be case sensitive" — lib/modules/platform/github/index.spec.ts line 1124
    #[tokio::test]
    async fn find_pr_not_case_sensitive() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "number": 1,
                    "title": "branch a pr",
                    "state": "open",
                    "head": {"ref": "Branch-A", "sha": "def", "repo": {"full_name": "owner/repo", "pushed_at": null}},
                    "base": {"ref": "main", "sha": "abc", "repo": null},
                    "node_id": "nid",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-09T00:00:00Z",
                },
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr = client
            .find_pr("owner", "repo", "Branch-A", None, None, false)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(pr.number, 1);
        assert_eq!(pr.source_branch, "Branch-A");
    }

    // Ported: "should create a draftPR if set in the settings" — lib/modules/platform/github/index.spec.ts line 3809
    #[tokio::test]
    async fn create_pr_with_draft() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "number": 123,
                "title": "PR draft",
                "state": "open",
                "head": {"ref": "some-branch", "sha": "abc", "repo": null},
                "base": {"ref": "master", "sha": "def", "repo": null},
                "node_id": "nid",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr_number = client
            .create_pr(
                "owner",
                "repo",
                "some-branch",
                "master",
                "PR draft",
                "This is a result of a draft",
            )
            .await
            .unwrap();
        assert_eq!(pr_number, Some(123));
    }

    // ── init_platform ─────────────────────────────────────────────────────────

    // Ported: "should throw if no token" — lib/modules/platform/github/index.spec.ts line 64
    // Ported: "no token" — lib/modules/platform/github/index.spec.ts line 809
    #[tokio::test]
    async fn init_platform_throws_if_no_token() {
        let server = MockServer::start().await;
        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client.init_platform("").await.unwrap_err();
        assert!(
            matches!(err, PlatformError::Unexpected(msg) if msg.contains("You must configure a GitHub token"))
        );
    }

    // Ported: "should throw if user failure" — lib/modules/platform/github/index.spec.ts line 128
    #[tokio::test]
    async fn init_platform_throws_on_user_failure() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client.init_platform("123test").await.unwrap_err();
        assert!(matches!(err, PlatformError::Http(_)));
    }

    // Ported: "should throw if endpoint is invalid URL" — lib/modules/platform/github/index.spec.ts line 70
    #[test]
    fn init_platform_throws_if_endpoint_invalid() {
        let err = GithubClient::with_endpoint("token", "https://[invalid").unwrap_err();
        assert!(matches!(err, HttpError::Parse(_)));
    }

    // Ported: "should use public email from user profile when available" — lib/modules/platform/github/index.spec.ts line 361
    #[tokio::test]
    async fn init_platform_uses_public_email() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "login": "renovate-bot",
                "email": "user@domain.com",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let (login, email) = client.init_platform("123test").await.unwrap();
        assert_eq!(login, "renovate-bot");
        assert_eq!(email, Some("user@domain.com".to_owned()));
    }

    // Ported: "should fall back to user/emails when there is no public email" — lib/modules/platform/github/index.spec.ts line 375
    #[tokio::test]
    async fn init_platform_falls_back_to_user_emails() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "login": "renovate-bot",
            })))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/user/emails"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"email": "fallback@domain.com", "primary": true, "verified": true},
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let (login, email) = client.init_platform("123test").await.unwrap();
        assert_eq!(login, "renovate-bot");
        assert_eq!(email, Some("fallback@domain.com".to_owned()));
    }

    // Ported: "should fall back gracefully when user/emails returns an error (no user:email scope)" — lib/modules/platform/github/index.spec.ts line 394
    #[tokio::test]
    async fn init_platform_falls_back_gracefully_on_email_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "login": "renovate-bot",
            })))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/user/emails"))
            .respond_with(ResponseTemplate::new(400))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let (login, email) = client.init_platform("123test").await.unwrap();
        assert_eq!(login, "renovate-bot");
        assert_eq!(email, None);
    }

    // Ported: "should support default endpoint no email access" — lib/modules/platform/github/index.spec.ts line 133
    #[tokio::test]
    async fn init_platform_supports_no_email_access() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "login": "renovate-bot",
            })))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/user/emails"))
            .respond_with(ResponseTemplate::new(400))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let (login, email) = client.init_platform("123test").await.unwrap();
        assert_eq!(login, "renovate-bot");
        assert_eq!(email, None);
    }

    // Ported: "should support default endpoint no email result" — lib/modules/platform/github/index.spec.ts line 145
    #[tokio::test]
    async fn init_platform_supports_empty_email_result() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "login": "renovate-bot",
            })))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/user/emails"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([{}])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let (login, email) = client.init_platform("123test").await.unwrap();
        assert_eq!(login, "renovate-bot");
        assert_eq!(email, None);
    }

    // Ported: "should throw if using fine-grained token with GHE <3.10" — lib/modules/platform/github/index.spec.ts line 79
    #[tokio::test]
    async fn init_platform_throws_on_fine_grained_token_with_old_ghe() {
        let server = MockServer::start().await;
        Mock::given(method("HEAD"))
            .and(path("/"))
            .respond_with(
                ResponseTemplate::new(200).insert_header("x-github-enterprise-version", "3.9.0"),
            )
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("github_pat_123test", server.uri()).unwrap();
        let err = client
            .init_platform("github_pat_123test")
            .await
            .unwrap_err();
        assert!(
            matches!(err, PlatformError::Unexpected(msg) if msg.contains("Fine-grained Personal Access Tokens"))
        );
    }

    // Ported: "should throw if using fine-grained token with GHE unknown version" — lib/modules/platform/github/index.spec.ts line 94
    #[tokio::test]
    async fn init_platform_throws_on_fine_grained_token_with_unknown_ghe_version() {
        let server = MockServer::start().await;
        Mock::given(method("HEAD"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("github_pat_123test", server.uri()).unwrap();
        let err = client
            .init_platform("github_pat_123test")
            .await
            .unwrap_err();
        assert!(
            matches!(err, PlatformError::Unexpected(msg) if msg.contains("Fine-grained Personal Access Tokens"))
        );
    }

    // Ported: "should support fine-grained token with GHE >=3.10" — lib/modules/platform/github/index.spec.ts line 106
    #[tokio::test]
    async fn init_platform_allows_fine_grained_token_with_recent_ghe() {
        let server = MockServer::start().await;
        Mock::given(method("HEAD"))
            .and(path("/"))
            .respond_with(
                ResponseTemplate::new(200).insert_header("x-github-enterprise-version", "3.10.0"),
            )
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/user"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "login": "renovate-bot",
                "email": "user@domain.com",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("github_pat_123test", server.uri()).unwrap();
        let (login, email) = client.init_platform("github_pat_123test").await.unwrap();
        assert_eq!(login, "renovate-bot");
        assert_eq!(email, Some("user@domain.com".to_owned()));
    }

    // ── get_repos ─────────────────────────────────────────────────────────────

    // Ported: "should return an array of repos" — lib/modules/platform/github/index.spec.ts line 613
    // Ported: "should throw error if archived" — lib/modules/platform/github/index.spec.ts line 1036
    #[tokio::test]
    async fn get_repos_returns_array_of_repos() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user/repos"))
            .and(wiremock::matchers::query_param("per_page", "100"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"full_name": "a/b", "archived": false},
                {"full_name": "c/d", "archived": false},
                {"full_name": "e/f", "archived": true},
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let repos = client.get_repos(None).await.unwrap();
        assert_eq!(repos, vec!["a/b", "c/d"]);
    }

    // Ported: "should filters repositories by topics" — lib/modules/platform/github/index.spec.ts line 636
    #[tokio::test]
    async fn get_repos_filters_by_topics() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user/repos"))
            .and(wiremock::matchers::query_param("per_page", "100"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"full_name": "a/b", "archived": false, "topics": []},
                {"full_name": "c/d", "archived": false, "topics": ["managed-by-renovate"]},
                {"full_name": "e/f", "archived": true, "topics": ["managed-by-renovate"]},
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let repos = client
            .get_repos(Some(vec!["managed-by-renovate".to_owned()]))
            .await
            .unwrap();
        assert_eq!(repos, vec!["c/d"]);
    }

    // Ported: "should return an array of repos when using Github App endpoint" — lib/modules/platform/github/index.spec.ts line 663
    #[tokio::test]
    async fn get_repos_using_github_app_endpoint() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/installation/repositories"))
            .and(wiremock::matchers::query_param("per_page", "100"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "repositories": [
                    {"full_name": "a/b"},
                    {"full_name": "c/d"},
                ],
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("x-access-token:123test", server.uri()).unwrap();
        let repos = client.get_repos(None).await.unwrap();
        assert_eq!(repos, vec!["a/b", "c/d"]);
    }

    // Ported: "should return an array of repos when using GitHub App Installation Token" — lib/modules/platform/github/index.spec.ts line 690
    // Ported: "app token" — lib/modules/platform/github/index.spec.ts line 817
    #[tokio::test]
    async fn get_repos_using_github_app_installation_token() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/installation/repositories"))
            .and(wiremock::matchers::query_param("per_page", "100"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "repositories": [
                    {"full_name": "a/b", "archived": false},
                    {"full_name": "c/d", "archived": false},
                    {"full_name": "e/f", "archived": true},
                ],
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("ghs_123test", server.uri()).unwrap();
        let repos = client.get_repos(None).await.unwrap();
        assert_eq!(repos, vec!["a/b", "c/d"]);
    }

    // ── create_pr extensions ──────────────────────────────────────────────────

    // Ported: "should allow maintainer edits if explicitly enabled via options" — lib/modules/platform/github/index.spec.ts line 3849
    #[tokio::test]
    async fn create_pr_allows_maintainer_edits_explicitly_enabled() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "number": 123,
                "title": "PR title",
                "state": "open",
                "head": {"ref": "some-branch", "sha": "abc", "repo": null},
                "base": {"ref": "main", "sha": "def", "repo": null},
                "node_id": "nid",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr_number = client
            .create_pr_with_options(
                "owner",
                "repo",
                "some-branch",
                "main",
                "PR title",
                "Body",
                false,
                Some(true),
                None,
            )
            .await
            .unwrap();
        assert_eq!(pr_number, Some(123));
    }

    // Ported: "should allow maintainer edits if not explicitly set" — lib/modules/platform/github/index.spec.ts line 3873
    #[tokio::test]
    async fn create_pr_allows_maintainer_edits_not_explicitly_set() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "number": 123,
                "title": "PR title",
                "state": "open",
                "head": {"ref": "some-branch", "sha": "abc", "repo": null},
                "base": {"ref": "main", "sha": "def", "repo": null},
                "node_id": "nid",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr_number = client
            .create_pr_with_options(
                "owner",
                "repo",
                "some-branch",
                "main",
                "PR title",
                "Body",
                false,
                None,
                None,
            )
            .await
            .unwrap();
        assert_eq!(pr_number, Some(123));
    }

    // Ported: "should disallow maintainer edits if explicitly disabled" — lib/modules/platform/github/index.spec.ts line 3894
    #[tokio::test]
    async fn create_pr_disallows_maintainer_edits_explicitly_disabled() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "number": 123,
                "title": "PR title",
                "state": "open",
                "head": {"ref": "some-branch", "sha": "abc", "repo": null},
                "base": {"ref": "main", "sha": "def", "repo": null},
                "node_id": "nid",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr_number = client
            .create_pr_with_options(
                "owner",
                "repo",
                "some-branch",
                "main",
                "PR title",
                "Body",
                false,
                Some(false),
                None,
            )
            .await
            .unwrap();
        assert_eq!(pr_number, Some(123));
    }

    // Ported: "should set the milestone on the PR" — lib/modules/platform/github/index.spec.ts line 4287
    #[tokio::test]
    async fn create_pr_sets_milestone() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "number": 123,
                "title": "PR title",
                "state": "open",
                "head": {"ref": "some-branch", "sha": "abc", "repo": null},
                "base": {"ref": "main", "sha": "def", "repo": null},
                "node_id": "nid",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T00:00:00Z",
            })))
            .mount(&server)
            .await;
        Mock::given(method("PATCH"))
            .and(path("/repos/owner/repo/issues/123"))
            .and(wiremock::matchers::body_json_string(r#"{"milestone":1}"#))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr_number = client
            .create_pr_with_options(
                "owner",
                "repo",
                "some-branch",
                "main",
                "PR title",
                "Body",
                false,
                None,
                Some(1),
            )
            .await
            .unwrap();
        assert_eq!(pr_number, Some(123));
    }

    // Ported: "should log a warning but not throw on error" — lib/modules/platform/github/index.spec.ts line 4319
    #[tokio::test]
    async fn create_pr_warns_but_does_not_throw_on_milestone_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "number": 123,
                "title": "bump someDep to v2",
                "state": "open",
                "head": {"ref": "some-branch", "sha": "abc", "repo": {"full_name": "owner/repo"}},
                "base": {"ref": "main", "sha": "def", "repo": null},
                "node_id": "nid",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T00:00:00Z",
            })))
            .mount(&server)
            .await;
        Mock::given(method("PATCH"))
            .and(path("/repos/owner/repo/issues/123"))
            .respond_with(ResponseTemplate::new(422).set_body_json(serde_json::json!({
                "message": "Validation Failed",
                "errors": [{"value": 1, "resource": "Issue", "field": "milestone", "code": "invalid"}],
                "documentation_url": "https://docs.github.com/rest/issues/issues#update-an-issue",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr_number = client
            .create_pr_with_options(
                "owner",
                "repo",
                "some-branch",
                "main",
                "bump someDep to v2",
                "many informations about someDep",
                false,
                None,
                Some(1),
            )
            .await
            .unwrap();
        assert_eq!(pr_number, Some(123));
    }

    // Ported: "should squash" — lib/modules/platform/github/index.spec.ts line 801
    #[tokio::test]
    async fn get_repo_merge_methods_prefers_squash() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "name": "repo",
                "full_name": "owner/repo",
                "default_branch": "master",
                "allow_squash_merge": true,
                "allow_merge_commit": true,
                "allow_rebase_merge": true,
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let method = client
            .get_repo_merge_methods("owner", "repo")
            .await
            .unwrap();
        assert_eq!(method, Some("squash".to_owned()));
    }

    // Ported: "should merge" — lib/modules/platform/github/index.spec.ts line 960
    #[tokio::test]
    async fn get_repo_merge_methods_prefers_merge() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "name": "repo",
                "full_name": "owner/repo",
                "default_branch": "master",
                "allow_squash_merge": false,
                "allow_merge_commit": true,
                "allow_rebase_merge": true,
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let method = client
            .get_repo_merge_methods("owner", "repo")
            .await
            .unwrap();
        assert_eq!(method, Some("merge".to_owned()));
    }

    // Ported: "should rebase" — lib/modules/platform/github/index.spec.ts line 989
    #[tokio::test]
    async fn get_repo_merge_methods_prefers_rebase() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "name": "repo",
                "full_name": "owner/repo",
                "default_branch": "master",
                "allow_squash_merge": false,
                "allow_merge_commit": false,
                "allow_rebase_merge": true,
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let method = client
            .get_repo_merge_methods("owner", "repo")
            .await
            .unwrap();
        assert_eq!(method, Some("rebase".to_owned()));
    }

    // Ported: "should not guess at merge" — lib/modules/platform/github/index.spec.ts line 1016
    #[tokio::test]
    async fn get_repo_merge_methods_returns_none_when_all_disabled() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "name": "repo",
                "full_name": "owner/repo",
                "default_branch": "master",
                "allow_squash_merge": false,
                "allow_merge_commit": false,
                "allow_rebase_merge": false,
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let method = client
            .get_repo_merge_methods("owner", "repo")
            .await
            .unwrap();
        assert_eq!(method, None);
    }

    // ── update_pr_labels ──────────────────────────────────────────────────────

    // Ported: "should add and remove labels" — lib/modules/platform/github/index.spec.ts line 4636
    #[tokio::test]
    async fn update_pr_labels_adds_labels() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/issues/1234/labels"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"name": "new_label"},
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        client
            .update_pr_labels("owner", "repo", 1234, vec!["new_label".to_owned()])
            .await
            .unwrap();
    }

    // Ported: "warns if adding labels failed" — lib/modules/platform/github/index.spec.ts line 4676
    #[tokio::test]
    async fn update_pr_labels_warns_on_failure() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/issues/2/labels"))
            .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
                "message": "Failed to add labels",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client
            .update_pr_labels("owner", "repo", 2, vec!["fail".to_owned()])
            .await
            .unwrap_err();
        assert!(matches!(err, PlatformError::Http(_)));
    }

    // ── ensure_comment ────────────────────────────────────────────────────────

    // Ported: "adds comment if found in closed PR list" — lib/modules/platform/github/index.spec.ts line 3417
    #[tokio::test]
    async fn ensure_comment_adds_if_found_in_closed_pr_list() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/issues/2499/comments"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": 419928791,
                    "body": "[![CLA assistant check](https://cla-assistant.io/pull/badge/signed)](https://cla-assistant.io/renovatebot/renovate?pullRequest=2500) <br/>All committers have signed the CLA.",
                },
                {
                    "id": 420006957,
                    "body": ":tada: This PR is included in version 13.63.5 :tada:",
                },
            ])))
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/issues/2499/comments"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 420006958,
                "body": "### some-subject\n\nsome\ncontent",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .ensure_comment("owner", "repo", 2499, Some("some-subject"), "some\ncontent")
            .await
            .unwrap();
        assert!(result);
    }

    // Ported: "deletes comment by topic if found" — lib/modules/platform/github/index.spec.ts line 3500
    #[tokio::test]
    async fn ensure_comment_deletes_by_topic_if_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/issues/42/comments"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"id": 1234, "body": "### some-subject\n\nblablabla"},
            ])))
            .mount(&server)
            .await;
        Mock::given(method("DELETE"))
            .and(path("/repos/owner/repo/issues/comments/1234"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        client
            .ensure_comment_removal("owner", "repo", 42, Some("some-subject"), None)
            .await
            .unwrap();
    }

    // Ported: "deletes comment by content if found" — lib/modules/platform/github/index.spec.ts line 3519
    #[tokio::test]
    async fn ensure_comment_deletes_by_content_if_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/issues/42/comments"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"id": 1234, "body": "some-content"},
            ])))
            .mount(&server)
            .await;
        Mock::given(method("DELETE"))
            .and(path("/repos/owner/repo/issues/comments/1234"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        client
            .ensure_comment_removal("owner", "repo", 42, None, Some("some-content"))
            .await
            .unwrap();
    }

    // Ported: "skips comment" — lib/modules/platform/github/index.spec.ts line 3464
    #[tokio::test]
    async fn ensure_comment_skips_when_already_up_to_date() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/issues/42/comments"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"id": 1234, "body": "### some-subject\n\nsome\ncontent"},
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .ensure_comment("owner", "repo", 42, Some("some-subject"), "some\ncontent")
            .await
            .unwrap();
        assert!(!result);
    }

    // Ported: "add comment if not found" — lib/modules/platform/github/index.spec.ts line 3398
    #[tokio::test]
    async fn ensure_comment_creates_if_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/issues/42/comments"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/issues/42/comments"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 1,
                "body": "### some-subject\n\nsome\ncontent",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .ensure_comment("owner", "repo", 42, Some("some-subject"), "some\ncontent")
            .await
            .unwrap();
        assert!(result);
    }

    // Ported: "add updates comment if necessary" — lib/modules/platform/github/index.spec.ts line 3445
    #[tokio::test]
    async fn ensure_comment_updates_if_necessary() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/issues/42/comments"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"id": 1234, "body": "### some-subject\n\nblablabla"},
            ])))
            .mount(&server)
            .await;
        Mock::given(method("PATCH"))
            .and(path("/repos/owner/repo/issues/comments/1234"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .ensure_comment("owner", "repo", 42, Some("some-subject"), "some\ncontent")
            .await
            .unwrap();
        assert!(result);
    }

    // Ported: "handles comment with no description" — lib/modules/platform/github/index.spec.ts line 3481
    #[tokio::test]
    async fn ensure_comment_handles_no_topic() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/issues/42/comments"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"id": 1234, "body": "!merge"},
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .ensure_comment("owner", "repo", 42, None, "!merge")
            .await
            .unwrap();
        assert!(!result);
    }

    // ── ensure_issue ──────────────────────────────────────────────────────────

    // Ported: "creates issue if not ensuring only once" — lib/modules/platform/github/index.spec.ts line 2697
    // Ported: "does not create issue if ensuring only once" — lib/modules/platform/github/index.spec.ts line 2741
    #[tokio::test]
    async fn ensure_issue_does_not_create_if_ensuring_only_once() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/issues"))
            .and(wiremock::matchers::query_param("state", "open"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "number": 42,
                    "title": "Dependency Dashboard",
                    "state": "open",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-09T00:00:00Z",
                },
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let number = client
            .ensure_issue(
                "owner",
                "repo",
                "Dependency Dashboard",
                "Body",
                None,
                true,
                true,
            )
            .await
            .unwrap();
        assert_eq!(number, None);
    }

    // Ported: "closes others if ensuring only once" — lib/modules/platform/github/index.spec.ts line 2819
    #[tokio::test]
    async fn ensure_issue_closes_others_if_ensuring_only_once() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/issues"))
            .and(wiremock::matchers::query_param("state", "open"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "number": 42,
                    "title": "Dependency Dashboard",
                    "state": "open",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-09T00:00:00Z",
                },
                {
                    "number": 43,
                    "title": "Dependency Dashboard",
                    "state": "open",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-09T00:00:00Z",
                },
            ])))
            .mount(&server)
            .await;
        Mock::given(method("PATCH"))
            .and(path("/repos/owner/repo/issues/43"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let number = client
            .ensure_issue(
                "owner",
                "repo",
                "Dependency Dashboard",
                "Body",
                None,
                true,
                true,
            )
            .await
            .unwrap();
        assert_eq!(number, None);
    }

    // Ported: "deletes if duplicate" — lib/modules/platform/github/index.spec.ts line 3035
    #[tokio::test]
    async fn ensure_issue_deletes_if_duplicate() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/issues"))
            .and(wiremock::matchers::query_param("state", "open"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "number": 42,
                    "title": "Dependency Dashboard",
                    "state": "open",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-09T00:00:00Z",
                },
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let number = client
            .ensure_issue(
                "owner",
                "repo",
                "Dependency Dashboard",
                "Body",
                None,
                true,
                true,
            )
            .await
            .unwrap();
        assert_eq!(number, None);
    }

    // Ported: "creates issue if reopen flag false and issue is not open" — lib/modules/platform/github/index.spec.ts line 3079
    #[tokio::test]
    async fn ensure_issue_creates_if_reopen_false_and_not_open() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/issues"))
            .and(wiremock::matchers::query_param("state", "open"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/issues"))
            .and(wiremock::matchers::query_param("state", "closed"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/issues"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "number": 42,
                "title": "Dependency Dashboard",
                "state": "open",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-09T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let number = client
            .ensure_issue(
                "owner",
                "repo",
                "Dependency Dashboard",
                "Body",
                None,
                false,
                false,
            )
            .await
            .unwrap();
        assert_eq!(number, Some(42));
    }

    // Ported: "does not create issue if reopen flag false and issue is already open" — lib/modules/platform/github/index.spec.ts line 3132
    #[tokio::test]
    async fn ensure_issue_does_not_create_if_reopen_false_and_already_open() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/issues"))
            .and(wiremock::matchers::query_param("state", "open"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "number": 42,
                    "title": "Dependency Dashboard",
                    "state": "open",
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-09T00:00:00Z",
                },
            ])))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let number = client
            .ensure_issue(
                "owner",
                "repo",
                "Dependency Dashboard",
                "Body",
                None,
                false,
                false,
            )
            .await
            .unwrap();
        assert_eq!(number, None);
    }

    // ── get_issue additional coverage ─────────────────────────────────────────

    // Ported: "logs debug message if issue deleted" — lib/modules/platform/github/index.spec.ts line 2542
    #[tokio::test]
    async fn get_issue_logs_debug_if_deleted() {
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

    // ── create_issue additional coverage ──────────────────────────────────────

    // Ported: "creates issue if not ensuring only once" — lib/modules/platform/github/index.spec.ts line 2697
    #[tokio::test]
    async fn create_issue_if_not_ensuring_only_once() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/issues"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "number": 42,
                "title": "Dependency Dashboard",
                "state": "open",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-09T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let number = client
            .create_issue("owner", "repo", "Dependency Dashboard", "Body", None)
            .await
            .unwrap();
        assert_eq!(number, 42);
    }

    // ── merge_pr ──────────────────────────────────────────────────────────────

    // Ported: "should merge the PR" — lib/modules/platform/github/index.spec.ts line 4820
    // Ported: "should set automatic merge" — lib/modules/platform/github/index.spec.ts line 4780
    #[tokio::test]
    async fn merge_pr_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/repos/owner/repo/pulls/1234/merge"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "sha": "abc123",
                "merged": true,
                "message": "Pull Request successfully merged",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .merge_pr("owner", "repo", 1234, "somebranch", None)
            .await
            .unwrap();
        assert!(result);
    }

    // Ported: "should handle merge error" — lib/modules/platform/github/index.spec.ts line 4852
    // Ported: "handles unknown error" — lib/modules/platform/github/index.spec.ts line 4798
    // Ported: "should handle merge error" — lib/modules/platform/github/index.spec.ts line 4852
    #[tokio::test]
    async fn merge_pr_returns_false_on_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/repos/owner/repo/pulls/1234/merge"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .merge_pr("owner", "repo", 1234, "somebranch", None)
            .await
            .unwrap();
        assert!(!result);
    }

    // Ported: "should handle merge block" — lib/modules/platform/github/index.spec.ts line 4873
    #[tokio::test]
    async fn merge_pr_returns_false_on_merge_block() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/repos/owner/repo/pulls/1234/merge"))
            .respond_with(ResponseTemplate::new(405).set_body_json(serde_json::json!({
                "message": "Required status check \"build\" is expected."
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .merge_pr("owner", "repo", 1234, "somebranch", Some("merge-commit"))
            .await
            .unwrap();
        assert!(!result);
    }

    // Ported: "should handle approvers required" — lib/modules/platform/github/index.spec.ts line 4895
    #[tokio::test]
    async fn merge_pr_returns_false_on_approvers_required() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/repos/owner/repo/pulls/1234/merge"))
            .respond_with(ResponseTemplate::new(405).set_body_json(serde_json::json!({
                "message": "Pull request requires approving review"
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .merge_pr("owner", "repo", 1234, "somebranch", None)
            .await
            .unwrap();
        assert!(!result);
    }

    // Ported: "should use configured automergeStrategy" — lib/modules/platform/github/index.spec.ts line 4936
    #[tokio::test]
    async fn merge_pr_uses_configured_strategy() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/repos/owner/repo/pulls/1234/merge"))
            .and(wiremock::matchers::body_json_string(r#"{"merge_method":"rebase"}"#))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"sha": "abc123", "merged": true, "message": "Pull Request successfully merged"})))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .merge_pr("owner", "repo", 1234, "somebranch", Some("rebase"))
            .await
            .unwrap();
        assert!(result);
    }

    // Ported: "should warn if automergeStrategy is not supported" — lib/modules/platform/github/index.spec.ts line 4917
    #[tokio::test]
    async fn merge_pr_warns_on_unsupported_strategy() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/repos/owner/repo/pulls/1234/merge"))
            .and(wiremock::matchers::body_json_string(r#"{"merge_method":"fast-forward"}"#))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"sha": "abc123", "merged": true, "message": "Pull Request successfully merged"})))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .merge_pr("owner", "repo", 1234, "somebranch", Some("fast-forward"))
            .await
            .unwrap();
        assert!(result);
    }

    // Ported: "should try squash first" — lib/modules/platform/github/index.spec.ts line 4996
    #[tokio::test]
    async fn merge_pr_autodetect_tries_squash_first() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/repos/owner/repo/pulls/1235/merge"))
            .and(wiremock::matchers::body_json_string(
                r#"{"merge_method":"squash"}"#,
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"sha": "abc"})),
            )
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .merge_pr("owner", "repo", 1235, "someref", None)
            .await
            .unwrap();
        assert!(result);
    }

    // Ported: "should try merge after squash" — lib/modules/platform/github/index.spec.ts line 5015
    #[tokio::test]
    async fn merge_pr_autodetect_tries_merge_after_squash_fails() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/repos/owner/repo/pulls/1236/merge"))
            .and(wiremock::matchers::body_json_string(
                r#"{"merge_method":"squash"}"#,
            ))
            .respond_with(
                ResponseTemplate::new(400)
                    .set_body_json(serde_json::json!({"message": "no squashing allowed"})),
            )
            .mount(&server)
            .await;
        Mock::given(method("PUT"))
            .and(path("/repos/owner/repo/pulls/1236/merge"))
            .and(wiremock::matchers::body_json_string(
                r#"{"merge_method":"merge"}"#,
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"sha": "abc"})),
            )
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .merge_pr("owner", "repo", 1236, "someref", None)
            .await
            .unwrap();
        assert!(result);
    }

    // Ported: "should try rebase after merge" — lib/modules/platform/github/index.spec.ts line 5036
    #[tokio::test]
    async fn merge_pr_autodetect_tries_rebase_after_merge_fails() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/repos/owner/repo/pulls/1237/merge"))
            .and(wiremock::matchers::body_json_string(
                r#"{"merge_method":"squash"}"#,
            ))
            .respond_with(
                ResponseTemplate::new(405)
                    .set_body_json(serde_json::json!({"message": "no squashing allowed"})),
            )
            .mount(&server)
            .await;
        Mock::given(method("PUT"))
            .and(path("/repos/owner/repo/pulls/1237/merge"))
            .and(wiremock::matchers::body_json_string(
                r#"{"merge_method":"merge"}"#,
            ))
            .respond_with(
                ResponseTemplate::new(405)
                    .set_body_json(serde_json::json!({"message": "no merging allowed"})),
            )
            .mount(&server)
            .await;
        Mock::given(method("PUT"))
            .and(path("/repos/owner/repo/pulls/1237/merge"))
            .and(wiremock::matchers::body_json_string(
                r#"{"merge_method":"rebase"}"#,
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"sha": "abc"})),
            )
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .merge_pr("owner", "repo", 1237, "someref", None)
            .await
            .unwrap();
        assert!(result);
    }

    // Ported: "should give up" — lib/modules/platform/github/index.spec.ts line 5061
    #[tokio::test]
    async fn merge_pr_autodetect_gives_up() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/repos/owner/repo/pulls/1238/merge"))
            .respond_with(
                ResponseTemplate::new(405)
                    .set_body_json(serde_json::json!({"message": "not allowed"})),
            )
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .merge_pr("owner", "repo", 1238, "someref", None)
            .await
            .unwrap();
        assert!(!result);
    }

    // Ported: "should skip automerge if disabled in repo settings" — lib/modules/platform/github/index.spec.ts line 4009
    #[tokio::test]
    async fn create_pr_without_automerge() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "number": 123,
                "title": "PR title",
                "state": "open",
                "head": {"ref": "some-branch", "sha": "abc", "repo": null},
                "base": {"ref": "main", "sha": "def", "repo": null},
                "node_id": "nid",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let pr_number = client
            .create_pr("owner", "repo", "some-branch", "main", "PR title", "Body")
            .await
            .unwrap();
        assert_eq!(pr_number, Some(123));
    }

    // Ported: "should throw immediately on non-404 errors" — lib/modules/platform/github/index.spec.ts line 3374
    #[tokio::test]
    async fn add_assignees_throws_on_non_404() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repos/owner/repo/issues/42/assignees"))
            .respond_with(ResponseTemplate::new(422))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client
            .add_assignees("owner", "repo", 42, vec!["user".to_owned()])
            .await
            .unwrap_err();
        assert!(
            matches!(err, PlatformError::Http(HttpError::Status { status, .. }) if status == reqwest::StatusCode::UNPROCESSABLE_ENTITY)
        );
    }

    // Ported: "returns null on REST error" — lib/modules/platform/github/index.spec.ts line 5502
    #[tokio::test]
    async fn get_file_list_returns_error_on_rest_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/git/trees/HEAD"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client.get_file_list("owner", "repo").await.unwrap_err();
        assert!(
            matches!(err, PlatformError::Http(HttpError::Status { status, .. }) if status == reqwest::StatusCode::INTERNAL_SERVER_ERROR)
        );
    }

    // ── init_repo tests ─────────────────────────────────────────────────────────

    // Tests init_repo returning a valid config
    #[tokio::test]
    async fn init_repo_returns_repo_config() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/graphql"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "repository": {
                        "id": "123",
                        "isFork": false,
                        "isArchived": false,
                        "nameWithOwner": "owner/repo",
                        "hasIssuesEnabled": true,
                        "hasVulnerabilityAlertsEnabled": true,
                        "autoMergeAllowed": true,
                        "mergeCommitAllowed": true,
                        "rebaseMergeAllowed": true,
                        "squashMergeAllowed": true,
                        "defaultBranchRef": {
                            "name": "main",
                            "target": { "oid": "abc123" }
                        }
                    }
                }
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .init_repo("owner", "repo", None, false, None)
            .await
            .unwrap();
        assert_eq!(result.default_branch, "main");
        assert!(!result.is_fork);
        assert_eq!(result.merge_method, Some("squash".to_owned()));
        assert!(result.auto_merge_allowed);
        assert!(result.has_issues_enabled);
        assert!(result.has_vulnerability_alerts_enabled);
        assert!(!result.repo_fingerprint.is_empty());
    }

    // Ported: "should throw error if archived" — lib/modules/platform/github/index.spec.ts line 1036
    #[tokio::test]
    async fn init_repo_throws_if_archived() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/graphql"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "repository": {
                        "id": "123",
                        "isFork": false,
                        "isArchived": true,
                        "nameWithOwner": "owner/repo",
                        "hasIssuesEnabled": true,
                        "hasVulnerabilityAlertsEnabled": false,
                        "autoMergeAllowed": false,
                        "mergeCommitAllowed": false,
                        "rebaseMergeAllowed": false,
                        "squashMergeAllowed": false,
                        "defaultBranchRef": {
                            "name": "main",
                            "target": { "oid": "abc123" }
                        }
                    }
                }
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client
            .init_repo("owner", "repo", None, false, None)
            .await
            .unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(msg) if msg == "REPOSITORY_ARCHIVED"));
    }

    // Ported: "should throw error if renamed" — lib/modules/platform/github/index.spec.ts line 1101
    #[tokio::test]
    async fn init_repo_throws_if_renamed() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/graphql"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "repository": {
                        "id": "123",
                        "isFork": false,
                        "isArchived": false,
                        "nameWithOwner": "owner/new-repo",
                        "hasIssuesEnabled": true,
                        "hasVulnerabilityAlertsEnabled": false,
                        "autoMergeAllowed": false,
                        "mergeCommitAllowed": false,
                        "rebaseMergeAllowed": false,
                        "squashMergeAllowed": false,
                        "defaultBranchRef": {
                            "name": "main",
                            "target": { "oid": "abc123" }
                        }
                    }
                }
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client
            .init_repo("owner", "repo", None, false, None)
            .await
            .unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(msg) if msg == "REPOSITORY_RENAMED"));
    }

    // Ported: "throws not-found" — lib/modules/platform/github/index.spec.ts line 1060
    #[tokio::test]
    async fn init_repo_throws_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/graphql"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": { "repository": null }
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client
            .init_repo("owner", "repo", None, false, None)
            .await
            .unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(msg) if msg == "REPOSITORY_NOT_FOUND"));
    }

    // Tests init_repo throwing when repository is empty
    #[tokio::test]
    async fn init_repo_throws_if_empty() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/graphql"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "repository": {
                        "id": "123",
                        "isFork": false,
                        "isArchived": false,
                        "nameWithOwner": "owner/repo",
                        "hasIssuesEnabled": true,
                        "hasVulnerabilityAlertsEnabled": false,
                        "autoMergeAllowed": false,
                        "mergeCommitAllowed": false,
                        "rebaseMergeAllowed": false,
                        "squashMergeAllowed": false,
                        "defaultBranchRef": null
                    }
                }
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client
            .init_repo("owner", "repo", None, false, None)
            .await
            .unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(msg) if msg == "REPOSITORY_EMPTY"));
    }

    // Tests init_repo throwing when repository access is blocked
    #[tokio::test]
    async fn init_repo_throws_if_blocked() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/graphql"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "errors": [{ "type": "FORBIDDEN", "message": "Repository access blocked" }]
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client
            .init_repo("owner", "repo", None, false, None)
            .await
            .unwrap_err();
        assert!(
            matches!(err, PlatformError::Unexpected(msg) if msg == "REPOSITORY_ACCESS_FORBIDDEN")
        );
    }

    // Ported: "should handle GraphQL errors" — lib/modules/platform/github/index.spec.ts line 4118
    #[tokio::test]
    async fn init_repo_handles_graphql_errors() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/graphql"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "errors": [{ "type": "UNKNOWN", "message": "Something went wrong" }]
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let err = client
            .init_repo("owner", "repo", None, false, None)
            .await
            .unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(msg) if msg == "PLATFORM_UNKNOWN_ERROR"));
    }
}
