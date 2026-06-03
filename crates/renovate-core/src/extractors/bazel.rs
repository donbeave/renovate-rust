//! Bazel WORKSPACE dependency extractor.
//!
//! Scans Bazel `WORKSPACE` / `WORKSPACE.bazel` / `.bzl` files for
//! `http_archive()`, `container_pull()`, and `oci_pull()` calls.
//!
//! Renovate reference:
//! - `lib/modules/manager/bazel/extract.ts`
//! - `lib/modules/manager/bazel/rules/git.ts`
//! - Patterns: `(^|/)WORKSPACE(\.bazel|\.bzlmod)?$`, `\.bzl$`
//! - Datasources: GitHub Tags, GitHub Releases, GitLab Tags, GitLab Releases,
//!   Docker (container_pull, oci_pull)

use std::sync::LazyLock;

use regex::Regex;

/// Source datasource for a Bazel dep.
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
    /// `container_pull()` → Docker datasource (registry + repository).
    ContainerPull {
        package_name: String,
        registry_url: String,
    },
    /// `oci_pull()` → Docker datasource (full image URL).
    OciPull { image: String },
    /// `go_repository()` → Go datasource.
    GoRepository {
        importpath: String,
        package_name: String,
    },
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
    /// `go_repository(remote=...)` points at an unsupported remote.
    UnsupportedRemote,
}

/// A single Bazel dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BazelDep {
    /// Rule name (the Bazel workspace rule name).
    pub dep_name: String,
    /// Extracted version tag.
    pub current_value: String,
    /// Extracted commit digest (40-char hex, or sha256:... for containers).
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
static GL_ARCHIVE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"https?://gitlab\.com/([^/]+/[^/]+)/-/archive/([^/]+)/").unwrap());

// ── Parsing ───────────────────────────────────────────────────────────────────

/// Extract Bazel dependencies from a WORKSPACE or .bzl file.
pub fn extract(content: &str) -> Vec<BazelDep> {
    let content = strip_comment_lines(content);
    let mut blocks = Vec::new();

    collect_rule_blocks(&content, "http_archive(", parse_http_archive, &mut blocks);
    collect_rule_blocks(&content, "http_file(", parse_http_archive, &mut blocks);
    collect_rule_blocks(
        &content,
        "container_pull(",
        parse_container_pull,
        &mut blocks,
    );
    collect_rule_blocks(&content, "oci_pull(", parse_oci_pull, &mut blocks);
    collect_rule_blocks(&content, "go_repository(", parse_go_repository, &mut blocks);
    collect_rule_blocks(
        &content,
        "git_repository(",
        parse_git_repository,
        &mut blocks,
    );
    collect_rule_blocks(
        &content,
        "new_git_repository(",
        parse_git_repository,
        &mut blocks,
    );
    collect_maybe_rule_blocks(&content, "http_archive", parse_http_archive, &mut blocks);
    collect_maybe_rule_blocks(&content, "go_repository", parse_go_repository, &mut blocks);

    blocks.sort_by_key(|(pos, _, _)| *pos);
    blocks
        .into_iter()
        .filter_map(|(_, block, parser)| parser(block))
        .collect()
}

fn strip_comment_lines(content: &str) -> String {
    content
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n")
}

type RuleParser = fn(&str) -> Option<BazelDep>;
type RuleBlock<'a> = (usize, &'a str, RuleParser);

fn collect_rule_blocks<'a>(
    content: &'a str,
    prefix: &str,
    parser: RuleParser,
    out: &mut Vec<RuleBlock<'a>>,
) {
    let mut search_pos = 0;
    while let Some(start) = content[search_pos..].find(prefix) {
        let abs_start = search_pos + start;
        if content[..abs_start]
            .chars()
            .next_back()
            .is_some_and(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        {
            search_pos = abs_start + prefix.len();
            continue;
        }
        let Some(block) = extract_block(&content[abs_start..]) else {
            break;
        };
        out.push((abs_start, block, parser));
        search_pos = abs_start + block.len().max(1);
    }
}

fn collect_maybe_rule_blocks<'a>(
    content: &'a str,
    rule_name: &str,
    parser: RuleParser,
    out: &mut Vec<RuleBlock<'a>>,
) {
    let mut search_pos = 0;
    while let Some(start) = content[search_pos..].find("maybe(") {
        let abs_start = search_pos + start;
        let Some(block) = extract_block(&content[abs_start..]) else {
            break;
        };
        if maybe_wraps_rule(block, rule_name) {
            out.push((abs_start, block, parser));
        }
        search_pos = abs_start + block.len().max(1);
    }
}

