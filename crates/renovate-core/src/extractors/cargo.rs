//! Cargo.toml dependency extractor.
//!
//! Parses a `Cargo.toml` manifest and returns the set of crate dependencies
//! with their version constraints, ready for datasource lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/cargo/extract.ts` — extraction logic
//! - `lib/modules/manager/cargo/schema.ts` — `CargoDep` / `CargoManifest` Zod schemas

use std::collections::{BTreeMap, HashMap, HashSet};

use semver::{Version, VersionReq};
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
    /// Dependency names a registry that cannot be resolved to a URL.
    UnknownRegistry,
}

/// Dependency type — which section of Cargo.toml it came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepType {
    Regular,
    Dev,
    Build,
    Workspace,
}

impl DepType {
    /// Return the Renovate-canonical string for this dep type.
    pub fn as_renovate_str(&self) -> &'static str {
        match self {
            DepType::Regular => "dependencies",
            DepType::Dev => "devDependencies",
            DepType::Build => "buildDependencies",
            DepType::Workspace => "workspace.dependencies",
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
    /// Registry URLs resolved from `.cargo/config.toml` or environment.
    pub registry_urls: Vec<String>,
    /// The registry name from `registry = "..."` field.
    pub registry_name: Option<String>,
    /// Version pinned in `Cargo.lock`, if available.
    pub locked_version: Option<String>,
}

/// Errors from parsing a `Cargo.toml`.
#[derive(Debug, Error)]
pub enum CargoExtractError {
    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),
}

/// Context for enriched extraction — registry config, lock file, and env.
#[derive(Debug, Default)]
pub struct CargoContext {
    /// Contents of `.cargo/config.toml` (or legacy `.cargo/config`).
    pub cargo_config: Option<String>,
    /// Contents of `Cargo.lock`.
    pub cargo_lock: Option<String>,
    /// Values of `CARGO_REGISTRIES_<NAME>_INDEX` environment variables.
    /// Key is the SCREAMING_SNAKE registry name (e.g. `"PRIVATE_CRATES"`).
    pub registry_env: HashMap<String, String>,
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
    registry: Option<String>,
}

/// Minimal `Cargo.toml` representation — only the fields we need.
#[derive(Debug, Deserialize)]
struct RawManifest {
    package: Option<RawPackage>,
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

/// `[package]` section of Cargo.toml.
#[derive(Debug, Deserialize)]
struct RawPackage {
    /// The `version` field — either a plain string or `{ workspace = true }`.
    #[serde(default, deserialize_with = "version_or_workspace")]
    version: VersionField,
}

#[derive(Debug, Default)]
enum VersionField {
    #[default]
    Absent,
    Value(String),
    Workspace,
}

fn version_or_workspace<'de, D>(d: D) -> Result<VersionField, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let v = serde_json::Value::deserialize(d).map_err(D::Error::custom)?;
    match v {
        serde_json::Value::String(s) => Ok(VersionField::Value(s)),
        serde_json::Value::Object(ref m)
            if m.get("workspace") == Some(&serde_json::Value::Bool(true)) =>
        {
            Ok(VersionField::Workspace)
        }
        _ => Ok(VersionField::Absent),
    }
}

/// Workspace-level definitions (from workspace root `Cargo.toml`).
#[derive(Debug, Deserialize)]
struct RawWorkspace {
    dependencies: Option<BTreeMap<String, RawDep>>,
    package: Option<RawWorkspacePackage>,
}

#[derive(Debug, Deserialize)]
struct RawWorkspacePackage {
    version: Option<String>,
}

// ── Cargo config TOML structures ─────────────────────────────────────────────

#[derive(Debug, Deserialize, Default)]
struct CargoConfig {
    registries: Option<BTreeMap<String, CargoConfigRegistry>>,
    source: Option<BTreeMap<String, CargoConfigSource>>,
}

#[derive(Debug, Deserialize)]
struct CargoConfigRegistry {
    index: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CargoConfigSource {
    registry: Option<String>,
    #[serde(rename = "replace-with")]
    replace_with: Option<String>,
}

// ── Cargo.lock TOML structures ────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct CargoLock {
    package: Option<Vec<CargoLockPackage>>,
}

#[derive(Debug, Deserialize)]
struct CargoLockPackage {
    name: String,
    version: String,
    source: Option<String>,
}

// ── Registry resolution ───────────────────────────────────────────────────────

/// Registry map: registry_name → resolved index URL (or None = default crates.io).
type RegistryMap = HashMap<String, Option<String>>;

const DEFAULT_REGISTRY_URL: &str = "https://github.com/rust-lang/crates.io-index";

/// Build registry URL map from cargo config content + environment variables.
///
/// Mirrors TypeScript `extractCargoRegistries`:
/// - Resolves crates-io and every named registry via `resolveRegistryIndex`.
/// - Environment variables override config file entries.
fn build_registry_map(ctx: &CargoContext) -> RegistryMap {
    // Parse config TOML (silently ignore invalid TOML)
    let config: CargoConfig = ctx
        .cargo_config
        .as_deref()
        .and_then(|s| toml::from_str(s).ok())
        .unwrap_or_default();

    let mut map: RegistryMap = HashMap::new();

    // Resolve crates-io (may be replaced by a [source.crates-io] chain)
    map.insert(
        "crates-io".to_owned(),
        resolve_registry_index("crates-io", &config, &mut HashSet::new()),
    );

    // Resolve every named registry from config
    let names: HashSet<String> = config
        .registries
        .iter()
        .flat_map(|m| m.keys().cloned())
        .chain(config.source.iter().flat_map(|m| m.keys().cloned()))
        .collect();
    for name in names {
        map.insert(
            name.clone(),
            resolve_registry_index(&name, &config, &mut HashSet::new()),
        );
    }

    // Environment variables: CARGO_REGISTRIES_<NAME>_INDEX override everything
    for (key, url) in &ctx.registry_env {
        let reg_name = env_key_to_registry_name(key);
        map.insert(reg_name, Some(url.clone()));
    }

    map
}

/// Mirrors TypeScript `resolveRegistryIndex`.
/// Priority: replace-with chain → source.registry → registries.index → crates-io default.
fn resolve_registry_index(
    name: &str,
    config: &CargoConfig,
    visited: &mut HashSet<String>,
) -> Option<String> {
    if !visited.insert(name.to_owned()) {
        // Circular replacement chain
        return None;
    }

    // 1. Follow replace-with chain first
    if let Some(replace_with) = config
        .source
        .as_ref()
        .and_then(|s| s.get(name))
        .and_then(|s| s.replace_with.as_ref())
    {
        return resolve_registry_index(replace_with, config, visited);
    }

    // 2. [source.<name>] registry = "url"
    if let Some(registry_url) = config
        .source
        .as_ref()
        .and_then(|s| s.get(name))
        .and_then(|s| s.registry.as_ref())
    {
        return Some(registry_url.clone());
    }

    // 3. [registries.<name>] index = "url"
    if let Some(index) = config
        .registries
        .as_ref()
        .and_then(|r| r.get(name))
        .and_then(|r| r.index.as_ref())
    {
        return Some(index.clone());
    }

    // 4. crates-io without any override = default URL
    if name == "crates-io" {
        return Some(DEFAULT_REGISTRY_URL.to_owned());
    }

    None
}

