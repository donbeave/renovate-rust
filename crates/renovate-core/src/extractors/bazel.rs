//! Bazel WORKSPACE `http_archive()` dependency extractor.
//!
//! Scans Bazel `WORKSPACE` / `WORKSPACE.bazel` / `.bzl` files for
//! `http_archive()` calls and extracts the GitHub/GitLab URL and version for
//! update tracking.
//!
//! Renovate reference:
//! - `lib/modules/manager/bazel/extract.ts`
//! - `lib/modules/manager/bazel/rules/git.ts`
//! - Patterns: `(^|/)WORKSPACE(\.bazel|\.bzlmod)?$`, `\.bzl$`
//! - Datasources: GitHub Tags, GitHub Releases, GitLab Tags, GitLab Releases
//!
//! ## Supported URL forms
//!
//! ```python
//! http_archive(
//!     name = "com_github_google_re2",
//!     sha256 = "abcdef...",
//!     urls = ["https://github.com/google/re2/archive/refs/tags/2023-03-01.tar.gz"],
//! )
//!
//! http_archive(
//!     name = "eigen3",
//!     url = "https://gitlab.com/libeigen/eigen/-/archive/3.3.5/eigen-3.3.5.zip",
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
    /// GitLab archive URL with semver tag → GitLab Releases.
    GitlabReleases { repo: String },
    /// GitLab archive URL with commit digest → GitLab Tags.
    GitlabTags { repo: String },
    /// Non-GitHub/GitLab or unrecognised URL.
    Unsupported,
}

/// Skip reason for a Bazel dep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BazelSkipReason {
    /// No GitHub/GitLab URL found in `url`/`urls`.
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
    /// Extracted commit digest (40-char hex, for GitLab/GitHub digest refs).
    pub current_digest: Option<String>,
    /// Source routing.
    pub source: BazelSource,
    pub skip_reason: Option<BazelSkipReason>,
}

// ── Regexes ───────────────────────────────────────────────────────────────────

/// A single quoted URL anywhere in a block.
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

