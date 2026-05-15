//! Homebrew formula Ruby dependency extractor.
//!
//! Extracts the `url` and `sha256` fields from Homebrew formula files and
//! routes them to the appropriate datasource (GitHub Tags, GitHub Releases,
//! or NPM).
//!
//! Renovate reference:
//! - `lib/modules/manager/homebrew/extract.ts`
//! - Pattern: `/^Formula/\w*/?[^/]+[.]rb$/`
//! - Datasources: GitHub Tags, GitHub Releases, NPM
//!
//! ## Supported URL forms
//!
//! ```ruby
//! # GitHub archive (Tags datasource)
//! url "https://github.com/owner/repo/archive/refs/tags/v1.2.3.tar.gz"
//!
//! # GitHub archive (old form)
//! url "https://github.com/owner/repo/archive/v1.2.3.tar.gz"
//!
//! # GitHub releases download
//! url "https://github.com/owner/repo/releases/download/v1.2.3/file.tar.gz"
//!
//! # NPM registry
//! url "https://registry.npmjs.org/lodash/-/lodash-4.17.21.tgz"
//! ```

use std::sync::LazyLock;

use regex::Regex;
use semver::Version;

/// Source type for the Homebrew formula.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HomebrewSource {
    /// GitHub archive/release (owner/repo).
    GitHub {
        repo: String,
        url_type: GitHubUrlType,
    },
    /// NPM registry package.
    Npm { package: String },
    /// Unsupported URL type.
    Unsupported(String),
}

/// Whether the GitHub URL is an archive or a release download.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitHubUrlType {
    Archive,
    Release,
}

/// A single Homebrew formula dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HomebrewDep {
    /// Formula class name.
    pub formula_name: String,
    /// Version extracted from the URL.
    pub current_value: String,
    /// Source datasource routing.
    pub source: HomebrewSource,
    /// SHA256 hash (for digest pinning, informational).
    pub sha256: Option<String>,
    /// Set when no lookup should be performed.
    pub skip_reason: Option<HomebrewSkipReason>,
}

/// Why a Homebrew dep is skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HomebrewSkipReason {
    /// SHA256 is missing or invalid.
    InvalidSha256,
    /// URL could not be parsed for a known source.
    UnsupportedUrl,
    /// No URL field found.
    MissingUrl,
}

/// Matches `class Name < Formula`
static CLASS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bclass\s+(\w+)\s*<\s*Formula\b").unwrap());

/// Matches `url "..."` or `url '...'`
static URL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\burl\s+(?:"([^"]+)"|'([^']+)')"#).unwrap());

/// Matches `sha256 "..."` or `sha256 '...'`
static SHA256_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\bsha256\s+(?:"([^"]+)"|'([^']+)')"#).unwrap());

/// GitHub archive URL: `/archive/refs/tags/v1.2.3.tar.gz` or `/archive/v1.2.3.tar.gz`
static GH_ARCHIVE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"github\.com/([^/]+/[^/]+)/archive(?:/refs/tags)?/([^/]+?)(?:\.tar\.gz|\.zip)?$")
        .unwrap()
});

/// GitHub release download: `/releases/download/v1.2.3/file.tar.gz`
static GH_RELEASE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"github\.com/([^/]+/[^/]+)/releases/download/([^/]+)/").unwrap());

/// NPM registry: `https://registry.npmjs.org/{name}/-/{name}-{version}.tgz`
/// Version always starts with a digit; use lazy repetition to skip hyphenated slug segments.
static NPM_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"registry\.npmjs\.org/(@[^/]+/[^/]+|[^/@]+)/-/(?:[^/]+-)+?(\d[^/]*)\.tgz$").unwrap()
});

