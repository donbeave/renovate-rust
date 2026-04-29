//! Poetry `pyproject.toml` dependency extractor.
//!
//! Parses Poetry-managed `pyproject.toml` files and returns the set of
//! package dependencies with their version specifiers, ready for PyPI
//! version lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/poetry/extract.ts` — `extractPackageFile`
//! - `lib/modules/manager/poetry/index.ts`   — `managerFilePatterns`
//!
//! ## Supported sections
//!
//! | Section | Dep type |
//! |---|---|
//! | `[tool.poetry.dependencies]`       | `Regular` |
//! | `[tool.poetry.dev-dependencies]`   | `Dev` |
//! | `[tool.poetry.group.*.dependencies]` | `Group` |
//!
//! ## Value forms
//!
//! | TOML form | Treatment |
//! |---|---|
//! | `requests = "^2.28.0"` | version = `^2.28.0` |
//! | `django = {version = "4.2.7"}` | version = `4.2.7` |
//! | `mylib = {git = "…"}` | skip (`GitSource`) |
//! | `locallib = {path = "…"}` | skip (`LocalPath`) |
//! | `pkg = {url = "…"}` | skip (`UrlInstall`) |
//! | `python = "…"` | skip (`PythonVersion`) |
//! | `pkg = "*"` | actionable, empty constraint |

use thiserror::Error;
use toml::Value;

/// Which `pyproject.toml` section the dep came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoetryDepType {
    /// `[tool.poetry.dependencies]`
    Regular,
    /// `[tool.poetry.dev-dependencies]`
    Dev,
    /// `[tool.poetry.group.*.dependencies]`
    Group,
    /// `[build-system].requires`
    BuildSystem,
}

impl PoetryDepType {
    pub fn as_renovate_str(&self) -> &'static str {
        match self {
            PoetryDepType::Regular => "dependencies",
            PoetryDepType::Dev => "dev-dependencies",
            PoetryDepType::Group => "group",
            PoetryDepType::BuildSystem => "build-system.requires",
        }
    }
}

/// Why a Poetry dependency is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoetrySkipReason {
    /// Entry is the Python interpreter version constraint.
    PythonVersion,
    /// Entry is a VCS (git) source.
    GitSource,
    /// Entry is a local path source.
    LocalPath,
    /// Entry is a direct URL source.
    UrlInstall,
}

/// A single extracted Poetry dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PoetryExtractedDep {
    /// Normalized package name (PEP 503: lowercase, separators → `-`).
    pub name: String,
    /// Raw version specifier.  Empty string = unconstrained (`"*"`).
    pub current_value: String,
    /// Which section this dep came from.
    pub dep_type: PoetryDepType,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<PoetrySkipReason>,
}

/// Errors from parsing a Poetry `pyproject.toml`.
#[derive(Debug, Error)]
pub enum PoetryExtractError {
    #[error("TOML parse error: {0}")]
    Toml(#[from] toml::de::Error),
}

// ── Public API ─────────────────────────────────────────────────────────────

/// Parse a Poetry `pyproject.toml` and extract all Python dependencies.
pub fn extract(content: &str) -> Result<Vec<PoetryExtractedDep>, PoetryExtractError> {
    let doc: Value = toml::from_str(content)?;
    let mut deps = Vec::new();

    // [tool.poetry.dependencies]
    if let Some(tbl) = nested_table(&doc, &["tool", "poetry", "dependencies"]) {
        for (name, value) in tbl {
            if let Some(dep) = parse_dep(name, value, PoetryDepType::Regular) {
                deps.push(dep);
            }
        }
    }

    // [tool.poetry.dev-dependencies]
    if let Some(tbl) = nested_table(&doc, &["tool", "poetry", "dev-dependencies"]) {
        for (name, value) in tbl {
            if let Some(dep) = parse_dep(name, value, PoetryDepType::Dev) {
                deps.push(dep);
            }
        }
    }

    // [tool.poetry.group.*.dependencies]
    if let Some(groups) = nested_table(&doc, &["tool", "poetry", "group"]) {
        for (_group_name, group_val) in groups {
            if let Some(group_deps) = group_val.get("dependencies").and_then(|d| d.as_table()) {
                for (name, value) in group_deps {
                    if let Some(dep) = parse_dep(name, value, PoetryDepType::Group) {
                        deps.push(dep);
                    }
                }
            }
        }
    }

    // [build-system].requires
    if let Some(requires) = doc.get("build-system").and_then(|bs| bs.get("requires")).and_then(|r| r.as_array()) {
        for req in requires {
            if let Some(s) = req.as_str() {
                if let Some(dep) = parse_build_system_req(s) {
                    deps.push(dep);
                }
            }
        }
    }

    Ok(deps)
}

// ── Helpers ───────────────────────────────────────────────────────────────

/// Traverse nested table keys, returning `None` at the first missing key.
fn nested_table<'v>(root: &'v Value, keys: &[&str]) -> Option<&'v toml::map::Map<String, Value>> {
    let mut cur = root;
    for &key in keys {
        cur = cur.get(key)?;
    }
    cur.as_table()
}

