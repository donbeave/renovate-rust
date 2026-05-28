//! `packageRules` evaluation — compiled rule types, matchers, and context.
//!
//! This module holds the types that represent Renovate's `packageRules` system:
//! the compiled rule struct, the dependency-matching context, and all the
//! per-field matcher methods.  Config discovery and the top-level `RepoConfig`
//! struct live in [`crate::repo_config`].
//!
//! Renovate reference:
//! - `lib/config/options/index.ts` — `packageRules` option spec
//! - `lib/util/package-rules/index.ts` — `matchesRule()` evaluation loop
//! - `lib/util/package-rules/*.ts` — individual matcher modules

use globset::{Glob, GlobSet, GlobSetBuilder};
use regex::Regex;
use serde::Deserialize;

use crate::managers::manager_categories;
use crate::versioning::semver_generic::UpdateType;

// ── PackageRule ───────────────────────────────────────────────────────────────

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
#[derive(Debug, Clone, Default)]
pub struct PackageRule {
    /// Package name patterns targeting the `packageName` / `depName` field.
    ///
    /// Populated from `matchPackageNames` (as-is), the deprecated
    /// `matchPackagePrefixes` (converted to `prefix**` globs), and the
    /// deprecated `matchPackagePatterns` (wrapped as `/regex/` strings).
    /// Supports exact, `/regex/`, glob, and `!negation` patterns via
    /// `match_regex_or_glob_list`.
    ///
    /// Renovate reference: `lib/util/package-rules/package-names.ts`
    pub match_package_names: Vec<String>,
    /// Dep name patterns from `matchDepNames`.
    /// Targets the `depName` field (may differ from `packageName`).
    /// Supports exact, `/regex/`, glob, and `!negation` patterns.
    ///
    /// Renovate reference: `lib/util/package-rules/dep-names.ts`
    pub match_dep_names: Vec<String>,
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
    /// Force-override for `enabled`.  From `force: { enabled: ... }` in the JSON config.
    /// `Some(true)` overrides any `enabled: false` from config or prior rules (vulnerability
    /// alert use case).  `Some(false)` sets skipReason regardless of `enabled: true`.
    /// Takes precedence over regular `enabled` in the skip-reason evaluation.
    ///
    /// Renovate reference: `lib/util/package-rules/index.ts` — `toApply.force?.enabled`
    pub force_enabled: Option<bool>,
    /// Version strings/ranges/regex patterns to ignore for packages matched
    /// by this rule.  Mirrors `ignoreVersions` in Renovate packageRules.
    pub ignore_versions: Vec<String>,
    /// Source URL patterns from `matchSourceUrls`.
    /// Supports exact strings, `/regex/`, glob, and `!negation` patterns.
    /// When non-empty, a dep's `sourceUrl` must match at least one entry.
    ///
    /// Renovate reference: `lib/util/package-rules/sourceurls.ts`
    pub match_source_urls: Vec<String>,
    /// Single regex/glob pattern to match against the raw `currentValue` string.
    /// Supports `/regex/flags`, glob, or exact strings.
    ///
    /// Renovate reference: `lib/util/package-rules/current-value.ts`
    pub match_current_value: Option<String>,
    /// Single regex/glob pattern to match against the proposed new version string.
    /// Supports `/regex/flags`, glob, or exact strings.
    ///
    /// Renovate reference: `lib/util/package-rules/new-value.ts`
    pub match_new_value: Option<String>,
    /// `true` when any `matchPackageNames` / `matchPackagePatterns` /
    /// `matchPackagePrefixes` entry was set (non-empty `match_package_names`).
    pub has_name_constraint: bool,
    /// `true` when `matchUpdateTypes` was set in the raw config, even if all
    /// specified types are unrecognized (e.g. `"pin"`, `"digest"`).
    /// When `true` and `match_update_types` is empty, the rule can never match.
    pub has_update_type_constraint: bool,

    // ── Per-rule metadata (applied when this rule matches) ───────────────────
    /// Group name for this rule's matching dependencies.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `groupName`.
    pub group_name: Option<String>,

    /// Explicit slug for the group branch topic.  When set, overrides the
    /// auto-computed `slugify(groupName)` used by `group_branch_topic()`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `groupSlug`.
    pub group_slug: Option<String>,

    /// Per-rule auto-merge override.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `automerge`.
    pub automerge: Option<bool>,

    /// Per-rule schedule override.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `schedule`.
    pub schedule: Vec<String>,

    /// Per-rule labels to add to PRs for matching deps.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `labels`.
    pub labels: Vec<String>,

    /// Additional labels appended to (not replacing) the existing label set.
    /// `mergeable: true` in Renovate — accumulates from all matching rules.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `addLabels`.
    pub add_labels: Vec<String>,

    /// Per-rule GitHub usernames/team slugs to assign as PR assignees.
    /// Overrides the repo-level `assignees` for matching deps.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `assignees`.
    pub assignees: Vec<String>,

    /// Per-rule GitHub usernames/team slugs to add as PR reviewers.
    /// Overrides the repo-level `reviewers` for matching deps.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `reviewers`.
    pub reviewers: Vec<String>,

    // ── Category / base-branch constraints ───────────────────────────────────
    /// Ecosystem category strings to match (e.g. `["js"]`, `["python", "rust"]`).
    ///
    /// Renovate reference: `lib/util/package-rules/categories.ts`
    pub match_categories: Vec<String>,

    /// Base branch patterns this rule applies to.
    ///
    /// Renovate reference: `lib/util/package-rules/base-branch.ts`
    pub match_base_branches: Vec<String>,

    // ── Registry URL constraint ───────────────────────────────────────────────
    /// Registry URL patterns from `matchRegistryUrls`.
    /// Supports exact strings, `/regex/`, glob, and `!negation` patterns.
    ///
    /// Renovate reference: `lib/util/package-rules/registryurls.ts`
    pub match_registry_urls: Vec<String>,

    // ── Repository constraint ─────────────────────────────────────────────────
    /// Repository name patterns from `matchRepositories`.
    /// Supports exact strings, `/regex/`, glob, and `!negation` patterns.
    ///
    /// Renovate reference: `lib/util/package-rules/repositories.ts`
    pub match_repositories: Vec<String>,

    // ── Age-based constraint ──────────────────────────────────────────────────
    /// Age range expression for the **currently installed** version.
    ///
    /// Renovate reference: `lib/util/package-rules/current-age.ts`
    pub match_current_age: Option<String>,

    // ── Per-rule update controls ──────────────────────────────────────────────
    /// Per-rule minimum release age.  When set, supersedes the global
    /// `minimumReleaseAge` for matching packages.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `minimumReleaseAge`.
    pub minimum_release_age: Option<String>,

    /// Pull Request creation priority.  Higher numbers are created first.
    /// Negative values are created last.  Default: `0`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `prPriority`.
    pub pr_priority: Option<i32>,

    /// Custom commit message topic template.  Supports `{{depName}}` and
    /// `{{{depName}}}` substitution.  `None` uses the default
    /// `"dependency {depName}"` topic.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `commitMessageTopic`.
    pub commit_message_topic: Option<String>,

    /// Per-rule override for the commit-message action verb (e.g. `"Pin"`,
    /// `"Roll back"`).  `None` = no override (use repo-level or default).
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `commitMessageAction`.
    pub commit_message_action: Option<String>,

    /// Per-rule prefix prepended to the commit message / PR title
    /// (e.g. `"fix(deps):"` for security presets).
    /// When set, it replaces `semanticCommits`-generated prefixes.
    /// `None` = no override.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `commitMessagePrefix`.
    pub commit_message_prefix: Option<String>,

    /// Per-rule semantic commit type (e.g. `"fix"`, `"feat"`, `"chore"`).
    /// Overrides the repo-level `semanticCommitType` for matching deps.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `semanticCommitType`.
    pub semantic_commit_type: Option<String>,