/// Extract a Homebrew formula dependency from a `.rb` formula file.
///
/// Returns `None` when no valid `class Name < Formula` header is found.
pub fn extract(content: &str) -> Option<HomebrewDep> {
    // Get formula class name — must find a valid `class X < Formula` header.
    let formula_name = CLASS_RE.captures(content).map(|cap| cap[1].to_owned())?;

    // Get URL
    let url = URL_RE
        .captures(content)
        .and_then(|cap| cap.get(1).or_else(|| cap.get(2)))
        .map(|m| m.as_str().to_owned());

    let Some(url) = url else {
        return Some(HomebrewDep {
            formula_name,
            current_value: String::new(),
            source: HomebrewSource::Unsupported(String::new()),
            sha256: None,
            skip_reason: Some(HomebrewSkipReason::MissingUrl),
        });
    };

    // Get sha256
    let sha256 = SHA256_RE
        .captures(content)
        .and_then(|cap| cap.get(1).or_else(|| cap.get(2)))
        .map(|m| m.as_str().to_owned());

    // Validate sha256 (64 hex chars for SHA-256)
    if sha256.as_deref().map(|s| s.len()) != Some(64) {
        return Some(HomebrewDep {
            formula_name,
            current_value: String::new(),
            source: HomebrewSource::Unsupported(url),
            sha256,
            skip_reason: Some(HomebrewSkipReason::InvalidSha256),
        });
    }

    // Try GitHub archive pattern
    if let Some(cap) = GH_ARCHIVE_RE.captures(&url) {
        let repo = cap[1].to_owned();
        let raw_version = cap[2].trim_start_matches('v').to_owned();
        return Some(HomebrewDep {
            formula_name,
            current_value: raw_version,
            source: HomebrewSource::GitHub {
                repo,
                url_type: GitHubUrlType::Archive,
            },
            sha256,
            skip_reason: None,
        });
    }

    // Try GitHub release pattern
    if let Some(cap) = GH_RELEASE_RE.captures(&url) {
        let repo = cap[1].to_owned();
        let raw_version = cap[2].trim_start_matches('v').to_owned();
        return Some(HomebrewDep {
            formula_name,
            current_value: raw_version,
            source: HomebrewSource::GitHub {
                repo,
                url_type: GitHubUrlType::Release,
            },
            sha256,
            skip_reason: None,
        });
    }

    // Try NPM registry pattern
    if let Some(cap) = NPM_RE.captures(&url) {
        let package = cap[1].to_owned();
        let version = cap[2].to_owned();
        return Some(HomebrewDep {
            formula_name,
            current_value: version,
            source: HomebrewSource::Npm { package },
            sha256,
            skip_reason: None,
        });
    }

    Some(HomebrewDep {
        formula_name,
        current_value: String::new(),
        source: HomebrewSource::Unsupported(url),
        sha256,
        skip_reason: Some(HomebrewSkipReason::UnsupportedUrl),
    })
}

// ---------------------------------------------------------------------------
// Internal URL parsing helper
// ---------------------------------------------------------------------------

fn parse_url_host_and_path(url_str: &str) -> Option<(String, String)> {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^https?://([^/?#]+)(/[^?#]*)?").unwrap());
    let cap = RE.captures(url_str)?;
    let hostname = cap[1].to_owned();
    let pathname = cap
        .get(2)
        .map(|m| m.as_str().to_owned())
        .unwrap_or_else(|| "/".to_owned());
    Some((hostname, pathname))
}

// ---------------------------------------------------------------------------
// GitHub handler
// ---------------------------------------------------------------------------

/// Parsed result from a GitHub formula URL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitHubParsedResult {
    pub current_value: String,
    pub owner_name: String,
    pub repo_name: String,
    pub url_type: GitHubUrlType,
}

/// Manager data for a GitHub-sourced Homebrew dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitHubManagerData {
    pub owner_name: String,
    pub repo_name: String,
    pub sha256: Option<String>,
    pub url: Option<String>,
}

/// Dependency result from [`github_create_dependency`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitHubDepResult {
    pub dep_name: String,
    pub current_value: String,
    pub datasource: &'static str,
    pub manager_data: GitHubManagerData,
}

/// Parse a GitHub URL from a Homebrew formula.
pub fn github_parse_url(url_str: &str) -> Option<GitHubParsedResult> {
    if url_str.is_empty() {
        return None;
    }
    let (hostname, pathname) = parse_url_host_and_path(url_str)?;
    if hostname != "github.com" {
        return None;
    }
    let segs: Vec<&str> = pathname.split('/').filter(|s| !s.is_empty()).collect();
    if segs.len() < 3 {
        return None;
    }
    let owner_name = segs[0].to_owned();
    let repo_name = segs[1].to_owned();

    let (current_value, url_type) = if segs[2] == "archive" {
        let raw = if segs.get(3).copied() == Some("refs") {
            segs.get(5).copied()?
        } else {
            segs.get(3).copied()?
        };
        let cv = raw.strip_suffix(".tar.gz").unwrap_or(raw).to_owned();
        (cv, GitHubUrlType::Archive)
    } else if segs.get(2).copied() == Some("releases")
        && segs.get(3).copied() == Some("download")
    {
        (segs.get(4).copied()?.to_owned(), GitHubUrlType::Release)
    } else {
        return None;
    };

    Some(GitHubParsedResult {
        current_value,
        owner_name,
        repo_name,
        url_type,
    })
}

/// Create a dependency record from a parsed GitHub result.
pub fn github_create_dependency(
    parsed: &GitHubParsedResult,
    sha256: Option<String>,
    url: String,
) -> GitHubDepResult {
    let datasource = match parsed.url_type {
        GitHubUrlType::Release => "github-releases",
        GitHubUrlType::Archive => "github-tags",
    };
    GitHubDepResult {
        dep_name: format!("{}/{}", parsed.owner_name, parsed.repo_name),
        current_value: parsed.current_value.clone(),
        datasource,
        manager_data: GitHubManagerData {
            owner_name: parsed.owner_name.clone(),
            repo_name: parsed.repo_name.clone(),
            sha256,
            url: Some(url),
        },
    }
}

