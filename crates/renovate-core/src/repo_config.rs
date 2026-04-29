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

    // ── Update grouping / limits ─────────────────────────────────────────────
    /// Maximum number of open Renovate PRs at any one time.  `0` = unlimited.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `prConcurrentLimit`.
    pub pr_concurrent_limit: u32,

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
            // The real preset also extends mergeConfidence, replacements, workarounds, and
            // helpers — those are either unsupported or deferred.
            "config:recommended" | "config:base" => {
                if seen.insert("config:recommended") {
                    // Key behavioral presets from config:recommended that we support:
                    result.push(":semanticPrefixFixDepsChoreOthers".to_owned());
                    result.push(":ignoreModulesAndTests".to_owned());
                    result.push("group:monorepos".to_owned());
                    result.push("group:recommended".to_owned());
                    // Keep the preset itself so downstream handlers (ignorePaths,
                    // group expansion) that match on "config:recommended" still fire.
                    result.push("config:recommended".to_owned());
                }
            }
            // config:best-practices extends config:recommended plus additional presets.
            "config:best-practices" => {
                if seen.insert("config:best-practices") {
                    result.push("config:recommended".to_owned());
                    result.push(":pinDevDependencies".to_owned());
                    // Keep for downstream handlers.
                    result.push("config:best-practices".to_owned());
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
///
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
            ":pathSemanticCommitType" | "pathSemanticCommitType" => {
                if args.len() >= 2 && !args[0].is_empty() && !args[1].is_empty() {
                    rules.push(PackageRule {
                        match_file_names: vec![args[0].to_string()],
                        semantic_commit_type: Some(args[1].to_string()),
                        ..Default::default()
                    });
                }
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
        match preset.as_str() {
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
        if preset == ":semanticPrefixFixDepsChoreOthers" {
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
                // monorepos groups monorepo packages together via large matchPackageNames lists.
                // Skip full expansion — those lists require network access to resolve.
                tracing::debug!(
                    "group:monorepos preset — partial support (grouped dep names not expanded)"
                );
            }
            // config:recommended transitively includes group:monorepos and group:recommended.
            // Expand group:recommended here so that `extends: ["config:recommended"]` users
            // get the full group preset treatment.
            "config:recommended" | "config:base" | "config:best-practices" => {
                let sub_extends = vec!["group:recommended".to_string()];
                let (sub_rules, _) = resolve_extends_group_presets(&sub_extends);
                rules.extend(sub_rules);
            }
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
            "label" | "labels" => {
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
            #[serde(rename = "matchPackagePatterns", default)]
            match_package_patterns: Vec<String>,
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
            #[serde(rename = "matchDatasources", default)]
            match_datasources: Vec<String>,
            #[serde(rename = "matchSourceUrls", default)]
            match_source_urls: Vec<String>,
            #[serde(rename = "matchManagers", default)]
            match_managers: Vec<String>,
            #[serde(rename = "matchUpdateTypes", default)]
            match_update_types: Vec<String>,
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
            #[serde(rename = "groupName")]
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
        }

        #[derive(Deserialize)]
        struct Raw {
            #[serde(default = "default_true")]
            enabled: bool,
            #[serde(rename = "ignoreDeps", default)]
            ignore_deps: Vec<String>,
            #[serde(rename = "ignorePaths", default)]
            ignore_paths: Vec<String>,
            #[serde(rename = "includePaths", default)]
            include_paths: Vec<String>,
            #[serde(rename = "packageRules", default)]
            package_rules: Vec<RawPackageRule>,
            #[serde(rename = "enabledManagers", default)]
            enabled_managers: Vec<String>,
            #[serde(rename = "disabledManagers", default)]
            disabled_managers: Vec<String>,
            #[serde(rename = "ignoreVersions", default)]
            ignore_versions: Vec<String>,
            #[serde(default)]
            schedule: Vec<String>,
            #[serde(rename = "automergeSchedule", default)]
            automerge_schedule: Vec<String>,
            timezone: Option<String>,
            #[serde(default)]
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
            #[serde(rename = "prConcurrentLimit", default)]
            pr_concurrent_limit: u32,
            #[serde(rename = "prHourlyLimit", default = "default_pr_hourly_limit")]
            pr_hourly_limit: u32,
            #[serde(rename = "groupName")]
            group_name: Option<String>,
            #[serde(rename = "separateMajorMinor", default = "default_true")]
            separate_major_minor: bool,
            #[serde(rename = "separateMultipleMajor", default)]
            separate_multiple_major: bool,
            #[serde(rename = "separateMinorPatch", default)]
            separate_minor_patch: bool,
            #[serde(rename = "separateMultipleMinor", default)]
            separate_multiple_minor: bool,
            #[serde(rename = "maxMajorIncrement", default = "default_max_major_increment")]
            max_major_increment: u32,
            #[serde(rename = "semanticCommits")]
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
            #[serde(default)]
            extends: Vec<String>,
            #[serde(rename = "ignorePresets", default)]
            ignore_presets: Vec<String>,
            #[serde(rename = "minimumReleaseAge")]
            minimum_release_age: Option<String>,
            #[serde(rename = "ignoreUnstable", default)]
            ignore_unstable: bool,
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
            #[serde(rename = "rangeStrategy", default = "default_range_strategy")]
            range_strategy: String,
            #[serde(rename = "hashedBranchLength")]
            hashed_branch_length: Option<u32>,
            major: Option<crate::package_rule::UpdateTypeConfig>,
            minor: Option<crate::package_rule::UpdateTypeConfig>,
            patch: Option<crate::package_rule::UpdateTypeConfig>,
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

        let raw: Raw = match json5::from_str(content) {
            Ok(r) => r,
            Err(e) => {
                tracing::debug!(%e, "failed to parse repo renovate config; using defaults");
                return Self::default();
            }
        };

        // Expand compound presets (presets that themselves extend other presets)
        // before filtering. This handles presets like config:js-app, config:js-lib,
        // config:semverAllMonthly, config:semverAllWeekly which are defined as
        // `extends: [other presets...]` in Renovate's preset registry.
        let expanded_extends = expand_compound_presets(&raw.extends);

        // Filter the extends list to remove any presets in `ignorePresets`.
        // This is evaluated before all preset resolution so ignored presets are
        // never expanded, matching Renovate's behaviour.
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
            .map(|r| {
                let has_name_constraint = !r.match_package_names.is_empty()
                    || !r.match_package_patterns.is_empty()
                    || !r.match_package_prefixes.is_empty();

                // Merge matchPackageNames, deprecated matchPackagePrefixes, and
                // deprecated matchPackagePatterns into one Vec<String> so that
                // match_regex_or_glob_list can apply positive/negative semantics.
                let mut match_package_names: Vec<String> = r.match_package_names;
                // matchPackagePrefixes → "prefix**" glob strings
                for prefix in r.match_package_prefixes {
                    match_package_names.push(format!("{prefix}**"));
                }
                // matchPackagePatterns → "/raw_regex/" inline strings
                for pat in r.match_package_patterns {
                    match_package_names.push(format!("/{pat}/"));
                }
                let has_update_type_constraint = !r.match_update_types.is_empty();
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
                PackageRule {
                    match_package_names,
                    match_dep_names,
                    match_source_urls: r.match_source_urls,
                    match_current_value: r.match_current_value,
                    match_new_value: r.match_new_value,
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
                    has_update_type_constraint,
                    group_name: r.group_name,
                    group_slug: r.group_slug,
                    automerge: r.automerge,
                    schedule: r.schedule,
                    labels: r.labels,
                    add_labels: r.add_labels,
                    assignees: r.assignees,
                    reviewers: r.reviewers,
                    match_categories: r.match_categories,
                    match_base_branches: r.match_base_branches,
                    match_registry_urls: r.match_registry_urls,
                    match_repositories: r.match_repositories,
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
        let mut enabled_managers = raw.enabled_managers;
        // Start with any managers explicitly disabled in the JSON config.
        let mut disabled_managers: Vec<String> = raw.disabled_managers;
        for preset in &effective_extends {
            match preset.as_str() {
                ":enablePreCommit" | "enablePreCommit" => {
                    if !enabled_managers.contains(&"pre-commit".to_owned()) {
                        enabled_managers.push("pre-commit".to_owned());
                    }
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
            base_branches: raw.base_branches,
            pr_concurrent_limit: scalar_pr_concurrent.unwrap_or(raw.pr_concurrent_limit),
            pr_hourly_limit: scalar_pr_hourly.unwrap_or(raw.pr_hourly_limit),
            group_name: raw.group_name,
            // group:all preset implies separateMajorMinor: false.
            // Explicit user config overrides the preset (but default true from serde means
            // we can't distinguish user-set vs default; group preset wins only when
            // the user hasn't explicitly set it to true in the raw JSON).
            separate_major_minor: scalar_sep_major_minor
                .or(group_separate_major_minor)
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
                preset_paths.extend(raw.ignore_paths);
                preset_paths
            },
            include_paths: raw.include_paths,
            extends: raw.extends,
            ignore_presets: raw.ignore_presets,
            minimum_release_age: raw.minimum_release_age,
            ignore_unstable: raw.ignore_unstable || preset_ignore_unstable,
            update_not_scheduled: preset_update_not_scheduled.unwrap_or(raw.update_not_scheduled),
            commit_message_action: raw.commit_message_action,
            commit_message_prefix: raw.commit_message_prefix,
            commit_message_extra: raw.commit_message_extra,
            commit_message_suffix: raw.commit_message_suffix,
            range_strategy: raw.range_strategy,
            hashed_branch_length: raw.hashed_branch_length,
            major_config: raw.major,
            minor_config: raw.minor,
            patch_config: raw.patch,
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
            pr_concurrent_limit: 0,
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
            update_not_scheduled: true,
            commit_message_action: "Update".to_owned(),
            commit_message_prefix: None,
            commit_message_extra: None,
            commit_message_suffix: None,
            range_strategy: "auto".to_owned(),
            hashed_branch_length: None,
            major_config: None,
            minor_config: None,
            patch_config: None,
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
        assert!(rule.current_version_matches("0.1.0"));
        // "1.0.0" starts with "1", does NOT match /^0/
        assert!(!rule.current_version_matches("1.0.0"));
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
        assert!(!rule.current_version_matches("0.1.0"));
        // "1.0.0" does NOT match /^0/ → !/^0/ is TRUE
        assert!(rule.current_version_matches("1.0.0"));
        // "2.5.3" does NOT match /^0/ → !/^0/ is TRUE
        assert!(rule.current_version_matches("2.5.3"));
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
        let group_names: Vec<&str> = c
            .package_rules
            .iter()
            .filter_map(|r| r.group_name.as_deref())
            .collect();
        assert!(
            group_names.contains(&"Node.js"),
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

    // ── scalar config presets ─────────────────────────────────────────────────

    #[test]
    fn combine_patch_minor_releases_clears_separate_minor_patch() {
        let c = RepoConfig::parse(r#"{"extends": ["combinePatchMinorReleases"]}"#);
        assert!(!c.separate_minor_patch);
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

    // ── per-rule minimumReleaseAge in RuleEffects ────────────────────────────

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
        assert!(rule.update_type_matches(UpdateType::Minor));
        assert!(rule.update_type_matches(UpdateType::Patch));
        assert!(!rule.update_type_matches(UpdateType::Major));
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
        assert!(rule.update_type_matches(UpdateType::Major));
        assert!(!rule.update_type_matches(UpdateType::Minor));
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
}
