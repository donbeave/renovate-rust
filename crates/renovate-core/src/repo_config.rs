//! Repository-level Renovate config discovery and parsing.
//!
//! Renovate reference:
//! - `lib/config/app-strings.ts` `getConfigFileNames()`
//! - `lib/config/options/index.ts` — `enabled`, `ignoreDeps`, `ignorePaths`
//!
//! Renovate searches a fixed ordered list of paths inside the repository;
//! the first one found wins. This module ports that list, wires it to the
//! platform client's file-reading capability, and parses the config content
//! into a typed `RepoConfig` struct.

use globset::{Glob, GlobSet, GlobSetBuilder};
use serde::Deserialize;

use crate::config::GlobalConfig;
use crate::platform::{AnyPlatformClient, PlatformError};

#[cfg(test)]
use base64::Engine as _;

/// Ordered list of candidate config file paths, matching Renovate's
/// `configFileNames` constant in `lib/config/app-strings.ts`.
///
/// The `package.json` entry is omitted for now — parsing a `renovate` key
/// inside `package.json` is a separate slice.
pub const CONFIG_FILE_CANDIDATES: &[&str] = &[
    "renovate.json",
    "renovate.json5",
    ".github/renovate.json",
    ".github/renovate.json5",
    ".gitlab/renovate.json",
    ".gitlab/renovate.json5",
    ".renovaterc",
    ".renovaterc.json",
    ".renovaterc.json5",
];

/// Parsed per-repository Renovate configuration.
///
/// Only the fields that affect the local update scan are included here.
/// Complex fields like `packageRules` and `extends` are deferred to later
/// slices.
///
/// Defaults match Renovate's option defaults.
#[derive(Debug, Clone)]
pub struct RepoConfig {
    /// If `false`, Renovate is disabled for this repository entirely.
    /// Defaults to `true`.
    pub enabled: bool,
    /// Dependency names to skip during update lookups.  Exact string match.
    pub ignore_deps: Vec<String>,
    /// File path patterns to exclude from scanning.  Patterns follow
    /// minimatch/globset syntax (`**/test/**`, `**/*.spec.ts`, etc.).  Plain
    /// paths (no glob characters) are treated as prefix matches.
    pub ignore_paths: Vec<String>,
}

/// Compiled path-ignore matcher built from a `RepoConfig`.
///
/// Separates plain-prefix patterns from glob patterns at construction time so
/// matching a single path is O(patterns) rather than building a GlobSet per
/// call.
///
/// Renovate reference: `lib/config/options/index.ts` — `ignorePaths`.
#[derive(Debug)]
pub struct PathMatcher {
    prefixes: Vec<String>,
    globs: GlobSet,
}

impl PathMatcher {
    /// Compile `patterns` into a `PathMatcher`.
    ///
    /// Patterns containing `*`, `?`, or `[` are compiled as globset globs;
    /// all others are treated as path prefixes (trailing `/` is stripped).
    pub fn new(patterns: &[String]) -> Self {
        let mut prefixes = Vec::new();
        let mut glob_builder = GlobSetBuilder::new();

        for raw in patterns {
            let pattern = raw.trim_end_matches('/');
            if pattern.contains('*') || pattern.contains('?') || pattern.contains('[') {
                if let Ok(g) = Glob::new(pattern) {
                    glob_builder.add(g);
                }
            } else {
                prefixes.push(pattern.to_owned());
            }
        }

        let globs = glob_builder.build().unwrap_or_else(|_| {
            GlobSetBuilder::new()
                .build()
                .expect("empty globset always builds")
        });

        PathMatcher { prefixes, globs }
    }

    /// Returns `true` when `path` matches any ignore pattern.
    pub fn is_ignored(&self, path: &str) -> bool {
        if self.globs.is_match(path) {
            return true;
        }
        self.prefixes
            .iter()
            .any(|p| path == p || path.starts_with(&format!("{p}/")))
    }
}

impl RepoConfig {
    /// Parse the raw content of a `renovate.json` / `.renovaterc` file.
    ///
    /// Supports JSON and JSON5.  Unknown fields are silently ignored.
    /// Returns a default `RepoConfig` (all defaults) when the content is
    /// empty or unparseable.
    pub fn parse(content: &str) -> Self {
        #[derive(Deserialize)]
        struct Raw {
            #[serde(default = "default_true")]
            enabled: bool,
            #[serde(rename = "ignoreDeps", default)]
            ignore_deps: Vec<String>,
            #[serde(rename = "ignorePaths", default)]
            ignore_paths: Vec<String>,
        }

        fn default_true() -> bool {
            true
        }

        let raw: Raw = match json5::from_str(content) {
            Ok(r) => r,
            Err(e) => {
                tracing::debug!(%e, "failed to parse repo renovate config; using defaults");
                return Self::default();
            }
        };

        Self {
            enabled: raw.enabled,
            ignore_deps: raw.ignore_deps,
            ignore_paths: raw.ignore_paths,
        }
    }

