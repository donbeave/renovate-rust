//! GitHub platform client.
//!
//! Implements [`PlatformClient`] against the GitHub REST API v3.
//!
//! Renovate reference: `lib/modules/platform/github/index.ts`.

use std::sync::LazyLock;

use base64::Engine as _;
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::Deserialize;

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
    replacements.sort_by(|a, b| b.0.cmp(&a.0));
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
#[derive(Debug, Deserialize)]
struct GithubUser {
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

#[cfg(test)]
mod tests {
    use wiremock::matchers::{header, header_exists, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

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

    // Ported: "performs multiple replacements" — modules/platform/github/massage-markdown-links.spec.ts line 4
    #[test]
    fn massage_markdown_links_performs_multiple_replacements() {
        let input = "Link [foo/bar#1](https://github.com/foo/bar/pull/1) points to https://github.com/foo/bar/pull/1.";
        let expected = "Link [foo/bar#1](https://redirect.github.com/foo/bar/pull/1) points to [https://github.com/foo/bar/pull/1](https://redirect.github.com/foo/bar/pull/1).";
        assert_eq!(massage_markdown_links(input), expected);
    }

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
}
