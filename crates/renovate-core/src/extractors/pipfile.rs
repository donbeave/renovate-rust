//! Pipenv `Pipfile` dependency extractor.
//!
//! Parses the `[packages]` and `[dev-packages]` sections of a `Pipfile`
//! (TOML format) and returns Python package dependencies for PyPI lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/pipenv/extract.ts`
//! - `lib/modules/manager/pipenv/index.ts` — pattern `/(^|/)Pipfile$/`
//!
//! ## Supported entry forms
//!
//! | Form | Treatment |
//! |---|---|
//! | `requests = ">=2.25"` | Actionable |
//! | `django = {version = ">=4.0", extras = ["bcrypt"]}` | Actionable |
//! | `mylib = {git = "..."}` | `GitDependency` skip |
//! | `locallib = {path = "..."}` | `LocalDependency` skip |
//! | `unspecified = "*"` | `Wildcard` skip |
//! | `unspecified = {version = "*"}` | `Wildcard` skip |

use std::collections::BTreeMap;

use toml::Value;

/// Why a Pipfile dep is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipfileSkipReason {
    /// Version is `"*"` (any version accepted).
    Wildcard,
    /// Declared with a `git` key.
    GitDependency,
    /// Declared with a `file` key.
    FileDependency,
    /// Declared with a `path` key.
    LocalDependency,
}

/// A single extracted Pipfile dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipfileDep {
    /// Normalized package name.
    pub name: String,
    /// Version specifier string (e.g. `">=2.25"`). Empty when skipped.
    pub current_value: String,
    /// True for `[dev-packages]` entries.
    pub is_dev: bool,
    pub registry_urls: Vec<String>,
    pub skip_reason: Option<PipfileSkipReason>,
}

/// Package-file level Pipenv extraction data.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PipfileExtract {
    pub deps: Vec<PipfileDep>,
    pub registry_urls: Vec<String>,
    pub extracted_constraints: BTreeMap<String, String>,
    pub lock_files: Vec<String>,
}

/// Parse a `Pipfile` and extract all deps.
pub fn extract(content: &str) -> Vec<PipfileDep> {
    extract_package_file(content).deps
}

