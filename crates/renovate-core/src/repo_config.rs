//! Repository-level Renovate config discovery and parsing.
//!
//! Renovate reference:
//! - `lib/config/app-strings.ts` `getConfigFileNames()`
//! - `lib/config/options/index.ts` — `enabled`, `ignoreDeps`, `ignorePaths`,
//!   `packageRules`, `matchDepNames`, `matchDatasources`
//!
//! Renovate searches a fixed ordered list of paths inside the repository;
//! the first one found wins. This module ports that list, wires it to the
//! platform client's file-reading capability, and parses the config content
//! into a typed `RepoConfig` struct.

use globset::{Glob, GlobSet, GlobSetBuilder};
use regex::Regex;
use serde::Deserialize;

use crate::versioning::semver_generic::UpdateType;

use crate::config::GlobalConfig;
use crate::platform::{AnyPlatformClient, PlatformError};

#[cfg(test)]
use base64::Engine as _;

/// Ordered list of candidate config file paths, matching Renovate's
/// `configFileNames` constant in `lib/config/app-strings.ts`.
///
/// The `package.json` entry is omitted for now — parsing a `renovate` key
/// inside `package.json` is a separate slice.
pub const CONFIG_FILE_CANDIDATES: &[&str] = &[
    "renovate.json",
    "renovate.json5",
    ".github/renovate.json",
    ".github/renovate.json5",
    ".gitlab/renovate.json",
    ".gitlab/renovate.json5",
    ".renovaterc",
    ".renovaterc.json",
    ".renovaterc.json5",
];

/// A compiled `packageRules` entry.
///
/// Renovate reference: `lib/config/options/index.ts` — `packageRules`.
///
/// Matcher fields are AND-ed together: all set matchers must fire for the rule
/// to apply.  Within each matcher, alternatives are OR-ed (`matchPackageNames`
/// ∪ `matchPackagePatterns`).  Unset matchers (empty list / `None`) are
/// skipped (match-all).
///
/// `matchPackageNames` targets `packageName`; `matchDepNames` targets
/// `depName`.  For most managers these are identical, but for Docker/Helm
/// images they may differ.
#[derive(Debug, Clone)]
pub struct PackageRule {
    /// Package name matchers: exact strings, `/regex/` inline patterns, and
    /// glob patterns (`@angular/**`).  Populated from `matchPackageNames` and
    /// the deprecated `matchPackagePrefixes` (converted to `prefix**` globs).
    /// Targets the `packageName` field of a dependency.
    pub match_package_names: Vec<PackageNameMatcher>,
    /// Compiled regex patterns from the deprecated `matchPackagePatterns` field.
    pub match_package_patterns: Vec<Regex>,
    /// Dep name matchers from `matchDepNames`.
    /// Targets the `depName` field (may differ from `packageName`).
    /// When non-empty, the dep name must match at least one entry.
    pub match_dep_names: Vec<PackageNameMatcher>,
    /// Datasource IDs to match (e.g. `"npm"`, `"pypi"`, `"docker"`).
    /// Empty = all datasources.
    pub match_datasources: Vec<String>,
    /// Limit this rule to specific managers (empty = all managers).
    pub match_managers: Vec<String>,
    /// Update types this rule applies to (`major`, `minor`, `patch`).
    /// Empty = all update types.
    pub match_update_types: Vec<UpdateType>,
    /// Semver range that the proposed new version must satisfy.
    /// When set, updates to versions outside this range are blocked.
    /// Regex patterns (`/pattern/`) are not yet supported.
    pub allowed_versions: Option<String>,
    /// Semver range that the current installed version must satisfy for this
    /// rule to apply.  E.g. `"< 2.0"` means "only apply this rule if we're
    /// currently below 2.0".  Regex patterns not yet supported.
    pub match_current_version: Option<String>,
    /// File name patterns (glob or exact) that the manifest file path must
    /// match for this rule to apply.  Empty = all files.
    pub match_file_names: Vec<String>,
    /// Dep types to match (e.g. `["dependencies"]`, `["devDependencies"]`).
    /// Empty = all dep types.
    pub match_dep_types: Vec<String>,
    /// If `Some(false)`, matching packages are disabled (skipped).
    pub enabled: Option<bool>,
    /// Version strings/ranges/regex patterns to ignore for packages matched
    /// by this rule.  Mirrors `ignoreVersions` in Renovate packageRules.
    pub ignore_versions: Vec<String>,
    /// `true` when the raw config specified at least one `matchPackageNames` /
    /// `matchPackagePatterns` / `matchPackagePrefixes` entry.  Prevents
    /// a fully-invalid pattern list from silently matching all deps.
    pub has_name_constraint: bool,
    /// `true` when the raw config specified at least one `matchDepNames` entry.
    pub has_dep_name_constraint: bool,
}

/// A compiled entry from `matchPackageNames`.
///
/// Modern Renovate treats `matchPackageNames` as a mixed list that can contain:
/// - Exact strings (`"express"`)
/// - Inline `/regex/` patterns (`"/^@angular/"`)
/// - Glob patterns (`"@aws-sdk/**"`)
///
/// The deprecated `matchPackagePrefixes` is converted to glob entries at parse
/// time (`"prefix"` → `"prefix**"`).
#[derive(Debug, Clone)]
pub enum PackageNameMatcher {
    Exact(String),
    Regex(Regex),
    /// Pre-compiled single-pattern glob matcher.
    Glob(globset::GlobMatcher),
}

impl PackageRule {
    /// Return `true` when this rule's name conditions match `dep_name`.
    ///
    /// If neither `matchPackageNames` nor `matchPackagePatterns` is set
    /// (tracked via `has_name_constraint`), the rule matches any package.
    pub fn name_matches(&self, dep_name: &str) -> bool {
        if !self.has_name_constraint {
            return true;
        }
        let name_match = self.match_package_names.iter().any(|m| match m {
            PackageNameMatcher::Exact(s) => s == dep_name,
            PackageNameMatcher::Regex(re) => re.is_match(dep_name),
            PackageNameMatcher::Glob(gm) => gm.is_match(dep_name),
        });
        name_match
            || self
                .match_package_patterns
                .iter()
                .any(|re| re.is_match(dep_name))
    }

    /// Return `true` when this rule's `matchDepNames` condition matches `dep_name`.
    ///
    /// If `matchDepNames` is not set (`has_dep_name_constraint` is false),
    /// returns `true` (matches all).  This is a separate AND condition from
    /// `name_matches`: if both `matchPackageNames` and `matchDepNames` are set,
    /// both must fire.
    pub fn dep_name_matches(&self, dep_name: &str) -> bool {
        if !self.has_dep_name_constraint {
            return true;
        }
        self.match_dep_names.iter().any(|m| match m {
            PackageNameMatcher::Exact(s) => s == dep_name,
            PackageNameMatcher::Regex(re) => re.is_match(dep_name),
            PackageNameMatcher::Glob(gm) => gm.is_match(dep_name),
        })
    }

