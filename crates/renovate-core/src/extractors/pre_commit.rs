//! Pre-commit `.pre-commit-config.yaml` dependency extractor.
//!
//! Scans the `repos:` list in a pre-commit config and extracts each
//! `repo` / `rev` pair as a dependency whose version is looked up via
//! the GitHub or GitLab tags datasource.
//!
//! Renovate reference:
//! - `lib/modules/manager/pre-commit/extract.ts`
//! - `lib/modules/manager/pre-commit/index.ts` — pattern `/(^|/)\\.pre-commit-config\\.ya?ml$/`
//!
//! ## Format
//!
//! ```yaml
//! repos:
//! - repo: https://github.com/pre-commit/pre-commit-hooks
//!   rev: v4.5.0
//!   hooks:
//!   - id: trailing-whitespace
//! - repo: https://github.com/psf/black
//!   rev: 23.7.0
//!   hooks:
//!   - id: black
//! ```
//!
//! ## Git host detection
//!
//! - `github.com` → GitHub tags datasource
//! - `gitlab.com` (or other with "gitlab" in hostname) → GitLab tags datasource
//! - `local` / `meta` → skipped

/// Which git hosting provider serves this hook's repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitHost {
    GitHub,
    GitLab,
}

/// Why a pre-commit dependency is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreCommitSkipReason {
    /// Repo is a local path hook (`:local`).
    LocalHook,
    /// Repo is a built-in meta hook (`meta`).
    MetaHook,
    /// Repo URL could not be parsed into a known hostname + owner/repo form.
    InvalidUrl,
    /// Hostname is not GitHub or GitLab.
    UnknownRegistry,
}

/// A single extracted pre-commit hook dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreCommitDep {
    /// `owner/repo` path extracted from the repo URL.
    pub dep_name: String,
    /// The `rev:` value (e.g. `"v4.5.0"`).
    pub current_value: String,
    /// Hosting provider, used to select the right datasource.
    pub git_host: Option<GitHost>,
    /// Set when no tag lookup should be performed.
    pub skip_reason: Option<PreCommitSkipReason>,
}

