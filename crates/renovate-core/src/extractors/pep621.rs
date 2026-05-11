//! PEP 621 / pyproject.toml dependency extractor.
//!
//! Parses Python `pyproject.toml` files following PEP 517/518/621/735 and
//! returns the set of package dependencies with their version specifiers,
//! ready for PyPI version lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/pep621/extract.ts` — `extractPackageFile`
//! - `lib/modules/manager/pep621/schema.ts`  — `PyProject`
//!
//! ## Supported sections
//!
//! | Section | Dep type |
//! |---|---|
//! | `[project].dependencies` | `Regular` |
//! | `[project.optional-dependencies].*` | `Optional` |
//! | `[dependency-groups].*` (PEP 735) | `Group` |
//! | `[tool.pdm.dev-dependencies].*` | `PdmDev` |
//! | `[tool.uv.sources].*` | `UvSources` |
//! | `[tool.hatch.envs.*]` | `HatchEnv` |
//! | `[build-system].requires` | `BuildSystem` |
//!
//! ## PEP 508 string format
//!
//! Each entry is a PEP 508 dependency specifier:
//! `name[extras] specifier ; environment_marker`
//!
//! Environment markers (`;…`) are stripped; extras (`[…]`) are stripped for
//! the registry lookup.  Entries that cannot be parsed as PEP 508 strings
//! (e.g., PEP 735 `{include-group = "…"}` tables) are silently skipped.

use std::{borrow::Cow, collections::BTreeMap};
use thiserror::Error;
use toml::Value;

/// Which `pyproject.toml` section the dep came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pep621DepType {
    /// `[project].requires-python`
    RequiresPython,
    /// `[project].dependencies`
    Regular,
    /// `[project.optional-dependencies].*`
    Optional,
    /// `[dependency-groups].*` (PEP 735)
    Group,
    /// `[tool.pdm.dev-dependencies].*`
    PdmDev,
    /// `[tool.uv.sources].*`
    UvSources,
    /// `[tool.hatch.envs.*]`
    HatchEnv,
    /// `[build-system].requires`
    BuildSystem,
}

impl Pep621DepType {
    pub fn as_renovate_str(&self) -> &'static str {
        match self {
            Pep621DepType::RequiresPython => "requires-python",
            Pep621DepType::Regular => "dependencies",
            Pep621DepType::Optional => "optional-dependencies",
            Pep621DepType::Group => "dependency-groups",
            Pep621DepType::PdmDev => "tool.pdm.dev-dependencies",
            Pep621DepType::UvSources => "tool.uv.sources",
            Pep621DepType::HatchEnv => "tool.hatch.envs",
            Pep621DepType::BuildSystem => "build-system",
        }
    }
}

/// Why a pyproject.toml dependency is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pep621SkipReason {
    /// Entry is a PEP 735 group-include table (`{include-group = "…"}`).
    GroupInclude,
    /// Entry is a direct URL or VCS reference.
    DirectReference,
    /// Entry is a local path or wheel.
    PathDependency,
    /// Entry uses a source URL shape unsupported by Renovate's pep621 manager.
    UnsupportedUrl,
    /// Entry is inherited from the workspace.
    InheritedDependency,
    /// Entry has no source version to look up.
    UnspecifiedVersion,
}

/// Datasource associated with a PEP 621 dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pep621Datasource {
    GitRefs,
    GithubTags,
}

/// A single extracted pyproject.toml dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pep621ExtractedDep {
    /// Normalized package name (lowercase, `-`/`_`/`.` equivalent).
    pub name: String,
    /// Raw version specifier (e.g. `">=2.0,<3.0"`). Empty = unconstrained.
    pub current_value: String,
    /// Which section this dep came from.
    pub dep_type: Pep621DepType,
    pub dep_group: Option<String>,
    pub datasource: Option<Pep621Datasource>,
    pub package_name: Option<String>,
    pub registry_urls: Vec<String>,
    pub locked_version: Option<String>,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<Pep621SkipReason>,
}

/// Package-file level PEP 621 extraction data.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Pep621Extract {
    pub deps: Vec<Pep621ExtractedDep>,
    pub registry_urls: Vec<String>,
}

