//! Cargo.toml dependency extractor.
//!
//! Parses a `Cargo.toml` manifest and returns the set of crate dependencies
//! with their version constraints, ready for datasource lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/cargo/extract.ts` — extraction logic
//! - `lib/modules/manager/cargo/schema.ts` — `CargoDep` / `CargoManifest` Zod schemas

use std::collections::BTreeMap;

use serde::Deserialize;
use thiserror::Error;

/// Why a dependency is being skipped (no version lookup needed).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkipReason {
    /// Dependency is a local path (`path = "../../foo"`).
    PathDependency,
    /// Dependency is sourced from git rather than a registry.
    GitSource,
    /// Dependency is inherited from `[workspace.dependencies]`.
    WorkspaceInherited,
    /// Dependency entry has no `version` and is not path/git/workspace.
    InvalidSpec,
}

/// Dependency type — which section of Cargo.toml it came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepType {
    Regular,
    Dev,
    Build,
}

impl DepType {
    /// Return the Renovate-canonical string for this dep type.
    pub fn as_renovate_str(&self) -> &'static str {
        match self {
            DepType::Regular => "dependencies",
            DepType::Dev => "devDependencies",
            DepType::Build => "buildDependencies",
        }
    }
}

/// A single extracted dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractedDep {
    /// The key name in `[dependencies]` (e.g. `"tokio"`).
    pub dep_name: String,
    /// The actual crate name — usually matches `dep_name` but overridable
    /// via the `package` field (e.g. `openssl = { package = "openssl-sys", ... }`).
    pub package_name: String,
    /// The version constraint string (e.g. `"1"`, `"^1.0"`, `">=1.0,<2"`).
    /// Empty string for skipped deps.
    pub current_value: String,
    /// Section the dep came from.
    pub dep_type: DepType,
    /// Set when the dep does not need a version lookup.
    pub skip_reason: Option<SkipReason>,
}

/// Errors from parsing a `Cargo.toml`.
#[derive(Debug, Error)]
pub enum CargoExtractError {
    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),
}

// ── Internal deserialization types ───────────────────────────────────────────

/// A dependency value: either a bare version string or a table.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RawDep {
    /// `tokio = "1.52"`
    Simple(String),
    /// `tokio = { version = "1.52", features = ["full"] }`
    Table(RawDepTable),
}

#[derive(Debug, Deserialize)]
struct RawDepTable {
    version: Option<String>,
    path: Option<String>,
    git: Option<String>,
    #[serde(rename = "package")]
    pkg: Option<String>,
    workspace: Option<bool>,
    // registry and other fields exist but are not needed for basic extraction
}

/// Minimal `Cargo.toml` representation — only the fields we need.
#[derive(Debug, Deserialize)]
struct RawManifest {
    dependencies: Option<BTreeMap<String, RawDep>>,
    #[serde(rename = "dev-dependencies")]
    dev_dependencies: Option<BTreeMap<String, RawDep>>,
    #[serde(rename = "build-dependencies")]
    build_dependencies: Option<BTreeMap<String, RawDep>>,
    workspace: Option<RawWorkspace>,
    /// Platform-conditional dependencies: `[target.'cfg(...)'.dependencies]`
    target: Option<BTreeMap<String, RawTargetDeps>>,
}

/// Platform-conditional dependency block: `[target.'cfg(...)'.dependencies]`.
#[derive(Debug, Deserialize)]
struct RawTargetDeps {
    dependencies: Option<BTreeMap<String, RawDep>>,
    #[serde(rename = "dev-dependencies")]
    dev_dependencies: Option<BTreeMap<String, RawDep>>,
    #[serde(rename = "build-dependencies")]
    build_dependencies: Option<BTreeMap<String, RawDep>>,
}

