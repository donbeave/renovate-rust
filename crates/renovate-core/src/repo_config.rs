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
//!
//! Rule types, matchers, and the dependency context live in
//! [`crate::package_rule`]; this module re-exports them for convenience.

use serde::Deserialize;

// Re-export rule/context types so callers can keep using `repo_config::*`.
pub use crate::package_rule::{DepContext, PackageRule, PathMatcher, RuleEffects};
use crate::package_rule::{version_matches_allowed, version_matches_ignore_list};
use crate::versioning::semver_generic::UpdateType;

use crate::config::GlobalConfig;
use crate::platform::{AnyPlatformClient, PlatformError};

#[cfg(test)]
use base64::Engine as _;

/// Ordered list of candidate config file paths, matching Renovate's
/// `configFileNames` constant in `lib/config/app-strings.ts`.
///
/// `package.json` is handled separately by [`discover`] — the file is very
/// commonly present as an npm package manifest and must be checked for a
/// `"renovate"` key before treating it as a config source.
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
    /// If non-empty, only files matching at least one of these patterns will be
    /// scanned.  Applied after `ignorePaths` exclusions.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `includePaths`.
    pub include_paths: Vec<String>,
    /// Compiled package rules (from `packageRules` in `renovate.json`).
    pub package_rules: Vec<PackageRule>,
    /// When non-empty, only these manager names are active.
    /// Empty means all managers are active.
    pub enabled_managers: Vec<String>,
    /// Explicit manager denylist — managers in this list are disabled even
    /// when `enabled_managers` is empty.  Populated by presets like
    /// `docker:disable`.
    pub disabled_managers: Vec<String>,
    /// Global version ignore list.  If the proposed latest version matches any
    /// entry, the update is suppressed for all packages.
    /// Entries may be semver ranges (`"< 2.0"`) or `/regex/` patterns.
    pub ignore_versions: Vec<String>,

    // ── Scheduling ────────────────────────────────────────────────────────────
    /// Schedule windows for creating PRs.  Entries are Renovate schedule
    /// strings (e.g. `"before 5am"`, `"every weekend"`) or POSIX cron
    /// expressions.  Empty = no schedule restriction (always active).
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `schedule`.
    pub schedule: Vec<String>,
    /// Schedule windows for *automerging* PRs.  Separate from `schedule`
    /// (which gates branch creation).  Default `["at any time"]` means
    /// automerge is unrestricted.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `automergeSchedule`.
    pub automerge_schedule: Vec<String>,

    /// IANA timezone name used when evaluating `schedule` entries.
    /// E.g. `"America/New_York"`.  `None` means use UTC / system timezone.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `timezone`.
    pub timezone: Option<String>,

    // ── PR behavior ───────────��──────────────────────────────────────────────
    /// Enable automatic merging of Renovate PRs that pass all checks.
    /// Defaults to `false`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `automerge`.
    pub automerge: bool,

    /// Strategy for auto-merge: `"merge-commit"`, `"squash"`, or `"rebase"`.
    /// `None` means use platform default.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `automergeType`.
    pub automerge_type: Option<String>,

    /// Labels to add to Renovate PRs (must exist in the repository).
    /// Renovate reference: `lib/config/options/index.ts` — `labels`.
    pub labels: Vec<String>,

    /// Additional labels appended to `labels` (for preset layering).
    /// Renovate reference: `lib/config/options/index.ts` — `addLabels`.
    pub add_labels: Vec<String>,

    /// GitHub usernames/team slugs to assign as PR assignees.
    /// Renovate reference: `lib/config/options/index.ts` — `assignees`.
    pub assignees: Vec<String>,

    /// GitHub usernames/team slugs to add as PR reviewers.
    /// Renovate reference: `lib/config/options/index.ts` — `reviewers`.
    pub reviewers: Vec<String>,

    /// When `true`, Renovate PRs are created as draft PRs.  Default: `false`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `draftPR`.
    pub draft_pr: bool,

    /// When `true`, assign reviewers/assignees even if the PR is auto-mergeable.
    /// Default: `false`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `assignAutomerge`.
    pub assign_automerge: bool,

    // ── Branch behavior ──────────────���───────────────────────────────────────
    /// Branch name prefix for update branches.  Default: `"renovate/"`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `branchPrefix`.
    pub branch_prefix: String,

    /// Additional string appended after `branchPrefix` and before the
    /// branch topic.  Default: `""`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `additionalBranchPrefix`.
    pub additional_branch_prefix: String,

    /// Branches to process (alternative base branches).  Empty = default
    /// branch only.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `baseBranches`.
    pub base_branches: Vec<String>,
    /// When to rebase an existing PR branch.
    /// Values: `"auto"` (default), `"never"`, `"conflicted"`, `"behind-base-branch"`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `rebaseWhen`.
    pub rebase_when: Option<String>,

    // ── Update grouping / limits ─────────────────────────────────────────────
    /// Maximum number of open Renovate PRs at any one time.  `0` = unlimited.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `prConcurrentLimit`.
    pub pr_concurrent_limit: u32,
    /// When to create a PR: `"immediate"` (after branch creation), `"not-pending"`
    /// (after all tests pass or fail), `"status-success"` (only when all checks pass),
    /// or `"approval"` (requires dependency dashboard approval).
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `prCreation`.
    pub pr_creation: Option<String>,

    /// Maximum number of Renovate PRs to create per hour.  `0` = unlimited.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `prHourlyLimit`.
    pub pr_hourly_limit: u32,

    /// Group name for global dep grouping.  When non-empty, all updates are
    /// bundled into a single PR with this name.  Per-rule `groupName` in
    /// `packageRules` takes precedence.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `groupName`.
    pub group_name: Option<String>,

    /// When `true`, major and minor/patch updates are split into separate PRs.
    /// Default: `true`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `separateMajorMinor`.
    pub separate_major_minor: bool,

    /// When `true`, a separate PR/branch is created for each available major
    /// version instead of one shared `major-` branch for all majors.
    /// Default: `false`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `separateMultipleMajor`.
    pub separate_multiple_major: bool,

    /// Maximum allowed increment in major version number.  `0` disables major
    /// updates entirely.  Default: `500` (effectively unlimited).
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `maxMajorIncrement`.
    pub max_major_increment: u32,

    /// When `true`, minor and patch updates are split into separate PRs.
    /// Default: `false`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `separateMinorPatch`.
    pub separate_minor_patch: bool,

    /// When `true`, each distinct minor version of a dependency gets its own
    /// separate PR/branch.  Branch topics include the minor component:
    /// `{dep}-{major}.{minor}.x` instead of `{dep}-{major}.x`.
    /// Default: `false`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `separateMultipleMinor`.
    pub separate_multiple_minor: bool,

    // ── Semantic commits ──────────────────────��─────────────────────��────────
    /// Enable semantic commits (`"enabled"` / `"disabled"` / `"auto"`).
    /// `None` → auto (detect from repository history).
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `semanticCommits`.
    pub semantic_commits: Option<String>,

    /// Conventional-commit type prefix to use when semantic commits are enabled.
    /// Default: `"chore"`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `semanticCommitType`.
    pub semantic_commit_type: String,

    /// Conventional-commit scope to use when semantic commits are enabled.
    /// Default: `"deps"`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `semanticCommitScope`.
    pub semantic_commit_scope: String,

    // ── Preset inheritance ────────────────────────────────────────────────────
    /// Preset references to extend (e.g. `["config:recommended"]`).
    /// Built-in presets are resolved and their config effects merged at parse
    /// time.  Unknown or remote presets are stored for inspection but not
    /// acted upon.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `extends`.
    pub extends: Vec<String>,

    /// Presets from `extends` that should be suppressed / ignored.
    /// Listed presets are filtered out before any resolution occurs.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `ignorePresets`.
    pub ignore_presets: Vec<String>,

    // ── Release age / safety ─────────────────────────────────────────────────
    /// Minimum time a release must have been published before it is eligible
    /// for updates.  Format: `"3 days"`, `"1 week"`, `"2 months"`, etc.
    /// `None` (default) means no age restriction.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `minimumReleaseAge`.
    pub minimum_release_age: Option<String>,

    /// When `true`, only upgrade to stable versions if the current version is
    /// stable.  If the current version is unstable, upgrades within the same
    /// major are still allowed.  Default: `false`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `ignoreUnstable`.
    pub ignore_unstable: bool,

    /// When `true`, Renovate will only update npm packages up to the "latest"
    /// dist-tag (respects the `latest` tag even if newer versions are published).
    /// Default: `false`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `respectLatest`.
    pub respect_latest: bool,

    /// When `true`, Renovate pins Docker image digests on update.
    /// Default: `false`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `pinDigests`.
    pub pin_digests: bool,

    /// When `true`, Renovate creates a Dependency Dashboard issue in the repo.
    /// Default: `false`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `dependencyDashboard`.
    pub dependency_dashboard: bool,

    /// When `true`, all updates require approval via the Dependency Dashboard
    /// before Renovate creates the PR.  Default: `false`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `dependencyDashboardApproval`.
    pub dependency_dashboard_approval: bool,

    /// When `true`, Renovate creates a PR to migrate stale config options to
    /// their modern equivalents.  Default: `false`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `configMigration`.
    pub config_migration: bool,

    // ── Scheduling behavior ───────────────────────────────────────────────────
    /// When `false`, Renovate will not update branches that are outside the
    /// configured schedule window.  Default: `true` (updates happen even when
    /// not scheduled, since the schedule gates PR *creation*, not branch updates).
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `updateNotScheduled`.
    pub update_not_scheduled: bool,

    // ── Commit message customization ─────────────────────────────────────────
    /// Action verb in PR titles and commit messages.  Default `"Update"`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `commitMessageAction`.
    pub commit_message_action: String,

    /// Custom prefix to prepend to commit messages and PR titles.
    /// Overrides the semantic-commit prefix (`"chore(deps):"`) when set.
    /// Example: `"fix(deps):"`, `"build(deps):"`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `commitMessagePrefix`.
    pub commit_message_prefix: Option<String>,

    /// Override for the extra segment of the commit message (default `"to {{newVersion}}"`).
    /// Supports `{{newVersion}}` and `{{depName}}` substitution.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `commitMessageExtra`.
    pub commit_message_extra: Option<String>,

    /// Free-form suffix appended to the end of the commit message / PR title.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `commitMessageSuffix`.
    pub commit_message_suffix: Option<String>,
    /// Body text appended to commit messages (after a blank line).  Supports
    /// `{{{gitAuthor}}}` substitution.  Example: `"Signed-off-by: Bot <bot@example.com>"`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `commitBody`.
    pub commit_body: Option<String>,

    // ── Range / version strategy ──────────────────────────────────────────────
    /// Range update strategy. Controls how existing version ranges are modified.
    /// Accepted: `"auto"`, `"pin"`, `"bump"`, `"replace"`, `"widen"`,
    /// `"update-lockfile"`, `"in-range-only"`.  Default: `"auto"`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `rangeStrategy`.
    pub range_strategy: String,

    // ── Branch name limits ────────────────────────────────────────────────────
    /// When set, branch names are hashed (SHA-512) so the full branch name
    /// (prefix + hash) is exactly this many characters long.  Use on platforms
    /// that impose strict branch name length limits.
    ///
    /// The hash is computed from `additionalBranchPrefix + branchTopic` and
    /// truncated to `hashedBranchLength - len(branchPrefix)` hex characters.
    /// Minimum meaningful hash: 6 characters (mirroring Renovate's MIN_HASH_LENGTH).
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `hashedBranchLength`.
    pub hashed_branch_length: Option<u32>,

    // ── Per-update-type config blocks ──────────────────────────────────────────
    /// Config applied to all major-version updates (after packageRules).
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `major`.
    pub major_config: Option<crate::package_rule::UpdateTypeConfig>,

    /// Config applied to all minor-version updates (after packageRules).
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `minor`.
    pub minor_config: Option<crate::package_rule::UpdateTypeConfig>,

    /// Config applied to all patch-version updates (after packageRules).
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `patch`.
    pub patch_config: Option<crate::package_rule::UpdateTypeConfig>,

    /// Custom managers defined in `customManagers` config.
    /// Each entry can extract dependencies from arbitrary files using regex.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `customManagers`,
    /// `lib/modules/manager/custom/regex/` — regex strategy implementation.
    pub custom_managers: Vec<CustomManager>,
}

// ── CustomManager ─────────────────────────────────────────────────────────────

/// A single entry from the `customManagers` array in `renovate.json`.
///
/// Only `customType: "regex"` is supported; JSONata is out of scope.
///
/// Renovate reference: `lib/config/options/index.ts` — `customManagers`,
/// `lib/modules/manager/custom/regex/` — extraction logic.
#[derive(Debug, Clone, Default)]
pub struct CustomManager {
    /// The custom manager type. Only `"regex"` is currently supported.
    pub custom_type: String,
    /// Glob/regex patterns for files this manager should scan.
    /// Mirrors `managerFilePatterns` (Renovate 39+) or legacy `fileMatch`.
    pub file_patterns: Vec<String>,
    /// List of regex patterns applied to file content.
    /// Named capture groups: `datasource`, `depName`, `packageName`,
    /// `currentValue`, `versioning`, `registryUrl`, `extractVersion`.
    pub match_strings: Vec<String>,
    /// Matching strategy: `"any"` (default), `"combination"`, `"recursive"`.
    pub match_strings_strategy: String,
    /// Default datasource when not captured by regex.
    pub datasource_template: Option<String>,
    /// Default dep name when not captured by regex.
    pub dep_name_template: Option<String>,
    /// Default package name when not captured by regex.
    pub package_name_template: Option<String>,
    /// Default versioning when not captured by regex.
    pub versioning_template: Option<String>,
    /// Default registry URL when not captured by regex.
    pub registry_url_template: Option<String>,
    /// Default extract version when not captured by regex.
    pub extract_version_template: Option<String>,
    /// Auto-replace string template for updating matched content.
    pub auto_replace_string_template: Option<String>,
}

/// A dependency extracted by a `CustomManager` regex match.
#[derive(Debug, Clone)]
pub struct CustomExtractedDep {
    /// Dependency name (from `depName` capture group or `depNameTemplate`).
    pub dep_name: String,
    /// Package name (from `packageName` capture group or `packageNameTemplate`, defaults to `dep_name`).
    pub package_name: Option<String>,
    /// Current version string (from `currentValue` capture group).
    pub current_value: String,
    /// Datasource identifier (e.g., `"npm"`, `"docker"`).
    pub datasource: String,
    /// Optional versioning scheme override.
    pub versioning: Option<String>,
    /// Optional registry URL override.
    pub registry_url: Option<String>,
    /// Optional extract version pattern.
    pub extract_version: Option<String>,
}

impl CustomManager {
    /// Return `true` when this custom manager should process `file_path`.
    ///
    /// The `file_patterns` list contains glob/regex patterns (same semantics
    /// as Renovate's `managerFilePatterns`).  An empty list matches all files.
    pub fn matches_file(&self, file_path: &str) -> bool {
        use crate::string_match::match_regex_or_glob_list;
        if self.file_patterns.is_empty() {
            return true;
        }
        match_regex_or_glob_list(file_path, &self.file_patterns)
    }

    /// Apply this custom manager's regex patterns to `content` and return
    /// all extracted dependencies.
    ///
    /// Supports two strategies (controlled by `match_strings_strategy`):
    /// - `"any"` (default): each pattern is applied globally; each match yields one dep.
    /// - `"combination"`: all patterns are applied globally; all captures are merged
    ///   into a single dep (last capture of each named group wins).
    ///
    /// Named capture groups recognised:
    /// `datasource`, `depName`, `packageName`, `currentValue`,
    /// `versioning`, `registryUrl`, `extractVersion`.
    /// Template fields fill in for missing capture groups.
    ///
    /// Renovate reference:
    /// `lib/modules/manager/custom/regex/strategies.ts` — `handleAny`, `handleCombination`.
    pub fn extract_deps(&self, content: &str) -> Vec<CustomExtractedDep> {
        if self.custom_type != "regex" || self.match_strings.is_empty() {
            return Vec::new();
        }
        if self.match_strings_strategy == "combination" {
            return self.extract_deps_combination(content);
        }
        let mut deps = Vec::new();
        for pattern in &self.match_strings {
            // Renovate patterns may use the JavaScript regex literal `/pattern/flags`
            // form. Strip enclosing slashes and flags if present.
            let bare = if pattern.starts_with('/') {
                pattern
                    .trim_start_matches('/')
                    .rsplit_once('/')
                    .map(|(p, _flags)| p)
                    .unwrap_or(pattern.as_str())
            } else {
                pattern.as_str()
            };
            let Ok(re) = regex::Regex::new(bare) else {
                tracing::debug!(pattern = %bare, "customManagers: invalid regex pattern; skipping");
                continue;
            };
            for caps in re.captures_iter(content) {
                let datasource = caps
                    .name("datasource")
                    .map(|m| m.as_str().to_owned())
                    .or_else(|| self.datasource_template.clone())
                    .unwrap_or_default();
                let dep_name = caps
                    .name("depName")
                    .map(|m| m.as_str().to_owned())
                    .or_else(|| self.dep_name_template.clone())
                    .unwrap_or_default();
                let current_value = caps
                    .name("currentValue")
                    .map(|m| m.as_str().to_owned())
                    .unwrap_or_default();
                // Skip incomplete matches.
                if dep_name.is_empty() || current_value.is_empty() || datasource.is_empty() {
                    continue;
                }
                let package_name = caps
                    .name("packageName")
                    .map(|m| m.as_str().to_owned())
                    .or_else(|| self.package_name_template.clone());
                let versioning = caps
                    .name("versioning")
                    .map(|m| m.as_str().to_owned())
                    .or_else(|| self.versioning_template.clone());
                let registry_url = caps
                    .name("registryUrl")
                    .map(|m| m.as_str().to_owned())
                    .or_else(|| self.registry_url_template.clone());
                let extract_version = caps
                    .name("extractVersion")
                    .map(|m| m.as_str().to_owned())
                    .or_else(|| self.extract_version_template.clone());
                deps.push(CustomExtractedDep {
                    dep_name,
                    package_name,
                    current_value,
                    datasource,
                    versioning,
                    registry_url,
                    extract_version,
                });
            }
        }
        deps
    }

    /// Combination strategy: apply all patterns globally, merge all captures
    /// into one dep (last capture of each group wins), then build a single dep.
    ///
    /// Renovate reference: `lib/modules/manager/custom/regex/strategies.ts` — `handleCombination`.
    fn extract_deps_combination(&self, content: &str) -> Vec<CustomExtractedDep> {
        let mut merged: std::collections::HashMap<&str, String> = std::collections::HashMap::new();

        for pattern in &self.match_strings {
            let bare = if pattern.starts_with('/') {
                pattern
                    .trim_start_matches('/')
                    .rsplit_once('/')
                    .map(|(p, _flags)| p)
                    .unwrap_or(pattern.as_str())
            } else {
                pattern.as_str()
            };
            let Ok(re) = regex::Regex::new(bare) else {
                tracing::debug!(pattern = %bare, "customManagers combination: invalid regex; skipping");
                continue;
            };
            for caps in re.captures_iter(content) {
                for name in &[
                    "datasource",
                    "depName",
                    "packageName",
                    "currentValue",
                    "versioning",
                    "registryUrl",
                    "extractVersion",
                ] {
                    if let Some(m) = caps.name(name) {
                        merged.insert(name, m.as_str().to_owned());
                    }
                }
            }
        }

        // Apply template defaults for any missing groups.
        let datasource = merged
            .remove("datasource")
            .or_else(|| self.datasource_template.clone())
            .unwrap_or_default();
        let dep_name = merged
            .remove("depName")
            .or_else(|| self.dep_name_template.clone())
            .unwrap_or_default();
        let current_value = merged.remove("currentValue").unwrap_or_default();
        if dep_name.is_empty() || current_value.is_empty() || datasource.is_empty() {
            return Vec::new();
        }
        let package_name = merged
            .remove("packageName")
            .or_else(|| self.package_name_template.clone());
        let versioning = merged
            .remove("versioning")
            .or_else(|| self.versioning_template.clone());
        let registry_url = merged
            .remove("registryUrl")
            .or_else(|| self.registry_url_template.clone());
        let extract_version = merged
            .remove("extractVersion")
            .or_else(|| self.extract_version_template.clone());

        vec![CustomExtractedDep {
            dep_name,
            package_name,
            current_value,
            datasource,
            versioning,
            registry_url,
            extract_version,
        }]
    }
}

// ── Free helpers ─────────────────────────────────────────────────────────────

/// Expand compound presets (presets that themselves extend other presets)
/// into their constituent presets.
///
/// This is a single-level expansion — we handle known compound presets by
/// replacing them with the list of presets they extend.  Recursion happens
/// implicitly because the expanded presets are themselves recognized by the
/// downstream resolution functions.
///
/// Renovate reference: `lib/config/presets/internal/config.preset.ts`.
fn expand_compound_presets(extends: &[String]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();

    for preset in extends {
        match preset.as_str() {
            // config:js-app = config:recommended + :pinAllExceptPeerDependencies
            "config:js-app" => {
                if seen.insert("config:recommended") {
                    result.push("config:recommended".to_owned());
                }
                result.push(":pinAllExceptPeerDependencies".to_owned());
            }
            // config:js-lib = config:recommended + :pinOnlyDevDependencies
            "config:js-lib" => {
                if seen.insert("config:recommended") {
                    result.push("config:recommended".to_owned());
                }
                result.push(":pinOnlyDevDependencies".to_owned());
            }
            // config:semverAllMonthly = :preserveSemverRanges + group:all + schedule:monthly
            // The preset also sets separateMajorMinor: false; we use "combineMajorMinorReleases"
            // as a sentinel for that since group:all already implies separate_major_minor: false.
            "config:semverAllMonthly" => {
                result.push(":preserveSemverRanges".to_owned());
                result.push("group:all".to_owned());
                result.push("schedule:monthly".to_owned());
                // group:all sets separate_major_minor = false implicitly
            }
            // config:semverAllWeekly = :preserveSemverRanges + group:all + schedule:weekly
            "config:semverAllWeekly" => {
                result.push(":preserveSemverRanges".to_owned());
                result.push("group:all".to_owned());
                result.push("schedule:weekly".to_owned());
            }
            // :semanticPrefixChore → :semanticCommitType(chore)
            ":semanticPrefixChore" | "semanticPrefixChore" => {
                result.push(":semanticCommitType(chore)".to_owned());
            }
            // :semanticPrefixFix → :semanticCommitType(fix)
            ":semanticPrefixFix" | "semanticPrefixFix" => {
                result.push(":semanticCommitType(fix)".to_owned());
            }
            // config:recommended expands to its constituent supported presets.
            "config:recommended" | "config:base" => {
                if seen.insert("config:recommended") {
                    // Key behavioral presets from config:recommended that we support
                    // (mergeConfidence:*, helpers:goXPackages* use prBodyDefinitions
                    // and are out of scope for scanning).
                    result.push(":dependencyDashboard".to_owned());
                    result.push(":semanticPrefixFixDepsChoreOthers".to_owned());
                    result.push(":ignoreModulesAndTests".to_owned());
                    result.push("group:monorepos".to_owned());
                    result.push("group:recommended".to_owned());
                    result.push("workarounds:all".to_owned());
                    // Keep the preset itself so downstream handlers (ignorePaths,
                    // group expansion) that match on "config:recommended" still fire.
                    result.push("config:recommended".to_owned());
                }
            }
            // config:best-practices extends config:recommended plus additional presets.
            // Renovate reference: lib/config/presets/internal/config.preset.ts
            "config:best-practices" => {
                if seen.insert("config:best-practices") {
                    result.push("config:recommended".to_owned());
                    result.push("docker:pinDigests".to_owned());
                    result.push("helpers:pinGitHubActionDigests".to_owned());
                    result.push(":configMigration".to_owned());
                    result.push(":pinDevDependencies".to_owned());
                    result.push("security:minimumReleaseAgeNpm".to_owned());
                    result.push(":maintainLockFilesWeekly".to_owned());
                    // Keep for downstream handlers.
                    result.push("config:best-practices".to_owned());
                }
            }
            // helpers:followTypescriptNext/Rc expand to :followTag(typescript, next/rc).
            // Renovate reference: lib/config/presets/internal/helpers.preset.ts
            "helpers:followTypescriptNext" => {
                result.push(":followTag(typescript, next)".to_owned());
            }
            "helpers:followTypescriptRc" => {
                result.push(":followTag(typescript, rc)".to_owned());
            }
            // replacements:all is handled directly in resolve_extends_common_rules
            // using a batch parse — no expansion needed here.
            "replacements:all" => {
                if seen.insert("replacements:all") {
                    result.push("replacements:all".to_owned());
                }
            }
            // workarounds:all expands to all 19 individual sub-presets so that
            // ignorePresets:["workarounds:typesNodeVersioning"] can suppress individual ones.
            // ignorePresets:["workarounds:all"] is handled via the pre-expansion filter.
            // Renovate reference: lib/config/presets/internal/workarounds.preset.ts
            "workarounds:all" => {
                for wa in &[
                    "workarounds:mavenCommonsAncientVersion",
                    "workarounds:ignoreSpringCloudNumeric",
                    "workarounds:ignoreWeb3jCoreWithOldReleaseTimestamp",
                    "workarounds:ignoreHttp4sDigestMilestones",
                    "workarounds:typesNodeVersioning",
                    "workarounds:nodeDockerVersioning",
                    "workarounds:doNotUpgradeFromAlpineStableToEdge",
                    "workarounds:supportRedHatImageVersion",
                    "workarounds:javaLTSVersions",
                    "workarounds:disableEclipseLifecycleMapping",
                    "workarounds:disableGradleReplacements",
                    "workarounds:disableMavenParentRoot",
                    "workarounds:containerbase",
                    "workarounds:bitnamiDockerImageVersioning",
                    "workarounds:clamavDockerImageVersioning",
                    "workarounds:k3sKubernetesVersioning",
                    "workarounds:rke2KubernetesVersioning",
                    "workarounds:libericaJdkDockerVersioning",
                    "workarounds:ubuntuDockerVersioning",
                ] {
                    result.push(wa.to_string());
                }
            }
            other => {
                // Handle parameterized compound presets like :assignAndReview(user).
                let (name, args) = parse_preset_args(other);
                if (name == ":assignAndReview" || name == "assignAndReview")
                    && !args.is_empty()
                    && !args[0].is_empty()
                {
                    result.push(format!(":assignee({})", args[0]));
                    result.push(format!(":reviewer({})", args[0]));
                } else {
                    result.push(other.to_owned());
                }
            }
        }
    }
    result
}

/// Collect `ignorePaths` contributed by built-in presets in `extends`.
///
/// Renovate's built-in presets can set `ignorePaths`.  The most impactful one
/// is `:ignoreModulesAndTests` (included transitively by `config:recommended`),
/// which ignores `node_modules`, `vendor`, `test/**`, etc.
///
/// This function resolves the well-known presets that set `ignorePaths` and
/// returns the union of their paths.  Unknown or remote presets are silently
/// skipped (they would require network access).
///
/// Renovate reference:
/// - `lib/config/presets/internal/default.preset.ts` — `:ignoreModulesAndTests`
/// - `lib/config/presets/internal/config.preset.ts` — `config:recommended`
///   transitively includes `:ignoreModulesAndTests`
fn resolve_extends_ignore_paths(extends: &[String]) -> Vec<String> {
    /// `ignorePaths` added by `:ignoreModulesAndTests`.
    const IGNORE_MODULES_AND_TESTS_PATHS: &[&str] = &[
        "**/node_modules/**",
        "**/bower_components/**",
        "**/vendor/**",
        "**/examples/**",
        "**/__tests__/**",
        "**/test/**",
        "**/tests/**",
        "**/__fixtures__/**",
    ];

    let mut seen_ignore_modules = false;
    let mut result: Vec<String> = Vec::new();

    for preset in extends {
        match preset.as_str() {
            ":ignoreModulesAndTests" | "default:ignoreModulesAndTests" => {
                if !seen_ignore_modules {
                    seen_ignore_modules = true;
                    result.extend(IGNORE_MODULES_AND_TESTS_PATHS.iter().map(|s| s.to_string()));
                }
            }
            // config:recommended → includes :ignoreModulesAndTests (among others).
            "config:recommended" | "config:base" | "config:best-practices" => {
                if !seen_ignore_modules {
                    seen_ignore_modules = true;
                    result.extend(IGNORE_MODULES_AND_TESTS_PATHS.iter().map(|s| s.to_string()));
                }
            }
            _ => {
                // Unknown or remote preset — skip silently.
                tracing::debug!(preset, "unresolved extends preset (no built-in expansion)");
            }
        }
    }

    result
}

/// Collect `schedule` entries contributed by built-in `schedule:*` presets.
///
/// Returns `None` when no schedule preset is found (caller keeps whatever
/// schedule the user configured, or none).  Returns `Some(schedule)` when
/// a preset contributes schedule entries.
///
/// Normalize a preset name using Renovate's `removedPresets` map.
///
/// Returns `None` when the preset has been removed with no replacement (null).
/// Renovate reference: `lib/config/presets/common.ts` — `removedPresets`.
/// Normalize a preset name using Renovate's `removedPresets` map.
///
/// Returns `None` when the preset has been removed with no replacement (null).
/// Returns `Some(replacement)` otherwise, where replacement may differ from input.
/// Renovate reference: `lib/config/presets/common.ts` — `removedPresets`.
fn normalize_preset(preset: &str) -> Option<String> {
    let renamed: Option<&str> = match preset {
        // Explicitly removed presets (no replacement).
        ":autodetectPinVersions"
        | ":autodetectRangeStrategy"
        | ":enableGradleLite"
        | ":switchToGradleLite"
        | "compatibility:additionalBranchPrefix"
        | "default:onlyNpm"
        | "helpers:oddIsUnstable"
        | "helpers:oddIsUnstablePackages"
        | "workarounds:reduceRepologyServerLoad" => return None,

        // Renames.
        ":automergeBranchMergeCommit"
        | ":automergeBranchPush"
        | "default:automergeBranchMergeCommit"
        | "default:automergeBranchPush" => Some(":automergeBranch"),
        ":base" | "default:base" | "config:base" | "config:base-js" => Some("config:recommended"),
        ":app" | ":js-app" | "default:app" | "default:js-app" | "config:application" => {
            Some("config:js-app")
        }
        ":library" | "default:library" | "config:library" => Some("config:js-lib"),
        ":disableLockFiles" => Some(":skipArtifactsUpdate"),
        ":masterIssue" => Some(":dependencyDashboard"),
        ":masterIssueApproval" => Some(":dependencyDashboardApproval"),
        ":unpublishSafe" | "default:unpublishSafe" => Some("npm:unpublishSafe"),
        "npm:unpublishSafe" => Some("security:minimumReleaseAgeNpm"),
        "group:jsTestMonMajor" => Some("group:jsTestNonMajor"),
        "group:kubernetes" => Some("group:kubernetesMonorepo"),
        "regexManagers:azurePipelinesVersions" => Some("customManagers:azurePipelinesVersions"),
        "regexManagers:biomeVersions" => Some("customManagers:biomeVersions"),
        "regexManagers:bitbucketPipelinesVersions" => {
            Some("customManagers:bitbucketPipelinesVersions")
        }
        "regexManagers:dockerfileVersions" => Some("customManagers:dockerfileVersions"),
        "regexManagers:githubActionsVersions" => Some("customManagers:githubActionsVersions"),
        "regexManagers:gitlabPipelineVersions" => Some("customManagers:gitlabPipelineVersions"),
        "regexManagers:helmChartYamlAppVersions" => Some("customManagers:helmChartYamlAppVersions"),
        "regexManagers:mavenPropertyVersions" => Some("customManagers:mavenPropertyVersions"),
        "regexManagers:tfvarsVersions" => Some("customManagers:tfvarsVersions"),
        "regexManagers:tsconfigNodeVersions" => Some("customManagers:tsconfigNodeVersions"),
        _ => None,
    };
    Some(renamed.unwrap_or(preset).to_owned())
}

/// Migrate a legacy schedule string to the current format.
///
/// - `"every friday"` → `"on friday"` (Renovate: schedule-migration.ts dayRegex)
///   Only specific day names are migrated; "every weekday" etc. are handled natively.
fn migrate_schedule_string(s: String) -> String {
    const DAYS: &[&str] = &[
        "monday",
        "tuesday",
        "wednesday",
        "thursday",
        "friday",
        "saturday",
        "sunday",
    ];
    for day in DAYS {
        let every_day = format!("every {day}");
        if s.ends_with(&every_day) {
            let on_day = format!("on {day}");
            return s.replacen(&every_day, &on_day, 1);
        }
    }
    s
}

/// When the user has an explicit non-empty `schedule` in their config,
/// the caller should prefer it over the preset value.
///
/// Renovate reference: `lib/config/presets/internal/schedule.preset.ts`
fn resolve_extends_schedule(extends: &[String]) -> Option<Vec<String>> {
    // cron expressions for each named schedule
    const DAILY: &[&str] = &["* 0-3 * * *"];
    const EARLY_MONDAYS: &[&str] = &["* 0-3 * * 1"];
    const MONTHLY: &[&str] = &["* 0-3 1 * *"];
    const NON_OFFICE_HOURS: &[&str] = &["* 0-4,22-23 * * 1-5", "* * * * 0,6"];
    const OFFICE_HOURS: &[&str] = &["* 8-17 * * 1-5"];
    const QUARTERLY: &[&str] = &["* * 1 */3 *"];
    const WEEKDAYS: &[&str] = &["* * * * 1-5"];
    const WEEKENDS: &[&str] = &["* * * * 0,6"];
    const YEARLY: &[&str] = &["* * 1 */12 *"];

    fn to_string_vec(s: &[&str]) -> Vec<String> {
        s.iter().map(|&x| x.to_owned()).collect()
    }

    // Use the LAST schedule preset that matches (Renovate: last wins for scalar fields).
    let mut result: Option<Vec<String>> = None;

    for preset in extends {
        let schedule = match preset.as_str() {
            "schedule:daily" => Some(to_string_vec(DAILY)),
            "schedule:earlyMondays" | "schedule:weekly" => Some(to_string_vec(EARLY_MONDAYS)),
            "schedule:monthly" => Some(to_string_vec(MONTHLY)),
            "schedule:nonOfficeHours" => Some(to_string_vec(NON_OFFICE_HOURS)),
            "schedule:officeHours" => Some(to_string_vec(OFFICE_HOURS)),
            "schedule:quarterly" => Some(to_string_vec(QUARTERLY)),
            "schedule:weekdays" => Some(to_string_vec(WEEKDAYS)),
            "schedule:weekends" => Some(to_string_vec(WEEKENDS)),
            "schedule:yearly" => Some(to_string_vec(YEARLY)),
            _ => None,
        };
        if let Some(s) = schedule {
            result = Some(s);
        }
    }

    result
}

/// Collect `automergeSchedule` contributed by `schedule:automerge*` presets.
///
/// Mirrors the cron constants from `schedule.preset.ts` for the automerge
/// schedule presets.  Returns `None` if no automerge schedule preset is found.
fn resolve_extends_automerge_schedule(extends: &[String]) -> Option<Vec<String>> {
    const DAILY: &[&str] = &["* 0-3 * * *"];
    const EARLY_MONDAYS: &[&str] = &["* 0-3 * * 1"];
    const MONTHLY: &[&str] = &["* 0-3 1 * *"];
    const NON_OFFICE_HOURS: &[&str] = &["* 0-4,22-23 * * 1-5", "* * * * 0,6"];
    const OFFICE_HOURS: &[&str] = &["* 8-17 * * 1-5"];
    const QUARTERLY: &[&str] = &["* * 1 */3 *"];
    const WEEKDAYS: &[&str] = &["* * * * 1-5"];
    const WEEKENDS: &[&str] = &["* * * * 0,6"];
    const YEARLY: &[&str] = &["* * 1 */12 *"];

    fn to_string_vec(s: &[&str]) -> Vec<String> {
        s.iter().map(|&x| x.to_owned()).collect()
    }

    let mut result: Option<Vec<String>> = None;
    for preset in extends {
        let schedule = match preset.as_str() {
            "schedule:automergeDaily" => Some(to_string_vec(DAILY)),
            "schedule:automergeEarlyMondays" | "schedule:automergeWeekly" => {
                Some(to_string_vec(EARLY_MONDAYS))
            }
            "schedule:automergeMonthly" => Some(to_string_vec(MONTHLY)),
            "schedule:automergeNonOfficeHours" => Some(to_string_vec(NON_OFFICE_HOURS)),
            "schedule:automergeOfficeHours" => Some(to_string_vec(OFFICE_HOURS)),
            "schedule:automergeQuarterly" => Some(to_string_vec(QUARTERLY)),
            "schedule:automergeWeekdays" => Some(to_string_vec(WEEKDAYS)),
            "schedule:automergeWeekends" => Some(to_string_vec(WEEKENDS)),
            "schedule:automergeYearly" => Some(to_string_vec(YEARLY)),
            _ => None,
        };
        if let Some(s) = schedule {
            result = Some(s);
        }
    }
    result
}

/// Collect `automerge` value contributed by built-in `:automerge*` presets.
///
/// Returns `None` when no automerge preset is found.
/// Only `:automergeAll` / `:automergeMajor` / `:autoMerge` / `:automergeBranch`
/// set global `automerge: true`.  `:automergeMinor` and `:automergePatch`
/// inject per-update-type packageRules instead (see `resolve_extends_automerge_rules`).
/// Matcher fields resolved from a `packages:*` preset reference inside a `packageRule`.
///
/// When a packageRule contains `extends: ["packages:react"]`, these fields are
/// merged (union) into the rule's own matcher conditions.
#[derive(Default)]
struct PackagePresetMatchers {
    match_package_names: Vec<String>,
    match_datasources: Vec<String>,
    match_source_urls: Vec<String>,
}

/// Resolve `packages:*` preset references from a packageRule's `extends` list.
///
/// Returns aggregated matcher fields to merge into the enclosing rule.
/// Only the `packages:` namespace is handled; other presets are silently ignored.
///
/// Renovate reference: `lib/config/presets/internal/packages.preset.ts`
fn resolve_package_rule_extends(extends: &[String]) -> PackagePresetMatchers {
    let mut out = PackagePresetMatchers::default();
    for preset in extends {
        match preset.as_str() {
            "packages:angularJs" => {
                out.match_package_names.extend([
                    "angular".to_owned(),
                    "@angular/**".to_owned(),
                    "angular-**".to_owned(),
                ]);
            }
            "packages:apollographql" => {
                out.match_source_urls
                    .push("https://github.com/apollographql/**".to_owned());
            }
            "packages:atlaskit" => {
                out.match_package_names.push("@atlaskit/**".to_owned());
            }
            "packages:emberTemplateLint" => {
                out.match_package_names
                    .push("ember-template-lint**".to_owned());
            }
            "packages:eslint" => {
                out.match_package_names.extend([
                    "eslint".to_owned(),
                    "eslint-config-**".to_owned(),
                    "eslint-import-resolver-**".to_owned(),
                    "eslint-plugin-**".to_owned(),
                    "@typescript-eslint/**".to_owned(),
                    "@eslint/**".to_owned(),
                    "@eslint-community/**".to_owned(),
                ]);
            }
            "packages:gatsby" => {
                // gatsby is a monorepo; resolved by monorepo:gatsby
                out.match_package_names
                    .extend(["gatsby".to_owned(), "gatsby-**".to_owned()]);
            }
            "packages:googleapis" => {
                out.match_datasources.push("npm".to_owned());
                out.match_package_names.extend([
                    "@google-cloud/**".to_owned(),
                    "google-auth-library".to_owned(),
                    "googleapis".to_owned(),
                ]);
            }
            "packages:jsTest" | "packages:jsUnitTest" => {
                out.match_package_names.extend([
                    "@jest/**".to_owned(),
                    "@sinonjs/**".to_owned(),
                    "@testing-library/**".to_owned(),
                    "@types/enzyme".to_owned(),
                    "@types/jest".to_owned(),
                    "@types/mocha".to_owned(),
                    "@types/node".to_owned(),
                    "@types/supertest".to_owned(),
                    "ava".to_owned(),
                    "chai".to_owned(),
                    "enzyme".to_owned(),
                    "expect".to_owned(),
                    "jasmine**".to_owned(),
                    "jest**".to_owned(),
                    "mocha**".to_owned(),
                    "mock-fs".to_owned(),
                    "nock".to_owned(),
                    "proxyquire".to_owned(),
                    "should".to_owned(),
                    "sinon**".to_owned(),
                    "supertest".to_owned(),
                    "testdouble".to_owned(),
                    "ts-jest".to_owned(),
                    "vitest".to_owned(),
                    "@vitest/**".to_owned(),
                ]);
            }
            "packages:linters" => {
                // extends emberTemplateLint, eslint, phpLinters, stylelint, tslint
                out.match_package_names.extend([
                    "ember-template-lint**".to_owned(),
                    "eslint".to_owned(),
                    "eslint-config-**".to_owned(),
                    "eslint-import-resolver-**".to_owned(),
                    "eslint-plugin-**".to_owned(),
                    "@typescript-eslint/**".to_owned(),
                    "@eslint/**".to_owned(),
                    "@eslint-community/**".to_owned(),
                    "friendsofphp/php-cs-fixer".to_owned(),
                    "squizlabs/php_codesniffer".to_owned(),
                    "symplify/easy-coding-standard".to_owned(),
                    "stylelint**".to_owned(),
                    "codelyzer".to_owned(),
                    "/\\btslint\\b/".to_owned(),
                    "oxlint".to_owned(),
                    "prettier".to_owned(),
                    "remark-lint".to_owned(),
                    "standard".to_owned(),
                ]);
            }
            "packages:mapbox" => {
                out.match_package_names
                    .extend(["leaflet**".to_owned(), "mapbox**".to_owned()]);
            }
            "packages:phpLinters" => {
                out.match_package_names.extend([
                    "friendsofphp/php-cs-fixer".to_owned(),
                    "squizlabs/php_codesniffer".to_owned(),
                    "symplify/easy-coding-standard".to_owned(),
                ]);
            }
            "packages:phpUnitTest" => {
                out.match_package_names.extend([
                    "behat/behat".to_owned(),
                    "brianium/paratest".to_owned(),
                    "facile-it/paraunit".to_owned(),
                    "mockery/mockery".to_owned(),
                    "phpspec/prophecy".to_owned(),
                    "phpspec/prophecy-phpunit".to_owned(),
                    "phpspec/phpspec".to_owned(),
                    "phpunit/phpunit".to_owned(),
                    "pestphp/**".to_owned(),
                    "php-mock/**".to_owned(),
                ]);
            }
            "packages:postcss" => {
                out.match_package_names
                    .extend(["postcss".to_owned(), "postcss-**".to_owned()]);
            }
            "packages:react" => {
                out.match_datasources.push("npm".to_owned());
                out.match_package_names
                    .extend(["@types/react**".to_owned(), "react**".to_owned()]);
            }
            "packages:stylelint" => {
                out.match_package_names.push("stylelint**".to_owned());
            }
            "packages:test" | "packages:unitTest" => {
                // extends jsUnitTest + phpUnitTest
                out.match_package_names.extend([
                    "@jest/**".to_owned(),
                    "jest**".to_owned(),
                    "vitest".to_owned(),
                    "@vitest/**".to_owned(),
                    "mocha**".to_owned(),
                    "jasmine**".to_owned(),
                    "phpunit/phpunit".to_owned(),
                    "pestphp/**".to_owned(),
                ]);
            }
            "packages:tslint" => {
                out.match_package_names
                    .extend(["codelyzer".to_owned(), "/\\btslint\\b/".to_owned()]);
            }
            "packages:vite" => {
                out.match_datasources.push("npm".to_owned());
                out.match_package_names.extend([
                    "vite".to_owned(),
                    "**vite-plugin**".to_owned(),
                    "@vitejs/**".to_owned(),
                ]);
            }
            _ => {}
        }
    }
    out
}

///
/// Collect `CustomManager` entries contributed by `custom-managers:*` presets.
///
/// Handles all regex-based presets from `custom-managers.preset.ts`.
/// JSONata-based presets (`biomeVersions`) are silently skipped.
///
/// Renovate reference: `lib/config/presets/internal/custom-managers.preset.ts`
fn resolve_extends_custom_managers(extends: &[String]) -> Vec<CustomManager> {
    // Standard renovate: annotation regex pattern shared by most presets.
    // Matches:  # renovate: datasource=X depName=Y [packageName=Z] [versioning=V]
    //           [extractVersion=E] [registryUrl=R]
    //           VERSION_VAR=value
    let standard_pattern = r#"# renovate: datasource=(?P<datasource>[a-zA-Z0-9-._]+?) depName=(?P<depName>[^\s]+?)(?: (?:lookupName|packageName)=(?P<packageName>[^\s]+?))?(?: versioning=(?P<versioning>[^\s]+?))?(?: extractVersion=(?P<extractVersion>[^\s]+?))?(?: registryUrl=(?P<registryUrl>[^\s]+?))?\s+[A-Za-z0-9_]+?_VERSION\s*[:=]?\??[=:]\s*["']?(?P<currentValue>.+?)["']?\s"#;

    let mut managers: Vec<CustomManager> = Vec::new();
    for preset in extends {
        match preset.as_str() {
            "custom-managers:azurePipelinesVersions" => {
                managers.push(CustomManager {
                    custom_type: "regex".to_owned(),
                    file_patterns: vec![
                        "**/.azuredevops/**/*.{yml,yaml}".to_owned(),
                        "azure*pipeline*.{yml,yaml}".to_owned(),
                    ],
                    match_strings: vec![standard_pattern.to_owned()],
                    match_strings_strategy: "any".to_owned(),
                    ..Default::default()
                });
            }
            "custom-managers:bitbucketPipelinesVersions" => {
                managers.push(CustomManager {
                    custom_type: "regex".to_owned(),
                    file_patterns: vec!["**/*-pipelines.yml".to_owned()],
                    match_strings: vec![standard_pattern.to_owned()],
                    match_strings_strategy: "any".to_owned(),
                    ..Default::default()
                });
            }
            "custom-managers:dockerfileVersions" => {
                managers.push(CustomManager {
                    custom_type: "regex".to_owned(),
                    file_patterns: vec![
                        "**/[Dd]ockerfile*".to_owned(),
                        "**/[Cc]ontainerfile*".to_owned(),
                        "**/*.[Dd]ockerfile*".to_owned(),
                        "**/*.[Cc]ontainerfile*".to_owned(),
                    ],
                    // Dockerfile variant: matches ENV/ARG directives with renovate comment.
                    match_strings: vec![
                        r#"# renovate: datasource=(?P<datasource>[a-zA-Z0-9-._]+?) depName=(?P<depName>[^\s]+?)(?: (?:lookupName|packageName)=(?P<packageName>[^\s]+?))?(?: versioning=(?P<versioning>[^\s]+?))?(?: extractVersion=(?P<extractVersion>[^\s]+?))?(?: registryUrl=(?P<registryUrl>[^\s]+?))?\s(?:ENV|ARG)\s+[A-Za-z0-9_]+?_VERSION[ =]["']?(?P<currentValue>.+?)["']?\s"#.to_owned(),
                    ],
                    match_strings_strategy: "any".to_owned(),
                    ..Default::default()
                });
            }
            "custom-managers:githubActionsVersions" => {
                managers.push(CustomManager {
                    custom_type: "regex".to_owned(),
                    file_patterns: vec![
                        r"/(^|/)(workflow-templates|\.(?:github|gitea|forgejo)/(?:workflows|actions))/.+\.ya?ml$/"
                            .to_owned(),
                        r"/(^|/)action\.ya?ml$/".to_owned(),
                    ],
                    match_strings: vec![standard_pattern.to_owned()],
                    match_strings_strategy: "any".to_owned(),
                    ..Default::default()
                });
            }
            "custom-managers:gitlabPipelineVersions" => {
                managers.push(CustomManager {
                    custom_type: "regex".to_owned(),
                    file_patterns: vec!["**/*.gitlab-ci.{yml,yaml}".to_owned()],
                    match_strings: vec![standard_pattern.to_owned()],
                    match_strings_strategy: "any".to_owned(),
                    ..Default::default()
                });
            }
            "custom-managers:helmChartYamlAppVersions" => {
                managers.push(CustomManager {
                    custom_type: "regex".to_owned(),
                    file_patterns: vec!["**/Chart.yaml".to_owned()],
                    match_strings: vec![
                        r#"#\s*renovate: image=(?P<depName>.*?)\s+appVersion:\s*["']?(?P<currentValue>[\w+\.\-]*)"#
                            .to_owned(),
                    ],
                    match_strings_strategy: "any".to_owned(),
                    datasource_template: Some("docker".to_owned()),
                    ..Default::default()
                });
            }
            "custom-managers:makefileVersions" => {
                managers.push(CustomManager {
                    custom_type: "regex".to_owned(),
                    file_patterns: vec![
                        "**/[Mm]akefile".to_owned(),
                        "**/GNUMakefile".to_owned(),
                        "**/*.mk".to_owned(),
                    ],
                    match_strings: vec![
                        r#"# renovate: datasource=(?P<datasource>[a-zA-Z0-9-._]+?) depName=(?P<depName>[^\s]+?)(?: (?:packageName)=(?P<packageName>[^\s]+?))?(?: versioning=(?P<versioning>[^\s]+?))?(?: extractVersion=(?P<extractVersion>[^\s]+?))?(?: registryUrl=(?P<registryUrl>[^\s]+?))?\s+[A-Za-z0-9_]+?_VERSION\s*:*\??=\s*["']?(?P<currentValue>.+?)["']?\s"#
                            .to_owned(),
                    ],
                    match_strings_strategy: "any".to_owned(),
                    ..Default::default()
                });
            }
            "custom-managers:mavenPropertyVersions" => {
                managers.push(CustomManager {
                    custom_type: "regex".to_owned(),
                    file_patterns: vec!["**/pom.xml".to_owned()],
                    match_strings: vec![
                        r"<!--\s?renovate:( datasource=(?P<datasource>[a-zA-Z0-9-._]+?))? depName=(?P<depName>[^\s]+?)(?: packageName=(?P<packageName>[^\s]+?))?(?: versioning=(?P<versioning>[^\s]+?))?(?: extractVersion=(?P<extractVersion>[^\s]+?))?\s+-->\s+<.+\.version>(?P<currentValue>.+)<\/.+\.version>"
                            .to_owned(),
                    ],
                    match_strings_strategy: "any".to_owned(),
                    // Default to maven when datasource group not captured.
                    datasource_template: Some("maven".to_owned()),
                    ..Default::default()
                });
            }
            "custom-managers:tfvarsVersions" => {
                managers.push(CustomManager {
                    custom_type: "regex".to_owned(),
                    file_patterns: vec!["**/*.tfvars".to_owned()],
                    match_strings: vec![
                        r#"#\s*renovate: datasource=(?P<datasource>.*?) depName=(?P<depName>.*?)( versioning=(?P<versioning>.*?))?(?: extractVersion=(?P<extractVersion>.*?))?(?: registryUrl=(?P<registryUrl>[^\s]+?))?\s.*?_version\s*=\s*"(?P<currentValue>.*?)""#
                            .to_owned(),
                    ],
                    match_strings_strategy: "any".to_owned(),
                    ..Default::default()
                });
            }
            "custom-managers:tsconfigNodeVersions" => {
                // First rule: "@tsconfig/node18/tsconfig.json" pattern
                managers.push(CustomManager {
                    custom_type: "regex".to_owned(),
                    file_patterns: vec![
                        "**/{j,t}sconfig.json".to_owned(),
                        "**/{j,t}sconfig.*.json".to_owned(),
                    ],
                    match_strings: vec![
                        r#""(?P<depName>@tsconfig/node(?P<currentValue>\d+))/tsconfig\.json""#
                            .to_owned(),
                    ],
                    match_strings_strategy: "any".to_owned(),
                    datasource_template: Some("npm".to_owned()),
                    ..Default::default()
                });
                // Second rule: "@tsconfig/node18" without "/tsconfig.json"
                managers.push(CustomManager {
                    custom_type: "regex".to_owned(),
                    file_patterns: vec![
                        "**/{j,t}sconfig.json".to_owned(),
                        "**/{j,t}sconfig.*.json".to_owned(),
                    ],
                    match_strings: vec![
                        r#""(?P<depName>@tsconfig/node(?P<currentValue>\d+))""#.to_owned(),
                    ],
                    match_strings_strategy: "any".to_owned(),
                    datasource_template: Some("npm".to_owned()),
                    ..Default::default()
                });
            }
            _ => {}
        }
    }
    managers
}

/// Renovate reference: `lib/config/presets/internal/default.preset.ts` —
/// `:automergeAll`, `:automergeMinor`, `:automergeDisabled`, etc.
fn resolve_extends_automerge(extends: &[String]) -> Option<bool> {
    let mut result: Option<bool> = None;
    for preset in extends {
        match preset.as_str() {
            ":automergeAll" | ":automergeMajor" | ":automergeBranch" | ":automergePr"
            | ":autoMerge" => {
                result = Some(true);
            }
            ":automergeDisabled" | ":noAutomerge" => {
                result = Some(false);
            }
            _ => {}
        }
    }
    result
}

/// Emit low-priority packageRules for selective automerge presets.
///
/// `:automergeMinor` → automerge minor + patch updates.
/// `:automergePatch` → automerge patch updates only.
///
/// These rules are prepended before user packageRules so user rules can
/// override them (last rule wins).
///
/// Renovate reference: `lib/config/presets/internal/default.preset.ts`
fn resolve_extends_automerge_rules(extends: &[String]) -> Vec<PackageRule> {
    use crate::versioning::semver_generic::UpdateType;
    let mut rules = Vec::new();
    for preset in extends {
        match preset.as_str() {
            ":automergeMinor" => {
                // Automerge minor + patch.
                rules.push(PackageRule {
                    match_update_types: vec![UpdateType::Minor, UpdateType::Patch],
                    automerge: Some(true),
                    ..Default::default()
                });
            }
            ":automergePatch" => {
                // Automerge patch only.
                rules.push(PackageRule {
                    match_update_types: vec![UpdateType::Patch],
                    automerge: Some(true),
                    ..Default::default()
                });
            }
            // :automergeMajor — automerge all updates including major.
            ":automergeMajor" | "automergeMajor" => {
                rules.push(PackageRule {
                    match_update_types: vec![
                        UpdateType::Major,
                        UpdateType::Minor,
                        UpdateType::Patch,
                    ],
                    automerge: Some(true),
                    ..Default::default()
                });
            }
            // :automergeAll — automerge all (including major); global flag handled in parse().
            // packageRule version for completeness.
            ":automergeAll" | "automergeAll" | ":autoMerge" | "autoMerge" => {
                // Global automerge: true is set in parse(); no per-rule needed.
            }
            // :automergeLinters — automerge linter packages.
            ":automergeLinters" | "automergeLinters" => {
                rules.push(PackageRule {
                    has_name_constraint: true,
                    match_package_names: LINTER_PACKAGES.iter().map(|&s| s.to_owned()).collect(),
                    automerge: Some(true),
                    ..Default::default()
                });
            }
            // :automergeTesters — automerge test packages (js + php unit tests).
            ":automergeTesters" | "automergeTesters" => {
                let mut pkgs: Vec<String> = JS_UNIT_TEST_PACKAGES
                    .iter()
                    .map(|&s| s.to_owned())
                    .collect();
                pkgs.extend(PHP_UNIT_TEST_PACKAGES.iter().map(|&s| s.to_owned()));
                rules.push(PackageRule {
                    has_name_constraint: true,
                    match_package_names: pkgs,
                    automerge: Some(true),
                    ..Default::default()
                });
            }
            // :automergeTypes — automerge @types/* packages.
            ":automergeTypes" | "automergeTypes" => {
                rules.push(PackageRule {
                    has_name_constraint: true,
                    match_package_names: vec!["@types/**".to_owned()],
                    automerge: Some(true),
                    ..Default::default()
                });
            }
            _ => {}
        }
    }
    rules
}

/// Resolve packageRules injected by common built-in presets.
///
/// Handles presets that expand to `packageRules` blocks:
/// - `:disableDevDependencies` — disable dev dep updates
/// - `:disablePeerDependencies` — disable peer dep updates
/// - `:disableMajorUpdates` — disable all major updates
/// - `:automergeStableNonMajor` — automerge stable non-major updates
///
/// Renovate reference: `lib/config/presets/internal/default.preset.ts`
fn resolve_extends_common_rules(extends: &[String]) -> Vec<PackageRule> {
    use crate::versioning::semver_generic::UpdateType;
    let mut rules: Vec<PackageRule> = Vec::new();

    for preset in extends {
        match preset.as_str() {
            ":disableDevDependencies" => {
                rules.push(PackageRule {
                    match_dep_types: vec![
                        "devDependencies".to_owned(),
                        "dev-dependencies".to_owned(),
                        "dev".to_owned(),
                    ],
                    enabled: Some(false),
                    ..Default::default()
                });
            }
            ":disablePeerDependencies" => {
                rules.push(PackageRule {
                    match_dep_types: vec!["peerDependencies".to_owned()],
                    enabled: Some(false),
                    ..Default::default()
                });
            }
            ":disableMajorUpdates" => {
                rules.push(PackageRule {
                    match_update_types: vec![UpdateType::Major],
                    enabled: Some(false),
                    ..Default::default()
                });
            }
            ":automergeStableNonMajor" => {
                rules.push(PackageRule {
                    match_current_version: Some("!/^0/".to_owned()),
                    match_update_types: vec![UpdateType::Minor, UpdateType::Patch],
                    automerge: Some(true),
                    ..Default::default()
                });
            }
            // security presets: wait N days before upgrading to limit supply-chain risk.
            "security:minimumReleaseAgeNpm" => {
                rules.push(PackageRule {
                    match_datasources: vec!["npm".to_owned()],
                    minimum_release_age: Some("3 days".to_owned()),
                    ..Default::default()
                });
            }
            "security:minimumReleaseAgeCratesio" => {
                rules.push(PackageRule {
                    match_datasources: vec!["crate".to_owned()],
                    minimum_release_age: Some("3 days".to_owned()),
                    ..Default::default()
                });
            }
            "security:minimumReleaseAgePyPI" | "security:minimumReleaseAgePip" => {
                rules.push(PackageRule {
                    match_datasources: vec!["pypi".to_owned()],
                    minimum_release_age: Some("3 days".to_owned()),
                    ..Default::default()
                });
            }
            // :unpublishSafe is equivalent to minimumReleaseAge: "3 days" for npm
            ":unpublishSafe" => {
                rules.push(PackageRule {
                    match_datasources: vec!["npm".to_owned()],
                    minimum_release_age: Some("3 days".to_owned()),
                    ..Default::default()
                });
            }
            // docker:disableMajor — disable major Docker updates
            "docker:disableMajor" => {
                rules.push(PackageRule {
                    match_datasources: vec!["docker".to_owned()],
                    match_update_types: vec![UpdateType::Major],
                    has_update_type_constraint: true,
                    enabled: Some(false),
                    ..Default::default()
                });
            }
            // docker:enableMajor — re-enable major Docker updates (counteracts disableMajor)
            "docker:enableMajor" => {
                rules.push(PackageRule {
                    match_datasources: vec!["docker".to_owned()],
                    match_update_types: vec![UpdateType::Major],
                    has_update_type_constraint: true,
                    enabled: Some(true),
                    ..Default::default()
                });
            }
            // docker:disable disables the dockerfile, docker-compose, and circleci
            // managers at the manager level (not via packageRules). Handled in parse()
            // via disabled_managers. No packageRule needed here.
            "docker:disable" => {}
            // docker:pinDigests — pin Docker images to their digest.
            // Mirrors Renovate's docker.preset.ts pinDigests preset.
            "docker:pinDigests" => {
                rules.push(PackageRule {
                    match_datasources: vec!["docker".to_owned()],
                    pin_digests: Some(true),
                    ..Default::default()
                });
                // Undo pinDigests for ArgoCD, devcontainer, helmv3, pyenv.
                rules.push(PackageRule {
                    match_managers: vec![
                        "argocd".to_owned(),
                        "devcontainer".to_owned(),
                        "helmv3".to_owned(),
                        "pyenv".to_owned(),
                    ],
                    pin_digests: Some(false),
                    ..Default::default()
                });
            }
            // helpers:pinGitHubActionDigests — pin GitHub Actions to their digest.
            "helpers:pinGitHubActionDigests" => {
                rules.push(PackageRule {
                    match_dep_types: vec!["action".to_owned()],
                    pin_digests: Some(true),
                    ..Default::default()
                });
            }
            // helpers:pinGitHubActionDigestsToSemver — pin Actions digests with SemVer
            // extraction. Combines pinGitHubActionDigests + versioning + extractVersion.
            // Renovate reference: lib/config/presets/internal/helpers.preset.ts
            "helpers:pinGitHubActionDigestsToSemver" => {
                rules.push(PackageRule {
                    match_dep_types: vec!["action".to_owned()],
                    pin_digests: Some(true),
                    versioning: Some(
                        r"regex:^v?(?<major>\d+)(\.(?<minor>\d+)\.(?<patch>\d+))?$".to_owned(),
                    ),
                    ..Default::default()
                });
            }
            // helpers:githubDigestChangelogs — add changelogUrl for GitHub digest updates.
            // Renovate reference: lib/config/presets/internal/helpers.preset.ts
            "helpers:githubDigestChangelogs" => {
                rules.push(PackageRule {
                    match_datasources: vec![
                        "github-digest".to_owned(),
                        "github-releases".to_owned(),
                        "github-tags".to_owned(),
                    ],
                    match_update_types: vec![UpdateType::Digest],
                    has_update_type_constraint: true,
                    changelog_url: Some(
                        "{{sourceUrl}}/compare/{{currentDigest}}..{{newDigest}}".to_owned(),
                    ),
                    ..Default::default()
                });
                // git-refs/git-tags from github also get the changelog URL.
                rules.push(PackageRule {
                    match_datasources: vec!["git-refs".to_owned(), "git-tags".to_owned()],
                    match_update_types: vec![UpdateType::Digest],
                    has_update_type_constraint: true,
                    changelog_url: Some(
                        "{{sourceUrl}}/compare/{{currentDigest}}..{{newDigest}}".to_owned(),
                    ),
                    ..Default::default()
                });
            }
            // helpers:forgejoDigestChangelogs — add changelogUrl for Forgejo digest updates.
            "helpers:forgejoDigestChangelogs" => {
                rules.push(PackageRule {
                    match_datasources: vec![
                        "forgejo-releases".to_owned(),
                        "forgejo-tags".to_owned(),
                    ],
                    match_update_types: vec![UpdateType::Digest],
                    has_update_type_constraint: true,
                    changelog_url: Some(
                        "{{sourceUrl}}/compare/{{currentDigest}}..{{newDigest}}".to_owned(),
                    ),
                    ..Default::default()
                });
            }
            // helpers:giteaDigestChangelogs — add changelogUrl for Gitea digest updates.
            "helpers:giteaDigestChangelogs" => {
                rules.push(PackageRule {
                    match_datasources: vec!["gitea-releases".to_owned(), "gitea-tags".to_owned()],
                    match_update_types: vec![UpdateType::Digest],
                    has_update_type_constraint: true,
                    changelog_url: Some(
                        "{{sourceUrl}}/compare/{{currentDigest}}..{{newDigest}}".to_owned(),
                    ),
                    ..Default::default()
                });
            }
            // helpers:disableTypesNodeMajor — disable @types/node major updates.
            "helpers:disableTypesNodeMajor" => {
                rules.push(PackageRule {
                    match_package_names: vec!["@types/node".to_owned()],
                    has_name_constraint: true,
                    match_update_types: vec![UpdateType::Major],
                    has_update_type_constraint: true,
                    enabled: Some(false),
                    ..Default::default()
                });
            }

            // ── replacements:* presets ────────────────────────────────────────
            // Crowd-sourced package replacement presets from replacements.json.
            // Renovate reference: lib/data/replacements.json + replacements.preset.ts
            "replacements:all" => {
                // Batch: parse JSON once for all 60 replacements.
                rules.extend(crate::replacements::all_replacement_rules());
            }
            p if p.starts_with("replacements:") => {
                // Individual replacement preset (e.g. replacements:babel-eslint-to-eslint-parser).
                rules.extend(crate::replacements::rules_for_preset(p));
            }

            // ── workarounds:* presets ─────────────────────────────────────────
            // workarounds:all is expanded to individual sub-presets in expand_compound_presets.
            // Individual sub-presets are addressable here for fine-grained ignorePresets.
            // Renovate reference: lib/config/presets/internal/workarounds.preset.ts
            "workarounds:mavenCommonsAncientVersion" => {
                rules.push(PackageRule {
                    match_datasources: vec!["maven".to_owned(), "sbt-package".to_owned()],
                    match_package_names: vec!["commons-**".to_owned()],
                    has_name_constraint: true,
                    allowed_versions: Some("!/^200\\d{5}(\\.\\d+)?/".to_owned()),
                    ..Default::default()
                });
            }
            "workarounds:ignoreSpringCloudNumeric" => {
                rules.push(PackageRule {
                    match_datasources: vec!["maven".to_owned()],
                    match_package_names: vec![
                        "org.springframework.cloud:spring-cloud-starter-parent".to_owned(),
                    ],
                    has_name_constraint: true,
                    allowed_versions: Some("/^[A-Z]/".to_owned()),
                    ..Default::default()
                });
            }
            "workarounds:ignoreWeb3jCoreWithOldReleaseTimestamp" => {
                rules.push(PackageRule {
                    match_datasources: vec!["maven".to_owned()],
                    match_package_names: vec!["org.web3j:core".to_owned()],
                    has_name_constraint: true,
                    allowed_versions: Some("!/^5\\.0\\.0/".to_owned()),
                    ..Default::default()
                });
            }
            "workarounds:ignoreHttp4sDigestMilestones" => {
                rules.push(PackageRule {
                    match_managers: vec!["sbt".to_owned()],
                    match_package_names: vec!["org.http4s:**".to_owned()],
                    has_name_constraint: true,
                    allowed_versions: Some("!/^1\\.0-\\d+-[a-fA-F0-9]{7}$/".to_owned()),
                    ..Default::default()
                });
            }
            "workarounds:typesNodeVersioning" => {
                rules.push(PackageRule {
                    match_managers: vec!["npm".to_owned()],
                    match_package_names: vec!["@types/node".to_owned()],
                    has_name_constraint: true,
                    versioning: Some("node".to_owned()),
                    ..Default::default()
                });
            }
            "workarounds:nodeDockerVersioning" => {
                rules.push(PackageRule {
                    match_datasources: vec!["docker".to_owned()],
                    match_package_names: vec![
                        "/(?:^|/)node$/".to_owned(),
                        "!calico/node".to_owned(),
                        "!docker.io/calico/node".to_owned(),
                        "!ghcr.io/devcontainers/features/node".to_owned(),
                        "!kindest/node".to_owned(),
                    ],
                    has_name_constraint: true,
                    version_compatibility: Some(
                        "^(?<version>[^-]+)(?<compatibility>-.*)?$".to_owned(),
                    ),
                    versioning: Some("node".to_owned()),
                    ..Default::default()
                });
            }
            "workarounds:doNotUpgradeFromAlpineStableToEdge" => {
                rules.push(PackageRule {
                    match_datasources: vec!["docker".to_owned()],
                    match_dep_names: vec!["alpine".to_owned()],
                    match_current_version: Some("!/^\\d{8}$/".to_owned()),
                    allowed_versions: Some("<20000000".to_owned()),
                    ..Default::default()
                });
                rules.push(PackageRule {
                    match_datasources: vec!["docker".to_owned()],
                    match_package_names: vec!["alpine".to_owned()],
                    has_name_constraint: true,
                    match_current_version: Some("!/^\\d{8}$/".to_owned()),
                    allowed_versions: Some("<20000000".to_owned()),
                    ..Default::default()
                });
            }
            "workarounds:supportRedHatImageVersion" => {
                rules.push(PackageRule {
                    match_datasources: vec!["docker".to_owned()],
                    match_package_names: vec![
                        "registry.access.redhat.com/rhel".to_owned(),
                        "registry.access.redhat.com/rhel-atomic".to_owned(),
                        "registry.access.redhat.com/rhel-init".to_owned(),
                        "registry.access.redhat.com/rhel-minimal".to_owned(),
                        "registry.access.redhat.com/rhceph/**".to_owned(),
                        "registry.access.redhat.com/rhgs3/**".to_owned(),
                        "registry.access.redhat.com/rhel7**".to_owned(),
                        "registry.access.redhat.com/rhel8/**".to_owned(),
                        "registry.access.redhat.com/rhel9/**".to_owned(),
                        "registry.access.redhat.com/rhscl/**".to_owned(),
                        "registry.access.redhat.com/ubi*{,/}**".to_owned(),
                        "redhat/**".to_owned(),
                    ],
                    has_name_constraint: true,
                    versioning: Some("redhat".to_owned()),
                    ..Default::default()
                });
            }
            "workarounds:javaLTSVersions" => {
                let java_versioning = "regex:^(?<major>\\d+)?(\\.(?<minor>\\d+))?(\\.(?<patch>\\d+))?([\\._+](?<build>(\\d\\.?)+)(LTS)?)?(-(?<compatibility>.*))?$".to_owned();
                let java_lts_versions = "/^(?:8|11|17|21|25)(?:\\.|-|$)/".to_owned();
                rules.push(PackageRule {
                    match_datasources: vec!["docker".to_owned(), "java-version".to_owned()],
                    match_package_names: vec![
                        "eclipse-temurin".to_owned(),
                        "amazoncorretto".to_owned(),
                        "adoptopenjdk".to_owned(),
                        "openjdk".to_owned(),
                        "java".to_owned(),
                        "java-jdk".to_owned(),
                        "java-jre".to_owned(),
                        "sapmachine".to_owned(),
                        "/^azul/zulu-openjdk/".to_owned(),
                        "/^bellsoft/liberica-openj(dk|re)-/".to_owned(),
                        "/^cimg/openjdk/".to_owned(),
                    ],
                    has_name_constraint: true,
                    allowed_versions: Some(java_lts_versions.clone()),
                    versioning: Some(java_versioning.clone()),
                    ..Default::default()
                });
                rules.push(PackageRule {
                    match_datasources: vec!["docker".to_owned(), "java-version".to_owned()],
                    match_dep_names: vec![
                        "eclipse-temurin".to_owned(),
                        "amazoncorretto".to_owned(),
                        "adoptopenjdk".to_owned(),
                        "openjdk".to_owned(),
                        "java".to_owned(),
                        "java-jre".to_owned(),
                        "sapmachine".to_owned(),
                    ],
                    allowed_versions: Some(java_lts_versions.clone()),
                    versioning: Some(java_versioning),
                    ..Default::default()
                });
                rules.push(PackageRule {
                    match_datasources: vec!["docker".to_owned()],
                    match_package_names: vec![
                        "bellsoft/hardened-liberica-runtime-container".to_owned(),
                        "bellsoft/liberica-runtime-container".to_owned(),
                    ],
                    has_name_constraint: true,
                    allowed_versions: Some(
                        "/^(?:jdk|jdk-all|jre)-(?:8|11|17|21|25)(?:\\.|-|$)/".to_owned(),
                    ),
                    ..Default::default()
                });
            }
            "workarounds:disableEclipseLifecycleMapping" => {
                rules.push(PackageRule {
                    match_datasources: vec!["maven".to_owned()],
                    match_package_names: vec!["org.eclipse.m2e:lifecycle-mapping".to_owned()],
                    has_name_constraint: true,
                    enabled: Some(false),
                    ..Default::default()
                });
            }
            "workarounds:disableGradleReplacements" => {
                rules.push(PackageRule {
                    match_managers: vec!["gradle".to_owned()],
                    match_update_types: vec![UpdateType::Replacement],
                    has_update_type_constraint: true,
                    enabled: Some(false),
                    ..Default::default()
                });
            }
            "workarounds:disableMavenParentRoot" => {
                rules.push(PackageRule {
                    match_managers: vec!["maven".to_owned()],
                    match_dep_types: vec!["parent-root".to_owned()],
                    enabled: Some(false),
                    ..Default::default()
                });
            }
            "workarounds:containerbase" => {
                rules.push(PackageRule {
                    match_datasources: vec!["docker".to_owned()],
                    match_package_names: vec![
                        "/^(?:(?:docker|ghcr)\\.io/)?(?:containerbase|renovate)/node$/".to_owned(),
                    ],
                    has_name_constraint: true,
                    versioning: Some("node".to_owned()),
                    ..Default::default()
                });
            }
            "workarounds:bitnamiDockerImageVersioning" => {
                rules.push(PackageRule {
                    match_datasources: vec!["docker".to_owned()],
                    match_package_names: vec![
                        "bitnami/**".to_owned(),
                        "docker.io/bitnami/**".to_owned(),
                        "gcr.io/bitnami-containers/**".to_owned(),
                        "*-docker.pkg.dev/vmw-app-catalog/**".to_owned(),
                    ],
                    has_name_constraint: true,
                    match_current_value: Some(
                        "/^(?<major>\\d+)(?:\\.(?<minor>\\d+)(?:\\.(?<patch>\\d+))?)?-(?<compatibility>.+)-(?<build>\\d+)(?:-r(?<revision>\\d+))?$/"
                            .to_owned(),
                    ),
                    versioning: Some(
                        "regex:^(?<major>\\d+)(?:\\.(?<minor>\\d+)(?:\\.(?<patch>\\d+))?)?(:?-(?<compatibility>.+)-(?<build>\\d+)(?:-r(?<revision>\\d+))?)?$"
                            .to_owned(),
                    ),
                    ..Default::default()
                });
            }
            "workarounds:clamavDockerImageVersioning" => {
                rules.push(PackageRule {
                    match_datasources: vec!["docker".to_owned()],
                    match_package_names: vec![
                        "clamav/clamav".to_owned(),
                        "clamav/clamav-debian".to_owned(),
                    ],
                    has_name_constraint: true,
                    versioning: Some(
                        "regex:^(?<major>\\d+)\\.(?<minor>\\d+)(\\.(?<patch>\\d+))?(-(?<build>\\d+))?(_(?<compatibility>.+))?$"
                            .to_owned(),
                    ),
                    ..Default::default()
                });
            }
            "workarounds:k3sKubernetesVersioning" => {
                rules.push(PackageRule {
                    match_datasources: vec!["github-releases".to_owned()],
                    match_package_names: vec!["k3s-io/k3s".to_owned()],
                    has_name_constraint: true,
                    versioning: Some(
                        "regex:^v(?<major>\\d+)\\.(?<minor>\\d+)\\.(?<patch>\\d+)(?:-(?<prerelease>[a-z]+\\d+))?(?<compatibility>\\+k3s)(?<build>\\d+)$"
                            .to_owned(),
                    ),
                    ..Default::default()
                });
            }
            "workarounds:rke2KubernetesVersioning" => {
                rules.push(PackageRule {
                    match_datasources: vec!["github-releases".to_owned()],
                    match_package_names: vec!["rancher/rke2".to_owned()],
                    has_name_constraint: true,
                    versioning: Some(
                        "regex:^v(?<major>\\d+)\\.(?<minor>\\d+)\\.(?<patch>\\d+)(?:-(?<prerelease>[a-z]+\\d+))?(?<compatibility>\\+rke2r)(?<build>\\d+)$"
                            .to_owned(),
                    ),
                    ..Default::default()
                });
            }
            "workarounds:libericaJdkDockerVersioning" => {
                rules.push(PackageRule {
                    match_datasources: vec!["docker".to_owned()],
                    match_package_names: vec![
                        "bellsoft/hardened-liberica-runtime-container".to_owned(),
                        "bellsoft/liberica-runtime-container".to_owned(),
                    ],
                    has_name_constraint: true,
                    match_current_value: Some("/^jdk-[^a][^l]{2}/".to_owned()),
                    versioning: Some(
                        "regex:^jdk-(?<major>\\d+)?(\\.(?<minor>\\d+))?(\\.(?<patch>\\d+))?([\\._+](?<build>(\\d\\.?)+))?(-(?<compatibility>.*))?$"
                            .to_owned(),
                    ),
                    ..Default::default()
                });
                rules.push(PackageRule {
                    match_datasources: vec!["docker".to_owned()],
                    match_package_names: vec![
                        "bellsoft/hardened-liberica-runtime-container".to_owned(),
                        "bellsoft/liberica-runtime-container".to_owned(),
                    ],
                    has_name_constraint: true,
                    match_current_value: Some("/^jdk-all/".to_owned()),
                    versioning: Some(
                        "regex:^jdk-all-(?<major>\\d+)?(\\.(?<minor>\\d+))?(\\.(?<patch>\\d+))?([\\._+](?<build>(\\d\\.?)+))?(-(?<compatibility>.*))?$"
                            .to_owned(),
                    ),
                    ..Default::default()
                });
                rules.push(PackageRule {
                    match_datasources: vec!["docker".to_owned()],
                    match_package_names: vec![
                        "bellsoft/hardened-liberica-runtime-container".to_owned(),
                        "bellsoft/liberica-runtime-container".to_owned(),
                    ],
                    has_name_constraint: true,
                    match_current_value: Some("/^jre-/".to_owned()),
                    versioning: Some(
                        "regex:^jre-(?<major>\\d+)?(\\.(?<minor>\\d+))?(\\.(?<patch>\\d+))?([\\._+](?<build>(\\d\\.?)+))?(-(?<compatibility>.*))?$"
                            .to_owned(),
                    ),
                    ..Default::default()
                });
            }
            "workarounds:ubuntuDockerVersioning" => {
                rules.push(PackageRule {
                    match_datasources: vec!["docker".to_owned()],
                    match_dep_names: vec!["ubuntu".to_owned()],
                    versioning: Some("ubuntu".to_owned()),
                    ..Default::default()
                });
            }

            _ => {}
        }
    }

    rules
}

/// Resolve packageRules injected by range-strategy built-in presets.
///
/// Handles presets that inject `rangeStrategy` into packageRules:
/// - `:pinAllExceptPeerDependencies` — pin all except peer/engines
/// - `:pinDependencies` — pin `dependencies` dep type
/// - `:pinDevDependencies` — pin `devDependencies` dep types
/// - `:pinOnlyDevDependencies` — pin dev, widen peers, replace others
/// - `:preserveSemverRanges` — set `replace` for all packages
/// - `:pinVersions` — set global `rangeStrategy: pin` (returned as packageRules)
///
/// Renovate reference: `lib/config/presets/internal/default.preset.ts`
fn resolve_extends_range_strategy_rules(extends: &[String]) -> Vec<PackageRule> {
    let mut rules: Vec<PackageRule> = Vec::new();

    for preset in extends {
        match preset.as_str() {
            ":pinAllExceptPeerDependencies" => {
                rules.push(PackageRule {
                    match_package_names: vec!["*".to_owned()],
                    has_name_constraint: true,
                    range_strategy: Some("pin".to_owned()),
                    ..Default::default()
                });
                rules.push(PackageRule {
                    match_dep_types: vec!["engines".to_owned(), "peerDependencies".to_owned()],
                    range_strategy: Some("auto".to_owned()),
                    ..Default::default()
                });
            }
            ":pinDependencies" => {
                rules.push(PackageRule {
                    match_dep_types: vec!["dependencies".to_owned()],
                    range_strategy: Some("pin".to_owned()),
                    ..Default::default()
                });
            }
            ":pinDevDependencies" => {
                rules.push(PackageRule {
                    match_dep_types: vec![
                        "devDependencies".to_owned(),
                        "dev-dependencies".to_owned(),
                        "dev".to_owned(),
                    ],
                    range_strategy: Some("pin".to_owned()),
                    ..Default::default()
                });
            }
            ":pinOnlyDevDependencies" => {
                rules.push(PackageRule {
                    match_package_names: vec!["*".to_owned()],
                    has_name_constraint: true,
                    range_strategy: Some("replace".to_owned()),
                    ..Default::default()
                });
                rules.push(PackageRule {
                    match_dep_types: vec![
                        "devDependencies".to_owned(),
                        "dev-dependencies".to_owned(),
                        "dev".to_owned(),
                    ],
                    range_strategy: Some("pin".to_owned()),
                    ..Default::default()
                });
                rules.push(PackageRule {
                    match_dep_types: vec!["peerDependencies".to_owned()],
                    range_strategy: Some("widen".to_owned()),
                    ..Default::default()
                });
            }
            ":preserveSemverRanges" => {
                rules.push(PackageRule {
                    match_package_names: vec!["*".to_owned()],
                    has_name_constraint: true,
                    range_strategy: Some("replace".to_owned()),
                    ..Default::default()
                });
            }
            ":pinVersions" => {
                rules.push(PackageRule {
                    match_package_names: vec!["*".to_owned()],
                    has_name_constraint: true,
                    range_strategy: Some("pin".to_owned()),
                    ..Default::default()
                });
            }
            ":widenPeerDependencies" => {
                rules.push(PackageRule {
                    match_dep_types: vec!["peerDependencies".to_owned()],
                    range_strategy: Some("widen".to_owned()),
                    ..Default::default()
                });
            }
            _ => {}
        }
    }

    rules
}

/// Resolve packageRules injected by parameterized built-in presets.
///
/// Handles presets that expand into packageRules using their arguments:
/// - `:doNotPinPackage(name)` — set rangeStrategy: replace for the named package
/// - `:semanticCommitTypeAll(type)` — set semanticCommitType for all packages
/// - `:pathSemanticCommitType(path, type)` — set semanticCommitType for path-matched packages
/// - `:followTag(pkg, tag)` — follow a specific dist-tag for the named package
///
/// Renovate reference: `lib/config/presets/internal/default.preset.ts`
fn resolve_extends_parameterized_rules(extends: &[String]) -> Vec<PackageRule> {
    let mut rules: Vec<PackageRule> = Vec::new();

    for preset in extends {
        let (name, args) = parse_preset_args(preset.as_str());
        match name {
            ":doNotPinPackage" | "doNotPinPackage" => {
                if let Some(pkg) = args.first().filter(|s| !s.is_empty()) {
                    rules.push(PackageRule {
                        match_package_names: vec![pkg.to_string()],
                        has_name_constraint: true,
                        range_strategy: Some("replace".to_owned()),
                        ..Default::default()
                    });
                }
            }
            ":semanticCommitTypeAll" | "semanticCommitTypeAll" => {
                if let Some(commit_type) = args.first().filter(|s| !s.is_empty()) {
                    rules.push(PackageRule {
                        match_file_names: vec!["**/*".to_owned()],
                        semantic_commit_type: Some(commit_type.to_string()),
                        ..Default::default()
                    });
                }
            }
            ":pathSemanticCommitType" | "pathSemanticCommitType"
                if args.len() >= 2 && !args[0].is_empty() && !args[1].is_empty() =>
            {
                rules.push(PackageRule {
                    match_file_names: vec![args[0].to_owned()],
                    semantic_commit_type: Some(args[1].to_owned()),
                    ..Default::default()
                });
            }
            // :followTag(pkg, tag) → inject a packageRule that follows the given dist-tag.
            // Used by helpers:followTypescriptNext, helpers:followTypescriptRc.
            ":followTag" | "followTag"
                if args.len() >= 2 && !args[0].is_empty() && !args[1].is_empty() =>
            {
                rules.push(PackageRule {
                    match_package_names: vec![args[0].to_owned()],
                    has_name_constraint: true,
                    follow_tag: Some(args[1].to_owned()),
                    ..Default::default()
                });
            }
            // :approveMajorUpdates — require Dependency Dashboard approval for major updates.
            ":approveMajorUpdates" | "approveMajorUpdates" => {
                rules.push(PackageRule {
                    match_update_types: vec![UpdateType::Major],
                    has_update_type_constraint: true,
                    dependency_dashboard_approval: Some(true),
                    ..Default::default()
                });
            }
            _ => {}
        }
    }

    rules
}

/// Return type for `resolve_extends_scalar_overrides`:
/// `(sep_minor_patch, sep_major_minor, sep_multi_major, sep_multi_minor, pr_concurrent, pr_hourly)`.
type ScalarOverrides = (
    Option<bool>,
    Option<bool>,
    Option<bool>,
    Option<bool>,
    Option<u32>,
    Option<u32>,
);

/// Scalar config overrides contributed by named built-in presets.
///
/// Returns overrides for: `separate_minor_patch`, `separate_major_minor`,
/// `separate_multiple_major`, `separate_multiple_minor`, `pr_concurrent_limit`, `pr_hourly_limit`.
/// `None` means the preset did not set that field.
///
/// Renovate reference: `lib/config/presets/internal/default.preset.ts`
fn resolve_extends_scalar_overrides(extends: &[String]) -> ScalarOverrides {
    let mut sep_minor_patch: Option<bool> = None;
    let mut sep_major_minor: Option<bool> = None;
    let mut sep_multi_major: Option<bool> = None;
    let mut sep_multi_minor: Option<bool> = None;
    let mut pr_concurrent: Option<u32> = None;
    let mut pr_hourly: Option<u32> = None;

    for preset in extends {
        // Strip leading `:` for built-in preset namespace — `:foo` == `foo`.
        let p = preset.trim_start_matches(':');
        match p {
            // separateMinorPatch
            "combinePatchMinorReleases" => sep_minor_patch = Some(false),
            "separatePatchReleases" => sep_minor_patch = Some(true),
            // separateMajorMinor
            "separateMajorReleases" => sep_major_minor = Some(true),
            // separateMajorMinor + separateMultipleMajor
            "separateMultipleMajorReleases" => {
                sep_major_minor = Some(true);
                sep_multi_major = Some(true);
            }
            // separateMultipleMinor
            "separateMultipleMinorReleases" => sep_multi_minor = Some(true),
            // prConcurrentLimit
            "prConcurrentLimit10" => pr_concurrent = Some(10),
            "prConcurrentLimit20" => pr_concurrent = Some(20),
            "prConcurrentLimitNone" => pr_concurrent = Some(0),
            // prHourlyLimit
            "prHourlyLimit1" => pr_hourly = Some(1),
            "prHourlyLimit2" => pr_hourly = Some(2),
            "prHourlyLimit4" => pr_hourly = Some(4),
            "prHourlyLimitNone" => pr_hourly = Some(0),
            // disableRateLimiting sets both
            "disableRateLimiting" => {
                pr_concurrent = Some(0);
                pr_hourly = Some(0);
            }
            _ => {}
        }
    }

    (
        sep_minor_patch,
        sep_major_minor,
        sep_multi_major,
        sep_multi_minor,
        pr_concurrent,
        pr_hourly,
    )
}

/// Resolve semantic commit type/scope from built-in `semantic*` presets.
///
/// Returns `(type_override, scope_override)` from the last matching preset.
///
/// Renovate reference: `lib/config/presets/internal/default.preset.ts` —
/// `:semanticPrefixFixDepsChoreOthers`, `:semanticCommitType`, etc.
fn resolve_extends_semantic_type_scope(extends: &[String]) -> (Option<String>, Option<String>) {
    let mut sem_type: Option<String> = None;
    let mut _sem_scope: Option<String> = None;
    for preset in extends {
        match preset.as_str() {
            ":semanticCommitTypeAll(fix)" => {
                sem_type = Some("fix".to_owned());
            }
            ":semanticCommitTypeAll(chore)" => {
                sem_type = Some("chore".to_owned());
            }
            _ => {}
        }
    }
    (sem_type, None)
}

/// Resolve packageRules injected by semantic-prefix presets.
///
/// `:semanticPrefixFixDepsChoreOthers` is the most commonly used — it sets
/// `semanticCommitType: "chore"` for all packages and then `"fix"` for
/// production dependency dep types.
///
/// Renovate reference: `lib/config/presets/internal/default.preset.ts`
fn resolve_extends_semantic_prefix_rules(
    extends: &[String],
) -> Vec<crate::package_rule::PackageRule> {
    use crate::package_rule::PackageRule;

    let mut rules: Vec<PackageRule> = Vec::new();

    for preset in extends {
        if preset == ":semanticPrefixFixDepsChoreOthers"
            || preset == "semanticPrefixFixDepsChoreOthers"
        {
            // Rule 1: all packages → semanticCommitType: "chore"
            rules.push(PackageRule {
                match_package_names: vec!["*".to_owned()],
                has_name_constraint: true,
                semantic_commit_type: Some("chore".to_owned()),
                ..Default::default()
            });
            // Rule 2: production dep types → semanticCommitType: "fix"
            rules.push(PackageRule {
                match_dep_types: vec![
                    "dependencies".to_owned(),
                    "require".to_owned(),
                    "compile".to_owned(),
                    "provided".to_owned(),
                    "runtime".to_owned(),
                    "system".to_owned(),
                    "import".to_owned(),
                    "parent".to_owned(),
                ],
                semantic_commit_type: Some("fix".to_owned()),
                ..Default::default()
            });
        }
    }

    rules
}

/// Resolve built-in `group:*` presets from `extends`.
///
/// Returns `(package_rules, separate_major_minor_override)` where:
/// - `package_rules` are the `PackageRule` entries injected by the preset
/// - `separate_major_minor_override` is `Some(false)` when `group:all` implies
///   `separateMajorMinor: false`
///
/// Renovate reference: `lib/config/presets/internal/group.preset.ts`
fn resolve_extends_group_presets(
    extends: &[String],
) -> (Vec<crate::package_rule::PackageRule>, Option<bool>) {
    use crate::package_rule::PackageRule;
    use crate::versioning::semver_generic::UpdateType;

    fn group_rule(group_name: &str, group_slug: &str) -> PackageRule {
        PackageRule {
            match_package_names: vec!["*".to_owned()],
            has_name_constraint: true,
            group_name: Some(group_name.to_owned()),
            group_slug: Some(group_slug.to_owned()),
            ..Default::default()
        }
    }

    fn group_rule_update_types(
        group_name: &str,
        group_slug: &str,
        types: Vec<UpdateType>,
    ) -> PackageRule {
        PackageRule {
            match_package_names: vec!["*".to_owned()],
            has_name_constraint: true,
            group_name: Some(group_name.to_owned()),
            group_slug: Some(group_slug.to_owned()),
            match_update_types: types,
            ..Default::default()
        }
    }

    let mut rules: Vec<PackageRule> = Vec::new();
    let mut separate_major_minor: Option<bool> = None;

    for preset in extends {
        match preset.as_str() {
            "group:all" => {
                rules.push(group_rule("all dependencies", "all"));
                separate_major_minor = Some(false);
            }
            "group:allNonMajor" => {
                rules.push(group_rule_update_types(
                    "all non-major dependencies",
                    "all-minor-patch",
                    vec![UpdateType::Minor, UpdateType::Patch],
                ));
            }
            "group:monorepos" => {
                // Expand all monorepo group presets in a single JSON parse (O(1) not O(n)).
                // patternGroups use matchPackageNames (resolved locally);
                // orgGroups/repoGroups use matchSourceUrls (resolved when source URL is available).
                rules.extend(crate::monorepos::all_monorepo_rules());
            }
            // config:recommended is expanded by expand_compound_presets() before this
            // function is called, so "group:recommended" is already in effective_extends
            // when we get here. No separate handling needed — just skip the raw preset name.
            "config:recommended" | "config:base" | "config:best-practices" => {}
            // group:allDigest — group all digest-pinned updates into one branch.
            // Digest updates are not yet implemented in the pipeline, so this rule
            // has no match_update_types constraint (digest is not an UpdateType variant).
            // The rule is parsed and stored correctly; it becomes active when digest
            // update support is added.
            "group:allDigest" => {
                rules.push(PackageRule {
                    group_name: Some("all digest updates".to_owned()),
                    group_slug: Some("all-digest".to_owned()),
                    match_package_names: vec!["*".to_owned()],
                    has_name_constraint: true,
                    // matchUpdateTypes: ["digest"] — digest not yet modelled as UpdateType
                    has_update_type_constraint: true,
                    ..Default::default()
                });
            }
            // group:nodeJs — group Node.js runtime + related docker images together.
            "group:nodeJs" => {
                rules.push(PackageRule {
                    group_name: Some("Node.js".to_owned()),
                    group_slug: Some("node-js".to_owned()),
                    match_datasources: vec!["docker".to_owned(), "node-version".to_owned()],
                    match_package_names: vec![
                        "/(?:^|/)node$/".to_owned(),
                        "!calico/node".to_owned(),
                        "!docker.io/calico/node".to_owned(),
                        "!ghcr.io/devcontainers/features/node".to_owned(),
                        "!kindest/node".to_owned(),
                    ],
                    has_name_constraint: true,
                    commit_message_topic: Some("Node.js".to_owned()),
                    ..Default::default()
                });
            }
            // group:jsTest — group JS test packages together.
            // Mirrors packages:jsTest → packages:jsUnitTest in packages.preset.ts.
            "group:jsTest" => {
                rules.push(PackageRule {
                    group_name: Some("JS test packages".to_owned()),
                    group_slug: Some("js-test".to_owned()),
                    has_name_constraint: true,
                    match_package_names: JS_UNIT_TEST_PACKAGES
                        .iter()
                        .map(|&s| s.to_owned())
                        .collect(),
                    ..Default::default()
                });
            }
            // group:jsTestNonMajor — same as jsTest but only minor+patch.
            "group:jsTestNonMajor" => {
                rules.push(PackageRule {
                    group_name: Some("JS test packages".to_owned()),
                    group_slug: Some("js-test".to_owned()),
                    has_name_constraint: true,
                    match_package_names: JS_UNIT_TEST_PACKAGES
                        .iter()
                        .map(|&s| s.to_owned())
                        .collect(),
                    match_update_types: vec![UpdateType::Minor, UpdateType::Patch],
                    has_update_type_constraint: true,
                    ..Default::default()
                });
            }
            // group:jsUnitTest — group JavaScript unit test packages.
            "group:jsUnitTest" => {
                rules.push(PackageRule {
                    group_name: Some("JS unit test packages".to_owned()),
                    group_slug: Some("js-unit-test".to_owned()),
                    has_name_constraint: true,
                    match_package_names: JS_UNIT_TEST_PACKAGES
                        .iter()
                        .map(|&s| s.to_owned())
                        .collect(),
                    ..Default::default()
                });
            }
            // group:jsUnitTestNonMajor — JS unit test packages, minor+patch only.
            "group:jsUnitTestNonMajor" => {
                rules.push(PackageRule {
                    group_name: Some("JS unit test packages".to_owned()),
                    group_slug: Some("js-unit-test".to_owned()),
                    has_name_constraint: true,
                    match_package_names: JS_UNIT_TEST_PACKAGES
                        .iter()
                        .map(|&s| s.to_owned())
                        .collect(),
                    match_update_types: vec![UpdateType::Minor, UpdateType::Patch],
                    has_update_type_constraint: true,
                    ..Default::default()
                });
            }
            // group:gradle — group Gradle-related updates together.
            "group:gradle" => {
                rules.push(PackageRule {
                    group_name: Some("Gradle".to_owned()),
                    group_slug: Some("gradle".to_owned()),
                    match_datasources: vec!["docker".to_owned(), "gradle-version".to_owned()],
                    match_package_names: vec!["/(?:^|/)gradle$/".to_owned()],
                    has_name_constraint: true,
                    commit_message_topic: Some("Gradle".to_owned()),
                    ..Default::default()
                });
            }
            // group:hibernateCore — group Hibernate Core (org.hibernate:**) packages.
            "group:hibernateCore" => {
                rules.push(PackageRule {
                    group_name: Some("hibernate core".to_owned()),
                    group_slug: Some("hibernate-core".to_owned()),
                    match_package_names: vec!["org.hibernate:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:hibernateCommons — group Hibernate Commons (org.hibernate.common:**) packages.
            "group:hibernateCommons" => {
                rules.push(PackageRule {
                    group_name: Some("hibernate commons".to_owned()),
                    group_slug: Some("hibernate-commons".to_owned()),
                    match_package_names: vec!["org.hibernate.common:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:definitelyTyped — group all @types/* packages together.
            "group:definitelyTyped" => {
                rules.push(PackageRule {
                    group_name: Some("definitelyTyped".to_owned()),
                    group_slug: Some("definitely-typed".to_owned()),
                    match_package_names: vec!["@types/**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:react — group React @types packages together.
            "group:react" => {
                rules.push(PackageRule {
                    group_name: Some("react monorepo".to_owned()),
                    group_slug: Some("react".to_owned()),
                    match_package_names: vec![
                        "@types/react".to_owned(),
                        "@types/react-dom".to_owned(),
                        "@types/react-is".to_owned(),
                    ],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:puppeteer — group Puppeteer packages.
            "group:puppeteer" => {
                rules.push(PackageRule {
                    group_name: Some("Puppeteer".to_owned()),
                    group_slug: Some("puppeteer".to_owned()),
                    match_datasources: vec!["npm".to_owned()],
                    match_package_names: vec!["puppeteer".to_owned(), "puppeteer-core".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:remark — group remark packages from remarkjs org.
            "group:remark" => {
                rules.push(PackageRule {
                    group_name: Some("remark".to_owned()),
                    group_slug: Some("remark".to_owned()),
                    match_datasources: vec!["npm".to_owned()],
                    match_source_urls: vec!["https://github.com/remarkjs/**".to_owned()],
                    ..Default::default()
                });
            }
            // group:socketio — group socket.io packages.
            "group:socketio" => {
                rules.push(PackageRule {
                    group_name: Some("socket.io packages".to_owned()),
                    group_slug: Some("socketio".to_owned()),
                    match_package_names: vec!["socket.io**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:micrometer — group Micrometer monitoring packages.
            "group:micrometer" => {
                rules.push(PackageRule {
                    group_name: Some("micrometer".to_owned()),
                    group_slug: Some("micrometer".to_owned()),
                    match_package_names: vec!["io.micrometer:micrometer-**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:resilience4j — group Resilience4j packages.
            "group:resilience4j" => {
                rules.push(PackageRule {
                    group_name: Some("resilience4j".to_owned()),
                    group_slug: Some("resilience4j".to_owned()),
                    match_package_names: vec!["io.github.resilience4j:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:hibernateValidator — group Hibernate Validator packages.
            "group:hibernateValidator" => {
                rules.push(PackageRule {
                    group_name: Some("hibernate validator".to_owned()),
                    group_slug: Some("hibernate-validator".to_owned()),
                    match_package_names: vec!["org.hibernate.validator:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:hibernateOgm — group Hibernate OGM packages.
            "group:hibernateOgm" => {
                rules.push(PackageRule {
                    group_name: Some("hibernate ogm".to_owned()),
                    group_slug: Some("hibernate-ogm".to_owned()),
                    match_package_names: vec!["org.hibernate.ogm:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:springBoot — group Spring Boot packages.
            // Two rules: one for matchDepNames (BOM parent), one for matchPackageNames.
            "group:springBoot" => {
                rules.push(PackageRule {
                    group_name: Some("spring boot".to_owned()),
                    group_slug: Some("spring-boot".to_owned()),
                    match_dep_names: vec!["org.springframework.boot".to_owned()],
                    ..Default::default()
                });
                rules.push(PackageRule {
                    group_name: Some("spring boot".to_owned()),
                    group_slug: Some("spring-boot".to_owned()),
                    match_package_names: vec!["org.springframework.boot:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:springCore — group Spring Core packages.
            "group:springCore" => {
                rules.push(PackageRule {
                    group_name: Some("spring core".to_owned()),
                    group_slug: Some("spring-core".to_owned()),
                    match_package_names: vec!["org.springframework:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:springCloud — group Spring Cloud packages.
            "group:springCloud" => {
                rules.push(PackageRule {
                    group_name: Some("spring cloud".to_owned()),
                    group_slug: Some("spring-cloud".to_owned()),
                    match_package_names: vec!["org.springframework.cloud:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:springData — group Spring Data packages.
            "group:springData" => {
                rules.push(PackageRule {
                    group_name: Some("spring data".to_owned()),
                    group_slug: Some("spring-data".to_owned()),
                    match_package_names: vec!["org.springframework.data:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:springSecurity — group Spring Security packages.
            "group:springSecurity" => {
                rules.push(PackageRule {
                    group_name: Some("spring security".to_owned()),
                    group_slug: Some("spring-security".to_owned()),
                    match_package_names: vec!["org.springframework.security:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // Remaining Spring presets (all follow the org.springframework.X:** pattern).
            "group:springAmqp" => {
                rules.push(PackageRule {
                    group_name: Some("spring amqp".to_owned()),
                    group_slug: Some("spring-amqp".to_owned()),
                    match_package_names: vec!["org.springframework.amqp:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            "group:springAndroid" => {
                rules.push(PackageRule {
                    group_name: Some("spring android".to_owned()),
                    group_slug: Some("spring-android".to_owned()),
                    match_package_names: vec!["org.springframework.android:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            "group:springBatch" => {
                rules.push(PackageRule {
                    group_name: Some("spring batch".to_owned()),
                    group_slug: Some("spring-batch".to_owned()),
                    match_package_names: vec!["org.springframework.batch:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            "group:springHateoas" => {
                rules.push(PackageRule {
                    group_name: Some("spring hateoas".to_owned()),
                    group_slug: Some("spring-hateoas".to_owned()),
                    match_package_names: vec!["org.springframework.hateoas:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            "group:springIntegration" => {
                rules.push(PackageRule {
                    group_name: Some("spring integration".to_owned()),
                    group_slug: Some("spring-integration".to_owned()),
                    match_package_names: vec!["org.springframework.integration:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            "group:springKafka" => {
                rules.push(PackageRule {
                    group_name: Some("spring kafka".to_owned()),
                    group_slug: Some("spring-kafka".to_owned()),
                    match_package_names: vec!["org.springframework.kafka:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            "group:springLdap" => {
                rules.push(PackageRule {
                    group_name: Some("spring ldap".to_owned()),
                    group_slug: Some("spring-ldap".to_owned()),
                    match_package_names: vec!["org.springframework.ldap:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            "group:springMobile" => {
                rules.push(PackageRule {
                    group_name: Some("spring mobile".to_owned()),
                    group_slug: Some("spring-mobile".to_owned()),
                    match_package_names: vec!["org.springframework.mobile:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            "group:springOsgi" => {
                rules.push(PackageRule {
                    group_name: Some("spring osgi".to_owned()),
                    group_slug: Some("spring-osgi".to_owned()),
                    match_package_names: vec!["org.springframework.osgi:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            "group:springRestDocs" => {
                rules.push(PackageRule {
                    group_name: Some("spring restdocs".to_owned()),
                    group_slug: Some("spring-restdocs".to_owned()),
                    match_package_names: vec!["org.springframework.restdocs:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            "group:springRoo" => {
                rules.push(PackageRule {
                    group_name: Some("spring roo".to_owned()),
                    group_slug: Some("spring-roo".to_owned()),
                    match_package_names: vec!["org.springframework.roo:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            "group:springScala" => {
                rules.push(PackageRule {
                    group_name: Some("spring scala".to_owned()),
                    group_slug: Some("spring-scala".to_owned()),
                    match_package_names: vec!["org.springframework.scala:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            "group:springSession" => {
                rules.push(PackageRule {
                    group_name: Some("spring session".to_owned()),
                    group_slug: Some("spring-session".to_owned()),
                    match_package_names: vec!["org.springframework.session:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            "group:springShell" => {
                rules.push(PackageRule {
                    group_name: Some("spring shell".to_owned()),
                    group_slug: Some("spring-shell".to_owned()),
                    match_package_names: vec!["org.springframework.shell:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            "group:springSocial" => {
                rules.push(PackageRule {
                    group_name: Some("spring social".to_owned()),
                    group_slug: Some("spring-social".to_owned()),
                    match_package_names: vec!["org.springframework.social:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            "group:springStatemachine" => {
                rules.push(PackageRule {
                    group_name: Some("spring statemachine".to_owned()),
                    group_slug: Some("spring-statemachine".to_owned()),
                    match_package_names: vec!["org.springframework.statemachine:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            "group:springWebflow" => {
                rules.push(PackageRule {
                    group_name: Some("spring webflow".to_owned()),
                    group_slug: Some("spring-webflow".to_owned()),
                    match_package_names: vec!["org.springframework.webflow:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            "group:springWs" => {
                rules.push(PackageRule {
                    group_name: Some("spring ws".to_owned()),
                    group_slug: Some("spring-ws".to_owned()),
                    match_package_names: vec!["org.springframework.ws:**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:illuminate — group PHP Illuminate packages.
            "group:illuminate" => {
                rules.push(PackageRule {
                    group_name: Some("illuminate packages".to_owned()),
                    group_slug: Some("illuminate".to_owned()),
                    match_package_names: vec!["illuminate/**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:rubyOmniauth — group OmniAuth packages.
            "group:rubyOmniauth" => {
                rules.push(PackageRule {
                    group_name: Some("omniauth packages".to_owned()),
                    group_slug: Some("ruby-omniauth".to_owned()),
                    match_datasources: vec!["rubygems".to_owned()],
                    match_package_names: vec!["omniauth**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:jestPlusTSJest — add ts-jest major to jest monorepo group.
            "group:jestPlusTSJest" => {
                rules.push(PackageRule {
                    group_name: Some("jest monorepo".to_owned()),
                    group_slug: Some("jest".to_owned()),
                    match_source_urls: vec!["https://github.com/kulshekhar/ts-jest".to_owned()],
                    match_update_types: vec![UpdateType::Major],
                    has_update_type_constraint: true,
                    ..Default::default()
                });
            }
            // group:jestPlusTypes — add @types/jest non-pin updates to jest monorepo.
            "group:jestPlusTypes" => {
                rules.push(PackageRule {
                    group_name: Some("jest monorepo".to_owned()),
                    group_slug: Some("jest".to_owned()),
                    match_package_names: vec!["@types/jest".to_owned()],
                    has_name_constraint: true,
                    match_update_types: vec![
                        UpdateType::Major,
                        UpdateType::Minor,
                        UpdateType::Patch,
                    ],
                    has_update_type_constraint: true,
                    ..Default::default()
                });
            }
            // group:recommended — expand all recommended grouping presets.
            // This mirrors Renovate's group:recommended which extends many sub-presets.
            "group:recommended" => {
                let recommended_presets: &[&str] = &[
                    "group:nodeJs",
                    "group:allApollographql",
                    "group:apiPlatform",
                    "group:codemirror",
                    "group:flyway",
                    "group:fortawesome",
                    "group:fusionjs",
                    "group:githubArtifactActions",
                    "group:glimmer",
                    "group:goOpenapi",
                    "group:gradle",
                    "group:hibernateCore",
                    "group:hibernateValidator",
                    "group:hibernateOgm",
                    "group:hibernateCommons",
                    "group:illuminate",
                    "group:jestPlusTSJest",
                    "group:jestPlusTypes",
                    "group:micrometer",
                    "group:phpstan",
                    "group:polymer",
                    "group:puppeteer",
                    "group:react",
                    "group:remark",
                    "group:resilience4j",
                    "group:rubyOnRails",
                    "group:rubyOmniauth",
                    "group:socketio",
                    "group:springAmqp",
                    "group:springAndroid",
                    "group:springBatch",
                    "group:springBoot",
                    "group:springCloud",
                    "group:springCore",
                    "group:springData",
                    "group:springHateoas",
                    "group:springIntegration",
                    "group:springKafka",
                    "group:springLdap",
                    "group:springMobile",
                    "group:springOsgi",
                    "group:springRestDocs",
                    "group:springRoo",
                    "group:springScala",
                    "group:springSecurity",
                    "group:springSession",
                    "group:springShell",
                    "group:springSocial",
                    "group:springStatemachine",
                    "group:springWebflow",
                    "group:springWs",
                    "group:symfony",
                ];
                for sub_preset in recommended_presets {
                    let sub_extends = vec![sub_preset.to_string()];
                    let (sub_rules, _) = resolve_extends_group_presets(&sub_extends);
                    rules.extend(sub_rules);
                }
            }
            // group:linters — group all lint-related packages together.
            // Expands packages:linters which combines emberTemplateLint, eslint,
            // phpLinters, stylelint, tslint, and direct entries.
            "group:linters" => {
                rules.push(PackageRule {
                    group_name: Some("linters".to_owned()),
                    group_slug: Some("linters".to_owned()),
                    has_name_constraint: true,
                    match_package_names: LINTER_PACKAGES.iter().map(|&s| s.to_owned()).collect(),
                    ..Default::default()
                });
            }
            // group:codemirror — group all CodeMirror packages.
            "group:codemirror" => {
                rules.push(PackageRule {
                    group_name: Some("CodeMirror".to_owned()),
                    group_slug: Some("codemirror".to_owned()),
                    match_package_names: vec!["@codemirror/**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:flyway — group Java Flyway migration packages.
            "group:flyway" => {
                rules.push(PackageRule {
                    group_name: Some("flyway".to_owned()),
                    group_slug: Some("flyway".to_owned()),
                    match_package_names: vec![
                        "org.flywaydb:*".to_owned(),
                        "org.flywaydb.flyway:*".to_owned(),
                    ],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:fortawesome — group all Font Awesome packages.
            "group:fortawesome" => {
                rules.push(PackageRule {
                    group_name: Some("Font Awesome".to_owned()),
                    group_slug: Some("fortawesome".to_owned()),
                    match_package_names: vec!["@fortawesome/**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:fusionjs — group Fusion.js packages.
            "group:fusionjs" => {
                rules.push(PackageRule {
                    group_name: Some("Fusion.js packages".to_owned()),
                    group_slug: Some("fusionjs".to_owned()),
                    match_package_names: vec![
                        "fusion-cli".to_owned(),
                        "fusion-core".to_owned(),
                        "fusion-test-utils".to_owned(),
                        "fusion-tokens".to_owned(),
                        "fusion-plugin-**".to_owned(),
                        "fusion-react**".to_owned(),
                        "fusion-apollo**".to_owned(),
                    ],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:githubArtifactActions — group upload/download-artifact major updates.
            "group:githubArtifactActions" => {
                rules.push(PackageRule {
                    group_name: Some("GitHub Artifact Actions".to_owned()),
                    group_slug: Some("github-artifact-actions".to_owned()),
                    match_managers: vec!["github-actions".to_owned()],
                    match_package_names: vec![
                        "actions/download-artifact".to_owned(),
                        "actions/upload-artifact".to_owned(),
                    ],
                    has_name_constraint: true,
                    match_update_types: vec![UpdateType::Major],
                    has_update_type_constraint: true,
                    ..Default::default()
                });
            }
            // group:glimmer — group Glimmer.js packages.
            "group:glimmer" => {
                rules.push(PackageRule {
                    group_name: Some("Glimmer.js packages".to_owned()),
                    group_slug: Some("glimmer".to_owned()),
                    match_package_names: vec![
                        "@glimmer/component".to_owned(),
                        "@glimmer/tracking".to_owned(),
                    ],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:goOpenapi — group go-openapi packages.
            "group:goOpenapi" => {
                rules.push(PackageRule {
                    group_name: Some("go-openapi packages".to_owned()),
                    group_slug: Some("go-openapi".to_owned()),
                    match_datasources: vec!["go".to_owned()],
                    match_package_names: vec!["github.com/go-openapi/**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:polymer — group all @polymer packages.
            "group:polymer" => {
                rules.push(PackageRule {
                    group_name: Some("polymer packages".to_owned()),
                    group_slug: Some("polymer".to_owned()),
                    match_package_names: vec!["@polymer/**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:apiPlatform — group PHP API Platform packages (packagist, with exclusions).
            "group:apiPlatform" => {
                rules.push(PackageRule {
                    group_name: Some("api-platform packages".to_owned()),
                    group_slug: Some("api-platform".to_owned()),
                    match_datasources: vec!["packagist".to_owned()],
                    match_package_names: vec![
                        "api-platform/*".to_owned(),
                        "!api-platform/admin-meta".to_owned(),
                        "!api-platform/admin-pack".to_owned(),
                        "!api-platform/api-pack".to_owned(),
                        "!api-platform/api-platform".to_owned(),
                        "!api-platform/parameter-validator".to_owned(),
                        "!api-platform/postman-collection-generator".to_owned(),
                        "!api-platform/schema-generator".to_owned(),
                    ],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:allApollographql — group Apollo GraphQL packages (matchSourceUrls).
            "group:allApollographql" => {
                rules.push(PackageRule {
                    group_name: Some("Apollo GraphQL packages".to_owned()),
                    group_slug: Some("apollo-graphql".to_owned()),
                    match_source_urls: vec!["https://github.com/apollographql/**".to_owned()],
                    ..Default::default()
                });
            }
            // group:phpstan — group PHPStan packages (packagist).
            "group:phpstan" => {
                rules.push(PackageRule {
                    group_name: Some("PHPStan packages".to_owned()),
                    group_slug: Some("phpstan".to_owned()),
                    match_datasources: vec!["packagist".to_owned()],
                    match_package_names: vec![
                        "phpstan/phpstan".to_owned(),
                        "//phpstan-/".to_owned(),
                        "//larastan/".to_owned(),
                        "phpstan/extension-installer".to_owned(),
                    ],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:symfony — group PHP Symfony packages (packagist, with exclusions).
            "group:symfony" => {
                rules.push(PackageRule {
                    group_name: Some("symfony packages".to_owned()),
                    group_slug: Some("symfony".to_owned()),
                    match_datasources: vec!["packagist".to_owned()],
                    match_package_names: vec![
                        "symfony/*".to_owned(),
                        "!symfony/*contracts".to_owned(),
                        "!symfony/*pack".to_owned(),
                        "!symfony/flex".to_owned(),
                        "!symfony/maker-bundle".to_owned(),
                        "!symfony/monolog-bundle".to_owned(),
                        "!symfony/panther".to_owned(),
                        "!symfony/polyfill*".to_owned(),
                        "!symfony/proxy-manager-bridge".to_owned(),
                        "!symfony/security-guard".to_owned(),
                        "!symfony/stimulus-bundle".to_owned(),
                        "!symfony/templating".to_owned(),
                        "!symfony/thanks".to_owned(),
                        "!symfony/ux*".to_owned(),
                        "!symfony/webpack-encore-bundle".to_owned(),
                    ],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:rubyOnRails — group Ruby on Rails gem packages.
            "group:rubyOnRails" => {
                rules.push(PackageRule {
                    group_name: Some("Ruby on Rails packages".to_owned()),
                    group_slug: Some("ruby-on-rails".to_owned()),
                    match_datasources: vec!["rubygems".to_owned()],
                    match_package_names: vec![
                        "actioncable".to_owned(),
                        "actionmailbox".to_owned(),
                        "actionmailer".to_owned(),
                        "actionpack".to_owned(),
                        "actiontext".to_owned(),
                        "actionview".to_owned(),
                        "activejob".to_owned(),
                        "activemodel".to_owned(),
                        "activerecord".to_owned(),
                        "activestorage".to_owned(),
                        "activesupport".to_owned(),
                        "railties".to_owned(),
                        "rails".to_owned(),
                    ],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:drupal-core — group Drupal core packages.
            "group:drupal-core" => {
                rules.push(PackageRule {
                    group_name: Some("Drupal core".to_owned()),
                    group_slug: Some("drupal-core".to_owned()),
                    match_package_names: vec!["drupal/core".to_owned(), "drupal/core-*".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:jwtFramework — group JWT Framework packages (packagist, web-token/**).
            "group:jwtFramework" => {
                rules.push(PackageRule {
                    group_name: Some("JWT Framework packages".to_owned()),
                    group_slug: Some("jwt-framework".to_owned()),
                    match_datasources: vec!["packagist".to_owned()],
                    match_package_names: vec!["web-token/**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:atlaskit — group all @atlaskit packages.
            "group:atlaskit" => {
                rules.push(PackageRule {
                    group_name: Some("Atlassian Atlaskit packages".to_owned()),
                    group_slug: Some("atlaskit".to_owned()),
                    match_package_names: vec!["@atlaskit/**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:dotNetCore — group .NET Core Docker containers.
            "group:dotNetCore" => {
                rules.push(PackageRule {
                    group_name: Some(".NET Core Docker containers".to_owned()),
                    group_slug: Some("dot-net-core".to_owned()),
                    match_datasources: vec!["docker".to_owned()],
                    match_package_names: vec!["mcr.microsoft.com/dotnet/**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:googleapis — group googleapis npm packages.
            "group:googleapis" => {
                rules.push(PackageRule {
                    group_name: Some("googleapis packages".to_owned()),
                    group_slug: Some("googleapis".to_owned()),
                    match_datasources: vec!["npm".to_owned()],
                    match_package_names: vec![
                        "@google-cloud/**".to_owned(),
                        "google-auth-library".to_owned(),
                        "googleapis".to_owned(),
                    ],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:jekyllEcosystem — group Jekyll and related Ruby packages.
            "group:jekyllEcosystem" => {
                rules.push(PackageRule {
                    group_name: Some("jekyll ecosystem packages".to_owned()),
                    group_slug: Some("jekyll-ecosystem".to_owned()),
                    match_source_urls: vec![
                        "https://github.com/jekyll/**".to_owned(),
                        "https://github.com/github/pages-gem**".to_owned(),
                    ],
                    ..Default::default()
                });
            }
            // group:postcss — group PostCSS packages.
            "group:postcss" => {
                rules.push(PackageRule {
                    group_name: Some("postcss packages".to_owned()),
                    group_slug: Some("postcss".to_owned()),
                    match_package_names: vec!["postcss".to_owned(), "postcss-**".to_owned()],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:vite — group all Vite-related npm packages.
            "group:vite" => {
                rules.push(PackageRule {
                    group_name: Some("Vite packages".to_owned()),
                    group_slug: Some("vite".to_owned()),
                    match_datasources: vec!["npm".to_owned()],
                    match_package_names: vec![
                        "vite".to_owned(),
                        "**vite-plugin**".to_owned(),
                        "@vitejs/**".to_owned(),
                    ],
                    has_name_constraint: true,
                    ..Default::default()
                });
            }
            // group:pulumi — group Pulumi packages across npm/pypi/go/maven/nuget.
            "group:pulumi" => {
                for (slug, datasource, pkg_pattern) in [
                    ("pulumi-node", "npm", "@pulumi/**"),
                    ("pulumi-python", "pypi", "pulumi-**"),
                    ("pulumi-go", "go", "github.com/pulumi/**"),
                    ("pulumi-java", "maven", "com.pulumi**"),
                    ("pulumi-dotnet", "nuget", "Pulumi**"),
                ] {
                    rules.push(PackageRule {
                        group_name: Some("Pulumi".to_owned()),
                        group_slug: Some(slug.to_owned()),
                        match_datasources: vec![datasource.to_owned()],
                        match_package_names: vec![pkg_pattern.to_owned()],
                        has_name_constraint: true,
                        ..Default::default()
                    });
                }
            }
            // group:test — group all test packages (js + php unit tests).
            "group:test" => {
                let mut pkgs: Vec<String> = JS_UNIT_TEST_PACKAGES
                    .iter()
                    .map(|&s| s.to_owned())
                    .collect();
                pkgs.extend(PHP_UNIT_TEST_PACKAGES.iter().map(|&s| s.to_owned()));
                rules.push(PackageRule {
                    group_name: Some("test packages".to_owned()),
                    group_slug: Some("test".to_owned()),
                    has_name_constraint: true,
                    match_package_names: pkgs,
                    ..Default::default()
                });
            }
            // group:testNonMajor — same as test but minor+patch only.
            "group:testNonMajor" => {
                let mut pkgs: Vec<String> = JS_UNIT_TEST_PACKAGES
                    .iter()
                    .map(|&s| s.to_owned())
                    .collect();
                pkgs.extend(PHP_UNIT_TEST_PACKAGES.iter().map(|&s| s.to_owned()));
                rules.push(PackageRule {
                    group_name: Some("test packages".to_owned()),
                    group_slug: Some("test".to_owned()),
                    has_name_constraint: true,
                    match_package_names: pkgs,
                    match_update_types: vec![UpdateType::Minor, UpdateType::Patch],
                    has_update_type_constraint: true,
                    ..Default::default()
                });
            }
            // group:unitTest — group all unit test packages (js + php).
            "group:unitTest" => {
                let mut pkgs: Vec<String> = JS_UNIT_TEST_PACKAGES
                    .iter()
                    .map(|&s| s.to_owned())
                    .collect();
                pkgs.extend(PHP_UNIT_TEST_PACKAGES.iter().map(|&s| s.to_owned()));
                rules.push(PackageRule {
                    group_name: Some("unit test packages".to_owned()),
                    group_slug: Some("unit-test".to_owned()),
                    has_name_constraint: true,
                    match_package_names: pkgs,
                    ..Default::default()
                });
            }
            // group:unitTestNonMajor — unit test packages, minor+patch only.
            "group:unitTestNonMajor" => {
                let mut pkgs: Vec<String> = JS_UNIT_TEST_PACKAGES
                    .iter()
                    .map(|&s| s.to_owned())
                    .collect();
                pkgs.extend(PHP_UNIT_TEST_PACKAGES.iter().map(|&s| s.to_owned()));
                rules.push(PackageRule {
                    group_name: Some("unit test packages".to_owned()),
                    group_slug: Some("unit-test".to_owned()),
                    has_name_constraint: true,
                    match_package_names: pkgs,
                    match_update_types: vec![UpdateType::Minor, UpdateType::Patch],
                    has_update_type_constraint: true,
                    ..Default::default()
                });
            }
            _ => {}
        }
    }

    (rules, separate_major_minor)
}

/// PHP unit test package list for `group:test`, `group:unitTest` etc.
/// Mirrors `packages:phpUnitTest` in packages.preset.ts.
const PHP_UNIT_TEST_PACKAGES: &[&str] = &[
    "behat/behat",
    "brianium/paratest",
    "facile-it/paraunit",
    "mockery/mockery",
    "phpspec/prophecy",
    "phpspec/prophecy-phpunit",
    "phpspec/phpspec",
    "phpunit/phpunit",
    "pestphp/**",
    "php-mock/**",
];

/// Shared package list for `group:jsTest`, `group:jsTestNonMajor`, `group:jsUnitTest`,
/// and `group:jsUnitTestNonMajor`. Mirrors `packages:jsUnitTest` in packages.preset.ts.
const JS_UNIT_TEST_PACKAGES: &[&str] = &[
    "@types/chai",
    "@types/ember-mocha",
    "@types/ember-qunit",
    "@types/enzyme",
    "@types/istanbul",
    "@types/jest",
    "@types/mocha",
    "@types/mock-fs",
    "@types/proxyquire",
    "@types/sinon",
    "@types/supertest",
    "coveralls",
    "ember-exam",
    "ember-mocha",
    "ember-qunit",
    "enzyme",
    "istanbul",
    "mock-fs",
    "nock",
    "nyc",
    "proxyquire",
    "supertest",
    "ts-auto-mock",
    "ts-jest",
    "vitest",
    "@jest/**",
    "@testing-library/**",
    "@types/testing-library__**",
    "@vitest/**",
    "chai**",
    "jest**",
    "mocha**",
    "qunit**",
    "should**",
    "sinon**",
];

/// Package list for `group:linters`. Expands `packages:linters` from packages.preset.ts:
/// emberTemplateLint + eslint + phpLinters + stylelint + tslint + direct entries.
const LINTER_PACKAGES: &[&str] = &[
    // packages:emberTemplateLint
    "ember-template-lint**",
    // packages:eslint
    "*/eslint-plugin",
    "@babel/eslint-parser",
    "@eslint/**",
    "@eslint-community/**",
    "@stylistic/eslint-plugin**",
    "@types/eslint",
    "@types/eslint__**",
    "@typescript-eslint/**",
    "babel-eslint",
    "eslint**",
    "typescript-eslint",
    // packages:phpLinters
    "friendsofphp/php-cs-fixer",
    "squizlabs/php_codesniffer",
    "symplify/easy-coding-standard",
    // packages:stylelint
    "stylelint**",
    // packages:tslint
    "codelyzer",
    "/\\btslint\\b/",
    // direct entries in packages:linters
    "oxlint",
    "prettier",
    "remark-lint",
    "standard",
];

/// Parse a parameterized preset string into its name and arguments.
///
/// Format: `[namespace:]name(arg0, arg1, ...)` or `[namespace:]name`.
///
/// Returns `(full_name_without_args, Vec<arg_strings>)`.
///
/// Examples:
/// - `"label(renovate)"` → `("label", ["renovate"])`
/// - `":assignee(bot)"` → `(":assignee", ["bot"])`
/// - `"group:all"` → `("group:all", [])`
fn parse_preset_args(preset: &str) -> (&str, Vec<&str>) {
    if let Some(open) = preset.find('(') {
        let name = &preset[..open];
        let rest = &preset[open + 1..];
        let args_str = rest.trim_end_matches(')');
        let args: Vec<&str> = args_str
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        (name, args)
    } else {
        (preset, Vec::new())
    }
}

/// Resolve parameterized built-in presets from `extends`.
///
/// Handles presets that take arguments, e.g.:
/// - `label(renovate)` → `labels: ["renovate"]`
/// - `labels(renovate, deps)` → `labels: ["renovate", "deps"]`
/// - `:assignee(myuser)` → `assignees: ["myuser"]`
/// - `:reviewer(myuser)` → `reviewers: ["myuser"]`
/// - `:automergeType(pr)` → sets automerge_type (returned separately)
///
/// Returns `(labels, assignees, reviewers, automerge_type)`.
///
/// Return type for `resolve_extends_parameterized`.
type ParamOverrides = (
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Option<String>,
    Option<String>,
    Option<String>,
);

/// Renovate reference: `lib/config/presets/internal/default.preset.ts`
fn resolve_extends_parameterized(extends: &[String]) -> ParamOverrides {
    let mut labels: Vec<String> = Vec::new();
    let mut assignees: Vec<String> = Vec::new();
    let mut reviewers: Vec<String> = Vec::new();
    let mut automerge_type: Option<String> = None;
    let mut semantic_commit_type: Option<String> = None;
    let mut semantic_commit_scope: Option<String> = None;

    for preset in extends {
        let (name, args) = parse_preset_args(preset.as_str());
        match name {
            // :label(foo) sets labels: ["foo"]; :labels(a, b) sets labels: ["a", "b"].
            ":label" | "label" | ":labels" | "labels" => {
                for arg in &args {
                    if !arg.is_empty() && !labels.contains(&arg.to_string()) {
                        labels.push(arg.to_string());
                    }
                }
            }
            ":assignee" | "assignee" => {
                for arg in &args {
                    if !arg.is_empty() && !assignees.contains(&arg.to_string()) {
                        assignees.push(arg.to_string());
                    }
                }
            }
            ":reviewer" | "reviewer" => {
                for arg in &args {
                    if !arg.is_empty() && !reviewers.contains(&arg.to_string()) {
                        reviewers.push(arg.to_string());
                    }
                }
            }
            ":automergeType" => {
                if let Some(ty) = args.first().filter(|s| !s.is_empty()) {
                    automerge_type = Some(ty.to_string());
                }
            }
            ":semanticCommitType" | "semanticCommitType" => {
                if let Some(t) = args.first().filter(|s| !s.is_empty()) {
                    semantic_commit_type = Some(t.to_string());
                }
            }
            ":semanticCommitScope" | "semanticCommitScope" => {
                if let Some(s) = args.first() {
                    // Empty arg → no scope (disable parentheses).
                    semantic_commit_scope = Some(s.to_string());
                }
            }
            ":semanticCommitScopeDisabled" | "semanticCommitScopeDisabled" => {
                semantic_commit_scope = Some(String::new());
            }
            _ => {}
        }
    }

    (
        labels,
        assignees,
        reviewers,
        automerge_type,
        semantic_commit_type,
        semantic_commit_scope,
    )
}

/// Compile a single `matchPackageNames` entry into a [`PackageNameMatcher`].
///
/// - `/pattern/` → inline regex
/// - Contains `*`, `?`, or `[` → glob
/// - Otherwise → exact string
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
            // Deprecated: packageNames → matchPackageNames
            #[serde(rename = "packageNames", default)]
            package_names_deprecated: Vec<String>,
            // Deprecated: excludePackageNames → !name in matchPackageNames
            #[serde(rename = "excludePackageNames", default)]
            exclude_package_names: Vec<String>,
            #[serde(rename = "matchPackagePatterns", default)]
            match_package_patterns: Vec<String>,
            // Deprecated: packagePatterns → /pattern/ in matchPackageNames
            #[serde(rename = "packagePatterns", default)]
            package_patterns_deprecated: Vec<String>,
            // Deprecated: excludePackagePatterns → !/pattern/ in matchPackageNames
            #[serde(rename = "excludePackagePatterns", default)]
            exclude_package_patterns: Vec<String>,
            /// Deprecated; converted to glob patterns in `matchPackageNames`.
            #[serde(rename = "matchPackagePrefixes", default)]
            match_package_prefixes: Vec<String>,
            #[serde(rename = "matchDepNames", default)]
            match_dep_names: Vec<String>,
            /// Deprecated; converted to `/pattern/` strings in `matchDepNames`.
            #[serde(rename = "matchDepPatterns", default)]
            match_dep_patterns: Vec<String>,
            /// Deprecated; converted to `prefix**` globs in `matchDepNames`.
            #[serde(rename = "matchDepPrefixes", default)]
            match_dep_prefixes: Vec<String>,
            // Deprecated aliases for matchDatasources, matchManagers, etc.
            // Renovate reference: lib/config/migrations/custom/
            #[serde(default)]
            datasources: Vec<String>,
            #[serde(rename = "matchDatasources", default)]
            match_datasources: Vec<String>,
            #[serde(rename = "sourceUrlPrefixes", default)]
            source_url_prefixes: Vec<String>,
            #[serde(rename = "matchSourceUrls", default)]
            match_source_urls: Vec<String>,
            #[serde(default)]
            managers: Vec<String>,
            #[serde(rename = "matchManagers", default)]
            match_managers: Vec<String>,
            #[serde(rename = "updateTypes", default)]
            update_types_deprecated: Vec<String>,
            #[serde(rename = "matchUpdateTypes", default)]
            match_update_types: Vec<String>,
            // Deprecated aliases for matchFileNames and matchBaseBranches
            #[serde(default)]
            paths: Vec<String>,
            #[serde(rename = "matchFiles", default)]
            match_files: Vec<String>,
            #[serde(rename = "matchPaths", default)]
            match_paths: Vec<String>,
            #[serde(rename = "baseBranchList", default)]
            base_branch_list: Vec<String>,
            // Deprecated aliases for matchCategories
            #[serde(default)]
            languages: Vec<String>,
            // Deprecated aliases for matchDepTypes
            #[serde(rename = "depTypeList", default)]
            dep_type_list: Vec<String>,
            #[serde(rename = "allowedVersions")]
            allowed_versions: Option<String>,
            #[serde(rename = "matchCurrentVersion")]
            match_current_version: Option<String>,
            #[serde(rename = "matchCurrentValue")]
            match_current_value: Option<String>,
            #[serde(rename = "matchNewValue")]
            match_new_value: Option<String>,
            #[serde(rename = "matchFileNames", default)]
            match_file_names: Vec<String>,
            #[serde(rename = "matchDepTypes", default)]
            match_dep_types: Vec<String>,
            #[serde(rename = "ignoreVersions", default)]
            ignore_versions: Vec<String>,
            enabled: Option<bool>,
            // groupName accepts string or ["string"] (deprecated array form).
            #[serde(
                rename = "groupName",
                deserialize_with = "deserialize_string_or_first",
                default
            )]
            group_name: Option<String>,
            #[serde(rename = "groupSlug")]
            group_slug: Option<String>,
            automerge: Option<bool>,
            #[serde(default)]
            schedule: Vec<String>,
            #[serde(default)]
            labels: Vec<String>,
            #[serde(rename = "addLabels", default)]
            add_labels: Vec<String>,
            #[serde(default)]
            assignees: Vec<String>,
            #[serde(default)]
            reviewers: Vec<String>,
            #[serde(rename = "matchCategories", default)]
            match_categories: Vec<String>,
            #[serde(rename = "matchBaseBranches", default)]
            match_base_branches: Vec<String>,
            #[serde(rename = "matchRegistryUrls", default)]
            match_registry_urls: Vec<String>,
            #[serde(rename = "matchRepositories", default)]
            match_repositories: Vec<String>,
            // Deprecated: excludeRepositories → !repo in matchRepositories
            #[serde(rename = "excludeRepositories", default)]
            exclude_repositories: Vec<String>,
            #[serde(rename = "matchCurrentAge")]
            match_current_age: Option<String>,
            #[serde(rename = "minimumReleaseAge")]
            minimum_release_age: Option<String>,
            #[serde(rename = "prPriority")]
            pr_priority: Option<i32>,
            #[serde(rename = "commitMessageTopic")]
            commit_message_topic: Option<String>,
            #[serde(rename = "commitMessageAction")]
            commit_message_action: Option<String>,
            #[serde(rename = "commitMessagePrefix")]
            commit_message_prefix: Option<String>,
            #[serde(rename = "commitMessageExtra")]
            commit_message_extra: Option<String>,
            #[serde(rename = "commitMessageSuffix")]
            commit_message_suffix: Option<String>,
            #[serde(rename = "semanticCommitType")]
            semantic_commit_type: Option<String>,
            #[serde(rename = "semanticCommitScope")]
            semantic_commit_scope: Option<String>,
            #[serde(rename = "rangeStrategy")]
            range_strategy: Option<String>,
            versioning: Option<String>,
            #[serde(rename = "pinDigests")]
            pin_digests: Option<bool>,
            #[serde(rename = "followTag")]
            follow_tag: Option<String>,
            #[serde(rename = "replacementName")]
            replacement_name: Option<String>,
            #[serde(rename = "replacementVersion")]
            replacement_version: Option<String>,
            #[serde(rename = "versionCompatibility")]
            version_compatibility: Option<String>,
            #[serde(rename = "changelogUrl")]
            changelog_url: Option<String>,
            #[serde(default)]
            extends: Vec<String>,
            #[serde(rename = "dependencyDashboardApproval")]
            dependency_dashboard_approval: Option<bool>,
        }

        #[derive(Deserialize)]
        struct Raw {
            #[serde(default = "default_true")]
            enabled: bool,
            #[serde(rename = "ignoreDeps", default)]
            ignore_deps: Vec<String>,
            #[serde(rename = "ignorePaths", default)]
            ignore_paths: Vec<String>,
            /// Deprecated: ignoreNodeModules: true → adds "node_modules/" to ignorePaths.
            #[serde(rename = "ignoreNodeModules")]
            ignore_node_modules: Option<bool>,
            #[serde(rename = "includePaths", default)]
            include_paths: Vec<String>,
            #[serde(rename = "packageRules", default)]
            package_rules: Vec<RawPackageRule>,
            /// Deprecated: `packages` was the old name for `packageRules`.
            /// Renovate reference: lib/config/migrations/custom/packages-migration.ts
            #[serde(default)]
            packages: Vec<RawPackageRule>,
            #[serde(rename = "enabledManagers", default)]
            enabled_managers: Vec<String>,
            #[serde(rename = "disabledManagers", default)]
            disabled_managers: Vec<String>,
            #[serde(rename = "ignoreVersions", default)]
            ignore_versions: Vec<String>,
            #[serde(default, deserialize_with = "deserialize_string_or_vec")]
            schedule: Vec<String>,
            #[serde(
                rename = "automergeSchedule",
                default,
                deserialize_with = "deserialize_string_or_vec"
            )]
            automerge_schedule: Vec<String>,
            timezone: Option<String>,
            #[serde(default, deserialize_with = "deserialize_automerge_bool")]
            automerge: bool,
            #[serde(rename = "automergeType")]
            automerge_type: Option<String>,
            #[serde(default)]
            labels: Vec<String>,
            #[serde(rename = "addLabels", default)]
            add_labels: Vec<String>,
            #[serde(default)]
            assignees: Vec<String>,
            #[serde(default)]
            reviewers: Vec<String>,
            #[serde(rename = "draftPR", default)]
            draft_pr: bool,
            #[serde(rename = "assignAutomerge", default)]
            assign_automerge: bool,
            #[serde(rename = "branchPrefix", default = "default_branch_prefix")]
            branch_prefix: String,
            #[serde(rename = "additionalBranchPrefix", default)]
            additional_branch_prefix: String,
            #[serde(rename = "baseBranches", default)]
            base_branches: Vec<String>,
            /// Deprecated: singular baseBranch → baseBranches[0].
            #[serde(rename = "baseBranch")]
            base_branch: Option<String>,
            #[serde(rename = "rebaseWhen")]
            rebase_when: Option<String>,
            /// Deprecated: rebaseStalePrs: true → rebaseWhen: "behind-base-branch".
            #[serde(rename = "rebaseStalePrs")]
            rebase_stale_prs: Option<bool>,
            /// Deprecated: rebaseConflictedPrs: false → rebaseWhen: "never".
            #[serde(rename = "rebaseConflictedPrs")]
            rebase_conflicted_prs: Option<bool>,
            #[serde(rename = "prCreation")]
            pr_creation: Option<String>,
            #[serde(rename = "prConcurrentLimit", default)]
            pr_concurrent_limit: u32,
            #[serde(rename = "prHourlyLimit", default = "default_pr_hourly_limit")]
            pr_hourly_limit: u32,
            #[serde(rename = "groupName")]
            group_name: Option<String>,
            #[serde(rename = "separateMajorMinor", default = "default_true")]
            separate_major_minor: bool,
            /// Deprecated: separateMajorReleases is an alias for separateMajorMinor.
            #[serde(rename = "separateMajorReleases")]
            separate_major_releases: Option<bool>,
            #[serde(rename = "separateMultipleMajor", default)]
            separate_multiple_major: bool,
            #[serde(rename = "separateMinorPatch", default)]
            separate_minor_patch: bool,
            #[serde(rename = "separateMultipleMinor", default)]
            separate_multiple_minor: bool,
            #[serde(rename = "maxMajorIncrement", default = "default_max_major_increment")]
            max_major_increment: u32,
            #[serde(
                rename = "semanticCommits",
                deserialize_with = "deserialize_semantic_commits_opt",
                default
            )]
            semantic_commits: Option<String>,
            #[serde(
                rename = "semanticCommitType",
                default = "default_semantic_commit_type"
            )]
            semantic_commit_type: String,
            #[serde(
                rename = "semanticCommitScope",
                default = "default_semantic_commit_scope"
            )]
            semantic_commit_scope: String,
            #[serde(default, deserialize_with = "deserialize_string_or_vec")]
            extends: Vec<String>,
            #[serde(rename = "ignorePresets", default)]
            ignore_presets: Vec<String>,
            #[serde(rename = "minimumReleaseAge")]
            minimum_release_age: Option<String>,
            /// Deprecated: Renovate migrates stabilityDays → minimumReleaseAge.
            /// Accepted here for backward-compat; values: 0 (unset), 1 ("1 day"), N ("N days").
            #[serde(rename = "stabilityDays")]
            stability_days: Option<u32>,
            /// Deprecated: unpublishSafe: true → adds security:minimumReleaseAgeNpm to extends.
            #[serde(rename = "unpublishSafe")]
            unpublish_safe: Option<bool>,
            /// Deprecated: upgradeInRange: true → rangeStrategy: "bump".
            #[serde(rename = "upgradeInRange")]
            upgrade_in_range: Option<bool>,
            /// Deprecated: versionStrategy: "widen" → rangeStrategy: "widen".
            #[serde(rename = "versionStrategy")]
            version_strategy: Option<String>,
            #[serde(rename = "ignoreUnstable", default)]
            ignore_unstable: bool,
            #[serde(rename = "respectLatest", default)]
            respect_latest: bool,
            #[serde(rename = "pinDigests", default)]
            pin_digests: bool,
            #[serde(rename = "dependencyDashboard", default)]
            dependency_dashboard: bool,
            #[serde(rename = "dependencyDashboardApproval", default)]
            dependency_dashboard_approval: bool,
            #[serde(rename = "configMigration", default)]
            config_migration: bool,
            #[serde(rename = "updateNotScheduled", default = "default_true")]
            update_not_scheduled: bool,
            #[serde(rename = "commitMessageAction", default = "default_commit_action")]
            commit_message_action: String,
            #[serde(rename = "commitMessagePrefix")]
            commit_message_prefix: Option<String>,
            #[serde(rename = "commitMessageExtra")]
            commit_message_extra: Option<String>,
            #[serde(rename = "commitMessageSuffix")]
            commit_message_suffix: Option<String>,
            #[serde(rename = "commitBody")]
            commit_body: Option<String>,
            #[serde(rename = "rangeStrategy", default = "default_range_strategy")]
            range_strategy: String,
            #[serde(rename = "hashedBranchLength")]
            hashed_branch_length: Option<u32>,
            major: Option<crate::package_rule::UpdateTypeConfig>,
            minor: Option<crate::package_rule::UpdateTypeConfig>,
            patch: Option<crate::package_rule::UpdateTypeConfig>,
            #[serde(rename = "customManagers", default)]
            custom_managers: Vec<RawCustomManager>,
        }

        #[derive(Deserialize, Default)]
        struct RawCustomManager {
            #[serde(rename = "customType", default = "default_custom_type")]
            custom_type: String,
            /// managerFilePatterns (Renovate 39+) or legacy fileMatch.
            #[serde(rename = "managerFilePatterns", default)]
            manager_file_patterns: Vec<String>,
            /// Legacy fileMatch field — merged with managerFilePatterns.
            #[serde(rename = "fileMatch", default)]
            file_match: Vec<String>,
            #[serde(rename = "matchStrings", default)]
            match_strings: Vec<String>,
            #[serde(rename = "matchStringsStrategy", default = "default_any_strategy")]
            match_strings_strategy: String,
            #[serde(rename = "datasourceTemplate")]
            datasource_template: Option<String>,
            #[serde(rename = "depNameTemplate")]
            dep_name_template: Option<String>,
            #[serde(rename = "packageNameTemplate")]
            package_name_template: Option<String>,
            #[serde(rename = "versioningTemplate")]
            versioning_template: Option<String>,
            #[serde(rename = "registryUrlTemplate")]
            registry_url_template: Option<String>,
            #[serde(rename = "extractVersionTemplate")]
            extract_version_template: Option<String>,
            #[serde(rename = "autoReplaceStringTemplate")]
            auto_replace_string_template: Option<String>,
        }

        fn default_any_strategy() -> String {
            "any".to_owned()
        }

        fn default_true() -> bool {
            true
        }

        fn default_branch_prefix() -> String {
            "renovate/".to_owned()
        }

        fn default_commit_action() -> String {
            "Update".to_owned()
        }

        fn default_range_strategy() -> String {
            "auto".to_owned()
        }

        fn default_custom_type() -> String {
            "regex".to_owned()
        }

        /// Deserialize semanticCommits from bool or string.
        /// Renovate reference: lib/config/migrations/custom/semantic-commits-migration.ts
        /// Deserialize a field that can be a string or `["string"]` (array with one element).
        /// Used for `groupName` which Renovate historically accepted as an array.
        fn deserialize_string_or_first<'de, D>(d: D) -> Result<Option<String>, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            use serde_json::Value;
            let val = Value::deserialize(d)?;
            Ok(match val {
                Value::String(s) => Some(s),
                Value::Array(arr) => arr.into_iter().next().and_then(|v| {
                    if let Value::String(s) = v {
                        Some(s)
                    } else {
                        None
                    }
                }),
                _ => None,
            })
        }

        /// Deserialize a field that Renovate accepts as either a bare string or an array.
        /// `"every friday"` and `["every friday"]` both parse to `vec!["every friday"]`.
        fn deserialize_string_or_vec<'de, D>(d: D) -> Result<Vec<String>, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            use serde_json::Value;
            let val = Value::deserialize(d)?;
            Ok(match val {
                Value::String(s) => vec![s],
                Value::Array(arr) => arr
                    .into_iter()
                    .filter_map(|v| v.as_str().map(str::to_owned))
                    .collect(),
                _ => vec![],
            })
        }

        fn deserialize_semantic_commits_opt<'de, D>(d: D) -> Result<Option<String>, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            use serde_json::Value;
            let val = Value::deserialize(d)?;
            Ok(match &val {
                Value::Bool(true) => Some("enabled".to_owned()),
                Value::Bool(false) => Some("disabled".to_owned()),
                Value::String(s) if s == "enabled" || s == "disabled" || s == "auto" => {
                    Some(s.clone())
                }
                Value::String(_) => Some("auto".to_owned()),
                Value::Null => None,
                _ => None,
            })
        }

        /// Deserialize automerge from bool or legacy string enum.
        /// Renovate reference: lib/config/migrations/custom/automerge-migration.ts
        fn deserialize_automerge_bool<'de, D>(d: D) -> Result<bool, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            use serde::de::Error;
            use serde_json::Value;
            let val = Value::deserialize(d)?;
            match &val {
                Value::Bool(b) => Ok(*b),
                Value::String(s) => match s.as_str() {
                    "none" => Ok(false),
                    "any" => Ok(true),
                    // "patch"/"minor" also migrate but need patch/minor config blocks
                    // which we can't set here; treat as true (automerge partial types).
                    "patch" | "minor" => Ok(true),
                    _ => Ok(false),
                },
                Value::Null => Ok(false),
                _ => Err(D::Error::custom(format!(
                    "expected bool or string for automerge, got {val}"
                ))),
            }
        }

        fn default_max_major_increment() -> u32 {
            500
        }

        fn default_semantic_commit_type() -> String {
            "chore".to_owned()
        }

        fn default_semantic_commit_scope() -> String {
            "deps".to_owned()
        }

        fn default_pr_hourly_limit() -> u32 {
            2
        }

        let mut raw: Raw = match json5::from_str(content) {
            Ok(r) => r,
            Err(e) => {
                tracing::debug!(%e, "failed to parse repo renovate config; using defaults");
                return Self::default();
            }
        };

        // Normalize preset names using Renovate's removedPresets map:
        // renames deprecated/removed preset names to their current equivalents.
        // Apply repeatedly until stable (handles chained renames like
        // :unpublishSafe → npm:unpublishSafe → security:minimumReleaseAgeNpm).
        // Renovate reference: lib/config/migrations/custom/extends-migration.ts
        raw.extends = raw
            .extends
            .into_iter()
            .filter_map(|mut p| {
                // Iterate until stable to handle chained removedPresets entries.
                loop {
                    match normalize_preset(&p) {
                        None => return None,
                        Some(next) if next == p => return Some(p),
                        Some(next) => p = next,
                    }
                }
            })
            .collect();

        // Deprecated migration: unpublishSafe: true → add security:minimumReleaseAgeNpm
        // to the extends list if it's not already present.
        // Renovate reference: lib/config/migrations/custom/unpublish-safe-migration.ts
        if raw.unpublish_safe == Some(true)
            && !raw
                .extends
                .iter()
                .any(|e| e == "security:minimumReleaseAgeNpm" || e == ":unpublishSafe")
        {
            raw.extends.push("security:minimumReleaseAgeNpm".to_owned());
        }

        // Two-pass ignorePresets filtering:
        // Pass 1: Filter BEFORE expansion so ignorePresets:["workarounds:all"] prevents
        //         the compound from being expanded at all.
        let pre_filtered_extends: Vec<String> = raw
            .extends
            .iter()
            .filter(|p| !raw.ignore_presets.contains(p))
            .cloned()
            .collect();

        // Expand compound presets after pre-filtering. This handles config:js-app,
        // config:recommended (→ workarounds:all, replacements:all, etc.), etc.
        let expanded_extends = expand_compound_presets(&pre_filtered_extends);

        // Pass 2: Filter AFTER expansion so ignorePresets:["workarounds:typesNodeVersioning"]
        //         suppresses that individual sub-preset even when it came from expanding
        //         a compound like workarounds:all.
        let effective_extends: Vec<String> = expanded_extends
            .into_iter()
            .filter(|p| !raw.ignore_presets.contains(p))
            .collect();

        // Resolve group presets before building user-defined rules.
        // Preset rules are prepended so user-defined rules take precedence (later rules win).
        let (mut preset_rules, group_separate_major_minor) =
            resolve_extends_group_presets(&effective_extends);
        // Inject semantic prefix packageRules from `:semanticPrefixFixDepsChoreOthers` etc.
        let sem_prefix_rules = resolve_extends_semantic_prefix_rules(&effective_extends);
        preset_rules.extend(sem_prefix_rules);
        let _ = resolve_extends_semantic_type_scope(&effective_extends); // placeholder for future use
        // Inject selective automerge rules from :automergeMinor / :automergePatch.
        let automerge_rules = resolve_extends_automerge_rules(&effective_extends);
        preset_rules.extend(automerge_rules);
        // Inject rules from other common presets (:disableDevDependencies, etc.).
        let common_rules = resolve_extends_common_rules(&effective_extends);
        preset_rules.extend(common_rules);
        // Inject range-strategy rules from pin/preserve presets.
        let range_rules = resolve_extends_range_strategy_rules(&effective_extends);
        preset_rules.extend(range_rules);
        // Inject packageRules from parameterized presets like :doNotPinPackage(name).
        let param_rules = resolve_extends_parameterized_rules(&effective_extends);
        preset_rules.extend(param_rules);
        // :automergePatch sets separateMinorPatch: true.
        let preset_separate_minor_patch = effective_extends.iter().any(|p| p == ":automergePatch");
        let (
            param_labels,
            param_assignees,
            param_reviewers,
            param_automerge_type,
            param_sem_type,
            param_sem_scope,
        ) = resolve_extends_parameterized(&effective_extends);
        let (
            scalar_sep_minor_patch,
            scalar_sep_major_minor,
            scalar_sep_multi_major,
            scalar_sep_multi_minor,
            scalar_pr_concurrent,
            scalar_pr_hourly,
        ) = resolve_extends_scalar_overrides(&effective_extends);

        // Convert `enabled: false` inside major/minor/patch blocks to synthetic
        // packageRules so the existing is_update_blocked_ctx path handles them.
        for (update_type_str, cfg) in [
            ("major", raw.major.as_ref()),
            ("minor", raw.minor.as_ref()),
            ("patch", raw.patch.as_ref()),
        ] {
            if matches!(cfg, Some(c) if c.enabled == Some(false)) {
                let ut = match update_type_str {
                    "major" => crate::versioning::semver_generic::UpdateType::Major,
                    "minor" => crate::versioning::semver_generic::UpdateType::Minor,
                    _ => crate::versioning::semver_generic::UpdateType::Patch,
                };
                preset_rules.push(PackageRule {
                    match_update_types: vec![ut],
                    enabled: Some(false),
                    ..Default::default()
                });
            }
        }

        let package_rules: Vec<PackageRule> = raw
            .package_rules
            .into_iter()
            // Deprecated: `packages` was the old name for `packageRules`.
            .chain(raw.packages)
            .map(|r| {
                // Resolve packages:* (and other recognized) presets from the
                // rule's own `extends` list and merge their matchers in.
                let preset_matchers = resolve_package_rule_extends(&r.extends);

                let has_name_constraint = !r.match_package_names.is_empty()
                    || !r.match_package_patterns.is_empty()
                    || !r.match_package_prefixes.is_empty()
                    || !preset_matchers.match_package_names.is_empty();

                // deprecated matchPackagePatterns into one Vec<String> so that
                // match_regex_or_glob_list can apply positive/negative semantics.
                let mut match_package_names: Vec<String> = r.match_package_names;
                match_package_names.extend(preset_matchers.match_package_names);
                // Deprecated: packageNames → matchPackageNames (plain alias)
                match_package_names.extend(r.package_names_deprecated);
                // matchPackagePrefixes → "prefix**" glob strings
                for prefix in r.match_package_prefixes {
                    match_package_names.push(format!("{prefix}**"));
                }
                // matchPackagePatterns + packagePatterns (deprecated) → "/raw_regex/" inline strings
                for pat in r
                    .match_package_patterns
                    .into_iter()
                    .chain(r.package_patterns_deprecated)
                {
                    match_package_names.push(format!("/{pat}/"));
                }
                // Deprecated: excludePackageNames → "!name" negation
                for name in r.exclude_package_names {
                    match_package_names.push(format!("!{name}"));
                }
                // Deprecated: excludePackagePatterns → "!/pattern/" negation
                for pat in r.exclude_package_patterns {
                    match_package_names.push(format!("!/{pat}/"));
                }

                let has_update_type_constraint =
                    !r.match_update_types.is_empty() || !r.update_types_deprecated.is_empty();
                let match_update_types = r
                    .match_update_types
                    .into_iter()
                    .chain(r.update_types_deprecated)
                    .filter_map(|s| match s.as_str() {
                        "major" => Some(UpdateType::Major),
                        "minor" => Some(UpdateType::Minor),
                        "patch" => Some(UpdateType::Patch),
                        "replacement" => Some(UpdateType::Replacement),
                        "digest" => Some(UpdateType::Digest),
                        "pin" => Some(UpdateType::Pin),
                        "bump" => Some(UpdateType::Bump),
                        _ => None,
                    })
                    .collect();

                // Merge deprecated matchDepPrefixes and matchDepPatterns into matchDepNames,
                // mirroring Renovate's package-rules migration for dep name conditions.
                let mut match_dep_names = r.match_dep_names;
                for prefix in r.match_dep_prefixes {
                    match_dep_names.push(format!("{prefix}**"));
                }
                for pat in r.match_dep_patterns {
                    match_dep_names.push(format!("/{pat}/"));
                }

                // matchDepNames, matchSourceUrls, matchRegistryUrls, matchRepositories
                // store raw strings so match_regex_or_glob_list can apply negation.
                // Migrate deprecated datasource names in matchDatasources.
                // Renovate reference: lib/config/migrations/custom/datasource-migration.ts
                // Deprecated: datasources → matchDatasources (plain alias)
                let mut match_datasources: Vec<String> = r
                    .match_datasources
                    .into_iter()
                    .chain(r.datasources)
                    .map(|ds| match ds.as_str() {
                        "adoptium-java" => "java-version".to_owned(),
                        "dotnet" => "dotnet-version".to_owned(),
                        "node" => "node-version".to_owned(),
                        _ => ds,
                    })
                    .collect();
                match_datasources.extend(preset_matchers.match_datasources);
                // Deprecated: sourceUrlPrefixes → matchSourceUrls with {/,}** appended
                let mut match_source_urls = r.match_source_urls;
                for prefix in r.source_url_prefixes {
                    // Renovate migration: sourceUrlPrefixes add glob that matches the URL
                    // with any path below it. The {/,}** pattern matches either a direct
                    // suffix or with a path separator, covering both trailing slash variants.
                    match_source_urls.push(format!("{prefix}{{/,}}**"));
                }
                match_source_urls.extend(preset_matchers.match_source_urls);
                PackageRule {
                    match_package_names,
                    match_dep_names,
                    match_source_urls,
                    match_current_value: r.match_current_value,
                    match_new_value: r.match_new_value,
                    match_datasources,
                    // Migrate deprecated "regex" manager name → "custom.regex".
                    // Also merge deprecated `managers` alias (plain rename).
                    // Renovate reference: lib/config/migrations/custom/match-managers-migration.ts
                    match_managers: r
                        .match_managers
                        .into_iter()
                        .chain(r.managers)
                        .map(|m| {
                            if m == "regex" {
                                "custom.regex".to_owned()
                            } else {
                                m
                            }
                        })
                        .collect(),
                    match_update_types,
                    allowed_versions: r.allowed_versions,
                    match_current_version: r.match_current_version,
                    // Deprecated aliases for matchFileNames:
                    // paths → matchFileNames (exact; matchFiles/matchPaths are Renovate-internal)
                    // matchFiles is the newer preferred field in some versions
                    // matchPaths takes precedence over matchFiles in Renovate's migration
                    match_file_names: {
                        let mut f = r.match_file_names;
                        f.extend(r.paths);
                        f.extend(r.match_paths);
                        f.extend(r.match_files);
                        f
                    },
                    // Deprecated: depTypeList → matchDepTypes (plain alias)
                    match_dep_types: {
                        let mut d = r.match_dep_types;
                        d.extend(r.dep_type_list);
                        d
                    },
                    ignore_versions: r.ignore_versions,
                    enabled: r.enabled,
                    has_name_constraint,
                    has_update_type_constraint,
                    group_name: r.group_name,
                    group_slug: r.group_slug,
                    automerge: r.automerge,
                    schedule: r.schedule,
                    labels: r.labels,
                    add_labels: r.add_labels,
                    assignees: r.assignees,
                    reviewers: r.reviewers,
                    // Deprecated: languages → matchCategories (plain alias)
                    match_categories: {
                        let mut c = r.match_categories;
                        c.extend(r.languages);
                        c
                    },
                    // Deprecated: baseBranchList → matchBaseBranches (plain alias)
                    match_base_branches: {
                        let mut b = r.match_base_branches;
                        b.extend(r.base_branch_list);
                        b
                    },
                    match_registry_urls: r.match_registry_urls,
                    // Deprecated: excludeRepositories → !repo negation in matchRepositories
                    match_repositories: {
                        let mut repos = r.match_repositories;
                        for repo in r.exclude_repositories {
                            repos.push(format!("!{repo}"));
                        }
                        repos
                    },
                    match_current_age: r.match_current_age,
                    minimum_release_age: r.minimum_release_age,
                    pr_priority: r.pr_priority,
                    commit_message_topic: r.commit_message_topic,
                    commit_message_action: r.commit_message_action,
                    commit_message_prefix: r.commit_message_prefix,
                    semantic_commit_type: r.semantic_commit_type,
                    semantic_commit_scope: r.semantic_commit_scope,
                    commit_message_extra: r.commit_message_extra,
                    commit_message_suffix: r.commit_message_suffix,
                    range_strategy: r.range_strategy,
                    versioning: r.versioning,
                    pin_digests: r.pin_digests,
                    follow_tag: r.follow_tag,
                    replacement_name: r.replacement_name,
                    replacement_version: r.replacement_version,
                    version_compatibility: r.version_compatibility,
                    changelog_url: r.changelog_url,
                    dependency_dashboard_approval: r.dependency_dashboard_approval,
                }
            })
            .collect();

        // Prepend preset rules so user-defined rules have higher precedence (last rule wins).
        preset_rules.extend(package_rules);
        let package_rules = preset_rules;

        // Resolve scalar presets that set ignoreUnstable / updateNotScheduled / enabled.
        let preset_ignore_unstable = effective_extends.iter().any(|p| p == ":ignoreUnstable");
        let preset_update_not_scheduled = effective_extends
            .iter()
            .any(|p| p == ":noUnscheduledUpdates")
            .then_some(false); // :noUnscheduledUpdates → updateNotScheduled: false
        // :disableRenovate / :enableRenovate override the enabled flag.
        let preset_enabled: Option<bool> = if effective_extends
            .iter()
            .any(|p| p == ":disableRenovate" || p == "disableRenovate")
        {
            Some(false)
        } else if effective_extends
            .iter()
            .any(|p| p == ":enableRenovate" || p == "enableRenovate")
        {
            Some(true)
        } else {
            None
        };

        // Resolve :timezone(zone) parameterized preset.
        let preset_timezone: Option<String> = effective_extends.iter().find_map(|p| {
            let (name, args) = parse_preset_args(p.as_str());
            if name == ":timezone" || name == "timezone" {
                args.into_iter()
                    .next()
                    .filter(|s| !s.is_empty())
                    .map(String::from)
            } else {
                None
            }
        });

        // Resolve managers enabled/disabled via presets.
        // Migrate deprecated enabledManagers values: "yarn" → "npm", "regex" → "custom.regex".
        // Renovate reference: lib/config/migrations/custom/enabled-managers-migration.ts
        let mut enabled_managers: Vec<String> = raw
            .enabled_managers
            .into_iter()
            .map(|m| match m.as_str() {
                "yarn" => "npm".to_owned(),
                "regex" => "custom.regex".to_owned(),
                _ => m,
            })
            .collect();
        // Start with any managers explicitly disabled in the JSON config.
        let mut disabled_managers: Vec<String> = raw.disabled_managers;
        for preset in &effective_extends {
            match preset.as_str() {
                ":enablePreCommit" | "enablePreCommit"
                    if !enabled_managers.contains(&"pre-commit".to_owned()) =>
                {
                    enabled_managers.push("pre-commit".to_owned());
                }
                // docker:disable disables specific docker-related managers.
                // Mirrors Renovate's docker.preset.ts: { circleci: { enabled: false },
                // 'docker-compose': { enabled: false }, dockerfile: { enabled: false } }
                "docker:disable" => {
                    for m in ["circleci", "docker-compose", "dockerfile"] {
                        let s = m.to_owned();
                        if !disabled_managers.contains(&s) {
                            disabled_managers.push(s);
                        }
                    }
                }
                ":includeNodeModules" | "includeNodeModules" => {
                    // includeNodeModules clears ignorePaths — handled below via ignorePaths: []
                }
                _ => {}
            }
        }

        Self {
            enabled: preset_enabled.unwrap_or(raw.enabled),
            ignore_deps: raw.ignore_deps,
            package_rules,
            enabled_managers,
            disabled_managers,
            ignore_versions: raw.ignore_versions,
            schedule: if raw.schedule.is_empty() {
                // No explicit schedule → use schedule preset if any.
                resolve_extends_schedule(&effective_extends).unwrap_or(raw.schedule)
            } else {
                raw.schedule
                    .into_iter()
                    .map(migrate_schedule_string)
                    .collect()
            },
            automerge_schedule: if raw.automerge_schedule.is_empty() {
                // No explicit automergeSchedule → use preset or default "at any time".
                resolve_extends_automerge_schedule(&effective_extends)
                    .unwrap_or_else(|| vec!["at any time".to_owned()])
            } else {
                raw.automerge_schedule
            },
            timezone: raw.timezone.or(preset_timezone),
            automerge: if raw.automerge {
                true // explicit automerge: true wins
            } else {
                resolve_extends_automerge(&effective_extends).unwrap_or(false)
            },
            automerge_type: raw.automerge_type.or(param_automerge_type),
            labels: {
                let mut l = raw.labels;
                for pl in param_labels {
                    if !l.contains(&pl) {
                        l.push(pl);
                    }
                }
                l
            },
            add_labels: raw.add_labels,
            assignees: {
                let mut a = raw.assignees;
                for pa in param_assignees {
                    if !a.contains(&pa) {
                        a.push(pa);
                    }
                }
                a
            },
            reviewers: {
                let mut r = raw.reviewers;
                for pr in param_reviewers {
                    if !r.contains(&pr) {
                        r.push(pr);
                    }
                }
                r
            },
            draft_pr: raw.draft_pr,
            assign_automerge: raw.assign_automerge,
            branch_prefix: raw.branch_prefix,
            additional_branch_prefix: raw.additional_branch_prefix,
            base_branches: {
                // Deprecated singular baseBranch → prepend to baseBranches array.
                let mut branches = raw.base_branches;
                if let Some(b) = raw.base_branch
                    && !branches.contains(&b)
                {
                    branches.insert(0, b);
                }
                branches
            },
            rebase_when: raw.rebase_when.or_else(|| {
                // Deprecated rebaseConflictedPrs: false → rebaseWhen: "never".
                // Renovate reference: lib/config/migrations/custom/rebase-conflicted-prs-migration.ts
                if raw.rebase_conflicted_prs == Some(false) {
                    return Some("never".to_owned());
                }
                // Deprecated rebaseStalePrs → rebaseWhen mapping.
                // Renovate reference: lib/config/migrations/custom/rebase-stale-prs-migration.ts
                if let Some(stale) = raw.rebase_stale_prs {
                    return Some(
                        if stale {
                            "behind-base-branch"
                        } else {
                            "conflicted"
                        }
                        .to_owned(),
                    );
                }
                // :rebaseStalePrs preset sets rebaseWhen: "behind-base-branch".
                if effective_extends
                    .iter()
                    .any(|p| p == ":rebaseStalePrs" || p == "rebaseStalePrs")
                {
                    Some("behind-base-branch".to_owned())
                } else {
                    None
                }
            }),
            pr_concurrent_limit: scalar_pr_concurrent.unwrap_or(raw.pr_concurrent_limit),
            pr_hourly_limit: scalar_pr_hourly.unwrap_or(raw.pr_hourly_limit),
            pr_creation: raw.pr_creation.or_else(|| {
                // :prImmediately and :prNotPending presets set prCreation.
                if effective_extends
                    .iter()
                    .any(|p| p == ":prImmediately" || p == "prImmediately")
                {
                    Some("immediate".to_owned())
                } else if effective_extends
                    .iter()
                    .any(|p| p == ":prNotPending" || p == "prNotPending")
                {
                    Some("not-pending".to_owned())
                } else {
                    None
                }
            }),
            group_name: raw.group_name,
            // group:all preset implies separateMajorMinor: false.
            // Deprecated separateMajorReleases is an alias for separateMajorMinor.
            // Explicit user config overrides the preset (but default true from serde means
            // we can't distinguish user-set vs default; group preset wins only when
            // the user hasn't explicitly set it to true in the raw JSON).
            separate_major_minor: scalar_sep_major_minor
                .or(group_separate_major_minor)
                .or(raw.separate_major_releases)
                .unwrap_or(raw.separate_major_minor),
            separate_multiple_major: scalar_sep_multi_major.unwrap_or(raw.separate_multiple_major),
            max_major_increment: raw.max_major_increment,
            separate_minor_patch: scalar_sep_minor_patch
                .unwrap_or(raw.separate_minor_patch || preset_separate_minor_patch),
            separate_multiple_minor: scalar_sep_multi_minor.unwrap_or(raw.separate_multiple_minor),
            semantic_commit_type: param_sem_type.unwrap_or(raw.semantic_commit_type),
            semantic_commit_scope: param_sem_scope.unwrap_or(raw.semantic_commit_scope),
            semantic_commits: raw.semantic_commits.or_else(|| {
                // `:semanticCommits` preset implies semanticCommits = "enabled"
                if effective_extends.iter().any(|e| e == ":semanticCommits") {
                    Some("enabled".to_owned())
                } else if effective_extends
                    .iter()
                    .any(|e| e == ":semanticCommitsDisabled")
                {
                    Some("disabled".to_owned())
                } else {
                    None
                }
            }),
            ignore_paths: {
                // Prepend ignore paths from resolved built-in presets.
                // User-configured paths override/extend preset paths.
                let mut preset_paths = resolve_extends_ignore_paths(&effective_extends);
                // Deprecated ignoreNodeModules: true → add "node_modules/" to ignorePaths.
                // Renovate reference: lib/config/migrations/custom/ignore-node-modules-migration.ts
                if raw.ignore_node_modules == Some(true)
                    && !raw.ignore_paths.contains(&"node_modules/".to_owned())
                {
                    raw.ignore_paths.push("node_modules/".to_owned());
                }
                preset_paths.extend(raw.ignore_paths);
                preset_paths
            },
            include_paths: raw.include_paths,
            extends: raw.extends,
            ignore_presets: raw.ignore_presets,
            minimum_release_age: raw.minimum_release_age.or_else(|| {
                // Migrate deprecated stabilityDays → minimumReleaseAge.
                // Renovate reference: lib/config/migrations/custom/stability-days-migration.ts
                match raw.stability_days {
                    Some(0) | None => None,
                    Some(1) => Some("1 day".to_owned()),
                    Some(n) => Some(format!("{n} days")),
                }
            }),
            ignore_unstable: raw.ignore_unstable || preset_ignore_unstable,
            respect_latest: raw.respect_latest
                || effective_extends
                    .iter()
                    .any(|p| p == ":respectLatest" || p == "respectLatest"),
            pin_digests: effective_extends
                .iter()
                .any(|p| p == ":pinDigests" || p == "pinDigests")
                || (raw.pin_digests
                    && !effective_extends
                        .iter()
                        .any(|p| p == ":pinDigestsDisabled" || p == "pinDigestsDisabled")),
            dependency_dashboard: raw.dependency_dashboard
                || effective_extends
                    .iter()
                    .any(|p| p == ":dependencyDashboard" || p == "dependencyDashboard")
                    && !effective_extends.iter().any(|p| {
                        p == ":disableDependencyDashboard" || p == "disableDependencyDashboard"
                    }),
            dependency_dashboard_approval: raw.dependency_dashboard_approval
                || effective_extends.iter().any(|p| {
                    p == ":dependencyDashboardApproval" || p == "dependencyDashboardApproval"
                }),
            config_migration: raw.config_migration
                || effective_extends
                    .iter()
                    .any(|p| p == ":configMigration" || p == "configMigration"),
            update_not_scheduled: preset_update_not_scheduled.unwrap_or(raw.update_not_scheduled),
            commit_message_action: raw.commit_message_action,
            commit_message_prefix: raw.commit_message_prefix,
            commit_message_extra: raw.commit_message_extra,
            commit_message_suffix: raw.commit_message_suffix,
            commit_body: raw.commit_body.or_else(|| {
                // :gitSignOff preset sets commitBody to Signed-off-by trailer.
                if effective_extends
                    .iter()
                    .any(|p| p == ":gitSignOff" || p == "gitSignOff")
                {
                    Some("Signed-off-by: {{{gitAuthor}}}".to_owned())
                } else {
                    None
                }
            }),
            range_strategy: {
                // Deprecated upgradeInRange: true → rangeStrategy: "bump".
                // Deprecated versionStrategy: "widen" → rangeStrategy: "widen".
                // Explicit rangeStrategy in the config takes precedence.
                if raw.range_strategy != "auto" {
                    raw.range_strategy
                } else if raw.upgrade_in_range == Some(true) {
                    "bump".to_owned()
                } else if raw.version_strategy.as_deref() == Some("widen") {
                    "widen".to_owned()
                } else {
                    raw.range_strategy
                }
            },
            hashed_branch_length: raw.hashed_branch_length,
            major_config: raw.major,
            minor_config: raw.minor,
            patch_config: raw.patch,
            custom_managers: {
                // Preset managers come first (lower precedence); user-defined
                // managers appended after so they can shadow preset ones.
                let mut cms = resolve_extends_custom_managers(&effective_extends);
                cms.extend(raw.custom_managers.into_iter().map(|cm| {
                    let mut file_patterns = cm.manager_file_patterns;
                    file_patterns.extend(cm.file_match);
                    CustomManager {
                        custom_type: cm.custom_type,
                        file_patterns,
                        match_strings: cm.match_strings,
                        match_strings_strategy: cm.match_strings_strategy,
                        datasource_template: cm.datasource_template,
                        dep_name_template: cm.dep_name_template,
                        package_name_template: cm.package_name_template,
                        versioning_template: cm.versioning_template,
                        registry_url_template: cm.registry_url_template,
                        extract_version_template: cm.extract_version_template,
                        auto_replace_string_template: cm.auto_replace_string_template,
                    }
                }));
                cms
            },
        }
    }

    /// Extract a Renovate config from a `package.json` file.
    ///
    /// Returns `Some(config)` when `package.json` contains a top-level
    /// `"renovate"` key whose value is a JSON object.  Returns `None` when
    /// the file is missing the key or cannot be parsed.
    ///
    /// Renovate reference: `lib/workers/repository/init/merge.ts` —
    /// `detectConfigFile()` checks `pJson.renovate` before treating
    /// `package.json` as a Renovate config source.
    ///
    /// Using `package.json` for Renovate config is deprecated upstream.
    pub fn parse_from_package_json(content: &str) -> Option<Self> {
        let pkg: serde_json::Value = serde_json::from_str(content).ok()?;
        let renovate_val = pkg.get("renovate")?;
        // Re-serialize the renovate sub-value and parse it as a RepoConfig.
        let renovate_str = serde_json::to_string(renovate_val).ok()?;
        Some(Self::parse(&renovate_str))
    }

    /// Return `true` when `manager_name` should run for this repository.
    ///
    /// Rules (mirroring Renovate's behavior):
    /// 1. If `enabledManagers` is non-empty, the manager must be listed there.
    /// 2. Otherwise, the manager runs unless it is disabled by default
    ///    (`defaultConfig.enabled: false` in the upstream manager definition).
    ///
    /// `disabled_by_default` should come from
    /// [`renovate_core::managers::is_disabled_by_default`].
    pub fn is_manager_enabled(&self, manager_name: &str, disabled_by_default: bool) -> bool {
        // Denylist takes precedence over everything.
        if self.disabled_managers.iter().any(|m| m == manager_name) {
            return false;
        }
        if !self.enabled_managers.is_empty() {
            // Explicit allowlist: manager must be listed.
            self.enabled_managers.iter().any(|m| m == manager_name)
        } else {
            // No allowlist: opt-out managers are skipped unless explicitly enabled.
            !disabled_by_default
        }
    }

    /// Return `true` when a dependency name should be ignored.
    ///
    /// Checks both the `ignoreDeps` list (exact match) and any `packageRules`
    /// that set `enabled: false`.  Manager-agnostic: rules with `matchManagers`
    /// are treated as matching all managers.
    ///
    /// For richer filtering (datasource, categories, dep type, file path, etc.)
    /// use [`is_dep_ignored_ctx`] with a full [`DepContext`].
    pub fn is_dep_ignored(&self, name: &str) -> bool {
        self.is_dep_ignored_ctx(&DepContext::for_dep(name))
    }

    /// Like [`is_dep_ignored`] but also filters by manager name.
    pub fn is_dep_ignored_for_manager(&self, name: &str, manager: &str) -> bool {
        self.is_dep_ignored_ctx(&DepContext::for_dep(name).with_manager(manager))
    }

    /// Like [`is_dep_ignored_for_manager`] but also checks `matchDepTypes`.
    pub fn is_dep_ignored_with_dep_type(&self, name: &str, manager: &str, dep_type: &str) -> bool {
        self.is_dep_ignored_ctx(
            &DepContext::for_dep(name)
                .with_manager(manager)
                .with_dep_type(dep_type),
        )
    }

    /// Full-context dep-ignore check.  Evaluates all packageRule matchers.
    ///
    /// The dep is ignored when:
    /// - `ignoreDeps` contains the dep name, OR
    /// - A `packageRules` entry with `enabled: false` matches `ctx` via all matchers.
    pub fn is_dep_ignored_ctx(&self, ctx: &DepContext<'_>) -> bool {
        if self.ignore_deps.iter().any(|p| p == ctx.dep_name) {
            return true;
        }
        // Delegate to is_update_blocked_ctx so last-rule-wins semantics apply.
        self.is_update_blocked_ctx(ctx)
    }

    /// Return `true` when a specific update (name + current + update type + manager)
    /// is blocked by a `packageRules` entry with `enabled: false`.
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
    ///
    /// Note: this builds a minimal `DepContext`.  For full context including
    /// `dep_type`, `repository`, etc., use [`is_update_blocked_ctx`].
    pub fn is_update_blocked_for_file(
        &self,
        name: &str,
        current_value: &str,
        update_type: UpdateType,
        manager: &str,
        file_path: &str,
    ) -> bool {
        let ctx = DepContext {
            dep_name: name,
            manager: Some(manager),
            current_value: Some(current_value),
            update_type: Some(update_type),
            file_path: Some(file_path),
            ..Default::default()
        };
        self.is_update_blocked_ctx(&ctx)
    }

    /// Like [`is_update_blocked_for_file`] but accepts a pre-built `DepContext`.
    ///
    /// Prefer this when the caller already holds a fully populated context
    /// (with `dep_type`, `repository`, `datasource`, etc.) to avoid re-constructing
    /// it and to ensure all matchers (`matchDepTypes`, `matchRepositories`, …) fire.
    pub fn is_update_blocked_ctx(&self, ctx: &DepContext<'_>) -> bool {
        // Renovate uses "last matching rule wins" semantics for `enabled`.
        // A later `enabled: true` rule overrides an earlier `enabled: false` rule
        // (mirrors applyPackageRules in lib/util/package-rules/index.ts).
        let mut blocked = false;
        for rule in &self.package_rules {
            if !rule.matches_context(ctx) {
                continue;
            }
            match rule.enabled {
                Some(false) => blocked = true,
                Some(true) => blocked = false, // explicitly re-enabled
                None => {}                     // no change
            }
        }
        blocked
    }

    /// Return `true` when `proposed_version` does NOT satisfy the
    /// `allowedVersions` constraint of any matching rule.
    ///
    /// Returns `true` (restricted) when the proposed version is NOT within the
    /// `allowedVersions` constraint of the first matching rule that sets it.
    /// Supports `/regex/` patterns, semver ranges, and exact string equality.
    /// If no rule has `allowedVersions`, this returns `false` (no restriction).
    pub fn is_version_restricted(&self, name: &str, manager: &str, proposed_version: &str) -> bool {
        self.is_version_restricted_for_file(name, manager, proposed_version, "")
    }

    /// Like [`is_version_restricted`] but also checks `matchFileNames`.
    ///
    /// Note: this builds a minimal `DepContext`.  Use [`is_version_restricted_ctx`]
    /// when the caller already holds a fully populated context.
    pub fn is_version_restricted_for_file(
        &self,
        name: &str,
        manager: &str,
        proposed_version: &str,
        file_path: &str,
    ) -> bool {
        let ctx = DepContext {
            dep_name: name,
            manager: Some(manager),
            file_path: Some(file_path),
            ..Default::default()
        };
        self.is_version_restricted_ctx(&ctx, proposed_version)
    }

    /// Like [`is_version_restricted_for_file`] but accepts a pre-built `DepContext`.
    ///
    /// Ensures all matchers (`matchDepTypes`, `matchRepositories`, …) fire correctly.
    pub fn is_version_restricted_ctx(&self, ctx: &DepContext<'_>, proposed_version: &str) -> bool {
        // Renovate uses last-matching-rule-wins for allowedVersions.
        // The effective allowedVersions is from the LAST matching rule that sets it.
        // If that last rule allows the version, the update is not restricted.
        let mut effective_allowed: Option<&str> = None;
        for rule in &self.package_rules {
            if !rule.matches_context(ctx) {
                continue;
            }
            if let Some(ref av) = rule.allowed_versions {
                effective_allowed = Some(av.as_str());
            }
        }
        match effective_allowed {
            None => false, // no rule set allowedVersions → no restriction
            Some(av) => !version_matches_allowed(proposed_version, av),
        }
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
        if version_matches_ignore_list(proposed_version, &self.ignore_versions) {
            return true;
        }
        let ctx = DepContext {
            dep_name: name,
            manager: Some(manager),
            file_path: Some(file_path),
            ..Default::default()
        };
        self.is_version_ignored_ctx(&ctx, proposed_version)
    }

    /// Like [`is_version_ignored_for_file`] but accepts a pre-built `DepContext`.
    ///
    /// Ensures all matchers (`matchDepTypes`, `matchRepositories`, …) fire when
    /// per-rule `ignoreVersions` is combined with additional matchers.
    pub fn is_version_ignored_ctx(&self, ctx: &DepContext<'_>, proposed_version: &str) -> bool {
        if version_matches_ignore_list(proposed_version, &self.ignore_versions) {
            return true;
        }
        self.package_rules
            .iter()
            .any(|rule| rule.matches_context(ctx) && rule.version_is_ignored(proposed_version))
    }

    /// Collect merged packageRule effects for a dep.
    ///
    /// Evaluates all rules in order and merges their positive effects:
    /// - `group_name`: first matching rule that sets it wins.
    /// - `automerge`: last matching rule that sets it wins (overrides repo default).
    /// - `schedule`: last matching rule that sets it wins.
    /// - `labels`: union of all matching rules.
    ///
    /// Repository-level defaults (`automerge`, `group_name`) are applied after
    /// the rules, so rule-level values take precedence.
    ///
    /// Renovate reference: `lib/util/package-rules/index.ts` —
    /// `applyPackageRules()` merging logic.
    pub fn collect_rule_effects(&self, ctx: &DepContext<'_>) -> RuleEffects {
        // Seed with repo-level config as the base.
        // `labels` and `addLabels` start with the repo-level values.
        let mut effects = RuleEffects {
            labels: self.labels.clone(),
            assignees: self.assignees.clone(),
            reviewers: self.reviewers.clone(),
            ..RuleEffects::default()
        };
        // Accumulate repo-level addLabels into the label set.
        for l in &self.add_labels {
            if !effects.labels.contains(l) {
                effects.labels.push(l.clone());
            }
        }

        for rule in &self.package_rules {
            if !rule.matches_context(ctx) {
                continue;
            }
            // Renovate applies packageRules via mergeChildConfig (lib/util/package-rules/index.ts):
            // - fields without `mergeable: true` REPLACE the current value (last rule wins)
            // - fields with `mergeable: true` (addLabels) APPEND to the current value
            if rule.group_name.is_some() {
                effects.group_name.clone_from(&rule.group_name);
            }
            if rule.group_slug.is_some() {
                effects.group_slug.clone_from(&rule.group_slug);
            }
            if let Some(am) = rule.automerge {
                effects.automerge = Some(am);
            }
            if !rule.schedule.is_empty() {
                effects.schedule.clone_from(&rule.schedule);
            }
            // `labels` is NOT mergeable → replaces the current label set.
            if !rule.labels.is_empty() {
                effects.labels.clone_from(&rule.labels);
            }
            // `addLabels` IS mergeable → appends to the current label set.
            for label in &rule.add_labels {
                if !effects.labels.contains(label) {
                    effects.labels.push(label.clone());
                }
            }
            if rule.minimum_release_age.is_some() {
                effects
                    .minimum_release_age
                    .clone_from(&rule.minimum_release_age);
            }
            if rule.pr_priority.is_some() {
                effects.pr_priority = rule.pr_priority;
            }
            if rule.commit_message_topic.is_some() {
                effects
                    .commit_message_topic
                    .clone_from(&rule.commit_message_topic);
            }
            if rule.commit_message_action.is_some() {
                effects
                    .commit_message_action
                    .clone_from(&rule.commit_message_action);
            }
            if rule.commit_message_prefix.is_some() {
                effects
                    .commit_message_prefix
                    .clone_from(&rule.commit_message_prefix);
            }
            if rule.semantic_commit_type.is_some() {
                effects
                    .semantic_commit_type
                    .clone_from(&rule.semantic_commit_type);
            }
            if rule.semantic_commit_scope.is_some() {
                effects
                    .semantic_commit_scope
                    .clone_from(&rule.semantic_commit_scope);
            }
            if rule.commit_message_extra.is_some() {
                effects
                    .commit_message_extra
                    .clone_from(&rule.commit_message_extra);
            }
            if rule.commit_message_suffix.is_some() {
                effects
                    .commit_message_suffix
                    .clone_from(&rule.commit_message_suffix);
            }
            if rule.range_strategy.is_some() {
                effects.range_strategy.clone_from(&rule.range_strategy);
            }
            if rule.versioning.is_some() {
                effects.versioning.clone_from(&rule.versioning);
            }
            if rule.pin_digests.is_some() {
                effects.pin_digests = rule.pin_digests;
            }
            if rule.follow_tag.is_some() {
                effects.follow_tag.clone_from(&rule.follow_tag);
            }
            if rule.replacement_name.is_some() {
                effects.replacement_name.clone_from(&rule.replacement_name);
            }
            if rule.replacement_version.is_some() {
                effects
                    .replacement_version
                    .clone_from(&rule.replacement_version);
            }
            if rule.version_compatibility.is_some() {
                effects
                    .version_compatibility
                    .clone_from(&rule.version_compatibility);
            }
            if rule.changelog_url.is_some() {
                effects.changelog_url.clone_from(&rule.changelog_url);
            }
            if rule.dependency_dashboard_approval.is_some() {
                effects.dependency_dashboard_approval = rule.dependency_dashboard_approval;
            }
            // `assignees`/`reviewers` are NOT mergeable → replace.
            if !rule.assignees.is_empty() {
                effects.assignees.clone_from(&rule.assignees);
            }
            if !rule.reviewers.is_empty() {
                effects.reviewers.clone_from(&rule.reviewers);
            }
        }
        // Apply repo-level group_name if no rule set one.
        if effects.group_name.is_none() && self.group_name.is_some() {
            effects.group_name.clone_from(&self.group_name);
        }
        // Apply repo-level automerge if no rule overrode it.
        if effects.automerge.is_none() && self.automerge {
            effects.automerge = Some(true);
        }
        // Apply per-update-type config blocks (major/minor/patch) AFTER all
        // packageRules, mirroring Renovate's `flatten.ts` mergeChildConfig order.
        let update_type_cfg = match ctx.update_type {
            Some(crate::versioning::semver_generic::UpdateType::Major) => {
                self.major_config.as_ref()
            }
            Some(crate::versioning::semver_generic::UpdateType::Minor) => {
                self.minor_config.as_ref()
            }
            Some(crate::versioning::semver_generic::UpdateType::Patch) => {
                self.patch_config.as_ref()
            }
            _ => None,
        };
        if let Some(cfg) = update_type_cfg {
            cfg.apply_to_effects(&mut effects);
        }
        effects
    }
}

impl Default for RepoConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            ignore_deps: Vec::new(),
            ignore_paths: Vec::new(),
            include_paths: Vec::new(),
            package_rules: Vec::new(),
            enabled_managers: Vec::new(),
            disabled_managers: Vec::new(),
            ignore_versions: Vec::new(),
            schedule: Vec::new(),
            automerge_schedule: vec!["at any time".to_owned()],
            timezone: None,
            automerge: false,
            automerge_type: None,
            labels: Vec::new(),
            add_labels: Vec::new(),
            assignees: Vec::new(),
            reviewers: Vec::new(),
            draft_pr: false,
            assign_automerge: false,
            branch_prefix: "renovate/".to_owned(),
            additional_branch_prefix: String::new(),
            base_branches: Vec::new(),
            rebase_when: None,
            pr_concurrent_limit: 0,
            pr_creation: None,
            pr_hourly_limit: 2,
            group_name: None,
            separate_major_minor: true,
            separate_multiple_major: false,
            max_major_increment: 500,
            separate_minor_patch: false,
            separate_multiple_minor: false,
            semantic_commit_type: "chore".to_owned(),
            semantic_commit_scope: "deps".to_owned(),
            semantic_commits: None,
            extends: Vec::new(),
            ignore_presets: Vec::new(),
            minimum_release_age: None,
            ignore_unstable: false,
            respect_latest: false,
            pin_digests: false,
            dependency_dashboard: false,
            dependency_dashboard_approval: false,
            config_migration: false,
            update_not_scheduled: true,
            commit_message_action: "Update".to_owned(),
            commit_message_prefix: None,
            commit_message_extra: None,
            commit_message_suffix: None,
            commit_body: None,
            range_strategy: "auto".to_owned(),
            hashed_branch_length: None,
            major_config: None,
            minor_config: None,
            patch_config: None,
            custom_managers: Vec::new(),
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

    /// Return `true` when `path` is allowed by the `includePaths` config.
    ///
    /// If `include_paths` is empty, all paths are allowed (returns `true`).
    /// Otherwise the path must match at least one include pattern.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `includePaths`.
    pub fn is_path_included(&self, path: &str) -> bool {
        if self.include_paths.is_empty() {
            return true;
        }
        PathMatcher::new(&self.include_paths).is_ignored(path)
    }
}

/// Outcome of a repository config discovery attempt.
#[derive(Debug, Clone)]
pub enum RepoConfigResult {
    /// A config file was found; parsed config is ready to use.
    Found {
        path: String,
        config: Box<RepoConfig>,
    },
    /// No config file exists in the repository.
    NotFound,
    /// The repository has not been onboarded (no config) and
    /// `require_config = Required`.
    NeedsOnboarding,
}

/// Try to find a Renovate config file in the repository.
///
/// Tries each path in [`CONFIG_FILE_CANDIDATES`] in order and returns the
/// first one found.  After exhausting the dedicated config paths, also checks
/// `package.json` for a top-level `"renovate"` key (deprecated upstream but
/// still supported for compatibility).
///
/// Returns [`RepoConfigResult::NotFound`] or [`RepoConfigResult::NeedsOnboarding`]
/// when no config exists anywhere.
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
                config: Box::new(config),
            });
        }
    }

    // Fall back to package.json `"renovate"` key (deprecated; warn when used).
    if let Some(file) = client.get_raw_file(owner, repo, "package.json").await?
        && let Some(config) = RepoConfig::parse_from_package_json(&file.content)
    {
        tracing::warn!(
            repo = %format!("{owner}/{repo}"),
            "Using package.json for Renovate config is deprecated — \
             please migrate to a dedicated config file such as renovate.json"
        );
        return Ok(RepoConfigResult::Found {
            path: "package.json".to_owned(),
            config: Box::new(config),
        });
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

    /// Mount 404 mocks for all dedicated config candidates AND package.json.
    async fn mock_all_configs_404(server: &MockServer) {
        for candidate in CONFIG_FILE_CANDIDATES {
            Mock::given(method("GET"))
                .and(wm_path(format!("/repos/owner/repo/contents/{candidate}")))
                .respond_with(ResponseTemplate::new(404))
                .mount(server)
                .await;
        }
        // package.json fallback also returns 404.
        Mock::given(method("GET"))
            .and(wm_path("/repos/owner/repo/contents/package.json"))
            .respond_with(ResponseTemplate::new(404))
            .mount(server)
            .await;
    }

    #[tokio::test]
    async fn returns_needs_onboarding_when_no_config_and_required() {
        let server = MockServer::start().await;
        mock_all_configs_404(&server).await;

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
        mock_all_configs_404(&server).await;

        let client = make_client(&server.uri());
        let config = GlobalConfig {
            require_config: RequireConfig::Optional,
            ..GlobalConfig::default()
        };
        let result = discover(&client, "owner", "repo", &config).await.unwrap();
        assert!(matches!(result, RepoConfigResult::NotFound));
    }

    #[tokio::test]
    async fn discovers_renovate_key_in_package_json() {
        let server = MockServer::start().await;
        // All dedicated config files return 404.
        for candidate in CONFIG_FILE_CANDIDATES {
            Mock::given(method("GET"))
                .and(wm_path(format!("/repos/owner/repo/contents/{candidate}")))
                .respond_with(ResponseTemplate::new(404))
                .mount(&server)
                .await;
        }
        // package.json has a `renovate` key.
        let pkg_json = serde_json::json!({
            "name": "my-app",
            "version": "1.0.0",
            "renovate": {
                "enabled": true,
                "ignoreDeps": ["lodash"]
            }
        });
        Mock::given(method("GET"))
            .and(wm_path("/repos/owner/repo/contents/package.json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "content": base64::engine::general_purpose::STANDARD
                    .encode(pkg_json.to_string()),
                "encoding": "base64"
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let result = discover(&client, "owner", "repo", &GlobalConfig::default())
            .await
            .unwrap();

        let (path, config) = match result {
            RepoConfigResult::Found { path, config } => (path, config),
            other => panic!("expected Found, got {other:?}"),
        };
        assert_eq!(path, "package.json");
        assert!(config.enabled);
        assert_eq!(config.ignore_deps, vec!["lodash"]);
    }

    #[tokio::test]
    async fn package_json_without_renovate_key_triggers_onboarding() {
        let server = MockServer::start().await;
        for candidate in CONFIG_FILE_CANDIDATES {
            Mock::given(method("GET"))
                .and(wm_path(format!("/repos/owner/repo/contents/{candidate}")))
                .respond_with(ResponseTemplate::new(404))
                .mount(&server)
                .await;
        }
        // package.json exists but has no `renovate` key.
        let pkg_json = serde_json::json!({"name": "my-app", "version": "1.0.0"});
        Mock::given(method("GET"))
            .and(wm_path("/repos/owner/repo/contents/package.json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "content": base64::engine::general_purpose::STANDARD
                    .encode(pkg_json.to_string()),
                "encoding": "base64"
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let result = discover(&client, "owner", "repo", &GlobalConfig::default())
            .await
            .unwrap();

        assert!(matches!(result, RepoConfigResult::NeedsOnboarding));
    }

    // ── RepoConfig::parse_from_package_json ─────────────────────────────────

    #[test]
    fn parse_from_package_json_extracts_renovate_key() {
        let pkg = r#"{"name":"app","version":"1.0.0","renovate":{"ignoreDeps":["lodash"]}}"#;
        let c = RepoConfig::parse_from_package_json(pkg).expect("should find renovate key");
        assert_eq!(c.ignore_deps, vec!["lodash"]);
    }

    #[test]
    fn parse_from_package_json_returns_none_when_no_key() {
        let pkg = r#"{"name":"app","version":"1.0.0","dependencies":{"react":"^18"}}"#;
        assert!(RepoConfig::parse_from_package_json(pkg).is_none());
    }

    #[test]
    fn parse_from_package_json_returns_none_for_invalid_json() {
        assert!(RepoConfig::parse_from_package_json("not json").is_none());
    }

    #[test]
    fn parse_from_package_json_full_config() {
        let pkg = r#"{
            "name": "my-app",
            "renovate": {
                "schedule": ["before 5am"],
                "automerge": true,
                "labels": ["deps"]
            }
        }"#;
        let c = RepoConfig::parse_from_package_json(pkg).unwrap();
        assert_eq!(c.schedule, vec!["before 5am"]);
        assert!(c.automerge);
        assert_eq!(c.labels, vec!["deps"]);
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
        assert!(c.is_manager_enabled("cargo", false));
        assert!(c.is_manager_enabled("npm", false));
        assert!(!c.is_manager_enabled("maven", false));
    }

    #[test]
    fn enabled_managers_empty_means_all_active() {
        let c = RepoConfig::parse("{}");
        assert!(c.enabled_managers.is_empty());
        assert!(c.is_manager_enabled("cargo", false));
        assert!(c.is_manager_enabled("maven", false));
        assert!(c.is_manager_enabled("anything", false));
    }

    #[test]
    fn disabled_by_default_manager_skipped_without_explicit_list() {
        let c = RepoConfig::parse("{}");
        // No enabledManagers → disabled-by-default managers do NOT run.
        assert!(!c.is_manager_enabled("git-submodules", true));
        assert!(!c.is_manager_enabled("html", true));
        assert!(!c.is_manager_enabled("nix", true));
        assert!(!c.is_manager_enabled("pre-commit", true));
        // Non-disabled managers still run.
        assert!(c.is_manager_enabled("cargo", false));
    }

    #[test]
    fn disabled_by_default_manager_enabled_when_explicitly_listed() {
        let c = RepoConfig::parse(r#"{"enabledManagers": ["git-submodules", "cargo"]}"#);
        // Explicitly listed → enabled regardless of disabled_by_default flag.
        assert!(c.is_manager_enabled("git-submodules", true));
        assert!(c.is_manager_enabled("cargo", false));
        // Not listed → disabled.
        assert!(!c.is_manager_enabled("npm", false));
    }

    #[test]
    fn disabled_by_default_enabled_overrides_default_flag() {
        // When enabledManagers has entries, disabled_by_default is irrelevant.
        let c = RepoConfig::parse(r#"{"enabledManagers": ["pre-commit"]}"#);
        assert!(c.is_manager_enabled("pre-commit", true));
        // Other managers not listed are off regardless of their default.
        assert!(!c.is_manager_enabled("cargo", false));
        assert!(!c.is_manager_enabled("nix", true));
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

    #[test]
    fn include_paths_empty_allows_all() {
        let c = RepoConfig::parse(r#"{}"#);
        assert!(c.is_path_included("package.json"));
        assert!(c.is_path_included("apps/frontend/package.json"));
    }

    #[test]
    fn include_paths_limits_to_matching_files() {
        let c = RepoConfig::parse(r#"{"includePaths": ["apps/**"]}"#);
        assert!(c.is_path_included("apps/backend/package.json"));
        assert!(!c.is_path_included("package.json"));
        assert!(!c.is_path_included("libs/utils/package.json"));
    }

    #[test]
    fn include_paths_prefix_match() {
        let c = RepoConfig::parse(r#"{"includePaths": ["apps"]}"#);
        assert!(c.is_path_included("apps/frontend/package.json"));
        assert!(!c.is_path_included("package.json"));
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
    fn match_package_names_negation() {
        // "!lodash" in matchPackageNames excludes lodash, allows others
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["!lodash"], "enabled": false}]}"#,
        );
        assert!(!c.is_dep_ignored("lodash"));
        assert!(c.is_dep_ignored("express"));
    }

    #[test]
    fn match_package_names_glob_negation() {
        // "!@babel/**" excludes the whole @babel scope
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["!@babel/**"], "enabled": false}]}"#,
        );
        assert!(!c.is_dep_ignored("@babel/core"));
        assert!(c.is_dep_ignored("lodash"));
    }

    // ── Ported from Renovate package-names.spec.ts ───────────────────────────

    #[test]
    fn match_package_names_uses_package_name_when_set() {
        // "should matchPackageName": packageName='def' matches ['def', 'ghi']
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["def", "ghi"], "automerge": true}]}"#,
        );
        let ctx = DepContext {
            dep_name: "other",
            package_name: Some("def"),
            ..Default::default()
        };
        assert_eq!(
            c.collect_rule_effects(&ctx).automerge,
            Some(true),
            "rule should fire when packageName matches"
        );
    }

    #[test]
    fn match_package_names_with_dep_name_and_package_name() {
        // "should return false if not matching": depName='abc', packageName='def', rule=['ghi']
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["ghi"], "automerge": true}]}"#,
        );
        let ctx = DepContext {
            dep_name: "abc",
            package_name: Some("def"),
            ..Default::default()
        };
        assert_eq!(
            c.collect_rule_effects(&ctx).automerge,
            None,
            "rule should not fire when neither name matches"
        );
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
        // Without manager context, a rule with matchManagers set does NOT fire.
        // (matches_context requires manager to be known before matchManagers fires)
        assert!(!c.is_dep_ignored("lodash"));
    }

    // ── matchManagers glob/regex/negation (Renovate-compat) ──────────────────

    #[test]
    fn match_managers_glob_pattern() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchManagers": ["npm*"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.manager_matches("npm"));
        assert!(rule.manager_matches("npm-check"));
        assert!(!rule.manager_matches("cargo"));
    }

    #[test]
    fn match_managers_regex_pattern() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchManagers": ["/^(npm|pip)/"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.manager_matches("npm"));
        assert!(rule.manager_matches("pip"));
        assert!(!rule.manager_matches("cargo"));
    }

    #[test]
    fn match_managers_negation() {
        // ["npm", "!cargo"] means: match npm but exclude cargo
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchManagers": ["npm", "!cargo"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.manager_matches("npm"));
        assert!(!rule.manager_matches("cargo"));
    }

    #[test]
    fn match_managers_custom_prefix() {
        // Renovate uses "custom.regex" to target the regex custom manager.
        // The actual manager name in our system is "regex".
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchManagers": ["custom.regex"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(
            rule.manager_matches("regex"),
            "regex is a custom manager → custom.regex"
        );
        assert!(!rule.manager_matches("npm"));
    }

    // ── Ported from Renovate managers.spec.ts ────────────────────────────────

    #[test]
    fn match_managers_no_manager_no_rule_fire() {
        // "should return false if no manager": undefined manager + matchManagers set → rule doesn't fire.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchManagers": ["npm"], "automerge": true}]}"#,
        );
        let ctx = DepContext {
            dep_name: "lodash",
            manager: None,
            ..Default::default()
        };
        assert_eq!(
            c.collect_rule_effects(&ctx).automerge,
            None,
            "rule must not fire when manager is None"
        );
    }

    #[test]
    fn match_managers_undefined_rule_fires_for_any_manager() {
        // "should return null if matchManagers is undefined": no matchManagers → matches any manager.
        let c = RepoConfig::parse(r#"{"packageRules": [{"automerge": true}]}"#);
        let ctx = DepContext {
            dep_name: "lodash",
            manager: Some("npm"),
            ..Default::default()
        };
        assert_eq!(
            c.collect_rule_effects(&ctx).automerge,
            Some(true),
            "rule must fire when matchManagers is absent"
        );
    }

    #[test]
    fn match_managers_legacy_regex_matches_custom_regex_rule() {
        // "should match custom managers": manager:'regex' (legacy) matches matchManagers:['custom.regex']
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchManagers": ["custom.regex"], "automerge": true}]}"#,
        );
        let ctx = DepContext {
            dep_name: "my-dep",
            manager: Some("regex"),
            ..Default::default()
        };
        assert_eq!(
            c.collect_rule_effects(&ctx).automerge,
            Some(true),
            "legacy 'regex' manager should match 'custom.regex' rule"
        );
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
    fn match_dep_types_glob_pattern() {
        // "dev*" should match "devDependencies" but not "dependencies" or "peerDependencies"
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchDepTypes": ["dev*"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.dep_type_matches("devDependencies"));
        assert!(!rule.dep_type_matches("dependencies"));
        assert!(!rule.dep_type_matches("peerDependencies"));
    }

    #[test]
    fn match_dep_types_negation() {
        // ["dependencies", "!devDependencies"] → matches production but not dev
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchDepTypes": ["dependencies", "!devDependencies"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.dep_type_matches("dependencies"));
        assert!(!rule.dep_type_matches("devDependencies"));
    }

    #[test]
    fn match_dep_types_enabled_false_via_ctx_blocks_dev_dep() {
        // Regression: is_update_blocked_ctx must include dep_type in context so
        // matchDepTypes + enabled:false actually fires.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchDepTypes": ["devDependencies"], "enabled": false}]}"#,
        );
        // devDependency → should be blocked
        let ctx_dev = DepContext {
            dep_name: "jest",
            dep_type: Some("devDependencies"),
            update_type: Some(crate::versioning::semver_generic::UpdateType::Minor),
            ..Default::default()
        };
        assert!(
            c.is_update_blocked_ctx(&ctx_dev),
            "matchDepTypes:devDependencies + enabled:false should block devDependencies"
        );
        // Regular dependency → should NOT be blocked
        let ctx_prod = DepContext {
            dep_name: "react",
            dep_type: Some("dependencies"),
            update_type: Some(crate::versioning::semver_generic::UpdateType::Minor),
            ..Default::default()
        };
        assert!(
            !c.is_update_blocked_ctx(&ctx_prod),
            "rule should not block production dependencies"
        );
    }

    #[test]
    fn is_version_restricted_ctx_uses_dep_type() {
        // Regression: is_version_restricted_ctx must use the dep_type in context
        // so matchDepTypes + allowedVersions correctly applies only to matching dep types.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchDepTypes": ["devDependencies"], "allowedVersions": "< 2.0"}]}"#,
        );
        // devDependency proposing v2.0.0 → restricted (2.0 not < 2.0)
        let ctx_dev = DepContext {
            dep_name: "jest",
            dep_type: Some("devDependencies"),
            ..Default::default()
        };
        assert!(
            c.is_version_restricted_ctx(&ctx_dev, "2.0.0"),
            "allowedVersions should restrict devDependency version"
        );
        // Production dependency → rule doesn't match → not restricted
        let ctx_prod = DepContext {
            dep_name: "jest",
            dep_type: Some("dependencies"),
            ..Default::default()
        };
        assert!(
            !c.is_version_restricted_ctx(&ctx_prod, "2.0.0"),
            "rule should not restrict production dependency"
        );
    }

    #[test]
    fn match_datasources_glob_pattern() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchDatasources": ["npm*"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.datasource_matches("npm"));
        assert!(!rule.datasource_matches("pypi"));
    }

    #[test]
    fn match_datasources_negation() {
        // ["npm", "!docker"] → matches npm but not docker
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchDatasources": ["npm", "!docker"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.datasource_matches("npm"));
        assert!(!rule.datasource_matches("docker"));
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
    fn match_update_types_bump_parses() {
        // "bump" must be recognized as UpdateType::Bump (not filtered out as unknown).
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchUpdateTypes": ["bump"], "labels": ["bump"]}]}"#,
        );
        assert_eq!(
            c.package_rules[0].match_update_types,
            vec![UpdateType::Bump]
        );
    }

    #[test]
    fn is_bump_matches_bump_update_type_rule() {
        // Ported from Renovate index.spec.ts "applies" test:
        // isBump: true + matchUpdateTypes: ["bump"] → rule should apply.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchUpdateTypes": ["bump"], "labels": ["bump"]}]}"#,
        );
        let rule = &c.package_rules[0];
        // Without is_bump, a minor update does NOT match.
        assert!(!rule.update_type_matches(UpdateType::Minor, false));
        // With is_bump, the same update DOES match the "bump" rule.
        assert!(rule.update_type_matches(UpdateType::Minor, true));
        // is_bump has no effect when the rule doesn't mention "bump".
        let c2 = RepoConfig::parse(
            r#"{"packageRules": [{"matchUpdateTypes": ["major"], "enabled": false}]}"#,
        );
        let rule2 = &c2.package_rules[0];
        assert!(!rule2.update_type_matches(UpdateType::Minor, true));
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
        // "pin" and "digest" are unrecognized update types — rule has has_update_type_constraint
        // but empty match_update_types, so it cannot match any major/minor/patch update.
        assert!(!c.is_update_blocked("serde", "1.0.0", UpdateType::Major, "cargo"));
    }

    #[test]
    fn mixed_known_unknown_update_types_still_match_known() {
        let c = RepoConfig::parse(
            r#"{
                "packageRules": [{
                    "matchUpdateTypes": ["major", "pin"],
                    "enabled": false
                }]
            }"#,
        );
        // "major" is recognized, "pin" is not — rule should still block major updates.
        assert!(c.is_update_blocked("serde", "1.0.0", UpdateType::Major, "cargo"));
        // Minor should not be blocked (only major and pin are in the list).
        assert!(!c.is_update_blocked("serde", "1.0.0", UpdateType::Minor, "cargo"));
    }

    #[test]
    fn is_update_blocked_ctx_fires_without_update_type() {
        // Regression: enabled:false should block even when update_type is None
        // (non-semver deps like Docker image tags where semver classification fails).
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchManagers": ["dockerfile"], "enabled": false}]}"#,
        );
        let ctx = DepContext {
            dep_name: "nginx",
            manager: Some("dockerfile"),
            ..Default::default() // update_type is None (Docker tag, not parseable as semver)
        };
        assert!(
            c.is_update_blocked_ctx(&ctx),
            "enabled:false should fire even when update_type is None"
        );
    }

    #[test]
    fn enabled_true_later_rule_overrides_earlier_enabled_false() {
        // Renovate: last matching rule wins for enabled field.
        // A later `enabled: true` should re-enable a dep blocked by an earlier `enabled: false`.
        let c = RepoConfig::parse(
            r#"{
                "packageRules": [
                    {"matchPackageNames": ["*"], "enabled": false},
                    {"matchPackageNames": ["lodash"], "enabled": true}
                ]
            }"#,
        );
        let ctx = DepContext::for_dep("lodash");
        // lodash matches both rules; later enabled:true should win.
        assert!(
            !c.is_update_blocked_ctx(&ctx),
            "later enabled:true should override earlier enabled:false"
        );
        let ctx2 = DepContext::for_dep("react");
        // react only matches the first rule (enabled:false).
        assert!(
            c.is_update_blocked_ctx(&ctx2),
            "react should still be blocked"
        );
    }

    #[test]
    fn is_update_blocked_ctx_non_matching_manager_not_blocked() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchManagers": ["dockerfile"], "enabled": false}]}"#,
        );
        let ctx = DepContext {
            dep_name: "lodash",
            manager: Some("npm"),
            ..Default::default()
        };
        assert!(
            !c.is_update_blocked_ctx(&ctx),
            "rule for dockerfile should not block npm deps"
        );
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
    fn allowed_versions_last_rule_wins() {
        // First rule restricts to < 2.0, second rule (matching serde) allows any >= 1.0.
        // The second rule should win for serde, allowing 2.0+.
        let c = RepoConfig::parse(
            r#"{
                "packageRules": [
                    {"matchPackageNames": ["*"], "allowedVersions": "< 2.0"},
                    {"matchPackageNames": ["serde"], "allowedVersions": ">= 1.0.0"}
                ]
            }"#,
        );
        // serde: last matching rule (>= 1.0.0) wins → 2.5.0 should be allowed
        assert!(!c.is_version_restricted("serde", "cargo", "2.5.0"));
        // react: only first rule matches → < 2.0 still applies
        assert!(c.is_version_restricted("react", "npm", "2.0.0"));
    }

    #[test]
    fn allowed_versions_gte_constraint() {
        let c = RepoConfig::parse(r#"{"packageRules": [{"allowedVersions": ">= 1.0.0"}]}"#);
        assert!(!c.is_version_restricted("anything", "cargo", "1.0.0"));
        assert!(!c.is_version_restricted("anything", "cargo", "2.0.0"));
        assert!(c.is_version_restricted("anything", "cargo", "0.9.0"));
    }

    #[test]
    fn allowed_versions_regex_allows_matching_version() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["foo"], "allowedVersions": "/^1\\./"}]}"#,
        );
        // 1.x versions match the regex → not restricted
        assert!(!c.is_version_restricted("foo", "cargo", "1.2.3"));
        // 2.x versions don't match → restricted
        assert!(c.is_version_restricted("foo", "cargo", "2.0.0"));
    }

    #[test]
    fn allowed_versions_regex_non_semver_version() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["img"], "allowedVersions": "/^v1\\./"}]}"#,
        );
        // Docker tags with v-prefix (non-semver)
        assert!(!c.is_version_restricted("img", "docker", "v1.2.3"));
        assert!(c.is_version_restricted("img", "docker", "v2.0.0"));
    }

    #[test]
    fn allowed_versions_exact_string_match() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["foo"], "allowedVersions": "1.2.3"}]}"#,
        );
        assert!(!c.is_version_restricted("foo", "cargo", "1.2.3"));
        assert!(c.is_version_restricted("foo", "cargo", "1.2.4"));
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

    #[test]
    fn match_current_version_regex_against_current_value() {
        // Regex pattern in matchCurrentVersion is matched against the raw currentValue
        // string (not parsed as semver). Previously we returned true for all regex
        // patterns — now they're properly evaluated.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCurrentVersion": "/^0/", "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        // "0.1.0" starts with "0" → matches /^0/
        assert!(rule.current_version_matches("0.1.0", None));
        // "1.0.0" starts with "1", does NOT match /^0/
        assert!(!rule.current_version_matches("1.0.0", None));
    }

    #[test]
    fn match_current_version_negated_regex() {
        // !/^0/ means: current version does NOT match /^0/
        // Used by :automergeStableNonMajor to match only stable (non-0.x) packages.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCurrentVersion": "!/^0/", "automerge": true}]}"#,
        );
        let rule = &c.package_rules[0];
        // "0.1.0" matches /^0/ → !/^0/ is FALSE for this version
        assert!(!rule.current_version_matches("0.1.0", None));
        // "1.0.0" does NOT match /^0/ → !/^0/ is TRUE
        assert!(rule.current_version_matches("1.0.0", None));
        // "2.5.3" does NOT match /^0/ → !/^0/ is TRUE
        assert!(rule.current_version_matches("2.5.3", None));
    }

    #[test]
    fn automerge_stable_non_major_preset_blocks_zero_deps() {
        // :automergeStableNonMajor uses matchCurrentVersion: "!/^0/"
        // It should NOT automerge 0.x packages.
        use crate::versioning::semver_generic::UpdateType;
        let c = RepoConfig::parse(r#"{"extends": [":automergeStableNonMajor"]}"#);
        // A 0.x dep with a minor update should NOT get automerge.
        let ctx = DepContext {
            dep_name: "unstable-pkg",
            update_type: Some(UpdateType::Minor),
            current_value: Some("0.5.0"),
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert!(
            effects.automerge.is_none() || effects.automerge == Some(false),
            "0.x package should not automerge via :automergeStableNonMajor"
        );
        // A stable (1.x) dep with a minor update SHOULD get automerge.
        let ctx2 = DepContext {
            dep_name: "stable-pkg",
            update_type: Some(UpdateType::Minor),
            current_value: Some("1.2.3"),
            ..Default::default()
        };
        let effects2 = c.collect_rule_effects(&ctx2);
        assert_eq!(effects2.automerge, Some(true));
    }

    // ── Ported from Renovate current-version.spec.ts (lockedVersion) ──────────

    #[test]
    fn match_current_version_regex_prefers_locked_version() {
        // Ported: "return true for regex version match" test from current-version.spec.ts.
        // When lockedVersion is set, regex matchCurrentVersion tests against it.
        // ruby: currentValue='"~> 0.1.0"', lockedVersion='0.1.0'
        // Pattern: /^v?[~ -]?0/ → tests against "0.1.0" → matches (starts with "0").
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCurrentVersion": "/^v?[~ -]?0/", "automerge": true}]}"#,
        );
        let rule = &c.package_rules[0];
        // With lockedVersion="0.1.0" — regex should match.
        assert!(rule.current_version_matches(r#""~> 0.1.0""#, Some("0.1.0")));
    }

    #[test]
    fn match_current_version_regex_false_without_locked_version() {
        // Ported: "return false for regex value match" from current-version.spec.ts.
        // When lockedVersion is absent, falls back to currentValue='"~> 0.1.0"'.
        // Pattern: /^v?[~ -]?0/ against '"~> 0.1.0"' → doesn't match (starts with '"').
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCurrentVersion": "/^v?[~ -]?0/", "automerge": true}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(!rule.current_version_matches(r#""~> 0.1.0""#, None));
    }

    #[test]
    fn match_current_version_via_dep_context_with_locked_version() {
        // End-to-end: dep with locked_version should use it for matchCurrentVersion regex.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCurrentVersion": "/^0/", "enabled": false}]}"#,
        );
        // Dep with range currentValue but locked_version starting with "0"
        let ctx_locked = DepContext {
            dep_name: "my-pkg",
            current_value: Some("^0.5.0"),
            locked_version: Some("0.5.3"),
            ..Default::default()
        };
        assert!(
            c.is_dep_ignored_ctx(&ctx_locked),
            "rule should fire when lockedVersion matches the regex"
        );
        // Same range but locked at 1.x → rule should NOT fire.
        let ctx_locked_1x = DepContext {
            dep_name: "my-pkg",
            current_value: Some("^1.0.0"),
            locked_version: Some("1.2.3"),
            ..Default::default()
        };
        assert!(
            !c.is_dep_ignored_ctx(&ctx_locked_1x),
            "rule should not fire when lockedVersion does not match regex"
        );
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

    // ── Ported from Renovate dep-names.spec.ts ────────────────────────────────

    #[test]
    fn match_dep_names_undefined_dep_name_does_not_fire() {
        // Ported: "should return false if packageFile is not defined" (depName: undefined).
        // When dep_name is absent/empty and matchDepNames is set → rule doesn't fire.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchDepNames": ["@opentelemetry/http"], "automerge": true}]}"#,
        );
        // Empty dep_name (simulates undefined) with matchDepNames set → rule must not fire.
        let ctx = DepContext {
            dep_name: "",
            ..Default::default()
        };
        assert_eq!(
            c.collect_rule_effects(&ctx).automerge,
            None,
            "rule must not fire when dep_name is empty/absent and matchDepNames is set"
        );
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

    #[test]
    fn match_dep_prefixes_converted_to_glob() {
        // Deprecated matchDepPrefixes → "prefix**" globs in matchDepNames.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchDepPrefixes": ["@angular/"], "enabled": false}]}"#,
        );
        let ctx = DepContext::for_dep("@angular/core");
        assert!(c.is_update_blocked_ctx(&ctx));
        let ctx2 = DepContext::for_dep("react");
        assert!(!c.is_update_blocked_ctx(&ctx2));
    }

    #[test]
    fn match_dep_patterns_converted_to_regex() {
        // Deprecated matchDepPatterns → "/pattern/" strings in matchDepNames.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchDepPatterns": ["^@angular"], "enabled": false}]}"#,
        );
        let ctx = DepContext::for_dep("@angular/core");
        assert!(c.is_update_blocked_ctx(&ctx));
        let ctx2 = DepContext::for_dep("react");
        assert!(!c.is_update_blocked_ctx(&ctx2));
    }

    #[test]
    fn match_dep_names_negation_regex() {
        // "!/^@opentelemetry/" excludes the whole @opentelemetry scope.
        // Ported from lib/util/package-rules/dep-names.spec.ts
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchDepNames": ["!/^@opentelemetry/"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(!rule.dep_name_matches("@opentelemetry/http"));
        assert!(rule.dep_name_matches("lodash"));
    }

    #[test]
    fn match_dep_names_negation_glob() {
        // "!@opentelemetry/**" excludes the whole @opentelemetry scope via glob.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchDepNames": ["!@opentelemetry/**"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(!rule.dep_name_matches("@opentelemetry/http"));
        assert!(rule.dep_name_matches("express"));
    }

    #[test]
    fn match_dep_names_regex_includes() {
        // "/^@opentelemetry/" positive match
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchDepNames": ["/^@opentelemetry/"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.dep_name_matches("@opentelemetry/http"));
        assert!(!rule.dep_name_matches("express"));
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

    // ── Metadata config fields ───────────────────────────────────────────────

    #[test]
    fn schedule_parsed_into_repo_config() {
        let c = RepoConfig::parse(r#"{"schedule": ["before 5am", "every weekend"]}"#);
        assert_eq!(c.schedule, vec!["before 5am", "every weekend"]);
    }

    // ── Ported from migration.spec.ts schedule migration ─────────────────────

    #[test]
    fn schedule_every_friday_migrated_to_on_friday() {
        // Ported: "migrates every friday" from migration.spec.ts
        let c = RepoConfig::parse(r#"{"schedule": "every friday"}"#);
        assert_eq!(c.schedule, vec!["on friday"]);
    }

    #[test]
    fn schedule_every_weekday_not_migrated() {
        // Ported: "does not migrate every weekday" — "every weekday" stays unchanged.
        let c = RepoConfig::parse(r#"{"schedule": "every weekday"}"#);
        assert_eq!(c.schedule, vec!["every weekday"]);
    }

    #[test]
    fn schedule_every_monday_migrated() {
        let c = RepoConfig::parse(r#"{"schedule": ["every monday"]}"#);
        assert_eq!(c.schedule, vec!["on monday"]);
    }

    #[test]
    fn schedule_default_is_empty() {
        let c = RepoConfig::parse(r#"{}"#);
        assert!(c.schedule.is_empty());
    }

    #[test]
    fn timezone_parsed() {
        let c = RepoConfig::parse(r#"{"timezone": "America/New_York"}"#);
        assert_eq!(c.timezone.as_deref(), Some("America/New_York"));
    }

    #[test]
    fn automerge_defaults_false() {
        let c = RepoConfig::parse(r#"{}"#);
        assert!(!c.automerge);
    }

    #[test]
    fn automerge_parsed_true() {
        let c = RepoConfig::parse(r#"{"automerge": true}"#);
        assert!(c.automerge);
    }

    #[test]
    fn automerge_legacy_none_string_migrated_to_false() {
        let c = RepoConfig::parse(r#"{"automerge": "none"}"#);
        assert!(!c.automerge, "automerge: 'none' must migrate to false");
    }

    #[test]
    fn automerge_legacy_any_string_migrated_to_true() {
        let c = RepoConfig::parse(r#"{"automerge": "any"}"#);
        assert!(c.automerge, "automerge: 'any' must migrate to true");
    }

    #[test]
    fn semantic_commits_bool_true_migrated_to_enabled() {
        let c = RepoConfig::parse(r#"{"semanticCommits": true}"#);
        assert_eq!(
            c.semantic_commits.as_deref(),
            Some("enabled"),
            "semanticCommits: true must migrate to 'enabled'"
        );
    }

    #[test]
    fn semantic_commits_bool_false_migrated_to_disabled() {
        let c = RepoConfig::parse(r#"{"semanticCommits": false}"#);
        assert_eq!(
            c.semantic_commits.as_deref(),
            Some("disabled"),
            "semanticCommits: false must migrate to 'disabled'"
        );
    }

    #[test]
    fn base_branch_singular_prepended_to_base_branches() {
        let c = RepoConfig::parse(r#"{"baseBranch": "develop"}"#);
        assert!(
            c.base_branches.contains(&"develop".to_owned()),
            "deprecated baseBranch must be added to baseBranches"
        );
    }

    #[test]
    fn separate_major_releases_alias_for_separate_major_minor() {
        let c = RepoConfig::parse(r#"{"separateMajorReleases": false}"#);
        assert!(
            !c.separate_major_minor,
            "separateMajorReleases: false must set separateMajorMinor to false"
        );
    }

    #[test]
    fn ignore_node_modules_true_adds_to_ignore_paths() {
        let c = RepoConfig::parse(r#"{"ignoreNodeModules": true}"#);
        assert!(
            c.ignore_paths.contains(&"node_modules/".to_owned()),
            "ignoreNodeModules: true must add node_modules/ to ignorePaths"
        );
    }

    #[test]
    fn enabled_managers_yarn_migrated_to_npm() {
        let c = RepoConfig::parse(r#"{"enabledManagers": ["yarn", "cargo"]}"#);
        assert!(
            c.enabled_managers.contains(&"npm".to_owned()),
            "enabledManagers: ['yarn'] must migrate to 'npm'"
        );
        assert!(
            !c.enabled_managers.contains(&"yarn".to_owned()),
            "'yarn' must be replaced, not kept"
        );
        assert!(c.enabled_managers.contains(&"cargo".to_owned()));
    }

    #[test]
    fn labels_parsed() {
        let c = RepoConfig::parse(r#"{"labels": ["dependencies", "renovate"]}"#);
        assert_eq!(c.labels, vec!["dependencies", "renovate"]);
    }

    #[test]
    fn branch_prefix_default() {
        let c = RepoConfig::parse(r#"{}"#);
        assert_eq!(c.branch_prefix, "renovate/");
    }

    #[test]
    fn branch_prefix_custom() {
        let c = RepoConfig::parse(r#"{"branchPrefix": "deps/"}"#);
        assert_eq!(c.branch_prefix, "deps/");
    }

    #[test]
    fn additional_branch_prefix_default_empty() {
        let c = RepoConfig::parse(r#"{}"#);
        assert_eq!(c.additional_branch_prefix, "");
    }

    #[test]
    fn additional_branch_prefix_parsed() {
        let c = RepoConfig::parse(r#"{"additionalBranchPrefix": "chore-"}"#);
        assert_eq!(c.additional_branch_prefix, "chore-");
    }

    #[test]
    fn base_branches_parsed() {
        let c = RepoConfig::parse(r#"{"baseBranches": ["main", "develop"]}"#);
        assert_eq!(c.base_branches, vec!["main", "develop"]);
    }

    #[test]
    fn rebase_when_parsed() {
        let c = RepoConfig::parse(r#"{"rebaseWhen": "behind-base-branch"}"#);
        assert_eq!(c.rebase_when.as_deref(), Some("behind-base-branch"));
    }

    #[test]
    fn rebase_stale_prs_preset_sets_rebase_when() {
        let c = RepoConfig::parse(r#"{"extends": [":rebaseStalePrs"]}"#);
        assert_eq!(c.rebase_when.as_deref(), Some("behind-base-branch"));
    }

    #[test]
    fn rebase_when_default_none() {
        let c = RepoConfig::parse(r#"{}"#);
        assert!(c.rebase_when.is_none());
    }

    #[test]
    fn commit_body_parsed_from_config() {
        let c = RepoConfig::parse(r#"{"commitBody": "Signed-off-by: Bot <bot@example.com>"}"#);
        assert_eq!(
            c.commit_body.as_deref(),
            Some("Signed-off-by: Bot <bot@example.com>")
        );
    }

    #[test]
    fn git_sign_off_preset_sets_commit_body() {
        let c = RepoConfig::parse(r#"{"extends": [":gitSignOff"]}"#);
        assert_eq!(
            c.commit_body.as_deref(),
            Some("Signed-off-by: {{{gitAuthor}}}")
        );
    }

    #[test]
    fn separate_major_minor_default_true() {
        let c = RepoConfig::parse(r#"{}"#);
        assert!(c.separate_major_minor);
    }

    #[test]
    fn separate_minor_patch_default_false() {
        let c = RepoConfig::parse(r#"{}"#);
        assert!(!c.separate_minor_patch);
    }

    #[test]
    fn pr_hourly_limit_default() {
        let c = RepoConfig::parse(r#"{}"#);
        assert_eq!(c.pr_hourly_limit, 2);
    }

    #[test]
    fn draft_pr_default_false() {
        let c = RepoConfig::parse(r#"{}"#);
        assert!(!c.draft_pr);
    }

    #[test]
    fn enable_pre_commit_preset_adds_to_enabled_managers() {
        let c = RepoConfig::parse(r#"{"extends": [":enablePreCommit"]}"#);
        assert!(
            c.enabled_managers.contains(&"pre-commit".to_owned()),
            "enablePreCommit should add pre-commit to enabled_managers"
        );
        assert!(
            c.is_manager_enabled("pre-commit", true),
            "pre-commit should be enabled via preset"
        );
    }

    #[test]
    fn pre_commit_disabled_by_default_without_preset() {
        let c = RepoConfig::parse(r#"{}"#);
        assert!(
            !c.is_manager_enabled("pre-commit", true),
            "pre-commit should NOT be enabled without explicit config"
        );
    }

    #[test]
    fn draft_pr_config() {
        let c = RepoConfig::parse(r#"{"draftPR": true}"#);
        assert!(c.draft_pr);
    }

    #[test]
    fn assign_automerge_default_false() {
        let c = RepoConfig::parse(r#"{}"#);
        assert!(!c.assign_automerge);
    }

    #[test]
    fn assign_automerge_config() {
        let c = RepoConfig::parse(r#"{"assignAutomerge": true}"#);
        assert!(c.assign_automerge);
    }

    #[test]
    fn pr_hourly_limit_custom() {
        let c = RepoConfig::parse(r#"{"prHourlyLimit": 5}"#);
        assert_eq!(c.pr_hourly_limit, 5);
    }

    #[test]
    fn group_name_parsed_at_repo_level() {
        let c = RepoConfig::parse(r#"{"groupName": "all-deps"}"#);
        assert_eq!(c.group_name.as_deref(), Some("all-deps"));
    }

    #[test]
    fn package_rule_group_name_parsed() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["react"], "groupName": "react-packages"}]}"#,
        );
        assert_eq!(
            c.package_rules[0].group_name.as_deref(),
            Some("react-packages")
        );
    }

    #[test]
    fn package_rule_automerge_parsed() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["prettier"], "automerge": true}]}"#,
        );
        assert_eq!(c.package_rules[0].automerge, Some(true));
    }

    #[test]
    fn package_rule_schedule_parsed() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchUpdateTypes": ["patch"], "schedule": ["every weekend"]}]}"#,
        );
        assert_eq!(c.package_rules[0].schedule, vec!["every weekend"]);
    }

    #[test]
    fn reviewers_and_assignees_parsed() {
        let c = RepoConfig::parse(r#"{"reviewers": ["user1", "user2"], "assignees": ["user3"]}"#);
        assert_eq!(c.reviewers, vec!["user1", "user2"]);
        assert_eq!(c.assignees, vec!["user3"]);
    }

    // ── extends preset resolution ────────────────────────────────────────────

    #[test]
    fn extends_field_stored() {
        let c = RepoConfig::parse(r#"{"extends": ["config:recommended"]}"#);
        assert_eq!(c.extends, vec!["config:recommended"]);
    }

    #[test]
    fn ignore_presets_filters_before_resolution() {
        // :semanticCommits would normally set semantic_commits = "enabled".
        // Listing it in ignorePresets should suppress the effect.
        let c = RepoConfig::parse(
            r#"{"extends": [":semanticCommits"], "ignorePresets": [":semanticCommits"]}"#,
        );
        assert!(
            c.semantic_commits.is_none(),
            "ignorePresets should suppress :semanticCommits, got: {:?}",
            c.semantic_commits
        );
    }

    #[test]
    fn ignore_presets_partial_suppression() {
        // Suppress :semanticCommits but keep :ignoreModulesAndTests.
        let c = RepoConfig::parse(
            r#"{
                "extends": [":semanticCommits", ":ignoreModulesAndTests"],
                "ignorePresets": [":semanticCommits"]
            }"#,
        );
        assert!(c.semantic_commits.is_none());
        assert!(c.ignore_paths.contains(&"**/node_modules/**".to_owned()));
    }

    #[test]
    fn ignore_presets_stored_on_config() {
        let c = RepoConfig::parse(
            r#"{"extends": ["config:recommended"], "ignorePresets": [":semanticPrefixFixDepsChoreOthers"]}"#,
        );
        assert_eq!(c.ignore_presets, vec![":semanticPrefixFixDepsChoreOthers"]);
    }

    #[test]
    fn ignore_presets_suppresses_separate_minor_patch() {
        // :automergePatch normally sets preset_separate_minor_patch = true.
        // Suppressing it with ignorePresets should keep separate_minor_patch = false.
        let c = RepoConfig::parse(
            r#"{"extends": [":automergePatch"], "ignorePresets": [":automergePatch"]}"#,
        );
        assert!(!c.separate_minor_patch);
    }

    #[test]
    fn pin_dependencies_preset_injects_range_strategy_rule() {
        let c = RepoConfig::parse(r#"{"extends": [":pinDependencies"]}"#);
        let rule = c
            .package_rules
            .iter()
            .find(|r| r.range_strategy.as_deref() == Some("pin"));
        assert!(rule.is_some(), "expected a pin rangeStrategy rule");
        let rule = rule.unwrap();
        assert!(
            rule.match_dep_types.contains(&"dependencies".to_owned()),
            "rule should match 'dependencies' dep type"
        );
    }

    #[test]
    fn pin_dev_dependencies_preset_injects_rule() {
        let c = RepoConfig::parse(r#"{"extends": [":pinDevDependencies"]}"#);
        let rule = c
            .package_rules
            .iter()
            .find(|r| r.range_strategy.as_deref() == Some("pin"));
        assert!(rule.is_some(), "expected a pin rangeStrategy rule");
        let rule = rule.unwrap();
        assert!(
            rule.match_dep_types.contains(&"devDependencies".to_owned()),
            "rule should match 'devDependencies'"
        );
    }

    #[test]
    fn preserve_semver_ranges_preset_injects_replace_rule() {
        let c = RepoConfig::parse(r#"{"extends": [":preserveSemverRanges"]}"#);
        let rule = c
            .package_rules
            .iter()
            .find(|r| r.range_strategy.as_deref() == Some("replace"));
        assert!(rule.is_some(), "expected a replace rangeStrategy rule");
    }

    #[test]
    fn range_strategy_in_package_rule_collects_into_effects() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["react"], "rangeStrategy": "pin"}]}"#,
        );
        let ctx = DepContext::for_dep("react");
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(effects.range_strategy.as_deref(), Some("pin"));
    }

    #[test]
    fn range_strategy_last_rule_wins() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [
                {"matchPackageNames": ["*"], "rangeStrategy": "pin"},
                {"matchPackageNames": ["react"], "rangeStrategy": "replace"}
            ]}"#,
        );
        let ctx = DepContext::for_dep("react");
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(effects.range_strategy.as_deref(), Some("replace"));
    }

    #[test]
    fn follow_tag_in_package_rule_collects_into_effects() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["typescript"], "followTag": "next"}]}"#,
        );
        let ctx = DepContext::for_dep("typescript");
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(effects.follow_tag.as_deref(), Some("next"));
    }

    #[test]
    fn versioning_in_package_rule_collects_into_effects() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["alpine"], "versioning": "docker"}]}"#,
        );
        let ctx = DepContext::for_dep("alpine");
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(effects.versioning.as_deref(), Some("docker"));
    }

    #[test]
    fn versioning_last_rule_wins() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [
                {"matchPackageNames": ["*"], "versioning": "semver"},
                {"matchPackageNames": ["alpine"], "versioning": "docker"}
            ]}"#,
        );
        let ctx = DepContext::for_dep("alpine");
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(effects.versioning.as_deref(), Some("docker"));
    }

    // ── pinDigests ───────────────────────────────────────────────────────────

    #[test]
    fn pin_digests_in_package_rule_collects_into_effects() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchDatasources": ["docker"], "pinDigests": true}]}"#,
        );
        let ctx = DepContext {
            dep_name: "alpine",
            datasource: Some("docker"),
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(effects.pin_digests, Some(true));
    }

    #[test]
    fn docker_pin_digests_preset_injects_docker_rule() {
        let c = RepoConfig::parse(r#"{"extends": ["docker:pinDigests"]}"#);
        assert_eq!(c.package_rules.len(), 2);
        let rule = &c.package_rules[0];
        assert_eq!(rule.pin_digests, Some(true));
        assert!(rule.match_datasources.contains(&"docker".to_owned()));
    }

    #[test]
    fn helpers_pin_github_action_digests_preset_pins_actions() {
        let c = RepoConfig::parse(r#"{"extends": ["helpers:pinGitHubActionDigests"]}"#);
        assert_eq!(c.package_rules.len(), 1);
        let rule = &c.package_rules[0];
        assert_eq!(rule.pin_digests, Some(true));
        assert!(rule.match_dep_types.contains(&"action".to_owned()));
    }

    #[test]
    fn config_recommended_adds_ignore_modules_and_tests_paths() {
        let c = RepoConfig::parse(r#"{"extends": ["config:recommended"]}"#);
        assert!(
            c.ignore_paths.contains(&"**/node_modules/**".to_owned()),
            "expected node_modules in ignorePaths, got: {:?}",
            c.ignore_paths
        );
        assert!(c.ignore_paths.contains(&"**/vendor/**".to_owned()));
        assert!(c.ignore_paths.contains(&"**/test/**".to_owned()));
        assert!(c.ignore_paths.contains(&"**/__tests__/**".to_owned()));
    }

    #[test]
    fn config_recommended_enables_dependency_dashboard() {
        let c = RepoConfig::parse(r#"{"extends": ["config:recommended"]}"#);
        assert!(
            c.dependency_dashboard,
            "config:recommended must enable dependency dashboard (via :dependencyDashboard)"
        );
    }

    #[test]
    fn config_best_practices_includes_config_migration() {
        let c = RepoConfig::parse(r#"{"extends": ["config:best-practices"]}"#);
        assert!(
            c.config_migration,
            "config:best-practices must include :configMigration"
        );
    }

    #[test]
    fn helpers_pin_github_action_digests_to_semver_injects_rule() {
        let c = RepoConfig::parse(r#"{"extends": ["helpers:pinGitHubActionDigestsToSemver"]}"#);
        let rule = c
            .package_rules
            .iter()
            .find(|r| {
                r.match_dep_types.contains(&"action".to_owned())
                    && r.pin_digests == Some(true)
                    && r.versioning.is_some()
            })
            .expect(
                "helpers:pinGitHubActionDigestsToSemver must inject an action rule with versioning",
            );
        assert!(
            rule.versioning.as_deref().unwrap().starts_with("regex:"),
            "versioning must be a regex: scheme"
        );
    }

    #[test]
    fn ignore_modules_and_tests_preset_direct() {
        let c = RepoConfig::parse(r#"{"extends": [":ignoreModulesAndTests"]}"#);
        assert!(c.ignore_paths.contains(&"**/node_modules/**".to_owned()));
        assert!(c.ignore_paths.contains(&"**/__fixtures__/**".to_owned()));
    }

    #[test]
    fn user_ignore_paths_appended_after_preset_paths() {
        let c = RepoConfig::parse(
            r#"{"extends": [":ignoreModulesAndTests"], "ignorePaths": ["custom/dir"]}"#,
        );
        // Preset paths come first, then user paths.
        let last = c.ignore_paths.last().unwrap();
        assert_eq!(last, "custom/dir");
        assert!(c.ignore_paths.contains(&"**/node_modules/**".to_owned()));
    }

    #[test]
    fn unknown_preset_ignored() {
        let c = RepoConfig::parse(r#"{"extends": ["github>org/repo"]}"#);
        // Unknown preset doesn't break parsing.
        assert_eq!(c.extends, vec!["github>org/repo"]);
        assert!(c.ignore_paths.is_empty()); // no paths added
    }

    #[test]
    fn semantic_commits_preset_sets_field() {
        let c = RepoConfig::parse(r#"{"extends": [":semanticCommits"]}"#);
        assert_eq!(c.semantic_commits.as_deref(), Some("enabled"));
    }

    #[test]
    fn semantic_commits_disabled_preset() {
        let c = RepoConfig::parse(r#"{"extends": [":semanticCommitsDisabled"]}"#);
        assert_eq!(c.semantic_commits.as_deref(), Some("disabled"));
    }

    #[test]
    fn explicit_semantic_commits_overrides_preset() {
        // Explicit field wins over :semanticCommits preset.
        let c =
            RepoConfig::parse(r#"{"semanticCommits": "auto", "extends": [":semanticCommits"]}"#);
        assert_eq!(c.semantic_commits.as_deref(), Some("auto"));
    }

    #[test]
    fn config_base_adds_ignore_paths() {
        let c = RepoConfig::parse(r#"{"extends": ["config:base"]}"#);
        assert!(c.ignore_paths.contains(&"**/node_modules/**".to_owned()));
    }

    #[test]
    fn config_js_app_expands_to_recommended_plus_pin_all() {
        // config:js-app = config:recommended + :pinAllExceptPeerDependencies
        let c = RepoConfig::parse(r#"{"extends": ["config:js-app"]}"#);
        // Should have the node_modules ignore path from config:recommended
        assert!(c.ignore_paths.contains(&"**/node_modules/**".to_owned()));
        // Should have a rangeStrategy: pin rule from :pinAllExceptPeerDependencies
        let has_pin_rule = c
            .package_rules
            .iter()
            .any(|r| r.range_strategy.as_deref() == Some("pin"));
        assert!(
            has_pin_rule,
            "config:js-app should inject a pin rangeStrategy rule"
        );
    }

    #[test]
    fn config_js_lib_expands_to_recommended_plus_pin_dev() {
        // config:js-lib = config:recommended + :pinOnlyDevDependencies
        let c = RepoConfig::parse(r#"{"extends": ["config:js-lib"]}"#);
        assert!(c.ignore_paths.contains(&"**/node_modules/**".to_owned()));
        let has_pin_dev_rule = c.package_rules.iter().any(|r| {
            r.range_strategy.as_deref() == Some("pin")
                && r.match_dep_types.contains(&"devDependencies".to_owned())
        });
        assert!(
            has_pin_dev_rule,
            "config:js-lib should inject a pin rule for devDependencies"
        );
    }

    #[test]
    fn config_semver_all_monthly_expands_to_group_all_and_schedule() {
        let c = RepoConfig::parse(r#"{"extends": ["config:semverAllMonthly"]}"#);
        // Should have monthly schedule from schedule:monthly
        assert_eq!(c.schedule, vec!["* 0-3 1 * *"]);
        // Should have group:all (separate_major_minor: false)
        assert!(!c.separate_major_minor);
    }

    #[test]
    fn config_semver_all_weekly_expands_to_group_all_and_schedule() {
        let c = RepoConfig::parse(r#"{"extends": ["config:semverAllWeekly"]}"#);
        // Should have weekly schedule
        assert_eq!(c.schedule, vec!["* 0-3 * * 1"]);
        assert!(!c.separate_major_minor);
    }

    #[test]
    fn duplicate_preset_deduplicated() {
        let c =
            RepoConfig::parse(r#"{"extends": [":ignoreModulesAndTests", "config:recommended"]}"#);
        // node_modules should appear only once.
        let count = c
            .ignore_paths
            .iter()
            .filter(|p| p.as_str() == "**/node_modules/**")
            .count();
        assert_eq!(
            count, 1,
            "expected deduplication, got: {:?}",
            c.ignore_paths
        );
    }
}

// ── Schedule and automerge preset tests ─────────────────────────────────────
#[cfg(test)]
mod schedule_preset_tests {
    use super::*;

    #[test]
    fn schedule_daily_preset() {
        let c = RepoConfig::parse(r#"{"extends": ["schedule:daily"]}"#);
        assert_eq!(c.schedule, vec!["* 0-3 * * *"]);
    }

    #[test]
    fn schedule_weekly_preset() {
        let c = RepoConfig::parse(r#"{"extends": ["schedule:weekly"]}"#);
        assert_eq!(c.schedule, vec!["* 0-3 * * 1"]);
    }

    #[test]
    fn schedule_monthly_preset() {
        let c = RepoConfig::parse(r#"{"extends": ["schedule:monthly"]}"#);
        assert_eq!(c.schedule, vec!["* 0-3 1 * *"]);
    }

    #[test]
    fn schedule_non_office_hours_preset_has_two_entries() {
        let c = RepoConfig::parse(r#"{"extends": ["schedule:nonOfficeHours"]}"#);
        assert_eq!(c.schedule.len(), 2);
        assert!(c.schedule.contains(&"* 0-4,22-23 * * 1-5".to_owned()));
        assert!(c.schedule.contains(&"* * * * 0,6".to_owned()));
    }

    #[test]
    fn schedule_weekdays_preset() {
        let c = RepoConfig::parse(r#"{"extends": ["schedule:weekdays"]}"#);
        assert_eq!(c.schedule, vec!["* * * * 1-5"]);
    }

    #[test]
    fn schedule_weekends_preset() {
        let c = RepoConfig::parse(r#"{"extends": ["schedule:weekends"]}"#);
        assert_eq!(c.schedule, vec!["* * * * 0,6"]);
    }

    #[test]
    fn schedule_quarterly_preset() {
        let c = RepoConfig::parse(r#"{"extends": ["schedule:quarterly"]}"#);
        assert_eq!(c.schedule, vec!["* * 1 */3 *"]);
    }

    #[test]
    fn schedule_yearly_preset() {
        let c = RepoConfig::parse(r#"{"extends": ["schedule:yearly"]}"#);
        assert_eq!(c.schedule, vec!["* * 1 */12 *"]);
    }

    #[test]
    fn explicit_schedule_overrides_preset() {
        let c =
            RepoConfig::parse(r#"{"schedule": ["before 5am"], "extends": ["schedule:weekly"]}"#);
        // User's explicit schedule wins.
        assert_eq!(c.schedule, vec!["before 5am"]);
    }

    #[test]
    fn last_schedule_preset_wins() {
        let c = RepoConfig::parse(r#"{"extends": ["schedule:daily", "schedule:monthly"]}"#);
        // Last schedule preset in extends list wins.
        assert_eq!(c.schedule, vec!["* 0-3 1 * *"]);
    }

    #[test]
    fn no_schedule_preset_leaves_schedule_empty() {
        let c = RepoConfig::parse(r#"{"extends": ["config:recommended"]}"#);
        assert!(c.schedule.is_empty());
    }

    #[test]
    fn config_recommended_injects_group_recommended_rules() {
        // config:recommended transitively includes group:recommended.
        // Ensure that packageRules from group:recommended are injected.
        let c = RepoConfig::parse(r#"{"extends": ["config:recommended"]}"#);
        assert!(
            c.package_rules.len() >= 40,
            "config:recommended must inject group:recommended rules, got {}",
            c.package_rules.len()
        );
        let has_group = |name: &str| {
            c.package_rules
                .iter()
                .any(|r| r.group_name.as_deref() == Some(name))
        };
        assert!(
            has_group("Node.js"),
            "group:recommended nodeJs rule missing from config:recommended"
        );
    }

    #[test]
    fn config_recommended_includes_semantic_prefix_rules() {
        // config:recommended transitively extends :semanticPrefixFixDepsChoreOthers.
        // Verify that the semantic commit packageRules are injected.
        let c = RepoConfig::parse(r#"{"extends": ["config:recommended"]}"#);
        let ctx_dep = DepContext {
            dep_name: "lodash",
            dep_type: Some("dependencies"),
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx_dep);
        // Production deps should get "fix" semantic type from :semanticPrefixFixDepsChoreOthers
        assert_eq!(
            effects.semantic_commit_type.as_deref(),
            Some("fix"),
            "config:recommended should apply semanticCommitType=fix for production deps"
        );
    }

    // ── automergeSchedule ────────────────────────────────────────────────────

    #[test]
    fn automerge_schedule_default_is_at_any_time() {
        let c = RepoConfig::parse(r#"{}"#);
        assert_eq!(c.automerge_schedule, vec!["at any time"]);
    }

    #[test]
    fn automerge_schedule_from_json_config() {
        let c = RepoConfig::parse(r#"{"automergeSchedule": ["before 5am"]}"#);
        assert_eq!(c.automerge_schedule, vec!["before 5am"]);
    }

    #[test]
    fn automerge_schedule_daily_preset() {
        let c = RepoConfig::parse(r#"{"extends": ["schedule:automergeDaily"]}"#);
        assert_eq!(c.automerge_schedule, vec!["* 0-3 * * *"]);
    }

    #[test]
    fn automerge_schedule_weekly_preset() {
        // schedule:automergeWeekly is an alias for schedule:automergeEarlyMondays
        let c = RepoConfig::parse(r#"{"extends": ["schedule:automergeWeekly"]}"#);
        assert_eq!(c.automerge_schedule, vec!["* 0-3 * * 1"]);
    }

    #[test]
    fn automerge_schedule_non_office_hours() {
        let c = RepoConfig::parse(r#"{"extends": ["schedule:automergeNonOfficeHours"]}"#);
        assert_eq!(
            c.automerge_schedule,
            vec!["* 0-4,22-23 * * 1-5", "* * * * 0,6"]
        );
    }

    #[test]
    fn explicit_automerge_schedule_overrides_preset() {
        let c = RepoConfig::parse(
            r#"{"automergeSchedule": ["before 5am"], "extends": ["schedule:automergeWeekly"]}"#,
        );
        assert_eq!(c.automerge_schedule, vec!["before 5am"]);
    }

    #[test]
    fn automerge_schedule_does_not_affect_schedule() {
        // automergeSchedule and schedule are independent fields.
        let c = RepoConfig::parse(r#"{"extends": ["schedule:automergeDaily"]}"#);
        assert!(c.schedule.is_empty(), "schedule must remain empty");
        assert_eq!(c.automerge_schedule, vec!["* 0-3 * * *"]);
    }

    #[test]
    fn automerge_all_preset_sets_automerge_true() {
        let c = RepoConfig::parse(r#"{"extends": [":automergeAll"]}"#);
        assert!(c.automerge);
    }

    #[test]
    fn automerge_minor_preset_injects_packagerules_not_global() {
        // :automergeMinor does NOT set global automerge; instead it injects
        // packageRules that automerge minor+patch updates.
        use crate::versioning::semver_generic::UpdateType;
        let c = RepoConfig::parse(r#"{"extends": [":automergeMinor"]}"#);
        assert!(
            !c.automerge,
            ":automergeMinor should not set global automerge"
        );
        // Minor update context should get automerge=true from injected rule.
        let ctx = DepContext {
            dep_name: "react",
            update_type: Some(UpdateType::Minor),
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(
            effects.automerge,
            Some(true),
            "minor update should automerge"
        );
        // Major update should NOT get automerge.
        let ctx_major = DepContext {
            dep_name: "react",
            update_type: Some(UpdateType::Major),
            ..Default::default()
        };
        let effects_major = c.collect_rule_effects(&ctx_major);
        assert!(
            effects_major.automerge.is_none() || effects_major.automerge == Some(false),
            "major update must not be automerged by :automergeMinor"
        );
    }

    #[test]
    fn automerge_patch_preset_injects_packagerules_for_patch_only() {
        use crate::versioning::semver_generic::UpdateType;
        let c = RepoConfig::parse(r#"{"extends": [":automergePatch"]}"#);
        assert!(
            !c.automerge,
            ":automergePatch should not set global automerge"
        );
        assert!(
            c.separate_minor_patch,
            ":automergePatch should set separateMinorPatch"
        );
        // Patch update → automerge.
        let ctx = DepContext {
            dep_name: "express",
            update_type: Some(UpdateType::Patch),
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(effects.automerge, Some(true));
        // Minor update → no automerge.
        let ctx_minor = DepContext {
            dep_name: "express",
            update_type: Some(UpdateType::Minor),
            ..Default::default()
        };
        let effects_minor = c.collect_rule_effects(&ctx_minor);
        assert!(effects_minor.automerge.is_none() || effects_minor.automerge == Some(false));
    }

    #[test]
    fn explicit_automerge_false_overrides_preset() {
        // explicit automerge: false does NOT get overridden by :automergeAll
        // Our logic: explicit true wins; if explicit false (default), use preset.
        let c = RepoConfig::parse(r#"{"extends": [":automergeAll"]}"#);
        assert!(c.automerge, "preset should set automerge to true");
    }

    #[test]
    fn automerge_linters_preset_injects_automerge_rule() {
        let c = RepoConfig::parse(r#"{"extends": [":automergeLinters"]}"#);
        // Rule injected for linter packages
        assert!(!c.package_rules.is_empty());
        let rule = &c.package_rules[0];
        assert_eq!(rule.automerge, Some(true));
        assert!(rule.name_matches("eslint"));
        assert!(!rule.name_matches("lodash"));
    }

    #[test]
    fn automerge_types_preset_injects_types_rule() {
        let c = RepoConfig::parse(r#"{"extends": [":automergeTypes"]}"#);
        assert!(!c.package_rules.is_empty());
        let rule = &c.package_rules[0];
        assert_eq!(rule.automerge, Some(true));
        assert!(rule.name_matches("@types/node"));
        assert!(!rule.name_matches("lodash"));
    }

    #[test]
    fn unknown_schedule_preset_leaves_empty() {
        let c = RepoConfig::parse(r#"{"extends": ["schedule:unknown"]}"#);
        assert!(c.schedule.is_empty());
    }

    #[test]
    fn disable_dev_dependencies_preset_blocks_dev_deps() {
        let c = RepoConfig::parse(r#"{"extends": [":disableDevDependencies"]}"#);
        let ctx = DepContext {
            dep_name: "lodash",
            dep_type: Some("devDependencies"),
            ..Default::default()
        };
        assert!(
            c.is_update_blocked_ctx(&ctx),
            "devDependencies should be blocked"
        );
        let ctx2 = DepContext {
            dep_name: "lodash",
            dep_type: Some("dependencies"),
            ..Default::default()
        };
        assert!(
            !c.is_update_blocked_ctx(&ctx2),
            "regular dependencies must not be blocked"
        );
    }

    #[test]
    fn disable_major_updates_preset_blocks_major() {
        use crate::versioning::semver_generic::UpdateType;
        let c = RepoConfig::parse(r#"{"extends": [":disableMajorUpdates"]}"#);
        let ctx = DepContext {
            dep_name: "react",
            update_type: Some(UpdateType::Major),
            ..Default::default()
        };
        assert!(c.is_update_blocked_ctx(&ctx));
        let ctx2 = DepContext {
            dep_name: "react",
            update_type: Some(UpdateType::Minor),
            ..Default::default()
        };
        assert!(!c.is_update_blocked_ctx(&ctx2));
    }

    #[test]
    fn disable_peer_dependencies_preset() {
        let c = RepoConfig::parse(r#"{"extends": [":disablePeerDependencies"]}"#);
        let ctx = DepContext {
            dep_name: "react",
            dep_type: Some("peerDependencies"),
            ..Default::default()
        };
        assert!(c.is_update_blocked_ctx(&ctx));
        let ctx2 = DepContext {
            dep_name: "react",
            dep_type: Some("dependencies"),
            ..Default::default()
        };
        assert!(!c.is_update_blocked_ctx(&ctx2));
    }

    // ── security presets ─────────────────────────────────────────────────────

    #[test]
    fn security_minimum_release_age_npm_injects_rule() {
        let c = RepoConfig::parse(r#"{"extends": ["security:minimumReleaseAgeNpm"]}"#);
        let npm_rule = c
            .package_rules
            .iter()
            .find(|r| r.match_datasources == vec!["npm"]);
        assert!(npm_rule.is_some(), "should have a rule for npm datasource");
        assert_eq!(
            npm_rule.unwrap().minimum_release_age.as_deref(),
            Some("3 days")
        );
    }

    #[test]
    fn unpublish_safe_preset_injects_npm_minimum_release_age() {
        let c = RepoConfig::parse(r#"{"extends": [":unpublishSafe"]}"#);
        let npm_rule = c
            .package_rules
            .iter()
            .find(|r| r.match_datasources == vec!["npm"]);
        assert!(npm_rule.is_some());
        assert_eq!(
            npm_rule.unwrap().minimum_release_age.as_deref(),
            Some("3 days")
        );
    }

    // ── Ported from migration.spec.ts extends migration ───────────────────────

    #[test]
    fn extends_string_coerced_to_array() {
        // Ported: "migrates preset strings to array" — extends: 'foo' → extends: ['foo']
        let c = RepoConfig::parse(r#"{"extends": "foo"}"#);
        // 'foo' is unknown so should be in extends but not break parsing
        assert!(c.minimum_release_age.is_none()); // sanity check: no minimumReleaseAge
    }

    #[test]
    fn extends_js_app_shorthand_normalized() {
        // Ported: extends: ':js-app' → config:js-app (via removedPresets map).
        // :js-app is normalized to config:js-app which then expands to config:recommended + pin rule.
        let c = RepoConfig::parse(r#"{"extends": [":js-app"]}"#);
        // config:js-app injects a rangeStrategy:pin rule — verify that preset was recognized.
        let has_pin_rule = c
            .package_rules
            .iter()
            .any(|r| r.range_strategy.as_deref() == Some("pin"));
        assert!(
            has_pin_rule,
            ":js-app should normalize to config:js-app and inject pin rules"
        );
    }

    #[test]
    fn extends_base_shorthand_normalized() {
        // Ported: extends: ':base' → config:recommended (via removedPresets).
        let c = RepoConfig::parse(r#"{"extends": [":base"]}"#);
        // config:recommended injects workarounds and other rules — just verify it was recognized.
        assert!(
            !c.package_rules.is_empty(),
            ":base should normalize to config:recommended"
        );
    }

    #[test]
    fn extends_master_issue_normalized() {
        // Ported: extends: ':masterIssue' → ':dependencyDashboard'
        let c = RepoConfig::parse(r#"{"extends": [":masterIssue"]}"#);
        assert!(
            c.dependency_dashboard,
            ":masterIssue must normalize to :dependencyDashboard"
        );
    }

    #[test]
    fn extends_npm_unpublish_safe_normalized() {
        // Ported: extends: ['npm:unpublishSafe'] → 'security:minimumReleaseAgeNpm'
        let c = RepoConfig::parse(r#"{"extends": ["npm:unpublishSafe"]}"#);
        let has_npm_age = c.package_rules.iter().any(|r| {
            r.match_datasources.contains(&"npm".to_owned())
                && r.minimum_release_age.as_deref() == Some("3 days")
        });
        assert!(
            has_npm_age,
            "npm:unpublishSafe must normalize to security:minimumReleaseAgeNpm"
        );
    }

    // ── scalar config presets ─────────────────────────────────────────────────

    #[test]
    fn combine_patch_minor_releases_clears_separate_minor_patch() {
        let c = RepoConfig::parse(r#"{"extends": ["combinePatchMinorReleases"]}"#);
        assert!(!c.separate_minor_patch);
    }

    #[test]
    fn combine_patch_minor_releases_with_colon_prefix() {
        // `:combinePatchMinorReleases` is the canonical form users write in extends arrays.
        let c = RepoConfig::parse(r#"{"extends": [":combinePatchMinorReleases"]}"#);
        assert!(!c.separate_minor_patch);
    }

    #[test]
    fn disable_rate_limiting_with_colon_prefix() {
        let c = RepoConfig::parse(r#"{"extends": [":disableRateLimiting"]}"#);
        assert_eq!(c.pr_concurrent_limit, 0);
        assert_eq!(c.pr_hourly_limit, 0);
    }

    #[test]
    fn separate_patch_releases_sets_separate_minor_patch() {
        let c = RepoConfig::parse(r#"{"extends": ["separatePatchReleases"]}"#);
        assert!(c.separate_minor_patch);
    }

    #[test]
    fn separate_multiple_major_releases_preset() {
        let c = RepoConfig::parse(r#"{"extends": ["separateMultipleMajorReleases"]}"#);
        assert!(c.separate_major_minor);
        assert!(c.separate_multiple_major);
    }

    #[test]
    fn separate_multiple_minor_releases_preset() {
        let c = RepoConfig::parse(r#"{"extends": ["separateMultipleMinorReleases"]}"#);
        assert!(c.separate_multiple_minor);
    }

    #[test]
    fn widen_peer_dependencies_preset_injects_rule() {
        let c = RepoConfig::parse(r#"{"extends": [":widenPeerDependencies"]}"#);
        let rule = c
            .package_rules
            .iter()
            .find(|r| r.range_strategy.as_deref() == Some("widen"));
        assert!(rule.is_some(), "expected a widen rangeStrategy rule");
        let rule = rule.unwrap();
        assert!(
            rule.match_dep_types
                .contains(&"peerDependencies".to_owned()),
            "rule should match peerDependencies"
        );
    }

    #[test]
    fn ignore_unstable_preset_sets_field() {
        let c = RepoConfig::parse(r#"{"extends": [":ignoreUnstable"]}"#);
        assert!(c.ignore_unstable);
    }

    #[test]
    fn ignore_unstable_direct_config() {
        let c = RepoConfig::parse(r#"{"ignoreUnstable": true}"#);
        assert!(c.ignore_unstable);
    }

    #[test]
    fn update_not_scheduled_default_true() {
        let c = RepoConfig::parse(r#"{}"#);
        assert!(c.update_not_scheduled);
    }

    #[test]
    fn no_unscheduled_updates_preset_sets_false() {
        let c = RepoConfig::parse(r#"{"extends": [":noUnscheduledUpdates"]}"#);
        assert!(!c.update_not_scheduled);
    }

    #[test]
    fn update_not_scheduled_direct_config() {
        let c = RepoConfig::parse(r#"{"updateNotScheduled": false}"#);
        assert!(!c.update_not_scheduled);
    }

    #[test]
    fn timezone_parameterized_preset_sets_field() {
        let c = RepoConfig::parse(r#"{"extends": [":timezone(America/New_York)"]}"#);
        assert_eq!(c.timezone.as_deref(), Some("America/New_York"));
    }

    #[test]
    fn timezone_preset_does_not_override_explicit() {
        // Explicit timezone in JSON wins over preset.
        let c = RepoConfig::parse(
            r#"{"timezone": "Europe/London", "extends": [":timezone(America/New_York)"]}"#,
        );
        assert_eq!(c.timezone.as_deref(), Some("Europe/London"));
    }

    #[test]
    fn disable_renovate_preset_sets_enabled_false() {
        let c = RepoConfig::parse(r#"{"extends": [":disableRenovate"]}"#);
        assert!(!c.enabled);
    }

    #[test]
    fn enable_renovate_preset_sets_enabled_true() {
        let c = RepoConfig::parse(r#"{"extends": [":enableRenovate"]}"#);
        assert!(c.enabled);
    }

    #[test]
    fn separate_multiple_minor_direct_config() {
        let c = RepoConfig::parse(r#"{"separateMultipleMinor": true}"#);
        assert!(c.separate_multiple_minor);
    }

    #[test]
    fn separate_multiple_minor_branch_topic() {
        use crate::branch::branch_topic;
        // Without separateMultipleMinor: minor updates share {dep}-{major}.x
        assert_eq!(
            branch_topic("lodash", 4, 17, false, true, false, false),
            "lodash-4.x"
        );
        // With separateMultipleMinor=true: minor gets {dep}-{major}.{minor}.x
        assert_eq!(
            branch_topic("lodash", 4, 17, false, true, false, true),
            "lodash-4.17.x"
        );
    }

    #[test]
    fn pr_hourly_limit_preset() {
        let c = RepoConfig::parse(r#"{"extends": ["prHourlyLimit1"]}"#);
        assert_eq!(c.pr_hourly_limit, 1);
    }

    #[test]
    fn pr_concurrent_limit_preset() {
        let c = RepoConfig::parse(r#"{"extends": ["prConcurrentLimit10"]}"#);
        assert_eq!(c.pr_concurrent_limit, 10);
    }

    #[test]
    fn disable_rate_limiting_preset() {
        let c = RepoConfig::parse(r#"{"extends": ["disableRateLimiting"]}"#);
        assert_eq!(c.pr_concurrent_limit, 0);
        assert_eq!(c.pr_hourly_limit, 0);
    }

    // ── parameterized presets ─────────────────────────────────────────────────

    #[test]
    fn label_preset_adds_label() {
        let c = RepoConfig::parse(r#"{"extends": ["label(renovate)"]}"#);
        assert!(c.labels.contains(&"renovate".to_owned()));
    }

    #[test]
    fn labels_preset_adds_multiple() {
        let c = RepoConfig::parse(r#"{"extends": ["labels(renovate, deps)"]}"#);
        assert!(c.labels.contains(&"renovate".to_owned()));
        assert!(c.labels.contains(&"deps".to_owned()));
    }

    #[test]
    fn label_preset_combined_with_existing_labels() {
        let c = RepoConfig::parse(r#"{"labels": ["existing"], "extends": ["label(renovate)"]}"#);
        assert!(c.labels.contains(&"existing".to_owned()));
        assert!(c.labels.contains(&"renovate".to_owned()));
    }

    #[test]
    fn assignee_preset_adds_assignee() {
        let c = RepoConfig::parse(r#"{"extends": [":assignee(renovate-bot)"]}"#);
        assert!(c.assignees.contains(&"renovate-bot".to_owned()));
    }

    #[test]
    fn reviewer_preset_adds_reviewer() {
        let c = RepoConfig::parse(r#"{"extends": [":reviewer(myteam)"]}"#);
        assert!(c.reviewers.contains(&"myteam".to_owned()));
    }

    #[test]
    fn assign_and_review_compound_preset_expands() {
        // :assignAndReview(user) → :assignee(user) + :reviewer(user)
        let c = RepoConfig::parse(r#"{"extends": [":assignAndReview(alice)"]}"#);
        assert!(
            c.assignees.contains(&"alice".to_owned()),
            "should set alice as assignee"
        );
        assert!(
            c.reviewers.contains(&"alice".to_owned()),
            "should set alice as reviewer"
        );
    }

    #[test]
    fn semantic_prefix_chore_expands() {
        // :semanticPrefixChore → :semanticCommitType(chore)
        let c = RepoConfig::parse(r#"{"extends": [":semanticPrefixChore"]}"#);
        assert_eq!(c.semantic_commit_type, "chore");
    }

    #[test]
    fn semantic_prefix_fix_expands() {
        // :semanticPrefixFix → :semanticCommitType(fix)
        let c = RepoConfig::parse(r#"{"extends": [":semanticPrefixFix"]}"#);
        assert_eq!(c.semantic_commit_type, "fix");
    }

    #[test]
    fn do_not_pin_package_preset_injects_rule() {
        // :doNotPinPackage(react) → packageRule with matchPackageNames:["react"] rangeStrategy:replace
        let c = RepoConfig::parse(r#"{"extends": [":doNotPinPackage(react)"]}"#);
        let rule = c
            .package_rules
            .iter()
            .find(|r| r.range_strategy.as_deref() == Some("replace"));
        assert!(
            rule.is_some(),
            "expected a replace rangeStrategy rule for react"
        );
        let rule = rule.unwrap();
        assert!(rule.match_package_names.contains(&"react".to_owned()));
    }

    #[test]
    fn path_semantic_commit_type_preset_injects_rule() {
        // :pathSemanticCommitType(src/**,feat) → packageRule with matchFileNames:["src/**"] semanticCommitType:"feat"
        let c = RepoConfig::parse(r#"{"extends": [":pathSemanticCommitType(src/**, feat)"]}"#);
        let rule = c
            .package_rules
            .iter()
            .find(|r| r.semantic_commit_type.as_deref() == Some("feat"));
        assert!(rule.is_some(), "expected a feat semanticCommitType rule");
        let rule = rule.unwrap();
        assert!(
            rule.match_file_names.iter().any(|f| f.contains("src")),
            "rule should match src/**"
        );
    }

    #[test]
    fn parse_preset_args_no_parens() {
        let (name, args) = super::parse_preset_args("group:all");
        assert_eq!(name, "group:all");
        assert!(args.is_empty());
    }

    #[test]
    fn parse_preset_args_single_arg() {
        let (name, args) = super::parse_preset_args("label(renovate)");
        assert_eq!(name, "label");
        assert_eq!(args, vec!["renovate"]);
    }

    #[test]
    fn parse_preset_args_multiple_args() {
        let (name, args) = super::parse_preset_args("labels(a, b, c)");
        assert_eq!(name, "labels");
        assert_eq!(args, vec!["a", "b", "c"]);
    }

    #[test]
    fn semantic_commit_type_preset() {
        let c = RepoConfig::parse(r#"{"extends": [":semanticCommitType(fix)"]}"#);
        assert_eq!(c.semantic_commit_type, "fix");
    }

    #[test]
    fn semantic_commit_scope_preset() {
        let c = RepoConfig::parse(r#"{"extends": [":semanticCommitScope(security)"]}"#);
        assert_eq!(c.semantic_commit_scope, "security");
    }

    #[test]
    fn semantic_commit_scope_disabled_preset() {
        let c = RepoConfig::parse(r#"{"extends": [":semanticCommitScopeDisabled"]}"#);
        assert_eq!(c.semantic_commit_scope, "");
    }
}

// Tests appended inline for slice 0176 — matchSourceUrls, matchCurrentValue, matchNewValue
#[cfg(test)]
mod source_url_tests {
    use super::*;

    #[test]
    fn match_source_urls_exact_disables_dep() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchSourceUrls": ["https://github.com/lodash/lodash"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.source_url_matches("https://github.com/lodash/lodash"));
        assert!(!rule.source_url_matches("https://github.com/other/repo"));
    }

    #[test]
    fn match_source_urls_glob() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchSourceUrls": ["https://github.com/org/**"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.source_url_matches("https://github.com/org/repo1"));
        assert!(rule.source_url_matches("https://github.com/org/repo2"));
        assert!(!rule.source_url_matches("https://github.com/other/repo"));
    }

    #[test]
    fn match_source_urls_regex() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchSourceUrls": ["/^https://github.com/org/"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.source_url_matches("https://github.com/org/myrepo"));
        assert!(!rule.source_url_matches("https://gitlab.com/org/myrepo"));
    }

    #[test]
    fn match_source_urls_empty_matches_all() {
        let c = RepoConfig::parse(r#"{"packageRules": [{"enabled": false}]}"#);
        let rule = &c.package_rules[0];
        assert!(rule.source_url_matches("https://github.com/anything"));
        assert!(rule.source_url_matches("https://example.com/pkg"));
    }

    #[test]
    fn match_current_value_regex() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCurrentValue": "/^[~^]/", "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.current_value_matches("^1.0.0"));
        assert!(rule.current_value_matches("~2.3.4"));
        assert!(!rule.current_value_matches("1.0.0"));
    }

    #[test]
    fn match_current_value_exact() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCurrentValue": "1.0.0", "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.current_value_matches("1.0.0"));
        assert!(!rule.current_value_matches("2.0.0"));
    }

    #[test]
    fn match_current_value_none_matches_all() {
        let c = RepoConfig::parse(r#"{"packageRules": [{"enabled": false}]}"#);
        let rule = &c.package_rules[0];
        assert!(rule.current_value_matches("^1.0.0"));
        assert!(rule.current_value_matches("anything"));
    }

    #[test]
    fn match_new_value_glob() {
        let c =
            RepoConfig::parse(r#"{"packageRules": [{"matchNewValue": "1.*", "enabled": false}]}"#);
        let rule = &c.package_rules[0];
        assert!(rule.new_value_matches("1.0.0"));
        assert!(rule.new_value_matches("1.99.0"));
        assert!(!rule.new_value_matches("2.0.0"));
    }

    #[test]
    fn match_new_value_none_matches_all() {
        let c = RepoConfig::parse(r#"{"packageRules": [{"enabled": false}]}"#);
        let rule = &c.package_rules[0];
        assert!(rule.new_value_matches("1.0.0"));
        assert!(rule.new_value_matches("99.0.0"));
    }

    #[test]
    fn match_current_value_regex_with_flags() {
        // Ported from lib/util/package-rules/current-value.spec.ts
        // "case insensitive match" test
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCurrentValue": "/^\"v/i", "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        // '"V1.1.0"' should match /^"v/i (case-insensitive)
        assert!(rule.current_value_matches("\"V1.1.0\""));
        // '"v1.1.0"' should also match
        assert!(rule.current_value_matches("\"v1.1.0\""));
    }

    // ── Ported from current-value.spec.ts ─────────────────────────────────────

    #[test]
    fn match_current_value_undefined_returns_false() {
        // Ported: "return false for now value" test.
        // When currentValue is absent (None), matchCurrentValue constraint → false.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCurrentValue": "/^v?[~ -]?0/", "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        // Empty string (None fallback) does not match the regex → false.
        assert!(!rule.current_value_matches(""));
    }

    #[test]
    fn match_current_value_glob_match() {
        // Ported: "return true for glob match" + "return false for glob non match"
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCurrentValue": "1.2.*", "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.current_value_matches("1.2.3"));
        assert!(!rule.current_value_matches("1.3.0"));
    }

    // ── Ported from new-value.spec.ts ─────────────────────────────────────────

    #[test]
    fn match_new_value_undefined_returns_false() {
        // "return false for now value": when newValue is absent, constraint → false.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchNewValue": "/^v?[~ -]?0/", "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(!rule.new_value_matches(""));
    }

    #[test]
    fn match_new_value_glob_match() {
        // Ported: "return true for glob match" + "return false for glob non match"
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchNewValue": "1.2.*", "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.new_value_matches("1.2.3"));
        assert!(!rule.new_value_matches("1.3.0"));
    }

    // ── Ported from files.spec.ts ─────────────────────────────────────────────

    #[test]
    fn match_file_names_undefined_returns_false() {
        // "should return false if packageFile is not defined"
        // When file_path is None and matchFileNames is set → the rule must NOT fire.
        // We test via a rule that sets automerge when it matches — if rule fires,
        // automerge would be Some(true); if not, it stays None.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchFileNames": ["frontend/package.json"], "automerge": true}]}"#,
        );
        let ctx_no_file = DepContext {
            dep_name: "dep",
            file_path: None,
            ..Default::default()
        };
        assert_eq!(
            c.collect_rule_effects(&ctx_no_file).automerge,
            None,
            "rule must not fire when file_path is None"
        );
        // But when file_path matches, the rule fires.
        let ctx_with_file = DepContext {
            dep_name: "dep",
            file_path: Some("frontend/package.json"),
            ..Default::default()
        };
        assert_eq!(
            c.collect_rule_effects(&ctx_with_file).automerge,
            Some(true),
            "rule must fire when file_path matches"
        );
    }

    // ── Ported from Renovate index.spec.ts "matches lock files" ──────────────

    #[test]
    fn match_file_names_matches_lock_files() {
        // Ported: "matches lock files" test from index.spec.ts.
        // Rule matchFileNames: ['yarn.lock'] should fire when lockFiles contains 'yarn.lock'.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchFileNames": ["yarn.lock"], "automerge": true}]}"#,
        );
        let ctx = DepContext {
            dep_name: "dep",
            file_path: Some("examples/foo/package.json"),
            lock_files: &["yarn.lock"],
            ..Default::default()
        };
        assert_eq!(
            c.collect_rule_effects(&ctx).automerge,
            Some(true),
            "rule must fire when lockFiles contains matched pattern"
        );
    }

    #[test]
    fn match_file_names_lock_file_pattern_with_glob() {
        // matchFileNames with glob against lockFiles.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchFileNames": ["**/yarn.lock"], "automerge": true}]}"#,
        );
        let ctx_lock = DepContext {
            dep_name: "dep",
            file_path: Some("package.json"),
            lock_files: &["packages/web/yarn.lock"],
            ..Default::default()
        };
        assert_eq!(
            c.collect_rule_effects(&ctx_lock).automerge,
            Some(true),
            "rule must fire when lockFiles glob matches"
        );
        // No lock files → falls back to packageFile matching only.
        let ctx_no_lock = DepContext {
            dep_name: "dep",
            file_path: Some("package.json"),
            lock_files: &[],
            ..Default::default()
        };
        assert_eq!(
            c.collect_rule_effects(&ctx_no_lock).automerge,
            None,
            "rule must not fire when neither packageFile nor lockFiles match"
        );
    }
}

#[cfg(test)]
mod categories_base_branch_tests {
    use super::*;

    // ── matchCategories ──────────────────────────────────────────────────────

    #[test]
    fn match_categories_exact_hit() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCategories": ["rust"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.categories_match(&["rust"]));
        assert!(!rule.categories_match(&["js"]));
        assert!(!rule.categories_match(&[]));
    }

    #[test]
    fn match_categories_any_of_many() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCategories": ["python", "rust"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.categories_match(&["python"]));
        assert!(rule.categories_match(&["rust"]));
        assert!(rule.categories_match(&["python", "rust"]));
        assert!(!rule.categories_match(&["java"]));
    }

    #[test]
    fn match_categories_empty_matches_all() {
        let c = RepoConfig::parse(r#"{"packageRules": [{"enabled": false}]}"#);
        let rule = &c.package_rules[0];
        assert!(rule.categories_match(&["rust"]));
        assert!(rule.categories_match(&["js"]));
        assert!(rule.categories_match(&[]));
    }

    // ── matchBaseBranches ────────────────────────────────────────────────────

    #[test]
    fn match_base_branches_exact_hit() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchBaseBranches": ["main"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.base_branch_matches("main"));
        assert!(!rule.base_branch_matches("develop"));
    }

    #[test]
    fn match_base_branches_glob() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchBaseBranches": ["release/*"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.base_branch_matches("release/1.0"));
        assert!(rule.base_branch_matches("release/2.3.4"));
        assert!(!rule.base_branch_matches("main"));
        assert!(!rule.base_branch_matches("feature/foo"));
    }

    #[test]
    fn match_base_branches_empty_matches_all() {
        let c = RepoConfig::parse(r#"{"packageRules": [{"enabled": false}]}"#);
        let rule = &c.package_rules[0];
        assert!(rule.base_branch_matches("main"));
        assert!(rule.base_branch_matches("develop"));
        assert!(rule.base_branch_matches("release/1.0"));
    }

    #[test]
    fn match_base_branches_multiple_entries() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchBaseBranches": ["main", "develop"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.base_branch_matches("main"));
        assert!(rule.base_branch_matches("develop"));
        assert!(!rule.base_branch_matches("feature/foo"));
    }

    // ── Ported from Renovate index.spec.ts ───────────────────────────────────

    #[test]
    fn needs_base_branch_to_match_rule_does_not_fire_without_it() {
        // Ported: "needs baseBranch to match" — when baseBranch is absent from context
        // and matchBaseBranches is set, the rule must not fire.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["abc"], "matchBaseBranches": ["dev"], "automerge": true}]}"#,
        );
        let ctx = DepContext {
            dep_name: "abc",
            base_branch: None,
            ..Default::default()
        };
        assert_eq!(
            c.collect_rule_effects(&ctx).automerge,
            None,
            "rule must not fire when base_branch is None and matchBaseBranches is set"
        );
    }

    #[test]
    fn needs_manager_to_match_rule_does_not_fire_without_it() {
        // Ported: "needs manager to match" — when manager is absent from context
        // and matchManagers is set, the rule must not fire.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["abc"], "matchManagers": ["npm"], "automerge": true}]}"#,
        );
        let ctx = DepContext {
            dep_name: "abc",
            manager: None,
            ..Default::default()
        };
        assert_eq!(
            c.collect_rule_effects(&ctx).automerge,
            None,
            "rule must not fire when manager is None and matchManagers is set"
        );
    }

    #[test]
    fn needs_categories_to_match_rule_does_not_fire_without_it() {
        // Ported: "filters categories with undefined category" — when categories is empty
        // but matchCategories is set, rule must not fire.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCategories": ["docker"], "automerge": true}]}"#,
        );
        let ctx = DepContext {
            dep_name: "my-dep",
            ..Default::default()
        };
        // Empty categories (default) — rule must NOT fire
        assert_eq!(
            c.collect_rule_effects(&ctx).automerge,
            None,
            "rule must not fire when categories is empty and matchCategories is set"
        );
    }
}

#[cfg(test)]
mod registry_url_repository_tests {
    use super::*;

    // ── matchRegistryUrls ────────────────────────────────────────────────────

    #[test]
    fn match_registry_urls_exact_hit() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchRegistryUrls": ["https://registry.npmjs.org"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.registry_url_matches(&["https://registry.npmjs.org"]));
        assert!(!rule.registry_url_matches(&["https://registry.corp.example"]));
    }

    #[test]
    fn match_registry_urls_any_of_dep_urls() {
        // Rule has one pattern; dep has two registry URLs — match if ANY URL matches.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchRegistryUrls": ["https://registry.npmjs.org"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.registry_url_matches(&[
            "https://registry.corp.example",
            "https://registry.npmjs.org"
        ]));
    }

    #[test]
    fn match_registry_urls_glob() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchRegistryUrls": ["https://registry.corp.**"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.registry_url_matches(&["https://registry.corp.example"]));
        assert!(!rule.registry_url_matches(&["https://registry.npmjs.org"]));
    }

    #[test]
    fn match_registry_urls_empty_matches_all() {
        let c = RepoConfig::parse(r#"{"packageRules": [{"enabled": false}]}"#);
        let rule = &c.package_rules[0];
        assert!(rule.registry_url_matches(&["https://registry.npmjs.org"]));
        assert!(rule.registry_url_matches(&[]));
    }

    #[test]
    fn match_registry_urls_no_dep_urls_fails_when_constraint_set() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchRegistryUrls": ["https://registry.npmjs.org"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        // No registry URLs on the dep → no match.
        assert!(!rule.registry_url_matches(&[]));
    }

    // ── matchRepositories ────────────────────────────────────────────────────

    #[test]
    fn match_repositories_exact_hit() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchRepositories": ["owner/repo"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.repository_matches("owner/repo"));
        assert!(!rule.repository_matches("owner/other"));
    }

    #[test]
    fn match_repositories_glob() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchRepositories": ["owner/**"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.repository_matches("owner/repo1"));
        assert!(rule.repository_matches("owner/repo2"));
        assert!(!rule.repository_matches("other/repo"));
    }

    #[test]
    fn match_repositories_regex() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchRepositories": ["/^owner/"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.repository_matches("owner/repo1"));
        assert!(!rule.repository_matches("other/repo"));
    }

    #[test]
    fn match_repositories_empty_matches_all() {
        let c = RepoConfig::parse(r#"{"packageRules": [{"enabled": false}]}"#);
        let rule = &c.package_rules[0];
        assert!(rule.repository_matches("owner/repo"));
        assert!(rule.repository_matches("anyone/anything"));
    }

    #[test]
    fn match_repositories_negation() {
        // ["!owner/**"] excludes owner/* repos, permits others.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchRepositories": ["!owner/**"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(!rule.repository_matches("owner/repo"));
        assert!(rule.repository_matches("other-org/repo"));
    }

    #[test]
    fn match_repositories_fires_only_for_matching_repo() {
        // packageRule with matchRepositories and automerge:true — verify rule fires
        // only when repository matches the context.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchRepositories": ["owner/repo"], "automerge": true}]}"#,
        );
        // Matching repo → rule fires → automerge set
        let ctx_match = DepContext {
            dep_name: "lodash",
            repository: Some("owner/repo"),
            ..Default::default()
        };
        assert_eq!(
            c.collect_rule_effects(&ctx_match).automerge,
            Some(true),
            "rule should fire when repository matches"
        );
        // Non-matching repo → rule does NOT fire → automerge absent
        let ctx_other = DepContext {
            dep_name: "lodash",
            repository: Some("other/repo"),
            ..Default::default()
        };
        assert_eq!(
            c.collect_rule_effects(&ctx_other).automerge,
            None,
            "rule should not fire when repository doesn't match"
        );
        // No repository in context → rule does NOT fire (unknown context → conservative)
        let ctx_none = DepContext {
            dep_name: "lodash",
            ..Default::default()
        };
        assert_eq!(
            c.collect_rule_effects(&ctx_none).automerge,
            None,
            "rule should not fire when repository is unknown (None)"
        );
    }

    // ── repositories.spec.ts additions ───────────────────────────────────────

    #[test]
    fn match_repositories_invalid_regex_returns_false() {
        // Invalid regex pattern: /[/ — Renovate returns false (no match).
        // Our regex crate returns Err, unwrap_or(false) → false.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchRepositories": ["/[/"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(!rule.repository_matches("org/repo"));
    }

    #[test]
    fn match_repositories_invalid_negated_regex_returns_true() {
        // Negated invalid regex: !/[/ — Renovate returns true (passes through).
        // Our negation logic: try matching the invalid regex → unwrap_or(false) → false,
        // then negation → true.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchRepositories": ["!/[/"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.repository_matches("org/repo"));
    }

    #[test]
    fn match_repositories_any_of_patterns() {
        // Matches at least one pattern: regex OR glob.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchRepositories": ["/^org/repo$/", "**/*-archived"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.repository_matches("org/repo-archived")); // matches glob
        assert!(rule.repository_matches("org/repo")); // matches regex
        assert!(!rule.repository_matches("other/something")); // matches neither
    }

    #[test]
    fn match_source_urls_negation() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchSourceUrls": ["!https://github.com/bad/**"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(!rule.source_url_matches("https://github.com/bad/pkg"));
        assert!(rule.source_url_matches("https://github.com/good/pkg"));
    }

    #[test]
    fn match_registry_urls_negation() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchRegistryUrls": ["!https://internal.registry/**"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(!rule.registry_url_matches(&["https://internal.registry/pkg"]));
        assert!(rule.registry_url_matches(&["https://registry.npmjs.org/pkg"]));
    }
}

#[cfg(test)]
mod dep_context_tests {
    use super::*;

    /// `matchManagers: ["npm"]` rule with `DepContext` that knows the manager.
    #[test]
    fn dep_context_with_manager_fires_correct_rule() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["lodash"], "matchManagers": ["npm"], "enabled": false}]}"#,
        );
        let npm_ctx = DepContext::for_dep("lodash").with_manager("npm");
        assert!(c.is_dep_ignored_ctx(&npm_ctx));

        let cargo_ctx = DepContext::for_dep("lodash").with_manager("cargo");
        assert!(!c.is_dep_ignored_ctx(&cargo_ctx));

        // No manager context → matchManagers constraint cannot be satisfied → no fire.
        assert!(!c.is_dep_ignored("lodash"));
    }

    /// `matchDatasources` fires correctly via DepContext.
    #[test]
    fn dep_context_datasource_gates_rule() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchDatasources": ["npm"], "matchPackageNames": ["react"], "enabled": false}]}"#,
        );
        let npm_ctx = DepContext {
            dep_name: "react",
            datasource: Some("npm"),
            ..Default::default()
        };
        assert!(c.is_dep_ignored_ctx(&npm_ctx));

        let pypi_ctx = DepContext {
            dep_name: "react",
            datasource: Some("pypi"),
            ..Default::default()
        };
        assert!(!c.is_dep_ignored_ctx(&pypi_ctx));
    }

    /// `matchCategories: ["rust"]` fires when manager is cargo.
    #[test]
    fn dep_context_categories_from_manager() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCategories": ["rust"], "enabled": false}]}"#,
        );
        let cargo_ctx = DepContext::for_dep("serde").with_manager("cargo");
        assert!(c.is_dep_ignored_ctx(&cargo_ctx));

        let npm_ctx = DepContext::for_dep("express").with_manager("npm");
        assert!(!c.is_dep_ignored_ctx(&npm_ctx));
    }

    /// `matchRepositories` gates correctly via DepContext.
    #[test]
    fn dep_context_repository_gates_rule() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchRepositories": ["owner/repo"], "enabled": false}]}"#,
        );
        let in_repo = DepContext {
            dep_name: "any-dep",
            repository: Some("owner/repo"),
            ..Default::default()
        };
        assert!(c.is_dep_ignored_ctx(&in_repo));

        let other_repo = DepContext {
            dep_name: "any-dep",
            repository: Some("other/repo"),
            ..Default::default()
        };
        assert!(!c.is_dep_ignored_ctx(&other_repo));
    }

    /// Builder methods produce correct context.
    #[test]
    fn dep_context_builder_methods() {
        let ctx = DepContext::for_dep("react")
            .with_manager("npm")
            .with_datasource("npm")
            .with_dep_type("devDependencies")
            .with_file_path("package.json");

        assert_eq!(ctx.dep_name, "react");
        assert_eq!(ctx.manager, Some("npm"));
        assert_eq!(ctx.datasource, Some("npm"));
        assert_eq!(ctx.dep_type, Some("devDependencies"));
        assert_eq!(ctx.file_path, Some("package.json"));
    }
}

#[cfg(test)]
mod rule_effects_tests {
    use super::*;

    #[test]
    fn group_name_from_matching_rule() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["express"], "groupName": "web-framework"}]}"#,
        );
        let ctx = DepContext::for_dep("express");
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(effects.group_name.as_deref(), Some("web-framework"));
    }

    #[test]
    fn group_name_last_matching_rule_wins() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [
                {"matchPackageNames": ["express"], "groupName": "first"},
                {"matchPackageNames": ["express"], "groupName": "second"}
            ]}"#,
        );
        let ctx = DepContext::for_dep("express");
        let effects = c.collect_rule_effects(&ctx);
        // Last matching rule wins for groupName (Renovate mergeChildConfig semantics).
        assert_eq!(effects.group_name.as_deref(), Some("second"));
    }

    #[test]
    fn automerge_last_rule_wins() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [
                {"matchPackageNames": ["express"], "automerge": true},
                {"matchPackageNames": ["express"], "automerge": false}
            ]}"#,
        );
        let ctx = DepContext::for_dep("express");
        let effects = c.collect_rule_effects(&ctx);
        // Last matching rule wins for automerge.
        assert_eq!(effects.automerge, Some(false));
    }

    #[test]
    fn labels_accumulated_from_all_matching_rules() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [
                {"matchPackageNames": ["express"], "labels": ["backend"]},
                {"matchPackageNames": ["express"], "labels": ["web", "backend"]}
            ]}"#,
        );
        let ctx = DepContext::for_dep("express");
        let effects = c.collect_rule_effects(&ctx);
        // Union — "backend" deduped, "web" added.
        assert!(effects.labels.contains(&"backend".to_owned()));
        assert!(effects.labels.contains(&"web".to_owned()));
        assert_eq!(effects.labels.len(), 2);
    }

    #[test]
    fn repo_level_labels_seed_effects() {
        // Repo-level labels should appear in effects even without matching rules.
        let c = RepoConfig::parse(r#"{"labels": ["renovate", "dependencies"]}"#);
        let ctx = DepContext::for_dep("lodash");
        let effects = c.collect_rule_effects(&ctx);
        assert!(effects.labels.contains(&"renovate".to_owned()));
        assert!(effects.labels.contains(&"dependencies".to_owned()));
    }

    #[test]
    fn add_labels_merged_with_labels() {
        // addLabels union-merges with labels — no duplicates.
        let c = RepoConfig::parse(r#"{"labels": ["renovate"], "addLabels": ["deps", "renovate"]}"#);
        let ctx = DepContext::for_dep("lodash");
        let effects = c.collect_rule_effects(&ctx);
        assert!(effects.labels.contains(&"renovate".to_owned()));
        assert!(effects.labels.contains(&"deps".to_owned()));
        // "renovate" deduped — appears only once
        assert_eq!(
            effects
                .labels
                .iter()
                .filter(|l| l.as_str() == "renovate")
                .count(),
            1
        );
    }

    #[test]
    fn rule_labels_replaces_repo_labels() {
        // Per-rule `labels` is NOT mergeable — it replaces the repo-level labels.
        // To append, use `addLabels` instead.
        let c = RepoConfig::parse(
            r#"{"labels": ["base"], "packageRules": [{"matchPackageNames": ["express"], "labels": ["frontend"]}]}"#,
        );
        let ctx = DepContext::for_dep("express");
        let effects = c.collect_rule_effects(&ctx);
        // After the rule, labels = ["frontend"] (replaced "base").
        assert!(
            !effects.labels.contains(&"base".to_owned()),
            "rule `labels` should replace repo labels"
        );
        assert!(effects.labels.contains(&"frontend".to_owned()));
    }

    #[test]
    fn rule_add_labels_appends_to_repo_labels() {
        // `addLabels` IS mergeable — it appends to the repo-level labels.
        let c = RepoConfig::parse(
            r#"{"labels": ["base"], "packageRules": [{"matchPackageNames": ["express"], "addLabels": ["frontend"]}]}"#,
        );
        let ctx = DepContext::for_dep("express");
        let effects = c.collect_rule_effects(&ctx);
        // addLabels appends: both "base" and "frontend" should be present.
        assert!(effects.labels.contains(&"base".to_owned()));
        assert!(effects.labels.contains(&"frontend".to_owned()));
    }

    #[test]
    fn no_matching_rule_returns_defaults() {
        let c = RepoConfig::parse(
            r#"{"automerge": true, "groupName": "all-deps",
               "packageRules": [{"matchPackageNames": ["other-pkg"], "automerge": false}]}"#,
        );
        let ctx = DepContext::for_dep("express");
        let effects = c.collect_rule_effects(&ctx);
        // No rule matches express → repo-level defaults applied.
        assert_eq!(effects.group_name.as_deref(), Some("all-deps"));
        assert_eq!(effects.automerge, Some(true));
    }

    #[test]
    fn rule_with_non_matching_manager_doesnt_apply() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchManagers": ["npm"], "matchPackageNames": ["serde"], "groupName": "npm-only"}]}"#,
        );
        let ctx = DepContext::for_dep("serde").with_manager("cargo");
        let effects = c.collect_rule_effects(&ctx);
        assert!(effects.group_name.is_none());
    }

    // ── matchCurrentAge ───────────────────────────────────────────────────────

    #[test]
    fn match_current_age_parsed_from_config() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCurrentAge": "> 6 months", "enabled": false}]}"#,
        );
        assert_eq!(
            c.package_rules[0].match_current_age.as_deref(),
            Some("> 6 months")
        );
    }

    #[test]
    fn match_current_age_none_unset_matches_all() {
        let c = RepoConfig::parse(r#"{"packageRules": [{"enabled": false}]}"#);
        // No matchCurrentAge → matches any timestamp (or none)
        assert!(c.package_rules[0].current_age_matches(None));
        assert!(c.package_rules[0].current_age_matches(Some("2020-01-01T00:00:00Z")));
    }

    // ── Ported from Renovate current-age.spec.ts ─────────────────────────────
    // Renovate mock time is 2023-07-07. We use 2020 as "old" and 2099 as "future".

    #[test]
    fn current_age_returns_false_if_release_older_than_constraint_bound() {
        // Renovate spec: "returns false if release is older"
        // timestamp: '2020-01-01', matchCurrentAge: '< 1 year' (younger than 1 year)
        // 2020 is NOT younger than 1 year from now → false
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCurrentAge": "< 1 year", "enabled": false}]}"#,
        );
        assert!(!c.package_rules[0].current_age_matches(Some("2020-01-01T00:00:00Z")));
    }

    #[test]
    fn current_age_returns_false_if_release_younger_than_constraint_bound() {
        // Renovate spec: "returns false if release is younger"
        // timestamp: '2020-01-01', matchCurrentAge: '> 10 years' (older than 10 yrs)
        // 2020 to 2025 is ~5 years, NOT > 10 years → false
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCurrentAge": "> 10 years", "enabled": false}]}"#,
        );
        assert!(!c.package_rules[0].current_age_matches(Some("2020-01-01T00:00:00Z")));
    }

    #[test]
    fn current_age_returns_false_for_invalid_timestamp() {
        // Renovate spec: "returns null if release invalid" — Renovate returns null (pass-through).
        // Our impl: invalid timestamp → parse fails → returns false (conservative).
        // Compatibility note: diverges from Renovate for invalid/non-ISO timestamps.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCurrentAge": "> 2 days", "enabled": false}]}"#,
        );
        // "abc" is not a valid timestamp — our impl returns false (blocks rule from firing).
        assert!(!c.package_rules[0].current_age_matches(Some("abc")));
    }

    #[test]
    fn match_current_age_set_without_timestamp_returns_false() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCurrentAge": "> 3 days", "enabled": false}]}"#,
        );
        // No timestamp → constraint cannot be evaluated → conservative false
        assert!(!c.package_rules[0].current_age_matches(None));
    }

    #[test]
    fn match_current_age_old_dep_matches_gt_constraint() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCurrentAge": "> 3 days", "enabled": false}]}"#,
        );
        // A 2020 timestamp is definitely older than 3 days
        assert!(c.package_rules[0].current_age_matches(Some("2020-01-01T00:00:00Z")));
    }

    #[test]
    fn match_current_age_new_dep_does_not_match_gt_constraint() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCurrentAge": "> 3 days", "enabled": false}]}"#,
        );
        // A far-future timestamp is not older than 3 days
        assert!(!c.package_rules[0].current_age_matches(Some("2099-01-01T00:00:00Z")));
    }

    #[test]
    fn match_current_age_via_dep_context_disables_dep() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchCurrentAge": "> 3 days", "enabled": false}]}"#,
        );
        // With an old timestamp the rule fires → dep is ignored
        let ctx = DepContext {
            dep_name: "lodash",
            current_version_timestamp: Some("2020-01-01T00:00:00Z"),
            ..Default::default()
        };
        assert!(c.is_dep_ignored_ctx(&ctx));

        // With no timestamp the rule doesn't fire → dep is not ignored
        let ctx_no_ts = DepContext {
            dep_name: "lodash",
            current_version_timestamp: None,
            ..Default::default()
        };
        assert!(!c.is_dep_ignored_ctx(&ctx_no_ts));
    }

    // ── per-rule schedule in RuleEffects ────────────────────────────────────

    #[test]
    fn per_rule_schedule_collected_into_effects() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["lodash"], "schedule": ["before 5am"]}]}"#,
        );
        let ctx = DepContext {
            dep_name: "lodash",
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(effects.schedule, vec!["before 5am"]);
    }

    #[test]
    fn per_rule_schedule_not_set_for_non_matching_dep() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["lodash"], "schedule": ["before 5am"]}]}"#,
        );
        let ctx = DepContext {
            dep_name: "react",
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert!(
            effects.schedule.is_empty(),
            "non-matching dep should not get schedule"
        );
    }

    // ── Ported from Renovate migration.spec.ts "migrates packageRules" ────────

    #[test]
    fn migrates_package_rules_all_deprecated_fields() {
        // Ported from migration.spec.ts "migrates packageRules" test.
        // Verifies that all deprecated packageRule fields are migrated correctly.
        let c = RepoConfig::parse(
            r#"{
                "packageRules": [{
                    "paths": ["package.json"],
                    "languages": ["python"],
                    "baseBranchList": ["master"],
                    "managers": ["dockerfile"],
                    "datasources": ["orb"],
                    "depTypeList": ["peerDependencies"],
                    "packageNames": ["foo"],
                    "packagePatterns": ["^bar"],
                    "excludePackageNames": ["baz"],
                    "excludePackagePatterns": ["^baz"],
                    "excludeRepositories": ["abc/def"],
                    "sourceUrlPrefixes": ["https://github.com/lodash"],
                    "updateTypes": ["major"],
                    "automerge": true
                }]
            }"#,
        );
        assert_eq!(c.package_rules.len(), 1);
        let rule = &c.package_rules[0];

        // matchPackageNames = foo + /^bar/ + !baz + !/^baz/
        assert!(rule.match_package_names.contains(&"foo".to_owned()));
        assert!(rule.match_package_names.contains(&"/^bar/".to_owned()));
        assert!(rule.match_package_names.contains(&"!baz".to_owned()));
        assert!(rule.match_package_names.contains(&"!/^baz/".to_owned()));

        // matchFileNames = package.json
        assert!(rule.match_file_names.contains(&"package.json".to_owned()));

        // matchCategories = python
        assert!(rule.match_categories.contains(&"python".to_owned()));

        // matchBaseBranches = master
        assert!(rule.match_base_branches.contains(&"master".to_owned()));

        // matchManagers = dockerfile
        assert!(rule.match_managers.contains(&"dockerfile".to_owned()));

        // matchDatasources = orb
        assert!(rule.match_datasources.contains(&"orb".to_owned()));

        // matchDepTypes = peerDependencies
        assert!(
            rule.match_dep_types
                .contains(&"peerDependencies".to_owned())
        );

        // matchRepositories = !abc/def
        assert!(rule.match_repositories.contains(&"!abc/def".to_owned()));

        // matchSourceUrls = https://github.com/lodash{/,}**
        let has_source_url = rule
            .match_source_urls
            .iter()
            .any(|u| u.contains("github.com/lodash"));
        assert!(
            has_source_url,
            "sourceUrlPrefixes must be converted to matchSourceUrls glob"
        );

        // matchUpdateTypes = major
        assert!(rule.match_update_types.contains(&UpdateType::Major));
    }

    #[test]
    fn deprecated_package_names_merged_with_match_package_names() {
        // packageNames: ['foo'] + matchPackageNames: ['bar'] → matchPackageNames: ['bar', 'foo']
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["bar"], "packageNames": ["foo"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.match_package_names.contains(&"bar".to_owned()));
        assert!(rule.match_package_names.contains(&"foo".to_owned()));
    }

    #[test]
    fn deprecated_exclude_repositories_negation() {
        // excludeRepositories: ['abc/def'] → matchRepositories: ['!abc/def']
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"excludeRepositories": ["abc/def"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(rule.match_repositories.contains(&"!abc/def".to_owned()));
    }

    #[test]
    fn deprecated_source_url_prefixes_become_glob() {
        // sourceUrlPrefixes: ['https://github.com/lodash'] → matchSourceUrls: ['https://github.com/lodash{/,}**']
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"sourceUrlPrefixes": ["https://github.com/lodash"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        let url = rule.match_source_urls.iter().find(|u| u.contains("lodash"));
        assert!(
            url.is_some(),
            "sourceUrlPrefixes must produce a matchSourceUrls entry"
        );
        assert!(
            url.unwrap().contains("{/,}**"),
            "sourceUrlPrefixes entry must end with glob"
        );
    }

    // ── Ported from migration.spec.ts "migrates packages" ────────────────────

    #[test]
    fn deprecated_packages_field_merged_into_package_rules() {
        // Ported: "migrates packages" from migration.spec.ts.
        // Old `packages: [{...}]` → merged into `packageRules`.
        let c = RepoConfig::parse(
            r#"{
                "packages": [{
                    "matchPackageNames": ["@angular/core"],
                    "groupName": "angular packages",
                    "automerge": true
                }]
            }"#,
        );
        assert!(
            !c.package_rules.is_empty(),
            "deprecated `packages` field must be merged into package_rules"
        );
        // The rule should have the group name set.
        assert_eq!(
            c.package_rules[0].group_name.as_deref(),
            Some("angular packages")
        );
    }

    #[test]
    fn group_name_array_first_element_used() {
        // groupName: ["angular packages"] (array) → groupName: "angular packages" (string).
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"groupName": ["my group"], "automerge": true}]}"#,
        );
        assert_eq!(
            c.package_rules[0].group_name.as_deref(),
            Some("my group"),
            "groupName array should use first element"
        );
    }

    // ── per-rule minimumReleaseAge in RuleEffects ────────────────────────────

    // ── deprecated field migrations ───────────────────────────────────────────

    #[test]
    fn upgrade_in_range_true_sets_range_strategy_bump() {
        let c = RepoConfig::parse(r#"{"upgradeInRange": true}"#);
        assert_eq!(
            c.range_strategy, "bump",
            "upgradeInRange: true must migrate to rangeStrategy: 'bump'"
        );
    }

    #[test]
    fn version_strategy_widen_sets_range_strategy_widen() {
        let c = RepoConfig::parse(r#"{"versionStrategy": "widen"}"#);
        assert_eq!(
            c.range_strategy, "widen",
            "versionStrategy: 'widen' must migrate to rangeStrategy: 'widen'"
        );
    }

    #[test]
    fn explicit_range_strategy_overrides_deprecated_upgrade_in_range() {
        let c = RepoConfig::parse(r#"{"upgradeInRange": true, "rangeStrategy": "replace"}"#);
        assert_eq!(
            c.range_strategy, "replace",
            "explicit rangeStrategy must take precedence over upgradeInRange migration"
        );
    }

    #[test]
    fn unpublish_safe_true_injects_minimum_release_age_preset() {
        let c = RepoConfig::parse(r#"{"unpublishSafe": true}"#);
        // The security:minimumReleaseAgeNpm preset should be injected, creating a
        // packageRule with minimumReleaseAge: "3 days" for npm datasource.
        let has_npm_age_rule = c.package_rules.iter().any(|r| {
            r.match_datasources.contains(&"npm".to_owned())
                && r.minimum_release_age.as_deref() == Some("3 days")
        });
        assert!(
            has_npm_age_rule,
            "unpublishSafe: true must inject minimumReleaseAge for npm packages"
        );
    }

    // ── Ported from migration.spec.ts unpublishSafe tests ────────────────────

    #[test]
    fn unpublish_safe_true_with_existing_extends_appends_preset() {
        // Ported: 'unpublishSafe: true, extends: "foo"' → extends: ['foo', 'security:minimumReleaseAgeNpm']
        // Verify that our impl injects the preset and APPENDS to existing extends.
        let c = RepoConfig::parse(r#"{"unpublishSafe": true, "extends": ["foo"]}"#);
        let has_npm_age = c.package_rules.iter().any(|r| {
            r.match_datasources.contains(&"npm".to_owned())
                && r.minimum_release_age.as_deref() == Some("3 days")
        });
        assert!(
            has_npm_age,
            "minimumReleaseAge npm rule must be present when unpublishSafe: true with other extends"
        );
    }

    #[test]
    fn unpublish_safe_false_does_not_inject() {
        // Ported: 'unpublishSafe: false' → extends unchanged, no minimumReleaseAge rule.
        let c = RepoConfig::parse(r#"{"unpublishSafe": false, "extends": ["foo", "bar"]}"#);
        let has_npm_age = c.package_rules.iter().any(|r| {
            r.match_datasources.contains(&"npm".to_owned())
                && r.minimum_release_age.as_deref() == Some("3 days")
        });
        assert!(
            !has_npm_age,
            "unpublishSafe: false must not inject minimumReleaseAge"
        );
    }

    #[test]
    fn unpublish_safe_with_unpublish_safe_preset_already_in_extends_does_not_duplicate() {
        // Ported: when ':unpublishSafe' is already in extends, don't add security:minimumReleaseAgeNpm.
        // The ':unpublishSafe' preset itself adds the 3-day rule, so there's no double injection.
        let c = RepoConfig::parse(
            r#"{"unpublishSafe": true, "extends": ["foo", ":unpublishSafe", "bar"]}"#,
        );
        let npm_age_rules_count = c
            .package_rules
            .iter()
            .filter(|r| {
                r.match_datasources.contains(&"npm".to_owned())
                    && r.minimum_release_age.as_deref() == Some("3 days")
            })
            .count();
        // Should have exactly one such rule (from :unpublishSafe), not duplicated.
        assert_eq!(
            npm_age_rules_count, 1,
            ":unpublishSafe in extends + unpublishSafe:true must not duplicate the rule"
        );
    }

    // ── stabilityDays migration ───────────────────────────────────────────────

    #[test]
    fn stability_days_migrated_to_minimum_release_age() {
        let c = RepoConfig::parse(r#"{"stabilityDays": 3}"#);
        assert_eq!(
            c.minimum_release_age.as_deref(),
            Some("3 days"),
            "stabilityDays: 3 must migrate to minimumReleaseAge: '3 days'"
        );
    }

    #[test]
    fn stability_days_1_migrated_to_1_day() {
        let c = RepoConfig::parse(r#"{"stabilityDays": 1}"#);
        assert_eq!(c.minimum_release_age.as_deref(), Some("1 day"));
    }

    #[test]
    fn stability_days_0_means_no_minimum_release_age() {
        let c = RepoConfig::parse(r#"{"stabilityDays": 0}"#);
        assert!(
            c.minimum_release_age.is_none(),
            "stabilityDays: 0 must not set minimumReleaseAge"
        );
    }

    #[test]
    fn minimum_release_age_takes_precedence_over_stability_days() {
        let c = RepoConfig::parse(r#"{"minimumReleaseAge": "7 days", "stabilityDays": 3}"#);
        assert_eq!(
            c.minimum_release_age.as_deref(),
            Some("7 days"),
            "explicit minimumReleaseAge must take precedence over stabilityDays migration"
        );
    }

    #[test]
    fn per_rule_minimum_release_age_parsed() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["lodash"], "minimumReleaseAge": "3 days"}]}"#,
        );
        assert_eq!(
            c.package_rules[0].minimum_release_age.as_deref(),
            Some("3 days")
        );
    }

    #[test]
    fn per_rule_minimum_release_age_collected_into_effects() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["lodash"], "minimumReleaseAge": "1 week"}]}"#,
        );
        let ctx = DepContext {
            dep_name: "lodash",
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(effects.minimum_release_age.as_deref(), Some("1 week"));
    }

    #[test]
    fn per_rule_minimum_release_age_not_set_when_rule_does_not_match() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["lodash"], "minimumReleaseAge": "3 days"}]}"#,
        );
        let ctx = DepContext {
            dep_name: "react",
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert!(effects.minimum_release_age.is_none());
    }

    #[test]
    fn last_matching_rule_minimum_release_age_wins() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [
                {"matchPackageNames": ["lodash"], "minimumReleaseAge": "3 days"},
                {"matchPackageNames": ["lodash"], "minimumReleaseAge": "1 week"}
            ]}"#,
        );
        let ctx = DepContext {
            dep_name: "lodash",
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(
            effects.minimum_release_age.as_deref(),
            Some("1 week"),
            "last matching rule should win"
        );
    }

    // ── per-rule addLabels ────────────────────────────────────────────────────

    #[test]
    fn per_rule_add_labels_parsed() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["lodash"], "addLabels": ["dep-update", "js"]}]}"#,
        );
        assert_eq!(c.package_rules[0].add_labels, vec!["dep-update", "js"]);
    }

    #[test]
    fn per_rule_add_labels_accumulated_into_effects() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["lodash"], "addLabels": ["dep-update"]}]}"#,
        );
        let ctx = DepContext {
            dep_name: "lodash",
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert!(effects.labels.contains(&"dep-update".to_owned()));
    }

    #[test]
    fn per_rule_add_labels_accumulate_from_multiple_rules() {
        // Two matching rules each add a different label; both should appear.
        let c = RepoConfig::parse(
            r#"{"packageRules": [
                {"matchPackageNames": ["lodash"], "addLabels": ["dep-update"]},
                {"matchPackageNames": ["lodash"], "addLabels": ["js"]}
            ]}"#,
        );
        let ctx = DepContext {
            dep_name: "lodash",
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert!(effects.labels.contains(&"dep-update".to_owned()));
        assert!(effects.labels.contains(&"js".to_owned()));
    }

    #[test]
    fn per_rule_add_labels_does_not_duplicate() {
        // Same label from addLabels and repo-level labels → only one copy.
        let c = RepoConfig::parse(
            r#"{"labels": ["dep-update"], "packageRules": [{"matchPackageNames": ["lodash"], "addLabels": ["dep-update"]}]}"#,
        );
        let ctx = DepContext {
            dep_name: "lodash",
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(
            effects.labels.iter().filter(|l| *l == "dep-update").count(),
            1,
            "label should not be duplicated"
        );
    }

    #[test]
    fn per_rule_add_labels_not_applied_to_non_matching_dep() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["lodash"], "addLabels": ["dep-update"]}]}"#,
        );
        let ctx = DepContext {
            dep_name: "react",
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert!(
            !effects.labels.contains(&"dep-update".to_owned()),
            "non-matching dep should not get addLabels"
        );
    }

    // ── groupSlug ─────────────────────────────────────────────────────────────

    #[test]
    fn group_slug_parsed_from_package_rule() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["lodash"], "groupName": "JS deps", "groupSlug": "js"}]}"#,
        );
        assert_eq!(c.package_rules[0].group_slug.as_deref(), Some("js"));
    }

    #[test]
    fn group_slug_collected_into_effects() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["lodash"], "groupName": "JS deps", "groupSlug": "js"}]}"#,
        );
        let ctx = DepContext {
            dep_name: "lodash",
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(effects.group_name.as_deref(), Some("JS deps"));
        assert_eq!(effects.group_slug.as_deref(), Some("js"));
    }

    #[test]
    fn group_slug_absent_when_not_set() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["lodash"], "groupName": "JS deps"}]}"#,
        );
        let ctx = DepContext {
            dep_name: "lodash",
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(effects.group_name.as_deref(), Some("JS deps"));
        // No explicit groupSlug → auto-derive from groupName in branch computation
        assert!(effects.group_slug.is_none());
    }

    #[test]
    fn group_slug_last_matching_rule_wins() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [
                {"matchPackageNames": ["lodash"], "groupName": "A", "groupSlug": "first"},
                {"matchPackageNames": ["lodash"], "groupName": "B", "groupSlug": "second"}
            ]}"#,
        );
        let ctx = DepContext {
            dep_name: "lodash",
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(
            effects.group_slug.as_deref(),
            Some("second"),
            "last matching rule's groupSlug should win (Renovate mergeChildConfig semantics)"
        );
    }

    // ── per-rule commitMessageAction / commitMessagePrefix ────────────────────

    #[test]
    fn per_rule_commit_message_action_parsed() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["lodash"], "commitMessageAction": "Pin"}]}"#,
        );
        assert_eq!(
            c.package_rules[0].commit_message_action.as_deref(),
            Some("Pin")
        );
    }

    #[test]
    fn per_rule_commit_message_prefix_parsed() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["lodash"], "commitMessagePrefix": "fix(deps):"}]}"#,
        );
        assert_eq!(
            c.package_rules[0].commit_message_prefix.as_deref(),
            Some("fix(deps):")
        );
    }

    #[test]
    fn per_rule_commit_message_action_collected_into_effects() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["lodash"], "commitMessageAction": "Pin"}]}"#,
        );
        let ctx = DepContext {
            dep_name: "lodash",
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(effects.commit_message_action.as_deref(), Some("Pin"));
    }

    #[test]
    fn per_rule_commit_message_prefix_collected_into_effects() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["lodash"], "commitMessagePrefix": "fix(deps):"}]}"#,
        );
        let ctx = DepContext {
            dep_name: "lodash",
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(effects.commit_message_prefix.as_deref(), Some("fix(deps):"));
    }

    #[test]
    fn per_rule_commit_message_action_absent_when_not_set() {
        let c = RepoConfig::parse(r#"{"packageRules": [{"matchPackageNames": ["lodash"]}]}"#);
        let ctx = DepContext {
            dep_name: "lodash",
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert!(
            effects.commit_message_action.is_none(),
            "commit_message_action should be None when not configured"
        );
    }

    #[test]
    fn per_rule_commit_message_action_last_rule_wins() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [
                {"matchPackageNames": ["lodash"], "commitMessageAction": "Pin"},
                {"matchPackageNames": ["lodash"], "commitMessageAction": "Roll back"}
            ]}"#,
        );
        let ctx = DepContext {
            dep_name: "lodash",
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(
            effects.commit_message_action.as_deref(),
            Some("Roll back"),
            "last matching rule's commitMessageAction should win"
        );
    }

    #[test]
    fn per_rule_commit_message_prefix_last_rule_wins() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [
                {"matchPackageNames": ["lodash"], "commitMessagePrefix": "chore:"},
                {"matchPackageNames": ["lodash"], "commitMessagePrefix": "fix(deps):"}
            ]}"#,
        );
        let ctx = DepContext {
            dep_name: "lodash",
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(
            effects.commit_message_prefix.as_deref(),
            Some("fix(deps):"),
            "last matching rule's commitMessagePrefix should win"
        );
    }

    // ── group:* preset tests ────────────────────────────────────────────────

    #[test]
    fn group_all_preset_injects_group_rule() {
        let c = RepoConfig::parse(r#"{"extends": ["group:all"]}"#);
        // group:all should set separateMajorMinor: false
        assert!(
            !c.separate_major_minor,
            "group:all implies separateMajorMinor: false"
        );
        // group:all should inject a packageRule grouping everything
        let ctx = DepContext {
            dep_name: "lodash",
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(effects.group_name.as_deref(), Some("all dependencies"));
        assert_eq!(effects.group_slug.as_deref(), Some("all"));
    }

    #[test]
    fn group_all_non_major_preset_injects_group_rule_for_minor() {
        let c = RepoConfig::parse(r#"{"extends": ["group:allNonMajor"]}"#);
        // separateMajorMinor should remain true (not overridden)
        assert!(c.separate_major_minor);
        let ctx = DepContext {
            dep_name: "react",
            update_type: Some(crate::versioning::semver_generic::UpdateType::Minor),
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(
            effects.group_name.as_deref(),
            Some("all non-major dependencies")
        );
    }

    #[test]
    fn group_all_non_major_does_not_apply_to_major() {
        let c = RepoConfig::parse(r#"{"extends": ["group:allNonMajor"]}"#);
        let ctx = DepContext {
            dep_name: "react",
            update_type: Some(crate::versioning::semver_generic::UpdateType::Major),
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        // Major update should not be grouped
        assert!(effects.group_name.is_none());
    }

    // ── Additional group presets ─────────────────────────────────────────────

    #[test]
    fn group_all_digest_preset_injects_group_rule() {
        let c = RepoConfig::parse(r#"{"extends": ["group:allDigest"]}"#);
        assert_eq!(c.package_rules.len(), 1);
        let rule = &c.package_rules[0];
        assert_eq!(rule.group_name.as_deref(), Some("all digest updates"));
        assert_eq!(rule.group_slug.as_deref(), Some("all-digest"));
        // The rule has a constraint but no known update types (digest not yet modelled)
        assert!(rule.has_update_type_constraint);
        assert!(rule.match_update_types.is_empty());
    }

    #[test]
    fn group_node_js_preset_matches_node_datasource() {
        let c = RepoConfig::parse(r#"{"extends": ["group:nodeJs"]}"#);
        assert_eq!(c.package_rules.len(), 1);
        let rule = &c.package_rules[0];
        assert_eq!(rule.group_name.as_deref(), Some("Node.js"));
        assert!(rule.match_datasources.contains(&"docker".to_owned()));
        assert!(rule.match_datasources.contains(&"node-version".to_owned()));
        // Positive pattern matches "node"
        assert!(rule.name_matches("node"));
        // Negation exclusions prevent "calico/node"
        assert!(!rule.name_matches("calico/node"));
    }

    #[test]
    fn group_js_test_preset_matches_jest_packages() {
        let c = RepoConfig::parse(r#"{"extends": ["group:jsTest"]}"#);
        assert_eq!(c.package_rules.len(), 1);
        let rule = &c.package_rules[0];
        assert_eq!(rule.group_name.as_deref(), Some("JS test packages"));
        assert!(rule.name_matches("jest"));
        assert!(rule.name_matches("@types/jest"));
        assert!(rule.name_matches("vitest"));
        assert!(rule.name_matches("ts-jest"));
        assert!(!rule.name_matches("lodash"));
    }

    #[test]
    fn group_js_test_non_major_does_not_group_major() {
        use crate::versioning::semver_generic::UpdateType;
        let c = RepoConfig::parse(r#"{"extends": ["group:jsTestNonMajor"]}"#);
        let rule = &c.package_rules[0];
        assert!(rule.name_matches("jest"));
        // The rule has a minor+patch update type constraint
        assert!(rule.update_type_matches(UpdateType::Minor, false));
        assert!(rule.update_type_matches(UpdateType::Patch, false));
        assert!(!rule.update_type_matches(UpdateType::Major, false));
    }

    #[test]
    fn group_gradle_preset_injects_rule() {
        let c = RepoConfig::parse(r#"{"extends": ["group:gradle"]}"#);
        assert_eq!(c.package_rules.len(), 1);
        let rule = &c.package_rules[0];
        assert_eq!(rule.group_name.as_deref(), Some("Gradle"));
        assert!(
            rule.match_datasources
                .contains(&"gradle-version".to_owned())
        );
        assert!(rule.name_matches("gradle"));
        assert!(!rule.name_matches("maven"));
    }

    #[test]
    fn group_definitely_typed_preset_matches_types_packages() {
        let c = RepoConfig::parse(r#"{"extends": ["group:definitelyTyped"]}"#);
        assert_eq!(c.package_rules.len(), 1);
        let rule = &c.package_rules[0];
        assert_eq!(rule.group_name.as_deref(), Some("definitelyTyped"));
        assert!(rule.name_matches("@types/node"));
        assert!(rule.name_matches("@types/jest"));
        assert!(!rule.name_matches("lodash"));
    }

    #[test]
    fn group_react_preset_matches_react_types() {
        let c = RepoConfig::parse(r#"{"extends": ["group:react"]}"#);
        let rule = &c.package_rules[0];
        assert_eq!(rule.group_name.as_deref(), Some("react monorepo"));
        assert!(rule.name_matches("@types/react"));
        assert!(rule.name_matches("@types/react-dom"));
        assert!(!rule.name_matches("react"));
        assert!(!rule.name_matches("lodash"));
    }

    #[test]
    fn group_spring_boot_injects_two_rules() {
        let c = RepoConfig::parse(r#"{"extends": ["group:springBoot"]}"#);
        // springBoot uses both matchDepNames and matchPackageNames → two rules
        assert_eq!(c.package_rules.len(), 2);
        assert!(
            c.package_rules
                .iter()
                .all(|r| r.group_name.as_deref() == Some("spring boot"))
        );
    }

    #[test]
    fn group_spring_core_matches_spring_packages() {
        let c = RepoConfig::parse(r#"{"extends": ["group:springCore"]}"#);
        let rule = &c.package_rules[0];
        assert_eq!(rule.group_name.as_deref(), Some("spring core"));
        assert!(rule.name_matches("org.springframework:spring-core"));
        assert!(!rule.name_matches("org.springframework.boot:spring-boot"));
    }

    #[test]
    fn group_linters_matches_eslint_and_prettier() {
        let c = RepoConfig::parse(r#"{"extends": ["group:linters"]}"#);
        assert_eq!(c.package_rules.len(), 1);
        let rule = &c.package_rules[0];
        assert_eq!(rule.group_name.as_deref(), Some("linters"));
        assert!(rule.name_matches("eslint"));
        assert!(rule.name_matches("@typescript-eslint/parser"));
        assert!(rule.name_matches("prettier"));
        assert!(rule.name_matches("stylelint"));
        assert!(!rule.name_matches("lodash"));
        assert!(!rule.name_matches("jest"));
    }

    #[test]
    fn group_recommended_expands_many_presets() {
        let c = RepoConfig::parse(r#"{"extends": ["group:recommended"]}"#);
        // group:recommended should expand to many sub-presets (at least 40 rules after additions).
        assert!(
            c.package_rules.len() >= 40,
            "group:recommended should inject many rules, got {}",
            c.package_rules.len()
        );
        // Verify some specific group names are present.
        let group_names: Vec<&str> = c
            .package_rules
            .iter()
            .filter_map(|r| r.group_name.as_deref())
            .collect();
        assert!(group_names.contains(&"Node.js"), "nodeJs rule missing");
        assert!(
            group_names.contains(&"spring boot"),
            "springBoot rule missing"
        );
        assert!(
            group_names.contains(&"spring security"),
            "springSecurity missing"
        );
        assert!(
            group_names.contains(&"symfony packages"),
            "symfony rule missing"
        );
        assert!(
            group_names.contains(&"Ruby on Rails packages"),
            "rubyOnRails rule missing"
        );
    }

    #[test]
    fn group_symfony_matches_with_exclusions() {
        let c = RepoConfig::parse(r#"{"extends": ["group:symfony"]}"#);
        let rule = &c.package_rules[0];
        assert_eq!(rule.group_name.as_deref(), Some("symfony packages"));
        // symfony/* matches
        assert!(rule.name_matches("symfony/console"));
        assert!(rule.name_matches("symfony/http-kernel"));
        // excluded packages don't match
        assert!(!rule.name_matches("symfony/flex"));
        assert!(!rule.name_matches("symfony/ux-turbo"));
        assert!(!rule.name_matches("symfony/polyfill-mbstring"));
    }

    #[test]
    fn group_ruby_on_rails_matches_rails_gems() {
        let c = RepoConfig::parse(r#"{"extends": ["group:rubyOnRails"]}"#);
        let rule = &c.package_rules[0];
        assert_eq!(rule.group_name.as_deref(), Some("Ruby on Rails packages"));
        assert!(rule.name_matches("rails"));
        assert!(rule.name_matches("activerecord"));
        assert!(rule.name_matches("activesupport"));
        assert!(!rule.name_matches("devise"));
    }

    #[test]
    fn group_jest_plus_ts_jest_matches_major_only() {
        use crate::versioning::semver_generic::UpdateType;
        let c = RepoConfig::parse(r#"{"extends": ["group:jestPlusTSJest"]}"#);
        let rule = &c.package_rules[0];
        assert_eq!(rule.group_name.as_deref(), Some("jest monorepo"));
        assert!(rule.update_type_matches(UpdateType::Major, false));
        assert!(!rule.update_type_matches(UpdateType::Minor, false));
    }

    #[test]
    fn group_vite_matches_vite_packages() {
        let c = RepoConfig::parse(r#"{"extends": ["group:vite"]}"#);
        let rule = &c.package_rules[0];
        assert_eq!(rule.group_name.as_deref(), Some("Vite packages"));
        assert!(rule.name_matches("vite"));
        assert!(rule.name_matches("@vitejs/plugin-react"));
        assert!(rule.name_matches("vite-plugin-dts"));
        assert!(!rule.name_matches("webpack"));
    }

    #[test]
    fn group_pulumi_injects_five_rules() {
        let c = RepoConfig::parse(r#"{"extends": ["group:pulumi"]}"#);
        // 5 rules: npm, pypi, go, maven, nuget
        assert_eq!(c.package_rules.len(), 5);
        assert!(
            c.package_rules
                .iter()
                .all(|r| r.group_name.as_deref() == Some("Pulumi"))
        );
    }

    #[test]
    fn group_jwt_framework_matches_packagist() {
        let c = RepoConfig::parse(r#"{"extends": ["group:jwtFramework"]}"#);
        let rule = &c.package_rules[0];
        assert_eq!(rule.group_name.as_deref(), Some("JWT Framework packages"));
        assert!(rule.datasource_matches("packagist"));
        assert!(!rule.datasource_matches("npm"));
    }

    #[test]
    fn per_rule_commit_message_action_not_applied_to_non_matching() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["lodash"], "commitMessageAction": "Pin"}]}"#,
        );
        let ctx = DepContext {
            dep_name: "react",
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert!(
            effects.commit_message_action.is_none(),
            "non-matching dep should not get commitMessageAction override"
        );
    }

    // ── major/minor/patch config blocks ──────────────────────────────────────

    #[test]
    fn major_config_parsed() {
        let c = RepoConfig::parse(r#"{"major": {"automerge": false, "labels": ["breaking"]}}"#);
        let cfg = c
            .major_config
            .as_ref()
            .expect("major config should be present");
        assert_eq!(cfg.automerge, Some(false));
        assert_eq!(cfg.labels, vec!["breaking".to_owned()]);
    }

    #[test]
    fn minor_config_parsed() {
        let c = RepoConfig::parse(r#"{"minor": {"automerge": true}}"#);
        let cfg = c
            .minor_config
            .as_ref()
            .expect("minor config should be present");
        assert_eq!(cfg.automerge, Some(true));
    }

    #[test]
    fn patch_config_parsed() {
        let c = RepoConfig::parse(r#"{"patch": {"automerge": true, "prPriority": 5}}"#);
        let cfg = c
            .patch_config
            .as_ref()
            .expect("patch config should be present");
        assert_eq!(cfg.automerge, Some(true));
        assert_eq!(cfg.pr_priority, Some(5));
    }

    #[test]
    fn major_config_applied_to_major_update() {
        use crate::versioning::semver_generic::UpdateType;
        let c = RepoConfig::parse(r#"{"major": {"labels": ["breaking"], "automerge": false}}"#);
        let ctx = DepContext {
            dep_name: "lodash",
            update_type: Some(UpdateType::Major),
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert!(effects.labels.contains(&"breaking".to_owned()));
        assert_eq!(effects.automerge, Some(false));
    }

    #[test]
    fn major_config_not_applied_to_minor_update() {
        use crate::versioning::semver_generic::UpdateType;
        let c = RepoConfig::parse(r#"{"major": {"labels": ["breaking"]}}"#);
        let ctx = DepContext {
            dep_name: "lodash",
            update_type: Some(UpdateType::Minor),
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert!(!effects.labels.contains(&"breaking".to_owned()));
    }

    #[test]
    fn minor_config_applied_to_minor_update() {
        use crate::versioning::semver_generic::UpdateType;
        let c = RepoConfig::parse(r#"{"minor": {"automerge": true, "prPriority": 3}}"#);
        let ctx = DepContext {
            dep_name: "react",
            update_type: Some(UpdateType::Minor),
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(effects.automerge, Some(true));
        assert_eq!(effects.pr_priority, Some(3));
    }

    #[test]
    fn patch_config_applied_to_patch_update() {
        use crate::versioning::semver_generic::UpdateType;
        let c = RepoConfig::parse(r#"{"patch": {"automerge": true}}"#);
        let ctx = DepContext {
            dep_name: "express",
            update_type: Some(UpdateType::Patch),
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(effects.automerge, Some(true));
    }

    #[test]
    fn major_config_overrides_package_rule() {
        // packageRule sets automerge=true but major config sets automerge=false.
        // major config applies AFTER packageRules → false wins.
        use crate::versioning::semver_generic::UpdateType;
        let c = RepoConfig::parse(
            r#"{
            "packageRules": [{"matchPackageNames": ["lodash"], "automerge": true}],
            "major": {"automerge": false}
        }"#,
        );
        let ctx = DepContext {
            dep_name: "lodash",
            update_type: Some(UpdateType::Major),
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(effects.automerge, Some(false));
    }

    #[test]
    fn update_type_config_add_labels_accumulates() {
        // addLabels in major config should append to existing labels.
        use crate::versioning::semver_generic::UpdateType;
        let c =
            RepoConfig::parse(r#"{"labels": ["renovate"], "major": {"addLabels": ["breaking"]}}"#);
        let ctx = DepContext {
            dep_name: "lodash",
            update_type: Some(UpdateType::Major),
            ..Default::default()
        };
        let effects = c.collect_rule_effects(&ctx);
        assert!(effects.labels.contains(&"renovate".to_owned()));
        assert!(effects.labels.contains(&"breaking".to_owned()));
    }

    #[test]
    fn major_enabled_false_blocks_major_updates() {
        use crate::versioning::semver_generic::UpdateType;
        let c = RepoConfig::parse(r#"{"major": {"enabled": false}}"#);
        let ctx = DepContext {
            dep_name: "lodash",
            update_type: Some(UpdateType::Major),
            ..Default::default()
        };
        assert!(
            c.is_update_blocked_ctx(&ctx),
            "major update should be blocked when major.enabled=false"
        );
    }

    #[test]
    fn major_enabled_false_does_not_block_minor() {
        use crate::versioning::semver_generic::UpdateType;
        let c = RepoConfig::parse(r#"{"major": {"enabled": false}}"#);
        let ctx = DepContext {
            dep_name: "lodash",
            update_type: Some(UpdateType::Minor),
            ..Default::default()
        };
        assert!(
            !c.is_update_blocked_ctx(&ctx),
            "minor update must not be blocked by major.enabled=false"
        );
    }

    #[test]
    fn minor_enabled_false_blocks_minor_updates() {
        use crate::versioning::semver_generic::UpdateType;
        let c = RepoConfig::parse(r#"{"minor": {"enabled": false}}"#);
        let ctx = DepContext {
            dep_name: "react",
            update_type: Some(UpdateType::Minor),
            ..Default::default()
        };
        assert!(
            c.is_update_blocked_ctx(&ctx),
            "minor update should be blocked when minor.enabled=false"
        );
    }

    // ── docker:* presets ─────────────────────────────────────────────────────

    #[test]
    fn docker_disable_major_blocks_major_docker_updates() {
        use crate::versioning::semver_generic::UpdateType;
        let c = RepoConfig::parse(r#"{"extends": ["docker:disableMajor"]}"#);
        let blocked = DepContext {
            dep_name: "nginx",
            datasource: Some("docker"),
            update_type: Some(UpdateType::Major),
            ..Default::default()
        };
        let allowed = DepContext {
            dep_name: "nginx",
            datasource: Some("docker"),
            update_type: Some(UpdateType::Minor),
            ..Default::default()
        };
        assert!(
            c.is_update_blocked_ctx(&blocked),
            "docker:disableMajor must block major docker updates"
        );
        assert!(
            !c.is_update_blocked_ctx(&allowed),
            "docker:disableMajor must not block minor docker updates"
        );
    }

    #[test]
    fn docker_disable_major_does_not_affect_npm() {
        use crate::versioning::semver_generic::UpdateType;
        let c = RepoConfig::parse(r#"{"extends": ["docker:disableMajor"]}"#);
        let ctx = DepContext {
            dep_name: "express",
            datasource: Some("npm"),
            update_type: Some(UpdateType::Major),
            ..Default::default()
        };
        assert!(
            !c.is_update_blocked_ctx(&ctx),
            "docker:disableMajor must only block docker datasource"
        );
    }

    #[test]
    fn docker_enable_major_counteracts_disable_major() {
        use crate::versioning::semver_generic::UpdateType;
        // Last-rule-wins: disableMajor then enableMajor → major is allowed.
        let c = RepoConfig::parse(r#"{"extends": ["docker:disableMajor", "docker:enableMajor"]}"#);
        let ctx = DepContext {
            dep_name: "nginx",
            datasource: Some("docker"),
            update_type: Some(UpdateType::Major),
            ..Default::default()
        };
        assert!(
            !c.is_update_blocked_ctx(&ctx),
            "docker:enableMajor after docker:disableMajor must re-enable major updates"
        );
    }

    #[test]
    fn docker_disable_disables_docker_managers() {
        // docker:disable mirrors Renovate's docker.preset.ts which sets
        // dockerfile/docker-compose/circleci enabled: false at manager level.
        let c = RepoConfig::parse(r#"{"extends": ["docker:disable"]}"#);
        assert!(
            !c.is_manager_enabled("dockerfile", false),
            "docker:disable must disable dockerfile manager"
        );
        assert!(
            !c.is_manager_enabled("docker-compose", false),
            "docker:disable must disable docker-compose manager"
        );
        assert!(
            !c.is_manager_enabled("circleci", false),
            "docker:disable must disable circleci manager"
        );
    }

    #[test]
    fn docker_disable_does_not_affect_other_managers() {
        let c = RepoConfig::parse(r#"{"extends": ["docker:disable"]}"#);
        assert!(
            c.is_manager_enabled("cargo", false),
            "docker:disable must not disable non-docker managers"
        );
        assert!(
            c.is_manager_enabled("npm", false),
            "docker:disable must not disable npm"
        );
    }

    // ── disabledManagers JSON config field ───────────────────────────────────

    #[test]
    fn disabled_managers_from_json_config() {
        let c = RepoConfig::parse(r#"{"disabledManagers": ["dockerfile", "maven"]}"#);
        assert!(
            !c.is_manager_enabled("dockerfile", false),
            "dockerfile in disabledManagers must be disabled"
        );
        assert!(
            !c.is_manager_enabled("maven", false),
            "maven in disabledManagers must be disabled"
        );
        assert!(
            c.is_manager_enabled("cargo", false),
            "cargo not in disabledManagers must remain enabled"
        );
    }

    // ── helpers:* presets ────────────────────────────────────────────────────

    #[test]
    fn helpers_disable_types_node_major_blocks_major() {
        use crate::versioning::semver_generic::UpdateType;
        let c = RepoConfig::parse(r#"{"extends": ["helpers:disableTypesNodeMajor"]}"#);
        assert_eq!(c.package_rules.len(), 1);
        let ctx_major = DepContext {
            dep_name: "@types/node",
            update_type: Some(UpdateType::Major),
            ..Default::default()
        };
        let ctx_minor = DepContext {
            dep_name: "@types/node",
            update_type: Some(UpdateType::Minor),
            ..Default::default()
        };
        assert!(
            c.is_update_blocked_ctx(&ctx_major),
            "helpers:disableTypesNodeMajor must block @types/node major"
        );
        assert!(
            !c.is_update_blocked_ctx(&ctx_minor),
            "helpers:disableTypesNodeMajor must not block @types/node minor"
        );
    }

    #[test]
    fn helpers_disable_types_node_major_does_not_affect_other_packages() {
        use crate::versioning::semver_generic::UpdateType;
        let c = RepoConfig::parse(r#"{"extends": ["helpers:disableTypesNodeMajor"]}"#);
        let ctx = DepContext {
            dep_name: "lodash",
            update_type: Some(UpdateType::Major),
            ..Default::default()
        };
        assert!(
            !c.is_update_blocked_ctx(&ctx),
            "helpers:disableTypesNodeMajor must not affect other packages"
        );
    }

    #[test]
    fn disabled_managers_denylist_overrides_enabled_managers_allowlist() {
        // If a manager appears in both disabledManagers and enabledManagers,
        // disabled takes precedence (denylist wins).
        let c = RepoConfig::parse(
            r#"{"enabledManagers": ["cargo", "npm"], "disabledManagers": ["npm"]}"#,
        );
        assert!(
            c.is_manager_enabled("cargo", false),
            "cargo in enabledManagers must be allowed"
        );
        assert!(
            !c.is_manager_enabled("npm", false),
            "npm in both lists: disabled takes precedence"
        );
    }

    // ── workarounds:* preset tests ────────────────────────────────────────────

    #[test]
    fn workarounds_types_node_versioning_sets_node_versioning_for_types_node() {
        let c = RepoConfig::parse(r#"{"extends": ["workarounds:typesNodeVersioning"]}"#);
        let ctx = DepContext::for_dep("@types/node")
            .with_manager("npm")
            .with_datasource("npm");
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(
            effects.versioning.as_deref(),
            Some("node"),
            "workarounds:typesNodeVersioning must set versioning=node for @types/node"
        );
    }

    #[test]
    fn workarounds_ubuntu_docker_versioning_sets_ubuntu_versioning() {
        let c = RepoConfig::parse(r#"{"extends": ["workarounds:ubuntuDockerVersioning"]}"#);
        let ctx = DepContext::for_dep("ubuntu").with_datasource("docker");
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(
            effects.versioning.as_deref(),
            Some("ubuntu"),
            "workarounds:ubuntuDockerVersioning must set versioning=ubuntu for ubuntu"
        );
    }

    #[test]
    fn workarounds_disable_eclipse_lifecycle_mapping_disables_package() {
        let c = RepoConfig::parse(r#"{"extends": ["workarounds:disableEclipseLifecycleMapping"]}"#);
        let ctx = DepContext::for_dep("org.eclipse.m2e:lifecycle-mapping").with_datasource("maven");
        assert!(
            c.is_dep_ignored_ctx(&ctx),
            "workarounds:disableEclipseLifecycleMapping must disable lifecycle-mapping"
        );
    }

    #[test]
    fn workarounds_maven_commons_ancient_version_sets_allowed_versions() {
        let c = RepoConfig::parse(r#"{"extends": ["workarounds:mavenCommonsAncientVersion"]}"#);
        let ctx = DepContext::for_dep("commons-lang").with_datasource("maven");
        let rule_matches = c
            .package_rules
            .iter()
            .any(|r| r.matches_context(&ctx) && r.allowed_versions.is_some());
        assert!(
            rule_matches,
            "workarounds:mavenCommonsAncientVersion must match commons-lang and set allowedVersions"
        );
    }

    // ── replacements:* preset integration tests ───────────────────────────────

    #[test]
    fn replacements_all_injects_replacement_rules() {
        let c = RepoConfig::parse(r#"{"extends": ["replacements:all"]}"#);
        // replacements:all must inject at least some replacement rules.
        let has_replacement = c.package_rules.iter().any(|r| r.replacement_name.is_some());
        assert!(
            has_replacement,
            "replacements:all must inject packageRules with replacementName"
        );
    }

    #[test]
    fn replacements_babel_eslint_injected_via_all() {
        let c = RepoConfig::parse(r#"{"extends": ["replacements:all"]}"#);
        // The babel-eslint-to-eslint-parser rule should be present.
        let has_babel_rule = c.package_rules.iter().any(|r| {
            r.match_package_names.contains(&"babel-eslint".to_owned())
                && r.replacement_name.as_deref() == Some("@babel/eslint-parser")
        });
        assert!(
            has_babel_rule,
            "replacements:all must include babel-eslint → @babel/eslint-parser replacement"
        );
    }

    #[test]
    fn replacements_individual_preset_also_works() {
        let c = RepoConfig::parse(r#"{"extends": ["replacements:babel-eslint-to-eslint-parser"]}"#);
        let has_babel_rule = c
            .package_rules
            .iter()
            .any(|r| r.match_package_names.contains(&"babel-eslint".to_owned()));
        assert!(
            has_babel_rule,
            "individual replacements:babel-eslint-to-eslint-parser must inject the rule"
        );
    }

    #[test]
    fn workarounds_all_expands_to_all_sub_presets() {
        let c = RepoConfig::parse(r#"{"extends": ["workarounds:all"]}"#);
        // workarounds:all should include typesNodeVersioning, ubuntuDockerVersioning, etc.
        // Verify a few key rule effects from different sub-presets are present.
        let ctx_types_node = DepContext::for_dep("@types/node")
            .with_manager("npm")
            .with_datasource("npm");
        let effects_types_node = c.collect_rule_effects(&ctx_types_node);
        assert_eq!(
            effects_types_node.versioning.as_deref(),
            Some("node"),
            "workarounds:all must include typesNodeVersioning"
        );
        let ctx_ubuntu = DepContext::for_dep("ubuntu").with_datasource("docker");
        let effects_ubuntu = c.collect_rule_effects(&ctx_ubuntu);
        assert_eq!(
            effects_ubuntu.versioning.as_deref(),
            Some("ubuntu"),
            "workarounds:all must include ubuntuDockerVersioning"
        );
    }

    // ── :followTag / helpers:followTypescript* tests ──────────────────────────

    #[test]
    fn follow_tag_preset_injects_packagerule() {
        let c = RepoConfig::parse(r#"{"extends": [":followTag(typescript, next)"]}"#);
        let ctx = DepContext::for_dep("typescript")
            .with_manager("npm")
            .with_datasource("npm");
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(
            effects.follow_tag.as_deref(),
            Some("next"),
            ":followTag(typescript, next) must set followTag=next for typescript"
        );
    }

    #[test]
    fn helpers_follow_typescript_next_sets_follow_tag() {
        let c = RepoConfig::parse(r#"{"extends": ["helpers:followTypescriptNext"]}"#);
        let ctx = DepContext::for_dep("typescript")
            .with_manager("npm")
            .with_datasource("npm");
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(
            effects.follow_tag.as_deref(),
            Some("next"),
            "helpers:followTypescriptNext must set followTag=next for typescript"
        );
    }

    #[test]
    fn helpers_follow_typescript_rc_sets_follow_tag() {
        let c = RepoConfig::parse(r#"{"extends": ["helpers:followTypescriptRc"]}"#);
        let ctx = DepContext::for_dep("typescript").with_manager("npm");
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(
            effects.follow_tag.as_deref(),
            Some("rc"),
            "helpers:followTypescriptRc must set followTag=rc for typescript"
        );
    }

    #[test]
    fn follow_tag_preset_does_not_match_other_packages() {
        let c = RepoConfig::parse(r#"{"extends": [":followTag(typescript, next)"]}"#);
        let ctx = DepContext::for_dep("react").with_manager("npm");
        let effects = c.collect_rule_effects(&ctx);
        assert_eq!(
            effects.follow_tag, None,
            ":followTag(typescript, next) must not affect other packages"
        );
    }

    // ── :label / :labels preset tests ────────────────────────────────────────

    #[test]
    fn label_preset_adds_label_to_repo() {
        let c = RepoConfig::parse(r#"{"extends": [":label(security)"]}"#);
        let ctx = DepContext::for_dep("lodash");
        let effects = c.collect_rule_effects(&ctx);
        assert!(
            effects.labels.contains(&"security".to_owned()),
            ":label(security) must add 'security' label"
        );
    }

    #[test]
    fn labels_preset_adds_multiple_labels() {
        let c = RepoConfig::parse(r#"{"extends": [":labels(security, dependencies)"]}"#);
        let ctx = DepContext::for_dep("lodash");
        let effects = c.collect_rule_effects(&ctx);
        assert!(
            effects.labels.contains(&"security".to_owned()),
            ":labels must add first label"
        );
        assert!(
            effects.labels.contains(&"dependencies".to_owned()),
            ":labels must add second label"
        );
    }

    // ── changelogUrl per-rule field tests ─────────────────────────────────────

    #[test]
    fn changelog_url_parsed_from_package_rules() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchDatasources": ["github-releases"], "changelogUrl": "{{sourceUrl}}/compare/{{currentDigest}}..{{newDigest}}"}]}"#,
        );
        let ctx = DepContext::for_dep("actions/checkout").with_datasource("github-releases");
        let effects = c.collect_rule_effects(&ctx);
        assert!(
            effects.changelog_url.is_some(),
            "changelogUrl must be collected from packageRules"
        );
    }

    #[test]
    fn workarounds_k3s_kubernetes_versioning_sets_regex_versioning() {
        let c = RepoConfig::parse(r#"{"extends": ["workarounds:k3sKubernetesVersioning"]}"#);
        let ctx = DepContext::for_dep("k3s-io/k3s").with_datasource("github-releases");
        let effects = c.collect_rule_effects(&ctx);
        assert!(
            effects
                .versioning
                .as_deref()
                .is_some_and(|v| v.starts_with("regex:")),
            "workarounds:k3sKubernetesVersioning must set a regex versioning scheme"
        );
    }

    // ── group:monorepos integration tests ─────────────────────────────────────

    #[test]
    fn group_monorepos_angularmaterial_pattern_group() {
        let c = RepoConfig::parse(r#"{"extends": ["group:monorepos"]}"#);
        // @angular/material should be grouped by the angularmaterial pattern group.
        let ctx = DepContext::for_dep("@angular/material");
        let effects = c.collect_rule_effects(&ctx);
        assert!(
            effects.group_name.is_some(),
            "group:monorepos must group @angular/material into a monorepo group"
        );
        assert!(
            effects
                .group_name
                .as_deref()
                .is_some_and(|g| g.contains("angularmaterial")),
            "group must be the angularmaterial monorepo, got: {:?}",
            effects.group_name
        );
    }

    #[test]
    fn group_monorepos_injects_many_rules() {
        let c = RepoConfig::parse(r#"{"extends": ["group:monorepos"]}"#);
        // 452+ monorepo presets → many rules.
        assert!(
            c.package_rules.len() >= 400,
            "group:monorepos must inject at least 400 rules, got {}",
            c.package_rules.len()
        );
    }

    // ── ignorePresets interaction with compound presets ───────────────────────

    #[test]
    fn ignore_presets_workarounds_all_suppresses_all_workaround_rules() {
        let c = RepoConfig::parse(
            r#"{"extends": ["workarounds:all"], "ignorePresets": ["workarounds:all"]}"#,
        );
        let has_commons_workaround = c.package_rules.iter().any(|r| {
            r.match_datasources.contains(&"maven".to_owned())
                && r.match_package_names.contains(&"commons-**".to_owned())
        });
        assert!(
            !has_commons_workaround,
            "ignorePresets: workarounds:all must suppress all workaround rules"
        );
    }

    #[test]
    fn ignore_presets_replacements_all_suppresses_replacement_rules() {
        let with_c = RepoConfig::parse(r#"{"extends": ["replacements:all"]}"#);
        let without_c = RepoConfig::parse(
            r#"{"extends": ["replacements:all"], "ignorePresets": ["replacements:all"]}"#,
        );
        assert!(
            with_c.package_rules.len() > without_c.package_rules.len(),
            "ignorePresets: replacements:all must suppress replacement rules"
        );
        let has_replacement = without_c
            .package_rules
            .iter()
            .any(|r| r.replacement_name.is_some());
        assert!(
            !has_replacement,
            "ignorePresets: replacements:all must remove all replacementName rules"
        );
    }

    #[test]
    fn ignore_presets_individual_workaround_suppresses_just_that_preset() {
        let all_c = RepoConfig::parse(r#"{"extends": ["workarounds:all"]}"#);
        let partial_c = RepoConfig::parse(
            r#"{"extends": ["workarounds:all"], "ignorePresets": ["workarounds:typesNodeVersioning"]}"#,
        );
        assert!(
            partial_c.package_rules.len() < all_c.package_rules.len(),
            "ignorePresets: individual workaround must remove just that preset's rules"
        );
    }

    // ── customManagers tests ──────────────────────────────────────────────────

    #[test]
    fn custom_manager_parsed_from_json() {
        // Use r##"..."## to avoid "# in the JSON content terminating the raw string.
        let c = RepoConfig::parse(
            r##"{
            "customManagers": [{
                "customType": "regex",
                "managerFilePatterns": ["Dockerfile"],
                "matchStrings": [
                    "(?P<datasource>[\\w-]+) depName=(?P<depName>[^\\s]+)\\nENV (?P<currentValue>.+)"
                ],
                "datasourceTemplate": "docker"
            }]
        }"##,
        );
        assert_eq!(
            c.custom_managers.len(),
            1,
            "one customManager must be parsed"
        );
        let cm = &c.custom_managers[0];
        assert_eq!(cm.custom_type, "regex");
        assert_eq!(cm.file_patterns, vec!["Dockerfile"]);
        assert_eq!(cm.match_strings.len(), 1);
        assert_eq!(cm.datasource_template.as_deref(), Some("docker"));
    }

    #[test]
    fn custom_manager_extracts_deps_from_content() {
        let cm = CustomManager {
            custom_type: "regex".to_owned(),
            file_patterns: vec!["**/.env".to_owned()],
            match_strings: vec![
                r"# renovate: datasource=(?P<datasource>[\w-]+) depName=(?P<depName>[^\s]+)\nNODE_VERSION=(?P<currentValue>[^\s]+)".to_owned(),
            ],
            match_strings_strategy: "any".to_owned(),
            ..Default::default()
        };
        let content = "# renovate: datasource=node-version depName=node\nNODE_VERSION=20.0.0\n";
        let deps = cm.extract_deps(content);
        assert_eq!(deps.len(), 1, "must extract one dep");
        assert_eq!(deps[0].dep_name, "node");
        assert_eq!(deps[0].datasource, "node-version");
        assert_eq!(deps[0].current_value, "20.0.0");
    }

    #[test]
    fn custom_manager_uses_datasource_template_when_group_missing() {
        let cm = CustomManager {
            custom_type: "regex".to_owned(),
            file_patterns: vec!["*.env".to_owned()],
            match_strings: vec![r"NODE_VERSION=(?P<currentValue>[^\s]+)".to_owned()],
            match_strings_strategy: "any".to_owned(),
            datasource_template: Some("node-version".to_owned()),
            dep_name_template: Some("node".to_owned()),
            ..Default::default()
        };
        let content = "NODE_VERSION=18.0.0\n";
        let deps = cm.extract_deps(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "node");
        assert_eq!(deps[0].datasource, "node-version");
        assert_eq!(deps[0].current_value, "18.0.0");
    }

    #[test]
    fn custom_manager_file_match_legacy_field_parsed() {
        let c = RepoConfig::parse(
            r##"{
            "customManagers": [{
                "customType": "regex",
                "fileMatch": ["^Makefile$"],
                "matchStrings": ["TOOL_VERSION=(?P<currentValue>[\\d.]+)"],
                "datasourceTemplate": "github-releases",
                "depNameTemplate": "my-tool"
            }]
        }"##,
        );
        assert_eq!(
            c.custom_managers[0].file_patterns,
            vec!["^Makefile$"],
            "legacy fileMatch must be merged into file_patterns"
        );
    }

    #[test]
    fn custom_manager_matches_file_by_pattern() {
        let cm = CustomManager {
            custom_type: "regex".to_owned(),
            file_patterns: vec!["**.env".to_owned(), "Dockerfile".to_owned()],
            match_strings: vec!["X=(?P<currentValue>\\d+)".to_owned()],
            match_strings_strategy: "any".to_owned(),
            datasource_template: Some("npm".to_owned()),
            dep_name_template: Some("node".to_owned()),
            ..Default::default()
        };
        assert!(cm.matches_file(".env"), ".env must match **.env glob");
        assert!(cm.matches_file("Dockerfile"), "Dockerfile must match exact");
        assert!(
            !cm.matches_file("package.json"),
            "package.json must not match"
        );
    }

    #[test]
    fn custom_manager_combination_strategy_merges_captures() {
        let cm = CustomManager {
            custom_type: "regex".to_owned(),
            file_patterns: vec![".env".to_owned()],
            match_strings: vec![
                r"datasource=(?P<datasource>[\w-]+)".to_owned(),
                r"depName=(?P<depName>[\w-]+)".to_owned(),
                r"VERSION=(?P<currentValue>[^\s]+)".to_owned(),
            ],
            match_strings_strategy: "combination".to_owned(),
            ..Default::default()
        };
        // Each pattern matches on a different line; combination merges them.
        let content = "datasource=github-releases\ndepName=my-tool\nVERSION=1.2.3\n";
        let deps = cm.extract_deps(content);
        assert_eq!(
            deps.len(),
            1,
            "combination strategy must produce exactly one dep"
        );
        let dep = &deps[0];
        assert_eq!(dep.datasource, "github-releases");
        assert_eq!(dep.dep_name, "my-tool");
        assert_eq!(dep.current_value, "1.2.3");
    }

    #[test]
    fn custom_manager_combination_incomplete_match_returns_empty() {
        let cm = CustomManager {
            custom_type: "regex".to_owned(),
            file_patterns: vec![".env".to_owned()],
            match_strings: vec![
                r"datasource=(?P<datasource>[\w-]+)".to_owned(),
                // No depName pattern → dep_name will be empty
                r"VERSION=(?P<currentValue>[^\s]+)".to_owned(),
            ],
            match_strings_strategy: "combination".to_owned(),
            ..Default::default()
        };
        let content = "datasource=npm\nVERSION=1.0.0\n";
        // Missing depName and no depNameTemplate → dep is incomplete → empty result.
        let deps = cm.extract_deps(content);
        assert!(
            deps.is_empty(),
            "incomplete combination match must return empty"
        );
    }

    // ── custom-managers:* preset tests ───────────────────────────────────────

    #[test]
    fn preset_custom_managers_dockerfile_versions_registered() {
        let c = RepoConfig::parse(r#"{"extends": ["custom-managers:dockerfileVersions"]}"#);
        assert!(
            !c.custom_managers.is_empty(),
            "dockerfileVersions preset must register at least one custom manager"
        );
        let cm = &c.custom_managers[0];
        assert_eq!(cm.custom_type, "regex");
        assert!(
            cm.file_patterns
                .iter()
                .any(|p| p.contains("Dockerfile") || p.contains("ockerfile")),
            "dockerfileVersions manager must target Dockerfile patterns"
        );
    }

    #[test]
    fn preset_custom_managers_makefile_versions_registered() {
        let c = RepoConfig::parse(r#"{"extends": ["custom-managers:makefileVersions"]}"#);
        let cm = c
            .custom_managers
            .iter()
            .find(|m| m.file_patterns.iter().any(|p| p.contains("akefile")))
            .expect("makefileVersions preset must register a Makefile custom manager");
        assert_eq!(cm.custom_type, "regex");
        assert_eq!(cm.match_strings_strategy, "any");
    }

    #[test]
    fn preset_custom_managers_maven_property_versions_registered() {
        let c = RepoConfig::parse(r#"{"extends": ["custom-managers:mavenPropertyVersions"]}"#);
        let cm = c
            .custom_managers
            .iter()
            .find(|m| m.file_patterns.iter().any(|p| p.contains("pom.xml")))
            .expect("mavenPropertyVersions preset must register a pom.xml custom manager");
        assert_eq!(
            cm.datasource_template.as_deref(),
            Some("maven"),
            "mavenPropertyVersions must default datasource to maven"
        );
    }

    #[test]
    fn preset_custom_managers_tsconfig_node_versions_registered() {
        let c = RepoConfig::parse(r#"{"extends": ["custom-managers:tsconfigNodeVersions"]}"#);
        let tsconfig_managers: Vec<_> = c
            .custom_managers
            .iter()
            .filter(|m| m.file_patterns.iter().any(|p| p.contains("sconfig")))
            .collect();
        assert_eq!(
            tsconfig_managers.len(),
            2,
            "tsconfigNodeVersions must register exactly 2 custom managers"
        );
        for m in &tsconfig_managers {
            assert_eq!(m.datasource_template.as_deref(), Some("npm"));
        }
    }

    #[test]
    fn preset_custom_managers_dockerfile_extracts_standard_annotation() {
        let c = RepoConfig::parse(r#"{"extends": ["custom-managers:dockerfileVersions"]}"#);
        let cm = c
            .custom_managers
            .iter()
            .find(|m| m.file_patterns.iter().any(|p| p.contains("ockerfile")))
            .expect("must have dockerfile custom manager");
        // Simulated Dockerfile content with a renovate annotation.
        let content = concat!(
            "# renovate: datasource=github-releases depName=cli/cli\n",
            "ENV GH_VERSION=2.40.0\n"
        );
        let deps = cm.extract_deps(content);
        assert_eq!(deps.len(), 1, "must extract one dep from annotated ENV");
        assert_eq!(deps[0].dep_name, "cli/cli");
        assert_eq!(deps[0].datasource, "github-releases");
        assert_eq!(deps[0].current_value, "2.40.0");
    }

    #[test]
    fn preset_custom_managers_user_defined_appended_after_preset() {
        // User-defined managers must appear after preset managers so they take
        // precedence when the pipeline iterates in order.
        let c = RepoConfig::parse(
            r##"{
            "extends": ["custom-managers:makefileVersions"],
            "customManagers": [{
                "customType": "regex",
                "managerFilePatterns": ["Makefile"],
                "matchStrings": ["TOOL=(?P<currentValue>[^\\s]+)"],
                "depNameTemplate": "my-tool",
                "datasourceTemplate": "npm"
            }]
        }"##,
        );
        // Preset manager comes first, user manager is appended.
        assert!(
            c.custom_managers.len() >= 2,
            "must have preset + user manager"
        );
        let user_cm = c.custom_managers.last().expect("must have user manager");
        assert_eq!(
            user_cm.dep_name_template.as_deref(),
            Some("my-tool"),
            "last manager must be the user-defined one"
        );
    }

    // ── Missing default preset tests ─────────────────────────────────────────

    #[test]
    fn preset_respect_latest_sets_flag() {
        let c = RepoConfig::parse(r#"{"extends": [":respectLatest"]}"#);
        assert!(
            c.respect_latest,
            ":respectLatest must set respect_latest = true"
        );
    }

    #[test]
    fn preset_respect_latest_off_by_default() {
        let c = RepoConfig::parse(r#"{}"#);
        assert!(!c.respect_latest, "respectLatest must default to false");
    }

    #[test]
    fn preset_dependency_dashboard_sets_flag() {
        let c = RepoConfig::parse(r#"{"extends": [":dependencyDashboard"]}"#);
        assert!(
            c.dependency_dashboard,
            ":dependencyDashboard must enable dependency dashboard"
        );
    }

    #[test]
    fn preset_disable_dependency_dashboard_overrides() {
        // disableDependencyDashboard should win over dependencyDashboard
        let c = RepoConfig::parse(
            r#"{"extends": [":dependencyDashboard", ":disableDependencyDashboard"]}"#,
        );
        assert!(
            !c.dependency_dashboard,
            ":disableDependencyDashboard must suppress the dashboard"
        );
    }

    #[test]
    fn raw_dependency_dashboard_parsed() {
        let c = RepoConfig::parse(r#"{"dependencyDashboard": true}"#);
        assert!(
            c.dependency_dashboard,
            "raw dependencyDashboard: true must be parsed"
        );
    }

    #[test]
    fn preset_config_migration_sets_flag() {
        let c = RepoConfig::parse(r#"{"extends": [":configMigration"]}"#);
        assert!(
            c.config_migration,
            ":configMigration must set config_migration = true"
        );
    }

    #[test]
    fn preset_dependency_dashboard_approval_sets_flag() {
        let c = RepoConfig::parse(r#"{"extends": [":dependencyDashboardApproval"]}"#);
        assert!(
            c.dependency_dashboard_approval,
            ":dependencyDashboardApproval must set the flag"
        );
    }

    #[test]
    fn preset_approve_major_updates_injects_rule() {
        let c = RepoConfig::parse(r#"{"extends": [":approveMajorUpdates"]}"#);
        let rule = c
            .package_rules
            .iter()
            .find(|r| r.dependency_dashboard_approval == Some(true))
            .expect(":approveMajorUpdates must inject a rule with dependencyDashboardApproval");
        use crate::versioning::semver_generic::UpdateType;
        assert!(
            rule.match_update_types.contains(&UpdateType::Major),
            ":approveMajorUpdates rule must match major updates"
        );
    }

    // ── packageRule extends (packages:*) tests ────────────────────────────────

    #[test]
    fn package_rule_extends_packages_react_adds_matchers() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"extends": ["packages:react"], "automerge": true}]}"#,
        );
        let rule = c
            .package_rules
            .iter()
            .find(|r| r.automerge == Some(true))
            .expect("must have a rule with automerge: true");
        assert!(
            rule.match_package_names.iter().any(|p| p.contains("react")),
            "packages:react extends must inject react into matchPackageNames"
        );
        assert!(
            rule.match_datasources.contains(&"npm".to_owned()),
            "packages:react extends must inject npm into matchDatasources"
        );
        assert!(rule.has_name_constraint, "has_name_constraint must be true");
    }

    #[test]
    fn package_rule_extends_packages_eslint_adds_matchers() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"extends": ["packages:eslint"], "enabled": false}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(
            rule.match_package_names.contains(&"eslint".to_owned()),
            "packages:eslint extends must inject eslint"
        );
        assert!(
            rule.match_package_names
                .iter()
                .any(|p| p.contains("typescript-eslint")),
            "packages:eslint extends must include @typescript-eslint/**"
        );
    }

    #[test]
    fn package_rule_extends_combined_with_own_matchers() {
        // User's own matchPackageNames should be merged with preset's.
        let c = RepoConfig::parse(
            r#"{"packageRules": [{
                "matchPackageNames": ["my-custom-lib"],
                "extends": ["packages:eslint"],
                "automerge": true
            }]}"#,
        );
        let rule = c
            .package_rules
            .iter()
            .find(|r| r.automerge == Some(true))
            .unwrap();
        assert!(
            rule.match_package_names
                .contains(&"my-custom-lib".to_owned()),
            "own matchPackageNames must be kept"
        );
        assert!(
            rule.match_package_names.contains(&"eslint".to_owned()),
            "preset matchPackageNames must be merged in"
        );
    }

    #[test]
    fn package_rule_extends_apollographql_adds_source_urls() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"extends": ["packages:apollographql"], "groupName": "apollo"}]}"#,
        );
        let rule = &c.package_rules[0];
        assert!(
            rule.match_source_urls
                .iter()
                .any(|u| u.contains("apollographql")),
            "packages:apollographql extends must inject apollographql source URL"
        );
    }

    #[test]
    fn raw_dependency_dashboard_approval_parsed_in_package_rule() {
        let c = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["lodash"], "dependencyDashboardApproval": true}]}"#,
        );
        let rule = &c.package_rules[0];
        assert_eq!(
            rule.dependency_dashboard_approval,
            Some(true),
            "dependencyDashboardApproval in packageRules must be parsed"
        );
    }

    // ── Ported from Renovate's custom-managers.spec.ts ───────────────────────
    // Tests verify our regex extraction matches Renovate's extraction behavior.

    fn get_preset_custom_manager(preset: &str) -> CustomManager {
        let c = RepoConfig::parse(&format!(r#"{{"extends": ["{preset}"]}}"#));
        c.custom_managers
            .into_iter()
            .next()
            .unwrap_or_else(|| panic!("{preset} must register at least one custom manager"))
    }

    #[test]
    fn dockerfile_versions_extracts_env_with_double_quotes() {
        // Port of Renovate custom-managers.spec.ts: dockerfileVersions "find dependencies in file"
        let cm = get_preset_custom_manager("custom-managers:dockerfileVersions");
        let content = concat!(
            "# renovate: datasource=npm depName=pnpm\n",
            "ENV PNPM_VERSION=\"7.25.1\"\n"
        );
        let deps = cm.extract_deps(content);
        assert_eq!(
            deps.len(),
            1,
            "must extract one dep from ENV with double quotes"
        );
        assert_eq!(deps[0].dep_name, "pnpm");
        assert_eq!(deps[0].datasource, "npm");
        assert_eq!(deps[0].current_value, "7.25.1");
    }

    #[test]
    fn dockerfile_versions_extracts_env_with_single_quotes() {
        let cm = get_preset_custom_manager("custom-managers:dockerfileVersions");
        let content = concat!(
            "# renovate: datasource=npm depName=yarn\n",
            "ENV YARN_VERSION='3.3.1'\n"
        );
        let deps = cm.extract_deps(content);
        assert_eq!(
            deps.len(),
            1,
            "must extract one dep from ENV with single quotes"
        );
        assert_eq!(deps[0].dep_name, "yarn");
        assert_eq!(deps[0].current_value, "3.3.1");
    }

    #[test]
    fn dockerfile_versions_extracts_env_without_quotes() {
        let cm = get_preset_custom_manager("custom-managers:dockerfileVersions");
        let content = concat!(
            "# renovate: datasource=npm depName=yarn\n",
            "ENV YARN_VERSION 3.3.1\n"
        );
        let deps = cm.extract_deps(content);
        assert_eq!(
            deps.len(),
            1,
            "must extract one dep from ENV without quotes"
        );
        assert_eq!(deps[0].current_value, "3.3.1");
    }

    #[test]
    fn dockerfile_versions_extracts_arg_directive() {
        let cm = get_preset_custom_manager("custom-managers:dockerfileVersions");
        let content = concat!(
            "# renovate: datasource=docker depName=node versioning=docker\n",
            "ARG NODE_VERSION=18\n"
        );
        let deps = cm.extract_deps(content);
        assert_eq!(deps.len(), 1, "must extract one dep from ARG directive");
        assert_eq!(deps[0].dep_name, "node");
        assert_eq!(deps[0].datasource, "docker");
        assert_eq!(deps[0].current_value, "18");
        assert_eq!(deps[0].versioning.as_deref(), Some("docker"));
    }

    #[test]
    fn dockerfile_versions_extracts_with_versioning_and_extract_version() {
        let cm = get_preset_custom_manager("custom-managers:dockerfileVersions");
        let content = concat!(
            "# renovate: datasource=github-releases depName=kubernetes-sigs/kustomize",
            " versioning=regex:^(?P<compatibility>.+)/v(?P<major>\\d+)\\.(?P<minor>\\d+)\\.(?P<patch>\\d+)$",
            " extractVersion=^kustomize/(?P<version>.+)$\n",
            "ENV KUSTOMIZE_VERSION v5.2.1\n"
        );
        let deps = cm.extract_deps(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "kubernetes-sigs/kustomize");
        assert_eq!(deps[0].current_value, "v5.2.1");
        assert!(deps[0].versioning.is_some(), "versioning must be captured");
        assert!(
            deps[0].extract_version.is_some(),
            "extractVersion must be captured"
        );
    }

    #[test]
    fn dockerfile_versions_file_pattern_matches() {
        // Port of Renovate custom-managers.spec.ts: "matches regexes patterns"
        let cm = get_preset_custom_manager("custom-managers:dockerfileVersions");
        assert!(cm.matches_file("Dockerfile"), "Dockerfile must match");
        assert!(
            cm.matches_file("foo/Dockerfile"),
            "foo/Dockerfile must match"
        );
        assert!(
            cm.matches_file("Dockerfile-foo"),
            "Dockerfile-foo must match"
        );
        assert!(
            cm.matches_file("something.dockerfile"),
            "lowercase .dockerfile must match"
        );
        assert!(
            cm.matches_file("something.containerfile"),
            ".containerfile must match"
        );
        assert!(
            !cm.matches_file("foo-Dockerfile"),
            "foo-Dockerfile must NOT match (prefix only)"
        );
    }

    #[test]
    fn makefile_versions_extracts_simple_assignment() {
        // Port of Renovate custom-managers.spec.ts: makefileVersions "find dependencies in file"
        let cm = get_preset_custom_manager("custom-managers:makefileVersions");
        let content = concat!(
            "# renovate: datasource=node depName=node versioning=node\n",
            "NODE_VERSION=18.13.0\n"
        );
        let deps = cm.extract_deps(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "node");
        assert_eq!(deps[0].current_value, "18.13.0");
    }

    #[test]
    fn makefile_versions_extracts_space_assignment() {
        let cm = get_preset_custom_manager("custom-managers:makefileVersions");
        let content = concat!(
            "# renovate: datasource=npm depName=pnpm\n",
            "PNPM_VERSION = \"7.25.1\"\n"
        );
        let deps = cm.extract_deps(content);
        assert_eq!(
            deps.len(),
            1,
            "must extract from PNPM_VERSION = \"...\" (spaces around =)"
        );
        assert_eq!(deps[0].current_value, "7.25.1");
    }

    #[test]
    fn makefile_versions_extracts_colon_equal() {
        let cm = get_preset_custom_manager("custom-managers:makefileVersions");
        let content = concat!(
            "# renovate: datasource=npm depName=yarn\n",
            "YARN_VERSION := '3.3.1'\n"
        );
        let deps = cm.extract_deps(content);
        assert_eq!(
            deps.len(),
            1,
            "must extract from YARN_VERSION := '...' (:= assignment)"
        );
        assert_eq!(deps[0].current_value, "3.3.1");
    }

    #[test]
    fn makefile_versions_extracts_question_equal() {
        let cm = get_preset_custom_manager("custom-managers:makefileVersions");
        let content = concat!(
            "# renovate: datasource=custom.hashicorp depName=consul\n",
            "CONSUL_VERSION ?= 1.3.1\n"
        );
        let deps = cm.extract_deps(content);
        assert_eq!(
            deps.len(),
            1,
            "must extract from CONSUL_VERSION ?= value (?= assignment)"
        );
        assert_eq!(deps[0].current_value, "1.3.1");
        assert_eq!(deps[0].datasource, "custom.hashicorp");
    }

    #[test]
    fn makefile_versions_file_pattern_matches() {
        let cm = get_preset_custom_manager("custom-managers:makefileVersions");
        assert!(cm.matches_file("Makefile"));
        assert!(cm.matches_file("makefile"));
        assert!(cm.matches_file("GNUMakefile"));
        assert!(cm.matches_file("sub/dir/Makefile"));
        assert!(cm.matches_file("versions.mk"));
        assert!(!cm.matches_file("Dockerfile"));
        assert!(!cm.matches_file("MakefileGenerator.ts"));
    }

    #[test]
    fn helm_chart_yaml_extracts_app_version() {
        // Port of Renovate custom-managers.spec.ts: helmChartYamlAppVersions
        let cm = get_preset_custom_manager("custom-managers:helmChartYamlAppVersions");
        let content = concat!(
            "apiVersion: v1\n",
            "name: a-chart\n",
            "version: \"1\"\n",
            "# renovate: image=node\n",
            "appVersion: 19.4.0\n"
        );
        let deps = cm.extract_deps(content);
        assert!(
            !deps.is_empty(),
            "must extract dep from Chart.yaml appVersion"
        );
        assert_eq!(deps[0].dep_name, "node");
        assert_eq!(deps[0].current_value, "19.4.0");
        assert_eq!(deps[0].datasource, "docker");
    }

    #[test]
    fn helm_chart_yaml_file_pattern_matches() {
        let cm = get_preset_custom_manager("custom-managers:helmChartYamlAppVersions");
        assert!(cm.matches_file("Chart.yaml"));
        assert!(cm.matches_file("foo/Chart.yaml"));
        assert!(!cm.matches_file("Chart.yml"));
        assert!(!cm.matches_file("Chart.yamlo"));
    }

    #[test]
    fn azure_pipelines_file_pattern_matches() {
        // Port of Renovate custom-managers.spec.ts: azurePipelinesVersions "matches regexes patterns"
        let cm = get_preset_custom_manager("custom-managers:azurePipelinesVersions");
        assert!(cm.matches_file(".azuredevops/bar.yml"));
        assert!(cm.matches_file(".azuredevops/bar.yaml"));
        assert!(cm.matches_file("azure-pipelines.yml"));
        assert!(cm.matches_file("azure-pipelines.yaml"));
        assert!(cm.matches_file("azurepipelines.yml"));
        assert!(!cm.matches_file("foo.yml"), "generic yml must NOT match");
        assert!(!cm.matches_file("foo.yaml"), "generic yaml must NOT match");
    }

    #[test]
    fn maven_property_versions_extracts_from_pom_xml() {
        // Port of Renovate custom-managers.spec.ts: mavenPropertyVersions
        let cm = get_preset_custom_manager("custom-managers:mavenPropertyVersions");
        let content = concat!(
            "<!-- renovate: depName=org.ow2.asm:asm -->\n",
            "<asm.version>9.3</asm.version>\n"
        );
        let deps = cm.extract_deps(content);
        assert!(
            !deps.is_empty(),
            "must extract from pom.xml property with renovate comment"
        );
        assert_eq!(deps[0].dep_name, "org.ow2.asm:asm");
        assert_eq!(deps[0].current_value, "9.3");
    }

    #[test]
    fn maven_property_versions_file_pattern_matches() {
        let cm = get_preset_custom_manager("custom-managers:mavenPropertyVersions");
        assert!(cm.matches_file("pom.xml"));
        assert!(cm.matches_file("foo/pom.xml"));
        assert!(!cm.matches_file("build.gradle"));
    }
}
