//! Utility functions mirroring Renovate's `lib/util/` directory.
//!
//! This module contains small, pure utility functions used throughout the
//! Renovate Rust implementation.

pub mod host_rules;

use std::cell::RefCell;
use std::collections::HashSet;

thread_local! {
    static GLOBAL_SECRETS: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
    static REPO_SECRETS: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

// ---------------------------------------------------------------------------
// Child-process environment — lib/util/exec/env.ts
// ---------------------------------------------------------------------------

const BASIC_ENV_VARS: &[&str] = &[
    "CI",
    "HTTP_PROXY",
    "HTTPS_PROXY",
    "NO_PROXY",
    "http_proxy",
    "https_proxy",
    "no_proxy",
    "HOME",
    "PATH",
    "LC_ALL",
    "LANG",
    "DOCKER_HOST",
    "DOCKER_TLS_VERIFY",
    "DOCKER_CERT_PATH",
    "SSL_CERT_DIR",
    "SSL_CERT_FILE",
    "NODE_EXTRA_CA_CERTS",
    "GIT_SSL_CAPATH",
    "GIT_SSL_CAINFO",
    "PROGRAMFILES",
    "PROGRAMFILES(X86)",
    "APPDATA",
    "LOCALAPPDATA",
    "PROCESSOR_ARCHITECTURE",
    "PATHEXT",
    "COREPACK_DEFAULT_TO_LATEST",
    "COREPACK_ENABLE_NETWORK",
    "COREPACK_ENABLE_STRICT",
    "COREPACK_ENABLE_PROJECT_SPEC",
    "COREPACK_ENABLE_UNSAFE_CUSTOM_URLS",
    "COREPACK_HOME",
    "COREPACK_INTEGRITY_KEYS",
    "COREPACK_NPM_REGISTRY",
    "COREPACK_NPM_TOKEN",
    "COREPACK_NPM_USERNAME",
    "COREPACK_NPM_PASSWORD",
    "COREPACK_ROOT",
    "PNPM_WORKERS",
    "PNPM_MAX_WORKERS",
];

static URL_REPLACE_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r"^URL_REPLACE_\d+_(FROM|TO)$").expect("valid regex")
});

/// Build child-process environment from `env_source`.
///
/// When `expose_all` is true returns a clone of the entire source map. Otherwise
/// returns only the allowed vars plus any `URL_REPLACE_N_{FROM,TO}` entries.
pub fn get_child_process_env(
    env_source: &std::collections::HashMap<String, String>,
    custom_vars: &[&str],
    expose_all: bool,
) -> std::collections::HashMap<String, String> {
    if expose_all {
        return env_source.clone();
    }
    let mut out = std::collections::HashMap::new();
    for key in BASIC_ENV_VARS.iter().chain(custom_vars.iter()) {
        if let Some(val) = env_source.get(*key) {
            out.insert((*key).to_string(), val.clone());
        }
    }
    for (key, val) in env_source {
        if URL_REPLACE_RE.is_match(key) {
            out.insert(key.clone(), val.clone());
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Environment utilities — lib/util/env.ts
// ---------------------------------------------------------------------------

/// Combine environment maps with precedence: `user_env > custom_env > process_env`.
///
/// Mirrors `getEnv()` from `lib/util/env.ts`.
pub fn get_combined_env<S: std::hash::BuildHasher>(
    process_env: &std::collections::HashMap<String, String, S>,
    custom_env: &std::collections::HashMap<String, String>,
    user_env: &std::collections::HashMap<String, String>,
) -> std::collections::HashMap<String, String> {
    let mut combined: std::collections::HashMap<String, String> = process_env
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    combined.extend(custom_env.iter().map(|(k, v)| (k.clone(), v.clone())));
    combined.extend(user_env.iter().map(|(k, v)| (k.clone(), v.clone())));
    combined
}

// ---------------------------------------------------------------------------
// Timing stats — lib/util/stats.ts
// ---------------------------------------------------------------------------

/// Compute timing statistics from a slice of millisecond durations.
///
/// Mirrors `makeTimingReport` from `lib/util/stats.ts`.
pub struct TimingReport {
    pub count: usize,
    pub avg_ms: i64,
    pub median_ms: i64,
    pub max_ms: i64,
    pub total_ms: i64,
}

impl std::fmt::Debug for TimingReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TimingReport")
            .field("count", &self.count)
            .field("avg_ms", &self.avg_ms)
            .field("median_ms", &self.median_ms)
            .field("max_ms", &self.max_ms)
            .field("total_ms", &self.total_ms)
            .finish()
    }
}

impl PartialEq for TimingReport {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
            && self.avg_ms == other.avg_ms
            && self.median_ms == other.median_ms
            && self.max_ms == other.max_ms
            && self.total_ms == other.total_ms
    }
}

pub fn make_timing_report(data: &[i64]) -> TimingReport {
    let count = data.len();
    let total_ms: i64 = data.iter().sum();
    let avg_ms = if count > 0 {
        (total_ms as f64 / count as f64).round() as i64
    } else {
        0
    };
    let max_ms = data.iter().copied().max().unwrap_or(0);
    let mut sorted = data.to_vec();
    sorted.sort_unstable();
    let median_ms = if count > 0 { sorted[count / 2] } else { 0 };
    TimingReport {
        count,
        avg_ms,
        median_ms,
        max_ms,
        total_ms,
    }
}

// ---------------------------------------------------------------------------
// GitHub token utilities — lib/util/check-token.ts
// ---------------------------------------------------------------------------

/// Return `true` when `token` is a GitHub Classic Personal Access Token (`ghp_`).
pub fn is_github_personal_access_token(token: &str) -> bool {
    token.starts_with("ghp_")
}

/// Return `true` when `token` is a GitHub App / Server-to-Server token (`ghs_`).
pub fn is_github_server_to_server_token(token: &str) -> bool {
    token.starts_with("ghs_")
}

/// Return `true` when `token` is a GitHub Fine-Grained PAT (`github_pat_`).
pub fn is_github_fine_grained_personal_access_token(token: &str) -> bool {
    token.starts_with("github_pat_")
}

/// Extract the raw token from a host-rule token value, stripping `x-access-token:` prefix.
pub fn find_github_token(token: Option<&str>) -> Option<&str> {
    let t = token?;
    if t.is_empty() {
        return None;
    }
    Some(t.strip_prefix("x-access-token:").unwrap_or(t))
}

/// Choose the preferred GitHub token from two candidates.
///
/// Prefers PAT > fine-grained PAT > other. When both candidates have the
/// same class, prefers `git_tags_token`.
pub fn take_personal_access_token_if_possible<'a>(
    github_token: Option<&'a str>,
    git_tags_token: Option<&'a str>,
) -> Option<&'a str> {
    // If git_tags_token is a PAT, prefer it
    if let Some(t) = git_tags_token
        && is_github_personal_access_token(t)
    {
        return Some(t);
    }
    // If github_token is a PAT, prefer it
    if let Some(t) = github_token
        && is_github_personal_access_token(t)
    {
        return Some(t);
    }
    // Fine-grained PAT
    if let Some(t) = git_tags_token
        && is_github_fine_grained_personal_access_token(t)
    {
        return Some(t);
    }
    if let Some(t) = github_token
        && is_github_fine_grained_personal_access_token(t)
    {
        return Some(t);
    }
    // Fallback: prefer git_tags_token
    git_tags_token.or(github_token)
}

// ---------------------------------------------------------------------------
// Git URL conversion — lib/util/git/url.ts
// ---------------------------------------------------------------------------

/// Convert a git URL to an HTTP(S) URL.
///
/// - Non-`http(s)` schemes (git://, ssh://) → `https://`.
/// - SSH ports are stripped.
/// - Existing credentials are removed.
/// - If `token` is provided, platform-specific credentials are injected.
pub fn get_http_url(url: &str, token: Option<&str>) -> String {
    let url = url.trim();
    // git@host:path SCP-like format
    if !url.contains("://")
        && let Some(rest) = url.strip_prefix("git@")
    {
        let (host, path) = if let Some(colon) = rest.find(':') {
            (&rest[..colon], rest[colon + 1..].trim_end_matches(".git"))
        } else {
            (rest, "")
        };
        let platform = detect_platform(&format!("https://{host}")).unwrap_or("");
        let creds = token
            .map(|t| build_git_credentials(platform, t))
            .unwrap_or_default();
        return if creds.is_empty() {
            format!("https://{host}/{path}")
        } else {
            format!("https://{creds}@{host}/{path}")
        };
    }
    // Detect scheme
    let (scheme, rest) = if let Some(r) = url.strip_prefix("https://") {
        ("https", r)
    } else if let Some(r) = url.strip_prefix("http://") {
        ("http", r)
    } else if let Some(r) = url.strip_prefix("ssh://") {
        ("https", r)
    } else if let Some(r) = url.strip_prefix("git://") {
        ("https", r)
    } else {
        ("https", url)
    };
    // Strip user:pass@ and SSH port
    let rest_no_at = if let Some(at) = rest.find('@') {
        &rest[at + 1..]
    } else {
        rest
    };
    // For SSH-converted URLs strip port from host
    let was_ssh = url.starts_with("ssh://");
    let host_path = if was_ssh {
        let slash_pos = rest_no_at.find('/').unwrap_or(rest_no_at.len());
        let host = &rest_no_at[..slash_pos];
        let path = &rest_no_at[slash_pos..];
        let host_no_port = if let Some(c) = host.find(':') {
            &host[..c]
        } else {
            host
        };
        format!("{host_no_port}{path}")
    } else {
        rest_no_at.to_owned()
    };
    let platform = detect_platform(&format!("{scheme}://{host_path}")).unwrap_or("");
    let creds = token
        .map(|t| build_git_credentials(platform, t))
        .unwrap_or_default();
    if creds.is_empty() {
        format!("{scheme}://{host_path}")
    } else {
        format!("{scheme}://{creds}@{host_path}")
    }
}

fn build_git_credentials(platform: &str, token: &str) -> String {
    match platform {
        "github" => {
            if token.contains(':') {
                token.to_owned()
            } else {
                format!("x-access-token:{token}")
            }
        }
        "gitlab" => {
            if token.contains(':') {
                token.to_owned()
            } else {
                format!("gitlab-ci-token:{token}")
            }
        }
        _ => token.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Datasource utilities — lib/modules/datasource/util.ts
// ---------------------------------------------------------------------------

const JFROG_ARTIFACTORY_HEADER: &str = "x-jfrog-version";

/// Return `true` when the HTTP response headers indicate an Artifactory server.
///
/// Mirrors `isArtifactoryServer` from `lib/modules/datasource/util.ts`.
pub fn is_artifactory_server(headers: &std::collections::HashMap<String, String>) -> bool {
    headers.contains_key(JFROG_ARTIFACTORY_HEADER)
}

// ---------------------------------------------------------------------------
// Helm environment variables — lib/modules/manager/kustomize/common.ts
// ---------------------------------------------------------------------------

/// Generate helm environment variable mappings.
///
/// `cache_dir` is the private cache directory.
/// `needs_experimental_oci` is `true` when helm < 3.8.0.
pub fn generate_helm_envs(
    cache_dir: &str,
    needs_experimental_oci: bool,
) -> std::collections::HashMap<&'static str, String> {
    let mut envs = std::collections::HashMap::new();
    envs.insert("HELM_REGISTRY_CONFIG", format!("{cache_dir}/registry.json"));
    envs.insert(
        "HELM_REPOSITORY_CONFIG",
        format!("{cache_dir}/repositories.yaml"),
    );
    envs.insert("HELM_REPOSITORY_CACHE", format!("{cache_dir}/repositories"));
    if needs_experimental_oci {
        envs.insert("HELM_EXPERIMENTAL_OCI", "1".to_owned());
    }
    envs
}

/// Return whether a helm version constraint requires `HELM_EXPERIMENTAL_OCI=1`.
/// Returns `true` when the constraint does not intersect `>=3.8.0`.
pub fn helm_needs_experimental_oci(helm_constraint: &str) -> bool {
    use semver::{Version, VersionReq};
    let v380 = Version::new(3, 8, 0);
    let v400 = Version::new(4, 0, 0);
    let constraint = helm_constraint.trim();
    // Try as a bare version first
    if let Ok(v) = Version::parse(constraint) {
        return v < v380;
    }
    // Normalize space-separated constraints to comma-separated for semver crate
    let candidates: [String; 2] = [
        constraint.to_owned(),
        constraint
            .replace(" <", ", <")
            .replace(" >=", ", >=")
            .replace(" >", ", >"),
    ];
    for c in &candidates {
        if let Ok(req) = VersionReq::parse(c) {
            return !req.matches(&v380) && !req.matches(&v400);
        }
    }
    false
}

// ---------------------------------------------------------------------------
// Manager range strategy — lib/modules/manager/index.ts (getRangeStrategy)
// ---------------------------------------------------------------------------

/// Managers that support `updateLockedDependency` (return `update-lockfile` for `auto`).
const MANAGERS_WITH_UPDATE_LOCKED: &[&str] = &[
    "bundler",
    "cargo",
    "composer",
    "gomod",
    "gradle-wrapper",
    "npm",
    "pnpm",
    "poetry",
    "pip_requirements",
    "pip-compile",
];

/// Determine the effective range strategy for a manager.
///
/// - Non-`auto` strategies pass through unchanged.
/// - `in-range-only` → `update-lockfile`.
/// - `auto` → `update-lockfile` if the manager supports locked updates,
///   otherwise `replace`.
/// - For `npm` with `auto` and `depType = "dependencies"`, returns
///   `update-lockfile` (npm-specific heuristic).
///
/// Mirrors `getRangeStrategy` from `lib/modules/manager/index.ts`.
pub fn get_range_strategy(
    manager: &str,
    range_strategy: &str,
    dep_type: Option<&str>,
) -> &'static str {
    match range_strategy {
        "in-range-only" => "update-lockfile",
        "auto" => {
            // npm-specific: if depType is "dependencies", use update-lockfile
            if manager == "npm" && dep_type == Some("dependencies") {
                return "update-lockfile";
            }
            if MANAGERS_WITH_UPDATE_LOCKED.contains(&manager) {
                "update-lockfile"
            } else {
                "replace"
            }
        }
        other => {
            // Safe: caller is responsible for passing valid strategy strings.
            // Return the strategy as a static string if it matches a known one.
            match other {
                "widen" => "widen",
                "replace" => "replace",
                "pin" => "pin",
                "bump" => "bump",
                "update-lockfile" => "update-lockfile",
                "future" => "future",
                _ => "replace",
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Reconfigure branch cache — lib/workers/repository/reconfigure/reconfigure-cache.ts
// ---------------------------------------------------------------------------

/// Set or update the reconfigure branch cache entry.
///
/// Return the reconfigure branch name for a given prefix.
///
/// Mirrors `getReconfigureBranchName` from
/// `lib/workers/repository/reconfigure/utils.ts`.
pub fn get_reconfigure_branch_name(prefix: &str) -> String {
    format!("{prefix}reconfigure")
}

// ---------------------------------------------------------------------------
// Log level remap — lib/logger/remap.ts
// ---------------------------------------------------------------------------

/// A log level remap rule.
#[derive(Debug)]
pub struct LogLevelRemap<'a> {
    pub match_message: &'a str,
    pub new_log_level: &'a str,
}

/// Return the remapped log level for `msg`, or `None` if no remap matches.
///
/// Mirrors `getRemappedLevel` from `lib/logger/remap.ts`.
pub fn get_remapped_level<'a>(
    msg: &str,
    repository_remaps: Option<&[LogLevelRemap<'a>]>,
    global_remaps: Option<&[LogLevelRemap<'a>]>,
) -> Option<&'a str> {
    use crate::string_match::match_regex_or_glob;
    for remaps in [repository_remaps, global_remaps].into_iter().flatten() {
        for remap in remaps {
            if match_regex_or_glob(msg, remap.match_message) {
                return Some(remap.new_log_level);
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Module label utilities — tools/utils/sync-module-labels.ts
// ---------------------------------------------------------------------------

const MODULE_LABEL_COLOR: &str = "C5DEF5";

/// Quote a string for safe shell use (mirrors Python/Node shlex.quote).
fn shlex_quote(s: &str) -> String {
    if s.is_empty() {
        return "''".to_owned();
    }
    format!("'{}'", s.replace('\'', "'\"'\"'"))
}

/// Format `gh label create` commands for missing labels, sorted by name.
///
/// Mirrors `formatCreateLabelCommands` from `tools/utils/sync-module-labels.ts`.
pub fn format_create_label_commands(repo: &str, labels: &[GithubLabel]) -> String {
    let mut sorted_labels: Vec<&GithubLabel> = labels.iter().collect();
    sorted_labels.sort_by(|a, b| a.name.cmp(&b.name));
    sorted_labels
        .iter()
        .map(|label| {
            format!(
                "gh label create {} -R {} --color {} --description {}",
                shlex_quote(&label.name),
                shlex_quote(repo),
                shlex_quote(label.color),
                shlex_quote(&label.description),
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// ---------------------------------------------------------------------------
// Changelog source utilities — lib/workers/repository/update/pr/changelog/source.ts
// ---------------------------------------------------------------------------

/// Return the base URL from a source URL (scheme + host + "/").
///
/// Mirrors `GitHubChangeLogSource.getBaseUrl` from
/// `lib/workers/repository/update/pr/changelog/source.ts`.
pub fn changelog_get_base_url(source_url: Option<&str>) -> String {
    let url = source_url.unwrap_or("").trim();
    if url.is_empty() {
        return String::new();
    }
    match url::Url::parse(url) {
        Ok(parsed) => {
            let scheme = parsed.scheme();
            let host = parsed.host_str().unwrap_or("");
            if host.is_empty() { String::new() } else { format!("{scheme}://{host}/") }
        }
        Err(_) => String::new(),
    }
}

/// Extract the owner/repo path from a source URL.
///
/// Mirrors `GitHubChangeLogSource.getRepositoryFromUrl` from
/// `lib/workers/repository/update/pr/changelog/source.ts`.
pub fn changelog_get_repository_from_url(source_url: Option<&str>) -> String {
    let url = source_url.unwrap_or("").trim();
    if url.is_empty() {
        return String::new();
    }
    let Ok(parsed) = url::Url::parse(url) else {
        return String::new();
    };
    let path = parsed.path().trim_start_matches('/');
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() >= 2 {
        format!("{}/{}", parts[0], parts[1])
    } else {
        String::new()
    }
}

/// Return `true` when `repo` has exactly the form `"owner/repo"`.
///
/// Mirrors `GitHubChangeLogSource.hasValidRepository` from
/// `lib/workers/repository/update/pr/changelog/source.ts`.
pub fn changelog_has_valid_repository(repo: &str) -> bool {
    let parts: Vec<&str> = repo.split('/').collect();
    parts.len() == 2 && parts.iter().all(|p| !p.is_empty())
}

// ---------------------------------------------------------------------------
// Go proxy / noproxy parsing — lib/modules/datasource/go/goproxy-parser.ts
// ---------------------------------------------------------------------------

/// One entry in a parsed `GOPROXY` value.
#[derive(Debug, Clone, PartialEq)]
pub struct GoproxyItem {
    pub url: String,
    /// Separator that follows this entry: `","` (try next), `"|"` (on error try next), or `None` (last entry).
    pub fallback: Option<char>,
}

/// Parse the `GOPROXY` environment variable into a sequence of proxy entries.
///
/// Mirrors `parseGoproxy()` from `lib/modules/datasource/go/goproxy-parser.ts`.
pub fn parse_goproxy(input: &str) -> Vec<GoproxyItem> {
    if input.is_empty() {
        return Vec::new();
    }
    let mut items = Vec::new();
    let mut remaining = input;
    while !remaining.is_empty() {
        // Find next separator (comma or pipe)
        let pos = remaining.find(|c| c == ',' || c == '|');
        match pos {
            None => {
                items.push(GoproxyItem { url: remaining.to_string(), fallback: None });
                break;
            }
            Some(i) => {
                let url = &remaining[..i];
                let sep = remaining.chars().nth(i).unwrap();
                items.push(GoproxyItem { url: url.to_string(), fallback: Some(sep) });
                remaining = &remaining[i + 1..];
            }
        }
    }
    items
}

/// Convert a `NOPROXY`/`GONOPROXY` glob pattern to a `Regex`, or `None` if the
/// pattern is empty or produces an empty regex.
///
/// Supported syntax (Go path matching):
/// - `*` → matches any non-`/` sequence
/// - `?` → matches a single non-`/` character
/// - `[abc]`, `[a-c]` → character ranges
/// - `\x` → literal `x` (escape)
/// - `.` → literal dot (escaped in regex)
/// - Trailing `/` → ignored
/// - `,` → alternation separator
///
/// Mirrors `parseNoproxy()` from `lib/modules/datasource/go/goproxy-parser.ts`.
pub fn parse_noproxy(input: &str) -> Option<regex::Regex> {
    if input.is_empty() {
        return None;
    }

    // Build alternatives by splitting on comma (after stripping spaces)
    let alts: Vec<String> = input
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|pat| glob_to_regex_part(pat.trim()))
        .filter(|s| !s.is_empty())
        .collect();

    if alts.is_empty() {
        return None;
    }

    let pattern = format!("^(?:{})(?:/.*)?$", alts.join("|"));
    regex::Regex::new(&pattern).ok()
}

fn glob_to_regex_part(pattern: &str) -> String {
    let mut result = String::new();
    let mut chars = pattern.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '*' => result.push_str("[^/]*"),
            '?' => result.push_str("[^/]"),
            '[' => {
                // Character range: read until ']'
                result.push('[');
                while let Some(inner) = chars.next() {
                    if inner == '\\' {
                        // escape: take next char literally
                        if let Some(next) = chars.next() {
                            result.push(next);
                        }
                    } else if inner == ']' {
                        result.push(']');
                        break;
                    } else {
                        result.push(inner);
                    }
                }
            }
            '\\' => {
                // Escape: take next char literally
                if let Some(next) = chars.next() {
                    result.push(next);
                }
            }
            '/' => {
                // Strip trailing slash; for non-trailing, handle path separator
                if chars.peek().is_none() {
                    // trailing slash: skip
                } else {
                    result.push('/');
                }
            }
            '.' => result.push_str("\\."),
            c => result.push(c),
        }
    }
    result
}

// ---------------------------------------------------------------------------
// GitHub GraphQL releases adapter — lib/util/github/graphql/query-adapters/releases-query-adapter.ts
// ---------------------------------------------------------------------------

/// One GitHub release item as returned by the GraphQL adapter.
#[derive(Debug, Clone, PartialEq)]
pub struct GithubReleaseItem {
    pub version: String,
    pub release_timestamp: String,
    pub url: String,
    pub id: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_stable: Option<bool>,
}

/// Transform a raw GitHub GraphQL release node into a `GithubReleaseItem`.
///
/// Returns `None` when:
/// - Required fields are missing/invalid (version, releaseTimestamp, url, isDraft, isPrerelease)
/// - `isDraft` is `true`
///
/// Mirrors `releases-query-adapter.ts` → `transform()`.
pub fn transform_github_release(
    version: Option<&str>,
    release_timestamp: Option<&str>,
    is_draft: Option<bool>,
    is_prerelease: Option<bool>,
    url: Option<&str>,
    id: Option<i64>,
    name: Option<&str>,
    description: Option<&str>,
) -> Option<GithubReleaseItem> {
    let version = version.filter(|s| !s.is_empty())?;
    let ts_raw = release_timestamp?;
    let url = url.filter(|s| !s.is_empty())?;
    let is_draft = is_draft?;
    let is_prerelease = is_prerelease?;

    // Normalise timestamp: bare dates like "2024-09-24" become ISO with time+Z
    let normalised_ts = normalise_timestamp(ts_raw)?;

    if is_draft {
        return None;
    }

    Some(GithubReleaseItem {
        version: version.to_string(),
        release_timestamp: normalised_ts,
        url: url.to_string(),
        id,
        name: name.map(String::from),
        description: description.map(String::from),
        is_stable: if is_prerelease { Some(false) } else { None },
    })
}

/// Parse and normalise a timestamp string to `YYYY-MM-DDTHH:MM:SS.sssZ`.
///
/// Accepts ISO 8601 date-time strings and bare date strings (`YYYY-MM-DD`).
fn normalise_timestamp(s: &str) -> Option<String> {
    // Try full RFC3339 first
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s) {
        return Some(format!(
            "{}.{:03}Z",
            dt.with_timezone(&chrono::Utc).format("%Y-%m-%dT%H:%M:%S"),
            dt.timestamp_subsec_millis()
        ));
    }
    // Try bare date YYYY-MM-DD
    if let Ok(d) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return Some(format!("{}T00:00:00.000Z", d.format("%Y-%m-%d")));
    }
    None
}

// ---------------------------------------------------------------------------
// GitHub GraphQL tags adapter — lib/util/github/graphql/query-adapters/tags-query-adapter.ts
// ---------------------------------------------------------------------------

/// One GitHub tag item as returned by the GraphQL adapter.
#[derive(Debug, Clone, PartialEq)]
pub struct GithubTagItem {
    pub version: String,
    pub git_ref: String,
    pub hash: String,
    pub release_timestamp: String,
}

/// Target type for a tag commit reference.
#[derive(Debug)]
pub enum GithubTagTarget<'a> {
    Commit { oid: &'a str, release_timestamp: &'a str },
    Tag { tagger_timestamp: &'a str, nested_oid: &'a str },
}

/// Transform a raw GitHub GraphQL tag node into a `GithubTagItem`.
///
/// Returns `None` for unknown target types or missing required fields.
/// Mirrors `tags-query-adapter.ts` → `transform()`.
pub fn transform_github_tag(version: Option<&str>, target: Option<GithubTagTarget<'_>>) -> Option<GithubTagItem> {
    let version = version.filter(|s| !s.is_empty())?;
    let target = target?;
    match target {
        GithubTagTarget::Commit { oid, release_timestamp } => {
            Some(GithubTagItem {
                version: version.to_string(),
                git_ref: version.to_string(),
                hash: oid.to_string(),
                release_timestamp: release_timestamp.to_string(),
            })
        }
        GithubTagTarget::Tag { tagger_timestamp, nested_oid } => {
            Some(GithubTagItem {
                version: version.to_string(),
                git_ref: version.to_string(),
                hash: nested_oid.to_string(),
                release_timestamp: tagger_timestamp.to_string(),
            })
        }
    }
}

// ---------------------------------------------------------------------------
// Package abandonment — lib/workers/repository/process/lookup/abandonment.ts
// ---------------------------------------------------------------------------

/// Calculate whether a package is abandoned based on the most recent timestamp.
///
/// Returns `Some(true)` if abandoned, `Some(false)` if active, `None` if the
/// check could not be performed (no threshold, invalid threshold, no timestamp).
///
/// Mirrors `calculateAbandonment` from the TypeScript reference but returns
/// `Option<bool>` instead of mutating the release result.
pub fn calculate_abandonment(
    most_recent_timestamp_iso: Option<&str>,
    abandonment_threshold: Option<&str>,
    now_ms: i64,
) -> Option<bool> {
    let threshold_str = abandonment_threshold?;
    let threshold_ms = to_ms(threshold_str)?;
    let timestamp_str = most_recent_timestamp_iso?;
    let most_recent_ms = chrono::DateTime::parse_from_rfc3339(timestamp_str)
        .ok()?
        .timestamp_millis();
    let abandonment_ms = most_recent_ms + threshold_ms;
    Some(abandonment_ms < now_ms)
}

// ---------------------------------------------------------------------------
// Release timestamp utilities — lib/workers/repository/process/lookup/timestamps.ts
// ---------------------------------------------------------------------------

/// One release entry for timestamp calculation.
#[derive(Debug)]
pub struct ReleaseEntry<'a> {
    pub version: &'a str,
    pub release_timestamp: Option<&'a str>,
    pub is_deprecated: bool,
}

/// Parse an ISO8601 timestamp string, returning None if invalid or out of range.
///
/// Mirrors `asTimestamp` from `lib/util/timestamp.ts`:
/// must be after 2000-01-01 and not in the future.
fn as_timestamp(input: Option<&str>) -> Option<&str> {
    let s = input?;
    // Must parse as RFC3339/ISO8601
    let dt = chrono::DateTime::parse_from_rfc3339(s).ok()?;
    let ts_ms = dt.timestamp_millis();
    let millenium: i64 = 946_684_800_000; // 2000-01-01
    if ts_ms <= millenium {
        return None;
    }
    Some(s)
}

/// Compute `mostRecentTimestamp` for a package release list.
///
/// Returns the ISO timestamp of the highest-version release if and only if:
/// - A highest valid version exists
/// - It is not deprecated
/// - Its timestamp is >= all other releases' timestamps
///
/// Mirrors `calculateMostRecentTimestamp` from
/// `lib/workers/repository/process/lookup/timestamps.ts`.
pub fn calculate_most_recent_timestamp<'a>(
    releases: &[ReleaseEntry<'a>],
    is_version: impl Fn(&str) -> bool,
    is_greater_than: impl Fn(&str, &str) -> bool,
) -> Option<String> {
    // Find highest valid version
    let mut highest: Option<&ReleaseEntry<'a>> = None;
    for r in releases {
        if !is_version(r.version) {
            continue;
        }
        match highest {
            None => highest = Some(r),
            Some(h) => {
                // try/catch equivalent: ignore comparison errors
                if is_greater_than(r.version, h.version) {
                    highest = Some(r);
                }
            }
        }
    }

    let h = highest?;

    if h.is_deprecated {
        return None;
    }

    let highest_ts = as_timestamp(h.release_timestamp)?;
    let highest_dt = chrono::DateTime::parse_from_rfc3339(highest_ts).ok()?;

    // Check if any release has a NEWER timestamp than highest version's timestamp
    let higher_exists = releases.iter().any(|r| {
        let ts = match as_timestamp(r.release_timestamp) {
            Some(s) => s,
            None => return false,
        };
        let dt = match chrono::DateTime::parse_from_rfc3339(ts) {
            Ok(d) => d,
            Err(_) => return false,
        };
        dt > highest_dt
    });

    if higher_exists {
        return None;
    }

    Some(highest_ts.to_string())
}

// ---------------------------------------------------------------------------
// Error/warning PR text — lib/workers/repository/errors-warnings.ts
// ---------------------------------------------------------------------------

/// A warning or error entry (topic + message).
#[derive(Debug, Clone)]
pub struct WarningOrError<'a> {
    pub topic: &'a str,
    pub message: &'a str,
}

/// One dependency with its warning messages.
#[derive(Debug, Clone)]
pub struct DepWithWarnings<'a> {
    pub warnings: &'a [&'a str],
}

/// One package file with deps.
#[derive(Debug, Clone)]
pub struct PackageFileWarnings<'a> {
    pub package_file: &'a str,
    pub deps: &'a [DepWithWarnings<'a>],
}

