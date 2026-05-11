//! package.json dependency extractor.
//!
//! Parses an npm `package.json` file and returns the set of package
//! dependencies with their version constraints, ready for registry lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/npm/extract/common/package-file.ts`
//! - `lib/modules/manager/npm/dep-types.ts` — `knownDepTypes`
//!
//! ## Supported dep sections
//!
//! Four standard dependency sections are extracted:
//! `dependencies`, `devDependencies`, `peerDependencies`,
//! `optionalDependencies`.
//!
//! ## Skip-reason classification
//!
//! Constraint strings that are not plain semver ranges are classified and
//! skipped:
//! - `workspace:*` / `workspace:^` etc. — pnpm/yarn workspace protocol
//! - `file:../path` / `link:../path` — local path reference
//! - `github:owner/repo` / `gitlab:...` / `bitbucket:...` — git platform shorthand
//! - `git+https://...` / `git://...` — git URL
//! - `http://...` / `https://...` — URL install
//! - `npm:other-pkg@...` — npm alias (deferred)

use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer};
use thiserror::Error;

/// Why an npm dependency is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NpmSkipReason {
    /// Dependency uses the workspace protocol (`workspace:*`).
    WorkspaceProtocol,
    /// Dependency is a local file/link reference (`file:../path`).
    LocalPath,
    /// Dependency is resolved from a git source.
    GitSource,
    /// Dependency is installed from a URL.
    UrlInstall,
    /// Dependency uses an npm alias (`npm:other-pkg`).
    NpmAlias,
    /// Dependency name is not valid for the npm registry.
    InvalidName,
    /// Dependency value is not a string version specifier.
    InvalidValue,
    /// Dependency has an empty version specifier.
    Empty,
    /// Dependency has no comparable version.
    UnspecifiedVersion,
    /// Engine name is not handled by Renovate.
    UnknownEngines,
}

/// Which `package.json` section the dep came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpmDepType {
    Regular,
    Dev,
    Peer,
    Optional,
    /// yarn `resolutions` override.
    Resolutions,
    /// npm 8+ `overrides` override.
    Overrides,
    /// `engines` constraints.
    Engines,
}

impl NpmDepType {
    /// Return the Renovate-compatible dep type string used in `matchDepTypes`.
    pub fn as_renovate_str(&self) -> &'static str {
        match self {
            NpmDepType::Regular => "dependencies",
            NpmDepType::Dev => "devDependencies",
            NpmDepType::Peer => "peerDependencies",
            NpmDepType::Optional => "optionalDependencies",
            NpmDepType::Resolutions => "resolutions",
            NpmDepType::Overrides => "overrides",
            NpmDepType::Engines => "engines",
        }
    }
}

/// A single extracted npm dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NpmExtractedDep {
    /// Package name (the key in the dep section).
    pub name: String,
    /// The version constraint string (e.g. `"^18.0.0"`).
    pub current_value: String,
    /// Which dep section this came from.
    pub dep_type: NpmDepType,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<NpmSkipReason>,
}

/// Yarn registry configuration relevant to npm package extraction.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct YarnConfig {
    pub npm_registry_server: Option<String>,
    pub npm_scopes: BTreeMap<String, YarnScopeConfig>,
}

/// Per-scope Yarn npm registry configuration.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct YarnScopeConfig {
    pub npm_registry_server: Option<String>,
}

/// Parsed npm package-lock data relevant to dependency extraction.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NpmLock {
    pub locked_versions: BTreeMap<String, String>,
    pub lockfile_version: Option<u64>,
}

/// Parsed Yarn lock metadata used to choose the expected Yarn version range.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YarnLock {
    pub is_yarn1: bool,
    pub lockfile_version: Option<u64>,
    pub locked_versions: BTreeMap<String, String>,
}

/// Yarn catalog dependency extracted from `.yarnrc.yml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YarnCatalogDep {
    pub name: String,
    pub current_value: String,
    pub dep_type: String,
}

/// Yarn catalog extraction result.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct YarnCatalogExtraction {
    pub deps: Vec<YarnCatalogDep>,
    pub yarn_lock: Option<String>,
    pub has_package_manager: bool,
}

/// pnpm workspace dependency extracted from `pnpm-workspace.yaml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PnpmWorkspaceDep {
    pub name: String,
    pub current_value: String,
    pub dep_type: String,
    pub package_name: Option<String>,
}

/// pnpm workspace extraction result.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PnpmWorkspaceExtraction {
    pub deps: Vec<PnpmWorkspaceDep>,
    pub pnpm_shrinkwrap: Option<String>,
}

/// Errors from parsing a `package.json`.
#[derive(Debug, Error)]
pub enum NpmExtractError {
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
}

// ── Internal deserialization ──────────────────────────────────────────────────

#[derive(Debug, Deserialize, Default)]
struct PackageJson {
    #[serde(rename = "_from")]
    from: Option<serde_json::Value>,
    #[serde(rename = "_id")]
    id: Option<serde_json::Value>,
    #[serde(rename = "_resolved")]
    resolved: Option<serde_json::Value>,
    #[serde(default, deserialize_with = "deserialize_dependency_section")]
    dependencies: BTreeMap<String, DependencySpec>,
    #[serde(
        rename = "devDependencies",
        default,
        deserialize_with = "deserialize_dependency_section"
    )]
    dev_dependencies: BTreeMap<String, DependencySpec>,
    #[serde(
        rename = "peerDependencies",
        default,
        deserialize_with = "deserialize_dependency_section"
    )]
    peer_dependencies: BTreeMap<String, DependencySpec>,
    #[serde(
        rename = "optionalDependencies",
        default,
        deserialize_with = "deserialize_dependency_section"
    )]
    optional_dependencies: BTreeMap<String, DependencySpec>,
    /// yarn `resolutions` block — flat `{ "pkg": "version" }`.
    #[serde(default, deserialize_with = "deserialize_dependency_section")]
    resolutions: BTreeMap<String, DependencySpec>,
    /// npm 8+ `overrides` block — flat `{ "pkg": "version" }`.
    #[serde(default, deserialize_with = "deserialize_dependency_section")]
    overrides: BTreeMap<String, DependencySpec>,
    /// package runtime/tool constraints.
    #[serde(default, deserialize_with = "deserialize_dependency_section")]
    engines: BTreeMap<String, DependencySpec>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DependencySpec {
    Version(String),
    InvalidValue,
}

fn deserialize_dependency_section<'de, D>(
    deserializer: D,
) -> Result<BTreeMap<String, DependencySpec>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    let Some(object) = value.as_object() else {
        return Ok(BTreeMap::new());
    };

    Ok(object
        .iter()
        .map(|(name, value)| {
            let spec = value
                .as_str()
                .map(|version| DependencySpec::Version(version.to_owned()))
                .unwrap_or(DependencySpec::InvalidValue);
            (name.clone(), spec)
        })
        .collect())
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Parse a `package.json` string and extract all npm dependencies.
///
/// Returns a flat list of deps across all four sections, each annotated with
/// its section type and any applicable skip reason.
pub fn extract(content: &str) -> Result<Vec<NpmExtractedDep>, NpmExtractError> {
    let pkg: PackageJson = serde_json::from_str(content)?;
    if pkg.is_vendorised() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();

    for (section, dep_type) in [
        (&pkg.dependencies, NpmDepType::Regular),
        (&pkg.dev_dependencies, NpmDepType::Dev),
        (&pkg.peer_dependencies, NpmDepType::Peer),
        (&pkg.optional_dependencies, NpmDepType::Optional),
        (&pkg.resolutions, NpmDepType::Resolutions),
        (&pkg.overrides, NpmDepType::Overrides),
        (&pkg.engines, NpmDepType::Engines),
    ] {
        for (name, value) in section {
            out.push(classify(
                normalize_package_key(name, dep_type),
                value,
                dep_type,
            ));
        }
    }

    Ok(out)
}