/// Extract pre-commit hook dependencies from a `.pre-commit-config.yaml`.
pub fn extract(content: &str) -> Vec<PreCommitDep> {
    let mut out = Vec::new();

    // Whether we're inside the `repos:` list.
    let mut in_repos = false;
    // The indent level of the `- repo:` list items (typically 0).
    let mut entry_indent: Option<usize> = None;
    // Accumulate the current entry's fields.
    let mut current_repo: Option<String> = None;
    let mut current_rev: Option<String> = None;

    for raw in content.lines() {
        let line = strip_comment(raw);
        if line.trim().is_empty() {
            continue;
        }

        let indent = leading_spaces(line);
        let trimmed = line.trim_start();

        // Top-level (indent 0): look for `repos:` key or other top-level keys.
        if indent == 0 {
            if let Some(val) = strip_key(trimmed, "repos") {
                // `repos:` starts the repos list.
                flush(&mut current_repo, &mut current_rev, &mut out);
                in_repos = val.trim().is_empty();
                entry_indent = None;
                continue;
            }
            // Any other top-level key ends repos section.
            if !trimmed.starts_with('-') {
                flush(&mut current_repo, &mut current_rev, &mut out);
                in_repos = false;
                entry_indent = None;
                continue;
            }
        }

        if !in_repos {
            continue;
        }

        // Detect list items: lines starting with `- `.
        if let Some(rest) = trimmed.strip_prefix("- ") {
            // Determine if this is an entry-level item or a nested item.
            let this_indent = indent;
            match entry_indent {
                None => {
                    // First `-` we see → this is the entry indent.
                    entry_indent = Some(this_indent);
                }
                Some(ei) if this_indent > ei => {
                    // Nested item (e.g. hooks list) — ignore.
                    continue;
                }
                Some(ei) if this_indent < ei => {
                    // Less indented than entry level — end of repos section.
                    flush(&mut current_repo, &mut current_rev, &mut out);
                    in_repos = false;
                    continue;
                }
                _ => {
                    // Same indent as entry level → new repo entry.
                    flush(&mut current_repo, &mut current_rev, &mut out);
                }
            }

            // Parse inline fields on the `- key: value` line.
            if let Some(url) = strip_key(rest, "repo") {
                current_repo = Some(url.trim().trim_matches('\'').trim_matches('"').to_owned());
            } else if let Some(rev) = strip_key(rest, "rev") {
                current_rev = Some(rev.trim().trim_matches('\'').trim_matches('"').to_owned());
            }
            continue;
        }

        // Non-list continuation line at the current entry level or deeper.
        if let Some(ei) = entry_indent {
            if indent <= ei {
                // At or shallower than entry indent but not a list item — end section.
                flush(&mut current_repo, &mut current_rev, &mut out);
                in_repos = false;
                continue;
            }
            // Deeper than entry indent → continuation line for current entry.
            if let Some(url) = strip_key(trimmed, "repo") {
                current_repo = Some(url.trim().trim_matches('\'').trim_matches('"').to_owned());
            } else if let Some(rev) = strip_key(trimmed, "rev") {
                current_rev = Some(rev.trim().trim_matches('\'').trim_matches('"').to_owned());
            }
        }
    }

    flush(&mut current_repo, &mut current_rev, &mut out);
    out
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn flush(repo: &mut Option<String>, rev: &mut Option<String>, out: &mut Vec<PreCommitDep>) {
    match (repo.take(), rev.take()) {
        (Some(repo_url), Some(rev_tag)) => {
            out.push(parse_dep(repo_url, rev_tag));
        }
        (Some(repo_url), None) => {
            // `local` and `meta` repos have no rev — still emit them so they appear as skipped.
            out.push(parse_dep(repo_url, String::new()));
        }
        _ => {}
    }
}

fn parse_dep(repo_url: String, rev: String) -> PreCommitDep {
    match repo_url.as_str() {
        "local" => {
            return PreCommitDep {
                dep_name: repo_url,
                current_value: rev,
                git_host: None,
                skip_reason: Some(PreCommitSkipReason::LocalHook),
            };
        }
        "meta" => {
            return PreCommitDep {
                dep_name: repo_url,
                current_value: rev,
                git_host: None,
                skip_reason: Some(PreCommitSkipReason::MetaHook),
            };
        }
        _ => {}
    }

    // Strip scheme prefix.
    let url = repo_url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("git://")
        .trim_start_matches("git@");

    // Extract hostname and path.
    let (hostname, path) = if let Some(pos) = url.find('/') {
        (&url[..pos], &url[pos + 1..])
    } else if let Some(pos) = url.find(':') {
        (&url[..pos], &url[pos + 1..])
    } else {
        return PreCommitDep {
            dep_name: repo_url,
            current_value: rev,
            git_host: None,
            skip_reason: Some(PreCommitSkipReason::InvalidUrl),
        };
    };

    if path.is_empty() {
        return PreCommitDep {
            dep_name: repo_url,
            current_value: rev,
            git_host: None,
            skip_reason: Some(PreCommitSkipReason::InvalidUrl),
        };
    }

    let dep_name = path.trim_end_matches(".git").to_owned();

    let git_host = if hostname == "github.com" {
        Some(GitHost::GitHub)
    } else if hostname.contains("gitlab") {
        Some(GitHost::GitLab)
    } else {
        None
    };

    let skip_reason = if git_host.is_none() {
        Some(PreCommitSkipReason::UnknownRegistry)
    } else {
        None
    };

    PreCommitDep {
        dep_name,
        current_value: rev,
        git_host,
        skip_reason,
    }
}

fn leading_spaces(s: &str) -> usize {
    s.len() - s.trim_start_matches([' ', '\t']).len()
}

fn strip_comment(line: &str) -> &str {
    for (i, c) in line.char_indices() {
        if c == '#' && i > 0 {
            let prev = line.as_bytes().get(i.wrapping_sub(1)).copied();
            if prev == Some(b' ') || prev == Some(b'\t') {
                return &line[..i];
            }
        }
    }
    line
}

fn strip_key<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}:");
    line.strip_prefix(prefix.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "
repos:
- repo: https://github.com/pre-commit/pre-commit-hooks
  rev: v4.5.0
  hooks:
  - id: trailing-whitespace
  - id: end-of-file-fixer
- repo: https://github.com/psf/black
  rev: '23.7.0'
  hooks:
  - id: black
- repo: https://gitlab.com/pycqa/flake8
  rev: '6.0.0'
  hooks:
  - id: flake8
- repo: local
  hooks:
  - id: my-hook
    name: my-hook
    language: system
    entry: ./scripts/check.sh
- repo: meta
  hooks:
  - id: check-hooks-apply
";

    // Ported: "extracts from complex config file correctly" — pre-commit/extract.spec.ts line 105
    #[test]
    fn extracts_github_hooks() {
        let deps = extract(SAMPLE);
        let gh: Vec<_> = deps
            .iter()
            .filter(|d| d.git_host == Some(GitHost::GitHub))
            .collect();
        assert_eq!(gh.len(), 2);

        let hooks = gh
            .iter()
            .find(|d| d.dep_name == "pre-commit/pre-commit-hooks")
            .unwrap();
        assert_eq!(hooks.current_value, "v4.5.0");
        assert!(hooks.skip_reason.is_none());

        let black = gh.iter().find(|d| d.dep_name == "psf/black").unwrap();
        assert_eq!(black.current_value, "23.7.0");
    }

    // Ported: "extracts from complex config file correctly" — pre-commit/extract.spec.ts line 105
    #[test]
    fn extracts_gitlab_hooks() {
        let deps = extract(SAMPLE);
        let gl: Vec<_> = deps
            .iter()
            .filter(|d| d.git_host == Some(GitHost::GitLab))
            .collect();
        assert_eq!(gl.len(), 1);
        assert_eq!(gl[0].dep_name, "pycqa/flake8");
        assert_eq!(gl[0].current_value, "6.0.0");
    }

    // Ported: "extracts from complex config file correctly" — pre-commit/extract.spec.ts line 105
    #[test]
    fn skips_local_hooks() {
        let deps = extract(SAMPLE);
        let local = deps
            .iter()
            .find(|d| d.skip_reason == Some(PreCommitSkipReason::LocalHook));
        assert!(local.is_some());
    }

    // Ported: "extracts from complex config file correctly" — pre-commit/extract.spec.ts line 105
    #[test]
    fn skips_meta_hooks() {
        let deps = extract(SAMPLE);
        let meta = deps
            .iter()
            .find(|d| d.skip_reason == Some(PreCommitSkipReason::MetaHook));
        assert!(meta.is_some());
    }

    // Ported: "extracts from complex config file correctly" — pre-commit/extract.spec.ts line 105
    #[test]
    fn total_dep_count() {
        let deps = extract(SAMPLE);
        // pre-commit-hooks, black, flake8 (gl), local, meta = 5
        assert_eq!(deps.len(), 5);
    }

    // Ported: "returns null for empty yaml file content" — pre-commit/extract.spec.ts line 57
    #[test]
    fn empty_content_returns_no_deps() {
        assert!(extract("").is_empty());
    }

    // Ported: "returns null for no repos" — pre-commit/extract.spec.ts line 68
    #[test]
    fn no_repos_section_returns_no_deps() {
        assert!(extract("default_language_version:\n  python: python3\n").is_empty());
    }

    #[test]
    fn git_suffix_stripped() {
        let content = "repos:\n- repo: https://github.com/owner/myhook.git\n  rev: v1.0\n  hooks:\n  - id: hook\n";
        let deps = extract(content);
        assert_eq!(deps[0].dep_name, "owner/myhook");
    }

    // Ported: "can handle invalid private git repos" — pre-commit/extract.spec.ts line 183
    #[test]
    fn unknown_registry_gets_skip_reason() {
        let content =
            "repos:\n- repo: https://bitbucket.org/owner/repo\n  rev: v1.0\n  hooks:\n  - id: x\n";
        let deps = extract(content);
        assert_eq!(
            deps[0].skip_reason,
            Some(PreCommitSkipReason::UnknownRegistry)
        );
    }

    // Ported: "returns null for invalid yaml file content" — pre-commit/extract.spec.ts line 52
    #[test]
    fn invalid_yaml_returns_empty() {
        assert!(extract("nothing here: [").is_empty());
    }

    // Ported: "returns null for empty repos" — pre-commit/extract.spec.ts line 73
    #[test]
    fn empty_repos_list_returns_empty() {
        assert!(extract("repos: []\n").is_empty());
    }

    // Ported: "returns null for invalid repo" — pre-commit/extract.spec.ts line 78
    #[test]
    fn repo_entry_without_repo_key_returns_empty() {
        let content = "repos:\n- hooks:\n  - id: some-hook\n";
        assert!(extract(content).is_empty());
    }
}