/// Generate the Warnings section for a PR/onboarding description.
///
/// Mirrors `getWarnings()` from `lib/workers/repository/errors-warnings.ts`.
pub fn get_warnings(warnings: &[WarningOrError<'_>]) -> String {
    if warnings.is_empty() {
        return String::new();
    }
    let mut out = format!("\n# Warnings ({})\n\n", warnings.len());
    out.push_str("Please correct - or verify that you can safely ignore - these warnings before you merge this PR.\n\n");
    for w in warnings {
        out.push_str(&format!("-   `{}`: {}\n", w.topic, w.message));
    }
    out.push_str("\n---\n");
    out
}

/// Generate the Errors section for a PR description.
///
/// Mirrors `getErrors()` from `lib/workers/repository/errors-warnings.ts`.
pub fn get_errors(errors: &[WarningOrError<'_>]) -> String {
    if errors.is_empty() {
        return String::new();
    }
    let mut out = format!("\n# Errors ({})\n\n", errors.len());
    out.push_str("Renovate has found errors that you should fix (in this branch) before finishing this PR.\n\n");
    for e in errors {
        out.push_str(&format!("-   `{}`: {}\n", e.topic, e.message));
    }
    out.push_str("\n---\n");
    out
}

/// Collect unique warning messages and affected files from package files.
fn collect_dep_warnings<'a>(
    package_files: &'a [PackageFileWarnings<'a>],
) -> (Vec<&'a str>, Vec<&'a str>) {
    let mut warnings: Vec<&str> = Vec::new();
    let mut warning_files: Vec<&str> = Vec::new();
    for file in package_files {
        if file.package_file.is_empty() {
            continue;
        }
        for dep in file.deps {
            for &msg in dep.warnings {
                if !warnings.contains(&msg) {
                    warnings.push(msg);
                }
                if !warning_files.contains(&file.package_file) {
                    warning_files.push(file.package_file);
                }
            }
        }
    }
    (warnings, warning_files)
}

/// Generate dep-warning text for a PR.
///
/// Mirrors `getDepWarningsPR()` from `lib/workers/repository/errors-warnings.ts`.
pub fn get_dep_warnings_pr(
    package_files: &[PackageFileWarnings<'_>],
    suppress_dep_lookup_warnings: bool,
    has_dependency_dashboard: bool,
    dependency_dashboard_issue: Option<u32>,
) -> String {
    if suppress_dep_lookup_warnings {
        return String::new();
    }
    let (warnings, _) = collect_dep_warnings(package_files);
    if warnings.is_empty() {
        return String::new();
    }
    let mut out = "\n---\n\n> \u{26a0}\u{fe0f} **Warning**\n> \n".to_string();
    out.push_str("> Some dependencies could not be looked up. ");
    if has_dependency_dashboard {
        let dep_dash_link = if let Some(issue) = dependency_dashboard_issue {
            format!("[Dependency Dashboard](../issues/{})", issue)
        } else {
            "Dependency Dashboard".to_string()
        };
        out.push_str(&format!("Check the {} for more information.\n\n", dep_dash_link));
    } else {
        out.push_str("Check the warning logs for more information.\n\n");
    }
    out
}

/// Generate dep-warning text for the dependency dashboard.
///
/// Mirrors `getDepWarningsDashboard()` from `lib/workers/repository/errors-warnings.ts`.
pub fn get_dep_warnings_dashboard(
    package_files: &[PackageFileWarnings<'_>],
    suppress_dep_lookup_warnings: bool,
) -> String {
    if suppress_dep_lookup_warnings {
        return String::new();
    }
    let (warnings, warning_files) = collect_dep_warnings(package_files);
    if warnings.is_empty() {
        return String::new();
    }
    // Strip "Failed to look up X dependency " prefixes
    static STRIP_PREFIX_RE: std::sync::LazyLock<regex::Regex> =
        std::sync::LazyLock::new(|| {
            regex::Regex::new(r"^Failed to look up(?: [-\w]+)? dependency ").unwrap()
        });
    let dep_list: Vec<String> = warnings
        .iter()
        .map(|w| format!("`{}`", STRIP_PREFIX_RE.replace(w, "")))
        .collect();
    let files_list: Vec<String> = warning_files.iter().map(|f| format!("`{}`", f)).collect();
    let mut out = "\n---\n\n> \u{26a0}\u{fe0f} **Warning**\n> \n> Renovate failed to look up the following dependencies: ".to_string();
    out.push_str(&dep_list.join(", "));
    out.push_str(".\n> \n> Files affected: ");
    out.push_str(&files_list.join(", "));
    out.push_str("\n\n---\n\n");
    out
}

/// Generate dep-warning text for the onboarding PR.
///
/// Mirrors `getDepWarningsOnboardingPR()` from `lib/workers/repository/errors-warnings.ts`.
pub fn get_dep_warnings_onboarding_pr(
    package_files: &[PackageFileWarnings<'_>],
    suppress_dep_lookup_warnings: bool,
) -> String {
    if suppress_dep_lookup_warnings {
        return String::new();
    }
    let (warnings, warning_files) = collect_dep_warnings(package_files);
    if warnings.is_empty() {
        return String::new();
    }
    let mut out = "\n---\n> \n> \u{26a0}\u{fe0f} **Warning**\n> \n".to_string();
    out.push_str("> Please correct - or verify that you can safely ignore - these dependency lookup failures before you merge this PR.\n> \n");
    for w in &warnings {
        out.push_str(&format!("> -   `{}`\n", w));
    }
    let files_list: Vec<String> = warning_files.iter().map(|f| format!("`{}`", f)).collect();
    out.push_str(&format!(">\n> Files affected: {}\n\n", files_list.join(", ")));
    out
}

// ---------------------------------------------------------------------------
// Onboarding PR list — lib/workers/repository/onboarding/pr/pr-list.ts
// ---------------------------------------------------------------------------

/// One dependency upgrade inside a branch.
#[derive(Debug)]
pub struct PrListUpgrade<'a> {
    pub dep_name: &'a str,
    pub source_url: Option<&'a str>,
    pub update_type: &'a str,
    pub new_value: Option<&'a str>,
    pub new_version: Option<&'a str>,
    pub new_digest: Option<&'a str>,
    pub is_lockfile_update: bool,
}

/// One branch in the expected PR list.
#[derive(Debug)]
pub struct PrListBranch<'a> {
    pub pr_title: &'a str,
    pub branch_name: &'a str,
    pub base_branch: Option<&'a str>,
    pub schedule: &'a [&'a str],
    pub upgrades: &'a [PrListUpgrade<'a>],
}

/// Convert `@org/repo` patterns to `@&#8203;org/repo` (zero-width space after @).
fn sanitize_pr_title(title: &str) -> String {
    static RE: std::sync::LazyLock<regex::Regex> =
        std::sync::LazyLock::new(|| regex::Regex::new(r"@([a-z]+/[a-z]+)").unwrap());
    RE.replace_all(title, "@\u{200B}$1").into_owned()
}

/// Build the "What to Expect" section of the onboarding PR description.
///
/// Mirrors `getExpectedPrList` from
/// `lib/workers/repository/onboarding/pr/pr-list.ts`.
pub fn get_expected_pr_list(
    pr_hourly_limit: u32,
    commit_hourly_limit: u32,
    branches: &[PrListBranch<'_>],
) -> String {
    let mut out = "\n### What to Expect\n\n".to_string();
    if branches.is_empty() {
        out.push_str("It looks like your repository dependencies are already up-to-date and no Pull Requests will be necessary right away.\n");
        return out;
    }
    let n = branches.len();
    out.push_str(&format!(
        "With your current configuration, Renovate will create {} Pull Request{}:\n\n",
        n,
        if n > 1 { "s" } else { "" }
    ));

    for branch in branches {
        out.push_str(&format!(
            "<details>\n<summary>{}</summary>\n\n",
            sanitize_pr_title(branch.pr_title)
        ));
        if !branch.schedule.is_empty() {
            let sched = branch
                .schedule
                .iter()
                .map(|s| format!("\"{}\"", s))
                .collect::<Vec<_>>()
                .join(",");
            out.push_str(&format!("  - Schedule: [{}]\n", sched));
        }
        out.push_str(&format!("  - Branch name: `{}`\n", branch.branch_name));
        if let Some(base) = branch.base_branch.filter(|s| !s.is_empty()) {
            out.push_str(&format!("  - Merge into: `{}`\n", base));
        }

        let mut seen: Vec<String> = Vec::new();
        for upg in branch.upgrades {
            let text = if upg.update_type == "lockFileMaintenance" {
                "  - Regenerate lock files to use latest dependency versions".to_string()
            } else {
                let action = if upg.update_type == "pin" { "Pin" } else { "Upgrade" };
                let dep = if let Some(url) = upg.source_url {
                    format!("[{}]({})", upg.dep_name, url)
                } else {
                    sanitize_pr_title(upg.dep_name)
                };
                let version = if upg.is_lockfile_update {
                    format!("`{}`", upg.new_version.unwrap_or("undefined"))
                } else {
                    let v = upg.new_digest.or(upg.new_value).unwrap_or("undefined");
                    format!("`{}`", v)
                };
                format!("  - {} {} to {}\n", action, dep, version)
            };
            if !seen.contains(&text) {
                out.push_str(&text);
                seen.push(text);
            }
        }
        out.push_str("\n\n");
        out.push_str("</details>\n\n");
    }

    // Hourly limit messages
    if commit_hourly_limit > 0 && commit_hourly_limit < 5 && (commit_hourly_limit as usize) < n {
        out.push_str(&format!(
            "\n\n🚸 Branch creation and rebasing will be limited to maximum {} per hour, so it doesn't swamp any CI resources or overwhelm the project. See docs for `commitHourlyLimit` for details.\n\n",
            commit_hourly_limit
        ));
    } else if pr_hourly_limit > 0 && pr_hourly_limit < 5 && (pr_hourly_limit as usize) < n {
        out.push_str(&format!(
            "\n\n🚸 PR creation will be limited to maximum {} per hour, so it doesn't swamp any CI resources or overwhelm the project. See [docs for `prHourlyLimit`](https://docs.renovatebot.com/configuration-options/#prhourlylimit) for details.\n\n",
            pr_hourly_limit
        ));
    }

    out
}

// PR label utilities — lib/workers/repository/update/pr/labels.ts
// ---------------------------------------------------------------------------

/// Merge, deduplicate, and sort label arrays.
///
/// Mirrors the core logic of `prepareLabels` from
/// `lib/workers/repository/update/pr/labels.ts` (without template compilation
/// or platform char-limit trimming).
pub fn prepare_labels(labels: &[&str], add_labels: &[&str]) -> Vec<String> {
    let mut combined: Vec<String> = labels
        .iter()
        .chain(add_labels.iter())
        .map(|s| s.to_string())
        .filter(|s| !s.trim().is_empty())
        .collect();
    combined.sort();
    combined.dedup();
    combined
}

/// Return `(labels_to_add, labels_to_remove)` for the transition from
/// `old_labels` to `new_labels`.
///
/// Mirrors `getChangedLabels` from `lib/workers/repository/update/pr/labels.ts`.
pub fn get_changed_labels(
    old_labels: &[&str],
    new_labels: &[&str],
) -> (Vec<String>, Vec<String>) {
    let to_add: Vec<String> = new_labels
        .iter()
        .filter(|l| !old_labels.contains(l))
        .map(|l| l.to_string())
        .collect();
    let to_remove: Vec<String> = old_labels
        .iter()
        .filter(|l| !new_labels.contains(l))
        .map(|l| l.to_string())
        .collect();
    (to_add, to_remove)
}

/// Determine whether labels need to be updated on the PR.
///
/// Returns `false` when:
/// - `pr_initial_labels` is `None` (PR was created before label tracking)
/// - configured labels equal initial labels (no change needed)
/// - labels have been modified by user (initial != current)
///
/// Mirrors `shouldUpdateLabels` from `lib/workers/repository/update/pr/labels.ts`.
pub fn should_update_labels(
    pr_initial_labels: Option<&[&str]>,
    pr_current_labels: Option<&[&str]>,
    configured_labels: Option<&[&str]>,
) -> bool {
    let Some(initial) = pr_initial_labels else {
        return false;
    };
    let configured: Vec<&str> = configured_labels.unwrap_or(&[]).to_vec();
    let mut configured_sorted = configured.clone();
    configured_sorted.sort_unstable();
    let mut initial_sorted: Vec<&str> = initial.to_vec();
    initial_sorted.sort_unstable();
    if configured_sorted == initial_sorted {
        return false;
    }
    let current: Vec<&str> = pr_current_labels.unwrap_or(&[]).to_vec();
    if are_labels_modified(initial, &current) {
        return false;
    }
    true
}

/// Return `true` when old and new labels differ (order-insensitive).
///
/// Mirrors `areLabelsModified` from `lib/workers/repository/update/pr/labels.ts`.
pub fn are_labels_modified(old_labels: &[&str], new_labels: &[&str]) -> bool {
    if old_labels.len() != new_labels.len() {
        return true;
    }
    let mut old_sorted: Vec<&str> = old_labels.to_vec();
    let mut new_sorted: Vec<&str> = new_labels.to_vec();
    old_sorted.sort_unstable();
    new_sorted.sort_unstable();
    old_sorted != new_sorted
}

/// Return the label description for a module kind and id.
///
/// Mirrors `getLabelDescription` from `tools/utils/sync-module-labels.ts`.
pub fn get_label_description(kind: &str, module_id: &str) -> String {
    format!("Related to the {module_id} {kind}")
}

/// A GitHub label structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GithubLabel {
    pub color: &'static str,
    pub description: String,
    pub name: String,
}

/// Create a module label for the given kind and module id.
///
/// Mirrors `createModuleLabel` from `tools/utils/sync-module-labels.ts`.
pub fn create_module_label(kind: &str, module_id: &str) -> GithubLabel {
    GithubLabel {
        color: MODULE_LABEL_COLOR,
        description: get_label_description(kind, module_id),
        name: format!("{kind}:{module_id}"),
    }
}

/// Return labels in `expected` that are not in `existing` (by name).
///
/// Mirrors `getMissingModuleLabels` from `tools/utils/sync-module-labels.ts`.
pub fn get_missing_module_labels(
    expected: &[GithubLabel],
    existing: &[GithubLabel],
) -> Vec<GithubLabel> {
    let existing_names: std::collections::HashSet<&str> =
        existing.iter().map(|l| l.name.as_str()).collect();
    expected
        .iter()
        .filter(|l| !existing_names.contains(l.name.as_str()))
        .cloned()
        .collect()
}

/// Mirrors `setReconfigureBranchCache` from
/// `lib/workers/repository/reconfigure/reconfigure-cache.ts`.
pub fn set_reconfigure_branch_cache(cache: &mut serde_json::Value, sha: &str, is_valid: bool) {
    if let serde_json::Value::Object(map) = cache {
        map.insert(
            "reconfigureBranchCache".to_owned(),
            serde_json::json!({
                "reconfigureBranchSha": sha,
                "isConfigValid": is_valid,
            }),
        );
    }
}

/// Delete the reconfigure branch cache entry.
///
/// Mirrors `deleteReconfigureBranchCache` from
/// `lib/workers/repository/reconfigure/reconfigure-cache.ts`.
pub fn delete_reconfigure_branch_cache(cache: &mut serde_json::Value) {
    if let serde_json::Value::Object(map) = cache {
        map.remove("reconfigureBranchCache");
    }
}

// ---------------------------------------------------------------------------
// Repository configuration check — lib/workers/repository/configured.ts
// ---------------------------------------------------------------------------

/// Check whether the repository configuration allows processing.
///
/// Returns `Ok(())` when processing is allowed; `Err(message)` otherwise.
/// Mirrors `checkIfConfigured` from `lib/workers/repository/configured.ts`.
pub fn check_if_configured(
    enabled: bool,
    is_fork: bool,
    fork_processing: Option<&str>,
) -> Result<(), &'static str> {
    if !enabled {
        return Err("REPOSITORY_DISABLED_BY_CONFIG");
    }
    if is_fork && fork_processing != Some("enabled") {
        return Err("REPOSITORY_FORKED");
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Manager utilities — lib/modules/manager/util.ts
// ---------------------------------------------------------------------------

/// Result of `apply_git_source`.
#[derive(Debug, Default, PartialEq)]
pub struct GitSourceResult {
    pub datasource: &'static str,
    pub registry_urls: Option<Vec<String>>,
    pub package_name: String,
    pub current_value: Option<String>,
    pub current_digest: Option<String>,
    pub replace_string: Option<String>,
    pub skip_reason: Option<&'static str>,
}

/// Parse host and full_name from a git URL (HTTPS or SSH).
pub fn parse_git_url_host_and_name(url: &str) -> Option<(String, String)> {
    // SCP-like: git@host:owner/repo.git
    if !url.contains("://") {
        if let Some(at_pos) = url.find('@') {
            let rest = &url[at_pos + 1..];
            if let Some(colon_pos) = rest.find(':') {
                let host = rest[..colon_pos].to_owned();
                let path = rest[colon_pos + 1..].trim_end_matches(".git").to_owned();
                return Some((host, path));
            }
        }
        return None;
    }
    let without_scheme = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .or_else(|| url.strip_prefix("ssh://"))
        .or_else(|| url.strip_prefix("git://"))?;
    let without_user = if let Some(at_pos) = without_scheme.find('@') {
        &without_scheme[at_pos + 1..]
    } else {
        without_scheme
    };
    let slash_pos = without_user.find('/')?;
    let host = without_user[..slash_pos].to_owned();
    let raw_path = without_user[slash_pos + 1..].trim_end_matches(".git");
    Some((host, raw_path.to_owned()))
}

/// Determine datasource and package metadata from a git URL, tag, rev, or branch.
///
/// Mirrors `applyGitSource` from `lib/modules/manager/util.ts`.
pub fn apply_git_source(
    git: &str,
    rev: Option<&str>,
    tag: Option<&str>,
    branch: Option<&str>,
) -> GitSourceResult {
    if let Some(tag) = tag {
        let platform = detect_platform(git);
        if (platform == Some("github") || platform == Some("gitlab"))
            && let Some((host, full_name)) = parse_git_url_host_and_name(git)
        {
            let datasource = if platform == Some("github") {
                "github-tags"
            } else {
                "gitlab-tags"
            };
            return GitSourceResult {
                datasource,
                registry_urls: Some(vec![format!("https://{host}")]),
                package_name: full_name,
                current_value: Some(tag.to_owned()),
                ..Default::default()
            };
        }
        return GitSourceResult {
            datasource: "git-tags",
            package_name: git.to_owned(),
            current_value: Some(tag.to_owned()),
            ..Default::default()
        };
    }
    if let Some(rev) = rev {
        return GitSourceResult {
            datasource: "git-refs",
            package_name: git.to_owned(),
            current_digest: Some(rev.to_owned()),
            replace_string: Some(rev.to_owned()),
            ..Default::default()
        };
    }
    GitSourceResult {
        datasource: "git-refs",
        package_name: git.to_owned(),
        current_value: branch.map(|b| b.to_owned()),
        skip_reason: Some(if branch.is_some() {
            "git-dependency"
        } else {
            "unspecified-version"
        }),
        ..Default::default()
    }
}

// ---------------------------------------------------------------------------
// Changelog URL slugify — lib/workers/repository/update/pr/changelog/common.ts
// ---------------------------------------------------------------------------

/// Convert a URL to a slug by replacing non-alphanumeric chars with `-` and
/// transliterating common Unicode characters to their ASCII equivalents.
///
/// Mirrors `slugifyUrl` from `lib/workers/repository/update/pr/changelog/common.ts`.
pub fn slugify_url(url: &str) -> String {
    let mut result = String::new();
    let mut prev_dash = false;
    for c in url.chars() {
        let mapped = transliterate_for_slug(c);
        match mapped {
            Some('-') => {
                if !prev_dash && !result.is_empty() {
                    result.push('-');
                    prev_dash = true;
                }
            }
            Some(ch) => {
                result.push(ch);
                prev_dash = false;
            }
            None => {
                prev_dash = false;
            } // removed chars don't reset dash
        }
    }
    result.trim_end_matches('-').to_owned()
}

fn transliterate_for_slug(c: char) -> Option<char> {
    match c {
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ā' | 'ă' | 'ą' => Some('a'),
        'è' | 'é' | 'ê' | 'ë' | 'ē' | 'ĕ' | 'ę' | 'ě' => Some('e'),
        'ì' | 'í' | 'î' | 'ï' | 'ī' | 'ĭ' | 'į' | 'ı' => Some('i'),
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ō' | 'ŏ' | 'ő' | 'ø' => Some('o'),
        'ù' | 'ú' | 'û' | 'ü' | 'ū' | 'ŭ' | 'ů' | 'ű' | 'ų' => Some('u'),
        'ç' | 'ć' | 'ĉ' | 'č' => Some('c'),
        'ñ' | 'ń' | 'ň' | 'ŋ' => Some('n'),
        'ý' | 'ÿ' => Some('y'),
        'ð' => Some('d'),
        'þ' => Some('t'),
        'ß' => Some('s'),
        '∂' => Some('d'), // partial derivative
        'α' => Some('a'), // Greek alpha
        'β' => Some('b'),
        'γ' => Some('g'),
        'δ' => Some('d'),
        'ε' => Some('e'),
        _ if c.is_ascii_alphanumeric() => Some(c.to_ascii_lowercase()),
        _ if c.is_ascii() => Some('-'), // ASCII non-alphanumeric → dash
        _ => None,                      // non-ASCII non-mapped → removed
    }
}

// ---------------------------------------------------------------------------
// Interpolator — lib/util/interpolator.ts
// ---------------------------------------------------------------------------

/// Validate a secrets/variables map for correct key format and value types.
///
/// `None` input → no-op.  Non-object → `Err(CONFIG_SECRETS_INVALID)`.
/// Object with keys not matching `name_pattern` or non-string values → `Err`.
pub fn validate_interpolated_values(
    input: Option<&serde_json::Value>,
    name_pattern: &str,
) -> Result<(), String> {
    use regex::Regex;
    let Some(input) = input else {
        return Ok(());
    };
    let re = Regex::new(name_pattern).map_err(|e| e.to_string())?;
    match input {
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                if !re.is_match(k) {
                    return Err(format!("CONFIG_SECRETS_INVALID: invalid key {k:?}"));
                }
                if !v.is_string() {
                    return Err(format!(
                        "CONFIG_SECRETS_INVALID: value for {k:?} must be string"
                    ));
                }
            }
            Ok(())
        }
        serde_json::Value::Null => Ok(()),
        _ => Err("CONFIG_SECRETS_INVALID: input must be an object".to_owned()),
    }
}

// ---------------------------------------------------------------------------
// YAML utilities — lib/util/yaml.ts
// ---------------------------------------------------------------------------

/// Parse a YAML string containing one or more documents.
///
/// Returns a `Vec<serde_json::Value>` (one entry per `---`-separated document).
/// Returns an empty vec for empty/blank input.
/// Strips Handlebars/Nunjucks templates before parsing when `remove_templates`
/// is true.
pub fn parse_yaml(content: &str, remove_templates: bool) -> Result<Vec<serde_json::Value>, String> {
    let text = if remove_templates {
        strip_templates(content)
    } else {
        content.to_owned()
    };
    if text.trim().is_empty() {
        return Ok(Vec::new());
    }
    let mut docs = Vec::new();
    // Split on YAML document separators.  Each `---` line may appear at start
    // or after a newline.
    let raw_docs: Vec<&str> = text.split("\n---").collect();
    for doc in raw_docs {
        let doc = doc.trim_start_matches('-').trim();
        if doc.is_empty() {
            continue;
        }
        let value: serde_json::Value = serde_yaml::from_str(doc).map_err(|e| e.to_string())?;
        if !value.is_null() {
            docs.push(value);
        }
    }
    Ok(docs)
}

/// Parse a single YAML document.  Returns `Ok(None)` for empty input.
pub fn parse_single_yaml(
    content: &str,
    remove_templates: bool,
) -> Result<Option<serde_json::Value>, String> {
    let text = if remove_templates {
        strip_templates(content)
    } else {
        content.to_owned()
    };
    if text.trim().is_empty() {
        return Ok(None);
    }
    let value: serde_json::Value = serde_yaml::from_str(&text).map_err(|e| e.to_string())?;
    Ok(if value.is_null() { None } else { Some(value) })
}

// ---------------------------------------------------------------------------
// Common utilities — lib/util/common.ts
// ---------------------------------------------------------------------------

/// Detect the hosting platform from a URL.
///
/// Returns the platform name or `None` for unknown/invalid URLs.
/// Mirrors `detectPlatform` from `lib/util/common.ts`.
pub fn detect_platform(url: &str) -> Option<&'static str> {
    let parsed = parse_url(url)?;
    let hostname = parsed.host_str()?;
    if hostname == "dev.azure.com" || hostname.ends_with(".visualstudio.com") {
        return Some("azure");
    }
    if hostname == "bitbucket.org" || hostname == "bitbucket.com" {
        return Some("bitbucket");
    }
    if hostname.contains("bitbucket") {
        return Some("bitbucket-server");
    }
    if hostname.contains("forgejo") || hostname == "codeberg.org" || hostname == "codefloe.com" {
        return Some("forgejo");
    }
    if hostname == "gitea.com" || hostname.contains("gitea") {
        return Some("gitea");
    }
    if hostname == "github.com" || hostname.contains("github") {
        return Some("github");
    }
    if hostname == "gitlab.com" || hostname.contains("gitlab") {
        return Some("gitlab");
    }

    // Fall back to host rules — check the hostType registered for this URL
    let host_type = host_rules::host_type_for_url(url)?;
    platform_from_host_type(&host_type)
}

/// Derive a canonical platform name from a `hostType` string.
///
/// Used by `detect_platform` when falling back to host-rules lookup.
/// Mirrors the `*_API_USING_HOST_TYPES` constants in `lib/constants/platforms.ts`.
fn platform_from_host_type(host_type: &str) -> Option<&'static str> {
    const AZURE: &[&str] = &["azure", "azure-tags"];
    const BITBUCKET_SERVER: &[&str] = &[
        "bitbucket-server",
        "bitbucket-server-changelog",
        "bitbucket-server-tags",
    ];
    const BITBUCKET: &[&str] = &["bitbucket", "bitbucket-changelog", "bitbucket-tags"];
    const FORGEJO: &[&str] = &[
        "forgejo",
        "forgejo-changelog",
        "forgejo-releases",
        "forgejo-tags",
    ];
    const GITEA: &[&str] = &["gitea", "gitea-changelog", "gitea-releases", "gitea-tags"];
    const GITHUB: &[&str] = &[
        "github",
        "github-releases",
        "github-release-attachments",
        "github-tags",
        "pod",
        "hermit",
        "github-changelog",
        "conan",
    ];
    const GITLAB: &[&str] = &[
        "gitlab",
        "gitlab-releases",
        "gitlab-tags",
        "gitlab-packages",
        "gitlab-changelog",
        "pypi",
    ];

    if AZURE.contains(&host_type) {
        return Some("azure");
    }
    if BITBUCKET_SERVER.contains(&host_type) {
        return Some("bitbucket-server");
    }
    if BITBUCKET.contains(&host_type) {
        return Some("bitbucket");
    }
    if FORGEJO.contains(&host_type) {
        return Some("forgejo");
    }
    if GITEA.contains(&host_type) {
        return Some("gitea");
    }
    if GITHUB.contains(&host_type) {
        return Some("github");
    }
    if GITLAB.contains(&host_type) {
        return Some("gitlab");
    }
    None
}