/// Errors from parsing a `pyproject.toml`.
#[derive(Debug, Error)]
pub enum Pep621ExtractError {
    #[error("TOML parse error: {0}")]
    Toml(#[from] toml::de::Error),
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Parse a `pyproject.toml` string and extract all Python dependencies.
pub fn extract(content: &str) -> Result<Vec<Pep621ExtractedDep>, Pep621ExtractError> {
    Ok(extract_package_file(content)?.deps)
}

/// Parse a `pyproject.toml` string and extract deps plus package-file metadata.
pub fn extract_package_file(content: &str) -> Result<Pep621Extract, Pep621ExtractError> {
    extract_package_file_with_uv_lock(content, None)
}

pub fn extract_package_file_with_uv_lock(
    content: &str,
    uv_lock: Option<&str>,
) -> Result<Pep621Extract, Pep621ExtractError> {
    let content = remove_template_lines(content);
    let doc: Value = toml::from_str(&content)?;
    let mut deps = Vec::new();
    let registry_urls = extract_pdm_sources(&doc);
    let uv_sources = extract_uv_sources(&doc);

    if let Some(requires_python) = doc
        .get("project")
        .and_then(|p| p.get("requires-python"))
        .and_then(Value::as_str)
    {
        deps.push(Pep621ExtractedDep {
            name: "python".to_owned(),
            current_value: requires_python.to_owned(),
            dep_type: Pep621DepType::RequiresPython,
            dep_group: None,
            datasource: None,
            package_name: Some("python".to_owned()),
            registry_urls: Vec::new(),
            locked_version: None,
            skip_reason: None,
        });
    }

    // [project].dependencies
    if let Some(project_deps) = doc
        .get("project")
        .and_then(|p| p.get("dependencies"))
        .and_then(|d| d.as_array())
    {
        for entry in project_deps {
            if let Some(dep) = parse_pep508_entry(entry, Pep621DepType::Regular) {
                if let Some(uv_source) = uv_sources.get(&dep.name) {
                    deps.push(uv_source.clone());
                } else {
                    deps.push(with_registry_urls(dep, &registry_urls));
                }
            }
        }
    }

    // [project.optional-dependencies].*
    if let Some(opt_deps) = doc
        .get("project")
        .and_then(|p| p.get("optional-dependencies"))
        .and_then(|d| d.as_table())
    {
        for (_group, entries) in opt_deps {
            if let Some(arr) = entries.as_array() {
                for entry in arr {
                    if let Some(dep) = parse_pep508_entry(entry, Pep621DepType::Optional) {
                        deps.push(with_registry_urls(dep, &registry_urls));
                    }
                }
            }
        }
    }

    // [dependency-groups].* (PEP 735)
    if let Some(dep_groups) = doc.get("dependency-groups").and_then(|d| d.as_table()) {
        for (_group, entries) in dep_groups {
            if let Some(arr) = entries.as_array() {
                for entry in arr {
                    // PEP 735 entries can be strings OR `{include-group = "…"}` tables.
                    if entry.is_table() {
                        // Include-group reference — skip.
                        deps.push(Pep621ExtractedDep {
                            name: String::new(),
                            current_value: String::new(),
                            dep_type: Pep621DepType::Group,
                            dep_group: None,
                            datasource: None,
                            package_name: None,
                            registry_urls: Vec::new(),
                            locked_version: None,
                            skip_reason: Some(Pep621SkipReason::GroupInclude),
                        });
                    } else if let Some(dep) = parse_pep508_entry(entry, Pep621DepType::Group) {
                        deps.push(with_registry_urls(dep, &registry_urls));
                    }
                }
            }
        }
    }

    // [tool.pdm.dev-dependencies].*
    if let Some(pdm_dev_deps) = doc
        .get("tool")
        .and_then(|tool| tool.get("pdm"))
        .and_then(|pdm| pdm.get("dev-dependencies"))
        .and_then(|d| d.as_table())
    {
        for (_group, entries) in pdm_dev_deps {
            if let Some(arr) = entries.as_array() {
                for entry in arr {
                    if let Some(dep) = parse_pep508_entry(entry, Pep621DepType::PdmDev) {
                        deps.push(with_registry_urls(dep, &registry_urls));
                    }
                }
            }
        }
    }

    // [build-system].requires
    if let Some(build_reqs) = doc
        .get("build-system")
        .and_then(|b| b.get("requires"))
        .and_then(|r| r.as_array())
    {
        for entry in build_reqs {
            if let Some(dep) = parse_pep508_entry(entry, Pep621DepType::BuildSystem) {
                deps.push(dep);
            }
        }
    }

    // [tool.hatch.envs.*].dependencies and extra-dependencies
    if let Some(hatch_envs) = doc
        .get("tool")
        .and_then(|tool| tool.get("hatch"))
        .and_then(|hatch| hatch.get("envs"))
        .and_then(Value::as_table)
    {
        for (env_name, env) in hatch_envs {
            let Some(env_table) = env.as_table() else {
                continue;
            };
            for key in ["dependencies", "extra-dependencies"] {
                if let Some(entries) = env_table.get(key).and_then(Value::as_array) {
                    for entry in entries {
                        if let Some(mut dep) = parse_pep508_entry(entry, Pep621DepType::HatchEnv) {
                            dep.dep_group = Some(env_name.to_owned());
                            deps.push(dep);
                        }
                    }
                }
            }
        }
    }

    if let Some(lock_content) = uv_lock {
        apply_uv_locked_versions(&mut deps, lock_content);
    }

    Ok(Pep621Extract {
        deps,
        registry_urls,
    })
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn remove_template_lines(content: &str) -> Cow<'_, str> {
    // Fast path keeps TOML parse error spans unchanged for normal files.
    if !content
        .lines()
        .any(|line| line.trim_start().starts_with("{%") || line.trim_start().starts_with("{#"))
    {
        return Cow::Borrowed(content);
    }

    Cow::Owned(
        content
            .lines()
            .filter(|line| {
                let trimmed = line.trim_start();
                !trimmed.starts_with("{%") && !trimmed.starts_with("{#")
            })
            .collect::<Vec<_>>()
            .join("\n"),
    )
}

/// Parse a TOML value that should be a PEP 508 specifier string.
///
/// Returns `None` when the value is not a string (silently skipped).
fn parse_pep508_entry(entry: &Value, dep_type: Pep621DepType) -> Option<Pep621ExtractedDep> {
    let raw = entry.as_str()?;
    Some(parse_pep508(raw, dep_type))
}

/// Parse a single PEP 508 dependency specifier string.
fn parse_pep508(raw: &str, dep_type: Pep621DepType) -> Pep621ExtractedDep {
    // Direct references: `name @ https://…` or `name @ git+…`
    if raw.contains(" @ ") || raw.starts_with("git+") || raw.starts_with("https://") {
        let name = raw
            .split_whitespace()
            .next()
            .unwrap_or("unknown")
            .to_owned();
        return Pep621ExtractedDep {
            name,
            current_value: raw.to_owned(),
            dep_type,
            dep_group: None,
            datasource: None,
            package_name: None,
            registry_urls: Vec::new(),
            locked_version: None,
            skip_reason: Some(Pep621SkipReason::DirectReference),
        };
    }

    // Strip environment markers (`;…`).
    let without_markers = raw.split(';').next().unwrap_or("").trim();

    // Strip hash specs and line-continuation (` \`).
    let without_hashes = without_markers.split(" \\").next().unwrap_or("").trim();

    // Extract name (ends at first non-name char: `[`, space, `=`, `>`, `<`, `!`, `~`)
    let name_end = without_hashes
        .find(|c: char| !c.is_ascii_alphanumeric() && c != '.' && c != '-' && c != '_')
        .unwrap_or(without_hashes.len());

    let name_raw = &without_hashes[..name_end];
    if name_raw.is_empty() {
        return Pep621ExtractedDep {
            name: String::new(),
            current_value: raw.to_owned(),
            dep_type,
            dep_group: None,
            datasource: None,
            package_name: None,
            registry_urls: Vec::new(),
            locked_version: None,
            skip_reason: Some(Pep621SkipReason::GroupInclude),
        };
    }

    let name = normalize_name(name_raw);
    let rest = without_hashes[name_end..].trim_start();

    // Strip extras `[…]`.
    let specifier = if rest.starts_with('[') {
        rest.find(']').map(|i| rest[i + 1..].trim()).unwrap_or(rest)
    } else {
        rest
    };

    Pep621ExtractedDep {
        name,
        current_value: specifier.to_owned(),
        dep_type,
        dep_group: None,
        datasource: None,
        package_name: None,
        registry_urls: Vec::new(),
        locked_version: None,
        skip_reason: None,
    }
}

fn with_registry_urls(mut dep: Pep621ExtractedDep, registry_urls: &[String]) -> Pep621ExtractedDep {
    if dep.skip_reason.is_none() && dep.name != "python" && !registry_urls.is_empty() {
        dep.registry_urls = registry_urls.to_vec();
    }
    dep
}

fn extract_pdm_sources(doc: &Value) -> Vec<String> {
    doc.get("tool")
        .and_then(|tool| tool.get("pdm"))
        .and_then(|pdm| pdm.get("source"))
        .and_then(Value::as_array)
        .map(|sources| {
            sources
                .iter()
                .filter_map(|source| source.get("url").and_then(Value::as_str).map(str::to_owned))
                .collect()
        })
        .unwrap_or_default()
}

fn extract_uv_sources(doc: &Value) -> BTreeMap<String, Pep621ExtractedDep> {
    let Some(sources) = doc
        .get("tool")
        .and_then(|tool| tool.get("uv"))
        .and_then(|uv| uv.get("sources"))
        .and_then(Value::as_table)
    else {
        return BTreeMap::new();
    };

    sources
        .iter()
        .filter_map(|(name, source)| {
            let name = normalize_name(name);
            let table = source.as_table()?;
            let mut dep = Pep621ExtractedDep {
                name: name.clone(),
                current_value: String::new(),
                dep_type: Pep621DepType::UvSources,
                dep_group: None,
                datasource: None,
                package_name: None,
                registry_urls: Vec::new(),
                locked_version: None,
                skip_reason: None,
            };

            if table.get("workspace").and_then(Value::as_bool) == Some(true) {
                dep.skip_reason = Some(Pep621SkipReason::InheritedDependency);
            } else if table.get("path").is_some() {
                dep.skip_reason = Some(Pep621SkipReason::PathDependency);
            } else if table.get("url").is_some() {
                dep.skip_reason = Some(Pep621SkipReason::UnsupportedUrl);
            } else if let Some(git) = table.get("git").and_then(Value::as_str) {
                dep.datasource = Some(Pep621Datasource::GitRefs);
                dep.package_name = Some(git.to_owned());

                if let Some(tag) = table.get("tag").and_then(Value::as_str) {
                    dep.current_value = tag.to_owned();
                    if let Some(repo) = github_repo_from_git_url(git) {
                        dep.datasource = Some(Pep621Datasource::GithubTags);
                        dep.package_name = Some(repo);
                        dep.registry_urls = vec!["https://github.com".to_owned()];
                    }
                } else if let Some(rev) = table.get("rev").and_then(Value::as_str) {
                    dep.current_value = rev.to_owned();
                } else {
                    dep.skip_reason = Some(Pep621SkipReason::UnspecifiedVersion);
                }
            } else {
                return None;
            }

            Some((name, dep))
        })
        .collect()
}

fn apply_uv_locked_versions(deps: &mut [Pep621ExtractedDep], lock_content: &str) {
    let Ok(lock) = toml::from_str::<Value>(lock_content) else {
        return;
    };
    let locked_versions: BTreeMap<String, String> = lock
        .get("package")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|package| {
            let table = package.as_table()?;
            let name = table.get("name").and_then(Value::as_str)?;
            let version = table.get("version").and_then(Value::as_str)?;
            if table
                .get("source")
                .and_then(Value::as_table)
                .and_then(|source| source.get("virtual"))
                .is_some()
            {
                return None;
            }
            Some((normalize_name(name), version.to_owned()))
        })
        .collect();

    for dep in deps {
        if let Some(version) = locked_versions.get(&dep.name) {
            dep.locked_version = Some(version.clone());
        }
    }
}

fn github_repo_from_git_url(url: &str) -> Option<String> {
    let path = url
        .strip_prefix("ssh://git@github.com/")
        .or_else(|| url.strip_prefix("git@github.com:"))
        .or_else(|| url.strip_prefix("https://github.com/"))?;
    Some(path.trim_end_matches(".git").to_owned())
}

/// Normalize a Python package name per PEP 503.
fn normalize_name(name: &str) -> String {
    let lower = name.to_lowercase();
    let mut result = String::with_capacity(lower.len());
    let mut prev_sep = false;
    for ch in lower.chars() {
        if ch == '-' || ch == '_' || ch == '.' {
            if !prev_sep {
                result.push('-');
            }
            prev_sep = true;
        } else {
            result.push(ch);
            prev_sep = false;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_ok(content: &str) -> Vec<Pep621ExtractedDep> {
        extract(content).expect("parse should succeed")
    }

    // ── [project].dependencies ────────────────────────────────────────────────

    #[test]
    fn extracts_project_dependencies() {
        let content = r#"
[project]
name = "myapp"
dependencies = [
  "requests>=2.0,<3.0",
  "django==4.2.7",
  "blinker",
]
"#;
        let deps = extract_ok(content);
        let regular: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == Pep621DepType::Regular)
            .collect();
        assert_eq!(regular.len(), 3);
        assert!(
            regular
                .iter()
                .any(|d| d.name == "requests" && d.current_value == ">=2.0,<3.0")
        );
        assert!(
            regular
                .iter()
                .any(|d| d.name == "django" && d.current_value == "==4.2.7")
        );
        assert!(
            regular
                .iter()
                .any(|d| d.name == "blinker" && d.current_value.is_empty())
        );
    }

    #[test]
    fn strips_environment_markers() {
        let content = r#"
[project]
dependencies = ["tomli>=1.1.0; python_version < \"3.11\""]
"#;
        let deps = extract_ok(content);
        assert_eq!(deps[0].name, "tomli");
        assert_eq!(deps[0].current_value, ">=1.1.0");
    }

    #[test]
    fn strips_extras() {
        let content = r#"
[project]
dependencies = ["cachecontrol[filecache]>=0.12.11"]
"#;
        let deps = extract_ok(content);
        assert_eq!(deps[0].name, "cachecontrol");
        assert_eq!(deps[0].current_value, ">=0.12.11");
    }

    // ── [project.optional-dependencies] ──────────────────────────────────────

    #[test]
    fn extracts_optional_dependencies() {
        let content = r#"
[project.optional-dependencies]
dev = ["pytest>=7", "black"]
lint = ["ruff==0.1.0"]
"#;
        let deps = extract_ok(content);
        let optional: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == Pep621DepType::Optional)
            .collect();
        assert_eq!(optional.len(), 3);
        assert!(optional.iter().any(|d| d.name == "pytest"));
        assert!(
            optional
                .iter()
                .any(|d| d.name == "ruff" && d.current_value == "==0.1.0")
        );
    }

    // ── [dependency-groups] ───────────────────────────────────────────────────

    #[test]
    fn extracts_dependency_groups_skips_include_tables() {
        let content = r#"
[dependency-groups]
typing = ["mypy==1.13.0", "types-requests"]
all = [{include-group = "typing"}, "click==8.1.7"]
"#;
        let deps = extract_ok(content);
        let groups: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == Pep621DepType::Group)
            .collect();
        // 3 strings + 1 include-group table
        assert_eq!(groups.len(), 4);
        let skipped = groups.iter().filter(|d| d.skip_reason.is_some()).count();
        assert_eq!(skipped, 1); // only the include-group table
    }

