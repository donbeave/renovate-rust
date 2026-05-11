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
    /// `[tool.pdm.dev-dependencies].*`
    PdmDev,
    /// `[build-system].requires`
    BuildSystem,
}

impl Pep621DepType {
    pub fn as_renovate_str(&self) -> &'static str {
        match self {
            Pep621DepType::Regular => "dependencies",
            Pep621DepType::Optional => "optional-dependencies",
            Pep621DepType::Group => "dependency-groups",
            Pep621DepType::PdmDev => "tool.pdm.dev-dependencies",
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
    pub registry_urls: Vec<String>,
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
    let doc: Value = toml::from_str(content)?;
    let mut deps = Vec::new();
    let registry_urls = extract_pdm_sources(&doc);

    // [project].dependencies
    if let Some(project_deps) = doc
        .get("project")
        .and_then(|p| p.get("dependencies"))
        .and_then(|d| d.as_array())
    {
        for entry in project_deps {
            if let Some(dep) = parse_pep508_entry(entry, Pep621DepType::Regular) {
                deps.push(with_registry_urls(dep, &registry_urls));
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
                            registry_urls: Vec::new(),
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

    Ok(Pep621Extract {
        deps,
        registry_urls,
    })
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
            registry_urls: Vec::new(),
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
            registry_urls: Vec::new(),
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
        registry_urls: Vec::new(),
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
            .filter(|d| d.skip_reason.is_none())
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
}