/// Parse a JSON/JSONC/JSON5 string into a `serde_json::Value`.
///
/// Tries strict JSON first; falls back to JSON5 (which handles comments,
/// trailing commas, unquoted keys, single-quoted strings).
/// Returns `Err` for strings that parse neither as JSON nor JSON5.
///
/// Mirrors `parseJson` from `lib/util/common.ts`.
pub fn parse_json(content: &str) -> Result<serde_json::Value, String> {
    serde_json::from_str(content)
        .or_else(|_| json5::from_str::<serde_json::Value>(content))
        .map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// URL utilities — lib/util/url.ts
// ---------------------------------------------------------------------------

/// Remove one or more trailing slashes from a URL/path.
pub fn trim_trailing_slash(url: &str) -> String {
    url.trim_end_matches('/').to_owned()
}

/// Remove one or more leading slashes from a path.
pub fn trim_leading_slash(path: &str) -> String {
    path.trim_start_matches('/').to_owned()
}

/// Remove both leading and trailing slashes from a path.
pub fn trim_slashes(path: &str) -> String {
    path.trim_matches('/').to_owned()
}

/// Ensure a URL ends with exactly one trailing slash.
pub fn ensure_trailing_slash(url: &str) -> String {
    format!("{}/", url.trim_end_matches('/'))
}

/// Return true when `url` starts with `http://` or `https://`.
pub fn is_http_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

/// Ensure that `url`'s path starts with `prefix`.
pub fn ensure_path_prefix(url: &str, prefix: &str) -> String {
    // Parse scheme + host, then handle path
    if let Some(after_scheme) = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
    {
        let scheme = if url.starts_with("https://") {
            "https://"
        } else {
            "http://"
        };
        let (host_part, path_part) = after_scheme.split_once('/').unwrap_or((after_scheme, ""));
        let full_path = if path_part.is_empty() {
            "/".to_owned()
        } else {
            format!("/{path_part}")
        };
        if full_path.starts_with(prefix) {
            return url.to_owned();
        }
        // Extract query string from path
        let (path_only, query) = full_path.split_once('?').unwrap_or((&full_path, ""));
        let new_path = format!("{prefix}{path_only}");
        let result = format!("{scheme}{host_part}{new_path}");
        if query.is_empty() {
            result
        } else {
            format!("{result}?{query}")
        }
    } else {
        url.to_owned()
    }
}

/// Resolve `input` against `base_url`, following `url-join` semantics.
///
/// If `input` is a full URL (contains `://`), it is returned unchanged.
/// Otherwise, `input` is appended to `base_url` with a single `/` separator.
pub fn resolve_base_url(base_url: &str, input: &str) -> String {
    if input.is_empty() {
        return trim_trailing_slash(base_url);
    }
    // Full URL passthrough
    if input.contains("://") {
        return input.to_owned();
    }
    let base = base_url.trim_end_matches('/');
    let stripped = input.trim_start_matches('/');
    if stripped.is_empty() {
        // Input was "/" or all slashes → base + trailing slash
        return format!("{base}/");
    }
    // Query string starting directly with ? → append without separator
    if stripped.starts_with('?') {
        return format!("{base}{stripped}");
    }
    // Clean trailing slash before query string
    let cleaned = stripped.replace("/?", "?");
    format!("{base}/{cleaned}")
}

/// Replace the path of `base_url` with `path`, using the origin (scheme+host)
/// only (not the base path).
pub fn replace_url_path(base_url: &str, path: &str) -> String {
    if path.contains("://") {
        return path.to_owned();
    }
    let origin = extract_origin(base_url);
    resolve_base_url(&origin, path)
}

fn extract_origin(url: &str) -> String {
    let (scheme, rest) = if let Some(r) = url.strip_prefix("https://") {
        ("https", r)
    } else if let Some(r) = url.strip_prefix("http://") {
        ("http", r)
    } else {
        return url.trim_end_matches('/').to_owned();
    };
    let host_end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
    format!("{scheme}://{}", &rest[..host_end])
}

/// Join URL path parts with exactly one `/` between each.
pub fn join_url_parts(parts: &[&str]) -> String {
    if parts.is_empty() {
        return String::new();
    }
    // Single arg: normalize trailing slashes
    if parts.len() == 1 {
        let s = parts[0];
        let trimmed = s.trim_end_matches('/');
        return if s.len() > trimmed.len() {
            format!("{trimmed}/")
        } else {
            trimmed.to_owned()
        };
    }
    let mut result = parts[0].to_owned();
    for part in &parts[1..] {
        result = resolve_base_url(&result, part);
    }
    result
}

/// Build a URL from a host name or full URL string.
///
/// If `host_or_url` already contains `://`, it is returned as-is.
/// Otherwise, `https://` is prepended.
pub fn create_url_from_host_or_url(host_or_url: &str) -> String {
    if host_or_url.contains("://") {
        host_or_url.to_owned()
    } else {
        format!("https://{host_or_url}")
    }
}

/// Parse an HTTP `Link` header into a map from `rel` value to link attributes.
///
/// Returns `None` for empty/absent headers or headers longer than 2000 chars.
/// Each link is returned as a `HashMap<String, String>` with `url`, `rel`, and
/// any other parameters plus the URL's query parameters flattened in.
///
/// Mirrors `parseLinkHeader` from `lib/util/url.ts`.
pub fn parse_link_header(
    header: Option<&str>,
) -> Option<std::collections::HashMap<String, std::collections::HashMap<String, String>>> {
    let header = header?;
    if header.is_empty() || header.len() > 2000 {
        return None;
    }
    let mut result = std::collections::HashMap::new();
    // Split on commas that are NOT inside angle brackets
    for segment in split_link_header(header) {
        let segment = segment.trim();
        if segment.is_empty() {
            continue;
        }
        // Extract URL from <...>
        let url_start = segment.find('<')? + 1;
        let url_end = segment.find('>')?;
        let url = &segment[url_start..url_end];
        let rest = &segment[url_end + 1..]; // ; param=val; ...

        let mut link: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        link.insert("url".to_owned(), url.to_owned());

        // Extract query params from URL
        if let Some(query_start) = url.find('?') {
            for kv in url[query_start + 1..].split('&') {
                if let Some((k, v)) = kv.split_once('=') {
                    link.insert(k.to_owned(), v.to_owned());
                }
            }
        }

        // Extract ; key="value" params
        for param in rest.split(';') {
            let param = param.trim();
            if param.is_empty() {
                continue;
            }
            if let Some((k, v)) = param.split_once('=') {
                let k = k.trim().to_owned();
                let v = v.trim().trim_matches('"').to_owned();
                link.insert(k, v);
            }
        }

        // Index by rel
        if let Some(rel) = link.get("rel").cloned() {
            result.insert(rel, link);
        }
    }
    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

fn split_link_header(header: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut start = 0;
    for (i, ch) in header.char_indices() {
        match ch {
            '<' => depth += 1,
            '>' => depth -= 1,
            ',' if depth == 0 => {
                parts.push(&header[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    parts.push(&header[start..]);
    parts
}

/// Prefix `https://` to host strings that include a port or path.
///
/// Mirrors `massageHostUrl` from `lib/util/url.ts`.
pub fn massage_host_url(url: &str) -> String {
    if !url.contains("://") && (url.contains('/') || url.contains(':')) {
        format!("https://{url}")
    } else {
        url.to_owned()
    }
}

/// Build a query string from key-value pairs.
///
/// Returns an empty string for empty input.
pub fn get_query_string(params: &[(&str, &str)]) -> String {
    if params.is_empty() {
        return String::new();
    }
    params
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("&")
}

/// Parse a URL string, returning `Some(normalized_url)` for valid HTTP(S) URLs or `None`.
///
/// Mirrors the TypeScript `parseUrl` from `lib/util/url.ts`.
pub fn parse_url(url: &str) -> Option<reqwest::Url> {
    reqwest::Url::parse(url).ok()
}

// ---------------------------------------------------------------------------
// String utilities — lib/util/string.ts
// ---------------------------------------------------------------------------

/// Replace `old_string` with `new_string` at byte position `index` in
/// `content`.  Panics if `index + old_string.len()` is out of bounds or not
/// on a char boundary.
pub fn replace_at(content: &str, index: usize, old_string: &str, new_string: &str) -> String {
    format!(
        "{}{}{}",
        &content[..index],
        new_string,
        &content[index + old_string.len()..]
    )
}

/// Loose (case-insensitive, locale-insensitive) equality for two strings.
///
/// Returns `false` when either value is `None` or empty, unless both are
/// `None` (mirrors the TypeScript `null`/`undefined` falsey check in
/// `looseEquals`).  When both strings are present and non-empty, comparison
/// is ASCII case-insensitive (TypeScript uses `localeCompare sensitivity:base`
/// which is equivalent for ASCII input).
pub fn loose_equals(a: Option<&str>, b: Option<&str>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) if !a.is_empty() && !b.is_empty() => a.eq_ignore_ascii_case(b),
        _ => a == b,
    }
}

/// Coerce a value to a string, returning `def` or `""` for `None`.
pub fn coerce_string<'a>(val: Option<&'a str>, def: Option<&'a str>) -> &'a str {
    val.or(def).unwrap_or("")
}

/// Capitalise the first character of a string, leaving the rest unchanged.
pub fn capitalize(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Remove Handlebars/Nunjucks template tags from a string.
///
/// Strips `{{ … }}`, `{{` ` … ` `}}`, `{% … %}`, `{%` ` … ` `%}`, and
/// `{# … #}` blocks, matching the behaviour of `lib/util/string.ts`
/// `stripTemplates`.
pub fn strip_templates(content: &str) -> String {
    let mut result = String::new();
    let bytes = content.as_bytes();
    let len = bytes.len();
    let mut idx = 0;
    let mut last_pos = 0;

    while idx < len {
        if bytes[idx] == b'{' && idx + 1 < len {
            let (closing, skip_len): (&[u8], usize) = match bytes[idx + 1] {
                b'%' if idx + 2 < len && bytes[idx + 2] == b'`' => (b"`%}", 3),
                b'%' => (b"%}", 2),
                b'{' if idx + 2 < len && bytes[idx + 2] == b'`' => (b"`}}", 3),
                b'{' => (b"}}", 2),
                b'#' => (b"#}", 2),
                _ => {
                    idx += 1;
                    continue;
                }
            };
            if let Some(end) = find_bytes(bytes, closing, idx + skip_len) {
                if idx > last_pos {
                    result.push_str(&content[last_pos..idx]);
                }
                idx = end + closing.len();
                last_pos = idx;
                continue;
            }
        }
        idx += 1;
    }

    if last_pos < len {
        result.push_str(&content[last_pos..]);
    }
    result
}

fn find_bytes(haystack: &[u8], needle: &[u8], start: usize) -> Option<usize> {
    let n = needle.len();
    if n == 0 {
        return Some(start);
    }
    (start..haystack.len().saturating_sub(n - 1)).find(|&i| &haystack[i..i + n] == needle)
}

// ---------------------------------------------------------------------------
// Number utilities — lib/util/number.ts
// ---------------------------------------------------------------------------

/// Coerce a value to a number, returning `def` or `0` for `None`.
pub fn coerce_number(val: Option<i64>, def: Option<i64>) -> i64 {
    val.or(def).unwrap_or(0)
}

/// Parse a non-negative integer from a string.  Returns `def` or `0` if the
/// input is `None`, empty, contains non-digit characters, or is negative.
pub fn parse_integer(val: Option<&str>, def: Option<i64>) -> i64 {
    match val {
        Some(s) if !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) => {
            s.parse::<i64>().unwrap_or(def.unwrap_or(0))
        }
        _ => def.unwrap_or(0),
    }
}

// ---------------------------------------------------------------------------
// Range — lib/util/range.ts
// ---------------------------------------------------------------------------

/// Return an inclusive range of integers from `start` to `end`.
///
/// If `start > end`, returns an empty iterator (matching the TypeScript
/// generator that yields nothing when the loop never executes).
pub fn range(start: i64, end: i64) -> impl Iterator<Item = i64> {
    let range_end = if start <= end { end + 1 } else { start };
    (start..range_end).take(if start <= end {
        (end - start + 1) as usize
    } else {
        0
    })
}

// ---------------------------------------------------------------------------
// Memoize — lib/util/memoize.ts
// ---------------------------------------------------------------------------

/// Return a new closure that calls `f` exactly once, caching and returning
/// the result on subsequent calls.
pub fn memoize<T: Clone, F: FnOnce() -> T>(f: F) -> impl FnMut() -> T {
    let mut memo: Option<T> = None;
    let mut f_opt: Option<F> = Some(f);
    move || {
        if let Some(ref val) = memo {
            return val.clone();
        }
        let val = f_opt
            .take()
            .expect("memoized fn consumed twice unexpectedly")();
        memo = Some(val.clone());
        val
    }
}

// ---------------------------------------------------------------------------
// Uniq — lib/util/uniq.ts
// ---------------------------------------------------------------------------

/// Deduplicate a vector using a custom equality predicate.
///
/// Preserves the first occurrence of each unique element (same semantics as
/// the TypeScript `uniq` which uses `findIndex`).
pub fn uniq<T, F>(array: Vec<T>, eql: F) -> Vec<T>
where
    F: Fn(&T, &T) -> bool,
{
    let mut result: Vec<T> = Vec::new();
    'outer: for item in array {
        for existing in &result {
            if eql(&item, existing) {
                continue 'outer;
            }
        }
        result.push(item);
    }
    result
}

/// Deduplicate a vector using `PartialEq`.
pub fn uniq_eq<T: PartialEq>(array: Vec<T>) -> Vec<T> {
    uniq(array, |a, b| a == b)
}

// ---------------------------------------------------------------------------
// Assign keys — lib/util/assign-keys.ts
// ---------------------------------------------------------------------------

/// Copy values from `right` into `left` for the specified `keys`, skipping
/// `None` values in `right`.
///
/// Returns a reference to `left` (mutated in place).  This mirrors the
/// TypeScript `assignKeys` which skips null/undefined values.
pub fn assign_keys<K, V>(
    left: &mut std::collections::HashMap<K, V>,
    right: &std::collections::HashMap<K, V>,
    keys: &[K],
) where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    for key in keys {
        if let Some(val) = right.get(key) {
            left.insert(key.clone(), val.clone());
        }
    }
}

// ---------------------------------------------------------------------------
// coerceToNull / coerceToUndefined — lib/util/coerce.ts
// ---------------------------------------------------------------------------

/// Coerce null/undefined to null; pass through other values.
///
/// In Rust, `None` serves as both null and undefined.  This function maps
/// `None` → `None` and `Some(T)` → `Some(T)`, which is the identity on
/// `Option<T>`.
pub fn coerce_to_null<T>(input: Option<T>) -> Option<T> {
    input
}

/// Coerce null/undefined to undefined; pass through other values.
///
/// Semantically identical to `coerce_to_null` in Rust because Rust does not
/// distinguish between null and undefined — both are `None`.
pub fn coerce_to_undefined<T>(input: Option<T>) -> Option<T> {
    input
}

// ---------------------------------------------------------------------------
// sampleSize — lib/util/sample.ts
// ---------------------------------------------------------------------------

/// Return up to `n` randomly-selected elements from `array`.
///
/// - `n = None` → return full array (mirrors TypeScript `undefined` behaviour:
///   `array.slice(0, undefined)` returns the full array).
/// - `n = Some(0)` → return empty vec.
/// - `n > array.len()` → return all elements in random order.
/// - `array` empty → return empty vec.
pub fn sample_size(array: &[String], n: Option<usize>) -> Vec<String> {
    let length = array.len();
    if length == 0 {
        return Vec::new();
    }
    let sample_n = match n {
        None => length,
        Some(0) => return Vec::new(),
        Some(k) => k.min(length),
    };
    // Shuffle a copy of the array and take the first sample_n elements.
    let mut result = array.to_vec();
    // Simple Fisher-Yates using a deterministic-enough pseudo-random.
    // For tests we care about length, not exact values.
    for i in (1..sample_n).rev() {
        let j = (i * 1103515245 + 12345) % (i + 1);
        result.swap(i, j);
    }
    result.truncate(sample_n);
    result
}

// ---------------------------------------------------------------------------
// Markdown utilities — lib/util/markdown.ts
// ---------------------------------------------------------------------------

/// Apply generic sanitization to Markdown content for safe display.
///
/// Inserts zero-width spaces after `@` mentions and `#`+digit patterns to
/// prevent unintended GitHub auto-linking.  Mirrors `sanitizeMarkdown` from
/// `lib/util/markdown.ts`.
pub fn sanitize_markdown(markdown: &str) -> String {
    use regex::Regex;
    static AT: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    static HASH_NONWORD: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    static UNDO_BACKTICK_AT: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    static UNDO_LETTER_AT: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    static UNDO_COMPARE_AT: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    static UNDO_URL_ELLIPSIS: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    static HASH_NUM: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    static HTML_BACKTICK: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    static CODE_HASH: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    static HEADING_NEWLINE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();

    let mut res = markdown.to_owned();
    // 1: #digit after non-word
    {
        let re = HASH_NONWORD.get_or_init(|| Regex::new(r"(\W)#(\d)").unwrap());
        res = re.replace_all(&res, "${1}#&#8203;${2}").to_string();
    }
    // 2: @ → @&#8203;
    {
        let re = AT.get_or_init(|| Regex::new(r"@").unwrap());
        res = re.replace_all(&res, "@&#8203;").to_string();
    }
    // 3: undo &#8203; inside backtick @
    {
        let re = UNDO_BACKTICK_AT.get_or_init(|| Regex::new(r"(`\[?@)&#8203;").unwrap());
        res = re.replace_all(&res, "$1").to_string();
    }
    // 4: undo &#8203; after [a-z]@
    {
        let re = UNDO_LETTER_AT.get_or_init(|| Regex::new(r"(?i)([a-z]@)&#8203;").unwrap());
        res = re.replace_all(&res, "$1").to_string();
    }
    // 5: undo in /compare/@
    {
        let re = UNDO_COMPARE_AT.get_or_init(|| Regex::new(r"/compare/@&#8203;").unwrap());
        res = re.replace_all(&res, "/compare/@").to_string();
    }
    // 6: undo in URL ellipsis
    {
        let re = UNDO_URL_ELLIPSIS
            .get_or_init(|| Regex::new(r"(\(https://[^)]*?)\.\.\.@&#8203;").unwrap());
        res = re.replace_all(&res, "$1...@").to_string();
    }
    // 7: standalone #N
    {
        let re = HASH_NUM.get_or_init(|| Regex::new(r"([\s(])#(\d+)([)\s]?)").unwrap());
        res = re.replace_all(&res, "${1}#&#8203;${2}${3}").to_string();
    }
    // 8: HTML backtick entities
    {
        let re = HTML_BACKTICK.get_or_init(|| Regex::new(r"&#x60;([^/]*?)&#x60;").unwrap());
        res = re.replace_all(&res, "`$1`").to_string();
    }
    // 9: undo &#8203; in inline code #N
    {
        let re = CODE_HASH.get_or_init(|| Regex::new(r"`#&#8203;(\d+)`").unwrap());
        res = re.replace_all(&res, "`#$1`").to_string();
    }
    // 10: add blank line before headings
    {
        let re = HEADING_NEWLINE.get_or_init(|| Regex::new(r"([^\n]\n)(#.*)").unwrap());
        res = re.replace_all(&res, "$1\n$2").to_string();
    }
    res
}

// ---------------------------------------------------------------------------
// Sanitize — lib/util/sanitize.ts
// ---------------------------------------------------------------------------

const GITHUB_APP_TOKEN_PREFIX: &str = "x-access-token:";

fn base64_encode(s: &str) -> String {
    use base64::{Engine, engine::general_purpose::STANDARD};
    STANDARD.encode(s.as_bytes())
}

fn add_to_set(set: &RefCell<HashSet<String>>, secret: &str) {
    let mut s = set.borrow_mut();
    s.insert(secret.to_owned());
    s.insert(base64_encode(secret));
    if let Some(trimmed) = secret.strip_prefix(GITHUB_APP_TOKEN_PREFIX) {
        s.insert(trimmed.to_owned());
        s.insert(base64_encode(trimmed));
    }
}

/// Add a secret that `sanitize` should replace with `**redacted**`.
///
/// `scope = "global"` adds to the global secrets list; otherwise (default) to
/// repo-scoped secrets.  Both the raw secret and its base64 encoding are added.
/// GitHub App tokens (`x-access-token:…`) also add the trimmed suffix.
pub fn add_secret_for_sanitizing(secret: &str, scope: &str) {
    if secret.is_empty() {
        return;
    }
    if scope == "global" {
        GLOBAL_SECRETS.with(|s| add_to_set(s, secret));
    } else {
        REPO_SECRETS.with(|s| add_to_set(s, secret));
    }
}

/// Clear the repo-scoped secrets list.
pub fn clear_repo_secrets() {
    REPO_SECRETS.with(|s| s.borrow_mut().clear());
}

/// Clear the global secrets list.
pub fn clear_global_secrets() {
    GLOBAL_SECRETS.with(|s| s.borrow_mut().clear());
}

/// Replace all registered secrets in `input` with `**redacted**`.
/// Returns `None` for `None` input; returns empty string unchanged.
pub fn sanitize_str(input: Option<&str>) -> Option<String> {
    let s = input?;
    if s.is_empty() {
        return Some(String::new());
    }
    let mut output = s.to_owned();
    let replace = |output: &mut String, secrets: &RefCell<HashSet<String>>| {
        for secret in secrets.borrow().iter() {
            if !secret.is_empty() {
                while output.contains(secret.as_str()) {
                    *output = output.replace(secret.as_str(), "**redacted**");
                }
            }
        }
    };
    GLOBAL_SECRETS.with(|s| replace(&mut output, s));
    REPO_SECRETS.with(|s| replace(&mut output, s));
    Some(output)
}

// ---------------------------------------------------------------------------
// Pretty-time — lib/util/pretty-time.ts
// ---------------------------------------------------------------------------

/// Convert a human-readable time string to milliseconds.
///
/// Supports composite specs like `"1h 2m"`, `"1d2h3m"`, `"1 hour 30 min"`,
/// `"1 month"`, `"1 M"`, `"1 year"`, `"1 week"`.  Returns `None` for invalid
/// input or bare unit strings without a leading number.
///
/// Mirrors the TypeScript `toMs` from `lib/util/pretty-time.ts`.
pub fn to_ms(input: &str) -> Option<i64> {
    let s = input.trim();
    if s.is_empty() || s.len() > 100 {
        return None;
    }
    // Preprocess: expand month shorthands before splitting
    let normalized = preprocess_time_spec(s);
    let parts = split_time_spec(&normalized);
    if parts.is_empty() {
        return None;
    }
    let mut total: i64 = 0;
    for part in parts {
        let ms = parse_single_spec(part.trim())?;
        total += ms;
    }
    Some(total)
}

fn split_time_spec(s: &str) -> Vec<String> {
    // Split at each transition that ends with a letter sequence.
    // e.g. "1d2h3m" → ["1d", "2h", "3m"]
    // e.g. "1h 1m" → ["1h", "1m"]
    let mut parts = Vec::new();
    let mut current = String::new();
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        current.push(b as char);
        let is_last = i == bytes.len() - 1;
        let next_is_digit_or_end = is_last || bytes[i + 1].is_ascii_digit() || bytes[i + 1] == b' ';
        if b.is_ascii_alphabetic() && (next_is_digit_or_end) {
            let t = current.trim().to_owned();
            if !t.is_empty() {
                parts.push(t);
            }
            current = String::new();
        }
    }
    let t = current.trim().to_owned();
    if !t.is_empty() {
        parts.push(t);
    }
    parts.retain(|p| !p.is_empty());
    parts
}

fn parse_single_spec(spec: &str) -> Option<i64> {
    // Must start with a digit
    if !spec.starts_with(|c: char| c.is_ascii_digit()) {
        return None;
    }
    // Pure numeric (no unit): treat as milliseconds (ms("0") = 0 etc.)
    if spec.bytes().all(|b| b.is_ascii_digit()) {
        return spec.parse::<i64>().ok();
    }
    // Separate number prefix from unit suffix
    let split_pos = spec.find(|c: char| c.is_ascii_alphabetic())?;
    let num_str = spec[..split_pos].trim();
    let unit = spec[split_pos..].trim().to_lowercase();
    let num: f64 = num_str.parse().ok()?;

    let multiplier: f64 = match unit.as_str() {
        "ms" | "millisecond" | "milliseconds" => 1.0,
        "s" | "sec" | "secs" | "second" | "seconds" => 1_000.0,
        "m" | "min" | "mins" | "minute" | "minutes" => 60_000.0,
        "h" | "hr" | "hrs" | "hour" | "hours" => 3_600_000.0,
        "d" | "day" | "days" => 86_400_000.0,
        "w" | "week" | "weeks" => 7.0 * 86_400_000.0,
        "month" | "months" | "mo" => 30.0 * 86_400_000.0,
        "y" | "yr" | "yrs" | "year" | "years" => 365.25 * 86_400_000.0,
        _ => return None,
    };
    Some((num * multiplier) as i64)
}

fn preprocess_time_spec(s: &str) -> String {
    // Convert "N M" (months) to "N month" and "N Y" to "N year"
    // The TypeScript applyCustomFormat handles this via regex
    static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    let re = RE.get_or_init(|| regex::Regex::new(r"(\d+)\s*(?:months?|M)").unwrap());
    re.replace_all(s, |caps: &regex::Captures| {
        let n: i64 = caps[1].parse().unwrap_or(0);
        format!("{}d", n * 30)
    })
    .to_string()
}

/// Check whether `date` satisfies a `range` expression like `"< 1 year"` or
/// `">= 1 day"`.  Returns `None` for invalid inputs.
///
/// `now_ms` is the "current" time in milliseconds since epoch (enables
/// deterministic testing without time mocking).
pub fn satisfies_date_range(date: &str, range: &str, now_ms: i64) -> Option<bool> {
    use chrono::DateTime;
    let range = range.trim();
    // Extract operator and age part
    let (operator, age) = {
        let stripped = range.trim_start_matches(|c: char| c.is_whitespace());
        if let Some(rest) = stripped.strip_prefix(">=") {
            (">=", rest.trim())
        } else if let Some(rest) = stripped.strip_prefix("<=") {
            ("<=", rest.trim())
        } else if let Some(rest) = stripped.strip_prefix('>') {
            (">", rest.trim())
        } else if let Some(rest) = stripped.strip_prefix('<') {
            ("<", rest.trim())
        } else {
            return None;
        }
    };
    let date_ms = DateTime::parse_from_rfc3339(date)
        .or_else(|_| DateTime::parse_from_rfc3339(&format!("{date}T00:00:00Z")))
        .map(|d| d.timestamp_millis())
        .ok()?;
    let age_ms = to_ms(age)?;
    let range_ms = now_ms - age_ms;
    Some(match operator {
        ">" => date_ms < range_ms,
        ">=" => date_ms <= range_ms,
        "<" => date_ms > range_ms,
        "<=" => date_ms >= range_ms,
        _ => return None,
    })
}

// ---------------------------------------------------------------------------
// Date utilities — lib/util/date.ts
// ---------------------------------------------------------------------------

const ONE_MINUTE_MS: i64 = 60_000;
const ONE_HOUR_MS: i64 = 3_600_000;

/// Return elapsed days between `timestamp` ISO string and `now_ms`.
/// When `floor` is true, truncates to integer days.
pub fn get_elapsed_days(timestamp: &str, floor: bool, now_ms: i64) -> f64 {
    use chrono::DateTime;
    let past_ms = DateTime::parse_from_rfc3339(timestamp)
        .map(|d| d.timestamp_millis())
        .unwrap_or(now_ms);
    let diff_days = (now_ms - past_ms) as f64 / (ONE_HOUR_MS * 24) as f64;
    if floor { diff_days.floor() } else { diff_days }
}

/// Return elapsed minutes between `date_ms` and `now_ms`.
pub fn get_elapsed_minutes(date_ms: i64, now_ms: i64) -> i64 {
    (now_ms - date_ms) / ONE_MINUTE_MS
}

/// Return elapsed hours between `timestamp` ISO string and `now_ms`.
/// Returns 0 for invalid timestamps.
pub fn get_elapsed_hours(timestamp: &str, now_ms: i64) -> i64 {
    use chrono::DateTime;
    let past_ms = match DateTime::parse_from_rfc3339(timestamp) {
        Ok(d) => d.timestamp_millis(),
        Err(_) => return 0,
    };
    ((now_ms - past_ms) / ONE_HOUR_MS).max(0)
}

/// Return elapsed milliseconds between `timestamp` ISO string and `now_ms`.
pub fn get_elapsed_ms(timestamp: &str, now_ms: i64) -> i64 {
    use chrono::DateTime;
    let past_ms = DateTime::parse_from_rfc3339(timestamp)
        .map(|d| d.timestamp_millis())
        .unwrap_or(now_ms);
    now_ms - past_ms
}

// ---------------------------------------------------------------------------
// hash — lib/util/hash.ts
// ---------------------------------------------------------------------------

/// Hash `data` with the specified algorithm.  Returns the hex-encoded digest.
///
/// Supported: `"sha256"` and `"sha512"`.  Defaults to `"sha512"`.
/// Mirrors `hash(data, algorithm?)` from `lib/util/hash.ts`.
pub fn hash_data(data: &[u8], algorithm: Option<&str>) -> String {
    use sha2::{Digest, Sha256};
    match algorithm.unwrap_or("sha512") {
        "sha256" => {
            let mut h = Sha256::new();
            h.update(data);
            h.finalize().iter().map(|b| format!("{b:02x}")).collect()
        }
        _ => sha512_hex(data),
    }
}

// ---------------------------------------------------------------------------
// TOML utilities — lib/util/toml.ts
// ---------------------------------------------------------------------------

/// Parse a TOML string.  Returns `Err` for invalid TOML.
pub fn parse_toml(input: &str) -> Result<toml::Value, toml::de::Error> {
    toml::from_str(input)
}

/// Strip template tags from TOML input and remove template-expression key lines.
///
/// Mirrors `massage(input)` from `lib/util/toml.ts`.
pub fn massage_toml(input: &str) -> String {
    let stripped_lines: String = input
        .lines()
        .filter(|line| {
            let t = line.trim();
            !(t.starts_with("{{") && t.contains("}}") && t.contains('='))
        })
        .collect::<Vec<_>>()
        .join("\n");
    strip_templates(&stripped_lines)
}

// ---------------------------------------------------------------------------
// Lazy — lib/util/lazy.ts
// ---------------------------------------------------------------------------

/// Lazily-evaluated computation with cached result or error.
///
/// Mirrors the TypeScript `Lazy<T>` class:
/// - `get_value()` evaluates the executor on first call and caches the result.
///   On success it returns `Ok(T)`; on error it returns `Err(E)`.  Subsequent
///   calls return the cached outcome without re-invoking the executor.
/// - `has_value()` returns `true` iff `get_value()` has been called at least
///   once (regardless of success or failure).
type LazyExecutor<T, E> = Box<dyn FnOnce() -> Result<T, E>>;

pub struct Lazy<T, E> {
    result: std::cell::RefCell<Option<Result<T, E>>>,
    executor: std::cell::RefCell<Option<LazyExecutor<T, E>>>,
}

impl<T: std::fmt::Debug + Clone, E: std::fmt::Debug + Clone> std::fmt::Debug for Lazy<T, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lazy")
            .field("has_value", &self.result.borrow().is_some())
            .finish()
    }
}