    /// Per-rule semantic commit scope (e.g. `"security"`, `"deps"`).
    /// Overrides the repo-level `semanticCommitScope` for matching deps.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `semanticCommitScope`.
    pub semantic_commit_scope: Option<String>,

    /// Per-rule override for the extra segment of the commit message
    /// (e.g. `"({{newVersion}})"` or `""`).  `None` = use default "to {version}".
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `commitMessageExtra`.
    pub commit_message_extra: Option<String>,

    /// Free-form suffix appended at the end of the commit message / PR title.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `commitMessageSuffix`.
    pub commit_message_suffix: Option<String>,

    /// Version range strategy override for matching packages.
    /// Values: `"auto"`, `"pin"`, `"replace"`, `"widen"`, `"bump"`, `"in-range-only"`.
    /// `None` = use repo-level `rangeStrategy`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `rangeStrategy`.
    pub range_strategy: Option<String>,
    /// Versioning scheme override for matching packages.
    /// E.g. `"semver"`, `"docker"`, `"regex:..."`.
    /// `None` = use the manager's default versioning.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `versioning`.
    pub versioning: Option<String>,
    /// When `Some(true)`, pin Docker images to their digest.
    /// When `Some(false)`, disable digest pinning.
    /// `None` = use the manager's default behavior.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `pinDigests`.
    pub pin_digests: Option<bool>,
    /// Specific npm dist-tag to follow (e.g. `"next"`, `"beta"`).
    /// When set, Renovate pins the package to the specified tag's latest version.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `followTag`.
    pub follow_tag: Option<String>,
    /// Replacement package name when the dep is being migrated to a different package.
    /// E.g. `"replacementName": "@babel/eslint-parser"` to replace `"babel-eslint"`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `replacementName`.
    pub replacement_name: Option<String>,
    /// Replacement version constraint when migrating to a different package.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `replacementVersion`.
    pub replacement_version: Option<String>,
    /// Regex pattern used to separate a version from its compatibility suffix
    /// (e.g. `"^(?<version>[^-]+)(?<compatibility>-.*)?$"` for node images).
    /// Stored and forwarded to the updater; not evaluated during dep scanning.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `versionCompatibility`.
    pub version_compatibility: Option<String>,
    /// Custom changelog URL template.  When set, Renovate puts this URL in the
    /// PR body instead of auto-detecting the changelog.  Supports Handlebars
    /// templates (e.g. `{{sourceUrl}}/compare/{{currentDigest}}..{{newDigest}}`).
    /// Stored and used during PR generation; not evaluated during dep scanning.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `changelogUrl`.
    pub changelog_url: Option<String>,
    /// Override source URL for matching deps. Stored after lightweight template
    /// rendering by `RepoConfig::collect_rule_effects`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `sourceUrl`.
    pub source_url: Option<String>,
    /// Changelog fetching mode for matching deps, e.g. `"off"`.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `fetchChangeLogs`.
    pub fetch_change_logs: Option<String>,
    /// When `Some(true)`, requires Dependency Dashboard approval before Renovate
    /// creates a PR for matching deps.  Forwarded to PR generator.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `dependencyDashboardApproval`.
    pub dependency_dashboard_approval: Option<bool>,
    /// Override the datasource value for matching deps.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `overrideDatasource`.
    pub override_datasource: Option<String>,
    /// Override the dep name for matching deps.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `overrideDepName`.
    pub override_dep_name: Option<String>,
    /// Override the lookup package name for matching deps.
    ///
    /// Renovate reference: `lib/config/options/index.ts` — `overridePackageName`.
    pub override_package_name: Option<String>,
}

// ── impl PackageRule ──────────────────────────────────────────────────────────

impl PackageRule {
    /// Return `true` when this rule's name conditions match `dep_name`.
    ///
    /// Checks `match_package_names` (which merges `matchPackageNames`,
    /// deprecated `matchPackagePrefixes` as `prefix**` globs, and deprecated
    /// `matchPackagePatterns` wrapped as `/regex/` strings) via
    /// `match_regex_or_glob_list` — supporting exact, regex, glob, and negation.
    pub fn name_matches(&self, dep_name: &str) -> bool {
        use crate::string_match::match_regex_or_glob_list;
        if !self.has_name_constraint {
            return true;
        }
        match_regex_or_glob_list(dep_name, &self.match_package_names)
    }

    /// Return `true` when this rule's `matchDepNames` condition matches `dep_name`.
    ///
    /// Supports exact, `/regex/`, glob, and `!negation` patterns.
    pub fn dep_name_matches(&self, dep_name: &str) -> bool {
        use crate::string_match::match_regex_or_glob_list;
        self.match_dep_names.is_empty() || match_regex_or_glob_list(dep_name, &self.match_dep_names)
    }

    /// Return `true` when this rule's `matchDatasources` condition matches `datasource`.
    ///
    /// Supports exact, `/regex/`, glob, and `!negation` patterns.
    ///
    /// Renovate reference: `lib/util/package-rules/datasources.ts`
    pub fn datasource_matches(&self, datasource: &str) -> bool {
        use crate::string_match::match_regex_or_glob_list;
        self.match_datasources.is_empty()
            || match_regex_or_glob_list(datasource, &self.match_datasources)
    }

    /// Return `true` when this rule's `matchSourceUrls` condition matches `source_url`.
    ///
    /// Supports exact, `/regex/`, glob, and `!negation` patterns.
    pub fn source_url_matches(&self, source_url: &str) -> bool {
        use crate::string_match::match_regex_or_glob_list;
        self.match_source_urls.is_empty()
            || match_regex_or_glob_list(source_url, &self.match_source_urls)
    }

    /// Return `true` when this rule's `matchCurrentValue` pattern matches `current_value`.
    ///
    /// Supports `/regex/flags`, glob, and exact strings.  When `None`, matches all.
    pub fn current_value_matches(&self, current_value: &str) -> bool {
        use crate::string_match::match_regex_or_glob;
        match &self.match_current_value {
            None => true,
            Some(pattern) => match_regex_or_glob(current_value, pattern),
        }
    }

    /// Return `true` when this rule's `matchNewValue` pattern matches `new_value`.
    ///
    /// Supports `/regex/flags`, glob, and exact strings.  When `None`, matches all.
    pub fn new_value_matches(&self, new_value: &str) -> bool {
        use crate::string_match::match_regex_or_glob;
        match &self.match_new_value {
            None => true,
            Some(pattern) => match_regex_or_glob(new_value, pattern),
        }
    }

    /// Return `true` if `proposed_version` matches any entry in this rule's
    /// `ignoreVersions` list.
    pub fn version_is_ignored(&self, proposed_version: &str) -> bool {
        version_matches_ignore_list(proposed_version, &self.ignore_versions)
    }

    /// Return `true` when this rule's manager condition matches `manager`.
    ///
    /// Supports exact, `/regex/`, glob, and `!negation` patterns.
    /// Custom managers (`"regex"`, `"jsonata"`) are matched as `"custom.<id>"`.
    ///
    /// Renovate reference: `lib/util/package-rules/managers.ts`
    pub fn manager_matches(&self, manager: &str) -> bool {
        use crate::string_match::match_regex_or_glob_list;
        if self.match_managers.is_empty() {
            return true;
        }
        const CUSTOM_MANAGERS: &[&str] = &["regex", "jsonata"];
        let effective = if CUSTOM_MANAGERS.contains(&manager) {
            std::borrow::Cow::Owned(format!("custom.{manager}"))
        } else {
            std::borrow::Cow::Borrowed(manager)
        };
        match_regex_or_glob_list(&effective, &self.match_managers)
    }

