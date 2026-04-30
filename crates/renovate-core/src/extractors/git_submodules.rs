//! `.gitmodules` git submodule dependency extractor.
//!
//! Parses Git's INI-like config format to extract submodule name, path, URL,
//! and optional branch.  The current commit digest (`currentDigest`) is not
//! available from static file content — it requires reading the git tree
//! (a future platform-API slice).
//!
//! Renovate reference:
//! - `lib/modules/manager/git-submodules/extract.ts`
//! - `lib/modules/manager/git-submodules/index.ts` — datasource: `git-refs`,
//!   disabled by default
//! - Pattern: `(^|/)\.gitmodules$`
//!
//! ## File format
//!
//! ```ini
//! [submodule "libs/lib1"]
//!     path = libs/lib1
//!     url  = https://github.com/org/lib1.git
//!     branch = main
//! ```
//!
//! Supported URL forms:
//! - HTTPS: `https://github.com/org/repo.git` (passthrough)
//! - SSH:   `git@github.com:org/repo` → `https://github.com/org/repo`
//! - Azure DevOps with user prefix: `user@dev.azure.com/...` → strip user
//! - Relative paths (e.g. `../../org/repo`) — passed through as-is;
//!   cannot be resolved without knowledge of the origin remote URL.

use std::sync::LazyLock;

use regex::Regex;

/// Header line: `[submodule "name"]`
static SECTION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\[submodule\s+"([^"]+)"\]"#).unwrap());

/// Key-value line: `  key = value`
static KV_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s+(\w+)\s*=\s*(.+)$").unwrap());

/// SSH URL: `git@host:org/repo` or `git@host:org/repo.git`
static SSH_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^git@([^:]+):(.+?)(?:\.git)?$").unwrap());

/// Azure DevOps user prefix: `user@dev.azure.com/...`
static AZURE_USER_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[^@]+@(dev\.azure\.com/.+)$").unwrap());

/// A single git submodule dependency extracted from `.gitmodules`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitSubmoduleDep {
    /// Human-visible name from the submodule section header.
    pub name: String,
    /// Relative path within the repository (from the `path` key).
    pub path: String,
    /// Normalized HTTP(S) URL of the submodule remote.
    /// SSH and relative URLs are converted where possible.
    pub url: String,
    /// Configured branch, if any.
    /// `Some(".")` is normalized to `None` (means "current branch").
    pub branch: Option<String>,
}

/// Convert a submodule URL to an HTTP(S) URL suitable as `packageName`.
///
/// - SSH `git@host:org/repo` → `https://host/org/repo`
/// - Azure DevOps `user@dev.azure.com/...` → `https://dev.azure.com/...`
/// - Trailing `.git` is stripped.
/// - Relative URLs are returned unchanged.
fn normalize_url(raw: &str) -> String {
    let trimmed = raw.trim();

    // Azure DevOps user prefix.
    if let Some(cap) = AZURE_USER_RE.captures(trimmed) {
        return format!("https://{}", &cap[1]);
    }

    // SSH URL.
    if let Some(cap) = SSH_RE.captures(trimmed) {
        return format!("https://{}/{}", &cap[1], &cap[2]);
    }

    // HTTPS — strip trailing .git.
    if trimmed.starts_with("https://") || trimmed.starts_with("http://") {
        return trimmed.trim_end_matches(".git").to_owned();
    }

    // Relative path or unknown form — pass through.
    trimmed.to_owned()
}

