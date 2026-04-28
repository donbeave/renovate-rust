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

use thiserror::Error;
use toml::Value;

/// Which `pyproject.toml` section the dep came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pep621DepType {
    /// `[project].dependencies`
    Regular,
    /// `[project.optional-dependencies].*`
    Optional,
    /// `[dependency-groups].*` (PEP 735)
    Group,
    /// `[build-system].requires`
    BuildSystem,
}

/// Why a pyproject.toml dependency is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pep621SkipReason {
    /// Entry is a PEP 735 group-include table (`{include-group = "…"}`).
    GroupInclude,
    /// Entry is a direct URL or VCS reference.
    DirectReference,
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
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<Pep621SkipReason>,
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
    let doc: Value = toml::from_str(content)?;
    let mut deps = Vec::new();

    // [project].dependencies
    if let Some(project_deps) = doc
        .get("project")
        .and_then(|p| p.get("dependencies"))
        .and_then(|d| d.as_array())
    {
        for entry in project_deps {
            if let Some(dep) = parse_pep508_entry(entry, Pep621DepType::Regular) {
                deps.push(dep);
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
                        deps.push(dep);
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
                            skip_reason: Some(Pep621SkipReason::GroupInclude),
                        });
                    } else if let Some(dep) = parse_pep508_entry(entry, Pep621DepType::Group) {
                        deps.push(dep);
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

    Ok(deps)
}

// ── Helpers ───────────────────────────────────────────────────────────────────

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
        skip_reason: None,
    }
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

    #[test]
    fn empty_content_returns_empty() {
        let deps = extract_ok("");
        assert!(deps.is_empty());
    }

    // ── real-world fixture (from Renovate pep621 fixture) ─────────────────────

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
}
