//! Puppet `Puppetfile` dependency extractor.
//!
//! Parses Puppet module declarations in `Puppetfile` format, producing
//! Puppet Forge deps and git-based deps (GitHub Tags / generic Git).
//!
//! Renovate reference:
//! - `lib/modules/manager/puppet/extract.ts`
//! - `lib/modules/manager/puppet/puppetfile-parser.ts`
//! - Pattern: `/(^|/)Puppetfile$/`
//! - Datasources: PuppetForgeDatasource, GithubTagsDatasource
//!
//! ## File format
//!
//! ```ruby
//! forge 'https://forgeapi.puppetlabs.com'
//!
//! mod 'puppetlabs/apache', '5.5.0'
//! mod 'puppetlabs/concat', '7.1.1'
//!
//! mod 'custom_git',
//!   :git => 'https://github.com/owner/custom_git',
//!   :tag => 'v1.2.3'
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// How a Puppet dep is sourced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PuppetSource {
    /// From Puppet Forge (`author/name` format).
    PuppetForge { forge_url: Option<String> },
    /// From GitHub (owner/repo).
    GitHub(String),
    /// From a generic git URL.
    Git(String),
}

/// Why a dep is skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PuppetSkipReason {
    /// `mod` line with no version or git tag.
    UnspecifiedVersion,
    /// Module has `:git =>` but no `:tag =>`.
    GitNoTag,
    /// Git URL points at github.com but does not use the `https://` scheme,
    /// so the github-tags datasource cannot be used to look it up.
    InvalidUrl,
    /// `mod` declaration has malformed positional arguments (more than two
    /// quoted strings before the symbol-keyword section).
    InvalidConfig,
}

/// A single extracted Puppet module dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PuppetDep {
    /// Module name. Forge modules use `author/name`; git-sourced use the name key.
    pub name: String,
    /// Version string (Forge) or tag (git).
    pub current_value: String,
    /// Source type.
    pub source: PuppetSource,
    /// Set when no lookup should be performed.
    pub skip_reason: Option<PuppetSkipReason>,
}

// Matches `forge 'url'` or `forge "url"`
static FORGE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^forge\s+['"]([^'"]+)['"]"#).unwrap());

// Matches `mod 'author/name', 'version'` or `mod 'author/name'` (version optional)
static MOD_START_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^\s*mod\s+['"]([^'"]+)['"]\s*(?:,\s*['"]([^'"]+)['"])?"#).unwrap()
});

// Matches `:key => 'value'` or `:key => "value"`
static SYMBOL_KV_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#":(\w+)\s*=>\s*['"]([^'"]+)['"]\s*"#).unwrap());

// Matches `mod 'X', 'Y', 'Z'` — three or more quoted positional arguments.
// Renovate flags such mod declarations as invalid-config.
static MOD_INVALID_TRIPLE_ARG: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^\s*mod\s+['"][^'"]+['"]\s*,\s*['"][^'"]+['"]\s*,\s*['"]"#).unwrap()
});