impl PackageJson {
    fn is_vendorised(&self) -> bool {
        self.id.is_some() && (self.from.is_some() || self.resolved.is_some())
    }
}

fn normalize_package_key(name: &str, dep_type: NpmDepType) -> String {
    if dep_type != NpmDepType::Resolutions {
        return name.to_owned();
    }

    if let Some(scoped_start) = name.rfind("/@") {
        return name[scoped_start + 1..].to_owned();
    }

    name.rsplit('/').next().unwrap_or(name).to_owned()
}

/// Resolve the registry URL for a package name from Yarn config.
///
/// Renovate reference: `lib/modules/manager/npm/extract/yarnrc.ts` `resolveRegistryUrl`.
pub fn resolve_yarn_registry_url(package_name: &str, config: &YarnConfig) -> Option<String> {
    if let Some(scope) = package_name
        .strip_prefix('@')
        .and_then(|rest| rest.split_once('/').map(|(scope, _)| scope))
        && let Some(scope_config) = config.npm_scopes.get(scope)
    {
        return scope_config.npm_registry_server.clone();
    }

    config.npm_registry_server.clone()
}

/// Parse the subset of `.yarnrc.yml` used for npm registry resolution.
///
/// Renovate reference: `lib/modules/manager/npm/extract/yarnrc.ts` `loadConfigFromYarnrcYml`.
pub fn load_config_from_yarnrc_yml(content: &str) -> Option<YarnConfig> {
    if content.trim().is_empty() {
        return None;
    }

    let mut config = YarnConfig::default();
    let mut current_scope: Option<String> = None;
    let mut saw_relevant_key = false;

    for raw_line in content.lines() {
        let line = raw_line.trim_end();
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if !line.starts_with(' ') {
            current_scope = None;
            if let Some(value) = trimmed.strip_prefix("npmRegistryServer:") {
                let value = parse_yarn_string_value(value)?;
                config.npm_registry_server = Some(value);
                saw_relevant_key = true;
            } else if let Some(value) = trimmed.strip_prefix("npmScopes:") {
                if !value.trim().is_empty() {
                    return None;
                }
                saw_relevant_key = true;
            }
            continue;
        }

        let indent = raw_line.chars().take_while(|ch| *ch == ' ').count();
        if indent == 2 && trimmed.ends_with(':') {
            let scope = trimmed.trim_end_matches(':').to_owned();
            config.npm_scopes.entry(scope.clone()).or_default();
            current_scope = Some(scope);
        } else if indent == 2 && trimmed.contains(':') {
            return None;
        } else if indent == 4
            && let Some(scope) = &current_scope
            && let Some(value) = trimmed.strip_prefix("npmRegistryServer:")
        {
            let value = parse_yarn_string_value(value)?;
            config
                .npm_scopes
                .entry(scope.clone())
                .or_default()
                .npm_registry_server = Some(value);
        }
    }

    saw_relevant_key.then_some(config)
}

/// Parse legacy `.yarnrc` registry settings.
///
/// Renovate reference: `lib/modules/manager/npm/extract/yarnrc.ts`
/// `loadConfigFromLegacyYarnrc`.
pub fn load_config_from_legacy_yarnrc(content: &str) -> YarnConfig {
    let mut config = YarnConfig::default();

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("--") {
            continue;
        }

        let Some((raw_key, raw_value)) = split_legacy_yarnrc_line(line) else {
            continue;
        };
        let key = unquote_yarnrc_token(raw_key);
        let value = unquote_yarnrc_token(raw_value);

        if key == "registry" {
            config.npm_registry_server = Some(value);
        } else if let Some(scope) = key
            .strip_prefix('@')
            .and_then(|key| key.strip_suffix(":registry"))
        {
            config
                .npm_scopes
                .entry(scope.to_owned())
                .or_default()
                .npm_registry_server = Some(value);
        }
    }

    config
}

/// Parse npm `package-lock.json` content into locked versions.
///
/// Renovate reference: `lib/modules/manager/npm/extract/npm.ts` `getNpmLock`.
pub fn parse_npm_lock(content: Option<&str>) -> NpmLock {
    let Some(content) = content else {
        return NpmLock::default();
    };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(content) else {
        return NpmLock::default();
    };

    let lockfile_version = value.get("lockfileVersion").and_then(|v| v.as_u64());
    let mut locked_versions = BTreeMap::new();

    if let Some(dependencies) = value.get("dependencies").and_then(|v| v.as_object()) {
        for (name, dep) in dependencies {
            if let Some(version) = dep.get("version").and_then(|v| v.as_str()) {
                locked_versions.insert(name.clone(), version.to_owned());
            }
        }
    }

    if locked_versions.is_empty()
        && let Some(packages) = value.get("packages").and_then(|v| v.as_object())
    {
        for (path, package) in packages {
            let Some(name) = path.strip_prefix("node_modules/") else {
                continue;
            };
            if let Some(version) = package.get("version").and_then(|v| v.as_str()) {
                locked_versions.insert(name.to_owned(), version.to_owned());
            }
        }
    }

    NpmLock {
        locked_versions,
        lockfile_version,
    }
}

/// Return the Yarn version range Renovate infers from lockfile metadata.
///
/// Renovate reference: `lib/modules/manager/npm/extract/yarn.ts`
/// `getYarnVersionFromLock`.
pub fn get_yarn_version_from_lock(lock: &YarnLock) -> &'static str {
    if lock.is_yarn1 {
        return "^1.22.18";
    }

    match lock.lockfile_version {
        Some(version) if version >= 12 => ">=4.0.0",
        Some(version) if version >= 10 => "^4.0.0",
        Some(version) if version >= 8 => "^3.0.0",
        Some(version) if version >= 6 => "^2.2.0",
        _ => "^2.0.0",
    }
}

/// Parse Yarn v1 and Berry lockfiles into locked package versions.
///
/// Renovate reference: `lib/modules/manager/npm/extract/yarn.ts` `getYarnLock`.
pub fn parse_yarn_lock(content: Option<&str>) -> YarnLock {
    let Some(content) = content else {
        return YarnLock {
            is_yarn1: true,
            lockfile_version: None,
            locked_versions: BTreeMap::new(),
        };
    };

    let is_yarn1 = !content.lines().any(|line| line.trim() == "__metadata:");
    let lockfile_version = parse_yarn_lockfile_version(content);
    let mut locked_versions = BTreeMap::new();
    let mut current_name: Option<String> = None;

    for raw_line in content.lines() {
        let line = raw_line.trim_end();
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if !line.starts_with(' ') && !line.starts_with('\t') && trimmed.ends_with(':') {
            current_name = yarn_lock_entry_name(trimmed.trim_end_matches(':'));
            continue;
        }

        let version = trimmed
            .strip_prefix("version ")
            .or_else(|| trimmed.strip_prefix("version:"))
            .and_then(parse_yarn_string_value);

        if let (Some(name), Some(version)) = (current_name.take(), version) {
            locked_versions.insert(name, version);
        }
    }

    YarnLock {
        is_yarn1,
        lockfile_version,
        locked_versions,
    }
}