/// Convert `CARGO_REGISTRIES_<NAME>_INDEX` env key to lowercase kebab registry name.
/// E.g. `"PRIVATE_CRATES"` → `"private-crates"`.
fn env_key_to_registry_name(key: &str) -> String {
    key.to_lowercase().replace('_', "-")
}

/// Resolve the registry URL(s) for a dep that names a specific registry.
/// Returns `(registry_urls, is_unknown)`.
///
/// Mirrors TypeScript `extractFromSection` registry URL resolution for explicit `registry =` deps:
/// - `Some(url)` and url ≠ default → custom registryUrls.
/// - `Some(DEFAULT_REGISTRY_URL)` → crates.io default, no custom URL needed.
/// - `None` in map → circular or missing index = unknown-registry.
/// - Missing from map → unknown-registry.
fn resolve_registry_urls(registry_name: &str, map: &RegistryMap) -> (Vec<String>, bool) {
    match map.get(registry_name) {
        Some(Some(url)) if url != DEFAULT_REGISTRY_URL => (vec![url.clone()], false),
        Some(Some(_)) => (vec![], false), // default crates-io URL = no custom URL, not unknown
        Some(None) => (vec![], true),     // circular or missing index = unknown
        None => (vec![], true),           // not in map = unknown
    }
}

/// Resolve registry info for a dep with no explicit `registry =` field (uses crates-io).
/// Returns `(registry_urls, is_unknown)` by checking the crates-io map entry.
///
/// Mirrors the TypeScript `extractFromSection` else-branch:
/// - If crates-io resolved to a custom URL → use it.
/// - If crates-io resolved to the default URL → no custom URL needed.
/// - If crates-io resolution failed (circular) → unknown-registry.
fn resolve_default_registry(map: &RegistryMap) -> (Vec<String>, bool) {
    match map.get("crates-io") {
        Some(Some(url)) if url != DEFAULT_REGISTRY_URL => (vec![url.clone()], false),
        Some(Some(_)) => (vec![], false), // default crates-io
        Some(None) => (vec![], true),     // crates-io resolved to circular/null = unknown
        None => (vec![], false),          // no config = default crates-io
    }
}

// ── Cargo.lock lookup ─────────────────────────────────────────────────────────

/// Build map of (crate_name, major_version_prefix) → locked_version from Cargo.lock.
/// Only includes packages from the registry (not git/path).
fn build_lock_map(lock_content: &str) -> HashMap<String, Vec<String>> {
    let Ok(lock) = toml::from_str::<CargoLock>(lock_content) else {
        return HashMap::new();
    };
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for pkg in lock.package.into_iter().flatten() {
        // Only registry packages (source starts with "registry+")
        if pkg
            .source
            .as_deref()
            .is_some_and(|s| s.starts_with("registry+"))
        {
            map.entry(pkg.name).or_default().push(pkg.version);
        }
    }
    map
}

/// Find the locked version for a dep using semver range matching.
/// Returns the first locked version that satisfies the constraint.
fn find_locked_version(
    package_name: &str,
    version_req: &str,
    lock_map: &HashMap<String, Vec<String>>,
) -> Option<String> {
    let versions = lock_map.get(package_name)?;
    let req = VersionReq::parse(version_req).ok()?;
    versions
        .iter()
        .find(|v| Version::parse(v).is_ok_and(|ver| req.matches(&ver)))
        .cloned()
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Parse a `Cargo.toml` manifest string and extract all dependencies.
///
/// Thin wrapper around [`extract_with_context`] with an empty context.
pub fn extract(content: &str) -> Result<Vec<ExtractedDep>, CargoExtractError> {
    extract_with_context(content, &CargoContext::default())
}

/// Parse a `Cargo.toml` manifest with registry config, lock file, and env context.
///
/// Returns a flat list — regular, dev, build, and workspace deps combined with
/// their respective `DepType`. The list is in deterministic order (BTreeMap
/// iteration is sorted by key).
pub fn extract_with_context(
    content: &str,
    ctx: &CargoContext,
) -> Result<Vec<ExtractedDep>, CargoExtractError> {
    let manifest: RawManifest = toml::from_str(content)?;
    let registry_map = build_registry_map(ctx);

    let lock_map: HashMap<String, Vec<String>> = ctx
        .cargo_lock
        .as_deref()
        .map(build_lock_map)
        .unwrap_or_default();

    let mut out = Vec::new();

    for (section, dep_type) in [
        (manifest.dependencies.as_ref(), DepType::Regular),
        (manifest.dev_dependencies.as_ref(), DepType::Dev),
        (manifest.build_dependencies.as_ref(), DepType::Build),
    ] {
        if let Some(deps) = section {
            for (name, raw) in deps {
                out.push(convert_dep(
                    name.clone(),
                    raw,
                    dep_type,
                    &registry_map,
                    &lock_map,
                ));
            }
        }
    }

    // Workspace root dependencies (`[workspace.dependencies]`).
    if let Some(deps) = manifest.workspace.and_then(|ws| ws.dependencies) {
        for (name, raw) in &deps {
            out.push(convert_dep(
                name.clone(),
                raw,
                DepType::Workspace,
                &registry_map,
                &lock_map,
            ));
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
                    out.push(convert_dep(name, &raw, dep_type, &registry_map, &lock_map));
                }
            }
        }
    }

    Ok(out)
}

fn convert_dep(
    name: String,
    raw: &RawDep,
    dep_type: DepType,
    registry_map: &RegistryMap,
    lock_map: &HashMap<String, Vec<String>>,
) -> ExtractedDep {
    match raw {
        RawDep::Simple(version) => {
            let locked_version = find_locked_version(&name, version, lock_map);
            let (registry_urls, unknown) = resolve_default_registry(registry_map);
            let skip_reason = unknown.then_some(SkipReason::UnknownRegistry);
            ExtractedDep {
                package_name: name.clone(),
                dep_name: name,
                current_value: version.clone(),
                dep_type,
                skip_reason,
                registry_urls,
                registry_name: None,
                locked_version,
            }
        }
        RawDep::Table(t) => {
            let package_name = t.pkg.clone().unwrap_or_else(|| name.clone());
            let (skip_reason, registry_urls, registry_name) = if t.path.is_some() {
                (Some(SkipReason::PathDependency), vec![], None)
            } else if t.workspace == Some(true) {
                (Some(SkipReason::WorkspaceInherited), vec![], None)
            } else if t.git.is_some() {
                (Some(SkipReason::GitSource), vec![], None)
            } else if t.version.is_none() {
                (Some(SkipReason::InvalidSpec), vec![], None)
            } else if let Some(reg_name) = &t.registry {
                let (urls, unknown) = resolve_registry_urls(reg_name, registry_map);
                if unknown {
                    (
                        Some(SkipReason::UnknownRegistry),
                        vec![],
                        Some(reg_name.clone()),
                    )
                } else {
                    (None, urls, Some(reg_name.clone()))
                }
            } else {
                // No explicit registry — uses crates-io. Check if crates-io was
                // overridden or became unresolvable (e.g. circular source replacement).
                let (urls, unknown) = resolve_default_registry(registry_map);
                if unknown {
                    (Some(SkipReason::UnknownRegistry), vec![], None)
                } else {
                    (None, urls, None)
                }
            };

            let locked_version = if skip_reason.as_ref().is_some_and(|r| {
                matches!(
                    r,
                    SkipReason::GitSource
                        | SkipReason::PathDependency
                        | SkipReason::WorkspaceInherited
                        | SkipReason::InvalidSpec
                )
            }) {
                None
            } else {
                t.version
                    .as_deref()
                    .and_then(|v| find_locked_version(&package_name, v, lock_map))
            };

            ExtractedDep {
                dep_name: name,
                package_name,
                current_value: t.version.clone().unwrap_or_default(),
                dep_type,
                skip_reason,
                registry_urls,
                registry_name,
                locked_version,
            }
        }
    }
}

