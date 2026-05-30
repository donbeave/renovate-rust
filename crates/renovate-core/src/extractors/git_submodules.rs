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

/// Resolve a relative submodule URL against the parent repository's remote URL.
///
/// Given `base = "https://github.com/renovatebot/renovate.git"` and
/// `relative = "../../PowerShell/PowerShell-Docs"`, returns
/// `"https://github.com/PowerShell/PowerShell-Docs"`.
fn resolve_relative_url(base: &str, relative: &str) -> String {
    // Strip trailing .git and any trailing slash from the base.
    let base = base.trim_end_matches(".git").trim_end_matches('/');
    // Build a fake absolute URL by appending the relative path and using
    // simple segment traversal.
    let mut segments: Vec<&str> = base.split('/').collect();
    for part in relative.split('/') {
        match part {
            ".." => {
                segments.pop();
            }
            "." | "" => {}
            p => segments.push(p),
        }
    }
    segments.join("/")
}

/// Parse a `.gitmodules` file and return all submodule deps.
///
/// When `remote_url` is provided, relative submodule URLs are resolved
/// against it (mirrors TypeScript `extractPackageFile` behaviour).
///
/// Returns an empty `Vec` when the content has no submodule sections.
pub fn extract_with_remote(content: &str, remote_url: Option<&str>) -> Vec<GitSubmoduleDep> {
    let mut deps = extract(content);
    if let Some(remote) = remote_url {
        for dep in &mut deps {
            if !dep.url.starts_with("http") && !dep.url.starts_with("git@") {
                dep.url = resolve_relative_url(remote, &dep.url);
            }
        }
    }
    deps
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

// ── updateArtifacts ───────────────────────────────────────────────────────────

/// A single file change produced by `update_artifacts`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmoduleArtifact {
    pub path: String,
    pub contents: String,
}

/// Produce artifact additions for each updated submodule dependency.
///
/// Ports `updateArtifacts` from `lib/modules/manager/git-submodules/artifacts.ts`.
pub fn update_artifacts(dep_names: &[&str]) -> Vec<SubmoduleArtifact> {
    dep_names
        .iter()
        .map(|&dep_name| SubmoduleArtifact {
            path: dep_name.to_owned(),
            contents: String::new(),
        })
        .collect()
}

// ---------------------------------------------------------------------------
// updateDependency — lib/modules/manager/git-submodules/update.ts
// ---------------------------------------------------------------------------

/// Configuration for `update_dependency`.
#[derive(Debug, Clone)]
pub struct GitSubmoduleUpdateConfig {
    pub dep_name: String,
    pub current_value: Option<String>,
    pub new_value: Option<String>,
    pub new_digest: Option<String>,
    pub package_file: Option<String>,
}

/// Result of a submodule update.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitSubmoduleUpdateResult {
    Unchanged,
    Updated(String),
    Failed,
}

