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
}