    /// Return `true` when this rule's update type condition matches.
    ///
    /// When `is_bump` is true, `UpdateType::Bump` is also checked, mirroring
    /// Renovate's `UpdateTypesMatcher` which adds `'bump'` to the checked set
    /// when `isBump` is set on the dep.
    pub fn update_type_matches(&self, update_type: UpdateType, is_bump: bool) -> bool {
        if self.has_update_type_constraint && self.match_update_types.is_empty() {
            // All specified update types were unrecognized — constraint never matches.
            return false;
        }
        if self.match_update_types.is_empty() {
            return true;
        }
        if self.match_update_types.contains(&update_type) {
            return true;
        }
        is_bump && self.match_update_types.contains(&UpdateType::Bump)
    }

    /// Return `true` when this rule's dep type condition matches `dep_type`.
    ///
    /// Supports exact, `/regex/`, glob, and `!negation` patterns.
    ///
    /// Renovate reference: `lib/util/package-rules/dep-types.ts`
    pub fn dep_type_matches(&self, dep_type: &str) -> bool {
        use crate::string_match::match_regex_or_glob_list;
        self.match_dep_types.is_empty() || match_regex_or_glob_list(dep_type, &self.match_dep_types)
    }

    /// Return `true` when `path` matches this rule's `matchFileNames` patterns.
    pub fn file_name_matches(&self, path: &str) -> bool {
        if self.match_file_names.is_empty() {
            return true;
        }
        PathMatcher::new(&self.match_file_names).is_ignored(path)
    }

    /// Return `true` when the dep's version context satisfies this rule's `matchCurrentVersion`.
    ///
    /// Mirrors Renovate's `CurrentVersionMatcher` three-way dispatch:
    ///
    /// 1. **Regex pattern** (`/re/` or `!/re/`):
    ///    Test against `locked_version ?? current_version ?? current_value`.
    ///
    /// 2. **Plain version** (matchCurrentVersion parses as a semver version):
    ///    Returns true when `matchCurrentVersion` is a version that satisfies the
    ///    `current_value` constraint (i.e. the installed version is in the declared range).
    ///    `isUnconstrainedValue` short-circuit: if lockedVersion is set and currentValue
    ///    is absent, return true.
    ///
    /// 3. **Semver range** (matchCurrentVersion is a constraint like `<= 2.0.0`):
    ///    Compare against `current_value` if it parses as a version, otherwise fall back
    ///    to `locked_version ?? current_version`.  Return false if no comparable version.
    pub fn current_version_matches(
        &self,
        current_value: &str,
        current_version: Option<&str>,
        locked_version: Option<&str>,
    ) -> bool {
        use crate::string_match::match_regex_or_glob;
        use crate::versioning::semver_generic::parse_padded;
        let Some(ref mcv) = self.match_current_version else {
            return true;
        };

        // Case 1: regex pattern — test against lockedVersion ?? currentVersion ?? currentValue.
        if mcv.starts_with('/') || mcv.starts_with("!/") {
            let compare = locked_version.or(current_version).unwrap_or(current_value);
            if compare.is_empty() {
                return false;
            }
            return match_regex_or_glob(compare, mcv);
        }

        // Case 2: matchCurrentVersion is itself a plain version (e.g. "2.1.0", "4.6.0").
        // Check whether that version satisfies the currentValue range.
        if let Some(mcv_sv) = parse_padded(mcv) {
            // isUnconstrainedValue: lockedVersion set but currentValue absent → always match.
            if locked_version.is_some() && current_value.is_empty() {
                return true;
            }
            if current_value.is_empty() {
                return false;
            }
            // currentValue must be a valid semver range; check if matchCurrentVersion is in it.
            return semver::VersionReq::parse(current_value)
                .map(|req| req.matches(&mcv_sv))
                .unwrap_or(false);
        }

        // Case 3: matchCurrentVersion is a semver range (e.g. "<= 2.0.0", ">= 1.0").
        // Use currentValue if it's a plain version, else fall back to lockedVersion ?? currentVersion.
        let compare_sv = if let Some(cv_sv) = parse_padded(current_value) {
            cv_sv
        } else {
            // currentValue is a range — use lockedVersion ?? currentVersion.
            let Some(fallback) = locked_version.or(current_version) else {
                return false;
            };
            let Some(sv) = parse_padded(fallback) else {
                return false;
            };
            sv
        };
        semver::VersionReq::parse(mcv)
            .map(|req| req.matches(&compare_sv))
            .unwrap_or(false)
    }

    /// Return `true` when this rule's `matchRegistryUrls` condition matches.
    ///
    /// Mirrors Renovate: any dep registry URL that satisfies the pattern list
    /// (with positive/negative semantics) → match.
    pub fn registry_url_matches(&self, registry_urls: &[&str]) -> bool {
        use crate::string_match::match_regex_or_glob_list;
        if self.match_registry_urls.is_empty() {
            return true;
        }
        registry_urls
            .iter()
            .any(|url| match_regex_or_glob_list(url, &self.match_registry_urls))
    }

    /// Return `true` when this rule's `matchRepositories` condition matches.
    ///
    /// Supports exact, `/regex/`, glob, and `!negation` patterns.
    pub fn repository_matches(&self, repository: &str) -> bool {
        use crate::string_match::match_regex_or_glob_list;
        self.match_repositories.is_empty()
            || match_regex_or_glob_list(repository, &self.match_repositories)
    }

    /// Return `true` when ALL matchers in this rule fire for `ctx`.
    ///
    /// Mirrors Renovate's `matchesRule()` from `lib/util/package-rules/index.ts`.
    /// Each matcher AND-s with the others; missing context fields are treated as
    /// "unknown → the rule's constraint cannot be satisfied → false".
    pub fn matches_context(&self, ctx: &DepContext<'_>) -> bool {
        let name = ctx.package_name.unwrap_or(ctx.dep_name);
        if !self.name_matches(name) {
            return false;
        }
        if !self.dep_name_matches(ctx.dep_name) {
            return false;
        }

        match ctx.manager {
            Some(mgr) => {
                if !self.manager_matches(mgr) {
                    return false;
                }
                // Prefer explicitly-provided dep categories; fall back to manager-derived.
                let cats: &[&str] = if !ctx.categories.is_empty() {
                    ctx.categories
                } else {
                    manager_categories(mgr)
                };
                if !self.categories_match(cats) {
                    return false;
                }
            }
            None => {
                if !self.match_managers.is_empty() {
                    return false;
                }
                // No manager: use explicit dep categories if provided.
                if !self.match_categories.is_empty() {
                    if ctx.categories.is_empty() {
                        return false;
                    }
                    if !self.categories_match(ctx.categories) {
                        return false;
                    }
                }
            }
        }

        match ctx.datasource {
            Some(ds) => {
                if !self.datasource_matches(ds) {
                    return false;
                }
            }
            None => {
                if !self.match_datasources.is_empty() {
                    return false;
                }
            }
        }

        // matchDepTypes: use dep_type first; fall back to dep_types (plural array).
        // Mirrors Renovate's DepTypesMatcher which checks depType then depTypes.
        if !self.match_dep_types.is_empty() {
            if let Some(dt) = ctx.dep_type {
                if !self.dep_type_matches(dt) {
                    return false;
                }
            } else if !ctx.dep_types.is_empty() {
                use crate::string_match::any_match_regex_or_glob_list;
                if !any_match_regex_or_glob_list(ctx.dep_types, &self.match_dep_types) {
                    return false;
                }
            } else {
                return false;
            }
        }

        // matchFileNames checks packageFile first, then falls back to lockFiles.
        // Mirrors Renovate's FileNamesMatcher which calls anyMatchRegexOrGlobList(lockFiles, patterns).
        if !self.match_file_names.is_empty() {
            let file = ctx.file_path.unwrap_or("");
            let pkg_matches = !file.is_empty() && self.file_name_matches(file);
            if !pkg_matches {
                use crate::string_match::any_match_regex_or_glob_list;
                if ctx.lock_files.is_empty()
                    || !any_match_regex_or_glob_list(ctx.lock_files, &self.match_file_names)
                {
                    return false;
                }
            }
        }

        match ctx.source_url {
            Some(url) => {
                if !self.source_url_matches(url) {
                    return false;
                }
            }
            None => {
                if !self.match_source_urls.is_empty() {
                    return false;
                }
            }
        }

        match ctx.registry_urls {
            Some(urls) => {
                if !self.registry_url_matches(urls) {
                    return false;
                }
            }
            None => {
                if !self.match_registry_urls.is_empty() {
                    return false;
                }
            }
        }

        match ctx.repository {
            Some(repo) => {
                if !self.repository_matches(repo) {
                    return false;
                }
            }
            None => {
                if !self.match_repositories.is_empty() {
                    return false;
                }
            }
        }

        let base = ctx.base_branch.unwrap_or("");
        if !self.base_branch_matches(base) {
            return false;
        }

        let current_val = ctx.current_value.unwrap_or("");
        if !self.current_value_matches(current_val) {
            return false;
        }
        if !self.current_version_matches(current_val, ctx.current_version, ctx.locked_version) {
            return false;
        }

        let new_val = ctx.new_value.unwrap_or("");
        if !self.new_value_matches(new_val) {
            return false;
        }

        if let Some(ut) = ctx.update_type
            && !self.update_type_matches(ut, ctx.is_bump)
        {
            return false;
        }

        if !self.current_age_matches(ctx.current_version_timestamp) {
            return false;
        }

        true
    }