    /// Return `true` when this rule's `matchDatasources` condition matches `datasource`.
    ///
    /// An empty `matchDatasources` list matches all datasources.
    pub fn datasource_matches(&self, datasource: &str) -> bool {
        self.match_datasources.is_empty() || self.match_datasources.iter().any(|d| d == datasource)
    }

    /// Return `true` if `proposed_version` matches any entry in this rule's
    /// `ignoreVersions` list.
    ///
    /// Entries can be:
    /// - `/regex/` — a regex pattern applied to the version string
    /// - A semver range string — the proposed version must satisfy it to be ignored
    /// - An exact version string — tested as both string equality and semver match
    pub fn version_is_ignored(&self, proposed_version: &str) -> bool {
        version_matches_ignore_list(proposed_version, &self.ignore_versions)
    }

    /// Return `true` when this rule's manager condition matches `manager`.
    ///
    /// An empty `matchManagers` list matches all managers.
    pub fn manager_matches(&self, manager: &str) -> bool {
        self.match_managers.is_empty() || self.match_managers.iter().any(|m| m == manager)
    }

    /// Return `true` when this rule's update type condition matches `update_type`.
    ///
    /// An empty `matchUpdateTypes` list matches all update types.
    pub fn update_type_matches(&self, update_type: UpdateType) -> bool {
        self.match_update_types.is_empty() || self.match_update_types.contains(&update_type)
    }

    /// Return `true` when this rule's dep type condition matches `dep_type`.
    ///
    /// An empty `matchDepTypes` list matches all dep types.
    pub fn dep_type_matches(&self, dep_type: &str) -> bool {
        self.match_dep_types.is_empty() || self.match_dep_types.iter().any(|t| t == dep_type)
    }

    /// Return `true` when `path` matches this rule's `matchFileNames` patterns.
    ///
    /// An empty `matchFileNames` list matches all files.
    /// Glob strings (`*`, `?`, `[`) are compiled as globset patterns;
    /// plain strings are treated as prefix matches.
    pub fn file_name_matches(&self, path: &str) -> bool {
        if self.match_file_names.is_empty() {
            return true;
        }
        PathMatcher::new(&self.match_file_names).is_ignored(path)
    }

    /// Return `true` when `current_value` satisfies this rule's
    /// `matchCurrentVersion` constraint.
    ///
    /// If `matchCurrentVersion` is unset, the rule matches any current version.
    /// The lower bound of `current_value` (stripped of operators) is parsed as
    /// semver and checked against the constraint range.
    /// Regex patterns (`/pattern/`) are silently ignored (treated as matching).
    pub fn current_version_matches(&self, current_value: &str) -> bool {
        use crate::versioning::semver_generic::{lower_bound, parse_padded};
        let Some(ref mcv) = self.match_current_version else {
            return true; // no constraint → matches all
        };
        // Regex patterns not yet supported — treat as matching.
        if mcv.starts_with('/') {
            return true;
        }
        let lb = lower_bound(current_value);
        let Some(current_sv) = parse_padded(lb) else {
            return true; // can't parse → don't restrict
        };
        match semver::VersionReq::parse(mcv) {
            Ok(req) => req.matches(&current_sv),
            Err(_) => true, // unparseable → don't restrict
        }
    }
}

/// Parsed per-repository Renovate configuration.
///
/// Defaults match Renovate's option defaults.
#[derive(Debug, Clone)]
pub struct RepoConfig {
    /// If `false`, Renovate is disabled for this repository entirely.
    /// Defaults to `true`.
    pub enabled: bool,
    /// Dependency names to skip during update lookups.  Exact string match.
    pub ignore_deps: Vec<String>,
    /// File path patterns to exclude from scanning.  Patterns follow
    /// minimatch/globset syntax (`**/test/**`, `**/*.spec.ts`, etc.).  Plain
    /// paths (no glob characters) are treated as prefix matches.
    pub ignore_paths: Vec<String>,
    /// Compiled package rules (from `packageRules` in `renovate.json`).
    pub package_rules: Vec<PackageRule>,
    /// When non-empty, only these manager names are active.
    /// Empty means all managers are active.
    pub enabled_managers: Vec<String>,
    /// Global version ignore list.  If the proposed latest version matches any
    /// entry, the update is suppressed for all packages.
    /// Entries may be semver ranges (`"< 2.0"`) or `/regex/` patterns.
    pub ignore_versions: Vec<String>,
}

/// Compiled path-ignore matcher built from a `RepoConfig`.
///
/// Separates plain-prefix patterns from glob patterns at construction time so
/// matching a single path is O(patterns) rather than building a GlobSet per
/// call.
///
/// Renovate reference: `lib/config/options/index.ts` — `ignorePaths`.
#[derive(Debug)]
pub struct PathMatcher {
    prefixes: Vec<String>,
    globs: GlobSet,
}

impl PathMatcher {
    /// Compile `patterns` into a `PathMatcher`.
    ///
    /// Patterns containing `*`, `?`, or `[` are compiled as globset globs;
    /// all others are treated as path prefixes (trailing `/` is stripped).
    pub fn new(patterns: &[String]) -> Self {
        let mut prefixes = Vec::new();
        let mut glob_builder = GlobSetBuilder::new();

        for raw in patterns {
            let pattern = raw.trim_end_matches('/');
            if pattern.contains('*') || pattern.contains('?') || pattern.contains('[') {
                if let Ok(g) = Glob::new(pattern) {
                    glob_builder.add(g);
                }
            } else {
                prefixes.push(pattern.to_owned());
            }
        }

        let globs = glob_builder.build().unwrap_or_else(|_| {
            GlobSetBuilder::new()
                .build()
                .expect("empty globset always builds")
        });

        PathMatcher { prefixes, globs }
    }

    /// Returns `true` when `path` matches any ignore pattern.
    pub fn is_ignored(&self, path: &str) -> bool {
        if self.globs.is_match(path) {
            return true;
        }
        self.prefixes
            .iter()
            .any(|p| path == p || path.starts_with(&format!("{p}/")))
    }
}

// ── Free helpers ─────────────────────────────────────────────────────────────

/// Compile a single `matchPackageNames` entry into a [`PackageNameMatcher`].
///
/// - `/pattern/` → inline regex
/// - Contains `*`, `?`, or `[` → glob
/// - Otherwise → exact string
fn compile_name_matcher(s: &str) -> PackageNameMatcher {
    // Inline regex: `/pattern/` or `/pattern/flags`
    if s.starts_with('/') {
        let inner = s.trim_start_matches('/');
        let pat = inner
            .trim_end_matches(|c: char| c.is_alphabetic())
            .trim_end_matches('/');
        if let Ok(re) = Regex::new(pat) {
            return PackageNameMatcher::Regex(re);
        }
    }
    // Glob: any glob metacharacter
    if (s.contains('*') || s.contains('?') || s.contains('['))
        && let Ok(g) = globset::Glob::new(s)
    {
        return PackageNameMatcher::Glob(g.compile_matcher());
    }
    PackageNameMatcher::Exact(s.to_owned())
}