fn maybe_wraps_rule(block: &str, rule_name: &str) -> bool {
    let Some(open) = block.find('(') else {
        return false;
    };
    let inner = block[open + 1..].trim_start();
    inner
        .strip_prefix(rule_name)
        .is_some_and(|rest| rest.trim_start().starts_with(','))
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

fn extract_field<'a>(block: &'a str, field: &str) -> Option<&'a str> {
    let re = Regex::new(&format!(
        r#"{}\s*=\s*["']([^"']+)["']"#,
        regex::escape(field)
    ))
    .unwrap();
    re.captures(block).map(|c| {
        let m = c.get(1).unwrap();
        &block[m.start()..m.end()]
    })
}

fn parse_container_pull(block: &str) -> Option<BazelDep> {
    let name_re = Regex::new(r#"name\s*=\s*["']([^"']+)["']"#).unwrap();
    let dep_name = name_re
        .captures(block)
        .map(|c| c[1].to_owned())
        .unwrap_or_else(|| "unknown".to_owned());

    let registry = extract_field(block, "registry").unwrap_or("index.docker.io");
    let repository = extract_field(block, "repository")?;
    let tag = extract_field(block, "tag").unwrap_or("").to_owned();
    let digest = extract_field(block, "digest").map(str::to_owned);

    Some(BazelDep {
        dep_name,
        current_value: tag,
        current_digest: digest,
        source: BazelSource::ContainerPull {
            package_name: repository.to_owned(),
            registry_url: registry.to_owned(),
        },
        skip_reason: None,
    })
}

fn parse_oci_pull(block: &str) -> Option<BazelDep> {
    let name_re = Regex::new(r#"name\s*=\s*["']([^"']+)["']"#).unwrap();
    let dep_name = name_re
        .captures(block)
        .map(|c| c[1].to_owned())
        .unwrap_or_else(|| "unknown".to_owned());

    let image = extract_field(block, "image")?.to_owned();
    let tag = extract_field(block, "tag").unwrap_or("").to_owned();
    let digest = extract_field(block, "digest").map(str::to_owned);

    Some(BazelDep {
        dep_name,
        current_value: tag,
        current_digest: digest,
        source: BazelSource::OciPull { image },
        skip_reason: None,
    })
}

fn parse_go_repository(block: &str) -> Option<BazelDep> {
    let dep_name = extract_field(block, "name").unwrap_or("unknown").to_owned();
    let importpath = extract_field(block, "importpath")?.to_owned();
    let remote = extract_field(block, "remote");
    let commit = extract_field(block, "commit").map(str::to_owned);
    let tag = extract_field(block, "tag").unwrap_or("").to_owned();

    let package_name = match remote {
        Some(remote) => normalize_go_remote(remote),
        None => Some(importpath.clone()),
    };

    let Some(package_name) = package_name else {
        return Some(BazelDep {
            dep_name,
            current_value: String::new(),
            current_digest: commit,
            source: BazelSource::GoRepository {
                importpath,
                package_name: String::new(),
            },
            skip_reason: Some(BazelSkipReason::UnsupportedRemote),
        });
    };

    Some(BazelDep {
        dep_name,
        current_value: tag,
        current_digest: commit,
        source: BazelSource::GoRepository {
            importpath,
            package_name,
        },
        skip_reason: None,
    })
}

fn normalize_go_remote(remote: &str) -> Option<String> {
    let path = remote.strip_prefix("https://github.com/")?;
    if path.contains('#') {
        return None;
    }
    Some(format!("github.com/{}", path.trim_end_matches(".git")))
}

fn parse_git_repository(block: &str) -> Option<BazelDep> {
    let dep_name = extract_field(block, "name").unwrap_or("unknown").to_owned();
    let remote = extract_field(block, "remote")?;
    let repo = normalize_github_remote(remote)?;
    let commit = extract_field(block, "commit").map(str::to_owned);
    let tag = extract_field(block, "tag").unwrap_or("").to_owned();

    let source = if commit.is_some() {
        BazelSource::GithubTags { repo }
    } else {
        BazelSource::GithubReleases { repo }
    };

    Some(BazelDep {
        dep_name,
        current_value: tag,
        current_digest: commit,
        source,
        skip_reason: None,
    })
}

fn normalize_github_remote(remote: &str) -> Option<String> {
    remote
        .strip_prefix("https://github.com/")
        .map(|path| path.trim_end_matches(".git").to_owned())
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
            let ref_str = &cap[2];
            if ref_str.len() == 40 && ref_str.chars().all(|c| c.is_ascii_hexdigit()) {
                return Some(BazelDep {
                    dep_name,
                    current_value: String::new(),
                    current_digest: Some(ref_str.to_owned()),
                    source: BazelSource::GithubTags { repo },
                    skip_reason: None,
                });
            }
            let version = ref_str.trim_start_matches('v').to_owned();
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

    // Ported: "extracts github tags" — lib/modules/manager/bazel/extract.spec.ts line 31
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

    // Ported: "extracts github tags" — lib/modules/manager/bazel/extract.spec.ts line 31
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

    // Ported: "sequential http_archive" — lib/modules/manager/bazel/extract.spec.ts line 166
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

    // Ported: "returns empty if fails to parse" — lib/modules/manager/bazel/extract.spec.ts line 10
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

    // Ported: "returns empty if fails to parse" — lib/modules/manager/bazel/extract.spec.ts line 10
    #[test]
    fn empty_file_returns_empty() {
        assert!(extract("").is_empty());
        assert!(extract("# just comments\n").is_empty());
    }

    // Ported: "returns empty if fails to parse" — lib/modules/manager/bazel/extract.spec.ts line 10
    #[test]
    fn invalid_content_returns_empty() {
        assert!(extract("blahhhhh:foo:@what\n").is_empty());
    }

    // Ported: "returns empty if cannot parse dependency" — lib/modules/manager/bazel/extract.spec.ts line 15
    #[test]
    fn git_repository_without_url_returns_empty() {
        // We only handle http_archive; git_repository alone returns nothing.
        assert!(extract("git_repository(\n  nothing\n)\n").is_empty());
    }

    // Ported: "sequential http_archive" (first archive uses `url =` singular) — lib/modules/manager/bazel/extract.spec.ts line 166
    #[test]
    fn singular_url_form_extracted() {
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

    // Ported: "http_archive with GitLab url" (semver version) — lib/modules/manager/bazel/extract.spec.ts line 190
    #[test]
    fn gitlab_archive_with_version_extracted() {
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

    // Ported: "http_archive with GitLab url" (commit digest) — lib/modules/manager/bazel/extract.spec.ts line 190
    #[test]
    fn gitlab_archive_with_commit_digest_extracted() {
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

    // Ported: "handle comments and strings" — lib/modules/manager/bazel/extract.spec.ts line 42
    #[test]
    fn workspace3_comments_and_strings() {
        let content = r#"
# http_archive(
#     name = "rules_foreign_cc",
#     url = "https://github.com/bazelbuild/rules_foreign_cc/archive/dfccdce2c9d1063c59ddd331b94eb7cb528a96ee.tar.gz",
#     sha256 = "5469ef8b4e2c475de443c13290cf91ba7d1255899442b1e42fcb7fcdee8ceed8",
# )
# FOREIGN_CC_EXPOSE_ALL_FILES = """filegroup(name = "all", srcs = glob(["**"]), visibility = ["//visibility:public"])"""

http_archive(
    name = "com_github_nelhage_rules_boost",
    url = "https://github.com/nelhage/rules_boost/archive/98495a618246683c9058dd87c2c78a2c06087999.tar.gz",
    sha256 = "f92cb7ed66a5b24f97a7fc3917407f808c70d2689273bdd68f93d70a379d22d3",
    strip_prefix = "rules_boost-98495a618246683c9058dd87c2c78a2c06087999",
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "com_github_nelhage_rules_boost");
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("98495a618246683c9058dd87c2c78a2c06087999")
        );
        assert_eq!(
            deps[0].source,
            BazelSource::GithubTags {
                repo: "nelhage/rules_boost".to_owned(),
            }
        );
    }

    // Ported: "returns empty for incomplete dependency" — lib/modules/manager/bazel/extract.spec.ts line 20
    #[test]
    fn http_archive_with_no_url_returns_dep_with_skip_reason() {
        // A git_repository with only foo = "bar" — no URLs — returns nothing
        // because the Rust extractor only scans for http_archive blocks.
        let content = "git_repository(\n foo = \"bar\" \n)";
        assert!(extract(content).is_empty());
    }

    // Ported: "extracts dependencies for container_pull deptype" — lib/modules/manager/bazel/extract.spec.ts line 65
    #[test]
    fn container_pull_extracted() {
        let content = r#"
container_pull(
  name="hasura",
  registry="index.docker.io",
  repository="hasura/graphql-engine",
  digest="sha256:a4e8d8c444ca04fe706649e82263c9f4c2a4229bc30d2a64561b5e1d20cc8548",
  tag="v1.0.0-alpha31.cli-migrations"
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let d = &deps[0];
        assert_eq!(d.dep_name, "hasura");
        assert_eq!(d.current_value, "v1.0.0-alpha31.cli-migrations");
        assert_eq!(
            d.current_digest,
            Some(
                "sha256:a4e8d8c444ca04fe706649e82263c9f4c2a4229bc30d2a64561b5e1d20cc8548"
                    .to_owned()
            )
        );
        assert_eq!(
            d.source,
            BazelSource::ContainerPull {
                package_name: "hasura/graphql-engine".to_owned(),
                registry_url: "index.docker.io".to_owned(),
            }
        );
        assert!(d.skip_reason.is_none());
    }

    // Ported: "extracts dependencies for oci_pull deptype" — lib/modules/manager/bazel/extract.spec.ts line 90
    #[test]
    fn oci_pull_extracted() {
        let content = r#"
oci_pull(
  name="hasura",
  image="index.docker.io/hasura/graphql-engine",
  digest="sha256:a4e8d8c444ca04fe706649e82263c9f4c2a4229bc30d2a64561b5e1d20cc8548",
  tag="v1.0.0-alpha31.cli-migrations"
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let d = &deps[0];
        assert_eq!(d.dep_name, "hasura");
        assert_eq!(d.current_value, "v1.0.0-alpha31.cli-migrations");
        assert_eq!(
            d.current_digest,
            Some(
                "sha256:a4e8d8c444ca04fe706649e82263c9f4c2a4229bc30d2a64561b5e1d20cc8548"
                    .to_owned()
            )
        );
        assert_eq!(
            d.source,
            BazelSource::OciPull {
                image: "index.docker.io/hasura/graphql-engine".to_owned(),
            }
        );
        assert!(d.skip_reason.is_none());
    }

    // Ported: "check remote option in go_repository" — lib/modules/manager/bazel/extract.spec.ts line 113
    #[test]
    fn go_repository_remote_option() {
        let success = extract(
            r#"
go_repository(
  name = "test_repository",
  importpath = "github.com/google/uuid",
  remote = "https://github.com/test/uuid-fork",
  commit = "dec09d789f3dba190787f8b4454c7d3c936fed9e"
)
"#,
        );
        assert_eq!(success.len(), 1);
        assert_eq!(
            success[0].current_digest.as_deref(),
            Some("dec09d789f3dba190787f8b4454c7d3c936fed9e")
        );
        assert_eq!(
            success[0].source,
            BazelSource::GoRepository {
                importpath: "github.com/google/uuid".to_owned(),
                package_name: "github.com/test/uuid-fork".to_owned(),
            }
        );
        assert!(success[0].skip_reason.is_none());

        for remote in [
            "https://github.com/test/uuid.git#branch",
            "https://github.mycompany.com/test/uuid",
            "https://gitlab.com/test/uuid",
        ] {
            let content = format!(
                r#"
go_repository(
  name = "test_repository",
  importpath = "github.com/google/uuid",
  remote = "{remote}",
  commit = "dec09d789f3dba190787f8b4454c7d3c936fed9e"
)
"#
            );
            let deps = extract(&content);
            assert_eq!(deps.len(), 1);
            assert_eq!(
                deps[0].skip_reason,
                Some(BazelSkipReason::UnsupportedRemote)
            );
        }
    }

    // Ported: "extracts multiple types of dependencies" — lib/modules/manager/bazel/extract.spec.ts line 25
    #[test]
    fn workspace1_multiple_dependency_types() {
        let content = r#"
go_repository(name = "com_github_bitly_go-nsq", importpath = "github.com/bitly/go-nsq", tag = "v1.0.5")
go_repository(name = "com_github_google_uuid", importpath = "github.com/google/uuid", commit = "dec09d789f3dba190787f8b4454c7d3c936fed9e")
go_repository(name = "com_gopkgin_mgo_v2", importpath = "gopkg.in/mgo.v2", tag = "v2")
git_repository(name = "build_bazel_rules_nodejs", remote = "https://github.com/bazelbuild/rules_nodejs.git", tag = "0.3.1")
new_git_repository(name = "build_bazel_rules_typescript", remote = "https://github.com/bazelbuild/rules_typescript.git", tag = "0.6.1")
http_archive(name="distroless", urls=["https://github.com/GoogleContainerTools/distroless/archive/446923c3756ceeaa75888f52fcbdd48bb314fbf8.tar.gz"])
http_archive(name = "bazel_toolchains", urls = ["https://mirror.bazel.build/github.com/bazelbuild/bazel-toolchains/archive/d665ccfa3e9c90fa789671bf4ef5f7c19c5715c4.tar.gz", "https://github.com/bazelbuild/bazel-toolchains/archive/d665ccfa3e9c90fa789671bf4ef5f7c19c5715c4.tar.gz"])
http_archive(name = "rules_nodejs", urls = ["https://github.com/bazelbuild/rules_nodejs/releases/download/5.5.3/rules_nodejs-core-5.5.3.tar.gz"])
git_repository(name = "io_bazel_rules_sass", remote = "https://github.com/bazelbuild/rules_sass.git", tag = "0.0.3")
git_repository(name = "com_github_bazelbuild_buildtools", remote = "https://github.com/bazelbuild/buildtools.git", commit = "b3b620e8bcff18ed3378cd3f35ebeb7016d71f71")
http_archive(name = "io_bazel_rules_go", url = "https://github.com/bazelbuild/rules_go/releases/download/0.7.1/rules_go-0.7.1.tar.gz")
http_archive(name = "bazel_skylib", urls = ["https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/archive/0.5.0.tar.gz", "https://github.com/bazelbuild/bazel-skylib/archive/0.5.0.tar.gz"])
http_archive(name="distroless", urls=["https://github.com/GoogleContainerTools/distroless/archive/446923c3756ceeaa75888f52fcbdd48bb314fbf8.tar.gz"])
maybe(http_archive, name = "io_bazel_rules_go", url = "https://github.com/bazelbuild/rules_go/releases/download/v0.29.0/rules_go-v0.29.0.zip")
maybe(http_archive, name = "bazel_gazelle", urls = ["https://mirror.bazel.build/github.com/bazelbuild/bazel-gazelle/releases/download/v0.24.0/bazel-gazelle-v0.24.0.tar.gz", "https://github.com/bazelbuild/bazel-gazelle/releases/download/v0.24.0/bazel-gazelle-v0.24.0.tar.gz"])
maybe(go_repository, name = "com_github_pkg_errors", commit = "816c9085562cd7ee03e7f8188a1cfd942858cded", importpath = "github.com/pkg/errors")
container_pull(name = "py3_image_base", digest = "sha256:d5a717649fd93ea5b9c430d7f84e4c37ba219eb53bd73ed1d4a5a98e9edd84a7", registry = "gcr.io", repository = "distroless/python3-debian10", tag = "latest")
http_file(name="distroless", urls=["https://github.com/GoogleContainerTools/distroless/archive/446923c3756ceeaa75888f52fcbdd48bb314fbf8.tar.gz"])
"#;

        let deps = extract(content);
        assert_eq!(deps.len(), 18);
        assert_eq!(deps[0].dep_name, "com_github_bitly_go-nsq");
        assert_eq!(deps[0].current_value, "v1.0.5");
        assert_eq!(
            deps[0].source,
            BazelSource::GoRepository {
                importpath: "github.com/bitly/go-nsq".to_owned(),
                package_name: "github.com/bitly/go-nsq".to_owned(),
            }
        );
        assert_eq!(deps[3].dep_name, "build_bazel_rules_nodejs");
        assert_eq!(deps[3].current_value, "0.3.1");
        assert_eq!(
            deps[3].source,
            BazelSource::GithubReleases {
                repo: "bazelbuild/rules_nodejs".to_owned(),
            }
        );
        assert_eq!(deps[9].dep_name, "com_github_bazelbuild_buildtools");
        assert_eq!(
            deps[9].current_digest.as_deref(),
            Some("b3b620e8bcff18ed3378cd3f35ebeb7016d71f71")
        );
        assert_eq!(
            deps[15].source,
            BazelSource::GoRepository {
                importpath: "github.com/pkg/errors".to_owned(),
                package_name: "github.com/pkg/errors".to_owned(),
            }
        );
        assert_eq!(deps[16].dep_name, "py3_image_base");
        assert_eq!(
            deps[16].source,
            BazelSource::ContainerPull {
                package_name: "distroless/python3-debian10".to_owned(),
                registry_url: "gcr.io".to_owned(),
            }
        );
        assert_eq!(deps[17].dep_name, "distroless");
        assert_eq!(
            deps[17].current_digest.as_deref(),
            Some("446923c3756ceeaa75888f52fcbdd48bb314fbf8")
        );
    }

    // Ported: "extracts dependencies from *.bzl files" — lib/modules/manager/bazel/extract.spec.ts line 47
    #[test]
    fn extracts_dependencies_from_bzl_files() {
        let content = r#"
def repositories():
    if "subpar" not in native.existing_rules().keys():
        http_archive(
            name = "subpar",
            sha256 = "7ab6ab37ede82255e00c0456846a1428b20e8813f77d83bcf54ddd59ba34377a",
            strip_prefix = "subpar-0356bef3fbbabec5f0e196ecfacdeb6db62d48c0",
            urls = ["https://github.com/google/subpar/archive/0356bef3fbbabec5f0e196ecfacdeb6db62d48c0.tar.gz"],
        )

    if "bazel_skylib" not in native.existing_rules().keys():
        http_archive(
            name = "bazel_skylib",
            sha256 = "eb5c57e4c12e68c0c20bc774bfbc60a568e800d025557bc4ea022c6479acc867",
            strip_prefix = "bazel-skylib-0.6.0",
            urls = ["https://github.com/bazelbuild/bazel-skylib/archive/0.6.0.tar.gz"],
        )

    maybe(
        http_archive,
        name = "io_bazel_stardoc",
        sha256 = "c9794dcc8026a30ff67cf7cf91ebe245ca294b20b071845d12c192afe243ad72",
        urls = [
            "https://mirror.bazel.build/github.com/bazelbuild/stardoc/releases/download/0.5.0/stardoc-0.5.0.tar.gz",
            "https://github.com/bazelbuild/stardoc/releases/download/0.5.0/stardoc-0.5.0.tar.gz",
        ],
    )
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
        assert_eq!(deps[0].dep_name, "subpar");
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("0356bef3fbbabec5f0e196ecfacdeb6db62d48c0")
        );
        assert_eq!(
            deps[0].source,
            BazelSource::GithubTags {
                repo: "google/subpar".to_owned(),
            }
        );
        assert_eq!(deps[1].dep_name, "bazel_skylib");
        assert_eq!(deps[1].current_value, "0.6.0");
        assert_eq!(
            deps[1].source,
            BazelSource::GithubTags {
                repo: "bazelbuild/bazel-skylib".to_owned(),
            }
        );
        assert_eq!(deps[2].dep_name, "io_bazel_stardoc");
        assert_eq!(deps[2].current_value, "0.5.0");
        assert_eq!(
            deps[2].source,
            BazelSource::GithubReleases {
                repo: "bazelbuild/stardoc".to_owned(),
            }
        );
    }
}