    /// Return `true` when this rule's `matchCategories` condition matches.
    pub fn categories_match(&self, categories: &[&str]) -> bool {
        if self.match_categories.is_empty() {
            return true;
        }
        self.match_categories
            .iter()
            .any(|c| categories.contains(&c.as_str()))
    }

    /// Return `true` when this rule's `matchBaseBranches` condition matches.
    pub fn base_branch_matches(&self, branch: &str) -> bool {
        if self.match_base_branches.is_empty() {
            return true;
        }
        crate::string_match::match_regex_or_glob_list(branch, &self.match_base_branches)
    }

    /// Return `true` when this rule's `matchCurrentAge` condition is satisfied.
    ///
    /// `timestamp` is the ISO 8601 release date of the currently-installed version.
    pub fn current_age_matches(&self, timestamp: Option<&str>) -> bool {
        let Some(ref range) = self.match_current_age else {
            return true;
        };
        let Some(ts) = timestamp else {
            return false;
        };
        crate::schedule::satisfies_date_range(ts, range)
    }
}

// ── DepContext ────────────────────────────────────────────────────────────────

/// Matching context for evaluating a dependency against `packageRules`.
///
/// All fields are `Option` so partial contexts (e.g., before update lookups)
/// are safe to use.  Missing fields follow upstream semantics:
/// - If the **rule** constraint is unset → matcher returns true (match-all).
/// - If the **rule** constraint IS set but the **context** field is `None` →
///   matcher returns false (the constraint cannot be satisfied).
///
/// Renovate reference: `PackageRuleInputConfig` in `lib/config/types.ts`.
#[derive(Debug, Default)]
pub struct DepContext<'a> {
    /// The `depName` field (logical dep name, used by `matchDepNames`).
    pub dep_name: &'a str,
    /// The `packageName` field.  When `None`, `dep_name` is used for
    /// `matchPackageNames` too.
    pub package_name: Option<&'a str>,
    /// Manager that detected this dep (e.g. `"npm"`, `"cargo"`).
    pub manager: Option<&'a str>,
    /// Datasource identifier (e.g. `"npm"`, `"pypi"`, `"docker"`).
    pub datasource: Option<&'a str>,
    /// Dep classification within the manifest (e.g. `"dependencies"`).
    /// Mirrors Renovate's `depType` (singular).  Used by `matchDepTypes` first.
    pub dep_type: Option<&'a str>,
    /// Multiple dep type classifications (e.g. `["build", "test"]`).
    /// Mirrors Renovate's `depTypes` (plural array) — used by `matchDepTypes`
    /// when `dep_type` is absent, checking if any element matches.
    pub dep_types: &'a [&'a str],
    /// Relative manifest file path.
    pub file_path: Option<&'a str>,
    /// Lock file paths associated with this dep (e.g. `["yarn.lock", "package-lock.json"]`).
    /// Mirrors Renovate's `lockFiles` field; checked by `matchFileNames` when
    /// `packageFile` (our `file_path`) doesn't match.
    pub lock_files: &'a [&'a str],
    /// Source repository URL reported by the datasource.
    pub source_url: Option<&'a str>,
    /// Registry URLs used by the dep (from manifest or datasource).
    pub registry_urls: Option<&'a [&'a str]>,
    /// Current repository name (`"owner/repo"`).
    pub repository: Option<&'a str>,
    /// Current base branch (e.g. `"main"`, `"develop"`).
    pub base_branch: Option<&'a str>,
    /// Raw current version string from the manifest (may be a range like `"^1.0.0"`).
    pub current_value: Option<&'a str>,
    /// Resolved current version for the dep (e.g. `"1.0.3"`).
    /// Mirrors Renovate's `currentVersion` field — the exact version in use,
    /// which may differ from `current_value` when `current_value` is a range.
    /// Used by `matchCurrentVersion` semver-range matching as the compare target.
    pub current_version: Option<&'a str>,
    /// Resolved exact version pinned in the lockfile (e.g. `"1.2.3"`).
    /// When present, `matchCurrentVersion` regex patterns test against this
    /// instead of `currentValue`, matching Renovate's `lockedVersion` field.
    pub locked_version: Option<&'a str>,
    /// Proposed new version string (after datasource lookup).
    pub new_value: Option<&'a str>,
    /// Classified update type (available after version lookup).
    pub update_type: Option<UpdateType>,
    /// ISO 8601 release timestamp for the **currently installed** version.
    pub current_version_timestamp: Option<&'a str>,
    /// When `true`, this dep represents a range bump (no new upstream version).
    /// Mirrors Renovate's `isBump` field; adds virtual `"bump"` update type to
    /// `matchUpdateTypes` matching.
    pub is_bump: bool,
    /// Explicit ecosystem categories for this dep (e.g. `&["javascript", "node"]`).
    /// When non-empty, used directly by `matchCategories` instead of the
    /// manager-derived categories from `manager_categories()`.
    /// Mirrors Renovate's `categories` field in `PackageRuleInputConfig`.
    pub categories: &'a [&'a str],
}

impl<'a> DepContext<'a> {
    /// Convenience constructor: just a dep name, all other fields default.
    pub fn for_dep(dep_name: &'a str) -> Self {
        Self {
            dep_name,
            ..Default::default()
        }
    }

    /// Set the manager and return `self` for builder-style chaining.
    pub fn with_manager(mut self, manager: &'a str) -> Self {
        self.manager = Some(manager);
        self
    }

    /// Set the datasource and return `self`.
    pub fn with_datasource(mut self, datasource: &'a str) -> Self {
        self.datasource = Some(datasource);
        self
    }

    /// Set the dep type and return `self`.
    pub fn with_dep_type(mut self, dep_type: &'a str) -> Self {
        self.dep_type = Some(dep_type);
        self
    }

    /// Set the file path and return `self`.
    pub fn with_file_path(mut self, file_path: &'a str) -> Self {
        self.file_path = Some(file_path);
        self
    }
}

// ── PathMatcher ───────────────────────────────────────────────────────────────

/// Compiled path-matcher for `ignorePaths` and `matchFileNames`.
///
/// Patterns containing `*`, `?`, or `[` are compiled as globset globs;
/// all others are treated as path prefixes (trailing `/` is stripped).
#[derive(Debug)]
pub struct PathMatcher {
    prefixes: Vec<String>,
    globs: GlobSet,
}