/// Return `true` if `proposed_version` is matched by any entry in `ignore_list`.
///
/// Entries may be:
/// - `/regex/` — version string is matched against the regex
/// - A semver range (starts with `<`, `>`, `~`, `^`, `=`, `*`) — the version
///   must satisfy the range to be ignored
/// - An exact version string — checked via string equality only
fn version_matches_ignore_list(proposed_version: &str, ignore_list: &[String]) -> bool {
    use crate::versioning::semver_generic::parse_padded;
    for entry in ignore_list {
        let e = entry.trim();
        // Regex pattern: `/pattern/`
        if e.starts_with('/') {
            let inner = e.trim_start_matches('/');
            let pat = inner
                .trim_end_matches(|c: char| c.is_alphabetic())
                .trim_end_matches('/');
            if let Ok(re) = Regex::new(pat)
                && re.is_match(proposed_version)
            {
                return true;
            }
            continue;
        }
        // Exact string match (always checked first to avoid false positives from
        // semver range parsing — "1.0.0-beta" as a VersionReq matches "1.0.0" stable).
        if e == proposed_version {
            return true;
        }
        // Semver range match: only try when the entry begins with a range operator.
        // This avoids treating exact version strings like "1.0.0-beta" as ranges.
        let first = e.chars().next().unwrap_or(' ');
        if matches!(first, '<' | '>' | '~' | '^' | '=' | '*')
            && let Ok(req) = semver::VersionReq::parse(e)
            && let Some(sv) = parse_padded(proposed_version)
            && req.matches(&sv)
        {
            return true;
        }
    }
    false
}

impl RepoConfig {
    /// Parse the raw content of a `renovate.json` / `.renovaterc` file.
    ///
    /// Supports JSON and JSON5.  Unknown fields are silently ignored.
    /// Returns a default `RepoConfig` (all defaults) when the content is
    /// empty or unparseable.
    pub fn parse(content: &str) -> Self {
        #[derive(Deserialize)]
        struct RawPackageRule {
            #[serde(rename = "matchPackageNames", default)]
            match_package_names: Vec<String>,
            #[serde(rename = "matchPackagePatterns", default)]
            match_package_patterns: Vec<String>,
            /// Deprecated; converted to glob patterns in `matchPackageNames`.
            #[serde(rename = "matchPackagePrefixes", default)]
            match_package_prefixes: Vec<String>,
            #[serde(rename = "matchDepNames", default)]
            match_dep_names: Vec<String>,
            #[serde(rename = "matchDatasources", default)]
            match_datasources: Vec<String>,
            #[serde(rename = "matchManagers", default)]
            match_managers: Vec<String>,
            #[serde(rename = "matchUpdateTypes", default)]
            match_update_types: Vec<String>,
            #[serde(rename = "allowedVersions")]
            allowed_versions: Option<String>,
            #[serde(rename = "matchCurrentVersion")]
            match_current_version: Option<String>,
            #[serde(rename = "matchFileNames", default)]
            match_file_names: Vec<String>,
            #[serde(rename = "matchDepTypes", default)]
            match_dep_types: Vec<String>,
            #[serde(rename = "ignoreVersions", default)]
            ignore_versions: Vec<String>,
            enabled: Option<bool>,
        }

        #[derive(Deserialize)]
        struct Raw {
            #[serde(default = "default_true")]
            enabled: bool,
            #[serde(rename = "ignoreDeps", default)]
            ignore_deps: Vec<String>,
            #[serde(rename = "ignorePaths", default)]
            ignore_paths: Vec<String>,
            #[serde(rename = "packageRules", default)]
            package_rules: Vec<RawPackageRule>,
            #[serde(rename = "enabledManagers", default)]
            enabled_managers: Vec<String>,
            #[serde(rename = "ignoreVersions", default)]
            ignore_versions: Vec<String>,
        }

        fn default_true() -> bool {
            true
        }

        let raw: Raw = match json5::from_str(content) {
            Ok(r) => r,
            Err(e) => {
                tracing::debug!(%e, "failed to parse repo renovate config; using defaults");
                return Self::default();
            }
        };

        let package_rules =
            raw.package_rules
                .into_iter()
                .map(|r| {
                    let has_name_constraint = !r.match_package_names.is_empty()
                        || !r.match_package_patterns.is_empty()
                        || !r.match_package_prefixes.is_empty();

                    // Compile each `matchPackageNames` entry as Exact / Regex / Glob.
                    let mut match_package_names: Vec<PackageNameMatcher> = r
                        .match_package_names
                        .iter()
                        .map(|s| compile_name_matcher(s))
                        .collect();
                    // Convert deprecated `matchPackagePrefixes` → glob `prefix**`.
                    for prefix in &r.match_package_prefixes {
                        let pattern = format!("{prefix}**");
                        match globset::Glob::new(&pattern) {
                            Ok(g) => match_package_names
                                .push(PackageNameMatcher::Glob(g.compile_matcher())),
                            Err(e) => tracing::warn!(
                                prefix,
                                %e,
                                "invalid matchPackagePrefixes glob"
                            ),
                        }
                    }

                    let match_package_patterns = r
                        .match_package_patterns
                        .iter()
                        .filter_map(|pat| {
                            Regex::new(pat)
                                .map_err(|e| {
                                    tracing::warn!(
                                        pattern = pat,
                                        %e,
                                        "invalid packageRules matchPackagePatterns regex"
                                    );
                                })
                                .ok()
                        })
                        .collect();
                    let match_update_types = r
                        .match_update_types
                        .iter()
                        .filter_map(|s| match s.as_str() {
                            "major" => Some(UpdateType::Major),
                            "minor" => Some(UpdateType::Minor),
                            "patch" => Some(UpdateType::Patch),
                            _ => None,
                        })
                        .collect();

                    let has_dep_name_constraint = !r.match_dep_names.is_empty();
                    let match_dep_names: Vec<PackageNameMatcher> = r
                        .match_dep_names
                        .iter()
                        .map(|s| compile_name_matcher(s))
                        .collect();

                    PackageRule {
                        match_package_names,
                        match_package_patterns,
                        match_dep_names,
                        has_dep_name_constraint,
                        match_datasources: r.match_datasources,
                        match_managers: r.match_managers,
                        match_update_types,
                        allowed_versions: r.allowed_versions,
                        match_current_version: r.match_current_version,
                        match_file_names: r.match_file_names,
                        match_dep_types: r.match_dep_types,
                        ignore_versions: r.ignore_versions,
                        enabled: r.enabled,
                        has_name_constraint,
                    }
                })
                .collect();

        Self {
            enabled: raw.enabled,
            ignore_deps: raw.ignore_deps,
            ignore_paths: raw.ignore_paths,
            package_rules,
            enabled_managers: raw.enabled_managers,
            ignore_versions: raw.ignore_versions,
        }
    }

    /// Return `true` when `manager_name` is active under `enabledManagers`.
    ///
    /// When `enabledManagers` is empty, all managers are active.
    pub fn is_manager_enabled(&self, manager_name: &str) -> bool {
        self.enabled_managers.is_empty() || self.enabled_managers.iter().any(|m| m == manager_name)
    }