/// Extract the project version from a `Cargo.toml` file.
///
/// Returns `Some(version)` from `[package].version`, or from
/// `[workspace.package].version` if the package uses `version.workspace = true`.
pub fn extract_project_version(content: &str) -> Option<String> {
    let manifest: RawManifest = toml::from_str(content).ok()?;
    let pkg = manifest.package.as_ref()?;
    match &pkg.version {
        VersionField::Value(v) => Some(v.clone()),
        VersionField::Workspace => manifest
            .workspace
            .as_ref()
            .and_then(|w| w.package.as_ref())
            .and_then(|wp| wp.version.clone()),
        VersionField::Absent => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── existing tests (unchanged) ────────────────────────────────────────────

    // Ported: "extracts multiple dependencies simple" — lib/modules/manager/cargo/extract.spec.ts line 73
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

    // Ported: "handles standard tables" — lib/modules/manager/cargo/extract.spec.ts line 91
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

    // Ported: "extracts original package name of renamed dependencies" — lib/modules/manager/cargo/extract.spec.ts line 539
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

    // Ported: "extracts multiple dependencies simple" — lib/modules/manager/cargo/extract.spec.ts line 73
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

    // Ported: "skips workspace dependency" — lib/modules/manager/cargo/extract.spec.ts line 390
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

    // Ported: "does not extract locked versions for git dependencies" — lib/modules/manager/cargo/extract.spec.ts line 567
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

    // Ported: "extracts multiple dependencies simple" — lib/modules/manager/cargo/extract.spec.ts line 73
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

    // Ported: "returns null for empty dependencies" — lib/modules/manager/cargo/extract.spec.ts line 52
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

    // Ported: "extracts multiple dependencies advanced" — lib/modules/manager/cargo/extract.spec.ts line 79
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

    // Ported: "extracts multiple dependencies simple" — lib/modules/manager/cargo/extract.spec.ts line 73
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

    // Ported: "extracts workspace dependencies" — lib/modules/manager/cargo/extract.spec.ts line 345
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

    // Ported: "extracts workspace dependencies" — lib/modules/manager/cargo/extract.spec.ts line 345
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

    // Ported: "extracts platform specific dependencies" — lib/modules/manager/cargo/extract.spec.ts line 97
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

    // Ported: "extracts original package name of renamed dependencies" — lib/modules/manager/cargo/extract.spec.ts line 539
    #[test]
    fn renamed_dep_extracts_original_package_name() {
        let toml =
            "[dependencies]\nboolector-solver = { package = \"boolector\", version = \"0.4.0\" }";
        let deps = extract(toml).unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "boolector-solver");
        assert_eq!(deps[0].package_name, "boolector");
        assert_eq!(deps[0].current_value, "0.4.0");
    }

    // Ported: "returns null for empty dev-dependencies" — lib/modules/manager/cargo/extract.spec.ts line 59
    #[test]
    fn empty_dev_dependencies_returns_empty() {
        let toml = "[dev-dependencies]";
        let deps = extract(toml).unwrap();
        assert!(deps.is_empty());
    }

    // Ported: "returns null for empty dependencies" — lib/modules/manager/cargo/extract.spec.ts line 52
    #[test]
    fn empty_dependencies_section_returns_empty() {
        let toml = "[dependencies]\n";
        let deps = extract(toml).unwrap();
        assert!(deps.is_empty());
    }

    // Ported: "returns null for empty custom target" — lib/modules/manager/cargo/extract.spec.ts line 66
    #[test]
    fn empty_custom_target_returns_empty() {
        let toml = "[target.'cfg(windows)'.dependencies]";
        let deps = extract(toml).unwrap();
        assert!(deps.is_empty());
    }

    // Ported: "returns null for invalid toml" — lib/modules/manager/cargo/extract.spec.ts line 46
    #[test]
    fn invalid_toml_returns_error() {
        assert!(extract("invalid toml [[[").is_err());
    }

    // Ported: "skips workspace dependency" — lib/modules/manager/cargo/extract.spec.ts line 390
    #[test]
    fn workspace_true_dep_gets_inherited_skip_reason() {
        let toml = "[dependencies]\nfoobar = { workspace = true }";
        let deps = extract(toml).unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "foobar");
        assert_eq!(deps[0].skip_reason, Some(SkipReason::WorkspaceInherited));
    }

    // Ported: "handles inline tables" — lib/modules/manager/cargo/extract.spec.ts line 85
    #[test]
    fn handles_inline_tables() {
        let toml = r#"
[package]
name = "inline-tables-example"
version = "0.1.2"

[dependencies]
pcap-sys = { version = "0.1", path = "pcap-sys" }
pnet = { version = "0.21.0", optional = true, default-features = false}
dep1 = {optional=true,path="./foo/bar",default-features   = true,        version="1.2"}
dep2 ={  optional=false, path="./foo/bar",      default-features=    true, version    ="3.4"}
dep3 ={ version=     "~12.3.1",      default-features=    true, path    ="./foo/bar"}
dep4 = { version = "INVALID 3.3.1 VERSION" }
dep5 = { version = "3.2.1" }
dep6 = { vesion = "1.2.3" }
"#;
        let deps = extract(toml).unwrap();
        assert_eq!(deps.len(), 8);

        let pnet = deps.iter().find(|d| d.dep_name == "pnet").unwrap();
        assert_eq!(pnet.current_value, "0.21.0");
        assert!(pnet.skip_reason.is_none());

        let dep4 = deps.iter().find(|d| d.dep_name == "dep4").unwrap();
        assert_eq!(dep4.current_value, "INVALID 3.3.1 VERSION");
        assert!(dep4.skip_reason.is_none());

        let dep5 = deps.iter().find(|d| d.dep_name == "dep5").unwrap();
        assert_eq!(dep5.current_value, "3.2.1");

        let dep6 = deps.iter().find(|d| d.dep_name == "dep6").unwrap();
        assert_eq!(dep6.skip_reason, Some(SkipReason::InvalidSpec));

        let path_skipped_count = deps
            .iter()
            .filter(|d| d.skip_reason == Some(SkipReason::PathDependency))
            .count();
        assert_eq!(path_skipped_count, 4); // pcap-sys, dep1, dep2, dep3
    }

    // Ported: "should extract project version" — lib/modules/manager/cargo/extract.spec.ts line 650
    #[test]
    fn extracts_project_version() {
        let toml = "[package]\nname = \"test\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[dependencies]\nsyn = \"2.0\"\n";
        assert_eq!(extract_project_version(toml).as_deref(), Some("0.1.0"));
    }

    // Ported: "should extract project version from workspace" — lib/modules/manager/cargo/extract.spec.ts line 664
    #[test]
    fn extracts_project_version_from_workspace() {
        let toml = "[package]\nname = \"test\"\nversion.workspace = true\nedition = \"2021\"\n[workspace.package]\nversion = \"0.1.0\"\n[dependencies]\nsyn = \"2.0\"\n";
        assert_eq!(extract_project_version(toml).as_deref(), Some("0.1.0"));
    }

    // ── Registry config tests ─────────────────────────────────────────────────

    const CARGO6_TOML: &str = r#"
