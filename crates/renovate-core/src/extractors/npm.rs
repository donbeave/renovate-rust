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
    /// Dependency is pinned to a moving or malformed source reference.
    UnversionedReference,
    /// Engine name is not handled by Renovate.
    UnknownEngines,
    /// Volta tool name is not handled by Renovate.
    UnknownVolta,
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
    /// pnpm `overrides` override.
    PnpmOverrides,
    /// `engines` constraints.
    Engines,
    /// `volta` tool constraints.
    Volta,
    /// `packageManager` tool constraint.
    PackageManager,
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
            NpmDepType::PnpmOverrides => "pnpm.overrides",
            NpmDepType::Engines => "engines",
            NpmDepType::Volta => "volta",
            NpmDepType::PackageManager => "packageManager",
        }
    }
}

/// A single extracted npm dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NpmExtractedDep {
    /// Package name (the key in the dep section).
    pub name: String,
    /// Registry package name when it differs from the package.json key.
    pub package_name: Option<String>,
    /// Datasource used to look up available versions.
    pub datasource: &'static str,
    /// Source repository URL for non-npm dependencies.
    pub source_url: Option<String>,
    /// Digest for commit-pinned non-npm dependencies.
    pub current_digest: Option<String>,
    /// Original value when current value/digest is normalized.
    pub current_raw_value: Option<String>,
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
    /// npm 8+ `overrides` block.
    #[serde(default)]
    overrides: serde_json::Value,
    /// package runtime/tool constraints.
    #[serde(default, deserialize_with = "deserialize_dependency_section")]
    engines: BTreeMap<String, DependencySpec>,
    /// Volta-pinned runtime/tool versions.
    #[serde(default, deserialize_with = "deserialize_dependency_section")]
    volta: BTreeMap<String, DependencySpec>,
    #[serde(rename = "packageManager")]
    package_manager: Option<String>,
    #[serde(default)]
    pnpm: PnpmPackageJson,
}

#[derive(Debug, Deserialize, Default)]
struct PnpmPackageJson {
    #[serde(default)]
    overrides: serde_json::Value,
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
        (&pkg.engines, NpmDepType::Engines),
        (&pkg.volta, NpmDepType::Volta),
    ] {
        for (name, value) in section {
            out.push(classify(
                normalize_package_key(name, dep_type),
                value,
                dep_type,
            ));
        }
    }
    collect_overrides(&pkg.overrides, NpmDepType::Overrides, &mut out);
    collect_overrides(&pkg.pnpm.overrides, NpmDepType::PnpmOverrides, &mut out);
    if let Some((name, version)) = parse_package_manager(pkg.package_manager.as_deref()) {
        out.push(classify(
            name,
            &DependencySpec::Version(version),
            NpmDepType::PackageManager,
        ));
    }

    Ok(out)
}

fn collect_overrides(
    value: &serde_json::Value,
    dep_type: NpmDepType,
    out: &mut Vec<NpmExtractedDep>,
) {
    let Some(overrides) = value.as_object() else {
        return;
    };

    for (name, value) in overrides {
        collect_override_entry(name, value, dep_type, out);
    }
}

fn collect_override_entry(
    name: &str,
    value: &serde_json::Value,
    dep_type: NpmDepType,
    out: &mut Vec<NpmExtractedDep>,
) {
    if let Some(version) = value.as_str() {
        out.push(classify(
            name.to_owned(),
            &DependencySpec::Version(version.to_owned()),
            dep_type,
        ));
        return;
    }

    let Some(children) = value.as_object() else {
        return;
    };
    for (child_name, child_value) in children {
        let dep_name = if child_name == "." { name } else { child_name };
        collect_override_entry(dep_name, child_value, dep_type, out);
    }
}

fn parse_package_manager(package_manager: Option<&str>) -> Option<(String, String)> {
    let package_manager = package_manager?;
    let (name, version) = package_manager.rsplit_once('@')?;
    if name.is_empty() || version.is_empty() {
        return None;
    }
    Some((name.to_owned(), version.to_owned()))
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
    let mut package_name = None;
    let mut datasource = "npm";
    let mut source_url = None;
    let mut current_digest = None;
    let mut current_raw_value = None;
    let mut current_value = match value {
        DependencySpec::Version(value) => value.clone(),
        DependencySpec::InvalidValue => String::new(),
    };
    let skip_reason = if matches!(value, DependencySpec::Version(_))
        && current_value.starts_with("npm:")
    {
        let alias = parse_npm_alias(&name, &current_value);
        package_name = alias.package_name;
        current_value = alias.current_value;
        alias.skip_reason
    } else if matches!(value, DependencySpec::Version(_))
        && let Some(github_dep) = parse_github_dependency(&current_value)
    {
        datasource = github_dep.datasource;
        source_url = github_dep.source_url;
        current_digest = github_dep.current_digest;
        current_raw_value = github_dep.current_raw_value;
        current_value = github_dep.current_value;
        github_dep.skip_reason
    } else {
        match dep_type {
            NpmDepType::Engines => engine_skip_reason_for(&name, value, &current_value),
            NpmDepType::Volta => volta_skip_reason_for(&name, value, &current_value),
            _ if invalid_package_name(&name) => Some(NpmSkipReason::InvalidName),
            _ if matches!(value, DependencySpec::InvalidValue) => Some(NpmSkipReason::InvalidValue),
            _ => skip_reason_for(&current_value),
        }
    };
    NpmExtractedDep {
        name,
        package_name,
        datasource,
        source_url,
        current_digest,
        current_raw_value,
        current_value,
        dep_type,
        skip_reason,
    }
}

struct ParsedGithubDep {
    datasource: &'static str,
    source_url: Option<String>,
    current_value: String,
    current_digest: Option<String>,
    current_raw_value: Option<String>,
    skip_reason: Option<NpmSkipReason>,
}

