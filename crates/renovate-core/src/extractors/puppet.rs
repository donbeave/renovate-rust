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

/// Extract Puppet module deps from a `Puppetfile`.
pub fn extract(content: &str) -> Vec<PuppetDep> {
    let mut deps = Vec::new();
    let mut current_forge: Option<String> = None;

    // Multi-line mod state
    let mut pending_name: Option<String> = None;
    let mut pending_version: Option<String> = None;
    let mut pending_git: Option<String> = None;
    let mut pending_tag: Option<String> = None;
    let mut continuation = false;

    let flush = |name: Option<String>,
                 version: Option<String>,
                 git: Option<String>,
                 tag: Option<String>,
                 forge_url: Option<String>,
                 deps: &mut Vec<PuppetDep>| {
        let Some(name) = name else {
            return;
        };

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
                    // Only use GitHub datasource when the host is exactly "github.com"
                    // (not subdomains like github.com.example.com).
                    let is_github = git_url.starts_with("https://github.com/")
                        || git_url.starts_with("http://github.com/")
                        || git_url.starts_with("git@github.com:");
                    let source = if is_github {
                        let repo = git_url
                            .trim_end_matches(".git")
                            .trim_start_matches("https://github.com/")
                            .trim_start_matches("http://github.com/")
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
                &mut deps,
            );
            pending_name = Some(cap[1].to_owned());
            pending_version = cap.get(2).map(|m| m.as_str().to_owned());
            pending_git = None;
            pending_tag = None;
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
        &mut deps,
    );

    deps
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
}
