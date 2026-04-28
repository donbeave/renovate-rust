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

use toml::Value;

/// Why a Pipfile dep is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipfileSkipReason {
    /// Version is `"*"` (any version accepted).
    Wildcard,
    /// Declared with a `git` key.
    GitDependency,
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
    pub skip_reason: Option<PipfileSkipReason>,
}

/// Parse a `Pipfile` and extract all deps.
pub fn extract(content: &str) -> Vec<PipfileDep> {
    let table: toml::Table = match toml::from_str(content) {
        Ok(t) => t,
        Err(_) => return Vec::new(),
    };

    let mut out = Vec::new();

    for (section_key, is_dev) in [("packages", false), ("dev-packages", true)] {
        if let Some(Value::Table(section)) = table.get(section_key) {
            for (raw_name, val) in section {
                let name = normalize_name(raw_name);
                let dep = parse_entry(name, val, is_dev);
                out.push(dep);
            }
        }
    }

    out
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn parse_entry(name: String, val: &Value, is_dev: bool) -> PipfileDep {
    match val {
        Value::String(s) => {
            if s == "*" {
                PipfileDep {
                    name,
                    current_value: String::new(),
                    is_dev,
                    skip_reason: Some(PipfileSkipReason::Wildcard),
                }
            } else {
                PipfileDep {
                    name,
                    current_value: s.clone(),
                    is_dev,
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
                    skip_reason: Some(PipfileSkipReason::GitDependency),
                };
            }
            if t.contains_key("path") || t.contains_key("file") {
                return PipfileDep {
                    name,
                    current_value: String::new(),
                    is_dev,
                    skip_reason: Some(PipfileSkipReason::LocalDependency),
                };
            }
            let version = t.get("version").and_then(|v| v.as_str()).unwrap_or("");
            if version == "*" || version.is_empty() {
                PipfileDep {
                    name,
                    current_value: String::new(),
                    is_dev,
                    skip_reason: Some(PipfileSkipReason::Wildcard),
                }
            } else {
                PipfileDep {
                    name,
                    current_value: version.to_owned(),
                    is_dev,
                    skip_reason: None,
                }
            }
        }
        _ => PipfileDep {
            name,
            current_value: String::new(),
            is_dev,
            skip_reason: Some(PipfileSkipReason::Wildcard),
        },
    }
}

/// Normalize PyPI package name: lowercase, replace `-`/`_`/`.` with `-`.
fn normalize_name(name: &str) -> String {
    name.to_ascii_lowercase().replace(['.', '_'], "-")
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

    #[test]
    fn extracts_string_form() {
        let deps = extract(SAMPLE);
        let req = deps.iter().find(|d| d.name == "requests").unwrap();
        assert_eq!(req.current_value, ">=2.25.1");
        assert!(!req.is_dev);
        assert!(req.skip_reason.is_none());
    }

    #[test]
    fn extracts_multi_constraint() {
        let deps = extract(SAMPLE);
        let flask = deps.iter().find(|d| d.name == "flask").unwrap();
        assert_eq!(flask.current_value, ">=2.0,<3.0");
    }

    #[test]
    fn extracts_table_form() {
        let deps = extract(SAMPLE);
        let django = deps.iter().find(|d| d.name == "django").unwrap();
        assert_eq!(django.current_value, ">=4.0");
        assert!(django.skip_reason.is_none());
    }

    #[test]
    fn wildcard_skipped() {
        let deps = extract(SAMPLE);
        let unver = deps.iter().find(|d| d.name == "unversioned").unwrap();
        assert_eq!(unver.skip_reason, Some(PipfileSkipReason::Wildcard));
    }

    #[test]
    fn git_dep_skipped() {
        let deps = extract(SAMPLE);
        let mylib = deps.iter().find(|d| d.name == "mylib").unwrap();
        assert_eq!(mylib.skip_reason, Some(PipfileSkipReason::GitDependency));
    }

    #[test]
    fn local_dep_skipped() {
        let deps = extract(SAMPLE);
        let local = deps.iter().find(|d| d.name == "locallib").unwrap();
        assert_eq!(local.skip_reason, Some(PipfileSkipReason::LocalDependency));
    }

    #[test]
    fn dev_packages_flagged() {
        let deps = extract(SAMPLE);
        let pytest = deps.iter().find(|d| d.name == "pytest").unwrap();
        assert_eq!(pytest.current_value, ">=7.0");
        assert!(pytest.is_dev);
        assert!(pytest.skip_reason.is_none());
    }

    #[test]
    fn dev_wildcard_skipped() {
        let deps = extract(SAMPLE);
        let black = deps.iter().find(|d| d.name == "black").unwrap();
        assert!(black.is_dev);
        assert_eq!(black.skip_reason, Some(PipfileSkipReason::Wildcard));
    }

    #[test]
    fn normalizes_package_names() {
        let content = "[packages]\nMy_Package = \">=1.0\"\n";
        let deps = extract(content);
        assert_eq!(deps[0].name, "my-package");
    }

    #[test]
    fn invalid_toml_returns_empty() {
        assert!(extract("not valid [toml").is_empty());
    }

    #[test]
    fn empty_content_returns_no_deps() {
        assert!(extract("").is_empty());
    }
}