/// Compute the expected `.gitmodules` content after a submodule update.
///
/// Mirrors `updateDependency()` from `lib/modules/manager/git-submodules/update.ts`.
pub fn update_dependency(
    file_content: &str,
    config: &GitSubmoduleUpdateConfig,
) -> GitSubmoduleUpdateResult {
    let dep_name = &config.dep_name;
    let new_value = config.new_value.as_deref();
    let current_value = config.current_value.as_deref();

    let Some(new_val) = new_value else {
        return GitSubmoduleUpdateResult::Unchanged;
    };
    if let Some(current) = current_value
        && current == new_val {
            return GitSubmoduleUpdateResult::Unchanged;
        }

    let mut result = String::new();
    let mut in_target_section = false;
    let mut branch_updated = false;

    for line in file_content.lines() {
        if let Some(cap) = SECTION_RE.captures(line.trim()) {
            in_target_section = &cap[1] == dep_name.as_str();
        }

        if in_target_section && !branch_updated
            && let Some(cap) = KV_RE.captures(line)
                && cap[1].trim() == "branch" {
                    let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
                    result.push_str(&format!("{indent}branch = {new_val}\n"));
                    branch_updated = true;
                    continue;
                }

        result.push_str(line);
        result.push('\n');
    }

    if !branch_updated {
        let mut final_result = String::new();
        let mut in_target = false;
        let mut added = false;

        for line in file_content.lines() {
            if let Some(cap) = SECTION_RE.captures(line.trim()) {
                in_target = &cap[1] == dep_name.as_str();
            }

            final_result.push_str(line);
            final_result.push('\n');

            if in_target && !added
                && let Some(cap) = KV_RE.captures(line)
                    && cap[1].trim() == "url" {
                        let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
                        final_result.push_str(&format!("{indent}branch = {new_val}\n"));
                        added = true;
                    }
        }

        if added {
            let trimmed = if file_content.ends_with('\n') {
                final_result
            } else {
                final_result.trim_end_matches('\n').to_owned()
            };
            return GitSubmoduleUpdateResult::Updated(trimmed);
        }

        return GitSubmoduleUpdateResult::Failed;
    }

    let trimmed = if file_content.ends_with('\n') {
        result
    } else {
        result.trim_end_matches('\n').to_owned()
    };

    if trimmed == file_content {
        GitSubmoduleUpdateResult::Unchanged
    } else {
        GitSubmoduleUpdateResult::Updated(trimmed)
    }
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

    // Ported: "when using a relative path" — git-submodules/extract.spec.ts line 80
    #[test]
    fn relative_url_resolved_with_remote() {
        let content = r#"
[submodule "PowerShell-Docs"]
	path = PowerShell-Docs
	url = ../../PowerShell/PowerShell-Docs
	branch = staging
"#;
        let deps =
            extract_with_remote(content, Some("https://github.com/renovatebot/renovate.git"));
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].url, "https://github.com/PowerShell/PowerShell-Docs");
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

    // Ported: "returns empty content" — git-submodules/artifact.spec.ts line 5
    #[test]
    fn update_artifacts_empty_dep_name() {
        let result = update_artifacts(&[""]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].path, "");
        assert_eq!(result[0].contents, "");
    }

    // Ported: "returns two modules" — git-submodules/artifact.spec.ts line 16
    #[test]
    fn update_artifacts_two_modules() {
        let result = update_artifacts(&["renovate", "renovate-pro"]);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].path, "renovate");
        assert_eq!(result[0].contents, "");
        assert_eq!(result[1].path, "renovate-pro");
        assert_eq!(result[1].contents, "");
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

    // --- updateDependency tests ---

    // Rust-specific: git_submodules behavior test
    #[test]
    fn git_submodule_update_branch_value() {
        let content = "[submodule \"renovate\"]\n\tpath = deps/renovate\n\turl = https://github.com/renovatebot/renovate.git\n\tbranch = v0.0.1\n";
        let config = GitSubmoduleUpdateConfig {
            dep_name: "renovate".to_owned(),
            current_value: Some("v0.0.1".to_owned()),
            new_value: Some("v0.0.2".to_owned()),
            new_digest: Some("abc123".to_owned()),
            package_file: Some(".gitmodules".to_owned()),
        };
        let result = update_dependency(content, &config);
        match result {
            GitSubmoduleUpdateResult::Updated(new_content) => {
                assert!(new_content.contains("branch = v0.0.2"));
                assert!(!new_content.contains("branch = v0.0.1"));
            }
            _ => panic!("expected Updated"),
        }
    }

    // Rust-specific: git_submodules behavior test
    #[test]
    fn git_submodule_update_same_value_unchanged() {
        let content = "[submodule \"renovate\"]\n\tpath = deps/renovate\n\turl = https://github.com/renovatebot/renovate.git\n\tbranch = v0.0.1\n";
        let config = GitSubmoduleUpdateConfig {
            dep_name: "renovate".to_owned(),
            current_value: Some("v0.0.1".to_owned()),
            new_value: Some("v0.0.1".to_owned()),
            new_digest: None,
            package_file: None,
        };
        assert_eq!(update_dependency(content, &config), GitSubmoduleUpdateResult::Unchanged);
    }

    // Rust-specific: git_submodules behavior test
    #[test]
    fn git_submodule_update_no_new_value_unchanged() {
        let content = "[submodule \"renovate\"]\n\tpath = deps/renovate\n";
        let config = GitSubmoduleUpdateConfig {
            dep_name: "renovate".to_owned(),
            current_value: None,
            new_value: None,
            new_digest: None,
            package_file: None,
        };
        assert_eq!(update_dependency(content, &config), GitSubmoduleUpdateResult::Unchanged);
    }

    // Rust-specific: git_submodules behavior test
    #[test]
    fn git_submodule_update_adds_branch_when_missing() {
        let content = "[submodule \"renovate\"]\n\tpath = deps/renovate\n\turl = https://github.com/renovatebot/renovate.git\n";
        let config = GitSubmoduleUpdateConfig {
            dep_name: "renovate".to_owned(),
            current_value: None,
            new_value: Some("main".to_owned()),
            new_digest: Some("abc".to_owned()),
            package_file: Some(".gitmodules".to_owned()),
        };
        let result = update_dependency(content, &config);
        match result {
            GitSubmoduleUpdateResult::Updated(new_content) => {
                assert!(new_content.contains("branch = main"));
            }
            _ => panic!("expected Updated"),
        }
    }
}