fn parse_github_dependency(raw_value: &str) -> Option<ParsedGithubDep> {
    let (repo, reference, raw_for_normalized) = github_repo_and_ref(raw_value)?;
    let Some((owner, name)) = repo.split_once('/') else {
        return Some(skipped_github_dep(
            raw_value,
            NpmSkipReason::UnspecifiedVersion,
        ));
    };
    if owner.is_empty()
        || name.is_empty()
        || owner.starts_with('-')
        || owner.starts_with('@')
        || name.starts_with('@')
    {
        return Some(skipped_github_dep(
            raw_value,
            NpmSkipReason::UnspecifiedVersion,
        ));
    }

    let source_url = format!(
        "https://github.com/{}/{}",
        owner,
        name.trim_end_matches(".git")
    );

    let Some(reference) = reference else {
        return Some(skipped_github_dep(
            raw_value,
            NpmSkipReason::UnspecifiedVersion,
        ));
    };
    if let Some(version) = reference.strip_prefix("semver:") {
        return Some(ParsedGithubDep {
            datasource: "github-tags",
            source_url: Some(source_url),
            current_value: version.to_owned(),
            current_digest: None,
            current_raw_value: None,
            skip_reason: None,
        });
    }
    if reference.starts_with('v') {
        return Some(ParsedGithubDep {
            datasource: "github-tags",
            source_url: Some(source_url),
            current_value: reference.to_owned(),
            current_digest: None,
            current_raw_value: raw_for_normalized.then(|| raw_value.to_owned()),
            skip_reason: None,
        });
    }
    if reference.chars().all(|ch| ch.is_ascii_hexdigit()) && reference.len() >= 7 {
        return Some(ParsedGithubDep {
            datasource: "github-tags",
            source_url: Some(source_url),
            current_value: String::new(),
            current_digest: Some(reference.to_owned()),
            current_raw_value: (raw_for_normalized && reference.len() < 40)
                .then(|| raw_value.to_owned()),
            skip_reason: None,
        });
    }

    Some(skipped_github_dep(
        raw_value,
        NpmSkipReason::UnversionedReference,
    ))
}

fn skipped_github_dep(raw_value: &str, skip_reason: NpmSkipReason) -> ParsedGithubDep {
    ParsedGithubDep {
        datasource: "npm",
        source_url: None,
        current_value: raw_value.to_owned(),
        current_digest: None,
        current_raw_value: None,
        skip_reason: Some(skip_reason),
    }
}

fn github_repo_and_ref(raw_value: &str) -> Option<(String, Option<&str>, bool)> {
    let value = raw_value.strip_prefix("git+").unwrap_or(raw_value);
    let repo_with_ref = value
        .strip_prefix("github:")
        .or_else(|| value.strip_prefix("https://github.com/"))
        .or_else(|| value.strip_prefix("http://github.com/"))
        .or_else(|| value.strip_prefix("git@github.com:"))
        .map(|repo| (repo, true))
        .or_else(|| {
            (value.contains('/')
                && !value.starts_with('@')
                && !value.starts_with("http")
                && !value.starts_with("file:")
                && !value.starts_with("link:")
                && !value.starts_with("portal:")
                && !value.starts_with("patch:")
                && !value.starts_with("gitlab:")
                && !value.starts_with("bitbucket:"))
            .then_some((value, false))
        })?;
    let (repo, ref_part) = repo_with_ref
        .0
        .split_once('#')
        .unwrap_or((repo_with_ref.0, ""));
    let repo = repo.trim_end_matches(".git").to_owned();
    let reference = (!ref_part.is_empty()).then_some(ref_part);
    Some((repo, reference, repo_with_ref.1))
}

struct ParsedNpmAlias {
    package_name: Option<String>,
    current_value: String,
    skip_reason: Option<NpmSkipReason>,
}

fn parse_npm_alias(dep_name: &str, raw_value: &str) -> ParsedNpmAlias {
    let Some(alias) = raw_value.strip_prefix("npm:") else {
        return ParsedNpmAlias {
            package_name: None,
            current_value: raw_value.to_owned(),
            skip_reason: None,
        };
    };

    if let Some((package_name, version)) = split_npm_alias(alias) {
        return ParsedNpmAlias {
            package_name: Some(package_name),
            current_value: version.to_owned(),
            skip_reason: skip_reason_for(version),
        };
    }

    if looks_like_version(alias) {
        return ParsedNpmAlias {
            package_name: Some(dep_name.to_owned()),
            current_value: alias.to_owned(),
            skip_reason: skip_reason_for(alias),
        };
    }

    ParsedNpmAlias {
        package_name: Some(dep_name.to_owned()).filter(|_| !invalid_scoped_alias(alias)),
        current_value: if invalid_scoped_alias(alias) {
            raw_value.to_owned()
        } else {
            alias.to_owned()
        },
        skip_reason: Some(NpmSkipReason::UnspecifiedVersion),
    }
}

fn split_npm_alias(alias: &str) -> Option<(String, &str)> {
    if let Some(scoped) = alias.strip_prefix('@') {
        let slash = scoped.find('/')?;
        let package_end = slash + 1 + scoped[slash + 1..].find('@')?;
        let package_name = format!("@{}", &scoped[..package_end]);
        let version = &scoped[package_end + 1..];
        if invalid_package_name(&package_name) || version.is_empty() {
            return None;
        }
        return Some((package_name, version));
    }

    let (package_name, version) = alias.split_once('@')?;
    if package_name.is_empty() || version.is_empty() {
        return None;
    }
    Some((package_name.to_owned(), version))
}

fn looks_like_version(value: &str) -> bool {
    value
        .chars()
        .next()
        .is_some_and(|ch| ch.is_ascii_digit() || matches!(ch, '^' | '~' | '>' | '<' | '=' | '*'))
}

fn invalid_scoped_alias(alias: &str) -> bool {
    alias
        .strip_prefix('@')
        .and_then(|scoped| scoped.split_once('/'))
        .is_some_and(|(_, package)| package.starts_with('@'))
}

fn volta_skip_reason_for(
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
        "node" | "npm" | "pnpm" => None,
        "yarn" => (current_value == "unknown").then_some(NpmSkipReason::UnspecifiedVersion),
        _ => Some(NpmSkipReason::UnknownVolta),
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
    if v.contains('#') {
        return Some(NpmSkipReason::UnspecifiedVersion);
    }

    // workspace protocol (pnpm / yarn)
    if v.starts_with("workspace:") {
        return Some(NpmSkipReason::WorkspaceProtocol);
    }
    if v.starts_with("gitlab:") {
        return Some(NpmSkipReason::UnspecifiedVersion);
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

/// A catalog entry from `pnpm-workspace.yaml` or yarn catalogs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Catalog {
    pub name: String,
    pub dependencies: Vec<(String, String)>,
}

/// A single extracted catalog dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogDep {
    pub dep_type: String,
    pub dep_name: String,
    pub current_value: String,
    pub datasource: &'static str,
    pub pretty_dep_type: String,
}

/// Extract dependencies from catalog entries.
///
/// Mirrors `lib/modules/manager/npm/extract/common/catalogs.ts` `extractCatalogDeps()`.
pub fn extract_catalog_deps(catalogs: &[Catalog], npm_manager: &str) -> Vec<CatalogDep> {
    let prefix = if npm_manager == "yarn" { "yarn.catalog" } else { "pnpm.catalog" };
    let mut deps = Vec::new();
    for catalog in catalogs {
        let dep_type = format!("{prefix}.{}", catalog.name);
        for (pkg_name, version) in &catalog.dependencies {
            deps.push(CatalogDep {
                dep_name: pkg_name.clone(),
                dep_type: dep_type.clone(),
                current_value: version.clone(),
                datasource: "npm",
                pretty_dep_type: dep_type.clone(),
            });
        }
    }
    deps
}