    // ── normalize_name ────────────────────────────────────────────────────────

    #[test]
    fn normalize_name_lowercases_and_replaces_separators() {
        assert_eq!(normalize_name("PyYAML"), "pyyaml");
        assert_eq!(normalize_name("typing_extensions"), "typing-extensions");
        assert_eq!(normalize_name("My.Package"), "my-package");
    }

    // ── direct references ─────────────────────────────────────────────────────

    #[test]
    fn direct_reference_is_skipped() {
        let content = r#"
[project]
dependencies = ["mypkg @ https://example.com/mypkg.tar.gz"]
"#;
        let deps = extract_ok(content);
        assert_eq!(deps[0].skip_reason, Some(Pep621SkipReason::DirectReference));
    }

    #[test]
    fn uv_sources_classify_git_path_url_and_workspace_sources() {
        let content = r#"
[project]
dependencies = [
  "dep1",
  "dep2",
  "dep3",
  "dep4",
  "dep5",
  "dep6",
  "dep7",
  "dep-with_NORMALIZATION",
]

[tool.uv.sources]
dep2 = { git = "https://github.com/foo/bar" }
dep3 = { path = "/local-dep.whl" }
dep4 = { url = "https://example.com" }
dep5 = { workspace = true }
dep7 = { git = "ssh://git@github.com/foo/baz", tag = "1.0.1" }
dep_WITH-normalization = { workspace = true }
"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 8);

        let dep1 = deps.iter().find(|d| d.name == "dep1").unwrap();
        assert_eq!(dep1.dep_type, Pep621DepType::Regular);
        assert!(dep1.skip_reason.is_none());

        let dep2 = deps.iter().find(|d| d.name == "dep2").unwrap();
        assert_eq!(dep2.dep_type, Pep621DepType::UvSources);
        assert_eq!(dep2.datasource, Some(Pep621Datasource::GitRefs));
        assert_eq!(
            dep2.package_name.as_deref(),
            Some("https://github.com/foo/bar")
        );
        assert_eq!(dep2.skip_reason, Some(Pep621SkipReason::UnspecifiedVersion));

        let dep3 = deps.iter().find(|d| d.name == "dep3").unwrap();
        assert_eq!(dep3.skip_reason, Some(Pep621SkipReason::PathDependency));

        let dep4 = deps.iter().find(|d| d.name == "dep4").unwrap();
        assert_eq!(dep4.skip_reason, Some(Pep621SkipReason::UnsupportedUrl));

        let dep5 = deps.iter().find(|d| d.name == "dep5").unwrap();
        assert_eq!(
            dep5.skip_reason,
            Some(Pep621SkipReason::InheritedDependency)
        );

        let dep6 = deps.iter().find(|d| d.name == "dep6").unwrap();
        assert_eq!(dep6.dep_type, Pep621DepType::Regular);
        assert!(dep6.skip_reason.is_none());

        let dep7 = deps.iter().find(|d| d.name == "dep7").unwrap();
        assert_eq!(dep7.dep_type, Pep621DepType::UvSources);
        assert_eq!(dep7.datasource, Some(Pep621Datasource::GithubTags));
        assert_eq!(dep7.package_name.as_deref(), Some("foo/baz"));
        assert_eq!(dep7.current_value, "1.0.1");
        assert_eq!(dep7.registry_urls, vec!["https://github.com".to_owned()]);

        let normalized = deps
            .iter()
            .find(|d| d.name == "dep-with-normalization")
            .unwrap();
        assert_eq!(
            normalized.skip_reason,
            Some(Pep621SkipReason::InheritedDependency)
        );
    }