    /// Return `true` when a dependency name should be ignored.
    pub fn is_dep_ignored(&self, name: &str) -> bool {
        self.ignore_deps.iter().any(|p| p == name)
    }
}

impl Default for RepoConfig {
    fn default() -> Self {
        Self {
            enabled: true, // Renovate default is enabled
            ignore_deps: Vec::new(),
            ignore_paths: Vec::new(),
        }
    }
}

impl RepoConfig {
    /// Build a compiled `PathMatcher` from this config's `ignore_paths`.
    ///
    /// Call this once and reuse the result when checking many paths (e.g. the
    /// repo file list), rather than calling `is_path_ignored` in a loop.
    pub fn build_path_matcher(&self) -> PathMatcher {
        PathMatcher::new(&self.ignore_paths)
    }

    /// Return `true` when a file path should be excluded from scanning.
    ///
    /// Supports globset patterns (`**/test/**`, `**/*.spec.ts`) and plain
    /// path prefixes (`vendor`, `packages/legacy`).  For large file lists,
    /// prefer [`build_path_matcher`] to amortize glob compilation.
    pub fn is_path_ignored(&self, path: &str) -> bool {
        self.build_path_matcher().is_ignored(path)
    }
}

/// Outcome of a repository config discovery attempt.
#[derive(Debug, Clone)]
pub enum RepoConfigResult {
    /// A config file was found; parsed config is ready to use.
    Found { path: String, config: RepoConfig },
    /// No config file exists in the repository.
    NotFound,
    /// The repository has not been onboarded (no config) and
    /// `require_config = Required`.
    NeedsOnboarding,
}