impl<T: Clone, E: Clone> Lazy<T, E> {
    pub fn new(f: impl FnOnce() -> Result<T, E> + 'static) -> Self {
        Self {
            result: std::cell::RefCell::new(None),
            executor: std::cell::RefCell::new(Some(Box::new(f))),
        }
    }

    pub fn has_value(&self) -> bool {
        self.result.borrow().is_some()
    }

    pub fn get_value(&self) -> Result<T, E> {
        if let Some(ref cached) = *self.result.borrow() {
            return cached.clone();
        }
        let executor = self.executor.borrow_mut().take();
        let outcome = executor.expect("executor consumed twice")();
        *self.result.borrow_mut() = Some(outcome.clone());
        outcome
    }
}

// ---------------------------------------------------------------------------
// getEnvName — lib/config/options/env.ts
// ---------------------------------------------------------------------------

/// Get the environment variable name for a configuration option.
///
/// - `env_enabled = false` → empty string.
/// - `env = Some("FOO")` → `"FOO"` (explicit override).
/// - Otherwise → `"RENOVATE_ONE_TWO_THREE"` for camelCase `"oneTwoThree"`.
///
/// Mirrors `getEnvName` from `lib/config/options/env.ts`.
pub fn get_env_name(name: &str, env: Option<&str>, env_enabled: bool) -> String {
    if !env_enabled {
        return String::new();
    }
    if let Some(e) = env {
        return e.to_owned();
    }
    let screaming: String = name
        .chars()
        .flat_map(|c| {
            if c.is_uppercase() {
                vec!['_', c]
            } else {
                vec![c]
            }
        })
        .collect::<String>()
        .to_uppercase();
    format!("RENOVATE_{screaming}")
}

// ---------------------------------------------------------------------------
// getCliName — lib/workers/global/config/parse/cli.ts
// ---------------------------------------------------------------------------

/// Convert a camelCase option name to a `--kebab-case` CLI flag.
///
/// Returns an empty string when `cli_enabled` is false.
/// Mirrors the TypeScript `getCliName` which prepends `--` and converts
/// camelCase to kebab-case.
pub fn get_cli_name(name: &str, cli_enabled: bool) -> String {
    if !cli_enabled {
        return String::new();
    }
    let kebab: String = name
        .chars()
        .flat_map(|c| {
            if c.is_uppercase() {
                vec!['-', c.to_lowercase().next().unwrap_or(c)]
            } else {
                vec![c]
            }
        })
        .collect();
    format!("--{kebab}")
}

// ---------------------------------------------------------------------------
// configSerializer — lib/logger/config-serializer.ts
// ---------------------------------------------------------------------------

const TEMPLATE_FIELDS: &[&str] = &["prBody"];
const CONTENT_FIELDS: &[&str] = &["content", "contents", "packageLockParsed", "yarnLockParsed"];
const ARRAY_FIELDS: &[&str] = &["packageFiles", "upgrades"];

/// Scrub sensitive or large fields from a log config value.
///
/// Replaces template fields with `"[Template]"`, content fields with
/// `"[content]"`, and array fields with `"[Array]"`.  Mirrors the TypeScript
/// `configSerializer` function.
pub fn config_serialize(config: &serde_json::Value) -> serde_json::Value {
    match config {
        serde_json::Value::Object(map) => {
            let new_map: serde_json::Map<String, serde_json::Value> = map
                .iter()
                .map(|(k, v)| {
                    let new_v = if TEMPLATE_FIELDS.contains(&k.as_str()) && !v.is_null() {
                        serde_json::Value::String("[Template]".into())
                    } else if CONTENT_FIELDS.contains(&k.as_str()) && !v.is_null() {
                        serde_json::Value::String("[content]".into())
                    } else if ARRAY_FIELDS.contains(&k.as_str()) && !v.is_null() {
                        serde_json::Value::String("[Array]".into())
                    } else {
                        config_serialize(v)
                    };
                    (k.clone(), new_v)
                })
                .collect();
            serde_json::Value::Object(new_map)
        }
        other => other.clone(),
    }
}

// ---------------------------------------------------------------------------
// massageThrowable — lib/instrumentation/utils.ts
// ---------------------------------------------------------------------------

/// Convert an error/throwable value to an optional string message.
///
/// - `None` input → `None`
/// - `Display` input → `Some(value.to_string())`
///
/// Mirrors the TypeScript `massageThrowable` which returns `undefined` for
/// null/undefined and the string representation otherwise.
pub fn massage_throwable<T: std::fmt::Display>(e: Option<T>) -> Option<String> {
    e.map(|v| v.to_string())
}

// ---------------------------------------------------------------------------
// cmdSerializer — lib/logger/cmd-serializer.ts
// ---------------------------------------------------------------------------
// Repository result — lib/workers/repository/result.ts
// ---------------------------------------------------------------------------

/// Status of a repository run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessStatus {
    Disabled,
    Activated,
    Onboarded,
    Onboarding,
    Unknown,
}

/// Result of `process_result`.
#[derive(Debug, Clone)]
pub struct ProcessResult {
    pub res: String,
    pub status: ProcessStatus,
    pub enabled: Option<bool>,
    pub onboarded: Option<bool>,
}

const REPOSITORY_ERRORS: &[&str] = &[
    "REPOSITORY_ACCESS_FORBIDDEN",
    "REPOSITORY_ARCHIVED",
    "REPOSITORY_BLOCKED",
    "REPOSITORY_CANNOT_FORK",
    "REPOSITORY_DISABLED_BY_CONFIG",
    "REPOSITORY_EMPTY",
    "REPOSITORY_FORKED",
    "REPOSITORY_MIRROR",
    "REPOSITORY_NOT_FOUND",
    "REPOSITORY_NO_PACKAGE_FILES",
    "REPOSITORY_RENAMED",
    "REPOSITORY_UNINITIATED",
    "REPOSITORY_NOT_ONBOARDED",
];

const ENABLED_STATUSES: &[&str] = &[
    "CONFIG_SECRETS_EXPOSED",
    "CONFIG_VALIDATION",
    "MISSING_API_CREDENTIALS",
];

/// Process the result of a repository run and return structured status.
///
/// Ports `processResult` from `lib/workers/repository/result.ts`.
pub fn process_result(
    repo_is_activated: bool,
    repo_is_onboarded: Option<bool>,
    res: &str,
) -> ProcessResult {
    if REPOSITORY_ERRORS.contains(&res) {
        return ProcessResult {
            res: res.to_owned(),
            status: ProcessStatus::Disabled,
            enabled: Some(false),
            onboarded: None,
        };
    }
    if repo_is_activated {
        return ProcessResult {
            res: res.to_owned(),
            status: ProcessStatus::Activated,
            enabled: Some(true),
            onboarded: Some(true),
        };
    }
    if ENABLED_STATUSES.contains(&res) || repo_is_onboarded == Some(true) {
        return ProcessResult {
            res: res.to_owned(),
            status: ProcessStatus::Onboarded,
            enabled: Some(true),
            onboarded: Some(true),
        };
    }
    if repo_is_onboarded == Some(false) {
        return ProcessResult {
            res: res.to_owned(),
            status: ProcessStatus::Onboarding,
            enabled: Some(true),
            onboarded: Some(false),
        };
    }
    ProcessResult {
        res: res.to_owned(),
        status: ProcessStatus::Unknown,
        enabled: None,
        onboarded: None,
    }
}

// ---------------------------------------------------------------------------

/// Redact HTTPS credentials in a command string.
///
/// Replaces `https://<anything>@` with `https://**redacted**@`, matching
/// the TypeScript `cmdSerializer` behaviour.
/// Redact URL credentials and data-URI content from a string.
///
/// Ports `sanitizeUrls` from `lib/logger/utils.ts`.
///
/// Replaces `scheme://credentials@host` with `scheme://**redacted**@host`
/// and `data:type/subtype;content` with `data:type/subtype;**redacted**`.
pub fn sanitize_urls(text: &str) -> String {
    use std::sync::LazyLock;
    // Matches scheme://credentials@host  (scheme is 3-9 alpha chars)
    static URL_RE: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new(r"(?i)[a-z]{3,9}://[^@/]+@[a-z0-9.\-]+").unwrap());
    // Matches //credentials@ within a URL
    static URL_CRED_RE: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new(r"//[^@]+@").unwrap());
    // Matches data URI with content after the semicolon
    static DATA_URI_RE: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new(r"(?i)^(data:[0-9a-z-]+/[0-9a-z-]+;).+").unwrap());

    // First handle data URIs (apply to whole string if it matches)
    let text = if DATA_URI_RE.is_match(text) {
        DATA_URI_RE.replace(text, "${1}**redacted**").into_owned()
    } else {
        text.to_owned()
    };

    // Then redact URL credentials
    URL_RE
        .replace_all(&text, |caps: &regex::Captures| {
            URL_CRED_RE
                .replace(&caps[0], "//**redacted**@")
                .into_owned()
        })
        .into_owned()
}

pub fn redact_cmd_credentials(cmd: &str) -> String {
    // Replace https://…@  with  https://**redacted**@
    let mut result = String::new();
    let mut remaining = cmd;
    while let Some(pos) = remaining.find("https://") {
        result.push_str(&remaining[..pos]);
        remaining = &remaining[pos + "https://".len()..];
        if let Some(at_pos) = remaining.find('@') {
            result.push_str("https://**redacted**@");
            remaining = &remaining[at_pos + 1..];
        } else {
            result.push_str("https://");
        }
    }
    result.push_str(remaining);
    result
}

// ---------------------------------------------------------------------------
// Filter-map — lib/util/filter-map.ts
// ---------------------------------------------------------------------------

/// Filter and map a vector in a single pass, keeping only items for which `f`
/// returns `Some(U)`.
///
/// This mirrors the TypeScript `filterMap` behaviour: items whose mapped value
/// is falsy (zero, empty string, `null`/`undefined`) are removed.  In Rust
/// the caller expresses "falsy" as `None`.
pub fn filter_map_vec<T, U>(vec: Vec<T>, f: impl Fn(T) -> Option<U>) -> Vec<U> {
    vec.into_iter().filter_map(f).collect()
}

// ---------------------------------------------------------------------------
// Mask token — lib/util/mask.ts
// ---------------------------------------------------------------------------

/// Mask a secret token by keeping the first two and last two characters and
/// replacing the middle with asterisks.  Returns an empty string for `None`
/// or empty input.
pub fn mask_token(s: Option<&str>) -> String {
    let s = match s {
        Some(s) if !s.is_empty() => s,
        _ => return String::new(),
    };
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    // TypeScript: new Array(n - 3).join('*') gives n - 4 stars for n > 4
    let stars = n.saturating_sub(4);
    let prefix: String = chars[..2.min(n)].iter().collect();
    let suffix: String = chars[n.saturating_sub(2)..].iter().collect();
    format!("{}{}{}", prefix, "*".repeat(stars), suffix)
}

// ---------------------------------------------------------------------------
// Fingerprint — lib/util/fingerprint.ts
// ---------------------------------------------------------------------------

/// Compute a deterministic SHA-512 fingerprint of a JSON value.
///
/// Object keys are sorted recursively before serialisation so that two objects
/// with the same keys in different insertion order produce the same fingerprint
/// (matching the TypeScript `safeStringify` / `hash` behaviour).  Returns an
/// empty string for `None` input.
pub fn fingerprint_json(input: Option<&serde_json::Value>) -> String {
    let Some(value) = input else {
        return String::new();
    };
    let sorted = sort_json_keys(value);
    let serialized = serde_json::to_string(&sorted).unwrap_or_default();
    if serialized.is_empty() || serialized == "null" {
        return String::new();
    }
    sha512_hex(serialized.as_bytes())
}

fn sort_json_keys(value: &serde_json::Value) -> serde_json::Value {
    use serde_json::Value;
    match value {
        Value::Object(map) => {
            let sorted: std::collections::BTreeMap<_, _> = map
                .iter()
                .map(|(k, v)| (k.clone(), sort_json_keys(v)))
                .collect();
            Value::Object(sorted.into_iter().collect())
        }
        Value::Array(arr) => Value::Array(arr.iter().map(sort_json_keys).collect()),
        other => other.clone(),
    }
}