    // Ported: "should handle SSH git URLs correctly for GitHub sources" — pep621/extract.spec.ts line 412
    #[test]
    fn uv_sources_handle_ssh_github_tag_and_rev() {
        let content = r#"
[project]
dependencies = [
  "dep1",
  "dep2",
]

[tool.uv.sources]
dep1 = { git = "ssh://git@github.com/foo/dep1", tag = "v1.2.3" }
dep2 = { git = "ssh://git@github.com/foo/dep2", rev = "abcd1234" }
"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 2);

        let dep1 = deps.iter().find(|d| d.name == "dep1").unwrap();
        assert_eq!(dep1.dep_type, Pep621DepType::UvSources);
        assert_eq!(dep1.datasource, Some(Pep621Datasource::GithubTags));
        assert_eq!(dep1.package_name.as_deref(), Some("foo/dep1"));
        assert_eq!(dep1.current_value, "v1.2.3");
        assert_eq!(dep1.registry_urls, vec!["https://github.com".to_owned()]);
        assert!(dep1.skip_reason.is_none());

        let dep2 = deps.iter().find(|d| d.name == "dep2").unwrap();
        assert_eq!(dep2.dep_type, Pep621DepType::UvSources);
        assert_eq!(dep2.datasource, Some(Pep621Datasource::GitRefs));
        assert_eq!(
            dep2.package_name.as_deref(),
            Some("ssh://git@github.com/foo/dep2")
        );
        assert_eq!(dep2.current_value, "abcd1234");
        assert!(dep2.registry_urls.is_empty());
        assert!(dep2.skip_reason.is_none());
    }