/// Parse a `.gitmodules` file and return all submodule deps.
///
/// Returns an empty `Vec` when the content has no submodule sections.
pub fn extract(content: &str) -> Vec<GitSubmoduleDep> {
    let mut deps: Vec<GitSubmoduleDep> = Vec::new();
    let mut current_name: Option<String> = None;
    let mut current_path: Option<String> = None;
    let mut current_url: Option<String> = None;
    let mut current_branch: Option<String> = None;

    let flush = |name: Option<String>,
                 path: Option<String>,
                 url: Option<String>,
                 branch: Option<String>,
                 deps: &mut Vec<GitSubmoduleDep>| {
        if let (Some(name), Some(path), Some(url)) = (name, path, url) {
            let normalized_url = normalize_url(&url);
            deps.push(GitSubmoduleDep {
                name,
                path,
                url: normalized_url,
                branch,
            });
        }
    };

    for line in content.lines() {
        if let Some(cap) = SECTION_RE.captures(line.trim()) {
            // Flush the previous section.
            flush(
                current_name.take(),
                current_path.take(),
                current_url.take(),
                current_branch.take(),
                &mut deps,
            );
            current_name = Some(cap[1].to_owned());
        } else if let Some(cap) = KV_RE.captures(line) {
            match cap[1].to_ascii_lowercase().as_str() {
                "path" => current_path = Some(cap[2].trim().to_owned()),
                "url" => current_url = Some(cap[2].trim().to_owned()),
                "branch" => {
                    let b = cap[2].trim();
                    // `branch = .` means "current branch" — treat as absent.
                    if b != "." {
                        current_branch = Some(b.to_owned());
                    }
                }
                _ => {}
            }
        }
    }

    // Flush the last section.
    flush(
        current_name.take(),
        current_path.take(),
        current_url.take(),
        current_branch.take(),
        &mut deps,
    );

    deps
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "empty submodule returns null" — git-submodules/extract.spec.ts line 48
    #[test]
    fn empty_content_returns_no_deps() {
        assert!(extract("").is_empty());
    }

    // Ported: "currentValue is unset when no branch is specified" — git-submodules/extract.spec.ts line 52
    #[test]
    fn single_submodule_no_branch() {
        let content = r#"
[submodule "PowerShell-Docs"]
	path = PowerShell-Docs
	url = git@github.com:PowerShell/PowerShell-Docs
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "PowerShell-Docs");
        assert_eq!(deps[0].path, "PowerShell-Docs");
        assert_eq!(deps[0].url, "https://github.com/PowerShell/PowerShell-Docs");
        assert_eq!(deps[0].branch, None);
    }

    // Ported: "given branch is used when branch is specified" — git-submodules/extract.spec.ts line 58
    #[test]
    fn single_submodule_with_branch() {
        let content = r#"
[submodule "PowerShell-Docs"]
	path = PowerShell-Docs
	url = git@github.com:PowerShell/PowerShell-Docs
	branch = staging
"#;
        let deps = extract(content);
        assert_eq!(deps[0].branch.as_deref(), Some("staging"));
    }

    // Ported: "fallback to current branch if special value is detected" — git-submodules/extract.spec.ts line 89
    #[test]
    fn branch_dot_normalized_to_none() {
        let content = r#"
[submodule "PowerShell-Docs"]
	path = PowerShell-Docs
	url = git@github.com:PowerShell/PowerShell-Docs
	branch = .
"#;
        let deps = extract(content);
        assert_eq!(deps[0].branch, None);
    }

    // Ported: "currentValue is unset when no branch is specified" — git-submodules/extract.spec.ts line 52
    #[test]
    fn multiple_submodules() {
        let content = r#"
[submodule "renovate"]
	path = deps/renovate
	url = https://github.com/renovatebot/renovate.git
[submodule "renovate-pro"]
	path = deps/renovate-pro
	url = https://github.com/renovatebot/pro.git
[submodule "renovate-config"]
	path = deps/renovate-config
	url = git@github.com:renovatebot/renovate-config.git
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
        assert_eq!(deps[0].name, "renovate");
        assert_eq!(deps[0].path, "deps/renovate");
        assert_eq!(deps[0].url, "https://github.com/renovatebot/renovate");
        assert_eq!(deps[1].url, "https://github.com/renovatebot/pro");
        assert_eq!(
            deps[2].url,
            "https://github.com/renovatebot/renovate-config"
        );
    }

    // Ported: "submodule packageName is constructed from relative path" — git-submodules/extract.spec.ts line 64
    #[test]
    fn https_url_strips_git_suffix() {
        let content = r#"
[submodule "foo"]
	path = foo
	url = https://github.com/org/repo.git
"#;
        let deps = extract(content);
        assert_eq!(deps[0].url, "https://github.com/org/repo");
    }

    // Ported: "submodule packageName is constructed from relative path" — git-submodules/extract.spec.ts line 64
    #[test]
    fn https_url_without_git_suffix_passthrough() {
        let content = r#"
[submodule "foo"]
	path = foo
	url = https://github.com/org/repo
"#;
        let deps = extract(content);
        assert_eq!(deps[0].url, "https://github.com/org/repo");
    }

    // Ported: "submodule packageName is constructed from relative path" — git-submodules/extract.spec.ts line 64
    #[test]
    fn azure_devops_user_prefix_stripped() {
        let content = r#"
[submodule "some-azure"]
  path = some-azure
  url = https://organization@dev.azure.com/organization/project/_git/repo
"#;
        let deps = extract(content);
        assert_eq!(
            deps[0].url,
            "https://dev.azure.com/organization/project/_git/repo"
        );
    }

    // Ported: "submodule packageName is constructed from relative path" — git-submodules/extract.spec.ts line 64
    #[test]
    fn relative_url_passthrough() {
        let content = r#"
[submodule "PowerShell-Docs"]
	path = PowerShell-Docs
	url = ../../PowerShell/PowerShell-Docs
"#;
        let deps = extract(content);
        assert_eq!(deps[0].url, "../../PowerShell/PowerShell-Docs");
    }

    // Ported: "given semver version is extracted from branch and versioning is set to semver" — git-submodules/extract.spec.ts line 127
    #[test]
    fn semver_and_non_semver_branches() {
        let content = r#"
[submodule "renovate1"]
	path = deps/renovate1
	url = https://github.com/renovatebot/renovate.git
	branch = v0.0.1
[submodule "renovate2"]
	path = deps/renovate2
	url = https://github.com/renovatebot/renovate.git
	branch = 0.0.1
[submodule "renovate3"]
	path = deps/renovate3
	url = https://github.com/renovatebot/renovate.git
	branch = not-a-semver
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
        assert_eq!(deps[0].branch.as_deref(), Some("v0.0.1"));
        assert_eq!(deps[1].branch.as_deref(), Some("0.0.1"));
        assert_eq!(deps[2].branch.as_deref(), Some("not-a-semver"));
    }

    // Ported: "submodule packageName is constructed from relative path" — git-submodules/extract.spec.ts line 64
    #[test]
    fn gitlab_url() {
        let content = r#"
[submodule "some-gitlab"]
	path = some-gitlab
	url = https://gitlab.com/some/repo.git
"#;
        let deps = extract(content);
        assert_eq!(deps[0].url, "https://gitlab.com/some/repo");
    }

    // Ported: "when using SSH clone URL" — git-submodules/extract.spec.ts line 73
    #[test]
    fn ssh_clone_url_converted_to_https_for_source_url() {
        // .gitmodules.3: git@github.com:PowerShell/PowerShell-Docs (no .git suffix)
        let content = r#"[submodule "PowerShell-Docs"]
	path = PowerShell-Docs
	url = git@github.com:PowerShell/PowerShell-Docs
	branch = staging
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].url, "https://github.com/PowerShell/PowerShell-Docs");
        assert_eq!(deps[0].branch.as_deref(), Some("staging"));
    }
}