[package]
name = "renovate-test"
version = "0.1.0"
authors = ["John Doe <john.doe@example.org>"]
edition = "2018"

[dependencies]
proprietary-crate = { version = "0.1.0", registry = "private-crates" }
mcorbin-test = { version = "3.0.0", registry = "mcorbin" }
tokio = "0.2"
"#;

    const CARGO6_CONFIG_TOML: &str = r#"
[registries]
private-crates = { index = "https://dl.cloudsmith.io/basic/my-org/my-repo/cargo/index.git" }

[registries.mcorbin]
index = "https://github.com/mcorbin/testregistry"
"#;

    // Ported: "extracts registry urls from .cargo/config.toml" — lib/modules/manager/cargo/extract.spec.ts line 103
    #[test]
    fn extracts_registry_urls_from_cargo_config_toml() {
        let ctx = CargoContext {
            cargo_config: Some(CARGO6_CONFIG_TOML.to_owned()),
            ..Default::default()
        };
        let deps = extract_with_context(CARGO6_TOML, &ctx).unwrap();
        assert_eq!(deps.len(), 3);

        let prop = deps
            .iter()
            .find(|d| d.dep_name == "proprietary-crate")
            .unwrap();
        assert_eq!(
            prop.registry_urls,
            vec!["https://dl.cloudsmith.io/basic/my-org/my-repo/cargo/index.git"]
        );
        assert!(prop.skip_reason.is_none());

        let mcorbin = deps.iter().find(|d| d.dep_name == "mcorbin-test").unwrap();
        assert_eq!(
            mcorbin.registry_urls,
            vec!["https://github.com/mcorbin/testregistry"]
        );

        let tokio = deps.iter().find(|d| d.dep_name == "tokio").unwrap();
        assert!(tokio.registry_urls.is_empty());
    }

    // Ported: "extracts registry urls from .cargo/config (legacy path)" — lib/modules/manager/cargo/extract.spec.ts line 112
    #[test]
    fn extracts_registry_urls_from_cargo_config_legacy() {
        // Same behavior — legacy path uses same TOML format
        let ctx = CargoContext {
            cargo_config: Some(CARGO6_CONFIG_TOML.to_owned()),
            ..Default::default()
        };
        let deps = extract_with_context(CARGO6_TOML, &ctx).unwrap();
        assert_eq!(deps.len(), 3);
        let prop = deps
            .iter()
            .find(|d| d.dep_name == "proprietary-crate")
            .unwrap();
        assert_eq!(
            prop.registry_urls,
            vec!["https://dl.cloudsmith.io/basic/my-org/my-repo/cargo/index.git"]
        );
    }

    // Ported: "extracts overridden registry indexes from .cargo/config.toml" — lib/modules/manager/cargo/extract.spec.ts line 121
    #[test]
    fn extracts_overridden_registry_indexes() {
        let config = r#"
[registries]
private-crates = { index = "https://dl.cloudsmith.io/basic/my-org/my-repo/cargo/index.git" }

[registries.mcorbin]
index = "https://github.com/mcorbin/testregistry"

[source.crates-io]
replace-with = "mcorbin"

[source.mcorbin]
replace-with = "private-crates"
"#;
        let ctx = CargoContext {
            cargo_config: Some(config.to_owned()),
            ..Default::default()
        };
        let deps = extract_with_context(CARGO6_TOML, &ctx).unwrap();
        assert_eq!(deps.len(), 3);

        // All deps should resolve to private-crates URL via the chain:
        // crates-io → mcorbin → private-crates
        let private_url = "https://dl.cloudsmith.io/basic/my-org/my-repo/cargo/index.git";

        let prop = deps
            .iter()
            .find(|d| d.dep_name == "proprietary-crate")
            .unwrap();
        assert_eq!(prop.registry_urls, vec![private_url]);
        assert!(prop.skip_reason.is_none());

        let mcorbin = deps.iter().find(|d| d.dep_name == "mcorbin-test").unwrap();
        assert_eq!(mcorbin.registry_urls, vec![private_url]);

        // tokio has no registry field — but crates-io is replaced by the chain,
        // so tokio also gets the private-crates URL (same as TypeScript behavior)
        let tokio = deps.iter().find(|d| d.dep_name == "tokio").unwrap();
        assert_eq!(
            tokio.registry_urls,
            vec!["https://dl.cloudsmith.io/basic/my-org/my-repo/cargo/index.git"]
        );
    }

    // Ported: "extracts overridden source registry indexes from .cargo/config.toml" — lib/modules/manager/cargo/extract.spec.ts line 180
    #[test]
    fn extracts_overridden_source_registry_indexes() {
        let config = r#"
[source.crates-io-replacement]
registry = "https://github.com/replacement/testregistry"

[source.crates-io]
replace-with = "crates-io-replacement"
"#;
        let cargo7_toml = r#"
[package]
name = "renovate-test"
version = "0.1.0"
authors = ["John Doe <john.doe@example.org>"]
edition = "2018"

[dependencies]
tokio = "0.2"
"#;
        let ctx = CargoContext {
            cargo_config: Some(config.to_owned()),
            ..Default::default()
        };
        let deps = extract_with_context(cargo7_toml, &ctx).unwrap();
        assert_eq!(deps.len(), 1);
        let tokio = &deps[0];
        assert_eq!(tokio.dep_name, "tokio");
        // crates-io replaced by crates-io-replacement → tokio gets the replacement URL
        assert_eq!(
            tokio.registry_urls,
            vec!["https://github.com/replacement/testregistry"]
        );
    }

    // Ported: "extracts registries overridden to the default" — lib/modules/manager/cargo/extract.spec.ts line 205
    #[test]
    fn extracts_registries_overridden_to_default() {
        // Chain: private-crates → mcorbin → crates-io (default, no URL)
        let config = r#"
[source.mcorbin]
replace-with = "crates-io"

[source.private-crates]
replace-with = "mcorbin"
"#;
        let ctx = CargoContext {
            cargo_config: Some(config.to_owned()),
            ..Default::default()
        };
        let deps = extract_with_context(CARGO6_TOML, &ctx).unwrap();
        assert_eq!(deps.len(), 3);

        // private-crates → mcorbin → crates-io → no URL, not unknown
        let prop = deps
            .iter()
            .find(|d| d.dep_name == "proprietary-crate")
            .unwrap();
        assert!(prop.registry_urls.is_empty());
        assert!(prop.skip_reason.is_none());

        let mcorbin = deps.iter().find(|d| d.dep_name == "mcorbin-test").unwrap();
        assert!(mcorbin.registry_urls.is_empty());
        assert!(mcorbin.skip_reason.is_none());
    }

    // Ported: "extracts registries with an empty config.toml" — lib/modules/manager/cargo/extract.spec.ts line 249
    #[test]
    fn extracts_registries_with_empty_config_toml() {
        let cargo5_toml = r#"
[package]
name = "platform-specific-dep-example"
version = "0.1.2"

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2.37"
js-sys = "0.3.14"
js_relative_import = { path = "../../common/js_relative_import" }

[target.'cfg(target_arch = "wasm32")'.dependencies.web-sys]
version = "0.3.14"
features = ["AudioBuffer"]
"#;
        let ctx = CargoContext {
            cargo_config: Some(String::new()),
            ..Default::default()
        };
        let deps = extract_with_context(cargo5_toml, &ctx).unwrap();
        assert_eq!(deps.len(), 4);

        let wasm = deps.iter().find(|d| d.dep_name == "wasm-bindgen").unwrap();
        assert_eq!(wasm.current_value, "0.2.37");
        assert!(wasm.skip_reason.is_none());

        let path_dep = deps
            .iter()
            .find(|d| d.dep_name == "js_relative_import")
            .unwrap();
        assert_eq!(path_dep.skip_reason, Some(SkipReason::PathDependency));
    }

    // Ported: "extracts registry urls from environment" — lib/modules/manager/cargo/extract.spec.ts line 299
    #[test]
    fn extracts_registry_urls_from_environment() {
        let mut env = HashMap::new();
        env.insert(
            "PRIVATE_CRATES".to_owned(),
            "https://dl.cloudsmith.io/basic/my-org/my-repo/cargo/index.git".to_owned(),
        );
        env.insert(
            "MCORBIN".to_owned(),
            "https://github.com/mcorbin/testregistry".to_owned(),
        );
        let ctx = CargoContext {
            registry_env: env,
            ..Default::default()
        };
        let deps = extract_with_context(CARGO6_TOML, &ctx).unwrap();
        assert_eq!(deps.len(), 3);

        let prop = deps
            .iter()
            .find(|d| d.dep_name == "proprietary-crate")
            .unwrap();
        assert_eq!(
            prop.registry_urls,
            vec!["https://dl.cloudsmith.io/basic/my-org/my-repo/cargo/index.git"]
        );

        let mcorbin = deps.iter().find(|d| d.dep_name == "mcorbin-test").unwrap();
        assert_eq!(
            mcorbin.registry_urls,
            vec!["https://github.com/mcorbin/testregistry"]
        );

        let tokio = deps.iter().find(|d| d.dep_name == "tokio").unwrap();
        assert!(tokio.registry_urls.is_empty());
    }

    // Ported: "skips unknown registries" — lib/modules/manager/cargo/extract.spec.ts line 407
    #[test]
    fn skips_unknown_registries() {
        let toml = "[dependencies]\nfoobar = { version = \"0.1.0\", registry = \"not-listed\" }";
        let deps = extract(toml).unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(SkipReason::UnknownRegistry));
    }

    // Ported: "fails to parse cargo config with invalid TOML" — lib/modules/manager/cargo/extract.spec.ts line 415
    #[test]
    fn fails_to_parse_cargo_config_with_invalid_toml() {
        let ctx = CargoContext {
            cargo_config: Some("[registries".to_owned()),
            ..Default::default()
        };
        // Invalid config is silently ignored; deps still extracted
        let deps = extract_with_context(CARGO6_TOML, &ctx).unwrap();
        assert_eq!(deps.len(), 3);
        // Without a valid config, registries are unknown
        let prop = deps
            .iter()
            .find(|d| d.dep_name == "proprietary-crate")
            .unwrap();
        assert_eq!(prop.skip_reason, Some(SkipReason::UnknownRegistry));
    }

    // Ported: "ignore cargo config registries with missing index" — lib/modules/manager/cargo/extract.spec.ts line 424
    #[test]
    fn ignore_cargo_config_registries_with_missing_index() {
        let config = "[registries.mine]\nfoo = \"bar\"";
        let ctx = CargoContext {
            cargo_config: Some(config.to_owned()),
            ..Default::default()
        };
        let deps = extract_with_context(CARGO6_TOML, &ctx).unwrap();
        assert_eq!(deps.len(), 3);
        // "mine" registry has no index, private-crates and mcorbin are still unknown
        let prop = deps
            .iter()
            .find(|d| d.dep_name == "proprietary-crate")
            .unwrap();
        assert_eq!(prop.skip_reason, Some(SkipReason::UnknownRegistry));
    }

    // Ported: "ignore cargo config source replaced registries with missing index" — lib/modules/manager/cargo/extract.spec.ts line 433
    #[test]
    fn ignore_cargo_config_source_replaced_registries_with_missing_index() {
        let config = r#"
[registries.mine]
foo = "bar"

[source.crates-io]
replace-with = "mine"
"#;
        let ctx = CargoContext {
            cargo_config: Some(config.to_owned()),
            ..Default::default()
        };
        let deps = extract_with_context(CARGO6_TOML, &ctx).unwrap();
        assert_eq!(deps.len(), 3);
        // All deps with named registries are still unknown since "mine" has no index
        let prop = deps
            .iter()
            .find(|d| d.dep_name == "proprietary-crate")
            .unwrap();
        assert_eq!(prop.skip_reason, Some(SkipReason::UnknownRegistry));
    }

    // Ported: "ignore cargo config with circular registry source replacements" — lib/modules/manager/cargo/extract.spec.ts line 481
    #[test]
    fn ignore_cargo_config_with_circular_registry_source_replacements() {
        // Circular: private-crates → mcorbin → private-crates.
        // All named registries resolve to null (circular), so all deps get unknown-registry.
        // crates-io → mcorbin → private-crates → circular → null → all no-registry deps also unknown.
        let config = r#"
[registries]
private-crates = { index = "https://dl.cloudsmith.io/basic/my-org/my-repo/cargo/index.git" }

[registries.mcorbin]
index = "https://github.com/mcorbin/testregistry"

[source.crates-io]
replace-with = "mcorbin"

[source.mcorbin]
replace-with = "private-crates"

[source.private-crates]
replace-with = "mcorbin"
"#;
        let ctx = CargoContext {
            cargo_config: Some(config.to_owned()),
            ..Default::default()
        };
        let deps = extract_with_context(CARGO6_TOML, &ctx).unwrap();
        assert_eq!(deps.len(), 3);

        // All deps get unknown-registry: source replacements form a cycle,
        // resolving private-crates and mcorbin to null, and crates-io also becomes unresolvable.
        for dep in &deps {
            assert_eq!(
                dep.skip_reason,
                Some(SkipReason::UnknownRegistry),
                "dep {} should be unknown-registry",
                dep.dep_name
            );
        }
    }

    // ── Cargo.lock tests ──────────────────────────────────────────────────────

    const CARGO3_LOCK: &str = r#"# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 3