fn parse_dep(name: &str, value: &Value, dep_type: PoetryDepType) -> Option<PoetryExtractedDep> {
    // Python itself is not a PyPI package.
    if name == "python" {
        return Some(PoetryExtractedDep {
            name: name.to_owned(),
            current_value: value.as_str().unwrap_or("").to_owned(),
            dep_type,
            skip_reason: Some(PoetrySkipReason::PythonVersion),
        });
    }

    let normalized = normalize_name(name);

    match value {
        Value::String(v) => {
            let current_value = if v == "*" { String::new() } else { v.clone() };
            Some(PoetryExtractedDep {
                name: normalized,
                current_value,
                dep_type,
                skip_reason: None,
            })
        }
        Value::Table(tbl) => {
            if tbl.contains_key("git") {
                return Some(PoetryExtractedDep {
                    name: normalized,
                    current_value: String::new(),
                    dep_type,
                    skip_reason: Some(PoetrySkipReason::GitSource),
                });
            }
            if tbl.contains_key("path") {
                return Some(PoetryExtractedDep {
                    name: normalized,
                    current_value: String::new(),
                    dep_type,
                    skip_reason: Some(PoetrySkipReason::LocalPath),
                });
            }
            if tbl.contains_key("url") {
                return Some(PoetryExtractedDep {
                    name: normalized,
                    current_value: String::new(),
                    dep_type,
                    skip_reason: Some(PoetrySkipReason::UrlInstall),
                });
            }
            // `{version = "…", optional = true, extras = […]}` — standard dep.
            let v = tbl.get("version")?.as_str()?;
            let current_value = if v == "*" {
                String::new()
            } else {
                v.to_owned()
            };
            Some(PoetryExtractedDep {
                name: normalized,
                current_value,
                dep_type,
                skip_reason: None,
            })
        }
        // Array form (platform-conditional) — skip for now.
        _ => None,
    }
}