/// Extract Puppet module deps from a `Puppetfile`.
pub fn extract(content: &str) -> Vec<PuppetDep> {
    let mut deps = Vec::new();
    let mut current_forge: Option<String> = None;

    // Multi-line mod state
    let mut pending_name: Option<String> = None;
    let mut pending_version: Option<String> = None;
    let mut pending_git: Option<String> = None;
    let mut pending_tag: Option<String> = None;
    let mut pending_invalid_config = false;
    let mut continuation = false;

    let flush = |name: Option<String>,
                 version: Option<String>,
                 git: Option<String>,
                 tag: Option<String>,
                 forge_url: Option<String>,
                 invalid_config: bool,
                 deps: &mut Vec<PuppetDep>| {
        let Some(name) = name else {
            return;
        };

        // A malformed `mod` declaration short-circuits all other parsing —
        // emit a single dep with InvalidConfig and the git URL (if any) as
        // the source so callers can still report the offending line.
        if invalid_config {
            let source = match git {
                Some(url) => PuppetSource::Git(url),
                None => PuppetSource::PuppetForge { forge_url },
            };
            deps.push(PuppetDep {
                name,
                current_value: tag.or(version).unwrap_or_default(),
                source,
                skip_reason: Some(PuppetSkipReason::InvalidConfig),
            });
            return;
        }

        if let Some(git_url) = git {
            match tag {
                None => {
                    deps.push(PuppetDep {
                        name,
                        current_value: String::new(),
                        source: PuppetSource::Git(git_url),
                        skip_reason: Some(PuppetSkipReason::GitNoTag),
                    });
                }
                Some(tag_val) => {
                    // A github.com URL using a non-https scheme (e.g. plain http://
                    // or git@github.com:) cannot be looked up via the github-tags
                    // datasource — flag it as InvalidUrl so the source URL is
                    // preserved but no update lookup is attempted.
                    if git_url.starts_with("http://github.com/") {
                        deps.push(PuppetDep {
                            name,
                            current_value: tag_val,
                            source: PuppetSource::Git(git_url),
                            skip_reason: Some(PuppetSkipReason::InvalidUrl),
                        });
                        return;
                    }

                    // Reject git URLs that do not match any recognised
                    // git URL scheme (https://, http://, ssh://, git@host:).
                    if !is_recognised_git_url(&git_url) {
                        deps.push(PuppetDep {
                            name,
                            current_value: tag_val,
                            source: PuppetSource::Git(git_url),
                            skip_reason: Some(PuppetSkipReason::InvalidUrl),
                        });
                        return;
                    }

                    // Only use the GitHub datasource when the host is exactly
                    // "github.com" (not subdomains like github.com.example.com)
                    // and the scheme is https or ssh.
                    let is_github = git_url.starts_with("https://github.com/")
                        || git_url.starts_with("git@github.com:");
                    let source = if is_github {
                        let repo = git_url
                            .trim_end_matches(".git")
                            .trim_start_matches("https://github.com/")
                            .trim_start_matches("git@github.com:")
                            .to_owned();
                        PuppetSource::GitHub(repo)
                    } else {
                        PuppetSource::Git(git_url)
                    };
                    deps.push(PuppetDep {
                        name,
                        current_value: tag_val,
                        source,
                        skip_reason: None,
                    });
                }
            }
        } else {
            match version {
                None => {
                    deps.push(PuppetDep {
                        name,
                        current_value: String::new(),
                        source: PuppetSource::PuppetForge { forge_url },
                        skip_reason: Some(PuppetSkipReason::UnspecifiedVersion),
                    });
                }
                Some(ver) => {
                    deps.push(PuppetDep {
                        name,
                        current_value: ver,
                        source: PuppetSource::PuppetForge { forge_url },
                        skip_reason: None,
                    });
                }
            }
        }
    };

    for raw_line in content.lines() {
        // Strip inline Ruby-style comments
        let line = if let Some(pos) = raw_line.find(" #") {
            &raw_line[..pos]
        } else {
            raw_line
        };
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        // Detect `forge` URL changes
        if let Some(cap) = FORGE_RE.captures(trimmed) {
            flush(
                pending_name.take(),
                pending_version.take(),
                pending_git.take(),
                pending_tag.take(),
                current_forge.clone(),
                std::mem::take(&mut pending_invalid_config),
                &mut deps,
            );
            continuation = false;
            current_forge = Some(cap[1].to_owned());
            continue;
        }

        // Start of a new `mod` declaration
        if let Some(cap) = MOD_START_RE.captures(trimmed) {
            // Flush previous
            flush(
                pending_name.take(),
                pending_version.take(),
                pending_git.take(),
                pending_tag.take(),
                current_forge.clone(),
                std::mem::take(&mut pending_invalid_config),
                &mut deps,
            );
            pending_name = Some(cap[1].to_owned());
            pending_version = cap.get(2).map(|m| m.as_str().to_owned());
            pending_git = None;
            pending_tag = None;
            // Three or more quoted positional arguments — invalid config.
            pending_invalid_config = MOD_INVALID_TRIPLE_ARG.is_match(trimmed);
            // Also extract inline symbol key-value pairs on the same line (e.g. :git => '...' :tag => '...')
            for kv in SYMBOL_KV_RE.captures_iter(trimmed) {
                match &kv[1] {
                    "git" => pending_git = Some(kv[2].to_owned()),
                    "tag" => pending_tag = Some(kv[2].to_owned()),
                    _ => {}
                }
            }
            continuation = trimmed.ends_with(',');
        } else if continuation || trimmed.starts_with(':') {
            // Continuation line — extract symbol key-value pairs
            for cap in SYMBOL_KV_RE.captures_iter(trimmed) {
                let key = &cap[1];
                let val = cap[2].to_owned();
                match key {
                    "git" => pending_git = Some(val),
                    "tag" => pending_tag = Some(val),
                    _ => {}
                }
            }
            continuation = trimmed.ends_with(',');
        }
    }

    // Flush final entry
    flush(
        pending_name.take(),
        pending_version.take(),
        pending_git.take(),
        pending_tag.take(),
        current_forge,
        pending_invalid_config,
        &mut deps,
    );

    deps
}