/// Build candidate archive URLs for a new GitHub version.
pub fn github_build_archive_urls(
    manager_data: &GitHubManagerData,
    new_version: &str,
) -> Vec<String> {
    let owner = &manager_data.owner_name;
    let repo = &manager_data.repo_name;
    let coerced = new_version.strip_prefix('v').unwrap_or(new_version);
    let ver_for_filename = Version::parse(coerced)
        .map(|v| v.to_string())
        .unwrap_or_else(|_| new_version.to_owned());
    vec![
        format!("https://github.com/{owner}/{repo}/releases/download/{new_version}/{repo}-{ver_for_filename}.tar.gz"),
        format!("https://github.com/{owner}/{repo}/archive/refs/tags/{new_version}.tar.gz"),
    ]
}

// ---------------------------------------------------------------------------
// NPM handler
// ---------------------------------------------------------------------------

/// Parsed result from an NPM registry formula URL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NpmParsedResult {
    pub current_value: String,
    pub package_name: String,
}

/// Manager data for an NPM-sourced Homebrew dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NpmManagerData {
    pub package_name: String,
    pub sha256: Option<String>,
    pub url: Option<String>,
}

/// Dependency result from [`npm_create_dependency`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NpmDepResult {
    pub dep_name: String,
    pub current_value: String,
    pub datasource: &'static str,
    pub manager_data: NpmManagerData,
}

static NPM_PATH_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^/(?P<pkg>(?:@[^/]+/)?[^/]+)/-/[^/]+-(?P<ver>[\d.]+(?:-[a-zA-Z0-9.-]*)?)\.tgz$",
    )
    .unwrap()
});

/// Parse an NPM registry URL from a Homebrew formula.
pub fn npm_parse_url(url_str: &str) -> Option<NpmParsedResult> {
    if url_str.is_empty() {
        return None;
    }
    let (hostname, pathname) = parse_url_host_and_path(url_str)?;
    if hostname != "registry.npmjs.org" {
        return None;
    }
    let cap = NPM_PATH_RE.captures(&pathname)?;
    Some(NpmParsedResult {
        package_name: cap["pkg"].to_owned(),
        current_value: cap["ver"].to_owned(),
    })
}

/// Create a dependency record from a parsed NPM result.
pub fn npm_create_dependency(
    parsed: &NpmParsedResult,
    sha256: Option<String>,
    url: String,
) -> NpmDepResult {
    NpmDepResult {
        dep_name: parsed.package_name.clone(),
        current_value: parsed.current_value.clone(),
        datasource: "npm",
        manager_data: NpmManagerData {
            package_name: parsed.package_name.clone(),
            sha256,
            url: Some(url),
        },
    }
}

/// Build the archive URL for a new NPM version.
pub fn npm_build_archive_urls(manager_data: &NpmManagerData, new_version: &str) -> Vec<String> {
    let pkg = &manager_data.package_name;
    let filename = if pkg.contains('/') {
        pkg.split('/').nth(1).unwrap_or(pkg)
    } else {
        pkg
    };
    vec![format!(
        "https://registry.npmjs.org/{pkg}/-/{filename}-{new_version}.tgz"
    )]
}

// ---------------------------------------------------------------------------
// Handler dispatch
// ---------------------------------------------------------------------------

/// Identifies which handler type was matched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HomebrewHandlerType {
    GitHub,
    Npm,
}

impl HomebrewHandlerType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GitHub => "github",
            Self::Npm => "npm",
        }
    }
}

/// Parsed result from any handler, tagged by type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HomebrewHandlerParsed {
    GitHub(GitHubParsedResult),
    Npm(NpmParsedResult),
}

/// Result of [`find_handler`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FindHandlerResult {
    pub handler_type: HomebrewHandlerType,
    pub parsed: HomebrewHandlerParsed,
}

/// Find the appropriate handler for a URL (GitHub before NPM).
pub fn find_handler(url: Option<&str>) -> Option<FindHandlerResult> {
    let url = url?;
    if let Some(p) = github_parse_url(url) {
        return Some(FindHandlerResult {
            handler_type: HomebrewHandlerType::GitHub,
            parsed: HomebrewHandlerParsed::GitHub(p),
        });
    }
    if let Some(p) = npm_parse_url(url) {
        return Some(FindHandlerResult {
            handler_type: HomebrewHandlerType::Npm,
            parsed: HomebrewHandlerParsed::Npm(p),
        });
    }
    None
}