fn parse_yarn_lockfile_version(content: &str) -> Option<u64> {
    let mut in_metadata = false;
    for raw_line in content.lines() {
        let trimmed = raw_line.trim();
        if trimmed == "__metadata:" {
            in_metadata = true;
            continue;
        }
        if in_metadata && trimmed.starts_with("version:") {
            return trimmed
                .strip_prefix("version:")
                .map(|version| version.trim().trim_matches('"').trim_matches('\''))
                .and_then(|version| version.parse().ok());
        }
        if in_metadata && !raw_line.starts_with(' ') && !trimmed.is_empty() {
            in_metadata = false;
        }
    }
    None
}

fn yarn_lock_entry_name(entry: &str) -> Option<String> {
    let first = entry
        .trim()
        .trim_matches('"')
        .split(',')
        .next()?
        .trim()
        .trim_matches('"');
    let descriptor = first.strip_prefix("patch:").unwrap_or(first);
    let descriptor = descriptor.strip_prefix("virtual:").unwrap_or(descriptor);
    let descriptor = descriptor.split('#').next().unwrap_or(descriptor);
    descriptor
        .rsplit_once('@')
        .map(|(name, _)| name)
        .filter(|name| is_valid_yarn_package_name(name))
        .map(str::to_owned)
}

fn is_valid_yarn_package_name(name: &str) -> bool {
    !name.is_empty() && name.chars().any(|ch| ch.is_ascii_alphabetic() || ch == '@')
}

/// Extract Yarn catalog dependencies from parsed `.yarnrc.yml` catalog blocks.
///
/// Renovate reference: `lib/modules/manager/npm/extract/yarn.ts`
/// `extractYarnCatalogs`.
pub fn extract_yarn_catalogs(
    default_catalog: &BTreeMap<String, String>,
    named_catalogs: &BTreeMap<String, BTreeMap<String, String>>,
    yarn_lock: Option<&str>,
    has_package_manager: bool,
) -> YarnCatalogExtraction {
    let mut deps = Vec::new();

    for (name, version) in default_catalog {
        deps.push(YarnCatalogDep {
            name: name.clone(),
            current_value: version.clone(),
            dep_type: "yarn.catalog.default".to_owned(),
        });
    }

    for (catalog_name, catalog) in named_catalogs {
        for (name, version) in catalog {
            deps.push(YarnCatalogDep {
                name: name.clone(),
                current_value: version.clone(),
                dep_type: format!("yarn.catalog.{catalog_name}"),
            });
        }
    }

    YarnCatalogExtraction {
        deps,
        yarn_lock: yarn_lock.map(str::to_owned),
        has_package_manager,
    }
}

/// Extract pnpm catalog and workspace override dependencies from parsed
/// `pnpm-workspace.yaml` data.
///
/// Renovate reference: `lib/modules/manager/npm/extract/pnpm.ts`
/// `extractPnpmWorkspaceFile`.
pub fn extract_pnpm_workspace_file(
    default_catalog: &BTreeMap<String, String>,
    named_catalogs: &BTreeMap<String, BTreeMap<String, String>>,
    overrides: &BTreeMap<String, String>,
    pnpm_lock: Option<&str>,
) -> PnpmWorkspaceExtraction {
    let mut deps = Vec::new();

    for (name, version) in default_catalog {
        deps.push(PnpmWorkspaceDep {
            name: name.clone(),
            current_value: version.clone(),
            dep_type: "pnpm.catalog.default".to_owned(),
            package_name: None,
        });
    }

    for (catalog_name, catalog) in named_catalogs {
        for (name, version) in catalog {
            deps.push(PnpmWorkspaceDep {
                name: name.clone(),
                current_value: version.clone(),
                dep_type: format!("pnpm.catalog.{catalog_name}"),
                package_name: None,
            });
        }
    }

    for (name, version) in overrides {
        deps.push(PnpmWorkspaceDep {
            name: name.clone(),
            current_value: version.clone(),
            dep_type: "pnpm-workspace.overrides".to_owned(),
            package_name: Some(pnpm_override_package_name(name)),
        });
    }

    PnpmWorkspaceExtraction {
        deps,
        pnpm_shrinkwrap: pnpm_lock.map(str::to_owned),
    }
}

fn parse_yarn_string_value(value: &str) -> Option<String> {
    let value = value.trim().trim_matches('"').trim_matches('\'');
    if value.is_empty() || value.parse::<i64>().is_ok() {
        return None;
    }
    Some(value.to_owned())
}

fn pnpm_override_package_name(dep_name: &str) -> String {
    if let Some((_, package)) = dep_name.rsplit_once('>')
        && package
            .chars()
            .any(|ch| ch.is_ascii_alphabetic() || ch == '@')
    {
        return package.to_owned();
    }

    let scoped_version_separator = dep_name
        .starts_with('@')
        .then(|| {
            dep_name
                .find('/')
                .and_then(|slash| dep_name[slash + 1..].find('@').map(|idx| slash + 1 + idx))
        })
        .flatten();
    let unscoped_version_separator = (!dep_name.starts_with('@'))
        .then(|| dep_name.find('@'))
        .flatten();
    let base = scoped_version_separator
        .or(unscoped_version_separator)
        .map(|idx| &dep_name[..idx])
        .unwrap_or(dep_name);

    base.rsplit('>').next().unwrap_or(base).to_owned()
}

fn split_legacy_yarnrc_line(line: &str) -> Option<(&str, &str)> {
    let trimmed = line.trim();
    if let Some(rest) = trimmed.strip_prefix('"') {
        let end = rest.find('"')? + 1;
        let key = &trimmed[..=end];
        let value = trimmed[end + 1..].trim();
        return (!value.is_empty()).then_some((key, value));
    }

    trimmed.split_once(char::is_whitespace)
}

fn unquote_yarnrc_token(token: &str) -> String {
    token.trim().trim_matches('"').trim_matches('\'').to_owned()
}

fn classify(name: String, value: &DependencySpec, dep_type: NpmDepType) -> NpmExtractedDep {
    let current_value = match value {
        DependencySpec::Version(value) => value.clone(),
        DependencySpec::InvalidValue => String::new(),
    };
    let skip_reason = if dep_type == NpmDepType::Engines {
        engine_skip_reason_for(&name, value, &current_value)
    } else if invalid_package_name(&name) {
        Some(NpmSkipReason::InvalidName)
    } else if matches!(value, DependencySpec::InvalidValue) {
        Some(NpmSkipReason::InvalidValue)
    } else {
        skip_reason_for(&current_value)
    };
    NpmExtractedDep {
        name,
        current_value,
        dep_type,
        skip_reason,
    }
}

fn engine_skip_reason_for(
    name: &str,
    value: &DependencySpec,
    current_value: &str,
) -> Option<NpmSkipReason> {
    if matches!(value, DependencySpec::InvalidValue) {
        return Some(NpmSkipReason::InvalidValue);
    }
    if current_value.is_empty() {
        return Some(NpmSkipReason::Empty);
    }
    match name {
        "node" | "npm" | "pnpm" | "vscode" => None,
        "yarn" => (current_value == "disabled").then_some(NpmSkipReason::UnspecifiedVersion),
        _ => Some(NpmSkipReason::UnknownEngines),
    }
}