    /// Return `true` when a dependency name should be ignored.
    ///
    /// Checks both the `ignoreDeps` list (exact match) and any `packageRules`
    /// that set `enabled: false`.  Manager-agnostic: rules with `matchManagers`
    /// are treated as matching all managers.
    pub fn is_dep_ignored(&self, name: &str) -> bool {
        if self.ignore_deps.iter().any(|p| p == name) {
            return true;
        }
        self.package_rules.iter().any(|rule| {
            rule.name_matches(name) && rule.dep_name_matches(name) && rule.enabled == Some(false)
        })
    }

    /// Like [`is_dep_ignored`] but also filters by manager name.
    ///
    /// Rules whose `matchManagers` list is non-empty are only applied when
    /// `manager` appears in that list.
    pub fn is_dep_ignored_for_manager(&self, name: &str, manager: &str) -> bool {
        if self.ignore_deps.iter().any(|p| p == name) {
            return true;
        }
        self.package_rules.iter().any(|rule| {
            rule.name_matches(name)
                && rule.dep_name_matches(name)
                && rule.manager_matches(manager)
                && rule.enabled == Some(false)
        })
    }

    /// Like [`is_dep_ignored_for_manager`] but also checks `matchDepTypes`.
    ///
    /// `dep_type` is the type string from the manifest (e.g. `"devDependencies"`,
    /// `"dependencies"`, `"peerDependencies"`).  An empty `dep_type` matches
    /// any rule regardless of its `matchDepTypes` setting.
    pub fn is_dep_ignored_with_dep_type(&self, name: &str, manager: &str, dep_type: &str) -> bool {
        if self.ignore_deps.iter().any(|p| p == name) {
            return true;
        }
        self.package_rules.iter().any(|rule| {
            rule.name_matches(name)
                && rule.dep_name_matches(name)
                && rule.manager_matches(manager)
                && rule.dep_type_matches(dep_type)
                && rule.enabled == Some(false)
        })
    }

    /// Return `true` when a specific update (name + current + update type + manager)
    /// is blocked by a `packageRules` entry with `enabled: false`.
    ///
    /// Checks `matchPackageNames`, `matchPackagePatterns`, `matchManagers`,
    /// `matchUpdateTypes`, and `matchCurrentVersion`.
    ///
    /// Used in the dep report building phase after fetching update summaries.
    pub fn is_update_blocked(
        &self,
        name: &str,
        current_value: &str,
        update_type: UpdateType,
        manager: &str,
    ) -> bool {
        self.is_update_blocked_for_file(name, current_value, update_type, manager, "")
    }

    /// Like [`is_update_blocked`] but also checks `matchFileNames`.
    pub fn is_update_blocked_for_file(
        &self,
        name: &str,
        current_value: &str,
        update_type: UpdateType,
        manager: &str,
        file_path: &str,
    ) -> bool {
        self.package_rules.iter().any(|rule| {
            rule.name_matches(name)
                && rule.dep_name_matches(name)
                && rule.manager_matches(manager)
                && rule.update_type_matches(update_type)
                && rule.current_version_matches(current_value)
                && rule.file_name_matches(file_path)
                && rule.enabled == Some(false)
        })
    }

    /// Return `true` when `proposed_version` does NOT satisfy the
    /// `allowedVersions` constraint of any matching rule.
    ///
    /// Only semver range strings are supported (regex `/pattern/` values are
    /// silently ignored).  If no rule has `allowedVersions`, this returns
    /// `false` (no restriction).
    pub fn is_version_restricted(&self, name: &str, manager: &str, proposed_version: &str) -> bool {
        self.is_version_restricted_for_file(name, manager, proposed_version, "")
    }

    /// Like [`is_version_restricted`] but also checks `matchFileNames`.
    pub fn is_version_restricted_for_file(
        &self,
        name: &str,
        manager: &str,
        proposed_version: &str,
        file_path: &str,
    ) -> bool {
        use crate::versioning::semver_generic::parse_padded;
        let Some(proposed_sv) = parse_padded(proposed_version) else {
            return false; // can't parse → don't restrict
        };
        self.package_rules.iter().any(|rule| {
            if !rule.name_matches(name)
                || !rule.dep_name_matches(name)
                || !rule.manager_matches(manager)
                || !rule.file_name_matches(file_path)
            {
                return false;
            }
            let Some(ref av) = rule.allowed_versions else {
                return false;
            };
            // Skip regex-style patterns (start with `/`).
            if av.starts_with('/') {
                return false;
            }
            // Parse the semver VersionReq and check the proposed version.
            match semver::VersionReq::parse(av) {
                Ok(req) => !req.matches(&proposed_sv),
                Err(_) => false, // unparseable constraint → don't restrict
            }
        })
    }

    /// Return `true` when `proposed_version` should be ignored according to the
    /// global `ignoreVersions` list or any matching packageRule's `ignoreVersions`.
    ///
    /// The global list is checked first; if it fires, per-rule checks are skipped.
    /// For per-rule checks, the rule must match `name` and `manager` (and optionally
    /// file path) before its `ignoreVersions` list is consulted.
    pub fn is_version_ignored(&self, name: &str, manager: &str, proposed_version: &str) -> bool {
        self.is_version_ignored_for_file(name, manager, proposed_version, "")
    }

    /// Like [`is_version_ignored`] but also checks `matchFileNames`.
    pub fn is_version_ignored_for_file(
        &self,
        name: &str,
        manager: &str,
        proposed_version: &str,
        file_path: &str,
    ) -> bool {
        // Global ignore list applies to all packages.
        if version_matches_ignore_list(proposed_version, &self.ignore_versions) {
            return true;
        }
        // Per-rule ignore list: only applies when the rule matches this dep.
        self.package_rules.iter().any(|rule| {
            rule.name_matches(name)
                && rule.dep_name_matches(name)
                && rule.manager_matches(manager)
                && rule.file_name_matches(file_path)
                && rule.version_is_ignored(proposed_version)
        })
    }
}

impl Default for RepoConfig {
    fn default() -> Self {
        Self {
            enabled: true, // Renovate default is enabled
            ignore_deps: Vec::new(),
            ignore_paths: Vec::new(),
            package_rules: Vec::new(),
            enabled_managers: Vec::new(),
            ignore_versions: Vec::new(),
        }
    }
}

impl RepoConfig {
    /// Build a compiled `PathMatcher` from this config's `ignore_paths`.
    ///
    /// Call this once and reuse the result when checking many paths (e.g. the
    /// repo file list), rather than calling `is_path_ignored` in a loop.
    pub fn build_path_matcher(&self) -> PathMatcher {
        PathMatcher::new(&self.ignore_paths)
    }

    /// Return `true` when a file path should be excluded from scanning.
    ///
    /// Supports globset patterns (`**/test/**`, `**/*.spec.ts`) and plain
    /// path prefixes (`vendor`, `packages/legacy`).  For large file lists,
    /// prefer [`build_path_matcher`] to amortize glob compilation.
    pub fn is_path_ignored(&self, path: &str) -> bool {
        self.build_path_matcher().is_ignored(path)
    }
}

