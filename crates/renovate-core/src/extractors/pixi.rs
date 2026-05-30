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
    /// No channels configured — cannot determine registry URL.
    UnknownRegistry,
}

/// A single Pixi dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PixiDep {
    pub dep_name: String,
    pub current_value: String,
    pub source: PixiSource,
    pub channels: Vec<String>,
    pub registry_strategy: Option<&'static str>,
    pub skip_reason: Option<PixiSkipReason>,
}

/// Returns true if the TOML table has at least one channel configured.
fn has_channels(table: &toml::Table) -> bool {
    // Check [project.channels] or [workspace.channels]
    for section in ["project", "workspace"] {
        if let Some(Value::Table(t)) = table.get(section)
            && let Some(Value::Array(channels)) = t.get("channels")
        {
            return !channels.is_empty();
        }
    }
    true // no channel config found → don't apply unknown-registry skip
}

/// Extract all Pixi dependencies from a `pixi.toml` file.
pub fn extract(content: &str) -> Vec<PixiDep> {
    let Ok(root) = toml::from_str::<Value>(content) else {
        return Vec::new();
    };
    let Some(table) = root.as_table() else {
        return Vec::new();
    };

    let channels_present = has_channels(table);
    let base_channels = collect_base_channels(table);
    let registry_strategy = registry_strategy(table);
    let mut deps = Vec::new();

    // `[dependencies]` → Conda
    if let Some(Value::Table(conda_deps)) = table.get("dependencies") {
        for (name, spec) in conda_deps {
            let mut dep = parse_conda_dep(name, spec, &base_channels, registry_strategy);
            if !channels_present && dep.skip_reason.is_none() {
                dep.skip_reason = Some(PixiSkipReason::UnknownRegistry);
            }
            deps.push(dep);
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
                    let channels = collect_feature_channels(feat_table, &base_channels);
                    for (name, spec) in conda_deps {
                        deps.push(parse_conda_dep(name, spec, &channels, registry_strategy));
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

    let base_channels = collect_base_channels(pixi_table);
    let registry_strategy = registry_strategy(pixi_table);
    let mut deps = Vec::new();

    if let Some(Value::Table(conda_deps)) = pixi_table.get("dependencies") {
        for (name, spec) in conda_deps {
            deps.push(parse_conda_dep(
                name,
                spec,
                &base_channels,
                registry_strategy,
            ));
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
                    let channels = collect_feature_channels(feat_table, &base_channels);
                    for (name, spec) in conda_deps {
                        deps.push(parse_conda_dep(name, spec, &channels, registry_strategy));
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
            channels: Vec::new(),
            registry_strategy: None,
            skip_reason: None,
        },
        None => PixiDep {
            dep_name: name.to_owned(),
            current_value: String::new(),
            source: PixiSource::Pypi,
            channels: Vec::new(),
            registry_strategy: None,
            skip_reason: Some(PixiSkipReason::UnspecifiedVersion),
        },
    }
}

fn parse_conda_dep(
    name: &str,
    spec: &Value,
    channels: &[String],
    registry_strategy: Option<&'static str>,
) -> PixiDep {
    let version = extract_version(spec);
    PixiDep {
        dep_name: name.to_owned(),
        current_value: version.clone().unwrap_or_default(),
        source: PixiSource::Conda,
        channels: channels.to_vec(),
        registry_strategy,
        skip_reason: if version.is_none() || version.as_deref() == Some("") {
            Some(PixiSkipReason::UnspecifiedVersion)
        } else {
            None
        },
    }
}

fn registry_strategy(table: &toml::Table) -> Option<&'static str> {
    table
        .get("project")
        .or_else(|| table.get("workspace"))
        .and_then(Value::as_table)
        .and_then(|t| t.get("channel-priority"))
        .and_then(Value::as_str)
        .filter(|priority| *priority == "disabled")
        .map(|_| "merge")
}

fn collect_base_channels(table: &toml::Table) -> Vec<String> {
    ["project", "workspace"]
        .iter()
        .find_map(|section| {
            table
                .get(*section)
                .and_then(Value::as_table)
                .and_then(|t| t.get("channels"))
                .and_then(channels_from_value)
        })
        .unwrap_or_default()
}

fn collect_feature_channels(feat_table: &toml::Table, base_channels: &[String]) -> Vec<String> {
    let mut channels = feat_table
        .get("channels")
        .and_then(channels_from_value)
        .unwrap_or_default();
    channels.extend_from_slice(base_channels);
    channels
}

fn channels_from_value(value: &Value) -> Option<Vec<String>> {
    let Value::Array(values) = value else {
        return None;
    };

    let mut channels: Vec<(usize, usize, String)> = values
        .iter()
        .enumerate()
        .filter_map(|(index, value)| match value {
            Value::String(channel) => Some((usize::MAX, index, channel.clone())),
            Value::Table(table) => table.get("channel").and_then(Value::as_str).map(|channel| {
                let priority = table
                    .get("priority")
                    .and_then(Value::as_integer)
                    .and_then(|priority| usize::try_from(priority).ok())
                    .unwrap_or(usize::MAX);
                (priority, index, channel.to_owned())
            }),
            _ => None,
        })
        .collect();
    channels.sort_by_key(|(priority, index, _)| (*priority, *index));
    Some(
        channels
            .into_iter()
            .map(|(_, _, channel)| channel)
            .collect(),
    )
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

    // Rust-specific: pixi behavior test
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

    // Ported: "returns null for empty pyproject.toml" — pixi/extract.spec.ts line 145
    #[test]
    fn empty_pyproject_returns_empty() {
        assert!(extract_from_pyproject("nothing here").is_empty());
        assert!(extract_from_pyproject("").is_empty());
    }

    // Ported: "returns package of pyproject.toml tool.pixi section" — pixi/extract.spec.ts line 316
    #[test]
    fn extract_tool_pixi_section_without_lockfile() {
        let content = r#"
[tool.pixi.dependencies]
numpy = ">=1.26"
"#;
        let deps = extract_from_pyproject(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "numpy");
        assert_eq!(deps[0].current_value, ">=1.26");
        assert_eq!(deps[0].source, PixiSource::Conda);
    }

    // Ported: "returns parse non-known config file as pyproject.toml" — pixi/extract.spec.ts line 481
    #[test]
    fn non_known_file_with_tool_pixi_section() {
        let content = r#"
[tool.pixi.project]
channels = ['conda-forge']
platforms = ["osx-arm64"]

[tool.pixi.dependencies]
requests = '*'
"#;
        let deps = extract_from_pyproject(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "requests");
        assert_eq!(deps[0].current_value, "*");
        assert_eq!(deps[0].source, PixiSource::Conda);
    }

    // Ported: "returns parse non-known config file as pixi.toml" — pixi/extract.spec.ts line 509
    #[test]
    fn non_known_file_with_project_section() {
        let content = r#"
[project]
channels = ['conda-forge']
platforms = ["osx-arm64"]

[dependencies]
requests = '*'
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "requests");
        assert_eq!(deps[0].current_value, "*");
        assert_eq!(deps[0].source, PixiSource::Conda);
    }

    // Ported: "extract feature with channels" — pixi/extract.spec.ts line 538
    #[test]
    fn extract_feature_with_url_channel() {
        let content = r#"
[project]
channels = ["https://prefix.dev/conda-forge"]
name = "pixi"
platforms = ["win-64"]
version = "0.1.0"

[dependencies]
scipy = { version = "==1.15.1" }
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "scipy");
        assert_eq!(deps[0].current_value, "==1.15.1");
        assert_eq!(deps[0].source, PixiSource::Conda);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "extract package from with workspace" — pixi/extract.spec.ts line 601
    #[test]
    fn extract_from_workspace_section() {
        let content = r#"
[workspace]
channels = ["conda-forge"]

[dependencies]
scipy = { version = "==1.15.1" }
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "scipy");
        assert_eq!(deps[0].current_value, "==1.15.1");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "extract package with channel priority" — pixi/extract.spec.ts line 630
    #[test]
    fn feature_channel_priority_prepends_prioritized_channels() {
        let content = r#"
[project]
channels = ["conda-forge", "conda-not-forge"]
name = "pixi"
platforms = ["win-64"]
version = "0.1.0"

[feature.scipy]
channels = ["anaconda", { channel = "cuda", priority = 1 }, { channel = "cuda2", priority = 1 }]
dependencies = { scipy = "==1.15.1" }

[feature.numpy]
dependencies = { numpy = "==1.15.1" }
"#;
        let deps = extract(content);
        let scipy = deps.iter().find(|d| d.dep_name == "scipy").unwrap();
        assert_eq!(
            scipy.channels,
            vec![
                "cuda",
                "cuda2",
                "anaconda",
                "conda-forge",
                "conda-not-forge"
            ]
        );

        let numpy = deps.iter().find(|d| d.dep_name == "numpy").unwrap();
        assert_eq!(numpy.channels, vec!["conda-forge", "conda-not-forge"]);
    }

    // Ported: "set registryStrategy='merge' for channel-priority='disabled'" — pixi/extract.spec.ts line 685
    #[test]
    fn disabled_channel_priority_sets_merge_registry_strategy() {
        let content = r#"
[project]
channels = ["anaconda", "conda-forge"]
platforms = ["win-64"]
channel-priority = "disabled"

[dependencies]
python = "3.12.*"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].channels, vec!["anaconda", "conda-forge"]);
        assert_eq!(deps[0].registry_strategy, Some("merge"));
    }

    // Ported: "use default registryStrategy for channel-priority='strict'" — pixi/extract.spec.ts line 706
    #[test]
    fn strict_channel_priority_uses_default_registry_strategy() {
        let content = r#"
[project]
channels = ["anaconda", "conda-forge"]
platforms = ["win-64"]

[dependencies]
python = "3.12.*"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].channels, vec!["anaconda", "conda-forge"]);
        assert_eq!(deps[0].registry_strategy, None);
    }

    // Ported: "returns null for non-known config file" — pixi/extract.spec.ts line 681
    #[test]
    fn non_toml_content_returns_empty() {
        assert!(extract("{}").is_empty());
        assert!(extract_from_pyproject("{}").is_empty());
    }

    // Ported: "skip package without channels" — pixi/extract.spec.ts line 571
    #[test]
    fn skip_package_without_channels() {
        let content = r#"
[project]
name = "pixi"
channels = []

[dependencies]
scipy = { version = "==1.15.1" }
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "scipy");
        assert_eq!(deps[0].skip_reason, Some(PixiSkipReason::UnknownRegistry));
    }
}