/// Parse a `Pipfile` and extract deps plus package-file metadata.
pub fn extract_package_file(content: &str) -> PipfileExtract {
    let table: toml::Table = match toml::from_str(content) {
        Ok(t) => t,
        Err(_) => return PipfileExtract::default(),
    };

    let mut out = Vec::new();
    let registry_urls = extract_sources(&table);
    let source_urls_by_name = extract_sources_by_name(&table);
    let mut extracted_constraints = extract_requires(&table);

    for (section_key, is_dev) in [("packages", false), ("dev-packages", true)] {
        if let Some(Value::Table(section)) = table.get(section_key) {
            for (raw_name, val) in section {
                if !is_valid_package_name(raw_name) {
                    out.push(PipfileDep {
                        name: normalize_name(raw_name),
                        current_value: String::new(),
                        is_dev,
                        registry_urls: Vec::new(),
                        skip_reason: Some(PipfileSkipReason::Wildcard),
                    });
                    continue;
                }
                let name = normalize_name(raw_name);
                let dep = parse_entry(name, val, is_dev, &source_urls_by_name);
                if dep.name == "pipenv" && dep.skip_reason.is_none() {
                    extracted_constraints.insert("pipenv".to_owned(), dep.current_value.clone());
                }
                out.push(dep);
            }
        }
    }

    PipfileExtract {
        deps: out,
        registry_urls,
        extracted_constraints,
        lock_files: vec!["Pipfile.lock".to_owned()],
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn parse_entry(
    name: String,
    val: &Value,
    is_dev: bool,
    source_urls_by_name: &BTreeMap<String, String>,
) -> PipfileDep {
    match val {
        Value::String(s) => {
            if s == "*" || !is_valid_version(s) {
                PipfileDep {
                    name,
                    current_value: s.clone(),
                    is_dev,
                    registry_urls: Vec::new(),
                    skip_reason: Some(PipfileSkipReason::Wildcard),
                }
            } else {
                PipfileDep {
                    name,
                    current_value: s.clone(),
                    is_dev,
                    registry_urls: Vec::new(),
                    skip_reason: None,
                }
            }
        }
        Value::Table(t) => {
            if t.contains_key("git") {
                return PipfileDep {
                    name,
                    current_value: String::new(),
                    is_dev,
                    registry_urls: Vec::new(),
                    skip_reason: Some(PipfileSkipReason::GitDependency),
                };
            }
            if t.contains_key("file") {
                return PipfileDep {
                    name,
                    current_value: String::new(),
                    is_dev,
                    registry_urls: Vec::new(),
                    skip_reason: Some(PipfileSkipReason::FileDependency),
                };
            }
            if t.contains_key("path") {
                return PipfileDep {
                    name,
                    current_value: String::new(),
                    is_dev,
                    registry_urls: Vec::new(),
                    skip_reason: Some(PipfileSkipReason::LocalDependency),
                };
            }
            let version = t.get("version").and_then(|v| v.as_str()).unwrap_or("");
            let registry_urls = t
                .get("index")
                .and_then(Value::as_str)
                .and_then(|name| source_urls_by_name.get(name))
                .map(|url| vec![url.clone()])
                .unwrap_or_default();
            if version == "*" || version.is_empty() {
                PipfileDep {
                    name,
                    current_value: version.to_owned(),
                    is_dev,
                    registry_urls,
                    skip_reason: Some(PipfileSkipReason::Wildcard),
                }
            } else {
                PipfileDep {
                    name,
                    current_value: version.to_owned(),
                    is_dev,
                    registry_urls,
                    skip_reason: None,
                }
            }
        }
        _ => PipfileDep {
            name,
            current_value: String::new(),
            is_dev,
            registry_urls: Vec::new(),
            skip_reason: Some(PipfileSkipReason::Wildcard),
        },
    }
}

fn extract_sources(table: &toml::Table) -> Vec<String> {
    match table.get("source") {
        Some(Value::Array(sources)) => sources
            .iter()
            .filter_map(|source| match source {
                Value::Table(source) => {
                    source.get("url").and_then(Value::as_str).map(str::to_owned)
                }
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn extract_sources_by_name(table: &toml::Table) -> BTreeMap<String, String> {
    match table.get("source") {
        Some(Value::Array(sources)) => sources
            .iter()
            .filter_map(|source| match source {
                Value::Table(source) => {
                    let name = source.get("name").and_then(Value::as_str)?;
                    let url = source.get("url").and_then(Value::as_str)?;
                    Some((name.to_owned(), url.to_owned()))
                }
                _ => None,
            })
            .collect(),
        _ => BTreeMap::new(),
    }
}

fn extract_requires(table: &toml::Table) -> BTreeMap<String, String> {
    let mut constraints = BTreeMap::new();
    if let Some(Value::Table(requires)) = table.get("requires") {
        if let Some(version) = requires.get("python_full_version").and_then(Value::as_str) {
            constraints.insert("python".to_owned(), format!("== {version}"));
        } else if let Some(version) = requires.get("python_version").and_then(Value::as_str) {
            constraints.insert("python".to_owned(), format!("== {version}.*"));
        }
    }
    constraints
}

/// Normalize PyPI package name: lowercase, replace `-`/`_`/`.` with `-`.
fn normalize_name(name: &str) -> String {
    name.to_ascii_lowercase().replace(['.', '_'], "-")
}

/// Returns true if the package name is a valid PyPI package name.
/// PEP 508: must start and end with alphanumeric; internal chars can be alphanumeric, `-`, `_`, `.`.
fn is_valid_package_name(name: &str) -> bool {
    let mut chars = name.chars();
    let first = chars.next();
    if !first.map(|c| c.is_ascii_alphanumeric()).unwrap_or(false) {
        return false;
    }
    // All characters must be alphanumeric or separators
    name.chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
}

/// Returns true if the version specifier is plausibly valid (no spaces within specifier).
fn is_valid_version(spec: &str) -> bool {
    !spec.contains(' ')
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
[[source]]
url = "https://pypi.org/simple"
verify_ssl = true
name = "pypi"

[packages]
requests = ">=2.25.1"
flask = ">=2.0,<3.0"
django = {version = ">=4.0", extras = ["bcrypt"]}
unversioned = "*"
mylib = {git = "https://github.com/org/mylib"}
locallib = {path = "../locallib"}

[dev-packages]
pytest = ">=7.0"
black = "*"
coverage = {version = ">=6.0"}
"#;

    // Ported: "extracts dependencies" — manager/pipenv/extract.spec.ts line 45
    #[test]
    fn extracts_string_form() {
        let deps = extract(SAMPLE);
        let req = deps.iter().find(|d| d.name == "requests").unwrap();
        assert_eq!(req.current_value, ">=2.25.1");
        assert!(!req.is_dev);
        assert!(req.skip_reason.is_none());
    }

    // Ported: "extracts dependencies" — manager/pipenv/extract.spec.ts line 45
    #[test]
    fn extracts_multi_constraint() {
        let deps = extract(SAMPLE);
        let flask = deps.iter().find(|d| d.name == "flask").unwrap();
        assert_eq!(flask.current_value, ">=2.0,<3.0");
    }

    // Ported: "extracts dependencies" — manager/pipenv/extract.spec.ts line 45
    #[test]
    fn extracts_table_form() {
        let deps = extract(SAMPLE);
        let django = deps.iter().find(|d| d.name == "django").unwrap();
        assert_eq!(django.current_value, ">=4.0");
        assert!(django.skip_reason.is_none());
    }

    // Ported: "ignores invalid versions" — manager/pipenv/extract.spec.ts line 223
    #[test]
    fn wildcard_skipped() {
        let deps = extract(SAMPLE);
        let unver = deps.iter().find(|d| d.name == "unversioned").unwrap();
        assert_eq!(unver.skip_reason, Some(PipfileSkipReason::Wildcard));
    }

    // Ported: "ignores git dependencies" — manager/pipenv/extract.spec.ts line 192
    #[test]
    fn git_dep_skipped() {
        let deps = extract(SAMPLE);
        let mylib = deps.iter().find(|d| d.name == "mylib").unwrap();
        assert_eq!(mylib.skip_reason, Some(PipfileSkipReason::GitDependency));
    }

    // Ported: "ignores relative path dependencies" — manager/pipenv/extract.spec.ts line 213
    #[test]
    fn local_dep_skipped() {
        let deps = extract(SAMPLE);
        let local = deps.iter().find(|d| d.name == "locallib").unwrap();
        assert_eq!(local.skip_reason, Some(PipfileSkipReason::LocalDependency));
    }

    // Ported: "extracts dependencies" — manager/pipenv/extract.spec.ts line 45
    #[test]
    fn dev_packages_flagged() {
        let deps = extract(SAMPLE);
        let pytest = deps.iter().find(|d| d.name == "pytest").unwrap();
        assert_eq!(pytest.current_value, ">=7.0");
        assert!(pytest.is_dev);
        assert!(pytest.skip_reason.is_none());
    }

    // Ported: "ignores invalid versions" — manager/pipenv/extract.spec.ts line 223
    #[test]
    fn dev_wildcard_skipped() {
        let deps = extract(SAMPLE);
        let black = deps.iter().find(|d| d.name == "black").unwrap();
        assert!(black.is_dev);
        assert_eq!(black.skip_reason, Some(PipfileSkipReason::Wildcard));
    }

    // Rust-specific: pipfile behavior test
    #[test]
    fn normalizes_package_names() {
        let content = "[packages]\nMy_Package = \">=1.0\"\n";
        let deps = extract(content);
        assert_eq!(deps[0].name, "my-package");
    }

    // Ported: "returns null for invalid toml file" — pipenv/extract.spec.ts line 41
    #[test]
    fn invalid_toml_returns_empty() {
        assert!(extract("not valid [toml").is_empty());
    }

    // Ported: "returns null for empty" — pipenv/extract.spec.ts line 37
    #[test]
    fn empty_content_returns_no_deps() {
        assert!(extract("").is_empty());
    }

    // Ported: "marks packages with \"extras\" as skipReason === unspecified-version" — pipenv/extract.spec.ts line 136
    #[test]
    fn packages_with_only_extras_are_skipped() {
        let content = r#"[packages]
raven = {extras = ['flask']}
Flask = "*"
Flask-Caching = '*'
flask-mako = {}
Flask-SQLAlchemy = {version = "*"}
Flask-Login = {editable = true}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 6);
        assert!(deps.iter().all(|d| d.skip_reason.is_some()));
    }

    // Ported: "ignores git dependencies" — pipenv/extract.spec.ts line 192
    #[test]
    fn git_dependency_in_mixed_list_skipped() {
        let content = r#"[packages]
flask = {git = "https://github.com/pallets/flask.git"}
werkzeug = ">=0.14"
"#;
        let deps = extract(content);
        let valid: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
        assert_eq!(valid.len(), 1);
        assert_eq!(valid[0].name, "werkzeug");
    }

    // Ported: "ignores invalid package names" — pipenv/extract.spec.ts line 202
    #[test]
    fn invalid_package_name_starting_with_underscore_skipped() {
        let content = r#"[packages]
foo = "==1.0.0"
_invalid = "==1.0.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        let valid: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
        assert_eq!(valid.len(), 1);
        assert_eq!(valid[0].name, "foo");
    }

    // Ported: "ignores relative path dependencies" — pipenv/extract.spec.ts line 213
    #[test]
    fn relative_path_in_mixed_list_skipped() {
        let content = r#"[packages]
foo = "==1.0.0"
test = {path = "."}
"#;
        let deps = extract(content);
        let valid: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
        assert_eq!(valid.len(), 1);
        assert_eq!(valid[0].name, "foo");
    }

    // Ported: "ignores invalid versions" — pipenv/extract.spec.ts line 223
    #[test]
    fn version_with_spaces_skipped() {
        let content = r#"[packages]
foo = "==1.0.0"
some-package = "==0 0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        let valid: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
        assert_eq!(valid.len(), 1);
        assert_eq!(valid[0].name, "foo");
    }

    // Ported: "extracts all sources" — pipenv/extract.spec.ts line 234
    #[test]
    fn extracts_all_sources() {
        let content = r#"
[[source]]
url = "source-url"
[[source]]
url = "other-source-url"
[packages]
foo = "==1.0.0"
"#;
        let package_file = extract_package_file(content);
        assert_eq!(
            package_file.registry_urls,
            vec!["source-url".to_owned(), "other-source-url".to_owned()]
        );
    }

    // Ported: "extracts example pipfile" — pipenv/extract.spec.ts line 247
    #[test]
    fn extracts_example_pipfile() {
        let package_file = extract_package_file(PIPFILE4);
        assert_eq!(
            package_file.registry_urls,
            vec!["https://pypi.python.org/simple".to_owned()]
        );
        assert_eq!(package_file.lock_files, vec!["Pipfile.lock".to_owned()]);
        assert_eq!(
            package_file.extracted_constraints.get("python"),
            Some(&"== 2.7.*".to_owned())
        );
        assert_eq!(package_file.deps.len(), 8);

        assert_dep(
            find_dep(&package_file.deps, "requests"),
            "requests",
            false,
            "",
            Some(&PipfileSkipReason::Wildcard),
            &[],
        );
        assert_dep(
            find_dep(&package_file.deps, "records"),
            "records",
            false,
            ">0.5.0",
            None,
            &[],
        );
        assert_dep(
            find_dep(&package_file.deps, "django"),
            "django",
            false,
            "",
            Some(&PipfileSkipReason::GitDependency),
            &[],
        );
        assert_dep(
            find_dep(&package_file.deps, "e682b37"),
            "e682b37",
            false,
            "",
            Some(&PipfileSkipReason::FileDependency),
            &[],
        );
        assert_dep(
            find_dep(&package_file.deps, "e1839a8"),
            "e1839a8",
            false,
            "",
            Some(&PipfileSkipReason::LocalDependency),
            &[],
        );
        assert_dep(
            find_dep(&package_file.deps, "pywinusb"),
            "pywinusb",
            false,
            "*",
            Some(&PipfileSkipReason::Wildcard),
            &["https://pypi.python.org/simple"],
        );
        assert_dep(
            find_dep(&package_file.deps, "nose"),
            "nose",
            true,
            "*",
            Some(&PipfileSkipReason::Wildcard),
            &[],
        );
        assert_dep(
            find_dep(&package_file.deps, "unittest2"),
            "unittest2",
            true,
            ">=1.0,<3.0",
            None,
            &[],
        );
    }

    fn find_dep<'a>(deps: &'a [PipfileDep], name: &str) -> &'a PipfileDep {
        deps.iter().find(|dep| dep.name == name).unwrap()
    }

    fn assert_dep(
        dep: &PipfileDep,
        name: &str,
        is_dev: bool,
        current_value: &str,
        skip_reason: Option<&PipfileSkipReason>,
        registry_urls: &[&str],
    ) {
        assert_eq!(dep.name, name);
        assert_eq!(dep.is_dev, is_dev);
        assert_eq!(dep.current_value, current_value);
        assert_eq!(dep.skip_reason.as_ref(), skip_reason);
        assert_eq!(
            dep.registry_urls,
            registry_urls
                .iter()
                .map(|url| (*url).to_owned())
                .collect::<Vec<_>>()
        );
    }

    // Ported: "supports custom index" — pipenv/extract.spec.ts line 313
    #[test]
    fn supports_custom_index() {
        let content = r#"
[[source]]
url = "https://pypi.python.org/simple"
verify_ssl = true
name = "pypi"

[[source]]
url = "https://testpypi.python.org/pypi"
verify_ssl = true
name = "testpypi"

[packages]
requests = {version = "==0.21.0", index = "testpypi"}
"#;
        let package_file = extract_package_file(content);
        assert_eq!(
            package_file.registry_urls,
            vec![
                "https://pypi.python.org/simple".to_owned(),
                "https://testpypi.python.org/pypi".to_owned(),
            ]
        );
        assert_eq!(package_file.deps.len(), 1);
        let dep = &package_file.deps[0];
        assert_eq!(dep.name, "requests");
        assert_eq!(dep.current_value, "==0.21.0");
        assert_eq!(
            dep.registry_urls,
            vec!["https://testpypi.python.org/pypi".to_owned()]
        );
        assert!(dep.skip_reason.is_none());
    }

    // Ported: "gets python constraint from python_version" — pipenv/extract.spec.ts line 338
    #[test]
    fn gets_python_constraint_from_python_version() {
        let content = r#"
[packages]
foo = "==1.0.0"
[requires]
python_version = "3.8"
"#;
        let package_file = extract_package_file(content);
        assert_eq!(
            package_file.extracted_constraints.get("python"),
            Some(&"== 3.8.*".to_owned())
        );
    }

    // Ported: "gets python constraint from python_full_version" — pipenv/extract.spec.ts line 350
    #[test]
    fn gets_python_constraint_from_python_full_version() {
        let content = r#"
[packages]
foo = "==1.0.0"
[requires]
python_full_version = "3.8.6"
"#;
        let package_file = extract_package_file(content);
        assert_eq!(
            package_file.extracted_constraints.get("python"),
            Some(&"== 3.8.6".to_owned())
        );
    }

    // Ported: "gets pipenv constraint from packages" — pipenv/extract.spec.ts line 362
    #[test]
    fn gets_pipenv_constraint_from_packages() {
        let content = r#"[packages]
pipenv = "==2020.8.13"
"#;
        let package_file = extract_package_file(content);
        assert_eq!(
            package_file.extracted_constraints.get("pipenv"),
            Some(&"==2020.8.13".to_owned())
        );
    }

    // Ported: "gets pipenv constraint from dev-packages" — pipenv/extract.spec.ts line 372
    #[test]
    fn gets_pipenv_constraint_from_dev_packages() {
        let content = r#"[dev-packages]
pipenv = "==2020.8.13"
"#;
        let package_file = extract_package_file(content);
        assert_eq!(
            package_file.extracted_constraints.get("pipenv"),
            Some(&"==2020.8.13".to_owned())
        );
    }

    const PIPFILE4: &str = r#"
[[source]]
url = 'https://pypi.python.org/simple'
verify_ssl = true
name = 'pypi'

[requires]
python_version = '2.7'

[packages]
requests = { extras = ['socks'] }
records = '>0.5.0'
django = { git = 'https://github.com/django/django.git', ref = '1.11.4', editable = true }
"e682b37" = {file = "https://github.com/divio/django-cms/archive/release/3.4.x.zip"}
"e1839a8" = {path = ".", editable = true}
pywinusb = { version = "*", os_name = "=='nt'", index="pypi"}

[dev-packages]
nose = '*'
unittest2 = {version = ">=1.0,<3.0", markers="python_version < '2.7.9' or (python_version >= '3.0' and python_version < '3.4')"}
"#;
}