/// Outcome of a repository config discovery attempt.
#[derive(Debug, Clone)]
pub enum RepoConfigResult {
    /// A config file was found; parsed config is ready to use.
    Found { path: String, config: RepoConfig },
    /// No config file exists in the repository.
    NotFound,
    /// The repository has not been onboarded (no config) and
    /// `require_config = Required`.
    NeedsOnboarding,
}

/// Try to find a Renovate config file in the repository.
///
/// Tries each path in [`CONFIG_FILE_CANDIDATES`] in order and returns the
/// first one found. Returns [`RepoConfigResult::NotFound`] or
/// [`RepoConfigResult::NeedsOnboarding`] when none exist.
pub async fn discover(
    client: &AnyPlatformClient,
    owner: &str,
    repo: &str,
    global_config: &GlobalConfig,
) -> Result<RepoConfigResult, PlatformError> {
    for path in CONFIG_FILE_CANDIDATES {
        if let Some(file) = client.get_raw_file(owner, repo, path).await? {
            tracing::debug!(repo = %format!("{owner}/{repo}"), path = %path, "found renovate config");
            let config = RepoConfig::parse(&file.content);
            return Ok(RepoConfigResult::Found {
                path: file.path,
                config,
            });
        }
    }

    tracing::debug!(repo = %format!("{owner}/{repo}"), "no renovate config found");

    use crate::config::RequireConfig;
    if global_config.require_config == RequireConfig::Required {
        Ok(RepoConfigResult::NeedsOnboarding)
    } else {
        Ok(RepoConfigResult::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path as wm_path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::config::GlobalConfig;
    use crate::platform::AnyPlatformClient;
    use crate::platform::github::GithubClient;

    use super::*;

    fn make_client(server_uri: &str) -> AnyPlatformClient {
        AnyPlatformClient::Github(GithubClient::with_endpoint("token", server_uri).unwrap())
    }

    #[tokio::test]
    async fn finds_renovate_json_first() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(wm_path("/repos/owner/repo/contents/renovate.json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "content": base64::engine::general_purpose::STANDARD
                    .encode(r#"{"extends":["config:recommended"]}"#),
                "encoding": "base64"
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let result = discover(&client, "owner", "repo", &GlobalConfig::default())
            .await
            .unwrap();

        assert!(
            matches!(result, RepoConfigResult::Found { ref path, .. } if path == "renovate.json")
        );
    }

    #[tokio::test]
    async fn returns_needs_onboarding_when_no_config_and_required() {
        let server = MockServer::start().await;
        // All file requests return 404
        for candidate in CONFIG_FILE_CANDIDATES {
            Mock::given(method("GET"))
                .and(wm_path(format!("/repos/owner/repo/contents/{candidate}")))
                .respond_with(ResponseTemplate::new(404))
                .mount(&server)
                .await;
        }

        let client = make_client(&server.uri());
        // require_config defaults to Required
        let result = discover(&client, "owner", "repo", &GlobalConfig::default())
            .await
            .unwrap();

        assert!(matches!(result, RepoConfigResult::NeedsOnboarding));
    }

    #[tokio::test]
    async fn returns_not_found_when_optional() {
        use crate::config::RequireConfig;
        let server = MockServer::start().await;
        for candidate in CONFIG_FILE_CANDIDATES {
            Mock::given(method("GET"))
                .and(wm_path(format!("/repos/owner/repo/contents/{candidate}")))
                .respond_with(ResponseTemplate::new(404))
                .mount(&server)
                .await;
        }

        let client = make_client(&server.uri());
        let config = GlobalConfig {
            require_config: RequireConfig::Optional,
            ..GlobalConfig::default()
        };
        let result = discover(&client, "owner", "repo", &config).await.unwrap();
        assert!(matches!(result, RepoConfigResult::NotFound));
    }

    // ── RepoConfig::parse ────────────────────────────────────────────────────

    #[test]
    fn defaults_when_empty() {
        let c = RepoConfig::parse("{}");
        assert!(c.enabled);
        assert!(c.ignore_deps.is_empty());
        assert!(c.ignore_paths.is_empty());
    }

    #[test]
    fn enabled_false() {
        let c = RepoConfig::parse(r#"{"enabled": false}"#);
        assert!(!c.enabled);
    }

    #[test]
    fn ignore_deps_parsed() {
        let c = RepoConfig::parse(r#"{"ignoreDeps": ["lodash", "react"]}"#);
        assert_eq!(c.ignore_deps, vec!["lodash", "react"]);
    }

    #[test]
    fn enabled_managers_parsed() {
        let c = RepoConfig::parse(r#"{"enabledManagers": ["cargo", "npm"]}"#);
        assert_eq!(c.enabled_managers, vec!["cargo", "npm"]);
        assert!(c.is_manager_enabled("cargo"));
        assert!(c.is_manager_enabled("npm"));
        assert!(!c.is_manager_enabled("maven"));
    }

    #[test]
    fn enabled_managers_empty_means_all_active() {
        let c = RepoConfig::parse("{}");
        assert!(c.enabled_managers.is_empty());
        assert!(c.is_manager_enabled("cargo"));
        assert!(c.is_manager_enabled("maven"));
        assert!(c.is_manager_enabled("anything"));
    }

    #[test]
    fn ignore_paths_parsed() {
        let c = RepoConfig::parse(r#"{"ignorePaths": ["test/**", "vendor"]}"#);
        assert_eq!(c.ignore_paths, vec!["test/**", "vendor"]);
    }

    #[test]
    fn json5_comments_are_accepted() {
        let c = RepoConfig::parse(
            r#"{
                // This is a JSON5 comment
                "ignoreDeps": ["jest"], // trailing comma ok in JSON5
            }"#,
        );
        assert_eq!(c.ignore_deps, vec!["jest"]);
    }

    #[test]
    fn malformed_json_returns_defaults() {
        let c = RepoConfig::parse("not valid json at all");
        assert!(c.enabled);
        assert!(c.ignore_deps.is_empty());
    }

    #[test]
    fn is_dep_ignored_matches_exactly() {
        let c = RepoConfig::parse(r#"{"ignoreDeps": ["lodash"]}"#);
        assert!(c.is_dep_ignored("lodash"));
        assert!(!c.is_dep_ignored("lodash-fp"));
        assert!(!c.is_dep_ignored("react"));
    }

    #[test]
    fn is_path_ignored_prefix_match() {
        let c = RepoConfig::parse(r#"{"ignorePaths": ["vendor"]}"#);
        assert!(c.is_path_ignored("vendor/react/index.js"));
        assert!(!c.is_path_ignored("src/vendor.ts"));
    }

    // ── PathMatcher glob tests ────────────────────────────────────────────────

    #[test]
    fn glob_double_star_node_modules() {
        let m = PathMatcher::new(&["**/node_modules/**".to_owned()]);
        assert!(m.is_ignored("node_modules/lodash/index.js"));
        assert!(m.is_ignored("packages/foo/node_modules/bar/index.js"));
        assert!(!m.is_ignored("src/foo.ts"));
    }

    #[test]
    fn glob_spec_files() {
        let m = PathMatcher::new(&["**/*.spec.ts".to_owned()]);
        assert!(m.is_ignored("src/foo.spec.ts"));
        assert!(m.is_ignored("tests/bar.spec.ts"));
        assert!(!m.is_ignored("src/foo.ts"));
        assert!(!m.is_ignored("src/foo.spec.js"));
    }

    #[test]
    fn glob_tests_directory() {
        let m = PathMatcher::new(&["**/test/**".to_owned()]);
        assert!(m.is_ignored("src/test/helpers.ts"));
        assert!(m.is_ignored("test/unit/foo.ts"));
        assert!(!m.is_ignored("src/testing.ts"));
    }

    #[test]
    fn glob_rooted_path_under_dir() {
        let m = PathMatcher::new(&["test/**".to_owned()]);
        assert!(m.is_ignored("test/foo.ts"));
        assert!(!m.is_ignored("src/test/foo.ts")); // rooted glob, not deep
    }

    #[test]
    fn prefix_with_trailing_slash_stripped() {
        let m = PathMatcher::new(&["vendor/".to_owned()]);
        assert!(m.is_ignored("vendor/react/index.js"));
        assert!(!m.is_ignored("src/vendor.ts"));
    }

    #[test]
    fn mixed_glob_and_prefix_patterns() {
        let m = PathMatcher::new(&["**/node_modules/**".to_owned(), "vendor".to_owned()]);
        assert!(m.is_ignored("node_modules/foo/bar.js"));
        assert!(m.is_ignored("vendor/react.js"));
        assert!(!m.is_ignored("src/foo.ts"));
    }

    #[test]
    fn empty_patterns_ignore_nothing() {
        let m = PathMatcher::new(&[]);
        assert!(!m.is_ignored("anything/at/all.ts"));
    }

    #[test]
    fn repo_config_build_path_matcher_uses_globs() {
        let c = RepoConfig::parse(r#"{"ignorePaths": ["**/test/**", "vendor"]}"#);
        let m = c.build_path_matcher();
        assert!(m.is_ignored("src/test/unit.ts"));
        assert!(m.is_ignored("vendor/lib.js"));
        assert!(!m.is_ignored("src/main.ts"));
    }

    // ── packageRules tests ────────────────────────────────────────────────────

    #[test]
    fn package_rules_parsed_from_json() {
        let c = RepoConfig::parse(
            r#"{
                "packageRules": [
                    {
                        "matchPackageNames": ["lodash"],
                        "enabled": false
                    }
                ]
            }"#,
        );
        assert_eq!(c.package_rules.len(), 1);
        assert_eq!(c.package_rules[0].match_package_names.len(), 1);
        assert!(c.package_rules[0].name_matches("lodash"));
        assert!(!c.package_rules[0].name_matches("react"));
        assert_eq!(c.package_rules[0].enabled, Some(false));
    }

    #[test]
    fn package_rules_disable_via_name() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["lodash"], "enabled": false}]}"#,
        );
        assert!(c.is_dep_ignored("lodash"));
        assert!(!c.is_dep_ignored("react"));
    }

    #[test]
    fn package_rules_disable_via_pattern() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackagePatterns": ["^@babel/"], "enabled": false}]}"#,
        );
        assert!(c.is_dep_ignored("@babel/core"));
        assert!(c.is_dep_ignored("@babel/preset-env"));
        assert!(!c.is_dep_ignored("babel-loader"));
    }

    #[test]
    fn package_rules_enabled_true_does_not_ignore() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["lodash"], "enabled": true}]}"#,
        );
        assert!(!c.is_dep_ignored("lodash"));
    }

    #[test]
    fn package_rules_match_managers_respected() {
        let c = RepoConfig::parse(
            r#"{
                "packageRules": [{
                    "matchPackageNames": ["lodash"],
                    "matchManagers": ["npm"],
                    "enabled": false
                }]
            }"#,
        );
        // With manager-aware check, only npm matches
        assert!(c.is_dep_ignored_for_manager("lodash", "npm"));
        assert!(!c.is_dep_ignored_for_manager("lodash", "cargo"));
        // Generic check ignores manager constraint (all managers)
        assert!(c.is_dep_ignored("lodash"));
    }

    #[test]
    fn match_dep_types_filters_by_dep_type() {
        let c = RepoConfig::parse(
            r#"{
                "packageRules": [{
                    "matchDepTypes": ["devDependencies"],
                    "enabled": false
                }]
            }"#,
        );
        // devDeps should be ignored
        assert!(c.is_dep_ignored_with_dep_type("lodash", "npm", "devDependencies"));
        // regular deps should NOT be ignored by this rule
        assert!(!c.is_dep_ignored_with_dep_type("lodash", "npm", "dependencies"));
        // empty dep_type matches any rule (backward-compat)
        assert!(!c.is_dep_ignored_with_dep_type("lodash", "npm", ""));
    }

    #[test]
    fn match_dep_types_empty_matches_all() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["lodash"], "enabled": false}]}"#,
        );
        // No matchDepTypes → matches all dep types
        assert!(c.is_dep_ignored_with_dep_type("lodash", "npm", "dependencies"));
        assert!(c.is_dep_ignored_with_dep_type("lodash", "npm", "devDependencies"));
    }

    #[test]
    fn package_rules_multiple_names_match_any() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["lodash", "moment"], "enabled": false}]}"#,
        );
        assert!(c.is_dep_ignored("lodash"));
        assert!(c.is_dep_ignored("moment"));
        assert!(!c.is_dep_ignored("dayjs"));
    }

    #[test]
    fn package_rules_invalid_regex_skipped() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackagePatterns": ["[invalid"], "enabled": false}]}"#,
        );
        // Invalid pattern is silently skipped — rule has no patterns, matches nothing
        assert!(!c.is_dep_ignored("anything"));
    }

    #[test]
    fn package_rules_no_name_constraint_matches_all() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchManagers": ["cargo"], "enabled": false}]}"#,
        );
        // No name constraint — all packages are disabled for cargo
        assert!(c.is_dep_ignored_for_manager("serde", "cargo"));
        assert!(c.is_dep_ignored_for_manager("tokio", "cargo"));
        // Different manager — not matched
        assert!(!c.is_dep_ignored_for_manager("serde", "npm"));
    }

    // ── matchUpdateTypes tests ────────────────────────────────────────────────

    #[test]
    fn match_update_types_parsed() {
        let c = RepoConfig::parse(
            r#"{
                "packageRules": [{
                    "matchUpdateTypes": ["major"],
                    "enabled": false
                }]
            }"#,
        );
        assert_eq!(c.package_rules.len(), 1);
        assert_eq!(
            c.package_rules[0].match_update_types,
            vec![UpdateType::Major]
        );
    }

    #[test]
    fn is_update_blocked_major_only() {
        let c = RepoConfig::parse(
            r#"{
                "packageRules": [{
                    "matchUpdateTypes": ["major"],
                    "enabled": false
                }]
            }"#,
        );
        assert!(c.is_update_blocked("serde", "1.0.0", UpdateType::Major, "cargo"));
        assert!(!c.is_update_blocked("serde", "1.0.0", UpdateType::Minor, "cargo"));
        assert!(!c.is_update_blocked("serde", "1.0.0", UpdateType::Patch, "cargo"));
    }

    #[test]
    fn is_update_blocked_with_package_name_filter() {
        let c = RepoConfig::parse(
            r#"{
                "packageRules": [{
                    "matchPackageNames": ["serde"],
                    "matchUpdateTypes": ["major"],
                    "enabled": false
                }]
            }"#,
        );
        assert!(c.is_update_blocked("serde", "1.0.0", UpdateType::Major, "cargo"));
        // Different package — not blocked
        assert!(!c.is_update_blocked("tokio", "1.0.0", UpdateType::Major, "cargo"));
    }

    #[test]
    fn is_update_blocked_multiple_types() {
        let c = RepoConfig::parse(
            r#"{
                "packageRules": [{
                    "matchUpdateTypes": ["major", "minor"],
                    "enabled": false
                }]
            }"#,
        );
        assert!(c.is_update_blocked("anything", "1.0.0", UpdateType::Major, "cargo"));
        assert!(c.is_update_blocked("anything", "1.0.0", UpdateType::Minor, "cargo"));
        assert!(!c.is_update_blocked("anything", "1.0.0", UpdateType::Patch, "cargo"));
    }

    #[test]
    fn is_update_blocked_unknown_type_strings_skipped() {
        let c = RepoConfig::parse(
            r#"{
                "packageRules": [{
                    "matchUpdateTypes": ["pin", "digest"],
                    "enabled": false
                }]
            }"#,
        );
        // "pin" and "digest" are not yet supported types — rule has no update type constraint
        // so it matches all update types
        assert!(c.is_update_blocked("serde", "1.0.0", UpdateType::Major, "cargo"));
    }

    // ── allowedVersions tests ─────────────────────────────────────────────────

    #[test]
    fn allowed_versions_restricts_outside_range() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["serde"], "allowedVersions": "< 2.0"}]}"#,
        );
        // 1.9.0 is < 2.0 → allowed
        assert!(!c.is_version_restricted("serde", "cargo", "1.9.0"));
        // 2.0.0 is NOT < 2.0 → restricted
        assert!(c.is_version_restricted("serde", "cargo", "2.0.0"));
    }

    #[test]
    fn allowed_versions_no_rule_means_no_restriction() {
        let c = RepoConfig::parse(r#"{}"#);
        assert!(!c.is_version_restricted("serde", "cargo", "99.0.0"));
    }

    #[test]
    fn allowed_versions_gte_constraint() {
        let c = RepoConfig::parse(r#"{"packageRules": [{"allowedVersions": ">= 1.0.0"}]}"#);
        assert!(!c.is_version_restricted("anything", "cargo", "1.0.0"));
        assert!(!c.is_version_restricted("anything", "cargo", "2.0.0"));
        assert!(c.is_version_restricted("anything", "cargo", "0.9.0"));
    }

    #[test]
    fn allowed_versions_regex_pattern_skipped() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["foo"], "allowedVersions": "/^1\\./"}]}"#,
        );
        // Regex patterns are not yet supported — no restriction applies
        assert!(!c.is_version_restricted("foo", "cargo", "2.0.0"));
    }

    #[test]
    fn allowed_versions_respects_manager_filter() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchManagers": ["npm"], "allowedVersions": "< 2.0"}]}"#,
        );
        assert!(c.is_version_restricted("serde", "npm", "2.0.0"));
        assert!(!c.is_version_restricted("serde", "cargo", "2.0.0"));
    }

    // ── matchCurrentVersion tests ─────────────────────────────────────────────

    #[test]
    fn match_current_version_parsed() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCurrentVersion": "< 2.0", "enabled": false}]}"#,
        );
        assert_eq!(
            c.package_rules[0].match_current_version.as_deref(),
            Some("< 2.0")
        );
    }

    #[test]
    fn match_current_version_blocks_when_below_range() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCurrentVersion": "< 2.0", "enabled": false}]}"#,
        );
        // Current 1.5 satisfies < 2.0 → rule applies
        assert!(c.is_update_blocked("anything", "1.5.0", UpdateType::Major, "cargo"));
        // Current 2.1 does NOT satisfy < 2.0 → rule does not apply
        assert!(!c.is_update_blocked("anything", "2.1.0", UpdateType::Major, "cargo"));
    }

    #[test]
    fn match_current_version_with_caret_range_current() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCurrentVersion": ">= 1.0", "enabled": false}]}"#,
        );
        // current "^1.2.3" has lower bound 1.2.3 which satisfies >= 1.0 → rule applies
        assert!(c.is_update_blocked("pkg", "^1.2.3", UpdateType::Major, "cargo"));
        // current "^0.9.0" lower bound 0.9.0 does NOT satisfy >= 1.0 → rule doesn't apply
        assert!(!c.is_update_blocked("pkg", "^0.9.0", UpdateType::Major, "cargo"));
    }

    #[test]
    fn match_current_version_absent_matches_all() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchUpdateTypes": ["major"], "enabled": false}]}"#,
        );
        // No matchCurrentVersion → applies regardless of current version
        assert!(c.is_update_blocked("pkg", "0.1.0", UpdateType::Major, "npm"));
        assert!(c.is_update_blocked("pkg", "99.0.0", UpdateType::Major, "npm"));
    }

    // ── matchFileNames tests ──────────────────────────────────────────────────

    #[test]
    fn match_file_names_parsed() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchFileNames": ["**/test/**"], "enabled": false}]}"#,
        );
        assert_eq!(c.package_rules[0].match_file_names, vec!["**/test/**"]);
    }

    #[test]
    fn match_file_names_blocks_matching_path() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchFileNames": ["**/test/**"], "enabled": false}]}"#,
        );
        assert!(c.is_update_blocked_for_file(
            "serde",
            "1.0.0",
            UpdateType::Major,
            "cargo",
            "packages/test/Cargo.toml"
        ));
        assert!(!c.is_update_blocked_for_file(
            "serde",
            "1.0.0",
            UpdateType::Major,
            "cargo",
            "packages/main/Cargo.toml"
        ));
    }

    #[test]
    fn match_file_names_exact_match() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchFileNames": ["package.json"], "enabled": false}]}"#,
        );
        assert!(c.is_update_blocked_for_file(
            "lodash",
            "1.0.0",
            UpdateType::Patch,
            "npm",
            "package.json"
        ));
        assert!(!c.is_update_blocked_for_file(
            "lodash",
            "1.0.0",
            UpdateType::Patch,
            "npm",
            "packages/frontend/package.json"
        ));
    }

    #[test]
    fn match_file_names_absent_matches_all_files() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["serde"], "enabled": false}]}"#,
        );
        assert!(c.is_update_blocked_for_file(
            "serde",
            "1.0.0",
            UpdateType::Major,
            "cargo",
            "any/path/Cargo.toml"
        ));
        assert!(c.is_update_blocked_for_file(
            "serde",
            "1.0.0",
            UpdateType::Major,
            "cargo",
            "other/Cargo.toml"
        ));
    }

    // ── matchPackageNames glob / regex / prefix tests ─────────────────────────

    #[test]
    fn match_package_names_glob_pattern() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["@angular/**"], "enabled": false}]}"#,
        );
        assert!(c.is_dep_ignored("@angular/core"));
        assert!(c.is_dep_ignored("@angular/router"));
        assert!(!c.is_dep_ignored("@react/core"));
        assert!(!c.is_dep_ignored("express"));
    }

    #[test]
    fn match_package_names_inline_regex() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["/^@aws-sdk/"], "enabled": false}]}"#,
        );
        assert!(c.is_dep_ignored("@aws-sdk/client-s3"));
        assert!(c.is_dep_ignored("@aws-sdk/credential-providers"));
        assert!(!c.is_dep_ignored("@gcp/storage"));
    }

    #[test]
    fn match_package_prefixes_converted_to_glob() {
        // `matchPackagePrefixes` is a deprecated field — converted to `prefix**` globs.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackagePrefixes": ["@angular/"], "enabled": false}]}"#,
        );
        assert!(c.is_dep_ignored("@angular/core"));
        assert!(c.is_dep_ignored("@angular/router"));
        assert!(!c.is_dep_ignored("@react/core"));
    }

    #[test]
    fn match_package_prefixes_multiple_prefixes() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackagePrefixes": ["@angular/", "@ngrx/"], "enabled": false}]}"#,
        );
        assert!(c.is_dep_ignored("@angular/core"));
        assert!(c.is_dep_ignored("@ngrx/store"));
        assert!(!c.is_dep_ignored("@react/core"));
    }

    // ── ignoreVersions tests ──────────────────────────────────────────────────

    #[test]
    fn global_ignore_versions_exact_match() {
        let c = RepoConfig::parse(r#"{"ignoreVersions": ["1.0.0-beta"]}"#);
        assert!(c.is_version_ignored("lodash", "npm", "1.0.0-beta"));
        assert!(!c.is_version_ignored("lodash", "npm", "1.0.0"));
    }

    #[test]
    fn global_ignore_versions_semver_range() {
        let c = RepoConfig::parse(r#"{"ignoreVersions": ["< 2.0"]}"#);
        assert!(c.is_version_ignored("any", "npm", "1.9.9"));
        assert!(!c.is_version_ignored("any", "npm", "2.0.0"));
        assert!(!c.is_version_ignored("any", "npm", "3.0.0"));
    }

    #[test]
    fn global_ignore_versions_regex() {
        let c = RepoConfig::parse(r#"{"ignoreVersions": ["/beta/", "/rc/"]}"#);
        assert!(c.is_version_ignored("pkg", "npm", "2.0.0-beta.1"));
        assert!(c.is_version_ignored("pkg", "npm", "2.0.0-rc.1"));
        assert!(!c.is_version_ignored("pkg", "npm", "2.0.0"));
    }

    #[test]
    fn package_rule_ignore_versions_scoped_to_matched_package() {
        let c = RepoConfig::parse(
            r#"{
            "packageRules": [{
                "matchPackageNames": ["lodash"],
                "ignoreVersions": ["< 4.0"]
            }]
        }"#,
        );
        // lodash below 4.0 should be ignored
        assert!(c.is_version_ignored("lodash", "npm", "3.9.0"));
        // lodash at 4.0 is fine
        assert!(!c.is_version_ignored("lodash", "npm", "4.0.0"));
        // moment is unaffected by this rule
        assert!(!c.is_version_ignored("moment", "npm", "2.0.0"));
    }

    #[test]
    fn empty_ignore_versions_ignores_nothing() {
        let c = RepoConfig::parse(r#"{}"#);
        assert!(!c.is_version_ignored("any", "npm", "99.0.0-rc.1"));
    }

    // ── matchDepNames ────────────────────────────────────────────────────────

    #[test]
    fn match_dep_names_exact_disables_dep() {
        let c = RepoConfig::parse(
            r#"{
            "packageRules": [{
                "matchDepNames": ["lodash"],
                "enabled": false
            }]
        }"#,
        );
        assert!(c.is_dep_ignored("lodash"));
        assert!(!c.is_dep_ignored("express"));
    }

    #[test]
    fn match_dep_names_regex_disables_dep() {
        let c = RepoConfig::parse(
            r#"{
            "packageRules": [{
                "matchDepNames": ["/^@angular/"],
                "enabled": false
            }]
        }"#,
        );
        assert!(c.is_dep_ignored("@angular/core"));
        assert!(c.is_dep_ignored("@angular/router"));
        assert!(!c.is_dep_ignored("react"));
    }

    #[test]
    fn match_dep_names_glob_disables_dep() {
        let c = RepoConfig::parse(
            r#"{
            "packageRules": [{
                "matchDepNames": ["@aws-sdk/**"],
                "enabled": false
            }]
        }"#,
        );
        assert!(c.is_dep_ignored("@aws-sdk/client-s3"));
        assert!(!c.is_dep_ignored("lodash"));
    }

    #[test]
    fn match_dep_names_and_package_names_both_must_match() {
        // Rule has both matchPackageNames and matchDepNames — both must fire.
        // In our impl, both check against dep_name, so this tests AND logic.
        let c = RepoConfig::parse(
            r#"{
            "packageRules": [{
                "matchPackageNames": ["lodash"],
                "matchDepNames": ["lodash"],
                "enabled": false
            }]
        }"#,
        );
        assert!(c.is_dep_ignored("lodash"));
        // If matchDepNames matched but matchPackageNames didn't — rule should not fire.
        // (Not easily testable without separate package_name concept, but the rule
        // correctly requires both to fire for "lodash".)
    }

    #[test]
    fn match_dep_names_no_constraint_matches_all() {
        // No matchDepNames set → dep_name_matches always true → name_matches still governs.
        let c = RepoConfig::parse(
            r#"{
            "packageRules": [{
                "matchPackageNames": ["lodash"],
                "enabled": false
            }]
        }"#,
        );
        assert!(c.is_dep_ignored("lodash"));
        assert!(!c.is_dep_ignored("express"));
    }

    // ── matchDatasources ─────────────────────────────────────────────────────

    #[test]
    fn match_datasources_method_matches_listed_datasource() {
        let c = RepoConfig::parse(
            r#"{
            "packageRules": [{
                "matchDatasources": ["npm", "pypi"],
                "enabled": false
            }]
        }"#,
        );
        // The datasource_matches method on the compiled rule works correctly.
        let rule = &c.package_rules[0];
        assert!(rule.datasource_matches("npm"));
        assert!(rule.datasource_matches("pypi"));
        assert!(!rule.datasource_matches("docker"));
    }

    #[test]
    fn match_datasources_empty_matches_all() {
        let c = RepoConfig::parse(
            r#"{
            "packageRules": [{
                "matchPackageNames": ["alpine"],
                "enabled": false
            }]
        }"#,
        );
        let rule = &c.package_rules[0];
        // No matchDatasources set → matches any datasource.
        assert!(rule.datasource_matches("docker"));
        assert!(rule.datasource_matches("npm"));
    }
}