/// Try to find a Renovate config file in the repository.
///
/// Tries each path in [`CONFIG_FILE_CANDIDATES`] in order and returns the
/// first one found. Returns [`RepoConfigResult::NotFound`] or
/// [`RepoConfigResult::NeedsOnboarding`] when none exist.
pub async fn discover(
    client: &AnyPlatformClient,
    owner: &str,
    repo: &str,
    global_config: &GlobalConfig,
) -> Result<RepoConfigResult, PlatformError> {
    for path in CONFIG_FILE_CANDIDATES {
        if let Some(file) = client.get_raw_file(owner, repo, path).await? {
            tracing::debug!(repo = %format!("{owner}/{repo}"), path = %path, "found renovate config");
            let config = RepoConfig::parse(&file.content);
            return Ok(RepoConfigResult::Found {
                path: file.path,
                config,
            });
        }
    }

    tracing::debug!(repo = %format!("{owner}/{repo}"), "no renovate config found");

    use crate::config::RequireConfig;
    if global_config.require_config == RequireConfig::Required {
        Ok(RepoConfigResult::NeedsOnboarding)
    } else {
        Ok(RepoConfigResult::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path as wm_path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::config::GlobalConfig;
    use crate::platform::AnyPlatformClient;
    use crate::platform::github::GithubClient;

    use super::*;

    fn make_client(server_uri: &str) -> AnyPlatformClient {
        AnyPlatformClient::Github(GithubClient::with_endpoint("token", server_uri).unwrap())
    }

    #[tokio::test]
    async fn finds_renovate_json_first() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(wm_path("/repos/owner/repo/contents/renovate.json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "content": base64::engine::general_purpose::STANDARD
                    .encode(r#"{"extends":["config:recommended"]}"#),
                "encoding": "base64"
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let result = discover(&client, "owner", "repo", &GlobalConfig::default())
            .await
            .unwrap();

        assert!(
            matches!(result, RepoConfigResult::Found { ref path, .. } if path == "renovate.json")
        );
    }

    #[tokio::test]
    async fn returns_needs_onboarding_when_no_config_and_required() {
        let server = MockServer::start().await;
        // All file requests return 404
        for candidate in CONFIG_FILE_CANDIDATES {
            Mock::given(method("GET"))
                .and(wm_path(format!("/repos/owner/repo/contents/{candidate}")))
                .respond_with(ResponseTemplate::new(404))
                .mount(&server)
                .await;
        }

        let client = make_client(&server.uri());
        // require_config defaults to Required
        let result = discover(&client, "owner", "repo", &GlobalConfig::default())
            .await
            .unwrap();

        assert!(matches!(result, RepoConfigResult::NeedsOnboarding));
    }

    #[tokio::test]
    async fn returns_not_found_when_optional() {
        use crate::config::RequireConfig;
        let server = MockServer::start().await;
        for candidate in CONFIG_FILE_CANDIDATES {
            Mock::given(method("GET"))
                .and(wm_path(format!("/repos/owner/repo/contents/{candidate}")))
                .respond_with(ResponseTemplate::new(404))
                .mount(&server)
                .await;
        }

        let client = make_client(&server.uri());
        let config = GlobalConfig {
            require_config: RequireConfig::Optional,
            ..GlobalConfig::default()
        };
        let result = discover(&client, "owner", "repo", &config).await.unwrap();
        assert!(matches!(result, RepoConfigResult::NotFound));
    }

    // ── RepoConfig::parse ────────────────────────────────────────────────────

    #[test]
    fn defaults_when_empty() {
        let c = RepoConfig::parse("{}");
        assert!(c.enabled);
        assert!(c.ignore_deps.is_empty());
        assert!(c.ignore_paths.is_empty());
    }

    #[test]
    fn enabled_false() {
        let c = RepoConfig::parse(r#"{"enabled": false}"#);
        assert!(!c.enabled);
    }

    #[test]
    fn ignore_deps_parsed() {
        let c = RepoConfig::parse(r#"{"ignoreDeps": ["lodash", "react"]}"#);
        assert_eq!(c.ignore_deps, vec!["lodash", "react"]);
    }

    #[test]
    fn ignore_paths_parsed() {
        let c = RepoConfig::parse(r#"{"ignorePaths": ["test/**", "vendor"]}"#);
        assert_eq!(c.ignore_paths, vec!["test/**", "vendor"]);
    }

    #[test]
    fn json5_comments_are_accepted() {
        let c = RepoConfig::parse(
            r#"{
                // This is a JSON5 comment
                "ignoreDeps": ["jest"], // trailing comma ok in JSON5
            }"#,
        );
        assert_eq!(c.ignore_deps, vec!["jest"]);
    }

    #[test]
    fn malformed_json_returns_defaults() {
        let c = RepoConfig::parse("not valid json at all");
        assert!(c.enabled);
        assert!(c.ignore_deps.is_empty());
    }

    #[test]
    fn is_dep_ignored_matches_exactly() {
        let c = RepoConfig::parse(r#"{"ignoreDeps": ["lodash"]}"#);
        assert!(c.is_dep_ignored("lodash"));
        assert!(!c.is_dep_ignored("lodash-fp"));
        assert!(!c.is_dep_ignored("react"));
    }

    #[test]
    fn is_path_ignored_prefix_match() {
        let c = RepoConfig::parse(r#"{"ignorePaths": ["vendor"]}"#);
        assert!(c.is_path_ignored("vendor/react/index.js"));
        assert!(!c.is_path_ignored("src/vendor.ts"));
    }

    // ── PathMatcher glob tests ────────────────────────────────────────────────

    #[test]
    fn glob_double_star_node_modules() {
        let m = PathMatcher::new(&["**/node_modules/**".to_owned()]);
        assert!(m.is_ignored("node_modules/lodash/index.js"));
        assert!(m.is_ignored("packages/foo/node_modules/bar/index.js"));
        assert!(!m.is_ignored("src/foo.ts"));
    }

    #[test]
    fn glob_spec_files() {
        let m = PathMatcher::new(&["**/*.spec.ts".to_owned()]);
        assert!(m.is_ignored("src/foo.spec.ts"));
        assert!(m.is_ignored("tests/bar.spec.ts"));
        assert!(!m.is_ignored("src/foo.ts"));
        assert!(!m.is_ignored("src/foo.spec.js"));
    }

    #[test]
    fn glob_tests_directory() {
        let m = PathMatcher::new(&["**/test/**".to_owned()]);
        assert!(m.is_ignored("src/test/helpers.ts"));
        assert!(m.is_ignored("test/unit/foo.ts"));
        assert!(!m.is_ignored("src/testing.ts"));
    }

    #[test]
    fn glob_rooted_path_under_dir() {
        let m = PathMatcher::new(&["test/**".to_owned()]);
        assert!(m.is_ignored("test/foo.ts"));
        assert!(!m.is_ignored("src/test/foo.ts")); // rooted glob, not deep
    }

    #[test]
    fn prefix_with_trailing_slash_stripped() {
        let m = PathMatcher::new(&["vendor/".to_owned()]);
        assert!(m.is_ignored("vendor/react/index.js"));
        assert!(!m.is_ignored("src/vendor.ts"));
    }

    #[test]
    fn mixed_glob_and_prefix_patterns() {
        let m = PathMatcher::new(&["**/node_modules/**".to_owned(), "vendor".to_owned()]);
        assert!(m.is_ignored("node_modules/foo/bar.js"));
        assert!(m.is_ignored("vendor/react.js"));
        assert!(!m.is_ignored("src/foo.ts"));
    }

    #[test]
    fn empty_patterns_ignore_nothing() {
        let m = PathMatcher::new(&[]);
        assert!(!m.is_ignored("anything/at/all.ts"));
    }

    #[test]
    fn repo_config_build_path_matcher_uses_globs() {
        let c = RepoConfig::parse(r#"{"ignorePaths": ["**/test/**", "vendor"]}"#);
        let m = c.build_path_matcher();
        assert!(m.is_ignored("src/test/unit.ts"));
        assert!(m.is_ignored("vendor/lib.js"));
        assert!(!m.is_ignored("src/main.ts"));
    }
}