/// Return `true` if `val` matches any of the given glob `patterns`.
///
/// Mirrors `lib/modules/manager/npm/extract/utils.ts` `matchesAnyPattern()`.
pub fn matches_any_pattern(val: &str, patterns: &[&str]) -> bool {
    patterns.iter().any(|pattern| {
        *pattern == format!("{val}/")
            || globset::Glob::new(pattern)
                .map(|g| g.compile_matcher())
                .is_ok_and(|m| m.is_match(val))
    })
}

/// Return `true` if `file_name` is under `pwd` and its relative path (without
/// the trailing `/package.json`) matches any workspace glob in `workspaces`.
///
/// Mirrors `lib/modules/manager/bun/utils.ts` `fileMatchesWorkspaces()`.
pub fn file_matches_workspaces(pwd: &str, file_name: &str, workspaces: &[&str]) -> bool {
    let Some(rel_full) = file_name.strip_prefix(pwd) else {
        return false;
    };
    let rel_full = rel_full.trim_start_matches('/');
    let rel = rel_full
        .strip_suffix("/package.json")
        .unwrap_or(rel_full.strip_suffix("package.json").unwrap_or(rel_full));

    workspaces.iter().any(|pattern| {
        globset::Glob::new(pattern)
            .map(|g| g.compile_matcher())
            .is_ok_and(|m| m.is_match(rel))
    })
}

/// Filter `files` to those under `pwd` that match any workspace glob.
///
/// Mirrors `lib/modules/manager/bun/utils.ts` `filesMatchingWorkspaces()`.
pub fn files_matching_workspaces<'a>(
    pwd: &str,
    files: &[&'a str],
    workspaces: &[&str],
) -> Vec<&'a str> {
    files
        .iter()
        .copied()
        .filter(|f| file_matches_workspaces(pwd, f, workspaces))
        .collect()
}

/// Bump the `"version"` field in `package.json` content.
///
/// Mirrors `lib/modules/manager/npm/update/package-version/index.ts` `bumpPackageVersion()`.
/// Supports `mirror:<dep>` to mirror a dependency version, or standard semver
/// bump types (`patch`, `minor`, `major`). Returns unchanged content on error.
pub fn bump_npm_package_version(
    content: &str,
    current_value: &str,
    bump_version: &str,
) -> String {
    use std::sync::LazyLock;
    static VERSION_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(r#"(?P<prefix>"version":\s*")[^"]*"#).unwrap()
    });

    let new_version = if let Some(mirror_pkg) = bump_version.strip_prefix("mirror:") {
        let parsed: serde_json::Value = match serde_json::from_str(content) {
            Ok(v) => v,
            Err(_) => return content.to_owned(),
        };
        let mirror_version = parsed
            .get("dependencies")
            .and_then(|d| d.get(mirror_pkg))
            .or_else(|| {
                parsed
                    .get("devDependencies")
                    .and_then(|d| d.get(mirror_pkg))
            })
            .or_else(|| {
                parsed
                    .get("optionalDependencies")
                    .and_then(|d| d.get(mirror_pkg))
            })
            .or_else(|| {
                parsed
                    .get("peerDependencies")
                    .and_then(|d| d.get(mirror_pkg))
            })
            .and_then(|v| v.as_str())
            .map(str::to_owned);
        match mirror_version {
            Some(v) => v,
            None => return content.to_owned(),
        }
    } else {
        let new_ver = (|| -> Option<String> {
            let mut parsed = semver::Version::parse(current_value).ok()?;
            match bump_version {
                "patch" => parsed.patch += 1,
                "minor" => {
                    parsed.minor += 1;
                    parsed.patch = 0;
                }
                "major" => {
                    parsed.major += 1;
                    parsed.minor = 0;
                    parsed.patch = 0;
                }
                _ => return None,
            }
            Some(parsed.to_string())
        })();
        match new_ver {
            Some(v) => v,
            None => return content.to_owned(),
        }
    };

    VERSION_RE
        .replace(content, |caps: &regex::Captures| {
            format!("{}{}", &caps["prefix"], new_version)
        })
        .into_owned()
}

/// Return `true` when `value` is a complex npm range (contains `||`).
fn is_complex_npm_range(value: &str) -> bool {
    value.contains("||")
}

/// Determine the effective npm range strategy.
///
/// Mirrors `lib/modules/manager/npm/range.ts` `getRangeStrategy()`.
pub fn get_range_strategy<'a>(
    range_strategy: &'a str,
    dep_type: Option<&str>,
    current_value: Option<&str>,
) -> &'a str {
    let is_complex = current_value.is_some_and(is_complex_npm_range);
    if range_strategy == "bump" && is_complex {
        return "widen";
    }
    if !range_strategy.is_empty() && range_strategy != "auto" {
        return range_strategy;
    }
    if dep_type == Some("peerDependencies") {
        return "widen";
    }
    if is_complex {
        return "widen";
    }
    "update-lockfile"
}

/// Find the new version for the `node` dep in a list of upgrades.
///
/// Mirrors `lib/modules/manager/npm/post-update/node-version.ts`
/// `getNodeUpdate()`.
pub fn get_node_update<'a>(upgrades: &[(&str, &'a str)]) -> Option<&'a str> {
    upgrades
        .iter()
        .find(|(dep_name, _)| *dep_name == "node")
        .map(|(_, new_value)| *new_value)
}

/// Result of parsing an npm lock file.
#[derive(Debug)]
pub struct NpmParseLockResult {
    /// Detected or default indentation string.
    pub detected_indent: String,
    /// Parsed JSON value, or `None` if the content is invalid JSON.
    pub lock_file_parsed: Option<serde_json::Value>,
}

/// Parse an npm lock file string.
///
/// Mirrors `lib/modules/manager/npm/utils.ts` `parseLockFile()`.
pub fn parse_npm_lock_file(content: &str) -> NpmParseLockResult {
    let detected_indent = detect_json_indent(content);
    let lock_file_parsed = serde_json::from_str(content).ok();
    NpmParseLockResult { detected_indent, lock_file_parsed }
}