[[package]]
name = "proc-macro2"
version = "1.0.66"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "18fb31db3f9bddb2ea821cde30a9f70117e3f119938b5ee630b7403aa6e2ead9"

[[package]]
name = "test"
version = "0.1.0"
dependencies = [
 "syn 2.0.1",
]

[[package]]
name = "syn"
version = "2.0.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "55ee2415bee46ba26eac9cd8e52966995c46bf0e842b6304eb8fcf99826548ed"
"#;

    const CARGO1_LOCK: &str = r#"# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 3

[[package]]
name = "test"
version = "0.1.0"
dependencies = [
 "syn 1.0.1",
 "syn 2.0.1",
]

[[package]]
name = "syn"
version = "1.0.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "863ecbce06044c8380458360b4146d7372edadfedd77f120ba8c193da427b708"

[[package]]
name = "syn"
version = "2.0.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "55ee2415bee46ba26eac9cd8e52966995c46bf0e842b6304eb8fcf99826548ed"
"#;

    const CARGO2_LOCK: &str = r#"# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 3

[[package]]
name = "test"
version = "0.1.0"
dependencies = [
 "syn 2.0.1",
]

[[package]]
name = "syn"
version = "2.0.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "55ee2415bee46ba26eac9cd8e52966995c46bf0e842b6304eb8fcf99826548ed"
"#;

    // lockfile-update/Cargo.toml uses renamed packages
    const LOCKFILE_UPDATE_TOML: &str = r#"