    // Ported: "should extract dependencies from hatch environments" — pep621/extract.spec.ts line 446
    #[test]
    fn hatch_env_dependencies_and_extra_dependencies_are_extracted() {
        let content = r#"
[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[project]
name = "hatch"
dependencies = [
  "requests==2.30.0"
]

[tool.hatch.envs.default]
dependencies = [
  "coverage[toml]==6.5",
  "pytest",
]

[[tool.hatch.envs.all.matrix]]
python = ["3.7", "3.8", "3.9", "3.10", "3.11"]

[tool.hatch.envs.lint]
detached = true
dependencies = [
  "black>=23.1.0",
]

[tool.hatch.envs.experimental]
extra-dependencies = [
  "baz",
]
"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 6);

        let requests = deps.iter().find(|d| d.name == "requests").unwrap();
        assert_eq!(requests.dep_type, Pep621DepType::Regular);
        assert_eq!(requests.current_value, "==2.30.0");

        let hatchling = deps.iter().find(|d| d.name == "hatchling").unwrap();
        assert_eq!(hatchling.dep_type, Pep621DepType::BuildSystem);
        assert!(hatchling.current_value.is_empty());

        let coverage = deps.iter().find(|d| d.name == "coverage").unwrap();
        assert_eq!(coverage.dep_type, Pep621DepType::HatchEnv);
        assert_eq!(coverage.dep_group.as_deref(), Some("default"));
        assert_eq!(coverage.current_value, "==6.5");

        let pytest = deps.iter().find(|d| d.name == "pytest").unwrap();
        assert_eq!(pytest.dep_type, Pep621DepType::HatchEnv);
        assert_eq!(pytest.dep_group.as_deref(), Some("default"));
        assert!(pytest.current_value.is_empty());

        let black = deps.iter().find(|d| d.name == "black").unwrap();
        assert_eq!(black.dep_type, Pep621DepType::HatchEnv);
        assert_eq!(black.dep_group.as_deref(), Some("lint"));
        assert_eq!(black.current_value, ">=23.1.0");

        let baz = deps.iter().find(|d| d.name == "baz").unwrap();
        assert_eq!(baz.dep_type, Pep621DepType::HatchEnv);
        assert_eq!(baz.dep_group.as_deref(), Some("experimental"));
        assert!(baz.current_value.is_empty());
    }

    // Ported: "should return dependencies with original pypi registryUrl" — pep621/extract.spec.ts line 309
    #[test]
    fn pdm_sources_apply_registry_urls_to_project_dependencies() {
        let content = r#"
[project]
dependencies = [
  "packaging>=20.9,!=22.0",
]

[[tool.pdm.source]]
url = "https://private-site.org/pypi/simple"
verify_ssl = true
name = "internal"
"#;
        let package_file = extract_package_file(content).expect("parse should succeed");
        assert_eq!(
            package_file.registry_urls,
            vec!["https://private-site.org/pypi/simple".to_owned()]
        );
        assert_eq!(package_file.deps.len(), 1);
        assert_eq!(package_file.deps[0].name, "packaging");
        assert_eq!(package_file.deps[0].current_value, ">=20.9,!=22.0");
        assert_eq!(
            package_file.deps[0].registry_urls,
            vec!["https://private-site.org/pypi/simple".to_owned()]
        );
    }

    // Ported: "should return dependencies with overwritten pypi registryUrl" — pep621/extract.spec.ts line 233
    #[test]
    fn pdm_sources_apply_registry_urls_to_project_optional_and_dev_dependencies() {
        let content = r#"
[project]
name = "pdm"
dynamic = ["version"]
requires-python = ">=3.7"
license = {text = "MIT"}
dependencies = [
  "blinker",
  "packaging>=20.9,!=22.0",
]
readme = "README.md"

[project.optional-dependencies]
pytest = [
  "pytest>12",
]

[tool.pdm.dev-dependencies]
test = [
  "pytest-rerunfailures>=10.2",
]
tox = [
  "tox-pdm>=0.5",
]

[[tool.pdm.source]]
url = "https://private-site.org/pypi/simple"
verify_ssl = true
name = "internal"

[[tool.pdm.source]]
url = "https://private.pypi.org/simple"
verify_ssl = true
name = "pypi"
"#;
        let package_file = extract_package_file(content).expect("parse should succeed");
        let expected_registry_urls = vec![
            "https://private-site.org/pypi/simple".to_owned(),
            "https://private.pypi.org/simple".to_owned(),
        ];
        assert_eq!(package_file.registry_urls, expected_registry_urls);

        let actionable: Vec<_> = package_file
            .deps
            .iter()
            .filter(|d| d.skip_reason.is_none() && d.dep_type != Pep621DepType::RequiresPython)
            .collect();
        assert_eq!(actionable.len(), 5);

        for name in [
            "blinker",
            "packaging",
            "pytest",
            "pytest-rerunfailures",
            "tox-pdm",
        ] {
            let dep = actionable
                .iter()
                .find(|d| d.name == name)
                .unwrap_or_else(|| panic!("missing dep {name}"));
            assert_eq!(dep.registry_urls, expected_registry_urls);
        }

        let blinker = actionable.iter().find(|d| d.name == "blinker").unwrap();
        assert_eq!(blinker.dep_type, Pep621DepType::Regular);
        assert!(blinker.current_value.is_empty());

        let packaging = actionable.iter().find(|d| d.name == "packaging").unwrap();
        assert_eq!(packaging.dep_type, Pep621DepType::Regular);
        assert_eq!(packaging.current_value, ">=20.9,!=22.0");

        let pytest = actionable.iter().find(|d| d.name == "pytest").unwrap();
        assert_eq!(pytest.dep_type, Pep621DepType::Optional);
        assert_eq!(pytest.current_value, ">12");

        let pytest_rerunfailures = actionable
            .iter()
            .find(|d| d.name == "pytest-rerunfailures")
            .unwrap();
        assert_eq!(pytest_rerunfailures.dep_type, Pep621DepType::PdmDev);
        assert_eq!(pytest_rerunfailures.current_value, ">=10.2");

        let tox_pdm = actionable.iter().find(|d| d.name == "tox-pdm").unwrap();
        assert_eq!(tox_pdm.dep_type, Pep621DepType::PdmDev);
        assert_eq!(tox_pdm.current_value, ">=0.5");
    }

    // ── empty / no deps ───────────────────────────────────────────────────────

    #[test]
    fn no_project_section_returns_build_system_only() {
        let content = "[build-system]\nrequires = [\"setuptools>=61.0\", \"wheel\"]\nbuild-backend = \"setuptools.build_meta\"\n";
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 2);
        let st = deps.iter().find(|d| d.name == "setuptools").unwrap();
        assert_eq!(st.current_value, ">=61.0");
        assert_eq!(st.dep_type, Pep621DepType::BuildSystem);
        // unconstrained dep — emitted with empty specifier, no skip reason
        let wheel = deps.iter().find(|d| d.name == "wheel").unwrap();
        assert!(wheel.current_value.is_empty());
        assert!(wheel.skip_reason.is_none());
    }