/// Serialize an npm lock file value back to a string.
///
/// Mirrors `lib/modules/manager/npm/utils.ts` `composeLockFile()`.
pub fn compose_npm_lock_file(parsed: &serde_json::Value, indent: &str) -> String {
    let formatted = if indent.is_empty() {
        serde_json::to_string(parsed).unwrap_or_default()
    } else {
        let spaces = indent.len();
        serde_json::to_string_pretty(parsed)
            .map(|s| {
                if spaces != 2 {
                    // serde_json::to_string_pretty uses 2-space indent; re-indent if needed
                    s.lines()
                        .map(|l| {
                            let leading = l.len() - l.trim_start().len();
                            let factor = leading / 2;
                            format!("{}{}", indent.repeat(factor), l.trim_start())
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                } else {
                    s
                }
            })
            .unwrap_or_default()
    };
    formatted + "\n"
}

/// Detect the indentation string used in a JSON document.
///
/// Mirrors `detect-indent` npm package behavior: finds the smallest
/// indentation unit used in the file; defaults to two spaces.
fn detect_json_indent(content: &str) -> String {
    let mut min_indent: Option<usize> = None;
    let mut uses_tabs = false;

    for line in content.lines() {
        if line.is_empty() {
            continue;
        }
        let indent_count = line.len() - line.trim_start().len();
        if indent_count == 0 {
            continue;
        }
        if line.starts_with('\t') {
            uses_tabs = true;
            break;
        }
        match min_indent {
            None => min_indent = Some(indent_count),
            Some(m) if indent_count < m => min_indent = Some(indent_count),
            _ => {}
        }
    }

    if uses_tabs {
        return "\t".to_owned();
    }
    match min_indent {
        Some(n) => " ".repeat(n),
        None => "  ".to_owned(),
    }
}

/// Replace the version of a locked dependency in a yarn v1 lock file.
///
/// For yarn 2+ (starts with `__metadata:`), the original content is returned unchanged.
///
/// Mirrors `lib/modules/manager/npm/update/locked-dependency/yarn-lock/replace.ts`
/// `replaceConstraintVersion()`.
pub fn replace_constraint_version(
    lock_file_content: &str,
    dep_name: &str,
    constraint: &str,
    new_version: &str,
    new_constraint: Option<&str>,
) -> String {
    if lock_file_content.starts_with("__metadata:") {
        return lock_file_content.to_owned();
    }

    let dep_name_constraint = format!("{dep_name}@{constraint}");
    // Escape: @, ^, ., \, |
    let mut escaped = String::with_capacity(dep_name_constraint.len() * 2);
    for c in dep_name_constraint.chars() {
        if matches!(c, '@' | '^' | '.' | '\\' | '|') {
            escaped.push('\\');
        }
        escaped.push(c);
    }

    let pattern = format!(
        r#"({escaped}(("|\",|,)[^\n:]*)?:\n)(.*\n)*?(\s+dependencies|\n[@a-z])"#
    );

    let Ok(re) = regex::Regex::new(&pattern) else {
        return lock_file_content.to_owned();
    };

    let result = re.replace(lock_file_content, |caps: &regex::Captures<'_>| {
        let mut constraint_line = caps[1].to_owned();
        if let Some(nc) = new_constraint {
            let new_dep_constraint = format!("{dep_name}@{nc}");
            constraint_line = constraint_line.replace(&dep_name_constraint, &new_dep_constraint);
        }
        let group5 = caps[5].to_owned();
        format!("{constraint_line}  version \"{new_version}\"\n{group5}")
    });

    result.into_owned()
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
          "a": "github:owner/repo",
          "b": "github:owner/repo#master",
          "c": "gitlab:owner/repo",
          "d": "owner/repo",
          "e": "git+https://github.com/owner/repo.git"
        }}"#;
        let deps = extract_ok(json);
        assert_eq!(
            deps.iter().find(|d| d.name == "a").unwrap().skip_reason,
            Some(NpmSkipReason::UnspecifiedVersion)
        );
        assert_eq!(
            deps.iter().find(|d| d.name == "b").unwrap().skip_reason,
            Some(NpmSkipReason::UnversionedReference)
        );
        assert_eq!(
            deps.iter().find(|d| d.name == "c").unwrap().skip_reason,
            Some(NpmSkipReason::UnspecifiedVersion)
        );
        assert_eq!(
            deps.iter().find(|d| d.name == "d").unwrap().skip_reason,
            Some(NpmSkipReason::UnspecifiedVersion)
        );
        assert_eq!(
            deps.iter().find(|d| d.name == "e").unwrap().skip_reason,
            Some(NpmSkipReason::UnspecifiedVersion)
        );
    }

    #[test]
    fn url_install_is_skipped() {
        let json = r#"{ "dependencies": { "pkg": "https://example.com/pkg.tgz" } }"#;
        let deps = extract_ok(json);
        assert_eq!(deps[0].skip_reason, Some(NpmSkipReason::UrlInstall));
    }

    // Ported: "extracts npm package alias" — npm/extract/index.spec.ts line 815
    #[test]
    fn npm_aliases_are_extracted() {
        let json = r#"{
          "dependencies": {
            "a": "npm:foo@1",
            "b": "npm:@foo/bar@1.2.3",
            "c": "npm:^1.2.3",
            "d": "npm:1.2.3",
            "e": "npm:1.x.x",
            "f": "npm:foo",
            "g": "npm:@foo/@bar/@1.2.3"
          }
        }"#;
        let deps = extract_ok(json);

        assert!(deps.iter().any(|dep| {
            dep.name == "a"
                && dep.package_name.as_deref() == Some("foo")
                && dep.current_value == "1"
                && dep.skip_reason.is_none()
        }));
        assert!(deps.iter().any(|dep| {
            dep.name == "b"
                && dep.package_name.as_deref() == Some("@foo/bar")
                && dep.current_value == "1.2.3"
                && dep.skip_reason.is_none()
        }));
        for (name, current_value) in [("c", "^1.2.3"), ("d", "1.2.3"), ("e", "1.x.x")] {
            assert!(deps.iter().any(|dep| {
                dep.name == name
                    && dep.package_name.as_deref() == Some(name)
                    && dep.current_value == current_value
                    && dep.skip_reason.is_none()
            }));
        }
        assert!(deps.iter().any(|dep| {
            dep.name == "f"
                && dep.package_name.as_deref() == Some("f")
                && dep.current_value == "foo"
                && dep.skip_reason == Some(NpmSkipReason::UnspecifiedVersion)
        }));
        assert!(deps.iter().any(|dep| {
            dep.name == "g"
                && dep.package_name.is_none()
                && dep.current_value == "npm:@foo/@bar/@1.2.3"
                && dep.skip_reason == Some(NpmSkipReason::UnspecifiedVersion)
        }));
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

    // Ported: "extracts volta" — npm/extract/index.spec.ts line 503
    #[test]
    fn package_json_extracts_volta() {
        let json = r#"{
          "main": "index.js",
          "engines": {
            "node": "8.9.2"
          },
          "volta": {
            "node": "8.9.2",
            "yarn": "1.12.3",
            "npm": "5.9.0",
            "pnpm": "6.11.2",
            "invalid": "1.0.0"
          }
        }"#;
        let deps = extract_ok(json);

        assert_eq!(deps.len(), 6);
        assert!(deps.iter().any(|dep| {
            dep.name == "node"
                && dep.current_value == "8.9.2"
                && dep.dep_type == NpmDepType::Engines
                && dep.skip_reason.is_none()
        }));
        for (name, current_value) in [
            ("node", "8.9.2"),
            ("yarn", "1.12.3"),
            ("npm", "5.9.0"),
            ("pnpm", "6.11.2"),
        ] {
            assert!(deps.iter().any(|dep| {
                dep.name == name
                    && dep.current_value == current_value
                    && dep.dep_type == NpmDepType::Volta
                    && dep.skip_reason.is_none()
            }));
        }
        assert!(deps.iter().any(|dep| {
            dep.name == "invalid"
                && dep.current_value == "1.0.0"
                && dep.dep_type == NpmDepType::Volta
                && dep.skip_reason == Some(NpmSkipReason::UnknownVolta)
        }));
    }

    // Ported: "extracts volta yarn unspecified-version" — npm/extract/index.spec.ts line 543
    #[test]
    fn package_json_extracts_volta_yarn_unspecified() {
        let json = r#"{
          "main": "index.js",
          "engines": {
            "node": "8.9.2"
          },
          "volta": {
            "node": "8.9.2",
            "yarn": "unknown"
          }
        }"#;
        let deps = extract_ok(json);

        assert!(deps.iter().any(|dep| {
            dep.name == "node"
                && dep.current_value == "8.9.2"
                && dep.dep_type == NpmDepType::Volta
                && dep.skip_reason.is_none()
        }));
        assert!(deps.iter().any(|dep| {
            dep.name == "yarn"
                && dep.current_value == "unknown"
                && dep.dep_type == NpmDepType::Volta
                && dep.skip_reason == Some(NpmSkipReason::UnspecifiedVersion)
        }));
    }

    // Ported: "extracts volta yarn higher than 1" — npm/extract/index.spec.ts line 584
    #[test]
    fn package_json_extracts_volta_yarn_higher_than_one() {
        let json = r#"{
          "main": "index.js",
          "engines": {
            "node": "16.0.0"
          },
          "volta": {
            "node": "16.0.0",
            "yarn": "3.2.4"
          }
        }"#;
        let deps = extract_ok(json);

        assert!(deps.iter().any(|dep| {
            dep.name == "node"
                && dep.current_value == "16.0.0"
                && dep.dep_type == NpmDepType::Volta
                && dep.skip_reason.is_none()
        }));
        assert!(deps.iter().any(|dep| {
            dep.name == "yarn"
                && dep.current_value == "3.2.4"
                && dep.dep_type == NpmDepType::Volta
                && dep.skip_reason.is_none()
        }));
    }

    // Ported: "extracts non-npmjs" — npm/extract/index.spec.ts line 626
    #[test]
    fn package_json_extracts_non_npmjs_github_dependencies() {
        let json = r#"{
          "dependencies": {
            "a": "github:owner/a",
            "b": "github:owner/b#master",
            "c": "github:owner/c#v1.1.0",
            "d": "github:owner/d#a7g3eaf",
            "e": "github:owner/e#49b5aca613b33c5b626ae68c03a385f25c142f55",
            "f": "owner/f#v2.0.0",
            "g": "gitlab:owner/g#v1.0.0",
            "h": "github:-hello/world#v1.0.0",
            "i": "@foo/bar#v2.0.0",
            "j": "github:frank#v0.0.1",
            "k": "github:owner/k#49b5aca",
            "l": "github:owner/l.git#abcdef0",
            "m": "https://github.com/owner/m.git#v1.0.0",
            "n": "git+https://github.com/owner/n#v2.0.0",
            "o": "git@github.com:owner/o.git#v2.0.0",
            "p": "Owner/P.git#v2.0.0",
            "q": "github:owner/q#semver:1.1.0",
            "r": "github:owner/r#semver:^1.0.0"
          }
        }"#;
        let deps = extract_ok(json);

        assert_eq!(deps.len(), 18);
        assert_eq!(
            deps.iter().find(|dep| dep.name == "a").unwrap().skip_reason,
            Some(NpmSkipReason::UnspecifiedVersion)
        );
        assert_eq!(
            deps.iter().find(|dep| dep.name == "b").unwrap().skip_reason,
            Some(NpmSkipReason::UnversionedReference)
        );
        assert_eq!(
            deps.iter().find(|dep| dep.name == "d").unwrap().skip_reason,
            Some(NpmSkipReason::UnversionedReference)
        );
        for (name, current_value, source_url) in [
            ("c", "v1.1.0", "https://github.com/owner/c"),
            ("f", "v2.0.0", "https://github.com/owner/f"),
            ("m", "v1.0.0", "https://github.com/owner/m"),
            ("n", "v2.0.0", "https://github.com/owner/n"),
            ("o", "v2.0.0", "https://github.com/owner/o"),
            ("p", "v2.0.0", "https://github.com/Owner/P"),
            ("q", "1.1.0", "https://github.com/owner/q"),
            ("r", "^1.0.0", "https://github.com/owner/r"),
        ] {
            assert!(deps.iter().any(|dep| {
                dep.name == name
                    && dep.datasource == "github-tags"
                    && dep.current_value == current_value
                    && dep.source_url.as_deref() == Some(source_url)
                    && dep.skip_reason.is_none()
            }));
        }
        for (name, digest, raw) in [
            ("e", "49b5aca613b33c5b626ae68c03a385f25c142f55", None),
            ("k", "49b5aca", Some("github:owner/k#49b5aca")),
            ("l", "abcdef0", Some("github:owner/l.git#abcdef0")),
        ] {
            assert!(deps.iter().any(|dep| {
                dep.name == name
                    && dep.datasource == "github-tags"
                    && dep.current_value.is_empty()
                    && dep.current_digest.as_deref() == Some(digest)
                    && dep.current_raw_value.as_deref() == raw
                    && dep.skip_reason.is_none()
            }));
        }
        for name in ["g", "h", "i", "j"] {
            assert_eq!(
                deps.iter()
                    .find(|dep| dep.name == name)
                    .unwrap()
                    .skip_reason,
                Some(NpmSkipReason::UnspecifiedVersion)
            );
        }
    }

    // Ported: "extracts packageManager" — npm/extract/index.spec.ts line 894
    #[test]
    fn package_json_extracts_package_manager() {
        let json = r#"{
          "packageManager": "yarn@3.0.0"
        }"#;
        let deps = extract_ok(json);

        assert_eq!(deps.len(), 1);
        let yarn = &deps[0];
        assert_eq!(yarn.name, "yarn");
        assert_eq!(yarn.current_value, "3.0.0");
        assert_eq!(yarn.dep_type, NpmDepType::PackageManager);
        assert_eq!(yarn.skip_reason, None);
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

    // Ported: "extracts dependencies from overrides" — npm/extract/index.spec.ts line 957
    #[test]
    fn extracts_npm_overrides() {
        let json = r#"{
          "devDependencies": {
            "@types/react": "18.0.5"
          },
          "overrides": {
            "node": "8.9.2",
            "@types/react": "18.0.5",
            "baz": {
              "node": "8.9.2",
              "bar": {
                "foo": "1.0.0"
              }
            },
            "foo2": {
              ".": "1.0.0",
              "bar2": "1.0.0"
            },
            "emptyObject": {}
          }
        }"#;
        let deps = extract_ok(json);
        assert!(deps.iter().any(|d| {
            d.name == "@types/react" && d.current_value == "18.0.5" && d.dep_type == NpmDepType::Dev
        }));

        let overrides: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == NpmDepType::Overrides)
            .collect();
        assert_eq!(overrides.len(), 6);
        for (name, current_value) in [
            ("node", "8.9.2"),
            ("@types/react", "18.0.5"),
            ("foo", "1.0.0"),
            ("foo2", "1.0.0"),
            ("bar2", "1.0.0"),
        ] {
            assert!(overrides.iter().any(|dep| {
                dep.name == name && dep.current_value == current_value && dep.skip_reason.is_none()
            }));
        }
        assert_eq!(
            overrides
                .iter()
                .filter(|dep| dep.name == "node" && dep.current_value == "8.9.2")
                .count(),
            2
        );
    }

    // Ported: "extracts dependencies from pnpm.overrides" — npm/extract/index.spec.ts line 1036
    #[test]
    fn extracts_pnpm_overrides() {
        let json = r#"{
          "devDependencies": {
            "@types/react": "18.0.5"
          },
          "pnpm": {
            "overrides": {
              "node": "8.9.2",
              "@types/react": "18.0.5",
              "baz": {
                "node": "8.9.2",
                "bar": {
                  "foo": "1.0.0"
                }
              },
              "foo2": {
                ".": "1.0.0",
                "bar2": "1.0.0"
              },
              "emptyObject": {}
            }
          }
        }"#;
        let deps = extract_ok(json);
        assert!(deps.iter().any(|d| {
            d.name == "@types/react" && d.current_value == "18.0.5" && d.dep_type == NpmDepType::Dev
        }));

        let overrides: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == NpmDepType::PnpmOverrides)
            .collect();
        assert_eq!(overrides.len(), 6);
        for (name, current_value) in [
            ("node", "8.9.2"),
            ("@types/react", "18.0.5"),
            ("foo", "1.0.0"),
            ("foo2", "1.0.0"),
            ("bar2", "1.0.0"),
        ] {
            assert!(overrides.iter().any(|dep| {
                dep.name == name && dep.current_value == current_value && dep.skip_reason.is_none()
            }));
        }
        assert_eq!(
            overrides
                .iter()
                .filter(|dep| dep.name == "node" && dep.current_value == "8.9.2")
                .count(),
            2
        );
    }

    // Ported: "extracts dependencies from pnpm.overrides, with version ranges in flat syntax" — npm/extract/index.spec.ts line 1117
    #[test]
    fn extracts_pnpm_override_range_keys() {
        let json = r#"{
          "pnpm": {
            "overrides": {
              "foo>bar": "2.0.0",
              "foo@1.0.0": "2.0.0",
              "foo@>1.0.0": "2.0.0",
              "foo@>=1.0.0": "2.0.0",
              "foo@1.0.0>bar": "2.0.0",
              "foo@>1.0.0>bar": "2.0.0",
              "foo@>=1.0.0 <2.0.0": ">=2.0.0"
            }
          }
        }"#;
        let deps = extract_ok(json);
        let overrides: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == NpmDepType::PnpmOverrides)
            .collect();

        assert_eq!(overrides.len(), 7);
        for (name, current_value) in [
            ("foo>bar", "2.0.0"),
            ("foo@1.0.0", "2.0.0"),
            ("foo@>1.0.0", "2.0.0"),
            ("foo@>=1.0.0", "2.0.0"),
            ("foo@1.0.0>bar", "2.0.0"),
            ("foo@>1.0.0>bar", "2.0.0"),
            ("foo@>=1.0.0 <2.0.0", ">=2.0.0"),
        ] {
            assert!(overrides.iter().any(|dep| {
                dep.name == name && dep.current_value == current_value && dep.skip_reason.is_none()
            }));
        }
    }

    // Ported: "returns same if not auto" — modules/manager/npm/range.spec.ts line 4
    #[test]
    fn npm_range_returns_same_if_not_auto() {
        assert_eq!(get_range_strategy("widen", None, None), "widen");
    }

    // Ported: "widens peerDependencies" — modules/manager/npm/range.spec.ts line 8
    #[test]
    fn npm_range_widens_peer_dependencies() {
        let result = get_range_strategy("auto", Some("peerDependencies"), None);
        assert_eq!(result, "widen");
    }

    // Ported: "widens complex ranges" — modules/manager/npm/range.spec.ts line 16
    #[test]
    fn npm_range_widens_complex_ranges() {
        let result = get_range_strategy("auto", Some("dependencies"), Some("^1.6.0 || ^2.0.0"));
        assert_eq!(result, "widen");
    }

    // Ported: "widens complex bump" — modules/manager/npm/range.spec.ts line 24
    #[test]
    fn npm_range_widens_complex_bump() {
        let result = get_range_strategy("bump", Some("dependencies"), Some("^1.6.0 || ^2.0.0"));
        assert_eq!(result, "widen");
    }

    // Ported: "defaults to update-lockfile" — modules/manager/npm/range.spec.ts line 32
    #[test]
    fn npm_range_defaults_to_update_lockfile() {
        let result = get_range_strategy("auto", Some("dependencies"), None);
        assert_eq!(result, "update-lockfile");
    }

    // Ported: "matches package in nested directory" — modules/manager/npm/extract/utils.spec.ts line 5
    #[test]
    fn matches_any_pattern_nested_directory() {
        assert!(matches_any_pattern(
            "packages/group/a/package.json",
            &["packages/**"]
        ));
    }

    // Ported: "matches package in non-nested directory" — modules/manager/npm/extract/utils.spec.ts line 14
    #[test]
    fn matches_any_pattern_non_nested_directory() {
        assert!(matches_any_pattern(
            "non-nested-packages/a/package.json",
            &["non-nested-packages/*/*"]
        ));
    }

    // Ported: "matches package in explicitly defined directory" — modules/manager/npm/extract/utils.spec.ts line 23
    #[test]
    fn matches_any_pattern_explicit_directory() {
        assert!(matches_any_pattern(
            "solo-package/package.json",
            &["solo-package/*"]
        ));
    }

    // Ported: "should return false when fileName does not start with pwd" — modules/manager/bun/utils.spec.ts line 7
    #[test]
    fn bun_file_matches_workspaces_false_when_different_pwd() {
        let result = file_matches_workspaces("/project", "/another-path/package.json", &["**"]);
        assert!(!result);
    }

    // Ported: "should correctly evaluate fileName when it starts with pwd" — modules/manager/bun/utils.spec.ts line 13
    #[test]
    fn bun_file_matches_workspaces_true_when_starts_with_pwd() {
        let result = file_matches_workspaces("/project", "/project/foo/package.json", &["foo"]);
        assert!(result);
    }

    // Ported: "should filter files matching workspaces and pwd" — modules/manager/bun/utils.spec.ts line 26
    #[test]
    fn bun_files_matching_workspaces_filters_correctly() {
        let pwd = "/project";
        let files = vec![
            "/project/foo/package.json",
            "/project/bar/package.json",
            "/other/baz/package.json",
        ];
        let workspaces = ["foo", "bar"];
        let result = files_matching_workspaces(pwd, &files, &workspaces);
        assert_eq!(
            result,
            vec!["/project/foo/package.json", "/project/bar/package.json"]
        );
    }

    fn sample_catalogs() -> Vec<Catalog> {
        vec![
            Catalog {
                name: "default".to_string(),
                dependencies: vec![("react".to_string(), "17.0.2".to_string())],
            },
            Catalog {
                name: "custom".to_string(),
                dependencies: vec![("lodash".to_string(), "4.17.21".to_string())],
            },
        ]
    }

    // Ported: "returns correct dependencies for pnpm" — modules/manager/npm/extract/common/catalogs.spec.ts line 5
    #[test]
    fn catalog_deps_for_pnpm() {
        let result = extract_catalog_deps(&sample_catalogs(), "pnpm");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].dep_type, "pnpm.catalog.default");
        assert_eq!(result[0].dep_name, "react");
        assert_eq!(result[0].current_value, "17.0.2");
        assert_eq!(result[0].datasource, "npm");
        assert_eq!(result[1].dep_type, "pnpm.catalog.custom");
        assert_eq!(result[1].dep_name, "lodash");
    }

    // Ported: "returns correct dependencies for yarn" — modules/manager/npm/extract/common/catalogs.spec.ts line 37
    #[test]
    fn catalog_deps_for_yarn() {
        let result = extract_catalog_deps(&sample_catalogs(), "yarn");
        assert_eq!(result[0].dep_type, "yarn.catalog.default");
        assert_eq!(result[1].dep_type, "yarn.catalog.custom");
    }

    // Ported: "handles empty catalogs list" — modules/manager/npm/extract/common/catalogs.spec.ts line 69
    #[test]
    fn catalog_deps_empty_list() {
        assert!(extract_catalog_deps(&[], "pnpm").is_empty());
        assert!(extract_catalog_deps(&[], "yarn").is_empty());
    }

    // Ported: "handles catalog with no dependencies" — modules/manager/npm/extract/common/catalogs.spec.ts line 76
    #[test]
    fn catalog_deps_empty_dependencies() {
        let catalogs = vec![Catalog {
            name: "empty".to_string(),
            dependencies: vec![],
        }];
        assert!(extract_catalog_deps(&catalogs, "pnpm").is_empty());
        assert!(extract_catalog_deps(&catalogs, "yarn").is_empty());
    }

    const NPM_PKG_CONTENT: &str =
        r#"{"name":"some-package","version":"0.0.2","dependencies":{"chalk":"2.4.2"}}"#;

    // Ported: "mirrors" — modules/manager/npm/update/package-version/index.spec.ts line 16
    #[test]
    fn npm_bump_mirrors_dependency_version() {
        let result = bump_npm_package_version(NPM_PKG_CONTENT, "0.0.2", "mirror:chalk");
        assert!(result.contains("\"version\":\"2.4.2\"") || result.contains("\"version\": \"2.4.2\""));
        assert_ne!(result, NPM_PKG_CONTENT);
    }

    // Ported: "aborts mirror" — modules/manager/npm/update/package-version/index.spec.ts line 24
    #[test]
    fn npm_bump_aborts_mirror_when_dep_not_found() {
        let result = bump_npm_package_version(NPM_PKG_CONTENT, "0.0.2", "mirror:a");
        assert_eq!(result, NPM_PKG_CONTENT);
    }

    // Ported: "increments" — modules/manager/npm/update/package-version/index.spec.ts line 31
    #[test]
    fn npm_bump_increments_patch() {
        let result = bump_npm_package_version(NPM_PKG_CONTENT, "0.0.2", "patch");
        assert!(result.contains("\"version\":\"0.0.3\"") || result.contains("\"version\": \"0.0.3\""));
        assert_ne!(result, NPM_PKG_CONTENT);
    }

    // Ported: "no ops" — modules/manager/npm/update/package-version/index.spec.ts line 38
    #[test]
    fn npm_bump_no_op_when_bumped_version_matches_content() {
        let result = bump_npm_package_version(NPM_PKG_CONTENT, "0.0.1", "patch");
        assert_eq!(result, NPM_PKG_CONTENT);
    }

    // Ported: "updates" — modules/manager/npm/update/package-version/index.spec.ts line 44
    #[test]
    fn npm_bump_updates_minor() {
        let result = bump_npm_package_version(NPM_PKG_CONTENT, "0.0.1", "minor");
        assert!(result.contains("\"version\":\"0.1.0\"") || result.contains("\"version\": \"0.1.0\""));
        assert_ne!(result, NPM_PKG_CONTENT);
    }

    // Ported: "returns content if bumping errors" — modules/manager/npm/update/package-version/index.spec.ts line 51
    #[test]
    fn npm_bump_returns_content_on_invalid_bump_type() {
        let result = bump_npm_package_version(NPM_PKG_CONTENT, "0.0.2", "invalid_type");
        assert_eq!(result, NPM_PKG_CONTENT);
    }

    // Ported: "returns version" — modules/manager/npm/post-update/node-version.spec.ts line 101
    #[test]
    fn npm_get_node_update_returns_version() {
        let upgrades = [("node", "16.15.0")];
        assert_eq!(get_node_update(&upgrades), Some("16.15.0"));
    }

    // Ported: "returns undefined" — modules/manager/npm/post-update/node-version.spec.ts line 107
    #[test]
    fn npm_get_node_update_returns_none_for_empty() {
        let upgrades: &[(&str, &str)] = &[];
        assert!(get_node_update(upgrades).is_none());
    }

    const NPM_PACKAGE_LOCK: &str =
        include_str!("../../tests/fixtures/npm/lockfile-parsing/package-lock.json");

    // Ported: "parses lockfile string into an object" — modules/manager/npm/utils.spec.ts line 18
    #[test]
    fn npm_parse_lock_file_parses_into_object() {
        let result = parse_npm_lock_file(NPM_PACKAGE_LOCK);
        assert_eq!(result.detected_indent, "  ");
        let parsed = result.lock_file_parsed.unwrap();
        assert_eq!(parsed["lockfileVersion"], 2);
        assert_eq!(parsed["name"], "lockfile-parsing");
        assert_eq!(parsed["version"], "1.0.0");
        assert_eq!(parsed["requires"], true);
        assert_eq!(parsed["packages"][""]["license"], "ISC");
    }

    // Ported: "can deal with invalid lockfiles" — modules/manager/npm/utils.spec.ts line 32
    #[test]
    fn npm_parse_lock_file_invalid_returns_none() {
        let result = parse_npm_lock_file("");
        assert_eq!(result.detected_indent, "  ");
        assert!(result.lock_file_parsed.is_none());
    }

    // Ported: "composes lockfile string out of an object" — modules/manager/npm/utils.spec.ts line 39
    #[test]
    fn npm_compose_lock_file_serializes_with_indent() {
        let val = serde_json::json!({
            "lockfileVersion": 2,
            "name": "lockfile-parsing",
            "packages": {
                "": {
                    "license": "ISC",
                    "name": "lockfile-parsing",
                    "version": "1.0.0"
                }
            },
            "requires": true,
            "version": "1.0.0"
        });
        let composed = compose_npm_lock_file(&val, "  ");
        assert!(composed.ends_with('\n'));
        let reparsed: serde_json::Value = serde_json::from_str(&composed).unwrap();
        assert_eq!(reparsed["name"], "lockfile-parsing");
        assert_eq!(reparsed["lockfileVersion"], 2);
    }

    // Ported: "adds trailing newline to match npms behavior and avoid diffs" — modules/manager/npm/utils.spec.ts line 49
    #[test]
    fn npm_compose_lock_file_round_trips_fixture() {
        let result = parse_npm_lock_file(NPM_PACKAGE_LOCK);
        let composed = compose_npm_lock_file(result.lock_file_parsed.as_ref().unwrap(), &result.detected_indent);
        assert_eq!(composed, NPM_PACKAGE_LOCK);
    }

    // ── yarn-lock replace tests ─────────────────────────────────────────────

    static YARN_LOCK1: &str = include_str!("../../tests/fixtures/yarn-lock/express.yarn.lock");
    static YARN_LOCK2: &str = include_str!("../../tests/fixtures/yarn-lock/2.yarn.lock");
    static YARN2_LOCK: &str = include_str!("../../tests/fixtures/yarn-lock/yarn2.lock");

    // Ported: "returns same if Yarn 2+" — modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts line 11
    #[test]
    fn yarn_replace_returns_same_for_yarn2() {
        let res = replace_constraint_version(YARN2_LOCK, "chalk", "^2.4.1", "2.5.0", None);
        assert_eq!(res, YARN2_LOCK);
    }

    // Ported: "replaces without dependencies" — modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts line 18
    #[test]
    fn yarn_replace_without_dependencies() {
        let res = replace_constraint_version(YARN_LOCK1, "fresh", "~0.2.1", "0.2.5", None);
        assert_ne!(res, YARN_LOCK1);
        assert!(res.contains("  version \"0.2.5\""), "expected new version in result");
        assert!(!res.contains("  version \"0.2.4\""), "old version should be gone");
        assert!(!res.contains("resolved \"https://registry.yarnpkg.com/fresh/-/fresh-0.2.4"), "old resolved line should be gone");
        // constraint line preserved
        assert!(res.contains("fresh@~0.2.1:\n  version \"0.2.5\""), "constraint line must be preserved");
    }

    // Ported: "replaces with dependencies" — modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts line 34
    #[test]
    fn yarn_replace_with_dependencies() {
        let res = replace_constraint_version(YARN_LOCK1, "express", "4.0.0", "4.4.0", None);
        assert_ne!(res, YARN_LOCK1);
        assert!(res.contains("  version \"4.4.0\""), "expected new version");
        assert!(!res.contains("  version \"4.0.0\""), "old version should be gone");
        assert!(!res.contains("resolved \"https://registry.yarnpkg.com/express/-/express-4.0.0"), "old resolved line should be gone");
        // dependencies section preserved
        assert!(res.contains("express@4.0.0:\n  version \"4.4.0\"\n  dependencies:"), "dependencies must follow new version");
    }

    // Ported: "replaces constraint too" — modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts line 51
    #[test]
    fn yarn_replace_constraint_too() {
        let res = replace_constraint_version(YARN_LOCK1, "express", "4.0.0", "4.4.0", Some("4.4.0"));
        assert_ne!(res, YARN_LOCK1);
        assert!(res.contains("express@4.4.0:\n  version \"4.4.0\""), "constraint + version must be updated");
        assert!(!res.contains("express@4.0.0:"), "old constraint must be gone");
    }

    // Ported: "handles escaped constraints" — modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts line 70
    #[test]
    fn yarn_replace_handles_escaped_constraints() {
        let res = replace_constraint_version(YARN_LOCK2, "string-width", "^1.0.1 || ^2.0.0", "2.2.0", None);
        assert_ne!(res, YARN_LOCK2);
        assert!(res.contains("  version \"2.2.0\""), "expected new version");
        assert!(!res.contains("  version \"2.0.0\""), "old version should be gone");
        assert!(!res.contains("resolved \"https://registry.yarnpkg.com/string-width/-/string-width-2.1.1"), "old resolved should be gone");
    }

    // Ported: "handles quoted" — modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts line 94
    #[test]
    fn yarn_replace_handles_quoted() {
        let res = replace_constraint_version(YARN_LOCK2, "@embroider/addon-shim", "^0.48.0", "0.48.1", None);
        assert_ne!(res, YARN_LOCK2);
        assert!(res.contains("  version \"0.48.1\""), "expected new version");
        assert!(!res.contains("  version \"0.48.0\""), "old version should be gone");
    }
}