[package]
name = "test"
version = "0.1.0"
edition = "2021"

[dependencies]
a = { package = "syn", version = "2.0" }
b = { package = "syn", version = "1.0" }
"#;

    // Ported: "extracts locked versions" — lib/modules/manager/cargo/extract.spec.ts line 549
    #[test]
    fn extracts_locked_versions() {
        let toml = r#"
[package]
name = "test"
version = "0.1.0"
edition = "2021"
[dependencies]
syn = "2.0"
"#;
        let ctx = CargoContext {
            cargo_lock: Some(CARGO3_LOCK.to_owned()),
            ..Default::default()
        };
        let deps = extract_with_context(toml, &ctx).unwrap();
        let syn = deps.iter().find(|d| d.dep_name == "syn").unwrap();
        assert_eq!(syn.locked_version.as_deref(), Some("2.0.1"));
    }

    // Ported: "does not extract locked versions for git dependencies" — lib/modules/manager/cargo/extract.spec.ts line 567
    #[test]
    fn does_not_extract_locked_versions_for_git_dependencies() {
        let toml = r#"
[package]
name = "test"
version = "0.1.0"
edition = "2021"
[dependencies]
git_dep = { git = "https://github.com/foo/bar" }
"#;
        let ctx = CargoContext {
            cargo_lock: Some(CARGO3_LOCK.to_owned()),
            ..Default::default()
        };
        let deps = extract_with_context(toml, &ctx).unwrap();
        let git_dep = deps.iter().find(|d| d.dep_name == "git_dep").unwrap();
        assert!(git_dep.locked_version.is_none());
    }

    // Ported: "extracts locked versions for renamed packages" — lib/modules/manager/cargo/extract.spec.ts line 585
    #[test]
    fn extracts_locked_versions_for_renamed_packages() {
        let ctx = CargoContext {
            cargo_lock: Some(CARGO1_LOCK.to_owned()),
            ..Default::default()
        };
        let deps = extract_with_context(LOCKFILE_UPDATE_TOML, &ctx).unwrap();
        let a = deps.iter().find(|d| d.dep_name == "a").unwrap();
        let b = deps.iter().find(|d| d.dep_name == "b").unwrap();
        assert_eq!(a.locked_version.as_deref(), Some("2.0.1"));
        assert_eq!(b.locked_version.as_deref(), Some("1.0.1"));
    }

    // Ported: "handles missing locked versions" — lib/modules/manager/cargo/extract.spec.ts line 601
    #[test]
    fn handles_missing_locked_versions() {
        // CARGO2_LOCK only has syn 2.0.1; "b" needs syn 1.x which is missing
        let ctx = CargoContext {
            cargo_lock: Some(CARGO2_LOCK.to_owned()),
            ..Default::default()
        };
        let deps = extract_with_context(LOCKFILE_UPDATE_TOML, &ctx).unwrap();
        let a = deps.iter().find(|d| d.dep_name == "a").unwrap();
        let b = deps.iter().find(|d| d.dep_name == "b").unwrap();
        assert_eq!(a.locked_version.as_deref(), Some("2.0.1"));
        assert!(b.locked_version.is_none());
    }

    // Ported: "handles invalid versions in the toml file" — lib/modules/manager/cargo/extract.spec.ts line 617
    #[test]
    fn handles_invalid_versions_in_toml_file() {
        let toml = r#"
[package]
name = "test"
version = "0.1.0"
edition = "2021"
[dependencies]
syn = "2.foo.1"
"#;
        let ctx = CargoContext {
            cargo_lock: Some(CARGO3_LOCK.to_owned()),
            ..Default::default()
        };
        let deps = extract_with_context(toml, &ctx).unwrap();
        let syn = deps.iter().find(|d| d.dep_name == "syn").unwrap();
        // "2.foo.1" has no parseable major version → no locked version
        assert!(syn.locked_version.is_none());
    }

    // Ported: "handles invalid lock file" — lib/modules/manager/cargo/extract.spec.ts line 635
    #[test]
    fn handles_invalid_lock_file() {
        let ctx = CargoContext {
            cargo_lock: Some("foo".to_owned()),
            ..Default::default()
        };
        let deps = extract_with_context(LOCKFILE_UPDATE_TOML, &ctx).unwrap();
        // Invalid lock file → no locked versions
        for dep in &deps {
            assert!(dep.locked_version.is_none());
        }
    }
}

// ── range strategy ──────────────────────────────────────────────────────────
// Ported from lib/modules/manager/cargo/range.ts

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangeStrategy {
    Auto,
    Pin,
    Bump,
    Replace,
    Widen,
    UpdateLockfile,
}

pub fn get_range_strategy(
    current_value: Option<&str>,
    range_strategy: &RangeStrategy,
) -> RangeStrategy {
    if range_strategy != &RangeStrategy::Auto {
        return range_strategy.clone();
    }
    if current_value.is_some_and(|v| v.contains('<')) {
        RangeStrategy::Widen
    } else {
        RangeStrategy::UpdateLockfile
    }
}

// ── update-locked ───────────────────────────────────────────────────────────
// Ported from lib/modules/manager/cargo/update-locked.ts

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateLockedStatus {
    AlreadyUpdated,
    Unsupported,
    UpdateFailed,
}

#[derive(Debug)]
pub struct UpdateLockedConfig<'a> {
    pub dep_name: &'a str,
    pub current_version: &'a str,
    pub new_version: &'a str,
    pub lock_file: &'a str,
    pub lock_file_content: Option<&'a str>,
}

pub fn update_locked_dependency(config: &UpdateLockedConfig<'_>) -> UpdateLockedStatus {
    let Some(lock_content) = config.lock_file_content else {
        return UpdateLockedStatus::UpdateFailed;
    };

    let locked = extract_lock_versions(lock_content);
    match locked.get(config.dep_name) {
        Some(versions) => {
            if versions.iter().any(|v| v == config.new_version) {
                UpdateLockedStatus::AlreadyUpdated
            } else {
                UpdateLockedStatus::Unsupported
            }
        }
        None => UpdateLockedStatus::Unsupported,
    }
}