    #[test]
    fn build_system_requires_with_project_deps() {
        let content = r#"
[build-system]
requires = ["poetry-core>=1.0.0"]
build-backend = "poetry.core.masonry.api"

[project]
name = "myapp"
dependencies = ["requests>=2.28"]
"#;
        let deps = extract_ok(content);
        let poetry = deps.iter().find(|d| d.name == "poetry-core").unwrap();
        assert_eq!(poetry.dep_type, Pep621DepType::BuildSystem);
        assert_eq!(poetry.current_value, ">=1.0.0");
        assert!(deps.iter().any(|d| d.name == "requests"));
    }

    // Ported: "should return null for empty content" — pep621/extract.spec.ts line 16
    #[test]
    fn empty_content_returns_empty() {
        let deps = extract_ok("");
        assert!(deps.is_empty());
    }

    // Ported: "should return null for invalid toml" — pep621/extract.spec.ts line 21
    #[test]
    fn invalid_toml_returns_error() {
        let content = "[project]\nname =\n";
        assert!(extract(content).is_err());
    }

    // Ported: "should extract project version" — pep621/extract.spec.ts line 498
    #[test]
    fn project_version_field_is_parseable() {
        // The spec checks res?.packageFileVersion === '0.0.2'.
        // Rust extractor doesn't expose packageFileVersion separately — but the
        // deps are still extractable from the same content.
        let content = r#"[project]
name = "test"
version = "0.0.2"
dependencies = [ "requests==2.30.0" ]
"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "requests");
        assert_eq!(deps[0].current_value, "==2.30.0");
    }