impl PathMatcher {
    /// Compile `patterns` into a `PathMatcher`.
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

// ── RuleEffects ───────────────────────────────────────────────────────────────

/// The merged effects collected from all matching `packageRules` for a dep.
///
/// Built by `RepoConfig::collect_rule_effects`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RuleEffects {
    /// `groupName` from the first matching rule that sets it (or repo default).
    pub group_name: Option<String>,
    /// Explicit `groupSlug` from the first matching rule that sets it.
    /// When set, this is used as the branch topic instead of auto-slugifying `groupName`.
    pub group_slug: Option<String>,
    /// `automerge` from the last matching rule that sets it (or repo default).
    pub automerge: Option<bool>,
    /// `schedule` from the last matching rule that sets it.  Empty = repo default.
    pub schedule: Vec<String>,
    /// Labels accumulated (union) from all matching rules (includes both `labels`
    /// and `addLabels` from each matching rule).
    pub labels: Vec<String>,
    /// Per-rule `minimumReleaseAge` override.  `None` = use repo-level default.
    /// The last matching rule that sets this wins.
    pub minimum_release_age: Option<String>,
    /// PR assignees from the last matching rule that sets them.
    /// Empty = use repo-level assignees.
    pub assignees: Vec<String>,
    /// PR reviewers from the last matching rule that sets them.
    /// Empty = use repo-level reviewers.
    pub reviewers: Vec<String>,
    /// PR creation priority.  Higher numbers → created first.
    /// `None` = not configured (treat as 0).  The last matching rule wins.
    pub pr_priority: Option<i32>,
    /// Per-rule commit message topic override.  The last matching rule wins.
    pub commit_message_topic: Option<String>,
    /// Per-rule action verb override (e.g. `"Pin"`).  Last rule wins.
    pub commit_message_action: Option<String>,
    /// Per-rule commit message prefix (e.g. `"fix(deps):"`).  Last rule wins.
    pub commit_message_prefix: Option<String>,
    /// Per-rule semantic commit type (e.g. `"fix"`, `"chore"`).  Last rule wins.
    pub semantic_commit_type: Option<String>,
    /// Per-rule semantic commit scope (e.g. `"security"`, `"deps"`).  Last rule wins.
    pub semantic_commit_scope: Option<String>,
    /// Per-rule override for the "to {{newVersion}}" extra segment.  Last rule wins.
    pub commit_message_extra: Option<String>,
    /// Per-rule suffix appended to the commit message.  Last rule wins.
    pub commit_message_suffix: Option<String>,

    /// Per-rule rangeStrategy override.  Values: `"pin"`, `"replace"`, `"widen"`,
    /// `"bump"`, `"auto"`, `"in-range-only"`.  `None` = use repo-level default.
    pub range_strategy: Option<String>,
    /// Per-rule versioning scheme override (e.g. `"docker"`, `"semver"`, `"regex:..."`).
    /// `None` = use the manager's default versioning.
    pub versioning: Option<String>,
    /// Per-rule digest pinning override.
    /// `None` = use manager default behavior.
    pub pin_digests: Option<bool>,
    /// Per-rule dist-tag to follow (e.g. `"next"`, `"beta"`).
    /// `None` = use the package's default release channel.
    pub follow_tag: Option<String>,
    /// Replacement package name for migration suggestions.
    pub replacement_name: Option<String>,
    /// Replacement version constraint for migration suggestions.
    pub replacement_version: Option<String>,
    /// Version-compatibility regex pattern (forwarded to updater; not evaluated here).
    pub version_compatibility: Option<String>,
    /// Custom changelog URL template (forwarded to PR generator; not evaluated here).
    pub changelog_url: Option<String>,
    /// Rendered source URL override from a matching package rule.
    pub source_url: Option<String>,
    /// Changelog fetching mode from a matching package rule.
    pub fetch_change_logs: Option<String>,
    /// Whether Dependency Dashboard approval is required before creating a PR.
    pub dependency_dashboard_approval: Option<bool>,
    /// Effective `overrideDatasource` from matching package rules.
    pub override_datasource: Option<String>,
    /// Rendered effective `overrideDepName` from matching package rules.
    pub override_dep_name: Option<String>,
    /// Rendered effective `overridePackageName` from matching package rules.
    pub override_package_name: Option<String>,
    /// Whether this dep is force-disabled (skipReason should be set).
    /// Computed from `force.enabled` in packageRules.
    /// `true` = force-enabled (overrides any `enabled: false`).
    /// `false` = force-disabled (skipReason regardless of `enabled: true`).
    pub force_enabled: Option<bool>,
}

fn deserialize_string_or_vec<'de, D>(d: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let val = serde_json::Value::deserialize(d)?;
    Ok(match val {
        serde_json::Value::String(s) => vec![s],
        serde_json::Value::Array(arr) => arr
            .into_iter()
            .filter_map(|v| v.as_str().map(str::to_owned))
            .collect(),
        _ => vec![],
    })
}

// ── UpdateTypeConfig ──────────────────────────────────────────────────────────

/// Per-update-type configuration block.
///
/// Maps the top-level `major`, `minor`, and `patch` config objects from
/// `renovate.json`.  These blocks apply their settings to all updates of the
/// matching type, *after* all `packageRules` have been evaluated.
///
/// Renovate reference: `lib/config/options/index.ts` — `major`, `minor`, `patch`.
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct UpdateTypeConfig {
    pub automerge: Option<bool>,
    pub enabled: Option<bool>,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(rename = "addLabels", default)]
    pub add_labels: Vec<String>,
    #[serde(default)]
    pub assignees: Vec<String>,
    #[serde(default)]
    pub reviewers: Vec<String>,
    #[serde(rename = "groupName")]
    pub group_name: Option<String>,
    #[serde(rename = "groupSlug")]
    pub group_slug: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub schedule: Vec<String>,
    #[serde(rename = "prPriority")]
    pub pr_priority: Option<i32>,
    #[serde(rename = "minimumReleaseAge")]
    pub minimum_release_age: Option<String>,
    #[serde(rename = "commitMessageTopic")]
    pub commit_message_topic: Option<String>,
    #[serde(rename = "commitMessageAction")]
    pub commit_message_action: Option<String>,
    #[serde(rename = "commitMessagePrefix")]
    pub commit_message_prefix: Option<String>,
    #[serde(rename = "semanticCommitType")]
    pub semantic_commit_type: Option<String>,
    #[serde(rename = "semanticCommitScope")]
    pub semantic_commit_scope: Option<String>,
    #[serde(rename = "commitMessageExtra")]
    pub commit_message_extra: Option<String>,
    #[serde(rename = "commitMessageSuffix")]
    pub commit_message_suffix: Option<String>,
}

impl UpdateTypeConfig {
    /// Merge this config into `effects` (last-writer-wins for scalar fields;
    /// `addLabels` appends, `labels` replaces when non-empty).
    pub fn apply_to_effects(&self, effects: &mut RuleEffects) {
        if let Some(am) = self.automerge {
            effects.automerge = Some(am);
        }
        if let Some(gn) = &self.group_name {
            effects.group_name = Some(gn.clone());
        }
        if let Some(gs) = &self.group_slug {
            effects.group_slug = Some(gs.clone());
        } else if let Some(gn) = &self.group_name {
            // Auto-generate groupSlug from groupName when the rule sets groupName but not
            // groupSlug and a prior rule already set a groupSlug.  This matches Renovate's
            // behaviour in applyPackageRules: the new groupName must override the stale slug.
            if effects.group_slug.is_some() {
                effects.group_slug = Some(crate::branch::group_branch_topic(gn));
            }
        }
        if !self.schedule.is_empty() {
            effects.schedule.clone_from(&self.schedule);
        }
        // `labels` replaces; `addLabels` appends.
        if !self.labels.is_empty() {
            effects.labels.clone_from(&self.labels);
        }
        for label in &self.add_labels {
            if !effects.labels.contains(label) {
                effects.labels.push(label.clone());
            }
        }
        if !self.assignees.is_empty() {
            effects.assignees.clone_from(&self.assignees);
        }
        if !self.reviewers.is_empty() {
            effects.reviewers.clone_from(&self.reviewers);
        }
        if let Some(p) = self.pr_priority {
            effects.pr_priority = Some(p);
        }
        if self.minimum_release_age.is_some() {
            effects
                .minimum_release_age
                .clone_from(&self.minimum_release_age);
        }
        if self.commit_message_topic.is_some() {
            effects
                .commit_message_topic
                .clone_from(&self.commit_message_topic);
        }
        if self.commit_message_action.is_some() {
            effects
                .commit_message_action
                .clone_from(&self.commit_message_action);
        }
        if self.commit_message_prefix.is_some() {
            effects
                .commit_message_prefix
                .clone_from(&self.commit_message_prefix);
        }
        if self.semantic_commit_type.is_some() {
            effects
                .semantic_commit_type
                .clone_from(&self.semantic_commit_type);
        }
        if self.semantic_commit_scope.is_some() {
            effects
                .semantic_commit_scope
                .clone_from(&self.semantic_commit_scope);
        }
        if self.commit_message_extra.is_some() {
            effects
                .commit_message_extra
                .clone_from(&self.commit_message_extra);
        }
        if self.commit_message_suffix.is_some() {
            effects
                .commit_message_suffix
                .clone_from(&self.commit_message_suffix);
        }
    }
}

