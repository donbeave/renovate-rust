//! Pixi `pixi.toml` dependency extractor.
//!
//! Extracts PyPI and Conda dependencies from Pixi project files.
//! PyPI deps use the PyPI datasource; Conda deps use the Anaconda datasource.
//!
//! Renovate reference:
//! - `lib/modules/manager/pixi/extract.ts`
//! - `lib/modules/manager/pixi/schema.ts`
//! - Patterns: `(^|/)pixi\.toml$`, `(^|/)pyproject\.toml$` (`[tool.pixi]`)
//! - Datasources: `pypi`, `conda`
//!
//! ## File formats
//!
//! ### pixi.toml
//!
//! ```toml
//! [dependencies]
//! numpy = ">=1.26"          # Conda dep — skipped
//!
//! [pypi-dependencies]
//! pandas = ">=2.2"          # PyPI dep — actionable
//! requests = { version = ">=2.31" }
//!
//! [feature.gpu.pypi-dependencies]
//! torch = ">=2.0"           # Feature PyPI dep — actionable
//! ```

use toml::Value;

/// Source for a Pixi dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PixiSource {
    /// PyPI package (via `pypi-dependencies`).
    Pypi,
    /// Conda package (via `dependencies`).
    Conda,
}

/// Skip reason for a Pixi dep that cannot be looked up.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PixiSkipReason {
    /// Version string could not be parsed.
    InvalidVersion,
    /// No version specified (e.g. path or git dependency).
    UnspecifiedVersion,
}

/// A single Pixi dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PixiDep {
    pub dep_name: String,
    pub current_value: String,
    pub source: PixiSource,
    pub skip_reason: Option<PixiSkipReason>,
}

/// Extract all Pixi dependencies from a `pixi.toml` file.
pub fn extract(content: &str) -> Vec<PixiDep> {
    let Ok(root) = toml::from_str::<Value>(content) else {
        return Vec::new();
    };
    let Some(table) = root.as_table() else {
        return Vec::new();
    };

    let mut deps = Vec::new();

    // `[dependencies]` → Conda (skipped)
    if let Some(Value::Table(conda_deps)) = table.get("dependencies") {
        for (name, spec) in conda_deps {
            deps.push(parse_conda_dep(name, spec));
        }
    }

    // `[pypi-dependencies]` → PyPI (actionable)
    if let Some(Value::Table(pypi_deps)) = table.get("pypi-dependencies") {
        for (name, spec) in pypi_deps {
            deps.push(parse_pypi_dep(name, spec));
        }
    }

    // `[feature.*.dependencies]` and `[feature.*.pypi-dependencies]`
    if let Some(Value::Table(features)) = table.get("feature") {
        for (_feat_name, feat_val) in features {
            if let Some(Value::Table(feat_table)) = Some(feat_val) {
                if let Some(Value::Table(conda_deps)) = feat_table.get("dependencies") {
                    for (name, spec) in conda_deps {
                        deps.push(parse_conda_dep(name, spec));
                    }
                }
                if let Some(Value::Table(pypi_deps)) = feat_table.get("pypi-dependencies") {
                    for (name, spec) in pypi_deps {
                        deps.push(parse_pypi_dep(name, spec));
                    }
                }
            }
        }
    }

    deps
}

/// Extract Pixi dependencies from a `pyproject.toml` `[tool.pixi]` section.
pub fn extract_from_pyproject(content: &str) -> Vec<PixiDep> {
    let Ok(root) = toml::from_str::<Value>(content) else {
        return Vec::new();
    };

    let pixi_table = root
        .get("tool")
        .and_then(|t| t.get("pixi"))
        .and_then(|v| v.as_table());

    let Some(pixi_table) = pixi_table else {
        return Vec::new();
    };

    let mut deps = Vec::new();

    if let Some(Value::Table(conda_deps)) = pixi_table.get("dependencies") {
        for (name, spec) in conda_deps {
            deps.push(parse_conda_dep(name, spec));
        }
    }

    if let Some(Value::Table(pypi_deps)) = pixi_table.get("pypi-dependencies") {
        for (name, spec) in pypi_deps {
            deps.push(parse_pypi_dep(name, spec));
        }
    }

    if let Some(Value::Table(features)) = pixi_table.get("feature") {
        for (_feat_name, feat_val) in features {
            if let Some(Value::Table(feat_table)) = Some(feat_val) {
                if let Some(Value::Table(conda_deps)) = feat_table.get("dependencies") {
                    for (name, spec) in conda_deps {
                        deps.push(parse_conda_dep(name, spec));
                    }
                }
                if let Some(Value::Table(pypi_deps)) = feat_table.get("pypi-dependencies") {
                    for (name, spec) in pypi_deps {
                        deps.push(parse_pypi_dep(name, spec));
                    }
                }
            }
        }
    }

    deps
}