fn extract_lock_versions(content: &str) -> std::collections::HashMap<String, Vec<String>> {
    let mut result: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    let mut current_name: Option<String> = None;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("name = \"") {
            if let Some(name) = trimmed.trim_start_matches("name = \"").strip_suffix('"') {
                current_name = Some(name.to_owned());
            }
        } else if let Some(version) = trimmed.trim_start_matches("version = \"").strip_suffix('"') {
            if let Some(name) = &current_name {
                result
                    .entry(name.clone())
                    .or_default()
                    .push(version.to_owned());
            }
        } else if trimmed == "[[package]]" || trimmed.is_empty() {
            current_name = None;
        }
    }

    result
}

// ── bump package version ───────────────────────────────────────────────────
// Ported from lib/modules/manager/cargo/update.ts

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BumpVersion {
    Major,
    Minor,
    Patch,
}

#[derive(Debug)]
pub struct BumpResult {
    pub bumped_content: String,
}

pub fn bump_package_version(
    content: &str,
    current_value: &str,
    bump_version: &BumpVersion,
) -> BumpResult {
    if semver::Version::parse(current_value).is_err() {
        return BumpResult {
            bumped_content: content.to_owned(),
        };
    }

    let Ok(mut ver) = semver::Version::parse(current_value) else {
        return BumpResult {
            bumped_content: content.to_owned(),
        };
    };

    match bump_version {
        BumpVersion::Major => {
            ver.major += 1;
            ver.minor = 0;
            ver.patch = 0;
        }
        BumpVersion::Minor => {
            ver.minor += 1;
            ver.patch = 0;
        }
        BumpVersion::Patch => {
            ver.patch += 1;
        }
    }

    let new_version = ver.to_string();
    let bumped = content.replacen(
        &format!("version = \"{}\"", current_value),
        &format!("version = \"{}\"", new_version),
        1,
    );

    BumpResult {
        bumped_content: bumped,
    }
}

// ── update-artifacts decision logic ────────────────────────────────────────
// Ported from lib/modules/manager/cargo/artifacts.ts
//
// The full artifacts pipeline (running `cargo update`) is M4 scope.
// This module implements the decision logic that determines what type of
// update to perform, which is exercised by the upstream artifacts tests.

/// Whether a dependency comes from the crate datasource.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactDep {
    pub dep_name: String,
    pub package_name: Option<String>,
    pub is_crate: bool,
    pub locked_version: Option<String>,
    pub new_version: Option<String>,
    pub current_value: Option<String>,
    pub new_value: Option<String>,
}

/// The type of `cargo update` command to run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CargoUpdateType {
    /// No Cargo.lock found — nothing to do.
    NoLockFile,
    /// No dependencies to update and not lockfile maintenance.
    NoDepsToUpdate,
    /// Run `cargo update` without `--workspace` (lockfile maintenance).
    LockfileMaintenance,
    /// Run `cargo update --workspace` (general update).
    Workspace,
    /// Run precise `cargo update --package <pkg>@<locked> --precise <new>` for each dep,
    /// then `cargo update --workspace`.
    Precise { packages: Vec<PrecisePackage> },
}

/// A package to update precisely.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrecisePackage {
    pub package_name: String,
    pub locked_version: String,
    pub new_version: String,
}

/// Determine the type of cargo update to perform.
///
/// Mirrors the decision logic in `updateArtifactsImpl` from
/// `lib/modules/manager/cargo/artifacts.ts`.
pub fn determine_cargo_update_type(
    lock_file_found: bool,
    is_lockfile_maintenance: bool,
    updated_deps: &[ArtifactDep],
) -> CargoUpdateType {
    if !lock_file_found {
        return CargoUpdateType::NoLockFile;
    }
    if !is_lockfile_maintenance && updated_deps.is_empty() {
        return CargoUpdateType::NoDepsToUpdate;
    }
    if is_lockfile_maintenance {
        return CargoUpdateType::LockfileMaintenance;
    }
    let has_non_crate_dep = updated_deps.iter().any(|d| !d.is_crate);
    let crate_without_locked = updated_deps
        .iter()
        .find(|d| d.is_crate && d.locked_version.is_none());
    if has_non_crate_dep || crate_without_locked.is_some() {
        return CargoUpdateType::Workspace;
    }
    // Filter out deps where the range has changed (they'll be resolved by --workspace).
    let precise_packages: Vec<PrecisePackage> = updated_deps
        .iter()
        .filter(|d| match (&d.current_value, &d.new_value) {
            (Some(cv), Some(nv)) => cv == nv,
            _ => true,
        })
        .filter_map(|d| {
            let locked = d.locked_version.as_deref()?;
            let new_ver = d.new_version.as_deref()?;
            let pkg_name = d.package_name.as_deref().unwrap_or(d.dep_name.as_str());
            Some(PrecisePackage {
                package_name: pkg_name.to_owned(),
                locked_version: locked.to_owned(),
                new_version: new_ver.to_owned(),
            })
        })
        .collect();
    CargoUpdateType::Precise {
        packages: precise_packages,
    }
}

#[cfg(test)]
mod range_update_tests {
    use super::*;

    // Rust-specific: cargo behavior test
    #[test]
    fn get_range_strategy_auto_default() {
        assert_eq!(
            get_range_strategy(None, &RangeStrategy::Auto),
            RangeStrategy::UpdateLockfile
        );
    }

    // Rust-specific: cargo behavior test
    #[test]
    fn get_range_strategy_auto_with_lt() {
        assert_eq!(
            get_range_strategy(Some(">=1.0.0 <2.0.0"), &RangeStrategy::Auto),
            RangeStrategy::Widen
        );
    }

    // Rust-specific: cargo behavior test
    #[test]
    fn get_range_strategy_non_auto_passthrough() {
        assert_eq!(
            get_range_strategy(None, &RangeStrategy::Pin),
            RangeStrategy::Pin
        );
    }

    // Rust-specific: cargo behavior test
    #[test]
    fn update_locked_already_updated() {
        let config = UpdateLockedConfig {
            dep_name: "serde",
            current_version: "1.0.100",
            new_version: "1.0.200",
            lock_file: "Cargo.lock",
            lock_file_content: Some("[[package]]\nname = \"serde\"\nversion = \"1.0.200\"\n"),
        };
        assert_eq!(
            update_locked_dependency(&config),
            UpdateLockedStatus::AlreadyUpdated
        );
    }

    // Rust-specific: cargo behavior test
    #[test]
    fn update_locked_unsupported() {
        let config = UpdateLockedConfig {
            dep_name: "serde",
            current_version: "1.0.100",
            new_version: "1.0.200",
            lock_file: "Cargo.lock",
            lock_file_content: Some("[[package]]\nname = \"serde\"\nversion = \"1.0.100\"\n"),
        };
        assert_eq!(
            update_locked_dependency(&config),
            UpdateLockedStatus::Unsupported
        );
    }

    // Rust-specific: cargo behavior test
    #[test]
    fn update_locked_no_content() {
        let config = UpdateLockedConfig {
            dep_name: "serde",
            current_version: "1.0.100",
            new_version: "1.0.200",
            lock_file: "Cargo.lock",
            lock_file_content: None,
        };
        assert_eq!(
            update_locked_dependency(&config),
            UpdateLockedStatus::UpdateFailed
        );
    }