// ── Free helpers (used by both PackageRule and RepoConfig) ────────────────────

/// Return `true` if `proposed_version` is within the `allowedVersions` constraint.
///
/// Supports:
/// - `/regex/[flags]` — version must match the regex
/// - Semver range (`<`, `>`, `~`, `^`, `=`, `*` prefix) — version must satisfy range
/// - Exact string — version must equal the constraint exactly
///
/// Returns `true` (allowed) if the constraint is satisfied, `false` if blocked.
/// Called from `RepoConfig::is_version_restricted_for_file`; a `false` return
/// means the version is NOT in the allowed set → it should be blocked.
pub(crate) fn version_matches_allowed(proposed_version: &str, allowed: &str) -> bool {
    use crate::versioning::semver_generic::parse_padded;
    let av = allowed.trim();
    if av.starts_with('/') {
        // `/pattern/[flags]` — extract pattern between the slashes
        let inner = av.trim_start_matches('/');
        let pat = inner
            .trim_end_matches(|c: char| c.is_alphabetic())
            .trim_end_matches('/');
        return Regex::new(pat)
            .map(|re| re.is_match(proposed_version))
            .unwrap_or(true); // malformed regex → don't restrict
    }
    if av == proposed_version {
        return true;
    }
    let first = av.chars().next().unwrap_or(' ');
    if matches!(first, '<' | '>' | '~' | '^' | '=' | '*') {
        if let Ok(req) = semver::VersionReq::parse(av)
            && let Some(sv) = parse_padded(proposed_version)
        {
            return req.matches(&sv);
        }
        // semver range but proposed_version is not parseable → don't restrict
        return true;
    }
    false
}

