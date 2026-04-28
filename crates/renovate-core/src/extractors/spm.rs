//! Swift Package Manager `Package.swift` dependency extractor.
//!
//! Parses `.package(url:, from:)` declarations with a regex scanner and
//! returns dependencies ready for GitHub/GitLab tags version lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/swift/extract.ts` — `extractPackageFile`
//! - `lib/modules/manager/swift/index.ts`   — pattern `/(^|/)Package\\.swift/`
//!
//! ## Supported version forms
//!
//! | Form | Treatment |
//! |---|---|
//! | `.package(url: "…", from: "1.0.0")` | Actionable |
//! | `.package(url: "…", exact: "1.0.0")` | Actionable |
//! | `.package(url: "…", .upToNextMajor(from: "1.0.0"))` | Actionable |
//! | `.package(url: "…", .upToNextMinor(from: "1.0.0"))` | Actionable |
//! | `.package(url: "…", "1.0.0"..<"2.0.0")` | Actionable — lower bound only |
//! | `.package(url: "non-github/non-gitlab URL", …)` | Skipped — `NonGitHub` |
//! | `.package(path: "…")` | Skipped — `LocalPath` |

use std::sync::LazyLock;

use regex::Regex;

/// Why a Swift dep is skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpmSkipReason {
    /// Package references a local path (`path:` form).
    LocalPath,
    /// Package URL is not a recognized Git hosting service.
    NonGitHost,
}

/// Which Git hosting service the URL belongs to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitHost {
    GitHub,
    GitLab,
}

/// A single extracted Swift Package dep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpmExtractedDep {
    /// `owner/repo` slug (e.g. `apple/swift-log`).
    pub owner_repo: String,
    /// Version constraint lower bound (e.g. `1.4.4`).
    pub current_value: String,
    /// Which Git hosting service hosts this package.
    pub git_host: Option<GitHost>,
    /// Set when no version lookup should be performed.
    pub skip_reason: Option<SpmSkipReason>,
}

// ── Compiled regexes ──────────────────────────────────────────────────────────

/// Matches a `.package(url: "…", <version>)` declaration.
///
/// Captures:
/// 1. Package URL (full https URL)
/// 2. First quoted version string inside the declaration
static PACKAGE_URL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\.package\s*\(\s*url\s*:\s*"([^"]+)"[^)]*?"(\d[^"]*)"[^)]*\)"#).unwrap()
});

/// Matches `.package(path: "…")` — local path dependencies.
static PACKAGE_PATH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\.package\s*\(\s*path\s*:\s*"[^"]+"\s*\)"#).unwrap());

// ── Public API ────────────────────────────────────────────────────────────────