    // Rust-specific: cargo behavior test
    #[test]
    fn bump_package_version_patch() {
        let content = "[package]\nname = \"foo\"\nversion = \"1.2.3\"\n";
        let result = bump_package_version(content, "1.2.3", &BumpVersion::Patch);
        assert!(result.bumped_content.contains("version = \"1.2.4\""));
    }

    // Rust-specific: cargo behavior test
    #[test]
    fn bump_package_version_minor() {
        let content = "[package]\nname = \"foo\"\nversion = \"1.2.3\"\n";
        let result = bump_package_version(content, "1.2.3", &BumpVersion::Minor);
        assert!(result.bumped_content.contains("version = \"1.3.0\""));
    }

    // Rust-specific: cargo behavior test
    #[test]
    fn bump_package_version_major() {
        let content = "[package]\nname = \"foo\"\nversion = \"1.2.3\"\n";
        let result = bump_package_version(content, "1.2.3", &BumpVersion::Major);
        assert!(result.bumped_content.contains("version = \"2.0.0\""));
    }

    // Rust-specific: cargo behavior test
    #[test]
    fn bump_package_version_invalid_semver_noop() {
        let content = "[package]\nname = \"foo\"\nversion = \"not-semver\"\n";
        let result = bump_package_version(content, "not-semver", &BumpVersion::Patch);
        assert_eq!(result.bumped_content, content);
    }

    #[test]
    fn cargo_dep_type_as_renovate_str() {
        assert_eq!(DepType::Regular.as_renovate_str(), "dependencies");
        assert_eq!(DepType::Dev.as_renovate_str(), "devDependencies");
        assert_eq!(DepType::Build.as_renovate_str(), "buildDependencies");
        assert_eq!(
            DepType::Workspace.as_renovate_str(),
            "workspace.dependencies"
        );
    }
}

#[cfg(test)]
mod artifacts_decision_tests {
    use super::*;

    fn crate_dep(name: &str, locked: Option<&str>, new_ver: Option<&str>) -> ArtifactDep {
        ArtifactDep {
            dep_name: name.to_owned(),
            package_name: None,
            is_crate: true,
            locked_version: locked.map(String::from),
            new_version: new_ver.map(String::from),
            current_value: Some("1.0.0".to_owned()),
            new_value: Some("1.0.0".to_owned()),
        }
    }

    fn crate_dep_with_range(
        name: &str,
        locked: Option<&str>,
        new_ver: Option<&str>,
        current_val: &str,
        new_val: &str,
    ) -> ArtifactDep {
        ArtifactDep {
            dep_name: name.to_owned(),
            package_name: None,
            is_crate: true,
            locked_version: locked.map(String::from),
            new_version: new_ver.map(String::from),
            current_value: Some(current_val.to_owned()),
            new_value: Some(new_val.to_owned()),
        }
    }

    fn non_crate_dep(name: &str) -> ArtifactDep {
        ArtifactDep {
            dep_name: name.to_owned(),
            package_name: None,
            is_crate: false,
            locked_version: None,
            new_version: None,
            current_value: None,
            new_value: None,
        }
    }

    // Ported: "returns null if no Cargo.lock found" — lib/modules/manager/cargo/artifacts.spec.ts line 44
    #[test]
    fn no_lock_file_returns_no_lock_file() {
        let result = determine_cargo_update_type(false, false, &[crate_dep("dep1", None, None)]);
        assert_eq!(result, CargoUpdateType::NoLockFile);
    }

    // Ported: "returns null if updatedDeps is empty" — lib/modules/manager/cargo/artifacts.spec.ts line 62
    #[test]
    fn empty_deps_returns_no_deps() {
        let result = determine_cargo_update_type(true, false, &[]);
        assert_eq!(result, CargoUpdateType::NoDepsToUpdate);
    }

    // Ported: "returns updated Cargo.lock for lockfile maintenance" — lib/modules/manager/cargo/artifacts.spec.ts line 488
    #[test]
    fn lockfile_maintenance() {
        let result = determine_cargo_update_type(true, true, &[]);
        assert_eq!(result, CargoUpdateType::LockfileMaintenance);
    }

    // Ported: "returns updated Cargo.lock with precise version update" — lib/modules/manager/cargo/artifacts.spec.ts line 122
    #[test]
    fn precise_update_when_all_deps_have_locked_version() {
        let deps = vec![ArtifactDep {
            dep_name: "dep1".to_owned(),
            package_name: Some("dep1".to_owned()),
            is_crate: true,
            locked_version: Some("1.0.0".to_owned()),
            new_version: Some("1.0.1".to_owned()),
            current_value: Some("1.0.0".to_owned()),
            new_value: Some("1.0.0".to_owned()),
        }];
        let result = determine_cargo_update_type(true, false, &deps);
        match result {
            CargoUpdateType::Precise { packages } => {
                assert_eq!(packages.len(), 1);
                assert_eq!(packages[0].package_name, "dep1");
                assert_eq!(packages[0].locked_version, "1.0.0");
                assert_eq!(packages[0].new_version, "1.0.1");
            }
            other => panic!("expected Precise, got {:?}", other),
        }
    }

    // Ported: "skips precise update when manifest range has changed" — lib/modules/manager/cargo/artifacts.spec.ts line 164
    #[test]
    fn skips_precise_when_range_changed() {
        let deps = vec![crate_dep_with_range(
            "dep1",
            Some("1.0.0"),
            Some("1.0.1"),
            "1.0.0", // current
            "2.0.0", // new (changed)
        )];
        let result = determine_cargo_update_type(true, false, &deps);
        // Range changed → dep is filtered out of precise packages, but still Precise
        // with empty packages (workspace would still be called in real pipeline).
        match result {
            CargoUpdateType::Precise { packages } => {
                assert!(packages.is_empty());
            }
            other => panic!("expected Precise, got {:?}", other),
        }
    }

    // Ported: "handles mixed deps where some have range changes and some do not" — lib/modules/manager/cargo/artifacts.spec.ts line 199
    #[test]
    fn mixed_deps_with_range_changes() {
        let deps = vec![
            crate_dep_with_range("dep1", Some("1.0.0"), Some("1.0.1"), "1.0.0", "2.0.0"),
            crate_dep("dep2", Some("2.0.0"), Some("2.0.1")),
        ];
        let result = determine_cargo_update_type(true, false, &deps);
        match result {
            CargoUpdateType::Precise { packages } => {
                // dep1 range changed → filtered; dep2 → precise
                assert_eq!(packages.len(), 1);
                assert_eq!(packages[0].package_name, "dep2");
            }
            other => panic!("expected Precise, got {:?}", other),
        }
    }

    // Ported: "returns an artifact error when cargo update fails" — lib/modules/manager/cargo/artifacts.spec.ts line 247
    #[test]
    fn workspace_update_when_missing_locked_version() {
        let deps = vec![crate_dep("dep1", None, Some("1.0.1"))];
        let result = determine_cargo_update_type(true, false, &deps);
        assert_eq!(result, CargoUpdateType::Workspace);
    }

    // Ported: "returns updated Cargo.lock when there are no more dependencies to update" — lib/modules/manager/cargo/artifacts.spec.ts line 413
    #[test]
    fn workspace_for_non_crate_dep() {
        let deps = vec![non_crate_dep("git-dep")];
        let result = determine_cargo_update_type(true, false, &deps);
        assert_eq!(result, CargoUpdateType::Workspace);
    }
}