/// Return `true` if `proposed_version` is matched by any entry in `ignore_list`.
///
/// Entries may be:
/// - `/regex/` — version string is matched against the regex
/// - A semver range (starts with `<`, `>`, `~`, `^`, `=`, `*`) — must satisfy range
/// - An exact version string — string equality
pub(crate) fn version_matches_ignore_list(proposed_version: &str, ignore_list: &[String]) -> bool {
    use crate::versioning::semver_generic::parse_padded;
    for entry in ignore_list {
        let e = entry.trim();
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
        if e == proposed_version {
            return true;
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn rule_with_managers(patterns: &[&str]) -> PackageRule {
        PackageRule {
            match_managers: patterns
                .iter()
                .map(|pattern| (*pattern).to_owned())
                .collect(),
            ..Default::default()
        }
    }

    fn rule_with_dep_names(patterns: &[&str]) -> PackageRule {
        PackageRule {
            match_dep_names: patterns
                .iter()
                .map(|pattern| (*pattern).to_owned())
                .collect(),
            ..Default::default()
        }
    }

    fn rule_with_package_names(patterns: &[&str]) -> PackageRule {
        PackageRule {
            match_package_names: patterns
                .iter()
                .map(|pattern| (*pattern).to_owned())
                .collect(),
            has_name_constraint: true,
            ..Default::default()
        }
    }

    fn rule_with_current_value(pattern: &str) -> PackageRule {
        PackageRule {
            match_current_value: Some(pattern.to_owned()),
            ..Default::default()
        }
    }

    fn rule_with_new_value(pattern: &str) -> PackageRule {
        PackageRule {
            match_new_value: Some(pattern.to_owned()),
            ..Default::default()
        }
    }

    fn rule_with_file_names(patterns: &[&str]) -> PackageRule {
        PackageRule {
            match_file_names: patterns
                .iter()
                .map(|pattern| (*pattern).to_owned())
                .collect(),
            ..Default::default()
        }
    }

    fn rule_with_repositories(patterns: &[&str]) -> PackageRule {
        PackageRule {
            match_repositories: patterns
                .iter()
                .map(|pattern| (*pattern).to_owned())
                .collect(),
            ..Default::default()
        }
    }

    fn rule_with_current_age(range: &str) -> PackageRule {
        PackageRule {
            match_current_age: Some(range.to_owned()),
            ..Default::default()
        }
    }

    fn rule_with_current_version(pattern: &str) -> PackageRule {
        PackageRule {
            match_current_version: Some(pattern.to_owned()),
            ..Default::default()
        }
    }

    // Ported: "should return true" — util/package-rules/managers.spec.ts line 7
    #[test]
    fn managers_matcher_returns_true_for_matching_manager() {
        let rule = rule_with_managers(&["npm", "regex"]);

        assert!(rule.manager_matches("npm"));
    }

    // Ported: "should return false for no match" — util/package-rules/managers.spec.ts line 19
    #[test]
    fn managers_matcher_returns_false_for_no_match() {
        let rule = rule_with_managers(&["docker"]);

        assert!(!rule.manager_matches("npm"));
    }

    // Ported: "should return null if matchManagers is undefined" — util/package-rules/managers.spec.ts line 31
    #[test]
    fn managers_matcher_without_patterns_is_not_a_constraint() {
        let rule = PackageRule::default();

        assert!(rule.manager_matches("npm"));
    }

    // Ported: "should return false if no manager" — util/package-rules/managers.spec.ts line 41
    #[test]
    fn managers_matcher_returns_false_if_no_manager() {
        let rule = rule_with_managers(&["npm"]);
        let ctx = DepContext::for_dep("lodash");

        assert!(!rule.matches_context(&ctx));
    }

    // Ported: "should match custom managers" — util/package-rules/managers.spec.ts line 51
    #[test]
    fn managers_matcher_matches_custom_managers() {
        let rule = rule_with_managers(&["custom.regex"]);

        assert!(rule.manager_matches("regex"));
    }

    // Ported: "should return false if packageFile is not defined" — util/package-rules/dep-names.spec.ts line 7
    #[test]
    fn dep_name_matcher_returns_false_if_dep_name_is_empty() {
        let rule = rule_with_dep_names(&["@opentelemetry/http"]);

        assert!(!rule.dep_name_matches(""));
    }

    // Ported: "should return false if depName is excluded prefix" — util/package-rules/dep-names.spec.ts line 19
    #[test]
    fn dep_name_matcher_returns_false_if_dep_name_is_excluded_prefix() {
        let regex_rule = rule_with_dep_names(&["!/^@opentelemetry/"]);
        let glob_rule = rule_with_dep_names(&["!@opentelemetry{/,}**"]);

        assert!(!regex_rule.dep_name_matches("@opentelemetry/http"));
        assert!(!glob_rule.dep_name_matches("@opentelemetry/http"));
    }

    // Ported: "should return true if depName is included prefix" — util/package-rules/dep-names.spec.ts line 42
    #[test]
    fn dep_name_matcher_returns_true_if_dep_name_is_included_prefix() {
        let regex_rule = rule_with_dep_names(&["/^@opentelemetry/"]);
        let glob_rule = rule_with_dep_names(&["@opentelemetry{/,}**"]);

        assert!(regex_rule.dep_name_matches("@opentelemetry/http"));
        assert!(glob_rule.dep_name_matches("@opentelemetry/http"));
    }

    // Ported: "should return false if for wrong prefix" — util/package-rules/dep-names.spec.ts line 65
    #[test]
    fn dep_name_matcher_returns_false_for_wrong_prefix() {
        let rule = rule_with_dep_names(&["@opentelemetry**"]);

        assert!(!rule.dep_name_matches("@opentelemetry/http"));
    }

    // Ported: "return true for exact match" — util/package-rules/current-value.spec.ts line 7
    #[test]
    fn current_value_matcher_returns_true_for_exact_match() {
        let rule = rule_with_current_value("1.1.0");

        assert!(rule.current_value_matches("1.1.0"));
    }

    // Ported: "return true for glob match" — util/package-rules/current-value.spec.ts line 19
    #[test]
    fn current_value_matcher_returns_true_for_glob_match() {
        let rule = rule_with_current_value("1.2.*");

        assert!(rule.current_value_matches("1.2.3"));
    }

    // Ported: "return false for glob non match" — util/package-rules/current-value.spec.ts line 31
    #[test]
    fn current_value_matcher_returns_false_for_glob_non_match() {
        let rule = rule_with_current_value("1.3.*");

        assert!(!rule.current_value_matches("1.2.3"));
    }

    // Ported: "return false for regex version non match" — util/package-rules/current-value.spec.ts line 43
    #[test]
    fn current_value_matcher_returns_false_for_regex_version_non_match() {
        let rule = rule_with_current_value("/^v/");

        assert!(!rule.current_value_matches("\"~> 1.1.0\""));
    }

    // Ported: "case insensitive match" — util/package-rules/current-value.spec.ts line 55
    #[test]
    fn current_value_matcher_is_case_insensitive_for_i_regex_flag() {
        let rule = rule_with_current_value("/^\"v/i");

        assert!(rule.current_value_matches("\"V1.1.0\""));
    }

    // Ported: "return true for regex version match" — util/package-rules/current-value.spec.ts line 67
    #[test]
    fn current_value_matcher_returns_true_for_regex_version_match() {
        let rule = rule_with_current_value("/^\"/");

        assert!(rule.current_value_matches("\"~> 0.1.0\""));
    }

    // Ported: "return false for now value" — util/package-rules/current-value.spec.ts line 79
    #[test]
    fn current_value_matcher_returns_false_for_missing_value() {
        let rule = rule_with_current_value("/^v?[~ -]?0/");
        let ctx = DepContext::for_dep("dep");

        assert!(!rule.matches_context(&ctx));
    }

    // Ported: "return true for exact match" — util/package-rules/new-value.spec.ts line 7
    #[test]
    fn new_value_matcher_returns_true_for_exact_match() {
        let rule = rule_with_new_value("1.1.0");

        assert!(rule.new_value_matches("1.1.0"));
    }

    // Ported: "return true for glob match" — util/package-rules/new-value.spec.ts line 19
    #[test]
    fn new_value_matcher_returns_true_for_glob_match() {
        let rule = rule_with_new_value("1.2.*");

        assert!(rule.new_value_matches("1.2.3"));
    }

    // Ported: "return false for glob non match" — util/package-rules/new-value.spec.ts line 31
    #[test]
    fn new_value_matcher_returns_false_for_glob_non_match() {
        let rule = rule_with_new_value("1.3.*");

        assert!(!rule.new_value_matches("1.2.3"));
    }

    // Ported: "return false for regex version non match" — util/package-rules/new-value.spec.ts line 43
    #[test]
    fn new_value_matcher_returns_false_for_regex_version_non_match() {
        let rule = rule_with_new_value("/^v/");

        assert!(!rule.new_value_matches("\"~> 1.1.0\""));
    }

    // Ported: "case insensitive match" — util/package-rules/new-value.spec.ts line 55
    #[test]
    fn new_value_matcher_is_case_insensitive_for_i_regex_flag() {
        let rule = rule_with_new_value("/^\"v/i");

        assert!(rule.new_value_matches("\"V1.1.0\""));
    }

    // Ported: "return true for regex version match" — util/package-rules/new-value.spec.ts line 67
    #[test]
    fn new_value_matcher_returns_true_for_regex_version_match() {
        let rule = rule_with_new_value("/^\"/");

        assert!(rule.new_value_matches("\"~> 0.1.0\""));
    }

    // Ported: "return false for now value" — util/package-rules/new-value.spec.ts line 79
    #[test]
    fn new_value_matcher_returns_false_for_missing_value() {
        let rule = rule_with_new_value("/^v?[~ -]?0/");
        let ctx = DepContext::for_dep("dep");

        assert!(!rule.matches_context(&ctx));
    }

    // Ported: "should return false if packageName is not defined" — util/package-rules/package-names.spec.ts line 7
    #[test]
    fn package_name_matcher_returns_false_if_package_name_is_empty() {
        let rule = rule_with_package_names(&["@opentelemetry/http"]);

        assert!(!rule.name_matches(""));
    }

    // Ported: "should return false if not matching" — util/package-rules/package-names.spec.ts line 19
    #[test]
    fn package_name_matcher_returns_false_if_not_matching() {
        let rule = rule_with_package_names(&["ghi"]);

        assert!(!rule.name_matches("def"));
    }

    // Ported: "should matchPackageName" — util/package-rules/package-names.spec.ts line 32
    #[test]
    fn package_name_matcher_matches_package_name() {
        let rule = rule_with_package_names(&["def", "ghi"]);

        assert!(rule.name_matches("def"));
    }

    // Ported: "should match pattern" — util/package-rules/package-names.spec.ts line 44
    #[test]
    fn package_name_matcher_matches_regex_pattern() {
        let rule = rule_with_package_names(&["/b/"]);

        assert!(rule.name_matches("b"));
    }

    // Ported: "should return false if packageFile is not defined" — util/package-rules/files.spec.ts line 7
    #[test]
    fn file_names_matcher_returns_false_if_package_file_is_missing() {
        let rule = rule_with_file_names(&["frontend/package.json"]);
        let ctx = DepContext::for_dep("dep");

        assert!(!rule.matches_context(&ctx));
    }

    // Ported: "returns false if release is older" — util/package-rules/current-age.spec.ts line 18
    #[test]
    fn current_age_matcher_returns_false_if_release_is_older() {
        let rule = rule_with_current_age("< 1 year");

        assert!(!rule.current_age_matches(Some("2020-01-01")));
    }

    // Ported: "returns false if release is younger" — util/package-rules/current-age.spec.ts line 30
    #[test]
    fn current_age_matcher_returns_false_if_release_is_younger() {
        let rule = rule_with_current_age("> 10 years");

        assert!(!rule.current_age_matches(Some("2020-01-01")));
    }

    // Ported: "returns null if release invalid" — util/package-rules/current-age.spec.ts line 42
    #[test]
    fn current_age_matcher_returns_false_if_release_invalid() {
        let rule = rule_with_current_age("> 2 days");

        assert!(!rule.current_age_matches(Some("abc")));
    }

    // Ported: "returns false if release undefined" — util/package-rules/current-age.spec.ts line 54
    #[test]
    fn current_age_matcher_returns_false_if_release_undefined() {
        let rule = rule_with_current_age("> 2 days");

        assert!(!rule.current_age_matches(None));
    }

    // Ported: "returns true if age matches" — util/package-rules/current-age.spec.ts line 66
    #[test]
    fn current_age_matcher_returns_true_if_age_matches() {
        let rule = rule_with_current_age("> 3 years");

        assert!(rule.current_age_matches(Some("2020-01-01")));
    }

    // Ported: "should return null if match repositories is not defined" — util/package-rules/repositories.spec.ts line 7
    #[test]
    fn repositories_matcher_without_patterns_is_not_a_constraint() {
        let rule = PackageRule::default();

        assert!(rule.repository_matches("org/repo"));
    }

    // Ported: "should return false if repository is not defined" — util/package-rules/repositories.spec.ts line 19
    #[test]
    fn repositories_matcher_returns_false_if_repository_is_missing() {
        let rule = rule_with_repositories(&["org/repo"]);
        let ctx = DepContext::for_dep("dep");

        assert!(!rule.matches_context(&ctx));
    }

    // Ported: "should return true if repository matches regex pattern" — util/package-rules/repositories.spec.ts line 31
    #[test]
    fn repositories_matcher_returns_true_for_regex_pattern() {
        let rule = rule_with_repositories(&["/^org/repo$/"]);

        assert!(rule.repository_matches("org/repo"));
    }

    // Ported: "should return false if repository has invalid regex pattern" — util/package-rules/repositories.spec.ts line 43
    #[test]
    fn repositories_matcher_returns_false_for_invalid_regex_pattern() {
        let rule = rule_with_repositories(&["/[/"]);

        assert!(!rule.repository_matches("org/repo"));
    }

    // Ported: "should return false if repository does not match regex pattern" — util/package-rules/repositories.spec.ts line 55
    #[test]
    fn repositories_matcher_returns_false_for_non_matching_regex_pattern() {
        let rule = rule_with_repositories(&["/^org/other-repo$/"]);

        assert!(!rule.repository_matches("org/repo"));
    }

    // Ported: "should return true if repository matches minimatch pattern" — util/package-rules/repositories.spec.ts line 67
    #[test]
    fn repositories_matcher_returns_true_for_minimatch_pattern() {
        let rule = rule_with_repositories(&["org/**"]);

        assert!(rule.repository_matches("org/repo"));
    }

    // Ported: "should return false if repository does not match minimatch pattern" — util/package-rules/repositories.spec.ts line 79
    #[test]
    fn repositories_matcher_returns_false_for_non_matching_minimatch_pattern() {
        let rule = rule_with_repositories(&["other-org/**"]);

        assert!(!rule.repository_matches("org/repo"));
    }

    // Ported: "should return true if repository matches at least one pattern" — util/package-rules/repositories.spec.ts line 91
    #[test]
    fn repositories_matcher_returns_true_if_any_pattern_matches() {
        let rule = rule_with_repositories(&["/^org/repo$/", "**/*-archived"]);

        assert!(rule.repository_matches("org/repo-archived"));
    }

    // Ported: "should return false if exclude repository is not defined" — util/package-rules/repositories.spec.ts line 105
    #[test]
    fn repositories_matcher_returns_false_if_exclude_repository_is_missing() {
        let rule = rule_with_repositories(&["!org/repo"]);
        let ctx = DepContext::for_dep("dep");

        assert!(!rule.matches_context(&ctx));
    }

    // Ported: "should return false if exclude repository matches regex pattern" — util/package-rules/repositories.spec.ts line 117
    #[test]
    fn repositories_matcher_returns_false_if_exclude_regex_matches() {
        let rule = rule_with_repositories(&["!/^org/repo$/"]);

        assert!(!rule.repository_matches("org/repo"));
    }

    // Ported: "should return true if exclude repository has invalid regex pattern" — util/package-rules/repositories.spec.ts line 129
    #[test]
    fn repositories_matcher_returns_true_if_exclude_regex_is_invalid() {
        let rule = rule_with_repositories(&["!/[/"]);

        assert!(rule.repository_matches("org/repo"));
    }

    // Ported: "should return true if exclude repository does not match regex pattern" — util/package-rules/repositories.spec.ts line 141
    #[test]
    fn repositories_matcher_returns_true_if_exclude_regex_does_not_match() {
        let rule = rule_with_repositories(&["!/^org/other-repo$/"]);

        assert!(rule.repository_matches("org/repo"));
    }

    // Ported: "should return false if exclude repository matches minimatch pattern" — util/package-rules/repositories.spec.ts line 153
    #[test]
    fn repositories_matcher_returns_false_if_exclude_minimatch_matches() {
        let rule = rule_with_repositories(&["!org/**"]);

        assert!(!rule.repository_matches("org/repo"));
    }

    // Ported: "should return true if exclude repository does not match minimatch pattern" — util/package-rules/repositories.spec.ts line 165
    #[test]
    fn repositories_matcher_returns_true_if_exclude_minimatch_does_not_match() {
        let rule = rule_with_repositories(&["!other-org/**"]);

        assert!(rule.repository_matches("org/repo"));
    }

    // Ported: "should return false if exclude repository matches at least one pattern" — util/package-rules/repositories.spec.ts line 177
    #[test]
    fn repositories_matcher_returns_false_if_any_exclude_pattern_matches() {
        let rule = rule_with_repositories(&["!/^org/repo$/", "!**/*-archived"]);

        assert!(!rule.repository_matches("org/repo-archived"));
    }

    // Ported: "returns true for null versioning" — util/package-rules/current-version.spec.ts line 8
    #[test]
    fn current_version_matcher_returns_true_for_null_versioning_equivalent() {
        let rule = rule_with_current_version("1.2.3");

        assert!(rule.current_version_matches("1.2.3", None, None));
    }

    // Ported: "return false if no version could be found" — util/package-rules/current-version.spec.ts line 52
    #[test]
    fn current_version_matcher_returns_false_if_no_version_found() {
        let rule = rule_with_current_version("bbbbbb");

        assert!(!rule.current_version_matches("aaaaaa", None, Some("bbbbbb")));
    }

    // Ported: "case insensitive match" — util/package-rules/current-version.spec.ts line 66
    #[test]
    fn current_version_matcher_regex_is_case_insensitive() {
        let rule = rule_with_current_version("/BBB.*/i");

        assert!(rule.current_version_matches("bbbbbb", None, None));
    }

    // Ported: "return false for regex version non match" — util/package-rules/current-version.spec.ts line 79
    #[test]
    fn current_version_matcher_returns_false_for_regex_version_non_match() {
        let rule = rule_with_current_version("/^v?[~ -]?0/");

        assert!(!rule.current_version_matches("\"~> 1.1.0\"", None, Some("1.1.4")));
    }

    // Ported: "return true for regex version match" — util/package-rules/current-version.spec.ts line 93
    #[test]
    fn current_version_matcher_returns_true_for_regex_version_match() {
        let rule = rule_with_current_version("/^v?[~ -]?0/");

        assert!(rule.current_version_matches("\"~> 0.1.0\"", None, Some("0.1.0")));
    }

    // Ported: "return false for regex value match" — util/package-rules/current-version.spec.ts line 107
    #[test]
    fn current_version_matcher_returns_false_for_regex_value_match_without_version() {
        let rule = rule_with_current_version("/^v?[~ -]?0/");

        assert!(!rule.current_version_matches("\"~> 0.1.0\"", None, None));
    }

    // Ported: "return true for same-major verisioning if version lies in expected range" — util/package-rules/current-version.spec.ts line 120
    #[test]
    fn current_version_matcher_same_major_in_range() {
        // matchCurrentVersion='6.0.400', currentValue='6.0.300'
        // 6.0.400 satisfies ^6.0.300 (>=6.0.300 <7) → true
        let rule = rule_with_current_version("6.0.400");
        assert!(rule.current_version_matches("6.0.300", None, None));
    }

    // Ported: "return false for same-major verisioning if version lies outside of expected range" — util/package-rules/current-version.spec.ts line 133
    #[test]
    fn current_version_matcher_same_major_out_of_range() {
        // matchCurrentVersion='6.0.100', currentValue='6.0.300'
        // 6.0.100 does NOT satisfy ^6.0.300 (6.0.100 < 6.0.300) → false
        let rule = rule_with_current_version("6.0.100");
        assert!(!rule.current_version_matches("6.0.300", None, None));
    }
}
