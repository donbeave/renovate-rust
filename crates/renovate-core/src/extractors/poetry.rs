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
    pub registry_urls: Vec<String>,
    pub source_name: Option<String>,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<PoetrySkipReason>,
}

/// Package-file level Poetry extraction data.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PoetryExtract {
    pub deps: Vec<PoetryExtractedDep>,
    pub registry_urls: Vec<String>,
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
    Ok(extract_package_file(content)?.deps)
}

/// Parse a Poetry `pyproject.toml` and extract deps plus package-file metadata.
pub fn extract_package_file(content: &str) -> Result<PoetryExtract, PoetryExtractError> {
    let doc: Value = toml::from_str(content)?;
    let mut deps = Vec::new();
    let registry_urls = extract_registry_urls(&doc);
    let source_urls_by_name = extract_source_urls_by_name(&doc);

    // [tool.poetry.dependencies]
    if let Some(tbl) = nested_table(&doc, &["tool", "poetry", "dependencies"]) {
        for (name, value) in tbl {
            if let Some(dep) = parse_dep(name, value, PoetryDepType::Regular, &source_urls_by_name)
            {
                deps.push(dep);
            }
        }
    }

    // [tool.poetry.dev-dependencies]
    if let Some(tbl) = nested_table(&doc, &["tool", "poetry", "dev-dependencies"]) {
        for (name, value) in tbl {
            if let Some(dep) = parse_dep(name, value, PoetryDepType::Dev, &source_urls_by_name) {
                deps.push(dep);
            }
        }
    }

    // [tool.poetry.group.*.dependencies]
    if let Some(groups) = nested_table(&doc, &["tool", "poetry", "group"]) {
        for (_group_name, group_val) in groups {
            if let Some(group_deps) = group_val.get("dependencies").and_then(|d| d.as_table()) {
                for (name, value) in group_deps {
                    if let Some(dep) =
                        parse_dep(name, value, PoetryDepType::Group, &source_urls_by_name)
                    {
                        deps.push(dep);
                    }
                }
            }
        }
    }

    // [build-system].requires
    if let Some(requires) = doc
        .get("build-system")
        .and_then(|bs| bs.get("requires"))
        .and_then(|r| r.as_array())
    {
        for req in requires {
            if let Some(s) = req.as_str()
                && let Some(dep) = parse_build_system_req(s)
            {
                deps.push(dep);
            }
        }
    }

    Ok(PoetryExtract {
        deps,
        registry_urls,
    })
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

fn parse_dep(
    name: &str,
    value: &Value,
    dep_type: PoetryDepType,
    source_urls_by_name: &std::collections::BTreeMap<String, String>,
) -> Option<PoetryExtractedDep> {
    // Python itself is not a PyPI package.
    if name == "python" {
        return Some(PoetryExtractedDep {
            name: name.to_owned(),
            current_value: value.as_str().unwrap_or("").to_owned(),
            dep_type,
            registry_urls: Vec::new(),
            source_name: None,
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
                registry_urls: Vec::new(),
                source_name: None,
                skip_reason: None,
            })
        }
        Value::Table(tbl) => {
            // Helper to extract version from table, defaulting to empty.
            let table_version = |tbl: &toml::map::Map<String, Value>| -> String {
                tbl.get("version")
                    .and_then(|v| v.as_str())
                    .filter(|v| *v != "*")
                    .unwrap_or("")
                    .to_owned()
            };
            if tbl.contains_key("git") {
                return Some(PoetryExtractedDep {
                    name: normalized,
                    current_value: table_version(tbl),
                    dep_type,
                    registry_urls: Vec::new(),
                    source_name: None,
                    skip_reason: Some(PoetrySkipReason::GitSource),
                });
            }
            if tbl.contains_key("path") {
                return Some(PoetryExtractedDep {
                    name: normalized,
                    current_value: table_version(tbl),
                    dep_type,
                    registry_urls: Vec::new(),
                    source_name: None,
                    skip_reason: Some(PoetrySkipReason::LocalPath),
                });
            }
            if tbl.contains_key("url") {
                return Some(PoetryExtractedDep {
                    name: normalized,
                    current_value: table_version(tbl),
                    dep_type,
                    registry_urls: Vec::new(),
                    source_name: None,
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
            let source_name = tbl.get("source").and_then(Value::as_str).map(str::to_owned);
            let registry_urls = source_name
                .as_deref()
                .and_then(|name| source_urls_by_name.get(name))
                .map(|url| vec![url.clone()])
                .unwrap_or_default();
            Some(PoetryExtractedDep {
                name: normalized,
                current_value,
                dep_type,
                registry_urls,
                source_name,
                skip_reason: None,
            })
        }
        // Array form (platform-conditional) — skip for now.
        _ => None,
    }
}

fn extract_registry_urls(doc: &Value) -> Vec<String> {
    let Some(sources) = doc
        .get("tool")
        .and_then(|tool| tool.get("poetry"))
        .and_then(|poetry| poetry.get("source"))
        .and_then(Value::as_array)
    else {
        return Vec::new();
    };

    let mut urls = Vec::new();
    let mut pypi_explicit = false;
    for source in sources {
        if source
            .get("name")
            .and_then(Value::as_str)
            .is_some_and(|name| name.eq_ignore_ascii_case("pypi"))
            && source
                .get("priority")
                .and_then(Value::as_str)
                .is_some_and(|priority| priority == "explicit")
        {
            pypi_explicit = true;
        }
        if let Some(url) = source.get("url").and_then(Value::as_str)
            && !urls.iter().any(|existing| existing == url)
        {
            urls.push(url.to_owned());
        }
    }

    const PYPI: &str = "https://pypi.org/pypi/";
    if !sources.is_empty() && !pypi_explicit && !urls.iter().any(|url| url == PYPI) {
        urls.push(PYPI.to_owned());
    }

    urls
}

fn extract_source_urls_by_name(doc: &Value) -> std::collections::BTreeMap<String, String> {
    let mut urls = std::collections::BTreeMap::from([(
        "pypi".to_owned(),
        "https://pypi.org/pypi/".to_owned(),
    )]);
    if let Some(sources) = doc
        .get("tool")
        .and_then(|tool| tool.get("poetry"))
        .and_then(|poetry| poetry.get("source"))
        .and_then(Value::as_array)
    {
        for source in sources {
            let Some(name) = source.get("name").and_then(Value::as_str) else {
                continue;
            };
            let Some(url) = source.get("url").and_then(Value::as_str) else {
                continue;
            };
            urls.insert(name.to_owned(), url.to_owned());
        }
    }
    urls
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
        registry_urls: Vec::new(),
        source_name: None,
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

    // Ported: "extracts multiple dependencies" — poetry/extract.spec.ts line 51
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

    // Ported: "extracts multiple dependencies (with dep = {version = \"1.2.3\"} case)" — poetry/extract.spec.ts line 60
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

    // Ported: "skips git dependencies" — poetry/extract.spec.ts line 363
    #[test]
    fn git_source_skipped() {
        let content = r#"
[tool.poetry.dependencies]
mylib = {git = "https://github.com/example/mylib.git", tag = "v1.0"}
"#;
        let deps = extract_ok(content);
        assert_eq!(deps[0].skip_reason, Some(PoetrySkipReason::GitSource));
    }

    // Ported: "skips path dependencies" — poetry/extract.spec.ts line 388
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

    // Ported: "extracts mixed versioning types" — poetry/extract.spec.ts line 118
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

    // Ported: "extracts dependencies from dependency groups" — poetry/extract.spec.ts line 160
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

    // Ported: "does not include registry url for dependency python" — poetry/extract.spec.ts line 413
    #[test]
    fn python_dependency_has_no_registry_urls() {
        let content = r#"
[tool.poetry.dependencies]
python = "^3.11"

[[tool.poetry.source]]
name = "custom-source"
url = "https://example.com"
priority = "explicit"
"#;
        let package_file = extract_package_file(content).expect("parse should succeed");
        assert_eq!(package_file.deps.len(), 1);
        let python = &package_file.deps[0];
        assert_eq!(python.name, "python");
        assert_eq!(python.current_value, "^3.11");
        assert_eq!(python.skip_reason, Some(PoetrySkipReason::PythonVersion));
        assert!(python.registry_urls.is_empty());
        assert_eq!(python.source_name, None);
    }

    // Ported: "can parse empty registries" — poetry/extract.spec.ts line 436
    #[test]
    fn empty_registry_list_returns_no_registry_urls() {
        let content = r#"
[tool.poetry]
name = "example"
version = "0.1.0"
source = []

[tool.poetry.dependencies]
dep0 = "0.0.0"
"#;
        let package_file = extract_package_file(content).expect("parse should succeed");
        assert!(package_file.registry_urls.is_empty());
    }

    // Ported: "can parse missing registries" — poetry/extract.spec.ts line 441
    #[test]
    fn missing_registry_list_returns_no_registry_urls() {
        let content = r#"
[tool.poetry]
name = "example"
version = "0.1.0"

[tool.poetry.dependencies]
dep0 = "0.0.0"
"#;
        let package_file = extract_package_file(content).expect("parse should succeed");
        assert!(package_file.registry_urls.is_empty());
    }

    // Ported: "extracts registries" — poetry/extract.spec.ts line 446
    #[test]
    fn extracts_registry_urls() {
        let content = r#"
[tool.poetry.dependencies]
dep0 = "0.0.0"

[[tool.poetry.source]]
name = "foo"
url = "https://foo.bar/simple/"

[[tool.poetry.source]]
name = "bar"
url = "https://bar.baz/+simple/"
"#;
        let package_file = extract_package_file(content).expect("parse should succeed");
        assert_eq!(
            package_file.registry_urls,
            vec![
                "https://foo.bar/simple/".to_owned(),
                "https://bar.baz/+simple/".to_owned(),
                "https://pypi.org/pypi/".to_owned(),
            ]
        );
    }

    // Ported: "dedupes registries" — poetry/extract.spec.ts line 455
    #[test]
    fn dedupes_registry_urls() {
        let content = r#"
[tool.poetry.dependencies]
dep0 = "0.0.0"

[[tool.poetry.source]]
name = "foo"
url = "https://pypi.org/pypi/"

[[tool.poetry.source]]
name = "bar"
url = "https://bar.baz/+simple/"

[[tool.poetry.source]]
name = "baz"
"#;
        let package_file = extract_package_file(content).expect("parse should succeed");
        assert_eq!(
            package_file.registry_urls,
            vec![
                "https://pypi.org/pypi/".to_owned(),
                "https://bar.baz/+simple/".to_owned(),
            ]
        );
    }

    // Ported: "source with priority=\"default\" and implicit PyPI priority=\"primary\"" — poetry/extract.spec.ts line 463
    #[test]
    fn source_default_with_implicit_pypi_primary() {
        let content = r#"
[tool.poetry.dependencies]
python = "^3.11"

[[tool.poetry.source]]
name = "foo"
url = "https://foo.bar/simple/"
priority = "default"

[[tool.poetry.source]]
name = "PyPI"
"#;
        let package_file = extract_package_file(content).expect("parse should succeed");
        assert_eq!(
            package_file.registry_urls,
            vec![
                "https://foo.bar/simple/".to_owned(),
                "https://pypi.org/pypi/".to_owned(),
            ]
        );
    }

    // Ported: "source with implicit priority and PyPI with priority=\"explicit\"" — poetry/extract.spec.ts line 483
    #[test]
    fn source_with_explicit_pypi_suppresses_implicit_pypi_url() {
        let content = r#"
[tool.poetry.dependencies]
python = "^3.11"

[[tool.poetry.source]]
name = "foo"
url = "https://foo.bar/simple/"

[[tool.poetry.source]]
name = "PyPI"
priority = "explicit"
"#;
        let package_file = extract_package_file(content).expect("parse should succeed");
        assert_eq!(
            package_file.registry_urls,
            vec!["https://foo.bar/simple/".to_owned()]
        );
    }

    // Ported: "supports dependencies with explicit source" — poetry/extract.spec.ts line 500
    #[test]
    fn dependencies_with_explicit_source_get_registry_urls() {
        let content = r#"
[tool.poetry.dependencies]
attrs = "^23.1.0"
typer = { version = "^0.9.0", source = "pypi" }
requests-cache = { version = "^1.1.0", source = "artifactory" }

[[tool.poetry.source]]
name = "artifactory"
url = "https://example.com"
priority = "explicit"
"#;
        let package_file = extract_package_file(content).expect("parse should succeed");
        let attrs = package_file
            .deps
            .iter()
            .find(|dep| dep.name == "attrs")
            .unwrap();
        assert_eq!(attrs.current_value, "^23.1.0");
        assert!(attrs.registry_urls.is_empty());
        assert_eq!(attrs.source_name, None);

        let typer = package_file
            .deps
            .iter()
            .find(|dep| dep.name == "typer")
            .unwrap();
        assert_eq!(typer.current_value, "^0.9.0");
        assert_eq!(typer.source_name.as_deref(), Some("pypi"));
        assert_eq!(
            typer.registry_urls,
            vec!["https://pypi.org/pypi/".to_owned()]
        );

        let requests_cache = package_file
            .deps
            .iter()
            .find(|dep| dep.name == "requests-cache")
            .unwrap();
        assert_eq!(requests_cache.current_value, "^1.1.0");
        assert_eq!(requests_cache.source_name.as_deref(), Some("artifactory"));
        assert_eq!(
            requests_cache.registry_urls,
            vec!["https://example.com".to_owned()]
        );
    }

    // ── Fixture: pyproject.1.toml ─────────────────────────────────────────────

    // Ported: "extracts multiple dependencies" — poetry/extract.spec.ts line 51
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

    // Ported: "extracts build-system.requires dependencies" — poetry/extract.spec.ts line 77
    #[test]
    fn extracts_build_system_requires() {
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

    // Ported: "returns null for parsed file without poetry section" — poetry/extract.spec.ts line 47
    #[test]
    fn no_poetry_section_returns_empty() {
        let content = "[project]\nname = \"myapp\"\n";
        let deps = extract_ok(content);
        assert!(deps.is_empty());
    }

    // Ported: "returns null for empty" — poetry/extract.spec.ts line 43
    #[test]
    fn empty_content_returns_empty() {
        let deps = extract_ok("");
        assert!(deps.is_empty());
        // "nothing here" is invalid TOML → parse error or empty
        let result = extract("nothing here");
        assert!(result.is_err() || result.unwrap().is_empty());
    }

    // Ported: "handles case with no dependencies" — poetry/extract.spec.ts line 66
    #[test]
    fn poetry_section_with_no_deps_returns_empty() {
        let content = r#"
[tool.poetry]
name = "myapp"
version = "1.0.0"
"#;
        let deps = extract_ok(content);
        assert!(deps.is_empty());
    }

    // Ported: "skips git dependencies with version" — poetry/extract.spec.ts line 375
    #[test]
    fn git_dep_with_version_shows_version() {
        let content = r#"[tool.poetry.dependencies]
flask = {git = "https://github.com/pallets/flask.git", version="1.2.3"}
werkzeug = ">=0.14"
"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 2);
        let flask = deps.iter().find(|d| d.name == "flask").unwrap();
        assert_eq!(flask.current_value, "1.2.3");
        assert_eq!(flask.skip_reason, Some(PoetrySkipReason::GitSource));
    }

    // Ported: "skips path dependencies with version" — poetry/extract.spec.ts line 400
    #[test]
    fn path_dep_with_version_shows_version() {
        let content = r#"[tool.poetry.dependencies]
flask = {path = "/some/path/", version = "1.2.3"}
werkzeug = ">=0.14"
"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 2);
        let flask = deps.iter().find(|d| d.name == "flask").unwrap();
        assert_eq!(flask.current_value, "1.2.3");
        assert_eq!(flask.skip_reason, Some(PoetrySkipReason::LocalPath));
    }
}
