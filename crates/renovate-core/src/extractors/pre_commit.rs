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
    /// The `rev:` value (e.g. `"v4.5.0"`), or the frozen version for digest revs.
    pub current_value: String,
    /// SHA digest when the rev line uses `sha # frozen: version` format.
    pub current_digest: Option<String>,
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
    // (rev_value, digest): digest is Some only for `sha # frozen: version` revs.
    let mut current_rev: Option<(String, Option<String>)> = None;

    for raw in content.lines() {
        // For rev: lines, we DON'T strip comments so we can detect `sha # frozen: version`.
        // For all other lines, strip comments first.
        let has_rev_key = raw.trim_start().starts_with("rev:")
            || raw
                .trim_start()
                .strip_prefix("- ")
                .unwrap_or("")
                .starts_with("rev:");
        let line = if has_rev_key { raw } else { strip_comment(raw) };

        if line.trim().is_empty() {
            continue;
        }

        let indent = leading_spaces(line);
        let trimmed = line.trim_start();

        // Top-level (indent 0): look for `repos:` key or other top-level keys.
        if indent == 0 {
            if let Some(val) = strip_key(trimmed, "repos") {
                flush(&mut current_repo, &mut current_rev, &mut out);
                in_repos = val.trim().is_empty();
                entry_indent = None;
                continue;
            }
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
            let this_indent = indent;
            match entry_indent {
                None => {
                    entry_indent = Some(this_indent);
                }
                Some(ei) if this_indent > ei => {
                    continue;
                }
                Some(ei) if this_indent < ei => {
                    flush(&mut current_repo, &mut current_rev, &mut out);
                    in_repos = false;
                    continue;
                }
                _ => {
                    flush(&mut current_repo, &mut current_rev, &mut out);
                }
            }

            if let Some(url) = strip_key(rest, "repo") {
                let stripped = strip_comment(url);
                current_repo = Some(
                    stripped
                        .trim()
                        .trim_matches('\'')
                        .trim_matches('"')
                        .to_owned(),
                );
            } else if let Some(rev) = strip_key(rest, "rev") {
                current_rev = Some(parse_rev(rev));
            }
            continue;
        }

        // Non-list continuation line at the current entry level or deeper.
        if let Some(ei) = entry_indent {
            if indent <= ei {
                flush(&mut current_repo, &mut current_rev, &mut out);
                in_repos = false;
                continue;
            }
            if let Some(url) = strip_key(trimmed, "repo") {
                let stripped = strip_comment(url);
                current_repo = Some(
                    stripped
                        .trim()
                        .trim_matches('\'')
                        .trim_matches('"')
                        .to_owned(),
                );
            } else if let Some(rev) = strip_key(trimmed, "rev") {
                current_rev = Some(parse_rev(rev));
            }
        }
    }

    flush(&mut current_repo, &mut current_rev, &mut out);
    out
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Parse a rev value, detecting the `sha # frozen: version` pattern.
/// Returns `(value, digest)` where digest is Some for frozen-pinned revs.
fn parse_rev(raw_rev: &str) -> (String, Option<String>) {
    // Look for `sha # frozen: version` pattern
    let trimmed = raw_rev.trim();
    if let Some(idx) = trimmed.find(" # frozen:") {
        let digest_part = trimmed[..idx].trim().trim_matches('"').trim_matches('\'');
        let version_part = trimmed[idx + " # frozen:".len()..]
            .trim()
            .trim_matches('"')
            .trim_matches('\'');
        if !digest_part.is_empty() && !version_part.is_empty() {
            return (version_part.to_owned(), Some(digest_part.to_owned()));
        }
    }
    // Normal rev value
    let v = trimmed.trim_matches('"').trim_matches('\'');
    // Strip any trailing inline comment
    let v = strip_comment(v).trim();
    (v.to_owned(), None)
}

fn flush(
    repo: &mut Option<String>,
    rev: &mut Option<(String, Option<String>)>,
    out: &mut Vec<PreCommitDep>,
) {
    match (repo.take(), rev.take()) {
        (Some(repo_url), Some((rev_tag, digest))) => {
            out.push(parse_dep(repo_url, rev_tag, digest));
        }
        (Some(repo_url), None) => {
            out.push(parse_dep(repo_url, String::new(), None));
        }
        _ => {}
    }
}

fn parse_dep(repo_url: String, rev: String, digest: Option<String>) -> PreCommitDep {
    match repo_url.as_str() {
        "local" => {
            return PreCommitDep {
                dep_name: repo_url,
                current_value: rev,
                current_digest: digest,
                git_host: None,
                skip_reason: Some(PreCommitSkipReason::LocalHook),
            };
        }
        "meta" => {
            return PreCommitDep {
                dep_name: repo_url,
                current_value: rev,
                current_digest: digest,
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
            current_digest: digest,
            git_host: None,
            skip_reason: Some(PreCommitSkipReason::InvalidUrl),
        };
    };

    if path.is_empty() {
        return PreCommitDep {
            dep_name: repo_url,
            current_value: rev,
            current_digest: digest,
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
        current_digest: digest,
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

    // Ported: "extracts from values.yaml correctly with same structure as \"pre-commit sample-config\"" — pre-commit/extract.spec.ts line 83
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

    // Ported: "returns null for no file content" — pre-commit/extract.spec.ts line 62
    #[test]
    fn null_content_returns_empty() {
        // TypeScript passes null; Rust equivalent is empty string.
        assert!(extract("").is_empty());
    }

    // Ported: "can handle pinned repo versions" — pre-commit/extract.spec.ts line 220
    #[test]
    fn frozen_digest_rev_extracts_version_and_digest() {
        let content = r#"failfast: true
repos:
  - repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v4.4.0
    hooks:
      - id: check-yaml

  - repo: https://github.com/pre-commit/mirrors-prettier
    rev: 6fd1ced85fc139abd7f5ab4f3d78dab37592cd5e # frozen: v3.0.0-alpha.9-for-vscode
    hooks:
      - id: prettier

  - repo: https://github.com/crate-ci/typos
    rev: 20b36ca07fa1bfe124912287ac8502cf12f140e6  # frozen: v1.14.12
    hooks:
      - id: typos

  - repo: https://github.com/python-jsonschema/check-jsonschema
    rev: a00caac4f0cec045f7f67d222c3fcd0744285c51 # frozen: 0.23.1
    hooks:
      - id: check-renovate
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 4);

        // First dep: normal tag
        assert_eq!(deps[0].dep_name, "pre-commit/pre-commit-hooks");
        assert_eq!(deps[0].current_value, "v4.4.0");
        assert!(deps[0].current_digest.is_none());

        // Second dep: frozen digest
        assert_eq!(deps[1].dep_name, "pre-commit/mirrors-prettier");
        assert_eq!(deps[1].current_value, "v3.0.0-alpha.9-for-vscode");
        assert_eq!(
            deps[1].current_digest.as_deref(),
            Some("6fd1ced85fc139abd7f5ab4f3d78dab37592cd5e")
        );

        // Third dep: frozen with extra whitespace before comment
        assert_eq!(deps[2].dep_name, "crate-ci/typos");
        assert_eq!(deps[2].current_value, "v1.14.12");
        assert_eq!(
            deps[2].current_digest.as_deref(),
            Some("20b36ca07fa1bfe124912287ac8502cf12f140e6")
        );

        // Fourth dep: frozen
        assert_eq!(deps[3].dep_name, "python-jsonschema/check-jsonschema");
        assert_eq!(deps[3].current_value, "0.23.1");
        assert!(deps[3].current_digest.is_some());
    }
}