/// GitLab archive URL (anchored to https://gitlab.com): owner/repo and ref.
static GL_ARCHIVE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"https?://gitlab\.com/([^/]+/[^/]+)/-/archive/([^/]+)/").unwrap()
});

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
    // Extract `name = "..."` (handles both inline and next-line forms).
    let name_re = Regex::new(r#"name\s*=\s*["']([^"']+)["']"#).unwrap();
    let dep_name = name_re
        .captures(block)
        .map(|c| c[1].to_owned())
        .unwrap_or_else(|| "unknown".to_owned());

    // Collect all HTTP URLs from the entire block (handles both `url = "..."` and `urls = [...]`).
    let urls: Vec<&str> = URL_RE
        .captures_iter(block)
        .map(|c| c.get(1).unwrap().as_str())
        .collect();

    if urls.is_empty() {
        return Some(BazelDep {
            dep_name,
            current_value: String::new(),
            current_digest: None,
            source: BazelSource::Unsupported,
            skip_reason: Some(BazelSkipReason::NoGithubUrl),
        });
    }

    // Find the first recognisable URL — GitLab takes priority over GitHub mirrors.
    for url in &urls {
        // GitLab archive (must be anchored to gitlab.com, not a mirror).
        if let Some(cap) = GL_ARCHIVE_RE.captures(url) {
            let repo = cap[1].to_owned();
            let ref_str = &cap[2];
            // 40-char lowercase hex = commit digest → GitLab Tags.
            if ref_str.len() == 40 && ref_str.chars().all(|c| c.is_ascii_hexdigit()) {
                return Some(BazelDep {
                    dep_name,
                    current_value: String::new(),
                    current_digest: Some(ref_str.to_owned()),
                    source: BazelSource::GitlabTags { repo },
                    skip_reason: None,
                });
            }
            // Semver-style tag → GitLab Releases.
            return Some(BazelDep {
                dep_name,
                current_value: ref_str.to_owned(),
                current_digest: None,
                source: BazelSource::GitlabReleases { repo },
                skip_reason: None,
            });
        }

        // GitHub archive.
        if let Some(cap) = GH_ARCHIVE_RE.captures(url) {
            let repo = cap[1].to_owned();
            let version = cap[2].trim_start_matches('v').to_owned();
            return Some(BazelDep {
                dep_name,
                current_value: version,
                current_digest: None,
                source: BazelSource::GithubTags { repo },
                skip_reason: None,
            });
        }

        // GitHub release.
        if let Some(cap) = GH_RELEASE_RE.captures(url) {
            let repo = cap[1].to_owned();
            let version = cap[2].trim_start_matches('v').to_owned();
            return Some(BazelDep {
                dep_name,
                current_value: version,
                current_digest: None,
                source: BazelSource::GithubReleases { repo },
                skip_reason: None,
            });
        }
    }

    Some(BazelDep {
        dep_name,
        current_value: String::new(),
        current_digest: None,
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

    #[test]
    fn invalid_content_returns_empty() {
        // Ported: "returns empty if fails to parse" — bazel/extract.spec.ts line 13
        assert!(extract("blahhhhh:foo:@what\n").is_empty());
    }

    #[test]
    fn git_repository_without_url_returns_empty() {
        // Ported: "returns empty if cannot parse dependency" — bazel/extract.spec.ts line 18
        // We only handle http_archive; git_repository alone returns nothing.
        assert!(extract("git_repository(\n  nothing\n)\n").is_empty());
    }

    #[test]
    fn singular_url_form_extracted() {
        // Ported: "sequential http_archive" (first archive uses `url =` singular) — bazel/extract.spec.ts line 126
        let content = r#"
http_archive(
  name = "aspect_rules_js",
  sha256 = "db9f446752fe4100320cf8487e8fd476b9af0adf6b99b601bcfd70b289bb0598",
  strip_prefix = "rules_js-1.1.2",
  url = "https://github.com/aspect-build/rules_js/archive/refs/tags/v1.1.2.tar.gz",
)

http_archive(
    name = "rules_nodejs",
    sha256 = "5aef09ed3279aa01d5c928e3beb248f9ad32dde6aafe6373a8c994c3ce643064",
    urls = ["https://github.com/bazelbuild/rules_nodejs/releases/download/5.5.3/rules_nodejs-core-5.5.3.tar.gz"],
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_name, "aspect_rules_js");
        assert_eq!(deps[0].current_value, "1.1.2");
        assert_eq!(deps[1].dep_name, "rules_nodejs");
        assert_eq!(deps[1].current_value, "5.5.3");
    }

    #[test]
    fn gitlab_archive_with_version_extracted() {
        // Ported: "http_archive with GitLab url" (semver version) — bazel/extract.spec.ts line 160
        let content = r#"
http_archive(
  name = "eigen3",
  url = "https://gitlab.com/libeigen/eigen/-/archive/3.3.5/eigen-3.3.5.zip",
  strip_prefix = "eigen-3.3.5",
  sha256 = "0e7ae...",
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "eigen3");
        assert_eq!(deps[0].current_value, "3.3.5");
        assert_eq!(
            deps[0].source,
            BazelSource::GitlabReleases {
                repo: "libeigen/eigen".to_owned()
            }
        );
        assert!(deps[0].current_digest.is_none());
        assert!(deps[0].skip_reason.is_none());
    }

    #[test]
    fn gitlab_archive_with_commit_digest_extracted() {
        // Ported: "http_archive with GitLab url" (commit digest) — bazel/extract.spec.ts line 160
        let digest = "90ee821c563fa20db4d64d6991ddca256d5c52f2";
        let content = format!(
            r#"
http_archive(
  name = "eigen",
  sha256 = "d76992f1",
  strip_prefix = "eigen-{digest}",
  urls = [
      "https://storage.googleapis.com/mirror.tensorflow.org/gitlab.com/libeigen/eigen/-/archive/{digest}/eigen-{digest}.tar.gz",
      "https://gitlab.com/foo/bar",
      "https://gitlab.com/libeigen/eigen/-/archive/{digest}/eigen-{digest}.tar.gz",
  ],
)
"#
        );
        let deps = extract(&content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "eigen");
        assert_eq!(deps[0].current_digest.as_deref(), Some(digest));
        assert_eq!(
            deps[0].source,
            BazelSource::GitlabTags {
                repo: "libeigen/eigen".to_owned()
            }
        );
        assert!(deps[0].skip_reason.is_none());
    }
}