fn parse_pypi_dep(name: &str, spec: &Value) -> PixiDep {
    let version = extract_version(spec);
    match version {
        Some(v) => PixiDep {
            dep_name: name.to_owned(),
            current_value: v,
            source: PixiSource::Pypi,
            skip_reason: None,
        },
        None => PixiDep {
            dep_name: name.to_owned(),
            current_value: String::new(),
            source: PixiSource::Pypi,
            skip_reason: Some(PixiSkipReason::UnspecifiedVersion),
        },
    }
}

fn parse_conda_dep(name: &str, spec: &Value) -> PixiDep {
    let version = extract_version(spec);
    PixiDep {
        dep_name: name.to_owned(),
        current_value: version.clone().unwrap_or_default(),
        source: PixiSource::Conda,
        skip_reason: if version.is_none() || version.as_deref() == Some("") {
            Some(PixiSkipReason::UnspecifiedVersion)
        } else {
            None
        },
    }
}

/// Extract a version string from a Pixi dep spec.
///
/// Accepts:
/// - plain string: `">=1.0"`
/// - table with `version` key: `{ version = ">=1.0" }`
fn extract_version(spec: &Value) -> Option<String> {
    match spec {
        Value::String(s) => Some(s.clone()),
        Value::Table(t) => t.get("version").and_then(|v| v.as_str()).map(str::to_owned),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "returns parse pixi.toml" — pixi/extract.spec.ts line 161
    #[test]
    fn extracts_pypi_deps() {
        let content = r#"
[project]
name = "test"

[pypi-dependencies]
pandas = ">=2.2"
requests = { version = ">=2.31" }
"#;
        let deps = extract(content);
        let pypi: Vec<_> = deps
            .iter()
            .filter(|d| d.source == PixiSource::Pypi)
            .collect();
        assert_eq!(pypi.len(), 2);
        assert!(
            pypi.iter()
                .any(|d| d.dep_name == "pandas" && d.current_value == ">=2.2")
        );
        assert!(
            pypi.iter()
                .any(|d| d.dep_name == "requests" && d.current_value == ">=2.31")
        );
        for d in &pypi {
            assert!(d.skip_reason.is_none());
        }
    }

    // Ported: "returns parse pixi.toml" — pixi/extract.spec.ts line 161
    #[test]
    fn extracts_conda_deps_as_actionable() {
        let content = r#"
[dependencies]
numpy = ">=1.26"
python = ">=3.9"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        for d in &deps {
            assert_eq!(d.source, PixiSource::Conda);
            assert!(
                d.skip_reason.is_none(),
                "conda deps should now be actionable"
            );
        }
    }

    // Ported: "returns parse pixi.toml with features" — pixi/extract.spec.ts line 335
    #[test]
    fn extracts_feature_pypi_deps() {
        let content = r#"
[pypi-dependencies]
pandas = ">=2.2"

[feature.gpu.pypi-dependencies]
torch = ">=2.0"
"#;
        let deps = extract(content);
        let pypi: Vec<_> = deps
            .iter()
            .filter(|d| d.source == PixiSource::Pypi)
            .collect();
        assert_eq!(pypi.len(), 2);
        assert!(pypi.iter().any(|d| d.dep_name == "torch"));
    }

    // Ported: "returns parse pixi section from pyproject.toml" — pixi/extract.spec.ts line 297
    #[test]
    fn extract_from_pyproject_tool_pixi() {
        let content = r#"
[tool.pixi.pypi-dependencies]
requests = ">=2.31"

[tool.pixi.dependencies]
numpy = ">=1.26"
"#;
        let deps = extract_from_pyproject(content);
        assert_eq!(deps.len(), 2);
        let pypi: Vec<_> = deps
            .iter()
            .filter(|d| d.source == PixiSource::Pypi)
            .collect();
        assert_eq!(pypi.len(), 1);
        assert_eq!(pypi[0].dep_name, "requests");
    }

    // Ported: "returns null for empty pixi.toml" — pixi/extract.spec.ts line 151
    #[test]
    fn empty_file_returns_empty() {
        assert!(extract("").is_empty());
        assert!(extract("nothing here").is_empty());
    }

    // Ported: "returns null for parsed file without pixi section" — pixi/extract.spec.ts line 155
    #[test]
    fn file_without_pixi_section_returns_empty() {
        let content = "[project]\nname = \"myapp\"\n";
        assert!(extract(content).is_empty());
        assert!(extract_from_pyproject(content).is_empty());
    }

    #[test]
    fn git_dep_has_unspecified_skip() {
        let content = r#"
[pypi-dependencies]
mylib = { git = "https://github.com/foo/mylib" }
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(PixiSkipReason::UnspecifiedVersion)
        );
    }
}