/// Regex matching `git@host:repository` SSH git format.
///
/// Mirrors `lib/modules/manager/puppet/common.ts` `RE_REPOSITORY_GENERIC_GIT_SSH_FORMAT`.
pub static GIT_SSH_RE: std::sync::LazyLock<regex::Regex> =
    std::sync::LazyLock::new(|| regex::Regex::new(r"^git@[^:]*:(?P<repository>.+)$").unwrap());

/// Parse `git` URL to `owner/repo` format.
///
/// Mirrors `lib/modules/manager/puppet/common.ts` `parseGitOwnerRepo()`.
/// Returns `None` for unrecognised URLs.
pub fn parse_git_owner_repo(git: &str, _github_url: bool) -> Option<String> {
    if let Some(cap) = GIT_SSH_RE.captures(git) {
        let repo = cap["repository"].trim_end_matches(".git");
        return Some(repo.to_owned());
    }
    // Try as HTTP(S) URL: strip scheme and host, extract path
    let after_scheme = git
        .strip_prefix("https://")
        .or_else(|| git.strip_prefix("http://"))
        .or_else(|| git.strip_prefix("ssh://"))?;
    // Skip host (up to first `/`)
    let path = after_scheme.split_once('/')?.1;
    let path = path.trim_end_matches(".git").trim_end_matches('/');
    if path.is_empty() {
        return None;
    }
    Some(path.to_owned())
}