fn invalid_package_name(name: &str) -> bool {
    if name.is_empty() {
        return true;
    }

    if let Some(rest) = name.strip_prefix('@') {
        let Some((scope, package)) = rest.split_once('/') else {
            return true;
        };
        return scope.is_empty() || package.is_empty() || package.contains('/');
    }

    name.contains('/')
}

/// Classify an npm version string and return the skip reason, if any.
///
/// Returns `None` for plain semver-style constraints that should be looked up
/// in the npm registry.
fn skip_reason_for(value: &str) -> Option<NpmSkipReason> {
    let v = value.trim();

    if v.is_empty() {
        return Some(NpmSkipReason::Empty);
    }
    if v == "latest" {
        return Some(NpmSkipReason::UnspecifiedVersion);
    }

    // workspace protocol (pnpm / yarn)
    if v.starts_with("workspace:") {
        return Some(NpmSkipReason::WorkspaceProtocol);
    }

    // local path references
    if v.starts_with("file:")
        || v.starts_with("link:")
        || v.starts_with("portal:")
        || v.starts_with("patch:")
    {
        return Some(NpmSkipReason::LocalPath);
    }

    // git URL forms
    if v.starts_with("git+")
        || v.starts_with("git://")
        || v.starts_with("github:")
        || v.starts_with("gitlab:")
        || v.starts_with("bitbucket:")
        || v.starts_with("gist:")
        // GitHub shorthand: "owner/repo" (contains exactly one slash, no sigil)
        || (v.contains('/') && !v.starts_with('@') && !v.starts_with("http") && v.split('/').count() == 2)
    {
        return Some(NpmSkipReason::GitSource);
    }

    // URL installs
    if v.starts_with("http://") || v.starts_with("https://") {
        return Some(NpmSkipReason::UrlInstall);
    }

    // npm alias
    if v.starts_with("npm:") {
        return Some(NpmSkipReason::NpmAlias);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_ok(json: &str) -> Vec<NpmExtractedDep> {
        extract(json).expect("parse should succeed")
    }

    fn yarn_config(default_registry: Option<&str>, scopes: &[(&str, Option<&str>)]) -> YarnConfig {
        YarnConfig {
            npm_registry_server: default_registry.map(str::to_owned),
            npm_scopes: scopes
                .iter()
                .map(|(scope, registry)| {
                    (
                        (*scope).to_owned(),
                        YarnScopeConfig {
                            npm_registry_server: registry.map(str::to_owned),
                        },
                    )
                })
                .collect(),
        }
    }

    // Ported: "considers default registry" — npm/extract/yarnrc.spec.ts line 10
    #[test]
    fn yarnrc_resolve_registry_url_considers_default_registry() {
        let config = yarn_config(Some("https://private.example.com/npm"), &[]);
        assert_eq!(
            resolve_yarn_registry_url("a-package", &config).as_deref(),
            Some("https://private.example.com/npm")
        );
    }

    // Ported: "chooses matching scoped registry over default registry" — npm/extract/yarnrc.spec.ts line 17
    #[test]
    fn yarnrc_resolve_registry_url_prefers_matching_scope() {
        let config = yarn_config(
            Some("https://private.example.com/npm"),
            &[("scope", Some("https://scope.example.com/npm"))],
        );
        assert_eq!(
            resolve_yarn_registry_url("@scope/a-package", &config).as_deref(),
            Some("https://scope.example.com/npm")
        );
    }

    // Ported: "ignores non matching scoped registry" — npm/extract/yarnrc.spec.ts line 29
    #[test]
    fn yarnrc_resolve_registry_url_ignores_non_matching_scope() {
        let config = yarn_config(
            None,
            &[("other-scope", Some("https://other-scope.example.com/npm"))],
        );
        assert!(resolve_yarn_registry_url("@scope/a-package", &config).is_none());
    }

    // Ported: "ignores partial scope match" — npm/extract/yarnrc.spec.ts line 40
    #[test]
    fn yarnrc_resolve_registry_url_ignores_partial_scope_match() {
        let config = yarn_config(None, &[("scope", Some("https://scope.example.com/npm"))]);
        assert!(resolve_yarn_registry_url("@scope-2/a-package", &config).is_none());
    }

    // Ported: "ignores missing scope registryServer" — npm/extract/yarnrc.spec.ts line 51
    #[test]
    fn yarnrc_resolve_registry_url_ignores_missing_scope_registry_server() {
        let config = yarn_config(Some("https://private.example.com/npm"), &[("scope", None)]);
        assert!(resolve_yarn_registry_url("@scope/a-package", &config).is_none());
    }

    // Ported: "produces expected config (%s)" — npm/extract/yarnrc.spec.ts line 63
    #[test]
    fn load_config_from_yarnrc_yml_produces_expected_config() {
        let cases = [
            (
                "npmRegistryServer: https://npm.example.com",
                Some(yarn_config(Some("https://npm.example.com"), &[])),
            ),
            (
                "npmRegistryServer: https://npm.example.com\nnpmScopes:\n  foo:\n    npmRegistryServer: https://npm-foo.example.com\n",
                Some(yarn_config(
                    Some("https://npm.example.com"),
                    &[("foo", Some("https://npm-foo.example.com"))],
                )),
            ),
            (
                "npmRegistryServer: https://npm.example.com\nnodeLinker: pnp\n",
                Some(yarn_config(Some("https://npm.example.com"), &[])),
            ),
            ("npmRegistryServer: 42", None),
            ("npmScopes: 42", None),
            ("npmScopes:\n  foo: 42\n", None),
            ("npmScopes:\n  foo:\n    npmRegistryServer: 42\n", None),
            ("", None),
        ];

        for (content, expected) in cases {
            assert_eq!(load_config_from_yarnrc_yml(content), expected);
        }
    }

    // Ported: "produces expected config (%s)" — npm/extract/yarnrc.spec.ts line 117
    #[test]
    fn load_config_from_legacy_yarnrc_produces_expected_config() {
        let cases = [
            (
                "# yarn lockfile v1\nregistry \"https://npm.example.com\"\n",
                yarn_config(Some("https://npm.example.com"), &[]),
            ),
            (
                "disturl \"https://npm-dist.example.com\"\nregistry https://npm.example.com\nsass_binary_site \"https://node-sass.example.com\"\n",
                yarn_config(Some("https://npm.example.com"), &[]),
            ),
            (
                "--install.frozen-lockfile true\n\"registry\" \"https://npm.example.com\"\n\"@foo:registry\" \"https://npm-foo.example.com\"\n\"@bar:registry\" \"https://npm-bar.example.com\"\n",
                yarn_config(
                    Some("https://npm.example.com"),
                    &[
                        ("foo", Some("https://npm-foo.example.com")),
                        ("bar", Some("https://npm-bar.example.com")),
                    ],
                ),
            ),
        ];

        for (content, expected) in cases {
            assert_eq!(load_config_from_legacy_yarnrc(content), expected);
        }
    }

    // Ported: "returns null if failed to parse" — npm/extract/npm.spec.ts line 9
    #[test]
    fn npm_lock_returns_empty_if_failed_to_parse() {
        let lock = parse_npm_lock(Some("abcd"));
        assert!(lock.locked_versions.is_empty());
    }

    // Ported: "extracts" — npm/extract/npm.spec.ts line 15
    #[test]
    fn npm_lock_extracts_v1_dependencies() {
        let lock = parse_npm_lock(Some(
            r#"{
              "lockfileVersion": 1,
              "dependencies": {
                "ansi-styles": { "version": "3.2.1" },
                "chalk": { "version": "2.4.1" },
                "color-convert": { "version": "1.9.1" },
                "color-name": { "version": "1.1.3" },
                "escape-string-regexp": { "version": "1.0.5" },
                "has-flag": { "version": "3.0.0" },
                "supports-color": { "version": "5.4.0" }
              }
            }"#,
        ));

        assert_eq!(lock.lockfile_version, Some(1));
        assert_eq!(lock.locked_versions.len(), 7);
        assert_eq!(
            lock.locked_versions.get("ansi-styles").map(String::as_str),
            Some("3.2.1")
        );
        assert_eq!(
            lock.locked_versions
                .get("supports-color")
                .map(String::as_str),
            Some("5.4.0")
        );
    }

    // Ported: "extracts npm 7 lockfile" — npm/extract/npm.spec.ts line 34
    #[test]
    fn npm_lock_extracts_v2_packages() {
        let lock = parse_npm_lock(Some(
            r#"{
              "lockfileVersion": 2,
              "packages": {
                "": { "name": "root", "version": "1.0.0" },
                "node_modules/ansi-styles": { "version": "3.2.1" },
                "node_modules/chalk": { "version": "2.4.1" },
                "node_modules/color-convert": { "version": "1.9.1" },
                "node_modules/color-name": { "version": "1.1.3" },
                "node_modules/escape-string-regexp": { "version": "1.0.5" },
                "node_modules/has-flag": { "version": "3.0.0" },
                "node_modules/supports-color": { "version": "5.4.0" }
              }
            }"#,
        ));

        assert_eq!(lock.lockfile_version, Some(2));
        assert_eq!(lock.locked_versions.len(), 7);
        assert_eq!(
            lock.locked_versions.get("chalk").map(String::as_str),
            Some("2.4.1")
        );
    }

    // Ported: "extracts npm 9 lockfile" — npm/extract/npm.spec.ts line 53
    #[test]
    fn npm_lock_extracts_v3_packages() {
        let lock = parse_npm_lock(Some(
            r#"{
              "lockfileVersion": 3,
              "packages": {
                "node_modules/ansi-styles": { "version": "3.2.1" },
                "node_modules/chalk": { "version": "2.4.2" },
                "node_modules/color-convert": { "version": "1.9.3" },
                "node_modules/color-name": { "version": "1.1.3" },
                "node_modules/escape-string-regexp": { "version": "1.0.5" },
                "node_modules/has-flag": { "version": "3.0.0" },
                "node_modules/supports-color": { "version": "5.5.0" }
              }
            }"#,
        ));

        assert_eq!(lock.lockfile_version, Some(3));
        assert_eq!(lock.locked_versions.len(), 7);
        assert_eq!(
            lock.locked_versions.get("chalk").map(String::as_str),
            Some("2.4.2")
        );
        assert_eq!(
            lock.locked_versions
                .get("supports-color")
                .map(String::as_str),
            Some("5.5.0")
        );
    }

    // Ported: "returns null if no deps" — npm/extract/npm.spec.ts line 72
    #[test]
    fn npm_lock_returns_empty_if_no_deps() {
        let lock = parse_npm_lock(Some("{}"));
        assert!(lock.locked_versions.is_empty());
    }

    // Ported: "returns null on read error" — npm/extract/npm.spec.ts line 78
    #[test]
    fn npm_lock_returns_empty_on_read_error() {
        let lock = parse_npm_lock(None);
        assert!(lock.locked_versions.is_empty());
    }

    // Ported: "returns empty if exception parsing" — npm/extract/yarn.spec.ts line 10
    #[test]
    fn yarn_lock_returns_empty_if_exception_parsing() {
        let lock = parse_yarn_lock(Some("abcd"));
        assert!(lock.is_yarn1);
        assert_eq!(lock.lockfile_version, None);
        assert!(lock.locked_versions.is_empty());
    }

    // Ported: "extracts yarn 1" — npm/extract/yarn.spec.ts line 17
    #[test]
    fn yarn_lock_extracts_yarn1_dependencies() {
        let lock = parse_yarn_lock(Some(
            r#"
# yarn lockfile v1

ansi-styles@^3.2.1:
  version "3.2.1"
chalk@^2.4.1:
  version "2.4.1"
color-convert@^1.9.0:
  version "1.9.1"
color-name@1.1.3:
  version "1.1.3"
escape-string-regexp@^1.0.5:
  version "1.0.5"
has-flag@^3.0.0:
  version "3.0.0"
supports-color@^5.3.0:
  version "5.4.0"
"#,
        ));
        assert!(lock.is_yarn1);
        assert_eq!(lock.lockfile_version, None);
        assert_eq!(lock.locked_versions.len(), 7);
        assert_eq!(
            lock.locked_versions.get("ansi-styles").map(String::as_str),
            Some("3.2.1")
        );
        assert_eq!(
            lock.locked_versions
                .get("supports-color")
                .map(String::as_str),
            Some("5.4.0")
        );
    }

    // Ported: "extracts yarn 2" — npm/extract/yarn.spec.ts line 27
    #[test]
    fn yarn_lock_extracts_yarn2_dependencies() {
        let lock = parse_yarn_lock(Some(
            r#"
__metadata:
  cacheKey: 8

"@babel/code-frame@npm:^7.0.0":
  version: 7.12.11
"@babel/helper-validator-identifier@npm:^7.10.4":
  version: 7.12.11
"@types/node@npm:^14.14.6":
  version: 14.14.6
"ansi-styles@npm:^4.3.0":
  version: 4.3.0
"chalk@npm:^4.1.0":
  version: 4.1.0
"color-convert@npm:^2.0.1":
  version: 2.0.1
"color-name@npm:~1.1.4":
  version: 1.1.4
"has-flag@npm:^4.0.0":
  version: 4.0.0
"#,
        ));
        assert!(!lock.is_yarn1);
        assert_eq!(lock.lockfile_version, None);
        assert_eq!(lock.locked_versions.len(), 8);
        assert_eq!(
            lock.locked_versions
                .get("@babel/code-frame")
                .map(String::as_str),
            Some("7.12.11")
        );
        assert_eq!(
            lock.locked_versions.get("chalk").map(String::as_str),
            Some("4.1.0")
        );
    }

    // Ported: "extracts yarn 2 cache version" — npm/extract/yarn.spec.ts line 37
    #[test]
    fn yarn_lock_extracts_yarn2_cache_version() {
        let lock = parse_yarn_lock(Some(
            r#"
__metadata:
  version: 6
  cacheKey: 8

"@babel/code-frame@npm:^7.0.0":
  version: 7.12.11
"@babel/helper-validator-identifier@npm:^7.10.4":
  version: 7.12.11
"@types/node@npm:^14.14.6":
  version: 14.14.6
"ansi-styles@npm:^4.3.0":
  version: 4.3.0
"chalk@npm:^4.1.0":
  version: 4.1.0
"color-convert@npm:^2.0.1":
  version: 2.0.1
"color-name@npm:~1.1.4":
  version: 1.1.4
"escape-string-regexp@npm:^1.0.5":
  version: 1.0.5
"has-flag@npm:^4.0.0":
  version: 4.0.0
"supports-color@npm:^7.1.0":
  version: 7.1.0
"#,
        ));
        assert!(!lock.is_yarn1);
        assert_eq!(lock.lockfile_version, Some(6));
        assert_eq!(lock.locked_versions.len(), 10);
        assert_eq!(
            lock.locked_versions
                .get("supports-color")
                .map(String::as_str),
            Some("7.1.0")
        );
    }

    // Ported: "ignores individual invalid entries" — npm/extract/yarn.spec.ts line 47
    #[test]
    fn yarn_lock_ignores_individual_invalid_entries() {
        let lock = parse_yarn_lock(Some(
            r#"
# yarn lockfile v1

1@^1.0.0:
  version "1.0.0"
ansi-styles@^3.2.1:
  version "3.2.1"
chalk@^2.4.1:
  version "2.4.1"
"#,
        ));
        assert!(lock.is_yarn1);
        assert_eq!(lock.locked_versions.len(), 2);
        assert!(!lock.locked_versions.contains_key("1"));
        assert_eq!(
            lock.locked_versions.get("chalk").map(String::as_str),
            Some("2.4.1")
        );
    }

    // Ported: "getYarnVersionFromLock" — npm/extract/yarn.spec.ts line 58
    #[test]
    fn yarn_version_from_lock_matches_lockfile_version() {
        assert_eq!(
            get_yarn_version_from_lock(&YarnLock {
                is_yarn1: true,
                lockfile_version: None,
                locked_versions: BTreeMap::new(),
            }),
            "^1.22.18"
        );
        assert_eq!(
            get_yarn_version_from_lock(&YarnLock {
                is_yarn1: false,
                lockfile_version: Some(12),
                locked_versions: BTreeMap::new(),
            }),
            ">=4.0.0"
        );
        assert_eq!(
            get_yarn_version_from_lock(&YarnLock {
                is_yarn1: false,
                lockfile_version: Some(10),
                locked_versions: BTreeMap::new(),
            }),
            "^4.0.0"
        );
        assert_eq!(
            get_yarn_version_from_lock(&YarnLock {
                is_yarn1: false,
                lockfile_version: Some(8),
                locked_versions: BTreeMap::new(),
            }),
            "^3.0.0"
        );
        assert_eq!(
            get_yarn_version_from_lock(&YarnLock {
                is_yarn1: false,
                lockfile_version: Some(6),
                locked_versions: BTreeMap::new(),
            }),
            "^2.2.0"
        );
        assert_eq!(
            get_yarn_version_from_lock(&YarnLock {
                is_yarn1: false,
                lockfile_version: Some(3),
                locked_versions: BTreeMap::new(),
            }),
            "^2.0.0"
        );
    }

    // Ported: "handles empty catalog entries" — npm/extract/yarn.spec.ts line 78
    #[test]
    fn yarn_catalogs_handles_empty_catalog_entries() {
        let extraction = extract_yarn_catalogs(&BTreeMap::new(), &BTreeMap::new(), None, false);
        assert!(extraction.deps.is_empty());
    }

    // Ported: "parses valid .yarnrc.yml file" — npm/extract/yarn.spec.ts line 86
    #[test]
    fn yarn_catalogs_parses_valid_yarnrc_yml() {
        let default_catalog = BTreeMap::from([("react".to_owned(), "18.3.0".to_owned())]);
        let named_catalogs = BTreeMap::from([(
            "react17".to_owned(),
            BTreeMap::from([("react".to_owned(), "17.0.2".to_owned())]),
        )]);

        let extraction =
            extract_yarn_catalogs(&default_catalog, &named_catalogs, Some("yarn.lock"), true);

        assert_eq!(
            extraction.deps,
            vec![
                YarnCatalogDep {
                    name: "react".to_owned(),
                    current_value: "18.3.0".to_owned(),
                    dep_type: "yarn.catalog.default".to_owned(),
                },
                YarnCatalogDep {
                    name: "react".to_owned(),
                    current_value: "17.0.2".to_owned(),
                    dep_type: "yarn.catalog.react17".to_owned(),
                },
            ]
        );
        assert_eq!(extraction.yarn_lock.as_deref(), Some("yarn.lock"));
        assert!(extraction.has_package_manager);
    }

    // Ported: "finds relevant lockfile" — npm/extract/yarn.spec.ts line 130
    #[test]
    fn yarn_catalogs_finds_relevant_lockfile() {
        let default_catalog = BTreeMap::from([("react".to_owned(), "18.3.1".to_owned())]);
        let extraction =
            extract_yarn_catalogs(&default_catalog, &BTreeMap::new(), Some("yarn.lock"), false);

        assert_eq!(extraction.yarn_lock.as_deref(), Some("yarn.lock"));
        assert!(!extraction.has_package_manager);
    }

    // Ported: "returns empty if no deps" — npm/extract/pnpm.spec.ts line 341
    #[test]
    fn pnpm_workspace_returns_empty_if_no_deps() {
        let extraction =
            extract_pnpm_workspace_file(&BTreeMap::new(), &BTreeMap::new(), &BTreeMap::new(), None);
        assert!(extraction.deps.is_empty());
    }

    // Ported: "handles empty catalog entries" — npm/extract/pnpm.spec.ts line 349
    #[test]
    fn pnpm_workspace_handles_empty_catalog_entries() {
        let extraction =
            extract_pnpm_workspace_file(&BTreeMap::new(), &BTreeMap::new(), &BTreeMap::new(), None);
        assert!(extraction.deps.is_empty());
    }

    // Ported: "parses valid pnpm-workspace.yaml file" — npm/extract/pnpm.spec.ts line 360
    #[test]
    fn pnpm_workspace_parses_valid_workspace_file() {
        let default_catalog = BTreeMap::from([("react".to_owned(), "18.3.0".to_owned())]);
        let named_catalogs = BTreeMap::from([(
            "react17".to_owned(),
            BTreeMap::from([("react".to_owned(), "17.0.2".to_owned())]),
        )]);
        let extraction =
            extract_pnpm_workspace_file(&default_catalog, &named_catalogs, &BTreeMap::new(), None);

        assert_eq!(
            extraction.deps,
            vec![
                PnpmWorkspaceDep {
                    name: "react".to_owned(),
                    current_value: "18.3.0".to_owned(),
                    dep_type: "pnpm.catalog.default".to_owned(),
                    package_name: None,
                },
                PnpmWorkspaceDep {
                    name: "react".to_owned(),
                    current_value: "17.0.2".to_owned(),
                    dep_type: "pnpm.catalog.react17".to_owned(),
                    package_name: None,
                },
            ]
        );
    }

    // Ported: "parses overrides in pnpm-workspace.yaml file" — npm/extract/pnpm.spec.ts line 395
    #[test]
    fn pnpm_workspace_parses_overrides() {
        let overrides = BTreeMap::from([
            ("foo>bar".to_owned(), "2.0.0".to_owned()),
            ("foo@1.0.0".to_owned(), "2.0.0".to_owned()),
            ("foo@>1.0.0".to_owned(), "2.0.0".to_owned()),
            ("foo@>=1.0.0".to_owned(), "2.0.0".to_owned()),
            ("foo@1.0.0>bar".to_owned(), "2.0.0".to_owned()),
            ("foo@>1.0.0>bar".to_owned(), "2.0.0".to_owned()),
            ("foo@>=1.0.0 <2.0.0".to_owned(), ">=2.0.0".to_owned()),
        ]);
        let extraction =
            extract_pnpm_workspace_file(&BTreeMap::new(), &BTreeMap::new(), &overrides, None);

        assert_eq!(extraction.deps.len(), 7);
        assert!(extraction.deps.iter().any(|dep| {
            dep.name == "foo>bar"
                && dep.package_name.as_deref() == Some("bar")
                && dep.dep_type == "pnpm-workspace.overrides"
        }));
        assert!(
            extraction.deps.iter().any(|dep| {
                dep.name == "foo@1.0.0" && dep.package_name.as_deref() == Some("foo")
            })
        );
        assert!(extraction.deps.iter().any(|dep| {
            dep.name == "foo@>=1.0.0 <2.0.0"
                && dep.current_value == ">=2.0.0"
                && dep.package_name.as_deref() == Some("foo")
        }));
        assert!(extraction.deps.iter().any(|dep| {
            dep.name == "foo@>1.0.0>bar" && dep.package_name.as_deref() == Some("bar")
        }));
    }

    // Ported: "finds relevant lockfile" — npm/extract/pnpm.spec.ts line 466
    #[test]
    fn pnpm_workspace_finds_relevant_lockfile() {
        let default_catalog = BTreeMap::from([("react".to_owned(), "18.3.1".to_owned())]);
        let extraction = extract_pnpm_workspace_file(
            &default_catalog,
            &BTreeMap::new(),
            &BTreeMap::new(),
            Some("pnpm-lock.yaml"),
        );

        assert_eq!(
            extraction.pnpm_shrinkwrap.as_deref(),
            Some("pnpm-lock.yaml")
        );
    }

    // Ported: "returns null if cannot parse" — npm/extract/index.spec.ts line 38
    #[test]
    fn package_json_extract_returns_error_if_cannot_parse() {
        assert!(extract("not json").is_err());
    }

    // Ported: "catches invalid names" — npm/extract/index.spec.ts line 47
    #[test]
    fn package_json_invalid_dependency_names_are_skipped() {
        let json = r#"{
          "dependencies": {
            "kgabis/parson": "0.0.0"
          },
          "development": {
            "silentbicycle/greatest": "v1.2.1"
          }
        }"#;
        let deps = extract_ok(json);

        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "kgabis/parson");
        assert_eq!(deps[0].current_value, "0.0.0");
        assert_eq!(deps[0].skip_reason, Some(NpmSkipReason::InvalidName));
    }

    // Ported: "ignores vendorised package.json" — npm/extract/index.spec.ts line 58
    #[test]
    fn package_json_vendorised_installed_package_is_ignored() {
        let json = r#"{
          "_from": "is-object@1.0.1",
          "_id": "is-object@1.0.1",
          "_resolved": "https://registry.npmjs.org/is-object/-/is-object-1.0.1.tgz",
          "devDependencies": {
            "covert": "~1.0.0",
            "jscs": "~1.6.0",
            "tape": "~2.14.0"
          },
          "name": "is-object",
          "version": "1.0.1"
        }"#;
        let deps = extract_ok(json);

        assert!(deps.is_empty());
    }

    #[test]
    fn extracts_all_four_sections() {
        let json = r#"{
          "dependencies": { "express": "^4.18.0" },
          "devDependencies": { "jest": "^29.0" },
          "peerDependencies": { "react": ">=17" },
          "optionalDependencies": { "fsevents": "^2.0" }
        }"#;
        let deps = extract_ok(json);
        assert_eq!(deps.len(), 4);
        assert!(
            deps.iter()
                .any(|d| d.name == "express" && d.dep_type == NpmDepType::Regular)
        );
        assert!(
            deps.iter()
                .any(|d| d.name == "jest" && d.dep_type == NpmDepType::Dev)
        );
        assert!(
            deps.iter()
                .any(|d| d.name == "react" && d.dep_type == NpmDepType::Peer)
        );
        assert!(
            deps.iter()
                .any(|d| d.name == "fsevents" && d.dep_type == NpmDepType::Optional)
        );
    }

    #[test]
    fn plain_semver_has_no_skip_reason() {
        let json =
            r#"{ "dependencies": { "lodash": "4.17.21", "axios": "^1.0", "chalk": "~5.0" } }"#;
        let deps = extract_ok(json);
        assert!(deps.iter().all(|d| d.skip_reason.is_none()));
    }

    #[test]
    fn workspace_protocol_is_skipped() {
        let json = r#"{ "dependencies": { "my-lib": "workspace:*", "other": "workspace:^1.0" } }"#;
        let deps = extract_ok(json);
        assert!(
            deps.iter()
                .all(|d| d.skip_reason == Some(NpmSkipReason::WorkspaceProtocol))
        );
    }

    #[test]
    fn file_reference_is_skipped() {
        let json =
            r#"{ "dependencies": { "local": "file:../local-lib", "linked": "link:../linked" } }"#;
        let deps = extract_ok(json);
        assert!(
            deps.iter()
                .all(|d| d.skip_reason == Some(NpmSkipReason::LocalPath))
        );
    }

    #[test]
    fn git_source_forms_are_skipped() {
        let json = r#"{ "dependencies": {
          "a": "git+https://github.com/owner/repo.git",
          "b": "github:owner/repo",
          "c": "gitlab:owner/repo",
          "d": "owner/repo"
        }}"#;
        let deps = extract_ok(json);
        assert!(
            deps.iter()
                .all(|d| d.skip_reason == Some(NpmSkipReason::GitSource))
        );
    }

    #[test]
    fn url_install_is_skipped() {
        let json = r#"{ "dependencies": { "pkg": "https://example.com/pkg.tgz" } }"#;
        let deps = extract_ok(json);
        assert_eq!(deps[0].skip_reason, Some(NpmSkipReason::UrlInstall));
    }

    #[test]
    fn npm_alias_is_skipped() {
        let json = r#"{ "dependencies": { "react": "npm:preact@^10" } }"#;
        let deps = extract_ok(json);
        assert_eq!(deps[0].skip_reason, Some(NpmSkipReason::NpmAlias));
    }

    #[test]
    fn scoped_package_name_is_not_confused_with_git_shorthand() {
        // "@scope/pkg" contains a slash but starts with "@" — must NOT be treated
        // as a git owner/repo shorthand.
        let json = r#"{ "dependencies": { "@types/node": "^20.0" } }"#;
        let deps = extract_ok(json);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "returns null if no deps" — npm/extract/index.spec.ts line 77
    #[test]
    fn empty_package_json_returns_empty_list() {
        let json = r#"{}"#;
        let deps = extract_ok(json);
        assert!(deps.is_empty());
    }

    // Ported: "handles invalid" — npm/extract/index.spec.ts line 86
    #[test]
    fn package_json_invalid_dependency_sections_return_empty() {
        let json = r#"{"dependencies": true, "devDependencies": []}"#;
        let deps = extract_ok(json);
        assert!(deps.is_empty());
    }

    // Ported: "returns an array of dependencies" — npm/extract/index.spec.ts line 95
    #[test]
    fn package_json_fixture_extracts_dependency_array() {
        let json = r#"{
          "dependencies": {
            "autoprefixer": "6.5.0",
            "bower": "~1.6.0",
            "browserify": "13.1.0",
            "browserify-css": "0.9.2",
            "cheerio": "=0.22.0",
            "config": "1.21.0"
          },
          "devDependencies": {
            "enabled": false,
            "angular": "^1.5.8",
            "angular-touch": "1.5.8",
            "angular-sanitize": "1.5.8",
            "@angular/core": "4.0.0-beta.1"
          },
          "resolutions": {
            "config": "1.21.0",
            "**/@angular/cli": "8.0.0",
            "**/angular": "1.33.0",
            "config/glob": "1.0.0"
          }
        }"#;
        let deps = extract_ok(json);

        assert_eq!(deps.len(), 15);
        for (name, current_value) in [
            ("autoprefixer", "6.5.0"),
            ("bower", "~1.6.0"),
            ("browserify", "13.1.0"),
            ("browserify-css", "0.9.2"),
            ("cheerio", "=0.22.0"),
            ("config", "1.21.0"),
            ("angular", "^1.5.8"),
            ("angular-touch", "1.5.8"),
            ("angular-sanitize", "1.5.8"),
            ("@angular/core", "4.0.0-beta.1"),
            ("@angular/cli", "8.0.0"),
            ("angular", "1.33.0"),
            ("glob", "1.0.0"),
        ] {
            assert!(deps.iter().any(|dep| {
                dep.name == name && dep.current_value == current_value && dep.skip_reason.is_none()
            }));
        }

        let enabled = deps.iter().find(|dep| dep.name == "enabled").unwrap();
        assert_eq!(enabled.skip_reason, Some(NpmSkipReason::InvalidValue));
    }

    // Ported: "returns an array of dependencies with resolution comments" — npm/extract/index.spec.ts line 122
    #[test]
    fn package_json_resolution_comments_are_invalid_names() {
        let json = r#"{
          "dependencies": {
            "autoprefixer": "6.5.0",
            "bower": "~1.6.0",
            "browserify": "13.1.0",
            "browserify-css": "0.9.2",
            "cheerio": "=0.22.0",
            "config": "1.21.0"
          },
          "devDependencies": {
            "enabled": false,
            "angular": "^1.5.8",
            "angular-touch": "1.5.8",
            "angular-sanitize": "1.5.8",
            "@angular/core": "4.0.0-beta.1"
          },
          "resolutions": {
            "//": ["This is a comment"],
            "**/config": "1.21.0"
          }
        }"#;
        let deps = extract_ok(json);

        assert_eq!(deps.len(), 13);
        assert!(deps.iter().any(|dep| {
            dep.name == "config"
                && dep.current_value == "1.21.0"
                && dep.dep_type == NpmDepType::Resolutions
                && dep.skip_reason.is_none()
        }));
        assert!(deps.iter().any(|dep| {
            dep.name.is_empty()
                && dep.dep_type == NpmDepType::Resolutions
                && dep.skip_reason == Some(NpmSkipReason::InvalidName)
        }));
    }

    // Ported: "extracts engines" — npm/extract/index.spec.ts line 412
    #[test]
    fn package_json_extracts_engines() {
        let json = r#"{
          "dependencies": {
            "angular": "1.6.0"
          },
          "devDependencies": {
            "@angular/cli": "1.6.0",
            "foo": "*",
            "bar": "file:../foo/bar",
            "baz": "",
            "other": "latest"
          },
          "engines": {
            "atom": ">=1.7.0 <2.0.0",
            "node": ">= 8.9.2",
            "npm": "^8.0.0",
            "pnpm": "^1.2.0",
            "yarn": "disabled",
            "vscode": ">=1.49.3"
          },
          "main": "index.js"
        }"#;
        let deps = extract_ok(json);

        assert_eq!(deps.len(), 12);
        assert!(deps.iter().any(|dep| {
            dep.name == "angular"
                && dep.current_value == "1.6.0"
                && dep.dep_type == NpmDepType::Regular
                && dep.skip_reason.is_none()
        }));
        assert!(deps.iter().any(|dep| {
            dep.name == "@angular/cli"
                && dep.current_value == "1.6.0"
                && dep.dep_type == NpmDepType::Dev
                && dep.skip_reason.is_none()
        }));
        assert!(deps.iter().any(|dep| {
            dep.name == "bar"
                && dep.current_value == "file:../foo/bar"
                && dep.skip_reason == Some(NpmSkipReason::LocalPath)
        }));
        assert!(deps.iter().any(|dep| {
            dep.name == "baz"
                && dep.current_value.is_empty()
                && dep.skip_reason == Some(NpmSkipReason::Empty)
        }));
        assert!(deps.iter().any(|dep| {
            dep.name == "other"
                && dep.current_value == "latest"
                && dep.skip_reason == Some(NpmSkipReason::UnspecifiedVersion)
        }));

        for (name, current_value) in [
            ("node", ">= 8.9.2"),
            ("npm", "^8.0.0"),
            ("pnpm", "^1.2.0"),
            ("vscode", ">=1.49.3"),
        ] {
            assert!(deps.iter().any(|dep| {
                dep.name == name
                    && dep.current_value == current_value
                    && dep.dep_type == NpmDepType::Engines
                    && dep.skip_reason.is_none()
            }));
        }
        assert!(deps.iter().any(|dep| {
            dep.name == "atom"
                && dep.current_value == ">=1.7.0 <2.0.0"
                && dep.dep_type == NpmDepType::Engines
                && dep.skip_reason == Some(NpmSkipReason::UnknownEngines)
        }));
        assert!(deps.iter().any(|dep| {
            dep.name == "yarn"
                && dep.current_value == "disabled"
                && dep.dep_type == NpmDepType::Engines
                && dep.skip_reason == Some(NpmSkipReason::UnspecifiedVersion)
        }));
    }

    #[test]
    fn missing_sections_are_ignored() {
        let json = r#"{ "dependencies": { "lodash": "^4" } }"#;
        let deps = extract_ok(json);
        assert_eq!(deps.len(), 1);
    }

    #[test]
    fn extracts_yarn_resolutions() {
        let json = r#"{
          "dependencies": { "lodash": "^4.17.0" },
          "resolutions": { "minimist": "^1.2.6", "lodash": ">=4.17.21" }
        }"#;
        let deps = extract_ok(json);
        let resolutions: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == NpmDepType::Resolutions)
            .collect();
        assert_eq!(resolutions.len(), 2);
        assert!(resolutions.iter().any(|d| d.name == "minimist"));
        assert!(resolutions.iter().any(|d| d.name == "lodash"));
    }

    #[test]
    fn extracts_npm_overrides() {
        let json = r#"{
          "overrides": { "semver": "^7.5.2", "tough-cookie": ">=4.1.3" }
        }"#;
        let deps = extract_ok(json);
        let overrides: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == NpmDepType::Overrides)
            .collect();
        assert_eq!(overrides.len(), 2);
        let semver = overrides.iter().find(|d| d.name == "semver").unwrap();
        assert_eq!(semver.current_value, "^7.5.2");
        assert!(semver.skip_reason.is_none());
    }
}