/// Parse a `Package.swift` file and extract all package dependencies.
pub fn extract(content: &str) -> Vec<SpmExtractedDep> {
    let mut deps = Vec::new();

    // Count local path deps (to include in the output as skipped).
    for _ in PACKAGE_PATH.find_iter(content) {
        // We cannot easily extract the package name from `path:` form,
        // so we emit a placeholder dep with a skip reason.
        deps.push(SpmExtractedDep {
            owner_repo: String::new(),
            current_value: String::new(),
            git_host: None,
            skip_reason: Some(SpmSkipReason::LocalPath),
        });
    }

    // Extract URL-based packages.
    for cap in PACKAGE_URL.captures_iter(content) {
        let url = cap[1].trim();
        let version = cap[2].trim().to_owned();

        if let Some((owner_repo, host)) = parse_git_url(url) {
            deps.push(SpmExtractedDep {
                owner_repo,
                current_value: version,
                git_host: Some(host),
                skip_reason: None,
            });
        } else {
            deps.push(SpmExtractedDep {
                owner_repo: url.to_owned(),
                current_value: version,
                git_host: None,
                skip_reason: Some(SpmSkipReason::NonGitHost),
            });
        }
    }

    deps
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Extract `owner/repo` and hosting service from a git URL.
///
/// Handles:
/// - `https://github.com/apple/swift-log.git` → `("apple/swift-log", GitHub)`
/// - `https://github.com/vapor/vapor`          → `("vapor/vapor", GitHub)`
/// - `https://gitlab.com/user/repo.git`        → `("user/repo", GitLab)`
fn parse_git_url(url: &str) -> Option<(String, GitHost)> {
    let (host, remainder) = if let Some(r) = url.strip_prefix("https://github.com/") {
        (GitHost::GitHub, r)
    } else if let Some(r) = url.strip_prefix("https://gitlab.com/") {
        (GitHost::GitLab, r)
    } else {
        return None;
    };

    // Take up to two path components: owner/repo.
    let parts: Vec<&str> = remainder.splitn(3, '/').collect();
    if parts.len() < 2 {
        return None;
    }
    let owner = parts[0];
    let repo = parts[1].trim_end_matches(".git");
    if owner.is_empty() || repo.is_empty() {
        return None;
    }
    Some((format!("{owner}/{repo}"), host))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_version() {
        let content = r#"
let package = Package(
    dependencies: [
        .package(url: "https://github.com/apple/swift-log.git", from: "1.4.4"),
    ]
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].owner_repo, "apple/swift-log");
        assert_eq!(deps[0].current_value, "1.4.4");
        assert_eq!(deps[0].git_host, Some(GitHost::GitHub));
        assert!(deps[0].skip_reason.is_none());
    }

    #[test]
    fn exact_version() {
        let content = r#"
.package(url: "https://github.com/vapor/vapor.git", exact: "4.65.1"),
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "4.65.1");
    }

    #[test]
    fn up_to_next_major() {
        let content = r#"
.package(url: "https://github.com/vapor/leaf.git", .upToNextMajor(from: "4.2.4")),
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "4.2.4");
    }

    #[test]
    fn up_to_next_minor() {
        let content = r#"
.package(url: "https://github.com/vapor/fluent.git", .upToNextMinor(from: "4.4.0")),
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "4.4.0");
    }

    #[test]
    fn range_constraint() {
        let content = r#"
.package(url: "https://github.com/apple/swift-crypto.git", "1.0.0"..<"3.0.0"),
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        // Lower bound of range
        assert_eq!(deps[0].current_value, "1.0.0");
    }

    #[test]
    fn local_path_skipped() {
        let content = r#"
.package(path: "../MyLocalPackage"),
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(SpmSkipReason::LocalPath));
    }

    #[test]
    fn non_github_url_skipped() {
        let content = r#"
.package(url: "https://bitbucket.org/myorg/myrepo.git", from: "1.0.0"),
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(SpmSkipReason::NonGitHost));
    }

    #[test]
    fn gitlab_url() {
        let content = r#"
.package(url: "https://gitlab.com/myorg/mypackage.git", from: "2.0.0"),
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].git_host, Some(GitHost::GitLab));
        assert_eq!(deps[0].owner_repo, "myorg/mypackage");
        assert!(deps[0].skip_reason.is_none());
    }

    #[test]
    fn real_world_package_swift() {
        let content = r#"
import PackageDescription

let package = Package(
    name: "MyApp",
    platforms: [.macOS(.v12), .iOS(.v15)],
    dependencies: [
        .package(url: "https://github.com/apple/swift-log.git", from: "1.4.4"),
        .package(url: "https://github.com/vapor/vapor.git", from: "4.65.1"),
        .package(url: "https://github.com/vapor/leaf.git", .upToNextMajor(from: "4.2.4")),
        .package(url: "https://github.com/apple/swift-nio.git", exact: "2.40.0"),
        .package(url: "https://github.com/apple/swift-crypto.git", "1.0.0"..<"3.0.0"),
        .package(path: "../MyLocalPackage"),
    ]
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 6);

        let log = deps.iter().find(|d| d.owner_repo == "apple/swift-log");
        assert!(log.is_some());
        assert_eq!(log.unwrap().current_value, "1.4.4");

        let local = deps
            .iter()
            .find(|d| d.skip_reason == Some(SpmSkipReason::LocalPath));
        assert!(local.is_some());
    }

    #[test]
    fn parse_git_url_strips_git_suffix() {
        let (slug, host) = parse_git_url("https://github.com/apple/swift-log.git").unwrap();
        assert_eq!(slug, "apple/swift-log");
        assert_eq!(host, GitHost::GitHub);
    }

    #[test]
    fn parse_git_url_no_suffix() {
        let (slug, _) = parse_git_url("https://github.com/vapor/vapor").unwrap();
        assert_eq!(slug, "vapor/vapor");
    }

    #[test]
    fn no_packages_returns_empty() {
        let content = "// just a comment\nlet x = 1\n";
        assert!(extract(content).is_empty());
    }
}