/// Whether `url` looks like one of the git-URL schemes Puppet's r10k
/// supports for `:git =>` references (https://, http://, ssh://, git@host:).
fn is_recognised_git_url(url: &str) -> bool {
    url.starts_with("https://")
        || url.starts_with("http://")
        || url.starts_with("ssh://")
        // `git@host:owner/repo` SCP-like form
        || url.starts_with("git@")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "extracts multiple modules from Puppetfile without a forge" — puppet/extract.spec.ts line 14
    #[test]
    fn extracts_forge_module_with_version() {
        let content = "mod 'puppetlabs/apache', '5.5.0'\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "puppetlabs/apache");
        assert_eq!(deps[0].current_value, "5.5.0");
        assert_eq!(
            deps[0].source,
            PuppetSource::PuppetForge { forge_url: None }
        );
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "extracts multiple modules from Puppetfile with multiple forges/registries" — puppet/extract.spec.ts line 47
    #[test]
    fn extracts_custom_forge() {
        let content = "forge 'https://forge.example.com'\nmod 'myorg/mymod', '1.0.0'\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].source,
            PuppetSource::PuppetForge {
                forge_url: Some("https://forge.example.com".to_owned())
            }
        );
    }

    // Ported: "extracts multiple git tag modules from Puppetfile" — puppet/extract.spec.ts line 100
    #[test]
    fn extracts_github_git_module() {
        let content = r#"
mod 'custom_mod',
  :git => 'https://github.com/owner/custom_mod',
  :tag => 'v1.2.3'
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "custom_mod");
        assert_eq!(deps[0].current_value, "v1.2.3");
        assert_eq!(
            deps[0].source,
            PuppetSource::GitHub("owner/custom_mod".to_owned())
        );
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "Git module without a tag should result in a skip reason" — puppet/extract.spec.ts line 162
    #[test]
    fn git_no_tag_skipped() {
        let content = "mod 'mymod',\n  :git => 'https://github.com/owner/repo'\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(PuppetSkipReason::GitNoTag));
    }

    // Ported: "extracts multiple modules from Puppetfile without a forge" — puppet/extract.spec.ts line 14
    #[test]
    fn multiple_modules() {
        let content = r#"
mod 'puppetlabs/apache', '5.5.0'
mod 'puppetlabs/concat', '7.1.1'
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].name, "puppetlabs/apache");
        assert_eq!(deps[1].name, "puppetlabs/concat");
    }

    #[test]
    fn module_without_version_skipped() {
        let content = "mod 'puppetlabs/stdlib'\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(PuppetSkipReason::UnspecifiedVersion)
        );
    }

    #[test]
    fn comment_lines_ignored() {
        let content = "# forge 'https://example.com'\nmod 'puppetlabs/apache', '5.5.0'\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "puppetlabs/apache");
    }

    // Ported: "returns null for empty Puppetfile" — puppet/extract.spec.ts line 10
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "Use GithubTagsDatasource only if host is exactly github.com" — puppet/extract.spec.ts line 125
    #[test]
    fn non_github_host_uses_git_tags_datasource() {
        // github.com.example.com is NOT github.com → should use GitTags
        let content = "mod 'apache', :git => 'https://github.com.example.com/puppetlabs/puppetlabs-apache', :tag => '0.9.0'";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "apache");
        assert_eq!(deps[0].current_value, "0.9.0");
        // should NOT be GitHub datasource since host is not exactly github.com
        assert!(!matches!(deps[0].source, PuppetSource::GitHub { .. }));
    }

    // Ported: "Github url without https is skipped" — puppet/extract.spec.ts line 146
    #[test]
    fn http_github_url_marked_invalid_url() {
        let content = "mod 'apache', :git => 'http://github.com/puppetlabs/puppetlabs-apache', :tag => '0.9.0'";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "apache");
        assert_eq!(deps[0].skip_reason, Some(PuppetSkipReason::InvalidUrl));
        assert!(matches!(deps[0].source, PuppetSource::Git(_)));
    }

    // Ported: "Skip reason should be overwritten by parser" — puppet/extract.spec.ts line 181
    #[test]
    fn malformed_mod_with_three_positional_args_is_invalid_config() {
        let content = "mod 'stdlib', '0.1.0', 'i create a skip reason'\n  :git => 'git@github.com:puppetlabs/puppetlabs-stdlib.git',\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "stdlib");
        assert_eq!(deps[0].skip_reason, Some(PuppetSkipReason::InvalidConfig));
        assert!(matches!(deps[0].source, PuppetSource::Git(_)));
    }

    // Ported: "GitTagsDatasource" — puppet/extract.spec.ts line 200
    //
    // Mirrors the Puppetfile.git_tag fixture — non-github git URLs
    // (gitlab.com, ssh, multi-dir paths) should produce GitTags-style
    // deps; the malformed `'hello world'` entry is flagged InvalidUrl.
    #[test]
    fn git_tags_fixture_extracts_four_valid_and_one_invalid() {
        let content = r#"
mod 'apache',
  :git => 'https://gitlab.com/example/project.git',
  :tag => '0.9.0'

mod 'stdlib',
  :git => 'git@gitlab.com:example/project_stdlib.git',
  :tag => '5.0.0'

mod 'multiple_dirs_ssh',
  :git => 'git@gitlab.com:dir1/dir2/project.git',
  :tag => '1.0.0'

mod 'multiple_dirs_https',
  :git => 'https://gitlab.com/dir1/dir2/project.git',
  :tag => '1.9.0'

mod 'invalid_url',
  :git => 'hello world',
  :tag => '0.0.0'
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 5);

        // The four valid gitlab entries — all Git source (non-github), no skip.
        for name in [
            "apache",
            "stdlib",
            "multiple_dirs_ssh",
            "multiple_dirs_https",
        ] {
            let dep = deps.iter().find(|d| d.name == name).unwrap();
            assert!(matches!(dep.source, PuppetSource::Git(_)));
            assert!(dep.skip_reason.is_none(), "{name} should not be skipped");
        }

        // `invalid_url` lacks a recognised URL scheme — InvalidUrl.
        let invalid = deps.iter().find(|d| d.name == "invalid_url").unwrap();
        assert_eq!(invalid.skip_reason, Some(PuppetSkipReason::InvalidUrl));
    }

    // Ported: "access by index" — modules/manager/puppet/common.spec.ts line 10
    #[test]
    fn puppet_git_ssh_regex_captures_repository() {
        let cap = GIT_SSH_RE
            .captures("git@gitlab.com:dir1/dir2/project.git")
            .unwrap();
        assert_eq!(&cap["repository"], "dir1/dir2/project.git");
    }

    // Ported: "access by named group" — modules/manager/puppet/common.spec.ts line 21
    #[test]
    fn puppet_git_ssh_regex_captures_named_group() {
        let cap = GIT_SSH_RE
            .captures("git@gitlab.com:dir1/dir2/project.git")
            .unwrap();
        assert_eq!(&cap["repository"], "dir1/dir2/project.git");
    }

    // Ported: "unable to parse url" — modules/manager/puppet/common.spec.ts line 34
    #[test]
    fn puppet_parse_git_owner_repo_returns_none_for_invalid() {
        assert!(parse_git_owner_repo("invalid-url-example", false).is_none());
    }

    // Ported: "parseable url" — modules/manager/puppet/common.spec.ts line 38
    #[test]
    fn puppet_parse_git_owner_repo_parses_https_url() {
        let result = parse_git_owner_repo("https://gitlab.com/example/example", false);
        assert_eq!(result.as_deref(), Some("example/example"));
    }
}