/// Workspace-level definitions (from workspace root `Cargo.toml`).
#[derive(Debug, Deserialize)]
struct RawWorkspace {
    dependencies: Option<BTreeMap<String, RawDep>>,
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Parse a `Cargo.toml` manifest string and extract all dependencies.
///
/// Returns a flat list — regular, dev, build, and workspace deps combined with
/// their respective `DepType`. The list is in deterministic order (BTreeMap
/// iteration is sorted by key).
///
/// Git, path, workspace-inherited, and spec-less dependencies are included
/// with a `skip_reason` so callers can report them without attempting a
/// version lookup.
pub fn extract(content: &str) -> Result<Vec<ExtractedDep>, CargoExtractError> {
    let manifest: RawManifest = toml::from_str(content)?;
    let mut out = Vec::new();

    for (section, dep_type) in [
        (manifest.dependencies, DepType::Regular),
        (manifest.dev_dependencies, DepType::Dev),
        (manifest.build_dependencies, DepType::Build),
    ] {
        if let Some(deps) = section {
            for (name, raw) in deps {
                out.push(convert_dep(name, raw, dep_type));
            }
        }
    }

    // Workspace root dependencies (`[workspace.dependencies]`).
    if let Some(deps) = manifest.workspace.and_then(|ws| ws.dependencies) {
        for (name, raw) in deps {
            out.push(convert_dep(name, raw, DepType::Regular));
        }
    }

    // Platform-conditional deps (`[target.'cfg(...)'.dependencies]`).
    for (_cfg, target) in manifest.target.into_iter().flatten() {
        for (section, dep_type) in [
            (target.dependencies, DepType::Regular),
            (target.dev_dependencies, DepType::Dev),
            (target.build_dependencies, DepType::Build),
        ] {
            if let Some(deps) = section {
                for (name, raw) in deps {
                    out.push(convert_dep(name, raw, dep_type));
                }
            }
        }
    }

    Ok(out)
}

fn convert_dep(name: String, raw: RawDep, dep_type: DepType) -> ExtractedDep {
    match raw {
        RawDep::Simple(version) => ExtractedDep {
            package_name: name.clone(),
            dep_name: name,
            current_value: version,
            dep_type,
            skip_reason: None,
        },
        RawDep::Table(t) => {
            let package_name = t.pkg.unwrap_or_else(|| name.clone());
            let skip_reason = if t.path.is_some() {
                Some(SkipReason::PathDependency)
            } else if t.workspace == Some(true) {
                Some(SkipReason::WorkspaceInherited)
            } else if t.git.is_some() {
                Some(SkipReason::GitSource)
            } else if t.version.is_none() {
                Some(SkipReason::InvalidSpec)
            } else {
                None
            };
            ExtractedDep {
                dep_name: name,
                package_name,
                current_value: t.version.unwrap_or_default(),
                dep_type,
                skip_reason,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_simple_string_deps() {
        let toml = r#"
[dependencies]
serde = "1.0"
tokio = "1.52"
"#;
        let deps = extract(toml).unwrap();
        let serde = deps.iter().find(|d| d.dep_name == "serde").unwrap();
        assert_eq!(serde.current_value, "1.0");
        assert_eq!(serde.dep_type, DepType::Regular);
        assert!(serde.skip_reason.is_none());
    }

    #[test]
    fn extracts_table_deps_with_version() {
        let toml = r#"
[dependencies]
tokio = { version = "1.52", features = ["full"] }
"#;
        let deps = extract(toml).unwrap();
        let tokio = deps.iter().find(|d| d.dep_name == "tokio").unwrap();
        assert_eq!(tokio.current_value, "1.52");
        assert!(tokio.skip_reason.is_none());
    }

    #[test]
    fn package_field_overrides_name() {
        let toml = r#"
[dependencies]
openssl = { package = "openssl-sys", version = "0.9" }
"#;
        let deps = extract(toml).unwrap();
        let dep = deps.iter().find(|d| d.dep_name == "openssl").unwrap();
        assert_eq!(dep.package_name, "openssl-sys");
        assert_eq!(dep.dep_name, "openssl");
        assert_eq!(dep.current_value, "0.9");
    }

    #[test]
    fn path_dep_is_skipped() {
        let toml = r#"
[dependencies]
my-lib = { path = "../my-lib" }
"#;
        let deps = extract(toml).unwrap();
        let dep = deps.iter().find(|d| d.dep_name == "my-lib").unwrap();
        assert_eq!(dep.skip_reason, Some(SkipReason::PathDependency));
    }

    #[test]
    fn workspace_dep_is_skipped() {
        let toml = r#"
[dependencies]
serde = { workspace = true }
"#;
        let deps = extract(toml).unwrap();
        let dep = deps.iter().find(|d| d.dep_name == "serde").unwrap();
        assert_eq!(dep.skip_reason, Some(SkipReason::WorkspaceInherited));
    }

    #[test]
    fn git_dep_is_skipped() {
        let toml = r#"
[dependencies]
foo = { git = "https://github.com/owner/foo", tag = "v1.0" }
"#;
        let deps = extract(toml).unwrap();
        let dep = deps.iter().find(|d| d.dep_name == "foo").unwrap();
        assert_eq!(dep.skip_reason, Some(SkipReason::GitSource));
    }

    #[test]
    fn dev_and_build_deps_have_correct_type() {
        let toml = r#"
[dev-dependencies]
criterion = "0.5"

[build-dependencies]
cc = "1.0"
"#;
        let deps = extract(toml).unwrap();
        let crit = deps.iter().find(|d| d.dep_name == "criterion").unwrap();
        let cc = deps.iter().find(|d| d.dep_name == "cc").unwrap();
        assert_eq!(crit.dep_type, DepType::Dev);
        assert_eq!(cc.dep_type, DepType::Build);
    }

    #[test]
    fn empty_manifest_returns_empty_list() {
        let toml = r#"
[package]
name = "my-crate"
version = "0.1.0"
"#;
        let deps = extract(toml).unwrap();
        assert!(deps.is_empty());
    }

    #[test]
    fn version_constraint_forms_are_preserved() {
        let toml = r#"
[dependencies]
a = "^1.0"
b = ">=1.0,<2"
c = "~1.2.3"
d = "*"
"#;
        let deps = extract(toml).unwrap();
        let a = deps.iter().find(|d| d.dep_name == "a").unwrap();
        let b = deps.iter().find(|d| d.dep_name == "b").unwrap();
        assert_eq!(a.current_value, "^1.0");
        assert_eq!(b.current_value, ">=1.0,<2");
    }

    #[test]
    fn mixed_manifest_extracts_all_sections() {
        let toml = r#"
[dependencies]
serde = "1"
tokio = { version = "1.52", features = ["full"] }
my-lib = { path = "../my-lib" }

[dev-dependencies]
criterion = "0.5"
"#;
        let deps = extract(toml).unwrap();
        assert_eq!(deps.len(), 4);
        assert_eq!(deps.iter().filter(|d| d.skip_reason.is_none()).count(), 3); // serde, tokio, criterion
    }

    #[test]
    fn workspace_dependencies_extracted() {
        let toml = r#"
[workspace]
members = ["crates/*"]

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = "1.35"
anyhow = { version = "1.0", path = "../anyhow" }
"#;
        let deps = extract(toml).unwrap();
        // serde (table with version) + tokio (simple) + anyhow (path, skipped)
        assert_eq!(deps.len(), 3);
        let serde = deps.iter().find(|d| d.dep_name == "serde").unwrap();
        assert_eq!(serde.current_value, "1.0");
        assert!(serde.skip_reason.is_none());

        let tokio = deps.iter().find(|d| d.dep_name == "tokio").unwrap();
        assert_eq!(tokio.current_value, "1.35");

        let anyhow = deps.iter().find(|d| d.dep_name == "anyhow").unwrap();
        assert_eq!(anyhow.skip_reason, Some(SkipReason::PathDependency));
    }

    #[test]
    fn workspace_and_member_deps_both_extracted() {
        let toml = r#"
[workspace.dependencies]
serde = "1.0"

[dependencies]
tokio = "1.35"
"#;
        let deps = extract(toml).unwrap();
        assert!(deps.iter().any(|d| d.dep_name == "serde"));
        assert!(deps.iter().any(|d| d.dep_name == "tokio"));
    }

    #[test]
    fn target_cfg_dependencies_extracted() {
        let toml = r#"
[dependencies]
serde = "1.0"

[target.'cfg(windows)'.dependencies]
winapi = { version = "0.3", features = ["winsock2"] }

[target.'cfg(unix)'.dev-dependencies]
libc = "0.2"
"#;
        let deps = extract(toml).unwrap();
        assert!(deps.iter().any(|d| d.dep_name == "serde"));
        let winapi = deps.iter().find(|d| d.dep_name == "winapi").unwrap();
        assert_eq!(winapi.current_value, "0.3");
        assert_eq!(winapi.dep_type, DepType::Regular);
        let libc = deps.iter().find(|d| d.dep_name == "libc").unwrap();
        assert_eq!(libc.current_value, "0.2");
        assert_eq!(libc.dep_type, DepType::Dev);
    }

    #[test]
    fn renamed_dep_extracts_original_package_name() {
        // Ported: "extracts original package name of renamed dependencies" — cargo/extract.spec.ts line 539
        let toml =
            "[dependencies]\nboolector-solver = { package = \"boolector\", version = \"0.4.0\" }";
        let deps = extract(toml).unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "boolector-solver");
        assert_eq!(deps[0].package_name, "boolector");
        assert_eq!(deps[0].current_value, "0.4.0");
    }

    #[test]
    fn empty_dev_dependencies_returns_empty() {
        // Ported: "returns null for empty dev-dependencies" — cargo/extract.spec.ts line 59
        let toml = "[dev-dependencies]";
        let deps = extract(toml).unwrap();
        assert!(deps.is_empty());
    }

    #[test]
    fn empty_dependencies_section_returns_empty() {
        // Ported: "returns null for empty dependencies" — cargo/extract.spec.ts line 52
        let toml = "[dependencies]\n";
        let deps = extract(toml).unwrap();
        assert!(deps.is_empty());
    }

    #[test]
    fn empty_custom_target_returns_empty() {
        // Ported: "returns null for empty custom target" — cargo/extract.spec.ts line 66
        let toml = "[target.'cfg(windows)'.dependencies]";
        let deps = extract(toml).unwrap();
        assert!(deps.is_empty());
    }

    #[test]
    fn invalid_toml_returns_error() {
        // Ported: "returns null for invalid toml" — cargo/extract.spec.ts line 46
        assert!(extract("invalid toml [[[").is_err());
    }

    #[test]
    fn workspace_true_dep_gets_inherited_skip_reason() {
        // Ported: "skips workspace dependency" — cargo/extract.spec.ts line 390
        let toml = "[dependencies]\nfoobar = { workspace = true }";
        let deps = extract(toml).unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "foobar");
        assert_eq!(deps[0].skip_reason, Some(SkipReason::WorkspaceInherited));
    }
}