/// Normalize a Python package name per PEP 503.
/// Parse a PEP 508 build-system requirement string like `"poetry-core>=1.0.0"`.
fn parse_build_system_req(req: &str) -> Option<PoetryExtractedDep> {
    // Find the first version operator char position.
    let op_chars = ['>', '<', '=', '!', '~', '^'];
    let sep = req.find(|c| op_chars.contains(&c)).unwrap_or(req.len());
    let raw_name = req[..sep].trim();
    let version = req[sep..].trim();

    if raw_name.is_empty() {
        return None;
    }

    // Strip any extras like `package[extra]`.
    let name_clean = raw_name.split('[').next().unwrap_or(raw_name).trim();
    let normalized = normalize_name(name_clean);

    Some(PoetryExtractedDep {
        name: normalized,
        current_value: version.to_owned(),
        dep_type: PoetryDepType::BuildSystem,
        skip_reason: None,
    })
}

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

    fn extract_ok(content: &str) -> Vec<PoetryExtractedDep> {
        extract(content).expect("parse should succeed")
    }

    // ── [tool.poetry.dependencies] ────────────────────────────────────────────

    #[test]
    fn extracts_string_deps() {
        let content = r#"
[tool.poetry.dependencies]
python = "^3.9"
requests = "^2.28.0"
django = "4.2.7"
boto3 = "*"
"#;
        let deps = extract_ok(content);
        let regular: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == PoetryDepType::Regular)
            .collect();
        assert_eq!(regular.len(), 4);

        let py = regular.iter().find(|d| d.name == "python").unwrap();
        assert_eq!(py.skip_reason, Some(PoetrySkipReason::PythonVersion));

        let req = regular.iter().find(|d| d.name == "requests").unwrap();
        assert_eq!(req.current_value, "^2.28.0");
        assert!(req.skip_reason.is_none());

        let boto = regular.iter().find(|d| d.name == "boto3").unwrap();
        assert!(
            boto.current_value.is_empty(),
            "wildcard should produce empty constraint"
        );
    }

    #[test]
    fn extracts_table_deps() {
        let content = r#"
[tool.poetry.dependencies]
django = {version = "4.2.7", optional = true}
mypackage = {version = "^1.0", extras = ["security"]}
"#;
        let deps = extract_ok(content);
        let django = deps.iter().find(|d| d.name == "django").unwrap();
        assert_eq!(django.current_value, "4.2.7");
        assert!(django.skip_reason.is_none());

        let mypackage = deps.iter().find(|d| d.name == "mypackage").unwrap();
        assert_eq!(mypackage.current_value, "^1.0");
    }

    #[test]
    fn git_source_skipped() {
        let content = r#"
[tool.poetry.dependencies]
mylib = {git = "https://github.com/example/mylib.git", tag = "v1.0"}
"#;
        let deps = extract_ok(content);
        assert_eq!(deps[0].skip_reason, Some(PoetrySkipReason::GitSource));
    }

    #[test]
    fn path_source_skipped() {
        let content = r#"
[tool.poetry.dependencies]
locallib = {path = "../locallib", develop = true}
"#;
        let deps = extract_ok(content);
        assert_eq!(deps[0].skip_reason, Some(PoetrySkipReason::LocalPath));
    }

    #[test]
    fn url_source_skipped() {
        let content = r#"
[tool.poetry.dependencies]
mypkg = {url = "https://example.com/mypkg-1.0.tar.gz"}
"#;
        let deps = extract_ok(content);
        assert_eq!(deps[0].skip_reason, Some(PoetrySkipReason::UrlInstall));
    }

    // ── [tool.poetry.dev-dependencies] ───────────────────────────────────────

    #[test]
    fn extracts_dev_dependencies() {
        let content = r#"
[tool.poetry.dev-dependencies]
pytest = "^7.0"
black = "^23.0"
"#;
        let deps = extract_ok(content);
        let dev: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == PoetryDepType::Dev)
            .collect();
        assert_eq!(dev.len(), 2, "dev dep count");
        assert!(
            dev.iter()
                .any(|d| d.name == "pytest" && d.current_value == "^7.0")
        );
    }

    // ── [tool.poetry.group.*.dependencies] ───────────────────────────────────

    #[test]
    fn extracts_group_dependencies() {
        let content = r#"
[tool.poetry.group.dev.dependencies]
pytest = "^7.0"

[tool.poetry.group.lint.dependencies]
ruff = "^0.1.0"
"#;
        let deps = extract_ok(content);
        let groups: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == PoetryDepType::Group)
            .collect();
        assert_eq!(groups.len(), 2);
        assert!(groups.iter().any(|d| d.name == "pytest"));
        assert!(
            groups
                .iter()
                .any(|d| d.name == "ruff" && d.current_value == "^0.1.0")
        );
    }

    // ── Name normalization ────────────────────────────────────────────────────

    #[test]
    fn name_normalized_per_pep503() {
        let content = r#"
[tool.poetry.dependencies]
PyYAML = "^6.0"
Typing_Extensions = "^4.0"
"#;
        let deps = extract_ok(content);
        assert!(deps.iter().any(|d| d.name == "pyyaml"));
        assert!(deps.iter().any(|d| d.name == "typing-extensions"));
    }

    // ── Fixture: pyproject.1.toml ─────────────────────────────────────────────

    #[test]
    fn poetry_fixture_1() {
        let content = r#"
[tool.poetry.dependencies]
dep1 = "0.0.0"
dep2 = "^0.6.0"
dep3 = "^0.33.6"
python = "~2.7 || ^3.4"

[tool.poetry.dev-dependencies]
dev_dep1 = "^3.0"
dev_dep2 = "Invalid version."
"#;
        let deps = extract_ok(content);
        let regular: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == PoetryDepType::Regular)
            .collect();
        assert_eq!(regular.len(), 4); // dep1, dep2, dep3, python

        let py = regular.iter().find(|d| d.name == "python").unwrap();
        assert_eq!(py.skip_reason, Some(PoetrySkipReason::PythonVersion));

        let dep2 = regular.iter().find(|d| d.name == "dep2").unwrap();
        assert_eq!(dep2.current_value, "^0.6.0");

        assert_eq!(
            deps.iter()
                .filter(|d| d.dep_type == PoetryDepType::Dev)
                .count(),
            2
        );
    }

    // ── [build-system].requires ──────────────────────────────────────────────

    #[test]
    fn extracts_build_system_requires() {
        // Ported: "extracts build-system.requires dependencies" — poetry/extract.spec.ts line 77
        let content = r#"
[build-system]
requires = ["poetry-core>=1.0.0", "setuptools>=40.0"]
build-backend = "poetry.core.masonry.api"

[tool.poetry]
name = "test"
version = "1.0.0"

[tool.poetry.dependencies]
abc = "^5.5"
"#;
        let deps = extract_ok(content);
        let bs: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == PoetryDepType::BuildSystem)
            .collect();
        assert_eq!(bs.len(), 2);
        let core = bs.iter().find(|d| d.name == "poetry-core").unwrap();
        assert_eq!(core.current_value, ">=1.0.0");
        assert_eq!(core.dep_type, PoetryDepType::BuildSystem);
        let setup = bs.iter().find(|d| d.name == "setuptools").unwrap();
        assert_eq!(setup.current_value, ">=40.0");

        // Regular dep is still extracted
        let regular: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == PoetryDepType::Regular)
            .collect();
        assert_eq!(regular.len(), 1);
        assert_eq!(regular[0].name, "abc");
    }

    // ── Empty / no poetry section ─────────────────────────────────────────────

    #[test]
    fn no_poetry_section_returns_empty() {
        let content = "[project]\nname = \"myapp\"\n";
        let deps = extract_ok(content);
        assert!(deps.is_empty());
    }

    #[test]
    fn empty_content_returns_empty() {
        let deps = extract_ok("");
        assert!(deps.is_empty());
    }
}
