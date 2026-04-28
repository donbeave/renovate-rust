//! Bazel WORKSPACE `http_archive()` dependency extractor.
//!
//! Scans Bazel `WORKSPACE` / `WORKSPACE.bazel` / `.bzl` files for
//! `http_archive()` calls and extracts the GitHub URL and version for update
//! tracking.
//!
//! Renovate reference:
//! - `lib/modules/manager/bazel/extract.ts`
//! - `lib/modules/manager/bazel/rules/git.ts`
//! - Patterns: `(^|/)WORKSPACE(\.bazel|\.bzlmod)?$`, `\.bzl$`
//! - Datasources: GitHub Tags, GitHub Releases
//!
//! ## Supported URL forms
//!
//! ```python
//! http_archive(
//!     name = "com_github_google_re2",
//!     sha256 = "abcdef...",
//!     urls = ["https://github.com/google/re2/archive/refs/tags/2023-03-01.tar.gz"],
//! )
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// Source datasource for a Bazel http_archive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BazelSource {
    /// GitHub archive URL → GitHub Tags.
    GithubTags { repo: String },
    /// GitHub release download URL → GitHub Releases.
    GithubReleases { repo: String },
    /// Non-GitHub or unrecognised URL.
    Unsupported,
}

/// Skip reason for a Bazel dep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BazelSkipReason {
    /// No GitHub URL found in `urls`.
    NoGithubUrl,
    /// `sha256` field is missing (reproducibility concern, skip).
    MissingSha256,
}

/// A single Bazel `http_archive` dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BazelDep {
    /// Archive name (the Bazel workspace rule name).
    pub dep_name: String,
    /// Extracted version (stripped of `v` prefix, or raw tag).
    pub current_value: String,
    /// Source routing.
    pub source: BazelSource,
    pub skip_reason: Option<BazelSkipReason>,
}

// ── Regexes ───────────────────────────────────────────────────────────────────

/// Match `http_archive(` block start — name capture on next line or same line.
static ARCHIVE_NAME_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?m)^\s*http_archive\s*\(\s*\n\s*name\s*=\s*["']([^"']+)["']"#).unwrap()
});

/// A single URL in the `urls = [...]` list.
static URL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"["'](https?://[^"']+)["']"#).unwrap());

/// GitHub archive URL: owner/repo and tag.
static GH_ARCHIVE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"github\.com/([^/]+/[^/]+)/archive(?:/refs/tags)?/([^/]+?)(?:\.tar\.gz|\.zip|\.tar\.bz2)?$",
    )
    .unwrap()
});

/// GitHub release download URL: owner/repo and tag.
static GH_RELEASE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"github\.com/([^/]+/[^/]+)/releases/download/([^/]+)/").unwrap());

// ── Parsing ───────────────────────────────────────────────────────────────────

/// Extract Bazel `http_archive` dependencies from a WORKSPACE or .bzl file.
pub fn extract(content: &str) -> Vec<BazelDep> {
    let mut deps = Vec::new();

    // Find each `http_archive(` block by scanning for the opening and closing.
    let mut search_pos = 0;
    while let Some(start) = content[search_pos..].find("http_archive(") {
        let abs_start = search_pos + start;
        // Find the matching closing `)` — simple brace counting.
        let Some(block) = extract_block(&content[abs_start..]) else {
            break;
        };
        if let Some(dep) = parse_http_archive(block) {
            deps.push(dep);
        }
        search_pos = abs_start + block.len().max(1);
    }

    deps
}

/// Extract the content of a `http_archive(...)` block (including the outer parens).
fn extract_block(s: &str) -> Option<&str> {
    let open = s.find('(')?;
    let mut depth = 0usize;
    for (i, ch) in s[open..].char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&s[..open + i + 1]);
                }
            }
            _ => {}
        }
    }
    None
}

fn parse_http_archive(block: &str) -> Option<BazelDep> {
    // Extract `name = "..."`.
    let name_cap = ARCHIVE_NAME_RE.captures(block);
    let dep_name = match name_cap {
        Some(ref c) => c[1].to_owned(),
        None => {
            // Try simpler inline name pattern.
            let name_re = Regex::new(r#"name\s*=\s*["']([^"']+)["']"#).unwrap();
            name_re
                .captures(block)
                .map(|c| c[1].to_owned())
                .unwrap_or_else(|| "unknown".to_owned())
        }
    };

    // Find the `urls = [...]` section.
    let urls_start = block.find("urls")?;
    let urls_section = &block[urls_start..];

    // Extract all URLs from the list.
    let urls: Vec<&str> = URL_RE
        .captures_iter(urls_section)
        .map(|c| c.get(1).unwrap().as_str())
        .collect();

    if urls.is_empty() {
        return Some(BazelDep {
            dep_name,
            current_value: String::new(),
            source: BazelSource::Unsupported,
            skip_reason: Some(BazelSkipReason::NoGithubUrl),
        });
    }

    // Find the first GitHub URL.
    for url in &urls {
        if let Some(cap) = GH_ARCHIVE_RE.captures(url) {
            let repo = cap[1].to_owned();
            let version = cap[2].trim_start_matches('v').to_owned();
            return Some(BazelDep {
                dep_name,
                current_value: version,
                source: BazelSource::GithubTags { repo },
                skip_reason: None,
            });
        }
        if let Some(cap) = GH_RELEASE_RE.captures(url) {
            let repo = cap[1].to_owned();
            let version = cap[2].trim_start_matches('v').to_owned();
            return Some(BazelDep {
                dep_name,
                current_value: version,
                source: BazelSource::GithubReleases { repo },
                skip_reason: None,
            });
        }
    }

    Some(BazelDep {
        dep_name,
        current_value: String::new(),
        source: BazelSource::Unsupported,
        skip_reason: Some(BazelSkipReason::NoGithubUrl),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_github_archive_dep() {
        let content = r#"
http_archive(
    name = "com_github_google_re2",
    sha256 = "abcdef1234",
    urls = ["https://github.com/google/re2/archive/refs/tags/2023-03-01.tar.gz"],
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let d = &deps[0];
        assert_eq!(d.dep_name, "com_github_google_re2");
        assert_eq!(d.current_value, "2023-03-01");
        assert_eq!(
            d.source,
            BazelSource::GithubTags {
                repo: "google/re2".to_owned()
            }
        );
        assert!(d.skip_reason.is_none());
    }

    #[test]
    fn extracts_github_release_dep() {
        let content = r#"
http_archive(
    name = "rules_go",
    urls = ["https://github.com/bazelbuild/rules_go/releases/download/v0.41.0/rules_go-v0.41.0.zip"],
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "0.41.0");
        assert_eq!(
            deps[0].source,
            BazelSource::GithubReleases {
                repo: "bazelbuild/rules_go".to_owned()
            }
        );
    }

    #[test]
    fn extracts_multiple_archives() {
        let content = r#"
http_archive(
    name = "dep_a",
    urls = ["https://github.com/owner/repo-a/archive/v1.0.0.tar.gz"],
)

http_archive(
    name = "dep_b",
    urls = ["https://github.com/owner/repo-b/archive/refs/tags/v2.0.0.tar.gz"],
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_name, "dep_a");
        assert_eq!(deps[1].dep_name, "dep_b");
    }

    #[test]
    fn skips_non_github_url() {
        let content = r#"
http_archive(
    name = "some_dep",
    urls = ["https://example.com/archive.tar.gz"],
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(BazelSkipReason::NoGithubUrl));
    }

    #[test]
    fn empty_file_returns_empty() {
        assert!(extract("").is_empty());
        assert!(extract("# just comments\n").is_empty());
    }
}
