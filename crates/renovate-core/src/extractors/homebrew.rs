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
static NPM_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"registry\.npmjs\.org/(@[^/]+/[^/]+|[^/@]+)/-/[^-]+-([^/]+)\.tgz$").unwrap()
});

/// Extract a Homebrew formula dependency from a `.rb` formula file.
pub fn extract(content: &str) -> Option<HomebrewDep> {
    // Get formula class name
    let formula_name = CLASS_RE
        .captures(content)
        .map(|cap| cap[1].to_owned())
        .unwrap_or_default();

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

    #[test]
    fn invalid_sha256_skipped() {
        let content = r#"class Bad < Formula
  url "https://github.com/owner/repo/archive/v1.0.0.tar.gz"
  sha256 "tooshort"
end"#;
        let dep = extract(content).unwrap();
        assert_eq!(dep.skip_reason, Some(HomebrewSkipReason::InvalidSha256));
    }

    #[test]
    fn unsupported_url_skipped() {
        let content = r#"class Other < Formula
  url "https://example.com/releases/v1.0.tar.gz"
  sha256 "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
end"#;
        let dep = extract(content).unwrap();
        assert_eq!(dep.skip_reason, Some(HomebrewSkipReason::UnsupportedUrl));
    }

    #[test]
    fn missing_url_skipped() {
        let dep = extract("class NoUrl < Formula\nend").unwrap();
        assert_eq!(dep.skip_reason, Some(HomebrewSkipReason::MissingUrl));
    }
}