/// Find a handler by its type string (`"github"` or `"npm"`).
pub fn find_handler_by_type(handler_type: &str) -> Option<HomebrewHandlerType> {
    match handler_type {
        "github" => Some(HomebrewHandlerType::GitHub),
        "npm" => Some(HomebrewHandlerType::Npm),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "extracts \"archive\" github dependency" — homebrew/extract.spec.ts line 99
    #[test]
    fn extracts_github_archive_refs_tags() {
        let content = r#"
class Mylib < Formula
  url "https://github.com/owner/mylib/archive/refs/tags/v1.2.3.tar.gz"
  sha256 "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
end"#;
        let dep = extract(content).unwrap();
        assert_eq!(dep.formula_name, "Mylib");
        assert_eq!(dep.current_value, "1.2.3");
        assert_eq!(
            dep.source,
            HomebrewSource::GitHub {
                repo: "owner/mylib".to_owned(),
                url_type: GitHubUrlType::Archive,
            }
        );
        assert!(dep.skip_reason.is_none());
    }

    // Ported: "handles old \"archive\" github url format" — homebrew/extract.spec.ts line 121
    #[test]
    fn extracts_github_archive_old_form() {
        let content = r#"class MyApp < Formula
  url "https://github.com/owner/myapp/archive/v2.0.0.tar.gz"
  sha256 "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
end"#;
        let dep = extract(content).unwrap();
        assert_eq!(dep.current_value, "2.0.0");
        assert_eq!(
            dep.source,
            HomebrewSource::GitHub {
                repo: "owner/myapp".to_owned(),
                url_type: GitHubUrlType::Archive,
            }
        );
    }

    // Ported: "extracts \"releases\" github dependency" — homebrew/extract.spec.ts line 77
    #[test]
    fn extracts_github_release() {
        let content = r#"class Mytool < Formula
  url "https://github.com/owner/mytool/releases/download/v3.1.0/mytool-linux.tar.gz"
  sha256 "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
end"#;
        let dep = extract(content).unwrap();
        assert_eq!(dep.current_value, "3.1.0");
        assert_eq!(
            dep.source,
            HomebrewSource::GitHub {
                repo: "owner/mytool".to_owned(),
                url_type: GitHubUrlType::Release,
            }
        );
    }

    // Ported: "skips if sha256 field is invalid" — homebrew/extract.spec.ts line 301
    #[test]
    fn invalid_sha256_skipped() {
        let content = r#"class Bad < Formula
  url "https://github.com/owner/repo/archive/v1.0.0.tar.gz"
  sha256 "tooshort"
end"#;
        let dep = extract(content).unwrap();
        assert_eq!(dep.skip_reason, Some(HomebrewSkipReason::InvalidSha256));
    }

    // Ported: "skips sourceforge dependency 1" — homebrew/extract.spec.ts line 10
    #[test]
    fn unsupported_url_skipped() {
        let content = r#"class Other < Formula
  url "https://example.com/releases/v1.0.tar.gz"
  sha256 "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
end"#;
        let dep = extract(content).unwrap();
        assert_eq!(dep.skip_reason, Some(HomebrewSkipReason::UnsupportedUrl));
    }

    // Ported: "skips if there is no url field" — homebrew/extract.spec.ts line 213
    #[test]
    fn missing_url_skipped() {
        let dep = extract("class NoUrl < Formula\nend").unwrap();
        assert_eq!(dep.skip_reason, Some(HomebrewSkipReason::MissingUrl));
    }

    // Ported: "returns null for invalid class header 2" — homebrew/extract.spec.ts line 198
    #[test]
    fn invalid_class_header_not_formula_returns_none() {
        // "class X < NotFormula" is not a valid Formula class
        let content = "class Ibazel < NotFormula\n  url \"https://example.com/v1.0.tar.gz\"\n  sha256 \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"\nend\n";
        assert!(extract(content).is_none());
    }

    #[test]
    fn empty_content_returns_none() {
        assert!(extract("").is_none());
    }

    // Ported: "returns null for invalid class header 1" — homebrew/extract.spec.ts line 183
    #[test]
    fn no_class_header_returns_none() {
        // Invalid class syntax (no " < Formula") → None
        let content = "class Ibazel !?# Formula\n  url \"https://example.com/v1.0.tar.gz\"\nend\n";
        assert!(extract(content).is_none());
    }

    // Ported: "skips sourceforge dependency 2" — homebrew/extract.spec.ts line 32
    #[test]
    fn skips_sourceforge_dependency_2() {
        let content = r#"class Aap < Formula
  desc "Make-like tool to download, build, and install software"
  homepage "http://www.a-a-p.org"
  url "https://downloads.sourceforge.net/project/a-a-p/aap-1.094.zip"
  sha256 "3f53b2fc277756042449416150acc477f29de93692944f8a77e8cef285a1efd8"
end"#;
        let dep = extract(content).unwrap();
        assert_eq!(dep.formula_name, "Aap");
        assert_eq!(dep.skip_reason, Some(HomebrewSkipReason::UnsupportedUrl));
    }

    // Ported: "skips github dependency with wrong format" — homebrew/extract.spec.ts line 54
    #[test]
    fn skips_github_dependency_wrong_format() {
        // Git-style URL with :tag/:revision instead of archive/release — no sha256 field
        let content = r#"class Acmetool < Formula
  desc "Automatic certificate acquisition tool for ACME (Let's Encrypt)"
  homepage "https://github.com/hlandau/acme"
  url "https://github.com/hlandau/acme.git",
    :tag      => "v0.0.67",
    :revision => "221ea15246f0bbcf254b350bee272d43a1820285"
end"#;
        let dep = extract(content).unwrap();
        assert_eq!(dep.formula_name, "Acmetool");
        assert_eq!(dep.skip_reason, Some(HomebrewSkipReason::InvalidSha256));
    }

    // Ported: "handles no space before class header" — homebrew/extract.spec.ts line 152
    #[test]
    fn handles_no_space_before_class_header() {
        let content = r#"class Ibazel < Formula
  desc "IBazel is a tool for building Bazel targets when source files change."
  homepage "https://github.com/bazelbuild/bazel-watcher"
  url "https://github.com/bazelbuild/bazel-watcher/archive/refs/tags/v0.8.2.tar.gz"
  sha256 "26f5125218fad2741d3caf937b02296d803900e5f153f5b1f733f15391b9f9b4"
end"#;
        let dep = extract(content).unwrap();
        assert_eq!(dep.current_value, "0.8.2");
        assert_eq!(
            dep.source,
            HomebrewSource::GitHub {
                repo: "bazelbuild/bazel-watcher".to_owned(),
                url_type: GitHubUrlType::Archive,
            }
        );
        assert!(dep.skip_reason.is_none());
    }

    // Ported: "skips if invalid url protocol" — homebrew/extract.spec.ts line 235
    #[test]
    fn skips_invalid_url_protocol() {
        // url ??https://... has no opening quote — URL_RE won't match → MissingUrl
        let content = "class Ibazel < Formula\n  url ??https://github.com/bazelbuild/bazel-watcher/archive/refs/tags/v0.8.2.tar.gz\"\n  sha256 '26f5125218fad2741d3caf937b02296d803900e5f153f5b1f733f15391b9f9b4'\nend";
        let dep = extract(content).unwrap();
        assert_eq!(dep.skip_reason, Some(HomebrewSkipReason::MissingUrl));
    }

    // Ported: "skips if invalid url" — homebrew/extract.spec.ts line 257
    #[test]
    fn skips_invalid_url() {
        let content = r#"class Ibazel < Formula
  url "invalid_url"
  sha256 "26f5125218fad2741d3caf937b02296d803900e5f153f5b1f733f15391b9f9b4"
end"#;
        let dep = extract(content).unwrap();
        assert_eq!(dep.skip_reason, Some(HomebrewSkipReason::UnsupportedUrl));
    }

    // Ported: "skips if there is no sha256 field" — homebrew/extract.spec.ts line 279
    #[test]
    fn skips_no_sha256_field() {
        let content = r#"class Ibazel < Formula
  url "https://github.com/bazelbuild/bazel-watcher/archive/refs/tags/v0.8.2.tar.gz"
  not_sha256 "26f5125218fad2741d3caf937b02296d803900e5f153f5b1f733f15391b9f9b4"
end"#;
        let dep = extract(content).unwrap();
        assert_eq!(dep.skip_reason, Some(HomebrewSkipReason::InvalidSha256));
    }

    // Ported: "extracts npm scoped package dependency" — homebrew/extract.spec.ts line 323
    #[test]
    fn extracts_npm_scoped_package() {
        let content = r#"class ClaudeCode < Formula
  desc "Anthropic's official CLI for Claude"
  url "https://registry.npmjs.org/@anthropic-ai/claude-code/-/claude-code-0.1.0.tgz"
  sha256 "345eae3fe4c682df3d8876141f32035bb2898263ce5a406e76e1d74ccb13f601"
end"#;
        let dep = extract(content).unwrap();
        assert_eq!(dep.formula_name, "ClaudeCode");
        assert_eq!(dep.current_value, "0.1.0");
        assert_eq!(
            dep.source,
            HomebrewSource::Npm {
                package: "@anthropic-ai/claude-code".to_owned()
            }
        );
        assert!(dep.skip_reason.is_none());
    }

    // Ported: "extracts npm unscoped package dependency" — homebrew/extract.spec.ts line 354
    #[test]
    fn extracts_npm_unscoped_package() {
        let content = r#"class Express < Formula
  desc "Fast, unopinionated, minimalist web framework"
  url "https://registry.npmjs.org/express/-/express-4.18.2.tgz"
  sha256 "abcd1234567890abcd1234567890abcd1234567890abcd1234567890abcd1234"
end"#;
        let dep = extract(content).unwrap();
        assert_eq!(dep.current_value, "4.18.2");
        assert_eq!(
            dep.source,
            HomebrewSource::Npm {
                package: "express".to_owned()
            }
        );
        assert!(dep.skip_reason.is_none());
    }

    // Ported: "skips npm package from custom registry" — homebrew/extract.spec.ts line 385
    #[test]
    fn skips_npm_custom_registry() {
        let content = r#"class CustomPackage < Formula
  url "https://registry.company.com/package/-/package-1.0.0.tgz"
  sha256 "abcd1234567890abcd1234567890abcd1234567890abcd1234567890abcd1234"
end"#;
        let dep = extract(content).unwrap();
        assert_eq!(dep.skip_reason, Some(HomebrewSkipReason::UnsupportedUrl));
    }

    // --- github handler tests ---

    // Ported: "returns null for empty string" — homebrew/handlers/github.spec.ts line 8
    #[test]
    fn github_parse_url_empty_string_returns_none() {
        assert!(github_parse_url("").is_none());
    }

    // Ported: "parses valid releases URL" — homebrew/handlers/github.spec.ts line 19
    #[test]
    fn github_parse_url_releases() {
        let result = github_parse_url(
            "https://github.com/aide/aide/releases/download/v0.16.1/aide-0.16.1.tar.gz",
        )
        .unwrap();
        assert_eq!(result.current_value, "v0.16.1");
        assert_eq!(result.owner_name, "aide");
        assert_eq!(result.repo_name, "aide");
        assert_eq!(result.url_type, GitHubUrlType::Release);
    }

    // Ported: "parses valid archive URL" — homebrew/handlers/github.spec.ts line 33
    #[test]
    fn github_parse_url_archive() {
        let result = github_parse_url(
            "https://github.com/bazelbuild/bazel-watcher/archive/refs/tags/v0.8.2.tar.gz",
        )
        .unwrap();
        assert_eq!(result.current_value, "v0.8.2");
        assert_eq!(result.owner_name, "bazelbuild");
        assert_eq!(result.repo_name, "bazel-watcher");
        assert_eq!(result.url_type, GitHubUrlType::Archive);
    }

    // Ported: "uses original version when semver.coerce fails" — homebrew/handlers/github.spec.ts line 49
    #[test]
    fn github_build_archive_urls_non_semver() {
        let data = GitHubManagerData {
            owner_name: "owner".to_owned(),
            repo_name: "repo".to_owned(),
            sha256: Some("abc123".to_owned()),
            url: Some(
                "https://github.com/owner/repo/archive/refs/tags/not-a-semver.tar.gz".to_owned(),
            ),
        };
        let urls = github_build_archive_urls(&data, "also-not-semver");
        assert_eq!(
            urls,
            vec![
                "https://github.com/owner/repo/releases/download/also-not-semver/repo-also-not-semver.tar.gz",
                "https://github.com/owner/repo/archive/refs/tags/also-not-semver.tar.gz",
            ]
        );
    }

    // Ported: "uses coerced version for filename when semver succeeds" — homebrew/handlers/github.spec.ts line 66
    #[test]
    fn github_build_archive_urls_semver_coerce() {
        let data = GitHubManagerData {
            owner_name: "owner".to_owned(),
            repo_name: "repo".to_owned(),
            sha256: Some("abc123".to_owned()),
            url: Some(
                "https://github.com/owner/repo/archive/refs/tags/v1.2.3.tar.gz".to_owned(),
            ),
        };
        let urls = github_build_archive_urls(&data, "v1.2.4");
        assert_eq!(
            urls,
            vec![
                "https://github.com/owner/repo/releases/download/v1.2.4/repo-1.2.4.tar.gz",
                "https://github.com/owner/repo/archive/refs/tags/v1.2.4.tar.gz",
            ]
        );
    }

    // Ported: "creates dependency with github-releases datasource for releases URL" — homebrew/handlers/github.spec.ts line 85
    #[test]
    fn github_create_dependency_releases() {
        let parsed = GitHubParsedResult {
            current_value: "v0.16.1".to_owned(),
            owner_name: "aide".to_owned(),
            repo_name: "aide".to_owned(),
            url_type: GitHubUrlType::Release,
        };
        let dep = github_create_dependency(
            &parsed,
            Some("0f2b7cecc70c1a27d35c06c98804fcdb9f326630de5d035afc447122186010b7".to_owned()),
            "https://github.com/aide/aide/releases/download/v0.16.1/aide-0.16.1.tar.gz"
                .to_owned(),
        );
        assert_eq!(dep.dep_name, "aide/aide");
        assert_eq!(dep.current_value, "v0.16.1");
        assert_eq!(dep.datasource, "github-releases");
    }

    // Ported: "creates dependency with github-tags datasource for archive URL" — homebrew/handlers/github.spec.ts line 107
    #[test]
    fn github_create_dependency_archive() {
        let parsed = GitHubParsedResult {
            current_value: "v0.8.2".to_owned(),
            owner_name: "bazelbuild".to_owned(),
            repo_name: "bazel-watcher".to_owned(),
            url_type: GitHubUrlType::Archive,
        };
        let dep = github_create_dependency(
            &parsed,
            Some("26f5125218fad2741d3caf937b02296d803900e5f153f5b1f733f15391b9f9b4".to_owned()),
            "https://github.com/bazelbuild/bazel-watcher/archive/refs/tags/v0.8.2.tar.gz"
                .to_owned(),
        );
        assert_eq!(dep.dep_name, "bazelbuild/bazel-watcher");
        assert_eq!(dep.current_value, "v0.8.2");
        assert_eq!(dep.datasource, "github-tags");
    }

    // --- npm handler tests ---

    // Ported: "returns null for empty string" — homebrew/handlers/npm.spec.ts line 8
    #[test]
    fn npm_parse_url_empty_string_returns_none() {
        assert!(npm_parse_url("").is_none());
    }

    // Ported: "returns null for non-npm registry URL" — homebrew/handlers/npm.spec.ts line 19
    #[test]
    fn npm_parse_url_non_npm_registry_returns_none() {
        assert!(npm_parse_url("https://example.com/package/-/package-1.0.0.tgz").is_none());
    }

    // Ported: "returns null for custom npm registry" — homebrew/handlers/npm.spec.ts line 25
    #[test]
    fn npm_parse_url_custom_registry_returns_none() {
        assert!(
            npm_parse_url("https://registry.company.com/package/-/package-1.0.0.tgz").is_none()
        );
    }

    // Ported: "parses scoped package URL" — homebrew/handlers/npm.spec.ts line 33
    #[test]
    fn npm_parse_url_scoped_package() {
        let result = npm_parse_url(
            "https://registry.npmjs.org/@anthropic-ai/claude-code/-/claude-code-0.1.0.tgz",
        )
        .unwrap();
        assert_eq!(result.current_value, "0.1.0");
        assert_eq!(result.package_name, "@anthropic-ai/claude-code");
    }

    // Ported: "parses unscoped package URL" — homebrew/handlers/npm.spec.ts line 45
    #[test]
    fn npm_parse_url_unscoped_package() {
        let result =
            npm_parse_url("https://registry.npmjs.org/express/-/express-4.18.2.tgz").unwrap();
        assert_eq!(result.current_value, "4.18.2");
        assert_eq!(result.package_name, "express");
    }

    // Ported: "parses version with prerelease" — homebrew/handlers/npm.spec.ts line 57
    #[test]
    fn npm_parse_url_prerelease_version() {
        let result = npm_parse_url(
            "https://registry.npmjs.org/package/-/package-1.0.0-beta.1.tgz",
        )
        .unwrap();
        assert_eq!(result.current_value, "1.0.0-beta.1");
        assert_eq!(result.package_name, "package");
    }

    // Ported: "parses version with build metadata" — homebrew/handlers/npm.spec.ts line 69
    #[test]
    fn npm_parse_url_build_metadata_version() {
        let result = npm_parse_url(
            "https://registry.npmjs.org/package/-/package-1.0.0-alpha.2.tgz",
        )
        .unwrap();
        assert_eq!(result.current_value, "1.0.0-alpha.2");
        assert_eq!(result.package_name, "package");
    }

    // Ported: "returns null for malformed URL" — homebrew/handlers/npm.spec.ts line 81
    #[test]
    fn npm_parse_url_malformed_returns_none() {
        assert!(npm_parse_url("https://registry.npmjs.org/invalid-url").is_none());
    }

    // Ported: "creates dependency with npm datasource for scoped package" — homebrew/handlers/npm.spec.ts line 89
    #[test]
    fn npm_create_dependency_scoped() {
        let parsed = NpmParsedResult {
            current_value: "0.1.0".to_owned(),
            package_name: "@anthropic-ai/claude-code".to_owned(),
        };
        let dep = npm_create_dependency(
            &parsed,
            Some("345eae3fe4c682df3d8876141f32035bb2898263ce5a406e76e1d74ccb13f601".to_owned()),
            "https://registry.npmjs.org/@anthropic-ai/claude-code/-/claude-code-0.1.0.tgz"
                .to_owned(),
        );
        assert_eq!(dep.dep_name, "@anthropic-ai/claude-code");
        assert_eq!(dep.current_value, "0.1.0");
        assert_eq!(dep.datasource, "npm");
        assert_eq!(dep.manager_data.package_name, "@anthropic-ai/claude-code");
        assert_eq!(
            dep.manager_data.sha256.as_deref(),
            Some("345eae3fe4c682df3d8876141f32035bb2898263ce5a406e76e1d74ccb13f601")
        );
    }

    // Ported: "creates dependency with npm datasource for unscoped package" — homebrew/handlers/npm.spec.ts line 116
    #[test]
    fn npm_create_dependency_unscoped() {
        let parsed = NpmParsedResult {
            current_value: "4.18.2".to_owned(),
            package_name: "express".to_owned(),
        };
        let dep = npm_create_dependency(
            &parsed,
            Some("abcd1234567890abcd1234567890abcd1234567890abcd1234567890abcd1234".to_owned()),
            "https://registry.npmjs.org/express/-/express-4.18.2.tgz".to_owned(),
        );
        assert_eq!(dep.dep_name, "express");
        assert_eq!(dep.current_value, "4.18.2");
        assert_eq!(dep.datasource, "npm");
    }

    // Ported: "builds URL for scoped package" — homebrew/handlers/npm.spec.ts line 145
    #[test]
    fn npm_build_archive_urls_scoped() {
        let data = NpmManagerData {
            package_name: "@anthropic-ai/claude-code".to_owned(),
            sha256: Some("abc123".to_owned()),
            url: Some("https://registry.npmjs.org/@anthropic-ai/claude-code/-/claude-code-0.1.0.tgz".to_owned()),
        };
        let urls = npm_build_archive_urls(&data, "0.2.0");
        assert_eq!(
            urls,
            vec!["https://registry.npmjs.org/@anthropic-ai/claude-code/-/claude-code-0.2.0.tgz"]
        );
    }

    // Ported: "builds URL for unscoped package" — homebrew/handlers/npm.spec.ts line 160
    #[test]
    fn npm_build_archive_urls_unscoped() {
        let data = NpmManagerData {
            package_name: "express".to_owned(),
            sha256: Some("abc123".to_owned()),
            url: Some("https://registry.npmjs.org/express/-/express-4.18.2.tgz".to_owned()),
        };
        let urls = npm_build_archive_urls(&data, "4.18.3");
        assert_eq!(
            urls,
            vec!["https://registry.npmjs.org/express/-/express-4.18.3.tgz"]
        );
    }

    // Ported: "builds URL with prerelease version" — homebrew/handlers/npm.spec.ts line 175
    #[test]
    fn npm_build_archive_urls_prerelease() {
        let data = NpmManagerData {
            package_name: "package".to_owned(),
            sha256: Some("abc123".to_owned()),
            url: Some("https://registry.npmjs.org/package/-/package-1.0.0.tgz".to_owned()),
        };
        let urls = npm_build_archive_urls(&data, "2.0.0-beta.1");
        assert_eq!(
            urls,
            vec!["https://registry.npmjs.org/package/-/package-2.0.0-beta.1.tgz"]
        );
    }

    // Ported: "builds URL for deeply scoped package" — homebrew/handlers/npm.spec.ts line 190
    #[test]
    fn npm_build_archive_urls_deeply_scoped() {
        let data = NpmManagerData {
            package_name: "@scope/package-name".to_owned(),
            sha256: Some("abc123".to_owned()),
            url: Some("https://registry.npmjs.org/@scope/package-name/-/package-name-1.0.0.tgz".to_owned()),
        };
        let urls = npm_build_archive_urls(&data, "1.1.0");
        assert_eq!(
            urls,
            vec!["https://registry.npmjs.org/@scope/package-name/-/package-name-1.1.0.tgz"]
        );
    }

    // --- handler dispatch tests ---

    // Ported: "returns null for handler type \"unknown\"" — homebrew/handlers/index.spec.ts line 5
    #[test]
    fn find_handler_by_type_unknown_returns_none() {
        assert!(find_handler_by_type("unknown").is_none());
    }

    // Ported: "returns null for handler type \"\"" — homebrew/handlers/index.spec.ts line 5
    #[test]
    fn find_handler_by_type_empty_returns_none() {
        assert!(find_handler_by_type("").is_none());
    }

    // Ported: "returns github handler for github type" — homebrew/handlers/index.spec.ts line 9
    #[test]
    fn find_handler_by_type_github() {
        let h = find_handler_by_type("github").unwrap();
        assert_eq!(h, HomebrewHandlerType::GitHub);
        assert_eq!(h.as_str(), "github");
    }

    // Ported: "returns null for null URL" — homebrew/handlers/index.spec.ts line 16
    #[test]
    fn find_handler_none_url_returns_none() {
        assert!(find_handler(None).is_none());
    }

    // Ported: "returns null for unsupported URL" — homebrew/handlers/index.spec.ts line 20
    #[test]
    fn find_handler_unsupported_url_returns_none() {
        assert!(find_handler(Some("https://example.com/file.tar.gz")).is_none());
    }

    // Ported: "returns handler and parsed result for GitHub URL" — homebrew/handlers/index.spec.ts line 24
    #[test]
    fn find_handler_github_url() {
        let result = find_handler(Some(
            "https://github.com/aide/aide/releases/download/v0.16.1/aide-0.16.1.tar.gz",
        ))
        .unwrap();
        assert_eq!(result.handler_type, HomebrewHandlerType::GitHub);
        let HomebrewHandlerParsed::GitHub(parsed) = result.parsed else {
            panic!("expected GitHub parsed result");
        };
        assert_eq!(parsed.current_value, "v0.16.1");
        assert_eq!(parsed.owner_name, "aide");
        assert_eq!(parsed.repo_name, "aide");
        assert_eq!(parsed.url_type, GitHubUrlType::Release);
    }
}