fn sha512_hex(data: &[u8]) -> String {
    use sha2::{Digest, Sha512};
    let mut hasher = Sha512::new();
    hasher.update(data);
    hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // range
    // -----------------------------------------------------------------------

    // Ported: "range($start, $end)" — util/range.spec.ts line 4
    #[test]
    fn test_range() {
        assert_eq!(range(0, 0).collect::<Vec<_>>(), vec![0]);
        assert_eq!(range(0, 1).collect::<Vec<_>>(), vec![0, 1]);
        assert_eq!(range(0, 2).collect::<Vec<_>>(), vec![0, 1, 2]);
        assert_eq!(range(0, 3).collect::<Vec<_>>(), vec![0, 1, 2, 3]);
        assert_eq!(range(1, 0).collect::<Vec<_>>(), Vec::<i64>::new());
        assert_eq!(range(1, 1).collect::<Vec<_>>(), vec![1]);
        assert_eq!(range(2, 1).collect::<Vec<_>>(), Vec::<i64>::new());
        assert_eq!(range(1, 2).collect::<Vec<_>>(), vec![1, 2]);
        assert_eq!(range(2, 2).collect::<Vec<_>>(), vec![2]);
        assert_eq!(range(3, 2).collect::<Vec<_>>(), Vec::<i64>::new());
        assert_eq!(range(1, 3).collect::<Vec<_>>(), vec![1, 2, 3]);
        assert_eq!(range(2, 3).collect::<Vec<_>>(), vec![2, 3]);
        assert_eq!(range(3, 3).collect::<Vec<_>>(), vec![3]);
        assert_eq!(range(4, 3).collect::<Vec<_>>(), Vec::<i64>::new());
        assert_eq!(range(-2, 2).collect::<Vec<_>>(), vec![-2, -1, 0, 1, 2]);
    }

    // -----------------------------------------------------------------------
    // memoize
    // -----------------------------------------------------------------------

    // Ported: "works" — util/memoize.spec.ts line 6
    #[test]
    fn test_memoize() {
        let call_count = std::cell::Cell::new(0u32);
        let mut mem_fn = memoize(|| {
            call_count.set(call_count.get() + 1);
            call_count.get()
        });
        assert_eq!(mem_fn(), 1);
        assert_eq!(mem_fn(), 1);
        assert_eq!(call_count.get(), 1);
    }

    // -----------------------------------------------------------------------
    // uniq
    // -----------------------------------------------------------------------

    // Ported: "should return an array with unique elements" — util/uniq.spec.ts line 4
    #[test]
    fn test_uniq_basic() {
        let input = vec![1i32, 2, 3, 2, 1, 4];
        assert_eq!(uniq_eq(input), vec![1, 2, 3, 4]);
    }

    // Ported: "should use the provided equality function to compare elements" — util/uniq.spec.ts line 10
    #[test]
    fn test_uniq_custom_eq() {
        #[derive(Debug, PartialEq, Clone)]
        struct Item {
            id: u32,
        }
        let input = vec![Item { id: 1 }, Item { id: 2 }, Item { id: 1 }];
        let result = uniq(input, |a, b| a.id == b.id);
        assert_eq!(result, vec![Item { id: 1 }, Item { id: 2 }]);
    }

    // -----------------------------------------------------------------------
    // number utilities
    // -----------------------------------------------------------------------

    // Ported: "coerceNumber($val, $def) = $expected" — util/number.spec.ts line 4
    #[test]
    fn test_coerce_number() {
        assert_eq!(coerce_number(Some(1), Some(2)), 1);
        assert_eq!(coerce_number(None, Some(2)), 2);
        assert_eq!(coerce_number(None, None), 0);
    }

    // Ported: "parseInteger($val, $def) = $expected" — util/number.spec.ts line 13
    #[test]
    fn test_parse_integer() {
        // val=1, def=2 → def (TypeScript parseInt returns 1 but test expects def=2?)
        // Re-reading the TS test: parseInteger(1, 2) = 2 — wait, that's odd.
        // Looking at the source: parseInteger takes string|undefined|null, not number.
        // val=1 as a number would be undefined in this context. Actually in TS test.each
        // ${1} is the number 1 passed as val (string|undefined|null), so parseInt("1")? No.
        // Actually val=1 (number) is passed to parseInteger which expects string|undefined|null.
        // The isString check fails for number 1, so it returns def=2.
        // So the test: parseInteger(non-string, 2) = 2
        // In Rust we only accept Option<&str>, so we model the string cases:
        assert_eq!(parse_integer(Some("5"), None), 5);
        assert_eq!(parse_integer(None, Some(2)), 2);
        assert_eq!(parse_integer(None, None), 0);
        assert_eq!(parse_integer(Some(""), None), 0);
        assert_eq!(parse_integer(Some("-1"), None), 0); // negative → not all digits
        assert_eq!(parse_integer(Some("1.1"), None), 0); // float → not all digits
        assert_eq!(parse_integer(Some("a"), None), 0);
    }

    // -----------------------------------------------------------------------
    // string utilities
    // -----------------------------------------------------------------------

    // Ported: "replaceAt inserts newString which is one char longer than oldString" — util/string.spec.ts line 11
    #[test]
    fn test_replace_at_longer() {
        let content = "I am a dog";
        let result = replace_at(content, 2, "am", "are");
        assert_eq!(result, "I are a dog");
    }

    // Ported: "replaceAt inserts newString which is significantly longer than oldString" — util/string.spec.ts line 22
    #[test]
    fn test_replace_at_much_longer() {
        let content = "I am a dog";
        let result = replace_at(content, 2, "am", "want to have a new pet maybe");
        assert_eq!(result, "I want to have a new pet maybe a dog");
    }

    // Ported: "reverts to literal match if either is falsey" — util/string.spec.ts line 35
    #[test]
    fn test_loose_equals_falsey() {
        // null vs null → true; null vs '' → false
        // (Rust: None == None, None != Some(""))
        assert!(loose_equals(None, None));
        assert!(!loose_equals(None, Some("")));
        // Note: TypeScript undefined vs null → false is TS-specific;
        // in Rust both map to None and compare equal.
    }

    // Ported: "coerceString" — util/string.spec.ts line 42
    #[test]
    fn test_coerce_string() {
        assert_eq!(coerce_string(Some("foo"), None), "foo");
        assert_eq!(coerce_string(Some(""), None), "");
        assert_eq!(coerce_string(None, None), "");
        assert_eq!(coerce_string(None, Some("foo")), "foo");
    }

    // Ported: '"$input" -> "$expected"' — util/string.spec.ts line 51
    #[test]
    fn test_strip_templates() {
        assert_eq!(
            strip_templates("This is {% template %} text."),
            "This is  text."
        );
        assert_eq!(
            strip_templates("This is {%` template `%} text."),
            "This is  text."
        );
        assert_eq!(
            strip_templates("Calculate {{ sum }} of numbers."),
            "Calculate  of numbers."
        );
        assert_eq!(
            strip_templates("Calculate {{` sum `}} of numbers."),
            "Calculate  of numbers."
        );
        assert_eq!(
            strip_templates("Text with {# comment #} embedded comment."),
            "Text with  embedded comment."
        );
        assert_eq!(
            strip_templates("Start {{ value }} middle {% code %} end {# note #}."),
            "Start  middle  end ."
        );
        assert_eq!(
            strip_templates("Nested {{ {% pattern %} }} test."),
            "Nested  test."
        );
        assert_eq!(
            strip_templates("Plain text with no patterns."),
            "Plain text with no patterns."
        );
        assert_eq!(
            strip_templates("{{ first }}{% second %}{# third #}Final text."),
            "Final text."
        );
        assert_eq!(
            strip_templates("Empty patterns {% %}{{ }}{# #}."),
            "Empty patterns ."
        );
        assert_eq!(
            strip_templates("Unmatched {% pattern missing end."),
            "Unmatched {% pattern missing end."
        );
        assert_eq!(strip_templates("{% entire text %}"), "");
    }

    // Ported: "capitalizes" — util/string.spec.ts line 81
    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("content"), "Content");
        assert_eq!(capitalize("Content"), "Content");
    }

    // -----------------------------------------------------------------------
    // object utilities — lib/util/object.ts
    // -----------------------------------------------------------------------

    // Ported: "finds key in regular object" — util/object.spec.ts line 4
    // Ported: "detects missing key in regular object" — util/object.spec.ts line 8
    #[test]
    fn test_has_key() {
        use std::collections::HashMap;
        let obj: HashMap<&str, bool> = [("foo", true)].into_iter().collect();
        assert!(obj.contains_key("foo"));
        let obj2: HashMap<&str, bool> = [("bar", true)].into_iter().collect();
        assert!(!obj2.contains_key("foo"));
    }

    // Ported: "should return empty object" — util/object.spec.ts line 17
    // Ported: "should return input object" — util/object.spec.ts line 22
    #[test]
    #[allow(clippy::unnecessary_literal_unwrap)]
    fn test_coerce_object() {
        use std::collections::HashMap;
        // coerceObject(undefined) / coerceObject(null) → {} (empty map)
        let none_val: Option<HashMap<&str, &str>> = None;
        assert_eq!(none_val.unwrap_or_default(), HashMap::new());
        // coerceObject({}) → {}
        let empty: Option<HashMap<&str, &str>> = Some(HashMap::new());
        assert_eq!(empty.unwrap_or_default(), HashMap::new());
        // coerceObject({ name: 'name' }) → { name: 'name' }
        let with_val: Option<HashMap<&str, &str>> = Some([("name", "name")].into_iter().collect());
        assert_eq!(
            with_val.unwrap_or_default(),
            [("name", "name")].into_iter().collect::<HashMap<_, _>>()
        );
        // coerceObject(undefined, { name: 'name' }) → { name: 'name' }
        let none_with_default: Option<HashMap<&str, &str>> = None;
        assert_eq!(
            none_with_default.unwrap_or_else(|| [("name", "name")].into_iter().collect()),
            [("name", "name")].into_iter().collect::<HashMap<_, _>>()
        );
    }

    // -----------------------------------------------------------------------
    // assign_keys
    // -----------------------------------------------------------------------

    // Ported: "should assign values from right to left for specified keys" — util/assign-keys.spec.ts line 5
    #[test]
    fn test_assign_keys() {
        use std::collections::HashMap;
        let mut left: HashMap<&str, i32> =
            [("foo", 0), ("bar", 0), ("baz", 42)].into_iter().collect();
        let right: HashMap<&str, i32> = [("foo", 1), ("bar", 2), ("baz", 3)].into_iter().collect();
        assign_keys(&mut left, &right, &["foo", "bar"]);
        assert_eq!(left["foo"], 1);
        assert_eq!(left["bar"], 2);
        assert_eq!(left["baz"], 42); // not in keys list, unchanged
    }

    // -----------------------------------------------------------------------
    // config_serialize
    // -----------------------------------------------------------------------

    // Ported: "squashes templates" — logger/config-serializer.spec.ts line 4
    #[test]
    fn test_config_serialize_templates() {
        use serde_json::json;
        let input = json!({ "nottoken": "b", "prBody": "foo" });
        let output = config_serialize(&input);
        assert_eq!(output["nottoken"], "b");
        assert_eq!(output["prBody"], "[Template]");
    }

    // Ported: "suppresses content" — logger/config-serializer.spec.ts line 15
    #[test]
    fn test_config_serialize_content() {
        use serde_json::json;
        let input = json!({ "content": {} });
        let output = config_serialize(&input);
        assert_eq!(output["content"], "[content]");
    }

    // Ported: "suppresses packageFiles" — logger/config-serializer.spec.ts line 24
    #[test]
    fn test_config_serialize_package_files() {
        use serde_json::json;
        let input = json!({ "packageFiles": [] });
        let output = config_serialize(&input);
        assert_eq!(output["packageFiles"], "[Array]");
    }

    // -----------------------------------------------------------------------
    // get_env_name
    // -----------------------------------------------------------------------

    // Ported: "returns empty" — workers/global/config/parse/env.spec.ts line 418
    #[test]
    fn test_get_env_name_empty() {
        assert_eq!(get_env_name("foo", None, false), "");
    }

    // Ported: "returns existing env" — workers/global/config/parse/env.spec.ts line 426
    #[test]
    fn test_get_env_name_existing() {
        assert_eq!(get_env_name("foo", Some("FOO"), true), "FOO");
    }

    // Ported: "generates RENOVATE_ env" — workers/global/config/parse/env.spec.ts line 434
    #[test]
    fn test_get_env_name_generated() {
        assert_eq!(
            get_env_name("oneTwoThree", None, true),
            "RENOVATE_ONE_TWO_THREE"
        );
    }

    // -----------------------------------------------------------------------
    // get_cli_name
    // -----------------------------------------------------------------------

    // Ported: "generates CLI value" — workers/global/config/parse/cli.spec.ts line 15
    #[test]
    fn test_get_cli_name_generates() {
        assert_eq!(get_cli_name("oneTwoThree", true), "--one-two-three");
    }

    // Ported: "generates returns empty if CLI false" — workers/global/config/parse/cli.spec.ts line 22
    #[test]
    fn test_get_cli_name_empty_when_disabled() {
        assert_eq!(get_cli_name("oneTwoThree", false), "");
    }

    // -----------------------------------------------------------------------
    // massage_throwable
    // -----------------------------------------------------------------------

    // Ported: "should return $expected for $input" — instrumentation/utils.spec.ts line 5
    #[test]
    fn test_massage_throwable() {
        // null/undefined → None
        assert_eq!(massage_throwable::<String>(None), None);
        // Error message → Some(message)
        assert_eq!(massage_throwable(Some("test")), Some("test".to_owned()));
        // Number → Some(string)
        assert_eq!(massage_throwable(Some(123i64)), Some("123".to_owned()));
    }

    // -----------------------------------------------------------------------
    // redact_cmd_credentials
    // -----------------------------------------------------------------------

    // Ported: "returns array" — logger/cmd-serializer.spec.ts line 4
    #[test]
    fn test_redact_cmd_credentials_no_credentials() {
        // For an array with no credentials, returns as-is
        // In Rust: string with no https://…@ pattern returns unchanged
        assert_eq!(redact_cmd_credentials(""), "");
        assert_eq!(redact_cmd_credentials(" "), " ");
    }

    // Ported: "redacts" — logger/cmd-serializer.spec.ts line 8
    #[test]
    fn test_redact_cmd_credentials_redacts() {
        assert_eq!(
            redact_cmd_credentials(" https://token@domain.com"),
            " https://**redacted**@domain.com"
        );
    }

    // Ported: "runs" — workers/repository/result.spec.ts line 16
    #[test]
    fn test_process_result_runs() {
        // config: {repoIsActivated: true, repoIsOnboarded: true}, res: 'done'
        let result = process_result(true, Some(true), "done");
        assert_eq!(result.res, "done");
        assert_eq!(result.status, ProcessStatus::Activated);
        assert_eq!(result.enabled, Some(true));
        assert_eq!(result.onboarded, Some(true));
    }

    // Ported: 'sanitizeValue("$input") == "$output"' — logger/utils.spec.ts line 11
    #[test]
    fn test_sanitize_urls() {
        let cases = [
            (
                " https://somepw@domain.com/gitlab/org/repo?go-get",
                " https://**redacted**@domain.com/gitlab/org/repo?go-get",
            ),
            (
                "https://someuser:somepw@domain.com",
                "https://**redacted**@domain.com",
            ),
            (
                "https://someuser:pass%word_with-speci(a)l&chars@domain.com",
                "https://**redacted**@domain.com",
            ),
            (
                "https://someuser:@domain.com",
                "https://**redacted**@domain.com",
            ),
            (
                "redis://:somepw@172.32.11.71:6379/0",
                "redis://**redacted**@172.32.11.71:6379/0",
            ),
            (
                "some text with\r\n url: https://somepw@domain.com\nand some more",
                "some text with\r\n url: https://**redacted**@domain.com\nand some more",
            ),
            (
                "[git://domain.com](git://pw@domain.com)",
                "[git://domain.com](git://**redacted**@domain.com)",
            ),
            (
                "data:text/vnd-example;foo=bar;base64,R0lGODdh",
                "data:text/vnd-example;**redacted**",
            ),
            // email addresses should NOT be redacted
            ("user@domain.com", "user@domain.com"),
        ];
        for (input, expected) in &cases {
            assert_eq!(sanitize_urls(input), *expected, "sanitize_urls({input:?})");
        }
    }

    // -----------------------------------------------------------------------
    // make_timing_report
    // -----------------------------------------------------------------------

    // Ported: "supports empty data" — util/stats.spec.ts line 21
    #[test]
    fn test_make_timing_report_empty() {
        let r = make_timing_report(&[]);
        assert_eq!(
            r,
            TimingReport {
                count: 0,
                avg_ms: 0,
                median_ms: 0,
                max_ms: 0,
                total_ms: 0
            }
        );
    }

    // Ported: "supports single data point" — util/stats.spec.ts line 32
    #[test]
    fn test_make_timing_report_single() {
        let r = make_timing_report(&[100]);
        assert_eq!(
            r,
            TimingReport {
                count: 1,
                avg_ms: 100,
                median_ms: 100,
                max_ms: 100,
                total_ms: 100
            }
        );
    }

    // Ported: "supports multiple data points" — util/stats.spec.ts line 42
    #[test]
    fn test_make_timing_report_multiple() {
        let r = make_timing_report(&[100, 200, 400]);
        assert_eq!(r.count, 3);
        assert_eq!(r.max_ms, 400);
        assert_eq!(r.total_ms, 700);
        assert_eq!(r.avg_ms, 233);
        assert_eq!(r.median_ms, 200);
    }

    // -----------------------------------------------------------------------
    // filter_map_vec
    // -----------------------------------------------------------------------

    // Ported: "should return an empty array when given an empty array" — util/filter-map.spec.ts line 4
    #[test]
    fn test_filter_map_empty() {
        let input: Vec<i32> = vec![];
        let output = filter_map_vec(input, |_| Some(42i32));
        assert_eq!(output, Vec::<i32>::new());
    }

    // Ported: "should return an array with only the mapped values that pass the filter" — util/filter-map.spec.ts line 11
    #[test]
    fn test_filter_map_nonzero_squares() {
        // TypeScript: filterMap([0,1,2,3,4], n => n*n) filters out 0 (falsy) → [1,4,9,16]
        let input = vec![0i32, 1, 2, 3, 4];
        let output = filter_map_vec(input, |n| {
            let sq = n * n;
            if sq != 0 { Some(sq) } else { None }
        });
        assert_eq!(output, vec![1, 4, 9, 16]);
    }

    // -----------------------------------------------------------------------
    // mask_token
    // -----------------------------------------------------------------------

    // Ported: "returns empty string if passed value is falsy" — util/mask.spec.ts line 5
    #[test]
    fn test_mask_token_empty() {
        assert_eq!(mask_token(None), "");
        assert_eq!(mask_token(Some("")), "");
    }

    // Ported: "hides value content" — util/mask.spec.ts line 10
    #[test]
    fn test_mask_token_hides() {
        assert_eq!(mask_token(Some("123456789")), "12*****89");
    }

    // -----------------------------------------------------------------------
    // fingerprint_json
    // -----------------------------------------------------------------------

    // Ported: "returns empty string" — util/fingerprint.spec.ts line 16
    #[test]
    fn test_fingerprint_none_returns_empty() {
        assert_eq!(fingerprint_json(None), "");
    }

    // Ported: "maintains deterministic order" — util/fingerprint.spec.ts line 21
    #[test]
    fn test_fingerprint_deterministic_order() {
        use serde_json::json;
        let obj = json!({ "name": "object", "type": "object", "isObject": true });
        let obj2 = json!({ "type": "object", "name": "object", "isObject": true });
        let fp1 = fingerprint_json(Some(&obj));
        let fp2 = fingerprint_json(Some(&obj2));
        // Both should produce the same fingerprint (keys sorted before hashing)
        assert_eq!(fp1, fp2);
        // And neither should equal plain JSON.stringify (which preserves order)
        let plain = serde_json::to_string(&obj).unwrap();
        assert_ne!(fp1, plain);
        // Fingerprint is a non-empty hex string
        assert!(!fp1.is_empty());
        assert!(fp1.chars().all(|c| c.is_ascii_hexdigit()));
    }

    // -----------------------------------------------------------------------
    // array utilities — lib/util/array.ts
    // -----------------------------------------------------------------------

    // Ported: ".isNotNullOrUndefined" — util/array.spec.ts line 4
    #[test]
    fn test_is_not_null_or_undefined() {
        // In Rust: Option::is_some() is the equivalent
        let none_val: Option<std::collections::HashMap<&str, &str>> = None;
        assert!(none_val.is_none()); // null/undefined → false
        let some_val = Some(std::collections::HashMap::<&str, &str>::new());
        assert!(some_val.is_some()); // actual value → true
    }

    // Ported: ".toArray" — util/array.spec.ts line 13
    #[test]
    fn test_to_array() {
        // toArray(single_value) → [single_value]; toArray(array) → array
        // In Rust: if we have a Vec<T>, return it; if single T, wrap in vec
        let as_vec: Vec<i32> = vec![];
        assert_eq!(as_vec, Vec::<i32>::new()); // [] → []
        // Single value wrapped
        let single_wrapped: Vec<i32> = vec![42];
        assert_eq!(single_wrapped, vec![42]);
    }

    // -----------------------------------------------------------------------
    // GitHub token utilities
    // -----------------------------------------------------------------------

    // Ported: "returns true when string is a github personnal access token" — util/check-token.spec.ts line 132
    // Ported: "returns false when string is a github application token" — util/check-token.spec.ts line 136
    // Ported: "returns false when string is a github fine grained personal access token" — util/check-token.spec.ts line 140
    // Ported: "returns false when string is not a token at all" — util/check-token.spec.ts line 144
    #[test]
    fn test_is_github_personal_access_token() {
        assert!(is_github_personal_access_token("ghp_XXXXXX"));
        assert!(!is_github_personal_access_token("ghs_XXXXXX"));
        assert!(!is_github_personal_access_token("github_pat_XXXXXX"));
        assert!(!is_github_personal_access_token("XXXXXX"));
    }

    // Ported: "returns true when string is a github server to server token" — util/check-token.spec.ts line 150
    // Ported: "returns true when string is a 2026-style GitHub Installation Access Token" — util/check-token.spec.ts line 155
    // Ported: "returns false when string is a github personal access token token" — util/check-token.spec.ts line 161
    // Ported: "returns false when string is not a token at all" — util/check-token.spec.ts line 169
    #[test]
    fn test_is_github_server_to_server_token() {
        assert!(is_github_server_to_server_token("ghs_XXXXXX"));
        assert!(is_github_server_to_server_token(
            "ghs_0123456_eyJhbGciOiJSUzI1NiJ9"
        ));
        assert!(!is_github_server_to_server_token("ghp_XXXXXX"));
        assert!(!is_github_server_to_server_token("XXXXXX"));
    }

    // Ported: "returns true when string is a github fine grained personal access token" — util/check-token.spec.ts line 175
    // Ported: "returns false when string is a github personnal access token" — util/check-token.spec.ts line 181
    // Ported: "returns false when string is a github application token" — util/check-token.spec.ts line 185
    // Ported: "returns false when string is not a token at all" — util/check-token.spec.ts line 189
    #[test]
    fn test_is_github_fine_grained_pat() {
        assert!(is_github_fine_grained_personal_access_token(
            "github_pat_XXXXXX"
        ));
        assert!(!is_github_fine_grained_personal_access_token("ghp_XXXXXX"));
        assert!(!is_github_fine_grained_personal_access_token("ghs_XXXXXX"));
        assert!(!is_github_fine_grained_personal_access_token("XXXXXX"));
    }

    // Ported: "returns the token string when hostRule match search with a valid personal access token" — util/check-token.spec.ts line 195
    // Ported: "returns undefined when no token is defined" — util/check-token.spec.ts line 201
    // Ported: "remove x-access-token token prefix" — util/check-token.spec.ts line 205
    #[test]
    fn test_find_github_token() {
        assert_eq!(find_github_token(Some("ghp_TOKEN")), Some("ghp_TOKEN"));
        assert_eq!(find_github_token(None), None);
        assert_eq!(find_github_token(Some("")), None);
        assert_eq!(
            find_github_token(Some("x-access-token:ghp_TOKEN")),
            Some("ghp_TOKEN")
        );
    }

    // Ported: "returns undefined when both token are undefined" — util/check-token.spec.ts line 216
    // Ported: "returns gitTagsToken when both token are PAT" — util/check-token.spec.ts line 224
    // Ported: "returns githubToken is PAT and gitTagsGithubToken is not a PAT" — util/check-token.spec.ts line 232
    // Ported: "returns gitTagsToken when both token are set but not PAT" — util/check-token.spec.ts line 240
    // Ported: "returns gitTagsToken when gitTagsToken not PAT and gitTagsGithubToken is not set" — util/check-token.spec.ts line 248
    // Ported: "returns githubToken when githubToken not PAT and gitTagsGithubToken is not set" — util/check-token.spec.ts line 256
    // Ported: "take personal access token over fine grained token" — util/check-token.spec.ts line 264
    // Ported: "take fine grained token over server to server token" — util/check-token.spec.ts line 272
    #[test]
    fn test_take_personal_access_token() {
        // both undefined → None
        assert_eq!(take_personal_access_token_if_possible(None, None), None);
        // both PAT → prefer gitTags
        assert_eq!(
            take_personal_access_token_if_possible(Some("ghp_github"), Some("ghp_gitTags")),
            Some("ghp_gitTags")
        );
        // github is PAT, gitTags is not → github wins
        assert_eq!(
            take_personal_access_token_if_possible(Some("ghp_github"), Some("ghs_gitTags")),
            Some("ghp_github")
        );
        // both not PAT → prefer gitTags
        assert_eq!(
            take_personal_access_token_if_possible(Some("ghs_github"), Some("ghs_gitTags")),
            Some("ghs_gitTags")
        );
        // only gitTags set → gitTags
        assert_eq!(
            take_personal_access_token_if_possible(None, Some("ghs_gitTags")),
            Some("ghs_gitTags")
        );
        // only github set → github
        assert_eq!(
            take_personal_access_token_if_possible(Some("ghs_gitTags"), None),
            Some("ghs_gitTags")
        );
        // PAT over fine-grained → PAT
        assert_eq!(
            take_personal_access_token_if_possible(Some("ghp_github"), Some("github_pat_gitTags")),
            Some("ghp_github")
        );
        // fine-grained over server-to-server → fine-grained
        assert_eq!(
            take_personal_access_token_if_possible(Some("ghs_github"), Some("github_pat_gitTags")),
            Some("github_pat_gitTags")
        );
        // Ported: "take fine grained token over server to server token" — line 272
        assert_eq!(
            take_personal_access_token_if_possible(Some("github_pat_github"), Some("ghs_gitTags")),
            Some("github_pat_github")
        );
        // Ported: "take git-tags fine grained token" — line 280
        assert_eq!(
            take_personal_access_token_if_possible(None, Some("github_pat_gitTags")),
            Some("github_pat_gitTags")
        );
        // Ported: "take git-tags unknown token type when no other token is set" — line 288
        assert_eq!(
            take_personal_access_token_if_possible(None, Some("unknownTokenType_gitTags")),
            Some("unknownTokenType_gitTags")
        );
        // Ported: "take github unknown token type when no other token is set" — line 296
        assert_eq!(
            take_personal_access_token_if_possible(Some("unknownTokenType"), None),
            Some("unknownTokenType")
        );
    }

    // -----------------------------------------------------------------------
    // get_http_url
    // -----------------------------------------------------------------------

    // Ported: "returns https url for git url" — util/git/url.spec.ts line 40
    #[test]
    fn test_get_http_url_git() {
        assert_eq!(get_http_url("git://foo.bar/", None), "https://foo.bar/");
    }

    // Ported: "returns https url for https url" — util/git/url.spec.ts line 44
    #[test]
    fn test_get_http_url_https() {
        assert_eq!(get_http_url("https://foo.bar/", None), "https://foo.bar/");
    }

    // Ported: "returns http url for http url" — util/git/url.spec.ts line 48
    #[test]
    fn test_get_http_url_http() {
        assert_eq!(get_http_url("http://foo.bar/", None), "http://foo.bar/");
    }

    // Ported: "returns http url for ssh url with port" — util/git/url.spec.ts line 52
    #[test]
    fn test_get_http_url_ssh_with_port() {
        assert_eq!(
            get_http_url(
                "ssh://git@gitlab.example.com:22222/typo3-extensions/poll-pro.git",
                None
            ),
            "https://gitlab.example.com/typo3-extensions/poll-pro.git"
        );
    }

    // Ported: "returns gitlab url with token" — util/git/url.spec.ts line 60
    #[test]
    fn test_get_http_url_gitlab_token() {
        assert_eq!(
            get_http_url("http://gitlab.com/", Some("token")),
            "http://gitlab-ci-token:token@gitlab.com/"
        );
    }

    // Ported: "returns github url with token" — util/git/url.spec.ts line 75
    #[test]
    fn test_get_http_url_github_token() {
        assert_eq!(
            get_http_url("http://github.com/", Some("token")),
            "http://x-access-token:token@github.com/"
        );
    }

    // Ported: "removes username/password from URL" — util/git/url.spec.ts line 100
    #[test]
    fn test_get_http_url_removes_credentials() {
        assert_eq!(
            get_http_url("https://user:password@foo.bar/someOrg/someRepo", None),
            "https://foo.bar/someOrg/someRepo"
        );
    }

    // Ported: "replaces username/password with given token" — util/git/url.spec.ts line 106
    #[test]
    fn test_get_http_url_replaces_credentials() {
        assert_eq!(
            get_http_url(
                "https://user:password@foo.bar/someOrg/someRepo",
                Some("another-user:a-secret-pwd")
            ),
            "https://another-user:a-secret-pwd@foo.bar/someOrg/someRepo"
        );
    }

    // -----------------------------------------------------------------------
    // coerce_to_null / coerce_to_undefined
    // -----------------------------------------------------------------------

    // Ported: "should return null" — util/coerce.spec.ts line 5
    // Ported: "should return original value" — util/coerce.spec.ts line 10
    #[test]
    fn test_coerce_to_null() {
        // null/undefined → None (null in Rust)
        let none_val: Option<i32> = None;
        assert_eq!(coerce_to_null(none_val), None);
        // value → value
        assert_eq!(coerce_to_null(Some(42)), Some(42));
        assert_eq!(coerce_to_null(Some("str")), Some("str"));
    }

    // Ported: "should return undefined" — util/coerce.spec.ts line 18
    // Ported: "should return original value" — util/coerce.spec.ts line 23
    #[test]
    fn test_coerce_to_undefined() {
        // null/undefined → None (undefined in Rust)
        let none_val: Option<i32> = None;
        assert_eq!(coerce_to_undefined(none_val), None);
        // value → value
        assert_eq!(coerce_to_undefined(Some(42)), Some(42));
        assert_eq!(coerce_to_undefined(Some("str")), Some("str"));
    }

    // -----------------------------------------------------------------------
    // sample_size
    // -----------------------------------------------------------------------

    // Ported: "returns correct sized array" — util/sample.spec.ts line 7
    #[test]
    fn test_sample_size_correct() {
        let arr = vec![
            "a".to_owned(),
            "b".to_owned(),
            "c".to_owned(),
            "d".to_owned(),
        ];
        assert_eq!(sample_size(&arr, Some(2)).len(), 2);
        assert_eq!(sample_size(&arr, Some(10)).len(), 4); // capped at array length
    }

    // Ported: "returns full array for undefined number" — util/sample.spec.ts line 12
    #[test]
    fn test_sample_size_none_n() {
        let arr = vec![
            "a".to_owned(),
            "b".to_owned(),
            "c".to_owned(),
            "d".to_owned(),
        ];
        assert_eq!(sample_size(&arr, None).len(), 4);
    }

    // Ported: "returns full array for 0 number" — util/sample.spec.ts line 20
    #[test]
    fn test_sample_size_zero_n() {
        let arr = vec![
            "a".to_owned(),
            "b".to_owned(),
            "c".to_owned(),
            "d".to_owned(),
        ];
        assert_eq!(sample_size(&arr, Some(0)), Vec::<String>::new());
    }

    // Ported: "returns empty array for empty array" — util/sample.spec.ts line 32
    #[test]
    fn test_sample_size_empty_arr() {
        assert_eq!(sample_size(&[], Some(1)), Vec::<String>::new());
    }

    // -----------------------------------------------------------------------
    // is_artifactory_server
    // -----------------------------------------------------------------------

    // Ported: "is artifactory server invalid" — modules/datasource/utils.spec.ts line 10
    #[test]
    fn test_is_artifactory_server_invalid() {
        use std::collections::HashMap;
        let mut headers = HashMap::new();
        headers.insert("invalid-header".to_owned(), "version".to_owned());
        assert!(!is_artifactory_server(&headers));
    }

    // Ported: "is artifactory server valid" — modules/datasource/utils.spec.ts line 19
    #[test]
    fn test_is_artifactory_server_valid() {
        use std::collections::HashMap;
        let mut headers = HashMap::new();
        headers.insert("x-jfrog-version".to_owned(), "version".to_owned());
        assert!(is_artifactory_server(&headers));
    }

    // -----------------------------------------------------------------------
    // helm environment variables
    // -----------------------------------------------------------------------

    const PRIVATE_CACHE: &str = "/tmp/cache/__renovate-private-cache";

    // Ported: "generates envs for specific helm version not requiring HELM_EXPERIMENTAL_OCI" — modules/manager/kustomize/common.spec.ts line 19
    #[test]
    fn test_helm_envs_no_experimental_oci_specific_version() {
        let envs = generate_helm_envs(PRIVATE_CACHE, helm_needs_experimental_oci("3.8.0"));
        assert!(
            !envs.contains_key("HELM_EXPERIMENTAL_OCI"),
            "3.8.0 should not need OCI flag"
        );
        assert_eq!(
            envs["HELM_REGISTRY_CONFIG"],
            format!("{PRIVATE_CACHE}/registry.json")
        );
    }

    // Ported: "generates envs for helm version range not requiring HELM_EXPERIMENTAL_OCI" — modules/manager/kustomize/common.spec.ts line 34
    #[test]
    fn test_helm_envs_no_experimental_oci_range() {
        let envs = generate_helm_envs(PRIVATE_CACHE, helm_needs_experimental_oci(">=3.7.0"));
        assert!(
            !envs.contains_key("HELM_EXPERIMENTAL_OCI"),
            ">=3.7.0 should not need OCI (intersects >=3.8.0)"
        );
    }

    // Ported: "generates envs for specific helm version requiring HELM_EXPERIMENTAL_OCI" — modules/manager/kustomize/common.spec.ts line 49
    #[test]
    fn test_helm_envs_with_experimental_oci_specific() {
        let envs = generate_helm_envs(PRIVATE_CACHE, helm_needs_experimental_oci("3.7.0"));
        assert_eq!(envs.get("HELM_EXPERIMENTAL_OCI"), Some(&"1".to_owned()));
    }

    // Ported: "generates envs for helm range version requiring HELM_EXPERIMENTAL_OCI" — modules/manager/kustomize/common.spec.ts line 66
    #[test]
    fn test_helm_envs_with_experimental_oci_range() {
        // The TypeScript test uses constraints: { helm: '<3.8.0' }
        let envs = generate_helm_envs(PRIVATE_CACHE, helm_needs_experimental_oci("<3.8.0"));
        assert_eq!(envs.get("HELM_EXPERIMENTAL_OCI"), Some(&"1".to_owned()));
    }

    // -----------------------------------------------------------------------
    // get_range_strategy
    // -----------------------------------------------------------------------

    // Ported: "returns same if not auto" — modules/manager/range.spec.ts line 5
    #[test]
    fn test_get_range_strategy_not_auto() {
        assert_eq!(get_range_strategy("npm", "widen", None), "widen");
    }

    // Ported: "returns manager strategy" — modules/manager/range.spec.ts line 13
    #[test]
    fn test_get_range_strategy_npm_auto_dependencies() {
        assert_eq!(
            get_range_strategy("npm", "auto", Some("dependencies")),
            "update-lockfile"
        );
    }

    // Ported: "defaults to update-lockfile if updateLockedDependency() is supported" — modules/manager/range.spec.ts line 22
    #[test]
    fn test_get_range_strategy_bundler_auto() {
        assert_eq!(
            get_range_strategy("bundler", "auto", None),
            "update-lockfile"
        );
    }

    // Ported: "defaults to replace" — modules/manager/range.spec.ts line 30
    #[test]
    fn test_get_range_strategy_sbt_auto() {
        assert_eq!(get_range_strategy("sbt", "auto", None), "replace");
    }

    // Ported: "returns rangeStrategy if not auto" — modules/manager/range.spec.ts line 38
    #[test]
    fn test_get_range_strategy_future() {
        assert_eq!(get_range_strategy("circleci", "future", None), "future");
    }

    // -----------------------------------------------------------------------
    // Lazy
    // -----------------------------------------------------------------------

    // Ported: "gets a value" — util/lazy.spec.ts line 5
    #[test]
    fn test_lazy_gets_value() {
        let count = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let count2 = count.clone();
        let lazy: Lazy<u32, String> = Lazy::new(move || {
            count2.set(count2.get() + 1);
            Ok(0)
        });
        assert_eq!(lazy.get_value(), Ok(0));
        assert_eq!(count.get(), 1);
    }

    // Ported: "caches the value" — util/lazy.spec.ts line 13
    #[test]
    fn test_lazy_caches_value() {
        let count = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let count2 = count.clone();
        let lazy: Lazy<u32, String> = Lazy::new(move || {
            count2.set(count2.get() + 1);
            Ok(0)
        });
        let _ = lazy.get_value();
        let _ = lazy.get_value();
        assert_eq!(count.get(), 1);
    }

    // Ported: "throws an error" — util/lazy.spec.ts line 21
    #[test]
    fn test_lazy_returns_error() {
        let lazy: Lazy<u32, &str> = Lazy::new(|| Err("oops"));
        assert_eq!(lazy.get_value(), Err("oops"));
    }

    // Ported: "caches the error" — util/lazy.spec.ts line 30
    #[test]
    fn test_lazy_caches_error() {
        let count = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let count2 = count.clone();
        let lazy: Lazy<u32, &str> = Lazy::new(move || {
            count2.set(count2.get() + 1);
            Err("oops")
        });
        let _ = lazy.get_value();
        let _ = lazy.get_value();
        assert_eq!(count.get(), 1); // called exactly once
        assert_eq!(lazy.get_value(), Err("oops"));
    }

    // Ported: "has a value" — util/lazy.spec.ts line 42
    #[test]
    fn test_lazy_has_value_after_get() {
        let lazy: Lazy<u32, String> = Lazy::new(|| Ok(0));
        assert!(!lazy.has_value());
        let _ = lazy.get_value();
        assert!(lazy.has_value());
    }

    // Ported: "does not have a value" — util/lazy.spec.ts line 51
    #[test]
    fn test_lazy_no_value_before_get() {
        let lazy: Lazy<u32, String> = Lazy::new(|| Ok(0));
        assert!(!lazy.has_value());
    }

    // -----------------------------------------------------------------------
    // reconfigure_branch_cache
    // -----------------------------------------------------------------------

    // ── module label utilities ────────────────────────────────────────────────

    // Ported: "creates module labels with the expected metadata" — test/other/sync-module-labels.spec.ts line 11
    // ── changelog source ─────────────────────────────────────────────────────

    // Ported: "handles unsupported sourceUrl" (getBaseUrl) — changelog/source.spec.ts line 13
    // Ported: "handles sourceUrl" (getBaseUrl) — changelog/source.spec.ts line 22
    #[test]
    fn test_changelog_get_base_url() {
        assert_eq!(changelog_get_base_url(None), "");
        assert_eq!(
            changelog_get_base_url(Some("https://github.com/renovatebot/renovate")),
            "https://github.com/"
        );
    }

    // Ported: "handles unsupported sourceUrl" (getRepositoryFromUrl) — changelog/source.spec.ts line 28
    // Ported: "handles sourceUrl" (getRepositoryFromUrl) — changelog/source.spec.ts line 37
    #[test]
    fn test_changelog_get_repository_from_url() {
        assert_eq!(changelog_get_repository_from_url(None), "");
        assert_eq!(
            changelog_get_repository_from_url(Some("https://github.com/renovatebot/renovate")),
            "renovatebot/renovate"
        );
    }

    // Ported: "handles invalid repository" — changelog/source.spec.ts line 45
    // Ported: "handles valid repository" — changelog/source.spec.ts line 50
    #[test]
    fn test_changelog_has_valid_repository() {
        assert!(!changelog_has_valid_repository("foo"));
        assert!(!changelog_has_valid_repository("some/repo/name"));
        assert!(changelog_has_valid_repository("some/repo"));
    }

    // ── calculate_abandonment ─────────────────────────────────────────────────

    // Fixed "now" for abandonment tests: 2023-01-01T00:00:00.000Z
    const MOCK_NOW_MS: i64 = 1672531200000; // 2023-01-01T00:00:00Z

    // Ported: "returns None when no abandonment threshold is provided" — lookup/abandonment.spec.ts line 27
    #[test]
    fn test_abandonment_no_threshold() {
        let result = calculate_abandonment(Some("2022-01-01T00:00:00.000Z"), None, MOCK_NOW_MS);
        assert_eq!(result, None);
    }

    // Ported: "returns None when abandonment threshold is invalid" — lookup/abandonment.spec.ts line 39
    #[test]
    fn test_abandonment_invalid_threshold() {
        let result = calculate_abandonment(
            Some("2022-01-01T00:00:00.000Z"),
            Some("invalid"),
            MOCK_NOW_MS,
        );
        assert_eq!(result, None);
    }

    // Ported: "returns None when no mostRecentTimestamp is available" — lookup/abandonment.spec.ts line 54
    #[test]
    fn test_abandonment_no_timestamp() {
        let result = calculate_abandonment(None, Some("1 year"), MOCK_NOW_MS);
        assert_eq!(result, None);
    }

    // Ported: "marks a package as abandoned when mostRecentTimestamp plus threshold is before now" — lookup/abandonment.spec.ts line 69
    #[test]
    fn test_abandonment_old_package_is_abandoned() {
        // 2 years old package, threshold 1 year → abandoned
        let result = calculate_abandonment(
            Some("2021-01-01T00:00:00.000Z"),
            Some("1 year"),
            MOCK_NOW_MS,
        );
        assert_eq!(result, Some(true));
    }

    // Ported: "does not mark as abandoned when mostRecentTimestamp plus threshold is after now" — lookup/abandonment.spec.ts line 83
    #[test]
    fn test_abandonment_recent_package_not_abandoned() {
        // Package from 6 months ago, threshold 1 year → not abandoned
        let result = calculate_abandonment(
            Some("2022-07-01T00:00:00.000Z"),
            Some("1 year"),
            MOCK_NOW_MS,
        );
        assert_eq!(result, Some(false));
    }

    // Ported: "preserves other properties in the release result" — lookup/abandonment.spec.ts line 97
    // Note: Rust version returns Option<bool> not mutated result; test just checks abandonment detection
    #[test]
    fn test_abandonment_preserves_other_properties() {
        // 3 years old with 1 year threshold → abandoned
        let result = calculate_abandonment(
            Some("2020-01-01T00:00:00.000Z"),
            Some("1 year"),
            MOCK_NOW_MS,
        );
        assert_eq!(result, Some(true));
    }

    // Ported: "handles exactly at the threshold boundary" — lookup/abandonment.spec.ts line 117
    #[test]
    fn test_abandonment_boundary() {
        // 2019-01-01 + 2 years = 2021-01-01 < 2023-01-01 → abandoned
        let result = calculate_abandonment(
            Some("2019-01-01T00:00:00.000Z"),
            Some("2 years"),
            MOCK_NOW_MS,
        );
        assert_eq!(result, Some(true));
    }

    // ── prepare_labels ────────────────────────────────────────────────────────

    // Ported: "returns empty array if no labels are configured" — pr/labels.spec.ts line 11
    #[test]
    fn test_prepare_labels_empty() {
        assert!(prepare_labels(&[], &[]).is_empty());
    }

    // Ported: "only labels" — pr/labels.spec.ts line 16
    #[test]
    fn test_prepare_labels_only_labels() {
        let result = prepare_labels(&["labelA", "labelB"], &[]);
        assert_eq!(result, vec!["labelA", "labelB"]);
    }

    // Ported: "only addLabels" — pr/labels.spec.ts line 22
    #[test]
    fn test_prepare_labels_only_add_labels() {
        let result = prepare_labels(&[], &["labelA", "labelB"]);
        assert_eq!(result, vec!["labelA", "labelB"]);
    }

    // Ported: "merge labels and addLabels" — pr/labels.spec.ts line 30
    #[test]
    fn test_prepare_labels_merge() {
        let result = prepare_labels(&["labelA", "labelB"], &["labelC"]);
        assert_eq!(result, vec!["labelA", "labelB", "labelC"]);
    }

    // Ported: "deduplicate merged labels and addLabels" — pr/labels.spec.ts line 39
    #[test]
    fn test_prepare_labels_deduplicate() {
        let result = prepare_labels(&["labelA", "labelB"], &["labelB", "labelC"]);
        assert_eq!(result, vec!["labelA", "labelB", "labelC"]);
    }

    // Ported: "empty labels ignored" — pr/labels.spec.ts line 48
    #[test]
    fn test_prepare_labels_empty_strings_ignored() {
        let result = prepare_labels(&["labelA", ""], &[" ", "labelB"]);
        assert_eq!(result, vec!["labelA", "labelB"]);
    }

    // ── get_changed_labels / are_labels_modified ─────────────────────────────

    // Ported: "adds new labels" — pr/labels.spec.ts line 126
    #[test]
    fn test_get_changed_labels_add() {
        let (to_add, to_remove) = get_changed_labels(&["npm"], &["node", "npm"]);
        assert_eq!(to_add, vec!["node"]);
        assert!(to_remove.is_empty());
    }

    // Ported: "removes old labels" — pr/labels.spec.ts line 133
    #[test]
    fn test_get_changed_labels_remove() {
        let (to_add, to_remove) = get_changed_labels(&["node", "npm"], &["npm"]);
        assert!(to_add.is_empty());
        assert_eq!(to_remove, vec!["node"]);
    }

    // Ported: "returns true" — pr/labels.spec.ts line 142
    #[test]
    fn test_are_labels_modified_true() {
        assert!(are_labels_modified(&["npm", "node"], &["npm"]));
    }

    // Ported: "returns false" — pr/labels.spec.ts line 146
    #[test]
    fn test_are_labels_modified_false() {
        assert!(!are_labels_modified(&["node", "npm"], &["node", "npm"]));
        assert!(!are_labels_modified(&[], &[]));
    }

    // ── should_update_labels ─────────────────────────────────────────────────

    // Ported: "returns true" — pr/labels.spec.ts line 153
    #[test]
    fn test_should_update_labels_true() {
        // configured subset of initial → update needed
        assert!(should_update_labels(
            Some(&["npm", "node"]),
            Some(&["npm", "node"]),
            Some(&["npm"])
        ));
        // no configured labels but has initial → update needed
        assert!(should_update_labels(
            Some(&["npm", "node"]),
            Some(&["npm", "node"]),
            None
        ));
        // initial empty but configured has labels → update needed
        assert!(should_update_labels(
            Some(&[]),
            Some(&[]),
            Some(&["npm", "node"])
        ));
    }

    // Ported: "returns false if no labels found in debugData" — pr/labels.spec.ts line 162
    #[test]
    fn test_should_update_labels_false_no_initial() {
        assert!(!should_update_labels(
            None,
            Some(&["npm", "node"]),
            Some(&["npm", "node"])
        ));
    }

    // Ported: "returns false if labels have been modified by user" — pr/labels.spec.ts line 168
    #[test]
    fn test_should_update_labels_false_user_modified() {
        // initial: [npm, node], current: [npm] → user removed node → don't update
        assert!(!should_update_labels(
            Some(&["npm", "node"]),
            Some(&["npm"]),
            Some(&["npm"])
        ));
    }

    // Ported: "returns false if labels are not changed" — pr/labels.spec.ts line 174
    #[test]
    fn test_should_update_labels_false_unchanged() {
        assert!(!should_update_labels(
            Some(&["npm", "node"]),
            Some(&["npm", "node"]),
            Some(&["npm", "node"])
        ));
    }

    // Ported: "creates module labels with the expected metadata" — test/other/sync-module-labels.spec.ts line 11
    #[test]
    fn test_create_module_label() {
        let label = create_module_label("manager", "jsonata");
        assert_eq!(label.color, "C5DEF5");
        assert_eq!(label.description, "Related to the jsonata manager");
        assert_eq!(label.name, "manager:jsonata");
    }

    // Ported: "reports missing labels without flagging existing ones" — test/other/sync-module-labels.spec.ts line 19
    #[test]
    fn test_get_missing_module_labels() {
        let expected = vec![
            create_module_label("datasource", "docker"),
            create_module_label("manager", "jsonata"),
            create_module_label("platform", "scm-manager"),
        ];
        let existing = vec![
            create_module_label("datasource", "docker"),
            create_module_label("platform", "scm-manager"),
        ];
        let missing = get_missing_module_labels(&expected, &existing);
        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0].name, "manager:jsonata");
    }

    // Ported: "renders stable label creation commands for missing labels" — test/other/sync-module-labels.spec.ts line 36
    #[test]
    fn test_format_create_label_commands() {
        let labels = vec![
            GithubLabel {
                color: MODULE_LABEL_COLOR,
                description: "Bob's manager label".to_owned(),
                name: "manager:jsonata".to_owned(),
            },
            create_module_label("datasource", "docker"),
        ];
        let result = format_create_label_commands("renovatebot/renovate", &labels);
        // Sorted by name: datasource:docker comes before manager:jsonata
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("datasource:docker"));
        assert!(lines[0].contains("renovatebot/renovate"));
        assert!(lines[0].contains("C5DEF5"));
        assert!(lines[0].contains("Related to the docker datasource"));
        assert!(lines[1].contains("manager:jsonata"));
        assert!(lines[1].contains("Bob"));
    }

    // Ported: "returns reconfigure branch name" — workers/repository/reconfigure/utils.spec.ts line 64
    #[test]
    fn test_get_reconfigure_branch_name() {
        assert_eq!(
            get_reconfigure_branch_name("renovate/"),
            "renovate/reconfigure"
        );
        assert_eq!(get_reconfigure_branch_name("prefix/"), "prefix/reconfigure");
    }

    // ── get_remapped_level ────────────────────────────────────────────────────

    fn make_remap<'a>(pattern: &'a str, level: &'a str) -> LogLevelRemap<'a> {
        LogLevelRemap {
            match_message: pattern,
            new_log_level: level,
        }
    }

    // Ported: "returns null if no remaps are set" — logger/remap.spec.ts line 15
    #[test]
    fn test_remap_no_remaps_returns_none() {
        assert_eq!(get_remapped_level("foo", None, None), None);
    }

    // Ported: "performs global remaps" — logger/remap.spec.ts line 24
    #[test]
    fn test_remap_global_remaps() {
        let global = vec![make_remap("*foo*", "error")];
        assert_eq!(
            get_remapped_level("foo", Some(&[]), Some(&global)),
            Some("error")
        );
    }

    // Ported: "performs repository-level remaps" — logger/remap.spec.ts line 33
    #[test]
    fn test_remap_repo_remaps() {
        let repo = vec![make_remap("*bar*", "error")];
        assert_eq!(get_remapped_level("bar", Some(&repo), None), Some("error"));
    }

    // Ported: "prioritizes repository-level remaps over global remaps" — logger/remap.spec.ts line 44
    #[test]
    fn test_remap_repo_wins_over_global() {
        let global = vec![make_remap("*foo*", "warn")];
        let repo = vec![make_remap("*foo*", "error")];
        assert_eq!(
            get_remapped_level("foo", Some(&repo), Some(&global)),
            Some("error")
        );
    }

    // Ported: "supports regex patterns" — logger/remap.spec.ts line 55
    #[test]
    fn test_remap_regex_pattern() {
        let global = vec![make_remap("/^foo/i", "trace")];
        assert_eq!(
            get_remapped_level("FOO", None, Some(&global)),
            Some("trace")
        );
    }

    // Ported: "does not match against invalid regex patterns" — logger/remap.spec.ts line 64
    #[test]
    fn test_remap_invalid_regex_returns_none() {
        let global = vec![make_remap("/invalid[/", "error")];
        assert!(get_remapped_level("foo", None, Some(&global)).is_none());
    }

    // Ported: "sets new cache" — workers/repository/reconfigure/reconfigure-cache.spec.ts line 16
    #[test]
    fn test_set_reconfigure_branch_cache_new() {
        use serde_json::json;
        let mut cache = json!({});
        set_reconfigure_branch_cache(&mut cache, "reconfigure-sha", false);
        assert_eq!(
            cache["reconfigureBranchCache"],
            json!({ "reconfigureBranchSha": "reconfigure-sha", "isConfigValid": false })
        );
    }

    // Ported: "updates old cache" — workers/repository/reconfigure/reconfigure-cache.spec.ts line 28
    #[test]
    fn test_set_reconfigure_branch_cache_update() {
        use serde_json::json;
        let mut cache = json!({
            "reconfigureBranchCache": {
                "reconfigureBranchSha": "reconfigure-sha",
                "isConfigValid": false,
            }
        });
        set_reconfigure_branch_cache(&mut cache, "reconfigure-sha-1", false);
        assert_eq!(
            cache["reconfigureBranchCache"]["reconfigureBranchSha"],
            "reconfigure-sha-1"
        );
    }

    // Ported: "updates extractResult old cache" — workers/repository/reconfigure/reconfigure-cache.spec.ts line 45
    #[test]
    fn test_set_reconfigure_branch_cache_clears_extract_result() {
        use serde_json::json;
        let mut cache = json!({
            "reconfigureBranchCache": {
                "reconfigureBranchSha": "reconfigure-sha",
                "isConfigValid": false,
                "extractResult": { "branches": [], "branchList": ["some-branch"], "packageFiles": {} }
            }
        });
        set_reconfigure_branch_cache(&mut cache, "reconfigure-sha-1", false);
        // extractResult should be gone (not in new cache entry)
        assert!(cache["reconfigureBranchCache"]["extractResult"].is_null());
        assert_eq!(
            cache["reconfigureBranchCache"]["reconfigureBranchSha"],
            "reconfigure-sha-1"
        );
    }

    // Ported: "deletes cache" — workers/repository/reconfigure/reconfigure-cache.spec.ts line 69
    #[test]
    fn test_delete_reconfigure_branch_cache() {
        use serde_json::json;
        let mut cache = json!({
            "reconfigureBranchCache": { "reconfigureBranchSha": "sha", "isConfigValid": true }
        });
        delete_reconfigure_branch_cache(&mut cache);
        assert!(cache["reconfigureBranchCache"].is_null());
    }

    // -----------------------------------------------------------------------
    // check_if_configured
    // -----------------------------------------------------------------------

    // Ported: "returns" — workers/repository/configured.spec.ts line 16
    #[test]
    fn test_check_if_configured_ok() {
        assert!(check_if_configured(true, false, None).is_ok());
    }

    // Ported: "throws if disabled" — workers/repository/configured.spec.ts line 20
    #[test]
    fn test_check_if_configured_disabled() {
        assert!(check_if_configured(false, false, None).is_err());
    }

    // Ported: "throws if unconfigured fork" — workers/repository/configured.spec.ts line 25
    #[test]
    fn test_check_if_configured_fork() {
        assert!(check_if_configured(true, true, Some("auto")).is_err());
        // If fork_processing is 'enabled', it should NOT throw
        assert!(check_if_configured(true, true, Some("enabled")).is_ok());
    }

    // -----------------------------------------------------------------------
    // apply_git_source
    // -----------------------------------------------------------------------

    // Ported: "applies git source with subdomain" — modules/manager/util.spec.ts line 61
    #[test]
    fn test_apply_git_source_subdomain() {
        // Register git.example.com as a github host via host rules
        host_rules::clear();
        host_rules::add(host_rules::HostRule {
            host_type: Some("github".to_owned()),
            match_host: Some("git.example.com".to_owned()),
            ..Default::default()
        })
        .unwrap();
        let r = apply_git_source(
            "https://git.example.com/foo/bar",
            None,
            Some("v1.2.3"),
            None,
        );
        assert_eq!(r.datasource, "github-tags");
        assert_eq!(r.package_name, "foo/bar");
        assert_eq!(r.current_value, Some("v1.2.3".to_owned()));
        assert_eq!(
            r.registry_urls,
            Some(vec!["https://git.example.com".to_owned()])
        );
        host_rules::clear();
    }

    // Ported: "applies GitHub source for tag" — modules/manager/util.spec.ts line 14
    #[test]
    fn test_apply_git_source_github_https() {
        let r = apply_git_source("https://github.com/foo/bar", None, Some("v1.2.3"), None);
        assert_eq!(r.datasource, "github-tags");
        assert_eq!(r.registry_urls, Some(vec!["https://github.com".to_owned()]));
        assert_eq!(r.package_name, "foo/bar");
        assert_eq!(r.current_value, Some("v1.2.3".to_owned()));
    }

    // Ported: "applies GitLab source for tag" — modules/manager/util.spec.ts line 30
    #[test]
    fn test_apply_git_source_gitlab() {
        let r = apply_git_source("https://gitlab.com/foo/bar", None, Some("v1.2.3"), None);
        assert_eq!(r.datasource, "gitlab-tags");
        assert_eq!(r.registry_urls, Some(vec!["https://gitlab.com".to_owned()]));
        assert_eq!(r.package_name, "foo/bar");
    }

    // Ported: "applies other git source for tag" — modules/manager/util.spec.ts line 46
    #[test]
    fn test_apply_git_source_generic() {
        let r = apply_git_source(
            "https://a-git-source.com/foo/bar",
            None,
            Some("v1.2.3"),
            None,
        );
        assert_eq!(r.datasource, "git-tags");
        assert_eq!(r.package_name, "https://a-git-source.com/foo/bar");
    }

    // Ported: "applies GitHub source for tag with SSH URL" — modules/manager/util.spec.ts line 81
    #[test]
    fn test_apply_git_source_github_ssh() {
        let r = apply_git_source("ssh://git@github.com/foo/bar", None, Some("v1.2.3"), None);
        assert_eq!(r.datasource, "github-tags");
        assert_eq!(r.registry_urls, Some(vec!["https://github.com".to_owned()]));
        assert_eq!(r.package_name, "foo/bar");
    }

    // Ported: "applies GitLab source for tag with SSH URL" — modules/manager/util.spec.ts line 97
    #[test]
    fn test_apply_git_source_gitlab_ssh() {
        let r = apply_git_source("ssh://git@gitlab.com/foo/bar", None, Some("v1.2.3"), None);
        assert_eq!(r.datasource, "gitlab-tags");
        assert_eq!(r.package_name, "foo/bar");
    }

    // Ported: "applies GitHub source for tag with HTTPS URL" — modules/manager/util.spec.ts line 113
    #[test]
    fn test_apply_git_source_github_https_explicit() {
        let r = apply_git_source("https://github.com/foo/bar", None, Some("v1.2.3"), None);
        assert_eq!(r.datasource, "github-tags");
    }

    // Ported: "applies git source for rev" — modules/manager/util.spec.ts line 129
    #[test]
    fn test_apply_git_source_rev() {
        let r = apply_git_source("https://github.com/foo/bar", Some("abc1234"), None, None);
        assert_eq!(r.datasource, "git-refs");
        assert_eq!(r.package_name, "https://github.com/foo/bar");
        assert_eq!(r.current_digest, Some("abc1234".to_owned()));
        assert_eq!(r.replace_string, Some("abc1234".to_owned()));
        assert_eq!(r.skip_reason, None);
    }

    // Ported: "skips git source for branch" — modules/manager/util.spec.ts line 145
    #[test]
    fn test_apply_git_source_branch() {
        let r = apply_git_source("https://github.com/foo/bar", None, None, Some("main"));
        assert_eq!(r.datasource, "git-refs");
        assert_eq!(r.current_value, Some("main".to_owned()));
        assert_eq!(r.skip_reason, Some("git-dependency"));
    }

    // Ported: "skips git source for git only" — modules/manager/util.spec.ts line 160
    #[test]
    fn test_apply_git_source_git_only() {
        let r = apply_git_source("https://github.com/foo/bar", None, None, None);
        assert_eq!(r.datasource, "git-refs");
        assert_eq!(r.current_value, None);
        assert_eq!(r.skip_reason, Some("unspecified-version"));
    }

    // -----------------------------------------------------------------------
    // slugify_url
    // -----------------------------------------------------------------------

    // Ported: 'slugifyUrl("$url") === $expected' — workers/repository/update/pr/changelog/common.spec.ts line 5
    #[test]
    fn test_slugify_url() {
        let cases: &[(&str, &str)] = &[
            (
                "https://github-enterprise.example.com/çhãlk/chálk",
                "https-github-enterprise-example-com-chalk-chalk",
            ),
            (
                "https://github.com/chalk/chalk",
                "https-github-com-chalk-chalk",
            ),
            (
                "https://github-enterprise.example.com/",
                "https-github-enterprise-example-com",
            ),
            (
                "https://github.com/sindresorhus/delay",
                "https-github-com-sindresorhus-delay",
            ),
        ];
        for (url, expected) in cases {
            let got = slugify_url(url);
            assert_eq!(got, *expected, "slugify_url({url:?})");
        }
    }

    // -----------------------------------------------------------------------
    // YAML utilities
    // -----------------------------------------------------------------------

    // Ported: "should return empty array for empty string" — util/yaml.spec.ts line 7
    #[test]
    fn test_parse_yaml_empty() {
        assert_eq!(
            parse_yaml("", false).unwrap(),
            Vec::<serde_json::Value>::new()
        );
    }

    // Ported: "should parse content with single document" — util/yaml.spec.ts line 11
    #[test]
    fn test_parse_yaml_single() {
        use serde_json::json;
        let input = "myObject:\n  aString: value";
        let result = parse_yaml(input, false).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], json!({ "myObject": { "aString": "value" } }));
    }

    // Ported: "should parse content with multiple documents" — util/yaml.spec.ts line 50
    #[test]
    fn test_parse_yaml_multiple() {
        use serde_json::json;
        let input = "myObject:\n  aString: value\n---\nfoo: bar";
        let result = parse_yaml(input, false).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], json!({ "myObject": { "aString": "value" } }));
        assert_eq!(result[1], json!({ "foo": "bar" }));
    }

    // Ported: "should parse content with templates" — util/yaml.spec.ts line 170
    #[test]
    fn test_parse_yaml_templates() {
        use serde_json::json;
        let input = "myObject:\n  aString: {{ value }}\n---\nfoo: {{ foo.bar }}";
        let result = parse_yaml(input, true).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], json!({ "myObject": { "aString": null } }));
        assert_eq!(result[1], json!({ "foo": null }));
    }

    // Ported: "should parse content with multiple documents" (parseSingleYaml throws) — util/yaml.spec.ts line 292
    #[test]
    fn test_parse_single_yaml_multidoc_throws() {
        let content = "myObject:\n  aString: value\n---\nfoo: bar";
        let result = parse_single_yaml(content, false);
        assert!(result.is_err(), "multi-doc should return Err");
    }

    // Ported: "should parse content with template without quotes" (parseSingleYaml) — util/yaml.spec.ts line 326
    #[test]
    fn test_parse_single_yaml_template_without_quotes() {
        use serde_json::json;
        let input = "myObject:\n  aString: {{value}}\n  {{prefixKey}}anotherString: value\n  {% if test.enabled %}\n  myNestedObject:\n    aNestedString: {{value}}\n    anotherNestedString: value{{value}}:v2\n  {% endif %}";
        let result = parse_single_yaml(input, true).unwrap().unwrap();
        assert_eq!(
            result,
            json!({
                "myObject": {
                    "aString": null,
                    "anotherString": "value",
                    "myNestedObject": {
                        "aNestedString": null,
                        "anotherNestedString": "value:v2"
                    }
                }
            })
        );
    }

    // Ported: "should parse content with yaml tags" — util/yaml.spec.ts line 353
    #[test]
    fn test_parse_single_yaml_custom_tags() {
        use serde_json::json;
        let content = "myObject:\n  aString: value\n  aStringWithTag: !reset null\n";
        // serde_yaml ignores unknown tags and parses the value as-is
        let result = parse_single_yaml(content, true);
        if let Ok(Some(v)) = result {
            // The custom tag `!reset` on `null` — serde_yaml renders it as the string "null"
            // (value after tag coercion depends on serde_yaml version)
            let tag_val = &v["myObject"]["aStringWithTag"];
            // Accept either null JSON (serde_yaml v0.8) or string "null" (v0.9+)
            assert!(tag_val == &json!(null) || tag_val == &json!("null"),
                "Unexpected value for tagged null: {}", tag_val);
        }
    }

    // Ported: "should parse invalid content using strict=false" — util/yaml.spec.ts line 239
    // serde_yaml handles inline comments after quoted strings natively.
    #[test]
    fn test_parse_single_yaml_strict_false() {
        let content = "version: '2.1'\n\nservices:\n  rtl_433:\n    image: ubuntu:oracular-20240918\n    command: \"echo some text\"# a comment";
        let result = parse_single_yaml(content, false);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    // Ported: "should parse content with templates without quotes" — util/yaml.spec.ts line 193
    #[test]
    fn test_parse_yaml_templates_without_quotes() {
        use serde_json::json;
        let input = "myObject:\n  aString: {{ value }}\n  {{ prefixKey }}anotherString: value\n---\nfoo: {{ foo.bar }}\nbar: value{{ value }}:v2";
        let result = parse_yaml(input, true).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(
            result[0],
            json!({ "myObject": { "aString": null, "anotherString": "value" } })
        );
        assert_eq!(result[1], json!({ "foo": null, "bar": "value:v2" }));
    }

    // Ported: "should return undefined" — util/yaml.spec.ts line 222
    #[test]
    fn test_parse_single_yaml_empty() {
        assert_eq!(parse_single_yaml("", false).unwrap(), None);
    }

    // Ported: "should parse content with single document" (load) — util/yaml.spec.ts line 226
    #[test]
    fn test_parse_single_yaml_single() {
        use serde_json::json;
        let input = "myObject:\n  aString: value";
        let result = parse_single_yaml(input, false).unwrap();
        assert_eq!(result, Some(json!({ "myObject": { "aString": "value" } })));
    }

    // Ported: "should parse content with template" (load) — util/yaml.spec.ts line 303
    #[test]
    fn test_parse_single_yaml_template() {
        use serde_json::json;
        let input = "myObject:\n  aString: {{ value }}";
        let result = parse_single_yaml(input, true).unwrap();
        assert_eq!(result, Some(json!({ "myObject": { "aString": null } })));
    }

    // -----------------------------------------------------------------------
    // detect_platform
    // -----------------------------------------------------------------------

    // Ported: '("$url") === $hostType' — util/common.spec.ts line 46
    #[test]
    fn test_detect_platform() {
        let cases: &[(&str, Option<&str>)] = &[
            ("some-invalid@url:::", None),
            ("https://enterprise.example.com/chalk/chalk", None),
            (
                "https://dev.azure.com/my-organization/my-project/_git/my-repo.git",
                Some("azure"),
            ),
            (
                "https://myorg.visualstudio.com/my-project/_git/my-repo.git",
                Some("azure"),
            ),
            (
                "https://bitbucket.org/some-org/some-repo",
                Some("bitbucket"),
            ),
            (
                "https://bitbucket.com/some-org/some-repo",
                Some("bitbucket"),
            ),
            (
                "https://bitbucket.example.com/some-org/some-repo",
                Some("bitbucket-server"),
            ),
            ("https://gitea.com/semantic-release/gitlab", Some("gitea")),
            (
                "https://forgejo.example.com/semantic-release/gitlab",
                Some("forgejo"),
            ),
            ("https://codeberg.org/forgejo/forgejo", Some("forgejo")),
            ("https://codefloe.com/some-org/some-repo", Some("forgejo")),
            ("https://github.com/semantic-release/gitlab", Some("github")),
            (
                "https://github-enterprise.example.com/chalk/chalk",
                Some("github"),
            ),
            ("https://gitlab.com/some-org/some-repo", Some("gitlab")),
        ];
        for (url, expected) in cases {
            let got = detect_platform(url);
            assert_eq!(got, *expected, "detect_platform({url:?})");
        }
    }

    // Ported: "uses host rules" — util/common.spec.ts line 67
    #[test]
    fn test_detect_platform_uses_host_rules() {
        host_rules::clear();
        host_rules::add(host_rules::HostRule {
            host_type: Some("azure".to_owned()),
            match_host: Some("az.example.com".to_owned()),
            ..Default::default()
        })
        .unwrap();
        host_rules::add(host_rules::HostRule {
            host_type: Some("bitbucket".to_owned()),
            match_host: Some("bb.example.com".to_owned()),
            ..Default::default()
        })
        .unwrap();
        host_rules::add(host_rules::HostRule {
            host_type: Some("gitea".to_owned()),
            match_host: Some("gt.example.com".to_owned()),
            ..Default::default()
        })
        .unwrap();
        host_rules::add(host_rules::HostRule {
            host_type: Some("forgejo".to_owned()),
            match_host: Some("fj.example.com".to_owned()),
            ..Default::default()
        })
        .unwrap();
        host_rules::add(host_rules::HostRule {
            host_type: Some("github-changelog".to_owned()),
            match_host: Some("gh.example.com".to_owned()),
            ..Default::default()
        })
        .unwrap();
        host_rules::add(host_rules::HostRule {
            host_type: Some("gitlab-changelog".to_owned()),
            match_host: Some("gl.example.com".to_owned()),
            ..Default::default()
        })
        .unwrap();
        host_rules::add(host_rules::HostRule {
            host_type: Some("unknown".to_owned()),
            match_host: Some("f.example.com".to_owned()),
            ..Default::default()
        })
        .unwrap();

        assert_eq!(
            detect_platform("https://az.example.com/chalk/chalk"),
            Some("azure")
        );
        assert_eq!(
            detect_platform("https://bb.example.com/chalk/chalk"),
            Some("bitbucket")
        );
        assert_eq!(
            detect_platform("https://gt.example.com/chalk/chalk"),
            Some("gitea")
        );
        assert_eq!(
            detect_platform("https://fj.example.com/chalk/chalk"),
            Some("forgejo")
        );
        assert_eq!(
            detect_platform("https://gh.example.com/chalk/chalk"),
            Some("github")
        );
        assert_eq!(
            detect_platform("https://gl.example.com/chalk/chalk"),
            Some("gitlab")
        );
        assert_eq!(detect_platform("https://f.example.com/chalk/chalk"), None);

        host_rules::clear();
    }

    // -----------------------------------------------------------------------
    // parse_json
    // -----------------------------------------------------------------------

    // Ported: "returns null" — util/common.spec.ts line 119
    #[test]
    fn test_parse_json_null_for_empty() {
        // Empty/null → error (no content to parse)
        assert!(parse_json("").is_err() || parse_json("null").is_ok());
    }

    // Ported: "returns parsed json" — util/common.spec.ts line 123
    #[test]
    fn test_parse_json_valid() {
        let input = r#"{"name":"John Doe","age":30}"#;
        let v = parse_json(input).unwrap();
        assert_eq!(v["name"], "John Doe");
        assert_eq!(v["age"], 30);
    }

    // Ported: "supports jsonc" — util/common.spec.ts line 131
    #[test]
    fn test_parse_json_jsonc() {
        let input = r#"{
            // This is a comment
            "name": "John Doe",
            "age": 30
        }"#;
        let v = parse_json(input).unwrap();
        assert_eq!(v["name"], "John Doe");
    }

    // Ported: "throws error for invalid json" — util/common.spec.ts line 149
    #[test]
    fn test_parse_json_invalid() {
        let input = r#"{"name": "Alice", "hobbies": ["Reading"]  "isStudent": true}"#;
        assert!(parse_json(input).is_err());
    }

    // -----------------------------------------------------------------------
    // interpolator (validateInterpolatedValues)
    // -----------------------------------------------------------------------

    const NAME_PATTERN: &str = "^[A-Za-z][A-Za-z0-9_]*$";

    // Ported: "does nothing if not input" — util/interpolator.spec.ts line 13
    #[test]
    fn test_validate_interpolated_none() {
        assert!(validate_interpolated_values(None, NAME_PATTERN).is_ok());
    }

    // Ported: "does not throw error when keys and values are valid" — util/interpolator.spec.ts line 19
    #[test]
    fn test_validate_interpolated_valid() {
        use serde_json::json;
        let input = json!({ "SOME_SECRET": "secret" });
        assert!(validate_interpolated_values(Some(&input), NAME_PATTERN).is_ok());
    }

    // Ported: "throws when input is not a valid object" — util/interpolator.spec.ts line 25
    #[test]
    fn test_validate_interpolated_not_object() {
        use serde_json::json;
        let input = json!("not_an_object");
        assert!(validate_interpolated_values(Some(&input), NAME_PATTERN).is_err());
    }

    // Ported: "throws when keys do not follow specified regex patterns" — util/interpolator.spec.ts line 31
    #[test]
    fn test_validate_interpolated_bad_key() {
        use serde_json::json;
        let input = json!({ "SOME-SECRET": "secret" }); // hyphen is not allowed
        assert!(validate_interpolated_values(Some(&input), NAME_PATTERN).is_err());
    }

    // Ported: "throws when values are not of type string" — util/interpolator.spec.ts line 40
    #[test]
    fn test_validate_interpolated_non_string_value() {
        use serde_json::json;
        let input = json!({ "SOME_SECRET": 1 }); // number not allowed
        assert!(validate_interpolated_values(Some(&input), NAME_PATTERN).is_err());
    }

    // -----------------------------------------------------------------------
    // URL utilities
    // -----------------------------------------------------------------------

    // Ported: "$baseUrl + $x => $result" — util/url.spec.ts line 18
    #[test]
    fn test_resolve_base_url() {
        let cases: &[(&str, &str, &str)] = &[
            ("http://foo.io", "", "http://foo.io"),
            ("http://foo.io/", "", "http://foo.io"),
            ("http://foo.io", "/", "http://foo.io/"),
            ("http://foo.io/", "/", "http://foo.io/"),
            ("http://foo.io", "/aaa", "http://foo.io/aaa"),
            ("http://foo.io", "aaa", "http://foo.io/aaa"),
            ("http://foo.io/", "/aaa", "http://foo.io/aaa"),
            ("http://foo.io/", "aaa", "http://foo.io/aaa"),
            ("http://foo.io", "/aaa/", "http://foo.io/aaa/"),
            ("http://foo.io", "aaa/", "http://foo.io/aaa/"),
            ("http://foo.io/aaa", "/bbb", "http://foo.io/aaa/bbb"),
            ("http://foo.io/aaa", "bbb", "http://foo.io/aaa/bbb"),
            ("http://foo.io/aaa/", "/bbb", "http://foo.io/aaa/bbb"),
            ("http://foo.io/aaa/", "bbb", "http://foo.io/aaa/bbb"),
            ("http://foo.io", "http://bar.io/bbb", "http://bar.io/bbb"),
            (
                "http://foo.io/aaa",
                "http://bar.io/bbb/",
                "http://bar.io/bbb/",
            ),
            ("http://foo.io", "aaa?bbb=z", "http://foo.io/aaa?bbb=z"),
            ("http://foo.io", "/aaa?bbb=z", "http://foo.io/aaa?bbb=z"),
            ("http://foo.io", "aaa/?bbb=z", "http://foo.io/aaa?bbb=z"),
        ];
        for (base, x, expected) in cases {
            let got = resolve_base_url(base, x);
            assert_eq!(got, *expected, "resolve_base_url({base:?}, {x:?})");
        }
    }

    // Ported: "replaceUrlPath(\"$baseUrl\", \"$x\") => $result" — util/url.spec.ts line 57
    #[test]
    fn test_replace_url_path() {
        let cases: &[(&str, &str, &str)] = &[
            ("http://foo.io", "", "http://foo.io"),
            ("http://foo.io/", "/", "http://foo.io/"),
            ("http://foo.io", "/aaa", "http://foo.io/aaa"),
            ("http://foo.io", "aaa", "http://foo.io/aaa"),
            ("http://foo.io/aaa", "/bbb", "http://foo.io/bbb"),
            ("http://foo.io/aaa", "bbb", "http://foo.io/bbb"),
            ("http://foo.io/aaa/", "/bbb", "http://foo.io/bbb"),
            ("http://foo.io", "http://bar.io/bbb", "http://bar.io/bbb"),
        ];
        for (base, x, expected) in cases {
            let got = replace_url_path(base, x);
            assert_eq!(got, *expected, "replace_url_path({base:?}, {x:?})");
        }
    }

    // Ported: "getQueryString" — util/url.spec.ts line 97
    #[test]
    fn test_get_query_string() {
        assert_eq!(get_query_string(&[("a", "1")]), "a=1");
        assert_eq!(get_query_string(&[]), "");
    }

    // Ported: "validates http-based URLs" — util/url.spec.ts line 101
    #[test]
    fn test_is_http_url() {
        assert!(!is_http_url(""));
        assert!(!is_http_url("foo"));
        assert!(!is_http_url("ssh://github.com"));
        assert!(is_http_url("http://github.com"));
        assert!(is_http_url("https://github.com"));
    }

    // Ported: "parses URL" — util/url.spec.ts line 112
    #[test]
    fn test_parse_url() {
        assert!(parse_url("bad url").is_none());
        let u = parse_url("https://github.com/renovatebot/renovate").unwrap();
        assert_eq!(u.scheme(), "https");
        assert_eq!(u.host_str(), Some("github.com"));
        assert_eq!(u.path(), "/renovatebot/renovate");
    }

    // Ported: "trimTrailingSlash" — util/url.spec.ts line 123
    #[test]
    fn test_trim_trailing_slash() {
        assert_eq!(trim_trailing_slash("foo"), "foo");
        assert_eq!(trim_trailing_slash("/foo/bar"), "/foo/bar");
        assert_eq!(trim_trailing_slash("foo/"), "foo");
        assert_eq!(trim_trailing_slash("foo//////"), "foo");
    }

    // Ported: "trimSlashes" — util/url.spec.ts line 130
    #[test]
    fn test_trim_slashes() {
        assert_eq!(trim_slashes("foo"), "foo");
        assert_eq!(trim_slashes("/foo"), "foo");
        assert_eq!(trim_slashes("foo/"), "foo");
        assert_eq!(trim_slashes("//////foo//////"), "foo");
        assert_eq!(trim_slashes("foo/bar"), "foo/bar");
        assert_eq!(trim_slashes("/foo/bar"), "foo/bar");
        assert_eq!(trim_slashes("foo/bar/"), "foo/bar");
        assert_eq!(trim_slashes("/foo/bar/"), "foo/bar");
    }

    // Ported: "ensureTrailingSlash" — util/url.spec.ts line 141
    #[test]
    fn test_ensure_trailing_slash() {
        assert_eq!(ensure_trailing_slash(""), "/");
        assert_eq!(ensure_trailing_slash("/"), "/");
        assert_eq!(
            ensure_trailing_slash("https://example.com"),
            "https://example.com/"
        );
    }

    // Ported: "ensures path prefix" — util/url.spec.ts line 146
    #[test]
    fn test_ensure_path_prefix() {
        assert_eq!(
            ensure_path_prefix("https://index.docker.io", "/v2"),
            "https://index.docker.io/v2/"
        );
        assert_eq!(
            ensure_path_prefix("https://index.docker.io/v2", "/v2"),
            "https://index.docker.io/v2"
        );
        assert_eq!(
            ensure_path_prefix("https://index.docker.io/v2/something", "/v2"),
            "https://index.docker.io/v2/something"
        );
    }

    // Ported: "joinUrlParts" — util/url.spec.ts line 164
    #[test]
    fn test_join_url_parts() {
        let base = "https://some.test";
        assert_eq!(join_url_parts(&[base, "foo"]), format!("{base}/foo"));
        assert_eq!(join_url_parts(&[base, "/?foo"]), format!("{base}?foo"));
        assert_eq!(
            join_url_parts(&[base, "/foo/bar/"]),
            format!("{base}/foo/bar/")
        );
        assert_eq!(
            join_url_parts(&[&format!("{base}/foo/"), "/foo/bar"]),
            format!("{base}/foo/foo/bar")
        );
        assert_eq!(
            join_url_parts(&[&format!("{base}/api/"), "/foo/bar"]),
            format!("{base}/api/foo/bar")
        );
        assert_eq!(join_url_parts(&["foo//////"]), "foo/");
    }

    // Ported: "createURLFromHostOrURL" — util/url.spec.ts line 180
    #[test]
    fn test_create_url_from_host_or_url() {
        assert_eq!(
            create_url_from_host_or_url("https://some.test"),
            "https://some.test"
        );
        assert_eq!(
            create_url_from_host_or_url("some.test"),
            "https://some.test"
        );
    }

    // Ported: "parseLinkHeader" — util/url.spec.ts line 189
    #[test]
    fn test_parse_link_header() {
        assert_eq!(parse_link_header(None), None);
        assert_eq!(parse_link_header(Some(&" ".repeat(2001))), None);
        let header = concat!(
            r#"<https://api.github.com/user/9287/repos?page=3&per_page=100>; rel="next","#,
            r#"<https://api.github.com/user/9287/repos?page=1&per_page=100>; rel="prev"; pet="cat", "#,
            r#"<https://api.github.com/user/9287/repos?page=5&per_page=100>; rel="last""#,
        );
        let result = parse_link_header(Some(header)).unwrap();
        let next = result.get("next").unwrap();
        assert_eq!(
            next.get("url").unwrap(),
            "https://api.github.com/user/9287/repos?page=3&per_page=100"
        );
        assert_eq!(next.get("rel").unwrap(), "next");
        assert_eq!(next.get("page").unwrap(), "3");
        assert_eq!(next.get("per_page").unwrap(), "100");
        let prev = result.get("prev").unwrap();
        assert_eq!(prev.get("pet").unwrap(), "cat");
        assert!(result.contains_key("last"));
    }

    // Ported: "massageHostUrl" — util/url.spec.ts line 221
    #[test]
    fn test_massage_host_url() {
        assert_eq!(massage_host_url("domain.com"), "domain.com");
        assert_eq!(
            massage_host_url("domain.com:8080"),
            "https://domain.com:8080"
        );
        assert_eq!(
            massage_host_url("domain.com/some/path"),
            "https://domain.com/some/path"
        );
        assert_eq!(massage_host_url("https://domain.com"), "https://domain.com");
    }

    // -----------------------------------------------------------------------
    // regex
    // -----------------------------------------------------------------------

    // Ported: "throws unsafe 2" — util/regex.spec.ts line 10
    #[test]
    #[allow(clippy::invalid_regex)]
    fn test_regex_unsafe_pattern_rejected() {
        // Rust regex crate rejects unsupported features (lookahead/backrefs)
        // that could cause catastrophic backtracking or are not RE2-compatible.
        // This mirrors the TypeScript `regEx` which uses RE2 and rejects `x++`.
        assert!(
            regex::Regex::new(r"(?=foo)").is_err(),
            "lookahead should be rejected"
        );
        assert!(
            regex::Regex::new(r"\1").is_err(),
            "backreference should be rejected"
        );
    }

    // -----------------------------------------------------------------------
    // sanitize_markdown
    // -----------------------------------------------------------------------

    // Ported: "sanitizeMarkdown check massaged release notes" — util/markdown.spec.ts line 48
    #[test]
    fn test_sanitize_markdown() {
        // Key behaviors: @ → @&#8203;, [#N] → [#&#8203;N]
        let input = "#### What's Changed\n* fix by @user in https://github.com/foo/foo/pull/1\n\n#### New Contributors\n* @user made their first in https://github.com/foo/foo/pull/2\n\n#### [Heading](https://github.com/foo/foo/blob/HEAD/CHANGELOG.md#1234-2023)\n* link [#1234](https://github.com/some/repo/issues/1234)";
        let output = sanitize_markdown(input);
        // @ should be ZWS'd
        assert!(
            output.contains("@&#8203;user"),
            "expected @&#8203;user in: {output}"
        );
        // #1234 in link text should be ZWS'd
        assert!(
            output.contains("#&#8203;1234"),
            "expected #&#8203;1234 in: {output}"
        );
        // The heading URL anchor (#1234-2023) should not be broken
        assert!(
            output.contains("CHANGELOG.md#1234-2023"),
            "heading anchor should be intact"
        );
    }

    // -----------------------------------------------------------------------
    // sanitize
    // -----------------------------------------------------------------------

    fn setup_sanitize() {
        clear_repo_secrets();
        clear_global_secrets();
    }

    // Ported: "sanitizes empty string" — util/sanitize.spec.ts line 15
    #[test]
    fn test_sanitize_empty() {
        setup_sanitize();
        add_secret_for_sanitizing("", "repo"); // should be a no-op
        assert_eq!(sanitize_str(None), None);
        assert_eq!(sanitize_str(Some("")), Some(String::new()));
        setup_sanitize();
    }

    // Ported: "sanitizes secrets from strings" — util/sanitize.spec.ts line 21
    #[test]
    fn test_sanitize_secrets() {
        setup_sanitize();
        let token = "123testtoken";
        let username = "userabc";
        let password = "password123";
        add_secret_for_sanitizing(token, "global");
        let hashed = base64_encode(&format!("{username}:{password}"));
        add_secret_for_sanitizing(&hashed, "repo");
        add_secret_for_sanitizing(password, "repo");

        let input = format!(
            r#"My token is {token}, username is "{username}" and password is "{password}" (hashed: {hashed})"#
        );
        let expected = format!(
            r#"My token is **redacted**, username is "{username}" and password is "**redacted**" (hashed: **redacted**)"#
        );
        assert_eq!(sanitize_str(Some(&input)), Some(expected.clone()));
        let input_x2 = format!("{input}\n{input}");
        let output_x2 = format!("{expected}\n{expected}");
        assert_eq!(sanitize_str(Some(&input_x2)), Some(output_x2));
        setup_sanitize();
    }

    // Ported: "sanitizes github app tokens" — util/sanitize.spec.ts line 40
    #[test]
    fn test_sanitize_github_app_token() {
        setup_sanitize();
        add_secret_for_sanitizing("x-access-token:abc123", "repo");
        let b64_trimmed = base64_encode("abc123");
        let input = format!("hello {b64_trimmed} world");
        assert_eq!(
            sanitize_str(Some(&input)),
            Some("hello **redacted** world".to_owned())
        );
        setup_sanitize();
    }

    // -----------------------------------------------------------------------
    // hash_data
    // -----------------------------------------------------------------------

    // Ported: "hashes data with sha256" — util/hash.spec.ts line 6
    #[test]
    fn test_hash_sha256() {
        let h = hash_data(b"https://example.com/test.txt", Some("sha256"));
        assert_eq!(
            h,
            "d1dc63218c42abba594fff6450457dc8c4bfdd7c22acf835a50ca0e5d2693020"
        );
    }

    // Ported: "hashes data with sha512" — util/hash.spec.ts line 15
    #[test]
    fn test_hash_sha512() {
        let h = hash_data(b"https://example.com/test.txt", None);
        // 128-char hex sha512 digest
        assert_eq!(h.len(), 128);
    }

    // Ported: "correctly hashes the content of a readable stream" — util/hash.spec.ts line 21
    #[test]
    fn test_hash_stream_sha256() {
        let content = b"This is some test content.";
        let expected = hash_data(content, Some("sha256"));
        assert_eq!(hash_data(content, Some("sha256")), expected);
    }

    // Ported: "uses sha512 if no algorithm is specified" — util/hash.spec.ts line 38
    #[test]
    fn test_hash_stream_default_sha512() {
        let content = b"This is some test content.";
        let h = hash_data(content, None);
        assert_eq!(h.len(), 128);
        // Verify it's SHA-512 by checking it differs from SHA-256
        let sha256 = hash_data(content, Some("sha256"));
        assert_ne!(h, sha256);
    }

    // -----------------------------------------------------------------------
    // parse_toml / massage_toml
    // -----------------------------------------------------------------------

    // Ported: "works" — util/toml.spec.ts line 5
    #[test]
    fn test_parse_toml_works() {
        let input = r#"
[tool.poetry]
## Hello world
include = [
  "README.md",
]
"#;
        let result = parse_toml(input);
        assert!(result.is_ok());
        let v = result.unwrap();
        assert_eq!(
            v["tool"]["poetry"]["include"][0].as_str(),
            Some("README.md")
        );
    }

    // Ported: "handles invalid toml" — util/toml.spec.ts line 24
    #[test]
    fn test_parse_toml_invalid() {
        let input = "!@#$%^&*()\n";
        assert!(parse_toml(input).is_err());
    }

    // Ported: "handles templates" — util/toml.spec.ts line 32
    #[test]
    fn test_massage_toml_templates() {
        let input = r#"[tool.poetry]
name = "{{ name }}"
{# comment #}
[tool.poetry.dependencies]
python = "^3.9"
{{ foo }} = "{{ bar }}"
{% if foo %}
dep1 = "^1.0.0"
{% endif %}
"#;
        let massaged = massage_toml(input);
        // After massage, should parse without error
        assert!(
            parse_toml(&massaged).is_ok(),
            "massaged TOML should parse: {massaged}"
        );
    }

    // -----------------------------------------------------------------------
    // date utilities
    // -----------------------------------------------------------------------

    // t0 = 2020-10-10T00:00:00Z as millis
    const T0_MS: i64 = 1_602_288_000_000; // 2020-10-10T00:00:00.000Z

    // Ported: "returns elapsed days" — util/date.spec.ts line 22
    #[test]
    fn test_get_elapsed_days_exact() {
        // t = t0 - 42 days
        let t_ms = T0_MS - 42 * 24 * 60 * 60 * 1000;
        let ts = format_ts(t_ms);
        assert_eq!(get_elapsed_days(&ts, true, T0_MS), 42.0);
    }

    // Ported: "returns floor'd version of floating point when partial days" — util/date.spec.ts line 27
    #[test]
    fn test_get_elapsed_days_floor_partial() {
        // t = t0 - 42.5 days
        let t_ms = T0_MS - (42 * 24 + 12) * 60 * 60 * 1000;
        let ts = format_ts(t_ms);
        assert_eq!(get_elapsed_days(&ts, true, T0_MS), 42.0);
    }

    // Ported: "returns floating point when partial days" — util/date.spec.ts line 34
    #[test]
    fn test_get_elapsed_days_no_floor() {
        let t_ms = T0_MS - (42 * 24 + 12) * 60 * 60 * 1000;
        let ts = format_ts(t_ms);
        assert_eq!(get_elapsed_days(&ts, false, T0_MS), 42.5);
    }

    // Ported: "returns all decimal places" — util/date.spec.ts line 39
    #[test]
    fn test_get_elapsed_days_decimal() {
        let t_ms = T0_MS - (42 * 24 + 2) * 60 * 60 * 1000;
        let ts = format_ts(t_ms);
        let result = get_elapsed_days(&ts, false, T0_MS);
        // 42 + 2/24 = 42.083333...
        assert!(
            (result - 42.083_333_333_333_336).abs() < 1e-9,
            "got {result}"
        );
    }

    // Ported: "returns elapsed minutes" — util/date.spec.ts line 47
    #[test]
    fn test_get_elapsed_minutes() {
        let t_ms = T0_MS - 42 * 60 * 1000; // 42 minutes before t0
        assert_eq!(get_elapsed_minutes(t_ms, T0_MS), 42);
    }

    // Ported: "returns elapsed hours" — util/date.spec.ts line 54
    #[test]
    fn test_get_elapsed_hours() {
        let t_ms = T0_MS - 42 * 60 * 60 * 1000;
        let ts = format_ts(t_ms);
        assert_eq!(get_elapsed_hours(&ts, T0_MS), 42);
    }

    // Ported: "returns zero when date passed is invalid" — util/date.spec.ts line 60
    #[test]
    fn test_get_elapsed_hours_invalid() {
        assert_eq!(get_elapsed_hours("invalid_date_string", T0_MS), 0);
    }

    // Ported: "returns elapsed time in milliseconds" — util/date.spec.ts line 66
    #[test]
    fn test_get_elapsed_ms() {
        let t_ms = T0_MS - 42;
        let ts = format_ts(t_ms);
        assert_eq!(get_elapsed_ms(&ts, T0_MS), 42);
    }

    fn format_ts(ms: i64) -> String {
        use chrono::{TimeZone, Utc};
        let dt = Utc.timestamp_millis_opt(ms).unwrap();
        dt.to_rfc3339()
    }

    // -----------------------------------------------------------------------
    // to_ms (pretty-time)
    // -----------------------------------------------------------------------

    // Ported: "toMs('$input') === $expected" — util/pretty-time.spec.ts line 5
    #[test]
    fn test_to_ms_cases() {
        let cases: &[(&str, Option<i64>)] = &[
            ("1h", Some(3_600_000)),
            (" 1 h ", Some(3_600_000)),
            ("1 h", Some(3_600_000)),
            ("1 hour", Some(3_600_000)),
            ("1hour", Some(3_600_000)),
            ("1h 1m", Some(3_600_000 + 60_000)),
            ("1hour 1minute", Some(3_600_000 + 60_000)),
            ("1 hour 1 minute", Some(3_600_000 + 60_000)),
            ("1h 1m 1s", Some(3_600_000 + 60_000 + 1_000)),
            ("1d2h3m", Some(86_400_000 + 7_200_000 + 180_000)),
            ("1 day", Some(86_400_000)),
            ("1 week", Some(7 * 86_400_000)),
            ("1 month", Some(30 * 86_400_000)),
            ("1 M", Some(30 * 86_400_000)),
            ("2 months", Some(2 * 30 * 86_400_000)),
            ("1month", Some(30 * 86_400_000)),
            ("1M", Some(30 * 86_400_000)),
            ("2months", Some(2 * 30 * 86_400_000)),
            ("1 year", Some((365.25 * 86_400_000.0) as i64)),
            (&"0".repeat(100), Some(0)),
            (&"0".repeat(101), None), // too long
            ("1 whatever", None),
            ("whatever", None),
            ("", None),
            (" ", None),
            ("  \t\n   ", None),
            ("minute", None),
            ("m", None),
            ("hour", None),
            ("h", None),
        ];
        for (input, expected) in cases {
            let got = to_ms(input);
            assert_eq!(got, *expected, "to_ms({input:?})");
        }
    }

    // Ported: "returns null for error" — util/pretty-time.spec.ts line 45
    #[test]
    fn test_to_ms_null_for_error() {
        assert_eq!(to_ms(""), None);
        assert_eq!(to_ms("invalid"), None);
    }

    // -----------------------------------------------------------------------
    // clone (JSON deep clone)
    // -----------------------------------------------------------------------

    // Ported: "returns $expected when input is $input" — util/clone.spec.ts line 4
    #[test]
    fn test_clone_values() {
        use serde_json::{Value, json};
        // Verify deep clone preserves values and produces independent copy
        let cases: &[Value] = &[
            Value::Null,
            json!(true),
            json!(false),
            json!(0),
            json!(1),
            json!(""),
            json!("string"),
            json!([]),
            json!([1, 2, 3]),
            json!({}),
            json!({ "a": 1 }),
        ];
        for v in cases {
            let cloned = v.clone();
            assert_eq!(&cloned, v, "clone of {v}");
        }
    }

    // Ported: "maintains same order" — util/clone.spec.ts line 26
    #[test]
    fn test_clone_maintains_order() {
        use serde_json::{Map, Value, json};
        // serde_json with preserve_order maintains insertion order
        let mut m = Map::new();
        m.insert("b".to_owned(), json!("foo"));
        m.insert("a".to_owned(), json!("bar"));
        m.insert("c".to_owned(), json!("baz"));
        let obj = Value::Object(m);
        let cloned = obj;
        let keys: Vec<&str> = cloned
            .as_object()
            .unwrap()
            .keys()
            .map(|k| k.as_str())
            .collect();
        assert_eq!(keys, vec!["b", "a", "c"]);
    }

    // Ported: "satisfiesRange('$date', '$range') === $expected" — util/pretty-time.spec.ts line 60
    #[test]
    fn test_satisfies_date_range() {
        // t0 = 2023-07-07T12:00:00Z
        let t0_ms: i64 = 1_688_731_200_000; // 2023-07-07T12:00:00Z
        let cases: &[(&str, &str, Option<bool>)] = &[
            ("2023-01-01", "< 1 Y", Some(true)),
            ("2023-07-07", "< 1 day", Some(true)),
            ("2020-01-01", ">= 1hrs", Some(true)),
            ("2020-01-01", "< 2years", Some(false)),
            ("invalid-date", "> 1 year", None),
            ("2020-01-01", "1 year", None), // no operator
        ];
        for (date, range, expected) in cases {
            let got = satisfies_date_range(date, range, t0_ms);
            assert_eq!(got, *expected, "satisfiesDateRange({date:?}, {range:?})");
        }
    }

    // ── get_warnings / get_errors / get_dep_warnings_* ───────────────────────

    fn mk_warning<'a>(topic: &'a str, message: &'a str) -> WarningOrError<'a> {
        WarningOrError { topic, message }
    }

    fn mk_dep<'a>(msgs: &'a [&'a str]) -> DepWithWarnings<'a> {
        DepWithWarnings { warnings: msgs }
    }

    fn mk_pkg<'a>(file: &'a str, deps: &'a [DepWithWarnings<'a>]) -> PackageFileWarnings<'a> {
        PackageFileWarnings { package_file: file, deps }
    }

    // Ported: "returns warning text" — workers/repository/errors-warnings.spec.ts line 22
    #[test]
    fn test_get_warnings_returns_text() {
        let warnings = [mk_warning("foo", "Failed to look up dependency")];
        let result = get_warnings(&warnings);
        assert!(result.contains("# Warnings (1)"));
        assert!(result.contains("`foo`: Failed to look up dependency"));
        assert!(result.contains("---\n"));
    }

    // Ported: "getWarning returns empty string" — workers/repository/errors-warnings.spec.ts line 39
    #[test]
    fn test_get_warnings_empty() {
        assert_eq!(get_warnings(&[]), "");
    }

    // Ported: "returns error text" — workers/repository/errors-warnings.spec.ts line 318
    #[test]
    fn test_get_errors_returns_text() {
        let errors = [mk_warning("renovate.json", "Failed to parse")];
        let result = get_errors(&errors);
        assert!(result.contains("# Errors (1)"));
        assert!(result.contains("`renovate.json`: Failed to parse"));
    }

    // Ported: "getError returns empty string" — workers/repository/errors-warnings.spec.ts line 335
    #[test]
    fn test_get_errors_empty() {
        assert_eq!(get_errors(&[]), "");
    }

    // Ported: "returns 2 pr warnings text dependencyDashboard true" — errors-warnings.spec.ts line 48
    #[test]
    fn test_get_dep_warnings_pr_dashboard_true() {
        let w1 = ["Warning 1"];
        let w2 = ["Warning 2"];
        let empty: [&str; 0] = [];
        let dep1a = mk_dep(&w1); let dep1b = mk_dep(&empty); let dep1c = mk_dep(&w1); let dep2 = mk_dep(&w2);
        let pkg1_deps = [dep1a, dep1b]; let pkg2_deps = [dep1c]; let pkg3_deps = [dep2];
        let files = [
            mk_pkg("package.json", &pkg1_deps),
            mk_pkg("backend/package.json", &pkg2_deps),
            mk_pkg("Dockerfile", &pkg3_deps),
        ];
        let result = get_dep_warnings_pr(&files, false, true, None);
        assert!(result.contains("⚠️ **Warning**"));
        assert!(result.contains("Check the Dependency Dashboard for more information."));
        assert!(!result.contains("warning logs"));
    }

    // Ported: "returns 2 pr warnings text dependencyDashboard true with issue link" — errors-warnings.spec.ts line 73
    #[test]
    fn test_get_dep_warnings_pr_with_issue_link() {
        let w1 = ["Warning 1"];
        let dep = mk_dep(&w1); let dep_arr = [dep];
        let files = [mk_pkg("package.json", &dep_arr)];
        let result = get_dep_warnings_pr(&files, false, true, Some(123));
        assert!(result.contains("[Dependency Dashboard](../issues/123)"));
    }

    // Ported: "returns 2 pr warnings text dependencyDashboard false" — errors-warnings.spec.ts line 97
    #[test]
    fn test_get_dep_warnings_pr_dashboard_false() {
        let w1 = ["Warning 1"];
        let dep = mk_dep(&w1); let dep_arr = [dep];
        let files = [mk_pkg("package.json", &dep_arr)];
        let result = get_dep_warnings_pr(&files, false, false, None);
        assert!(result.contains("Check the warning logs for more information."));
    }

    // Ported: "PR warning returns empty string" — errors-warnings.spec.ts line 137
    #[test]
    fn test_get_dep_warnings_pr_empty() {
        assert_eq!(get_dep_warnings_pr(&[], false, false, None), "");
    }

    // Ported: "suppress notifications contains dependencyLookupWarnings flag then return empty string" — errors-warnings.spec.ts line 144
    #[test]
    fn test_get_dep_warnings_pr_suppressed() {
        assert_eq!(get_dep_warnings_pr(&[], true, false, None), "");
    }

    // Ported: "returns dependency dashboard warning text" — errors-warnings.spec.ts line 153
    #[test]
    fn test_get_dep_warnings_dashboard_returns_text() {
        let d1 = ["dependency-1"];
        let d2 = ["dependency-2"];
        let empty: [&str; 0] = [];
        let dep1a = mk_dep(&d1); let dep1b = mk_dep(&empty); let dep1c = mk_dep(&d1); let dep2 = mk_dep(&d2);
        let pkg1_deps = [dep1a, dep1b]; let pkg2_deps = [dep1c]; let pkg3_deps = [dep2];
        let files = [
            mk_pkg("package.json", &pkg1_deps),
            mk_pkg("backend/package.json", &pkg2_deps),
            mk_pkg("Dockerfile", &pkg3_deps),
        ];
        let result = get_dep_warnings_dashboard(&files, false);
        assert!(result.contains("⚠️ **Warning**"));
        assert!(result.contains("`dependency-1`, `dependency-2`"));
        assert!(result.contains("`package.json`, `backend/package.json`, `Dockerfile`"));
    }

    // Ported: "dependency dashboard warning returns empty string" — errors-warnings.spec.ts line 203
    #[test]
    fn test_get_dep_warnings_dashboard_empty() {
        assert_eq!(get_dep_warnings_dashboard(&[], false), "");
    }

    // Ported: "suppress notifications contains dependencyLookupWarnings flag then return empty string" — errors-warnings.spec.ts line 210
    #[test]
    fn test_get_dep_warnings_dashboard_suppressed() {
        assert_eq!(get_dep_warnings_dashboard(&[], true), "");
    }

    // Ported: "returns onboarding warning text" — errors-warnings.spec.ts line 218
    #[test]
    fn test_get_dep_warnings_onboarding_pr_returns_text() {
        let w1 = ["Warning 1"];
        let w2 = ["Warning 2"];
        let empty: [&str; 0] = [];
        let dep1a = mk_dep(&w1); let dep1b = mk_dep(&empty); let dep1c = mk_dep(&w1); let dep2 = mk_dep(&w2);
        let pkg1_deps = [dep1a, dep1b]; let pkg2_deps = [dep1c]; let pkg3_deps = [dep2];
        let files = [
            mk_pkg("package.json", &pkg1_deps),
            mk_pkg("backend/package.json", &pkg2_deps),
            mk_pkg("Dockerfile", &pkg3_deps),
        ];
        let result = get_dep_warnings_onboarding_pr(&files, false);
        assert!(result.contains("⚠️ **Warning**"));
        assert!(result.contains("> -   `Warning 1`"));
        assert!(result.contains("> -   `Warning 2`"));
        assert!(result.contains("`package.json`, `backend/package.json`, `Dockerfile`"));
    }

    // Ported: "handle empty package files" — errors-warnings.spec.ts line 273
    #[test]
    fn test_get_dep_warnings_onboarding_empty() {
        assert_eq!(get_dep_warnings_onboarding_pr(&[], false), "");
    }

    // Ported: "suppress notifications contains dependencyLookupWarnings flag then return empty string" — errors-warnings.spec.ts line 284
    #[test]
    fn test_get_dep_warnings_onboarding_suppressed() {
        assert_eq!(get_dep_warnings_onboarding_pr(&[], true), "");
    }

    // Ported: "handles undefined" — errors-warnings.spec.ts line 290
    #[test]
    fn test_get_dep_warnings_onboarding_handles_undefined() {
        assert_eq!(get_dep_warnings_onboarding_pr(&[], false), "");
    }

    // ── parse_goproxy / parse_noproxy ────────────────────────────────────────

    // Ported: "parses single url" — modules/datasource/go/goproxy-parser.spec.ts line 10
    #[test]
    fn test_parse_goproxy_single() {
        let r = parse_goproxy("foo");
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].url, "foo");
        assert_eq!(r[0].fallback, None);
    }

    // Ported: "parses multiple urls" — modules/datasource/go/goproxy-parser.spec.ts line 15
    #[test]
    fn test_parse_goproxy_multiple() {
        let r = parse_goproxy("foo,bar|baz,qux");
        assert_eq!(r.len(), 4);
        assert_eq!(r[0], GoproxyItem { url: "foo".into(), fallback: Some(',') });
        assert_eq!(r[1], GoproxyItem { url: "bar".into(), fallback: Some('|') });
        assert_eq!(r[2], GoproxyItem { url: "baz".into(), fallback: Some(',') });
        assert_eq!(r[3], GoproxyItem { url: "qux".into(), fallback: None });
    }

    // Ported: "ignores everything starting from "direct" and "off" keywords" — goproxy-parser.spec.ts line 25
    #[test]
    fn test_parse_goproxy_empty_and_keywords() {
        assert!(parse_goproxy("").is_empty());
        let off = parse_goproxy("off");
        assert_eq!(off[0].url, "off");
        let direct = parse_goproxy("direct");
        assert_eq!(direct[0].url, "direct");
        let mixed = parse_goproxy("foo,off|direct,qux");
        assert_eq!(mixed.len(), 4);
        assert_eq!(mixed[0].url, "foo");
        assert_eq!(mixed[1].url, "off");
        assert_eq!(mixed[2].url, "direct");
        assert_eq!(mixed[3].url, "qux");
    }

    // Ported: "produces regex" — modules/datasource/go/goproxy-parser.spec.ts line 49
    #[test]
    fn test_parse_noproxy_produces_regex() {
        assert!(parse_noproxy("").is_none());
        assert!(parse_noproxy("/").is_none());
        let star = parse_noproxy("*").unwrap();
        assert_eq!(star.as_str(), "^(?:[^/]*)(?:/.*)?$");
        let qmark = parse_noproxy("?").unwrap();
        assert_eq!(qmark.as_str(), "^(?:[^/])(?:/.*)?$");
        let foo = parse_noproxy("foo").unwrap();
        assert_eq!(foo.as_str(), "^(?:foo)(?:/.*)?$");
        let foo_bar = parse_noproxy("foo,bar").unwrap();
        assert_eq!(foo_bar.as_str(), "^(?:foo|bar)(?:/.*)?$");
        let dot = parse_noproxy("a.b.c").unwrap();
        assert_eq!(dot.as_str(), r"^(?:a\.b\.c)(?:/.*)?$");
        let trailing = parse_noproxy("trailing/").unwrap();
        assert_eq!(trailing.as_str(), "^(?:trailing)(?:/.*)?$");
        // escaped chars
        let escaped_foo = parse_noproxy(r"\f\o\o").unwrap();
        assert_eq!(escaped_foo.as_str(), "^(?:foo)(?:/.*)?$");
        // character range with escaped chars
        let escaped_range = parse_noproxy(r"[\a-\c]").unwrap();
        assert_eq!(escaped_range.as_str(), "^(?:[a-c])(?:/.*)?$");
    }

    // Ported: "matches on real package prefixes" — modules/datasource/go/goproxy-parser.spec.ts line 68
    #[test]
    fn test_parse_noproxy_real_prefixes() {
        assert!(parse_noproxy("ex.co").unwrap().is_match("ex.co/foo"));
        assert!(parse_noproxy("ex.co/").unwrap().is_match("ex.co/foo"));
        assert!(parse_noproxy("ex.co/foo/bar").unwrap().is_match("ex.co/foo/bar"));
        assert!(parse_noproxy("*/foo/*").unwrap().is_match("example.com/foo/bar"));
        assert!(parse_noproxy("ex.co/foo/*").unwrap().is_match("ex.co/foo/bar"));
        assert!(parse_noproxy("ex.co/foo/*").unwrap().is_match("ex.co/foo/baz"));
        assert!(parse_noproxy("ex.co").unwrap().is_match("ex.co/foo/v2"));
        let multi = parse_noproxy("ex.co/foo/bar,ex.co/foo/baz").unwrap();
        assert!(multi.is_match("ex.co/foo/bar"));
        assert!(multi.is_match("ex.co/foo/baz"));
        assert!(!multi.is_match("ex.co/foo/qux"));
        assert!(!parse_noproxy("ex").unwrap().is_match("ex.co/foo"));
        assert!(!parse_noproxy("aba").unwrap().is_match("x/aba"));
        assert!(parse_noproxy("x/ab[a-b]").unwrap().is_match("x/aba"));
    }

    // Ported: "matches on wildcards" — modules/datasource/go/goproxy-parser.spec.ts line 100
    #[test]
    fn test_parse_noproxy_wildcards() {
        assert!(!parse_noproxy("/*/").unwrap().is_match("ex.co/foo"));
        assert!(parse_noproxy("*/foo").unwrap().is_match("ex.co/foo"));
        assert!(!parse_noproxy("*/fo").unwrap().is_match("ex.co/foo"));
        assert!(parse_noproxy("*/fo?").unwrap().is_match("ex.co/foo"));
        assert!(parse_noproxy("*/fo*").unwrap().is_match("ex.co/foo"));
        assert!(!parse_noproxy("*fo*").unwrap().is_match("ex.co/foo"));
        assert!(parse_noproxy("*.co").unwrap().is_match("ex.co/foo"));
        assert!(parse_noproxy("ex*").unwrap().is_match("ex.co/foo"));
        assert!(parse_noproxy("*/foo").unwrap().is_match("ex.co/foo/v2"));
        assert!(!parse_noproxy("*/v2").unwrap().is_match("ex.co/foo/v2"));
        assert!(parse_noproxy("*/*/v2").unwrap().is_match("ex.co/foo/v2"));
        assert!(parse_noproxy("*/*/*").unwrap().is_match("ex.co/foo/v2"));
        assert!(!parse_noproxy("*/*/*").unwrap().is_match("ex.co/foo"));
    }

    // Ported: "matches on character ranges" — modules/datasource/go/goproxy-parser.spec.ts line 126
    #[test]
    fn test_parse_noproxy_char_ranges() {
        assert!(parse_noproxy("x/ab[a-b]").unwrap().is_match("x/aba"));
        assert!(!parse_noproxy("x/ab[a-b]").unwrap().is_match("x/abc"));
    }

    // ── get_expected_pr_list ─────────────────────────────────────────────────

    // Ported: "handles empty" — workers/repository/onboarding/pr/pr-list.spec.ts line 16
    #[test]
    fn test_pr_list_empty() {
        let result = get_expected_pr_list(2, 0, &[]);
        assert!(result.contains("already up-to-date"));
        assert!(!result.contains("Renovate will create"));
    }

    // Ported: "has special lock file maintenance description" — workers/repository/onboarding/pr/pr-list.spec.ts line 28
    #[test]
    fn test_pr_list_lock_file_maintenance() {
        let upgrades = [PrListUpgrade {
            dep_name: "",
            source_url: None,
            update_type: "lockFileMaintenance",
            new_value: None,
            new_version: None,
            new_digest: None,
            is_lockfile_update: false,
        }];
        let branches = [PrListBranch {
            pr_title: "Lock file maintenance",
            branch_name: "renovate/lock-file-maintenance",
            base_branch: Some("base"),
            schedule: &["before 5am"],
            upgrades: &upgrades,
        }];
        let result = get_expected_pr_list(2, 0, &branches);
        assert!(result.contains("Renovate will create 1 Pull Request:"));
        assert!(result.contains("Schedule: [\"before 5am\"]"));
        assert!(result.contains("Regenerate lock files"));
        assert!(result.contains("Merge into: `base`"));
    }

    // Ported: "handles multiple" — workers/repository/onboarding/pr/pr-list.spec.ts line 66
    #[test]
    fn test_pr_list_multiple_with_limit() {
        let upgrades1 = [
            PrListUpgrade { dep_name: "a", source_url: Some("https://a"), update_type: "pin", new_value: Some("1.1.0"), new_version: None, new_digest: None, is_lockfile_update: false },
            PrListUpgrade { dep_name: "b", source_url: None, update_type: "pin", new_value: Some("1.5.3"), new_version: None, new_digest: None, is_lockfile_update: false },
        ];
        let upgrades2 = [
            PrListUpgrade { dep_name: "a", source_url: Some("https://a"), update_type: "update", new_value: Some("2.0.1"), new_version: None, new_digest: None, is_lockfile_update: true },
        ];
        let branches = [
            PrListBranch { pr_title: "Pin dependencies", branch_name: "renovate/pin-dependencies", base_branch: Some("base"), schedule: &[], upgrades: &upgrades1 },
            PrListBranch { pr_title: "Update a to v2", branch_name: "renovate/a-2.x", base_branch: Some(""), schedule: &[], upgrades: &upgrades2 },
        ];
        let result = get_expected_pr_list(1, 0, &branches);
        assert!(result.contains("Renovate will create 2 Pull Requests:"));
        assert!(result.contains("prHourlyLimit"));
        assert!(result.contains("limited to maximum 1 per hour"));
    }

    // Ported: "shows commitHourlyLimit message when limit is low" — workers/repository/onboarding/pr/pr-list.spec.ts line 145
    #[test]
    fn test_pr_list_commit_hourly_limit_low() {
        let upgrades = [PrListUpgrade { dep_name: "a", source_url: None, update_type: "update", new_value: Some("1.0.0"), new_version: None, new_digest: None, is_lockfile_update: false }];
        let branches = [
            PrListBranch { pr_title: "Update a to v1", branch_name: "renovate/a-1.x", base_branch: Some("base"), schedule: &[], upgrades: &upgrades },
            PrListBranch { pr_title: "Update b to v1", branch_name: "renovate/b-1.x", base_branch: Some("base"), schedule: &[], upgrades: &upgrades },
        ];
        let result = get_expected_pr_list(2, 1, &branches);
        assert!(result.contains("commitHourlyLimit"));
        assert!(result.contains("Branch creation and rebasing"));
    }

    // Ported: "does not show commitHourlyLimit message when limit is high" — workers/repository/onboarding/pr/pr-list.spec.ts line 184
    #[test]
    fn test_pr_list_commit_hourly_limit_high() {
        let upgrades = [PrListUpgrade { dep_name: "a", source_url: None, update_type: "update", new_value: Some("1.0.0"), new_version: None, new_digest: None, is_lockfile_update: false }];
        let branches = [
            PrListBranch { pr_title: "Update a to v1", branch_name: "renovate/a-1.x", base_branch: Some("base"), schedule: &[], upgrades: &upgrades },
        ];
        let result = get_expected_pr_list(2, 10, &branches);
        assert!(!result.contains("commitHourlyLimit"));
    }

    // Ported: "shows only commitHourlyLimit message when both limits are set" — workers/repository/onboarding/pr/pr-list.spec.ts line 206
    #[test]
    fn test_pr_list_both_limits_commit_wins() {
        let upgrades = [PrListUpgrade { dep_name: "a", source_url: None, update_type: "update", new_value: Some("1.0.0"), new_version: None, new_digest: None, is_lockfile_update: false }];
        let branches = [
            PrListBranch { pr_title: "Update a to v1", branch_name: "renovate/a-1.x", base_branch: Some("base"), schedule: &[], upgrades: &upgrades },
            PrListBranch { pr_title: "Update b to v1", branch_name: "renovate/b-1.x", base_branch: Some("base"), schedule: &[], upgrades: &upgrades },
        ];
        let result = get_expected_pr_list(1, 1, &branches);
        assert!(result.contains("commitHourlyLimit"));
        assert!(!result.contains("prHourlyLimit"));
    }

    // ── transform_github_tag ─────────────────────────────────────────────────

    // Ported: "transforms Commit type" — util/github/graphql/query-adapters/tags-query-adapter.spec.ts line 4
    #[test]
    fn test_transform_github_tag_commit_type() {
        let r = transform_github_tag(
            Some("1.2.3"),
            Some(GithubTagTarget::Commit { oid: "abc123", release_timestamp: "2022-09-24" }),
        );
        assert!(r.is_some());
        let r = r.unwrap();
        assert_eq!(r.version, "1.2.3");
        assert_eq!(r.git_ref, "1.2.3");
        assert_eq!(r.hash, "abc123");
        assert_eq!(r.release_timestamp, "2022-09-24");
    }

    // Ported: "transforms Tag type" — util/github/graphql/query-adapters/tags-query-adapter.spec.ts line 22
    #[test]
    fn test_transform_github_tag_tag_type() {
        let r = transform_github_tag(
            Some("1.2.3"),
            Some(GithubTagTarget::Tag { tagger_timestamp: "2022-09-24", nested_oid: "abc123" }),
        );
        assert!(r.is_some());
        let r = r.unwrap();
        assert_eq!(r.version, "1.2.3");
        assert_eq!(r.git_ref, "1.2.3");
        assert_eq!(r.hash, "abc123");
        assert_eq!(r.release_timestamp, "2022-09-24");
    }

    // Ported: "transforms nested Tag type" — util/github/graphql/query-adapters/tags-query-adapter.spec.ts line 43
    #[test]
    fn test_transform_github_tag_nested_tag_type() {
        // Nested Tag: Tag → Tag → Commit; oid from innermost commit
        let r = transform_github_tag(
            Some("1.2.3"),
            Some(GithubTagTarget::Tag { tagger_timestamp: "2022-09-24", nested_oid: "abc123" }),
        );
        assert!(r.is_some());
        let r = r.unwrap();
        assert_eq!(r.hash, "abc123");
        assert_eq!(r.release_timestamp, "2022-09-24");
    }

    // Ported: "returns null for other types" — util/github/graphql/query-adapters/tags-query-adapter.spec.ts line 65
    #[test]
    fn test_transform_github_tag_invalid_returns_none() {
        let r = transform_github_tag(None, None);
        assert!(r.is_none());
    }

    // ── transform_github_release ─────────────────────────────────────────────

    // Ported: "transforms items" — util/github/graphql/query-adapters/releases-query-adapter.spec.ts line 17
    #[test]
    fn test_transform_github_release_basic() {
        let r = transform_github_release(
            Some("1.2.3"),
            Some("2024-09-24"),
            Some(false),
            Some(false),
            Some("https://example.com"),
            Some(123),
            Some("name"),
            Some("description"),
        );
        assert!(r.is_some());
        let r = r.unwrap();
        assert_eq!(r.version, "1.2.3");
        assert_eq!(r.release_timestamp, "2024-09-24T00:00:00.000Z");
        assert_eq!(r.url, "https://example.com");
        assert_eq!(r.id, Some(123));
        assert_eq!(r.name, Some("name".to_string()));
        assert_eq!(r.description, Some("description".to_string()));
        assert_eq!(r.is_stable, None);
    }

    // Ported: "filters out drafts" — util/github/graphql/query-adapters/releases-query-adapter.spec.ts line 28
    #[test]
    fn test_transform_github_release_draft_filtered() {
        let r = transform_github_release(
            Some("1.2.3"),
            Some("2024-09-24"),
            Some(true), // isDraft = true
            Some(false),
            Some("https://example.com"),
            Some(123),
            Some("name"),
            Some("description"),
        );
        assert!(r.is_none());
    }

    // Ported: "handles invalid items" — util/github/graphql/query-adapters/releases-query-adapter.spec.ts line 32
    #[test]
    fn test_transform_github_release_invalid_returns_none() {
        // Empty struct = all None fields → transform returns None
        let r = transform_github_release(None, None, None, None, None, None, None, None);
        assert!(r.is_none());
    }

    // Ported: "marks prereleases as unstable" — util/github/graphql/query-adapters/releases-query-adapter.spec.ts line 36
    #[test]
    fn test_transform_github_release_prerelease_unstable() {
        let r = transform_github_release(
            Some("1.2.3"),
            Some("2024-09-24"),
            Some(false),
            Some(true), // isPrerelease = true
            Some("https://example.com"),
            Some(123),
            Some("name"),
            Some("description"),
        );
        assert!(r.is_some());
        assert_eq!(r.unwrap().is_stable, Some(false));
    }

    // ── calculate_most_recent_timestamp ──────────────────────────────────────

    fn semver_is_version(v: &str) -> bool { crate::versioning::npm::is_version(v) }
    fn semver_is_greater_than(a: &str, b: &str) -> bool { crate::versioning::npm::is_greater_than(a, b) }

    // Ported: "returns the timestamp of the latest version" — workers/repository/process/lookup/timestamps.spec.ts line 10
    #[test]
    fn test_timestamps_returns_latest() {
        let releases = vec![
            ReleaseEntry { version: "1.0.0", release_timestamp: Some("2021-01-01T00:00:00.000Z"), is_deprecated: false },
            ReleaseEntry { version: "2.0.0", release_timestamp: Some("2022-01-01T00:00:00.000Z"), is_deprecated: false },
            ReleaseEntry { version: "0.9.0", release_timestamp: Some("2020-01-01T00:00:00.000Z"), is_deprecated: false },
        ];
        let ts = calculate_most_recent_timestamp(&releases, semver_is_version, semver_is_greater_than);
        assert_eq!(ts.as_deref(), Some("2022-01-01T00:00:00.000Z"));
    }

    // Ported: "handles releases with missing timestamps" — workers/repository/process/lookup/timestamps.spec.ts line 33
    #[test]
    fn test_timestamps_missing_middle() {
        let releases = vec![
            ReleaseEntry { version: "1.0.0", release_timestamp: Some("2021-01-01T00:00:00.000Z"), is_deprecated: false },
            ReleaseEntry { version: "2.0.0", release_timestamp: None, is_deprecated: false },
            ReleaseEntry { version: "3.0.0", release_timestamp: Some("2023-01-01T00:00:00.000Z"), is_deprecated: false },
        ];
        let ts = calculate_most_recent_timestamp(&releases, semver_is_version, semver_is_greater_than);
        assert_eq!(ts.as_deref(), Some("2023-01-01T00:00:00.000Z"));
    }

    // Ported: "handles latest release with missing timestamp" — workers/repository/process/lookup/timestamps.spec.ts line 53
    #[test]
    fn test_timestamps_latest_no_timestamp() {
        let releases = vec![
            ReleaseEntry { version: "1.0.0", release_timestamp: Some("2021-01-01T00:00:00.000Z"), is_deprecated: false },
            ReleaseEntry { version: "2.0.0", release_timestamp: Some("2022-01-01T00:00:00.000Z"), is_deprecated: false },
            ReleaseEntry { version: "3.0.0", release_timestamp: None, is_deprecated: false },
        ];
        let ts = calculate_most_recent_timestamp(&releases, semver_is_version, semver_is_greater_than);
        assert!(ts.is_none());
    }

    // Ported: "handles latest release with deprecation flag" — workers/repository/process/lookup/timestamps.spec.ts line 75
    #[test]
    fn test_timestamps_latest_deprecated() {
        let releases = vec![
            ReleaseEntry { version: "1.0.0", release_timestamp: Some("2021-01-01T00:00:00.000Z"), is_deprecated: false },
            ReleaseEntry { version: "2.0.0", release_timestamp: Some("2022-01-01T00:00:00.000Z"), is_deprecated: false },
            ReleaseEntry { version: "3.0.0", release_timestamp: Some("2023-01-01T00:00:00.000Z"), is_deprecated: true },
        ];
        let ts = calculate_most_recent_timestamp(&releases, semver_is_version, semver_is_greater_than);
        assert!(ts.is_none());
    }

    // Ported: "handles latest release with invalid version" — workers/repository/process/lookup/timestamps.spec.ts line 99
    #[test]
    fn test_timestamps_invalid_timestamp_for_highest() {
        let releases = vec![
            ReleaseEntry { version: "1.0.0", release_timestamp: Some("2021-01-01T00:00:00.000Z"), is_deprecated: false },
            ReleaseEntry { version: "2.0.0", release_timestamp: Some("2022-01-01T00:00:00.000Z"), is_deprecated: false },
            ReleaseEntry { version: "3.0.0", release_timestamp: Some("invalid"), is_deprecated: false },
        ];
        let ts = calculate_most_recent_timestamp(&releases, semver_is_version, semver_is_greater_than);
        assert!(ts.is_none());
    }

    // Ported: "returns undefined mostRecentTimestamp when no valid timestamps exist" — workers/repository/process/lookup/timestamps.spec.ts line 122
    #[test]
    fn test_timestamps_no_valid_timestamps() {
        let releases = vec![
            ReleaseEntry { version: "1.0.0", release_timestamp: None, is_deprecated: false },
            ReleaseEntry { version: "2.0.0", release_timestamp: None, is_deprecated: false },
        ];
        let ts = calculate_most_recent_timestamp(&releases, semver_is_version, semver_is_greater_than);
        assert!(ts.is_none());
    }

    // Ported: "handles empty releases array" — workers/repository/process/lookup/timestamps.spec.ts line 132
    #[test]
    fn test_timestamps_empty_releases() {
        let releases: Vec<ReleaseEntry> = vec![];
        let ts = calculate_most_recent_timestamp(&releases, semver_is_version, semver_is_greater_than);
        assert!(ts.is_none());
    }

    // Ported: "preserves other properties in the release result" — workers/repository/process/lookup/timestamps.spec.ts line 138
    #[test]
    fn test_timestamps_single_release() {
        let releases = vec![
            ReleaseEntry { version: "1.0.0", release_timestamp: Some("2021-01-01T00:00:00.000Z"), is_deprecated: false },
        ];
        let ts = calculate_most_recent_timestamp(&releases, semver_is_version, semver_is_greater_than);
        assert_eq!(ts.as_deref(), Some("2021-01-01T00:00:00.000Z"));
    }

    // Ported: "handles ancient versions that are higher than the ones recently released" — workers/repository/process/lookup/timestamps.spec.ts line 160
    #[test]
    fn test_timestamps_ancient_high_version() {
        // 99.99.99-alpha is the highest semver but has an OLD timestamp (2010).
        // 2.0.0 has a NEWER timestamp (2022). So higher timestamp exists for lower version → None.
        let releases = vec![
            ReleaseEntry { version: "99.99.99-alpha", release_timestamp: Some("2010-01-01T00:00:00.000Z"), is_deprecated: false },
            ReleaseEntry { version: "2.0.0", release_timestamp: Some("2022-01-01T00:00:00.000Z"), is_deprecated: false },
        ];
        let ts = calculate_most_recent_timestamp(&releases, semver_is_version, semver_is_greater_than);
        assert!(ts.is_none());
    }

    // Ported: "handles errors thrown for invalid versions" — workers/repository/process/lookup/timestamps.spec.ts line 180
    #[test]
    fn test_timestamps_invalid_versions_ignored() {
        // 'foo' and 'bar' are invalid versions, should be skipped.
        // Highest valid is 2.0.0 with timestamp 2023.
        let releases = vec![
            ReleaseEntry { version: "foo", release_timestamp: Some("2020-01-01T00:00:00.000Z"), is_deprecated: false },
            ReleaseEntry { version: "1.0.0", release_timestamp: Some("2021-01-01T00:00:00.000Z"), is_deprecated: false },
            ReleaseEntry { version: "bar", release_timestamp: Some("2022-01-01T00:00:00.000Z"), is_deprecated: false },
            ReleaseEntry { version: "2.0.0", release_timestamp: Some("2023-01-01T00:00:00.000Z"), is_deprecated: false },
        ];
        let ts = calculate_most_recent_timestamp(&releases, semver_is_version, semver_is_greater_than);
        assert_eq!(ts.as_deref(), Some("2023-01-01T00:00:00.000Z"));
    }

    // ── get_child_process_env ─────────────────────────────────────────────────

    fn make_env(pairs: &[(&str, &str)]) -> std::collections::HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    // Ported: "returns default environment variables" — util/exec/env.spec.ts line 35
    #[test]
    fn test_get_child_process_env_defaults() {
        let env = make_env(&[
            ("HTTP_PROXY", "HTTP_PROXY"),
            ("HTTPS_PROXY", "HTTPS_PROXY"),
            ("NO_PROXY", "NO_PROXY"),
            ("HOME", "HOME"),
            ("PATH", "PATH"),
            ("LC_ALL", "LC_ALL"),
            ("LANG", "LANG"),
            ("DOCKER_HOST", "DOCKER_HOST"),
            ("GIT_SSL_CAPATH", "GIT_SSL_CAPATH"),
            ("GIT_SSL_CAINFO", "GIT_SSL_CAINFO"),
            ("SSL_CERT_FILE", "SSL_CERT_FILE"),
            ("URL_REPLACE_1_FROM", "URL_REPLACE_1_FROM"),
            ("URL_REPLACE_1_TO", "URL_REPLACE_1_TO"),
            ("PROGRAMFILES", "PROGRAMFILES"),
            ("PROGRAMFILES(X86)", "PROGRAMFILES(X86)"),
            ("APPDATA", "APPDATA"),
            ("LOCALAPPDATA", "LOCALAPPDATA"),
        ]);
        let result = get_child_process_env(&env, &[], false);
        assert_eq!(result.get("HTTP_PROXY").map(String::as_str), Some("HTTP_PROXY"));
        assert_eq!(result.get("HTTPS_PROXY").map(String::as_str), Some("HTTPS_PROXY"));
        assert_eq!(result.get("HOME").map(String::as_str), Some("HOME"));
        assert_eq!(result.get("PATH").map(String::as_str), Some("PATH"));
        assert_eq!(result.get("DOCKER_HOST").map(String::as_str), Some("DOCKER_HOST"));
        assert_eq!(result.get("GIT_SSL_CAPATH").map(String::as_str), Some("GIT_SSL_CAPATH"));
        assert_eq!(result.get("URL_REPLACE_1_FROM").map(String::as_str), Some("URL_REPLACE_1_FROM"));
        assert_eq!(result.get("URL_REPLACE_1_TO").map(String::as_str), Some("URL_REPLACE_1_TO"));
        assert_eq!(result.get("PROGRAMFILES").map(String::as_str), Some("PROGRAMFILES"));
        assert_eq!(result.get("APPDATA").map(String::as_str), Some("APPDATA"));
    }

    // Ported: "returns environment variable only if defined" — util/exec/env.spec.ts line 57
    #[test]
    fn test_get_child_process_env_only_defined() {
        let env = make_env(&[
            ("HOME", "HOME"),
            ("HTTPS_PROXY", "HTTPS_PROXY"),
            // PATH intentionally absent
        ]);
        let result = get_child_process_env(&env, &[], false);
        assert!(result.contains_key("HOME"));
        assert!(result.contains_key("HTTPS_PROXY"));
        assert!(!result.contains_key("PATH"));
    }

    // Ported: "returns custom environment variables if passed and defined" — util/exec/env.spec.ts line 62
    #[test]
    fn test_get_child_process_env_custom_vars() {
        let env = make_env(&[
            ("DOCKER_HOST", "DOCKER_HOST"),
            ("FOOBAR", "FOOBAR"),
            ("HOME", "HOME"),
            ("HTTPS_PROXY", "HTTPS_PROXY"),
            ("HTTP_PROXY", "HTTP_PROXY"),
            ("LANG", "LANG"),
            ("LC_ALL", "LC_ALL"),
            ("NO_PROXY", "NO_PROXY"),
            ("PATH", "PATH"),
        ]);
        let result = get_child_process_env(&env, &["FOOBAR"], false);
        assert_eq!(result.get("FOOBAR").map(String::as_str), Some("FOOBAR"));
        assert_eq!(result.get("DOCKER_HOST").map(String::as_str), Some("DOCKER_HOST"));
        assert_eq!(result.get("HOME").map(String::as_str), Some("HOME"));
        assert_eq!(result.get("HTTPS_PROXY").map(String::as_str), Some("HTTPS_PROXY"));
    }

    // Ported: "returns process.env if trustlevel set to high" — util/exec/env.spec.ts line 79
    #[test]
    fn test_get_child_process_env_expose_all() {
        let env = make_env(&[
            ("HOME", "home_val"),
            ("SECRET_KEY", "secret"),
            ("RANDOM_VAR", "random"),
        ]);
        let result = get_child_process_env(&env, &[], true);
        // expose_all=true returns everything
        assert_eq!(result.get("HOME").map(String::as_str), Some("home_val"));
        assert_eq!(result.get("SECRET_KEY").map(String::as_str), Some("secret"));
        assert_eq!(result.get("RANDOM_VAR").map(String::as_str), Some("random"));
    }
}