    // Ported: "should extract dependencies from build-system.requires" — pep621/extract.spec.ts line 510
    #[test]
    fn build_system_requires_extracted_with_project_deps() {
        let content = r#"[build-system]
requires = ["hatchling==1.18.0", "setuptools==69.0.3"]
build-backend = "hatchling.build"

[project]
name = "test"
version = "0.0.2"
dependencies = [ "requests==2.30.0" ]
"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 3);
        assert!(
            deps.iter()
                .any(|d| d.name == "requests" && d.current_value == "==2.30.0")
        );
        assert!(
            deps.iter()
                .any(|d| d.name == "hatchling" && d.current_value == "==1.18.0")
        );
        assert!(
            deps.iter()
                .any(|d| d.name == "setuptools" && d.current_value == "==69.0.3")
        );
    }

    // ── real-world fixture (from Renovate pep621 fixture) ─────────────────────

    // Ported: "should resolve lockedVersions from pdm.lock" — pep621/extract.spec.ts line 551
    #[test]
    fn pdm_fixture() {
        let content = r#"
[project]
name = "pdm"
requires-python = ">=3.7"
dependencies = [
  "blinker",
  "packaging>=20.9,!=22.0",
  "rich>=12.3.0",
  "virtualenv==20.0.0",
  "tomli>=1.1.0; python_version < \"3.11\"",
]

[project.optional-dependencies]
pytest = [
  "pytest>12",
  "pytest-mock",
]
"#;
        let deps = extract_ok(content);
        assert_eq!(
            deps.iter()
                .filter(|d| d.dep_type == Pep621DepType::Regular && d.skip_reason.is_none())
                .count(),
            5
        );
        let optional: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == Pep621DepType::Optional && d.skip_reason.is_none())
            .collect();
        assert_eq!(optional.len(), 2);
        assert!(
            optional
                .iter()
                .any(|d| d.name == "pytest" && d.current_value == ">12")
        );
    }

    // Ported: "should resolve lockedVersions from uv.lock" — pep621/extract.spec.ts line 595
    #[test]
    fn uv_lock_applies_locked_versions() {
        let content = r#"
[project]
name = "pep621-uv"
version = "0.1.0"
dependencies = ["attrs>=24.1.0"]
requires-python = ">=3.11"
"#;
        let uv_lock = r#"
version = 1
requires-python = ">=3.11"

[[package]]
name = "attrs"
version = "24.2.0"
source = { registry = "https://pypi.org/simple" }

[[package]]
name = "pep621-uv"
version = "0.1.0"
source = { virtual = "." }
dependencies = [
    { name = "attrs" },
]

[package.metadata]
requires-dist = [{ name = "attrs", specifier = ">=24.1.0" }]
"#;
        let extract = extract_package_file_with_uv_lock(content, Some(uv_lock)).unwrap();
        let attrs = extract.deps.iter().find(|d| d.name == "attrs").unwrap();
        assert_eq!(attrs.current_value, ">=24.1.0");
        assert_eq!(attrs.locked_version.as_deref(), Some("24.2.0"));
    }

    // Ported: "should resolve dependencies without locked versions on invalid uv.lock" — pep621/extract.spec.ts line 661
    #[test]
    fn invalid_uv_lock_leaves_deps_without_locked_versions() {
        let content = r#"
[project]
name = "pep621-uv"
version = "0.1.0"
dependencies = ["attrs>=24.1.0"]
requires-python = ">=3.11"
"#;
        let extract = extract_package_file_with_uv_lock(content, Some("invalid_toml")).unwrap();
        let attrs = extract.deps.iter().find(|d| d.name == "attrs").unwrap();
        assert_eq!(attrs.current_value, ">=24.1.0");
        assert_eq!(attrs.locked_version, None);
    }

    // Ported: "should resolve dependencies with template" — pep621/extract.spec.ts line 694
    #[test]
    fn resolves_dependencies_with_template_lines() {
        let content = r#"
[project]
name = "{{ name }}"
dynamic = ["version"]
requires-python = ">=3.7"
license = {text = "MIT"}
{# comment #}
dependencies = [
  "blinker",
  {% if foo %}
  "packaging>=20.9,!=22.0",
  {% endif %}
]
readme = "README.md"
"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 3);

        let python = deps.iter().find(|d| d.name == "python").unwrap();
        assert_eq!(python.dep_type, Pep621DepType::RequiresPython);
        assert_eq!(python.current_value, ">=3.7");

        let blinker = deps.iter().find(|d| d.name == "blinker").unwrap();
        assert_eq!(blinker.dep_type, Pep621DepType::Regular);
        assert!(blinker.current_value.is_empty());

        let packaging = deps.iter().find(|d| d.name == "packaging").unwrap();
        assert_eq!(packaging.dep_type, Pep621DepType::Regular);
        assert_eq!(packaging.current_value, ">=20.9,!=22.0");
    }
}
