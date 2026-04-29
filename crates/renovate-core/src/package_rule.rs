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

    /// Return `true` when this rule's update type condition matches `update_type`.
    pub fn update_type_matches(&self, update_type: UpdateType) -> bool {
        if self.has_update_type_constraint && self.match_update_types.is_empty() {
            // All specified update types were unrecognized (e.g. ["pin", "digest"]).
            // An unrecognized-only constraint never matches our known update types.
            return false;
        }
        self.match_update_types.is_empty() || self.match_update_types.contains(&update_type)
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

    /// Return `true` when `current_value` satisfies this rule's `matchCurrentVersion`.
    pub fn current_version_matches(&self, current_value: &str) -> bool {
        use crate::string_match::match_regex_or_glob;
        use crate::versioning::semver_generic::{lower_bound, parse_padded};
        let Some(ref mcv) = self.match_current_version else {
            return true;
        };
        // Regex patterns and negated regex (!/pat/) are matched via match_regex_or_glob
        // which now handles the ! prefix. Route anything that starts with / or !/ here.
        if mcv.starts_with('/') || mcv.starts_with("!/") {
            return match_regex_or_glob(current_value, mcv);
        }
        // Semver range: extract the lower-bound version and compare.
        let lb = lower_bound(current_value);
        let Some(current_sv) = parse_padded(lb) else {
            return true; // can't parse → don't restrict
        };
        match semver::VersionReq::parse(mcv) {
            Ok(req) => req.matches(&current_sv),
            Err(_) => true,
        }
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
                let cats = manager_categories(mgr);
                if !self.categories_match(cats) {
                    return false;
                }
            }
            None => {
                if !self.match_managers.is_empty() || !self.match_categories.is_empty() {
                    return false;
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

        let dep_type = ctx.dep_type.unwrap_or("");
        if !self.dep_type_matches(dep_type) {
            return false;
        }

        let file = ctx.file_path.unwrap_or("");
        if !self.file_name_matches(file) {
            return false;
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
        if !self.current_version_matches(current_val) {
            return false;
        }

        let new_val = ctx.new_value.unwrap_or("");
        if !self.new_value_matches(new_val) {
            return false;
        }

        if let Some(ut) = ctx.update_type
            && !self.update_type_matches(ut)
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
        self.match_base_branches.iter().any(|pattern| {
            if pattern.contains('*') || pattern.contains('?') || pattern.contains('[') {
                globset::Glob::new(pattern)
                    .ok()
                    .map(|g| g.compile_matcher().is_match(branch))
                    .unwrap_or(false)
            } else {
                pattern == branch
            }
        })
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
    pub dep_type: Option<&'a str>,
    /// Relative manifest file path.
    pub file_path: Option<&'a str>,
    /// Source repository URL reported by the datasource.
    pub source_url: Option<&'a str>,
    /// Registry URLs used by the dep (from manifest or datasource).
    pub registry_urls: Option<&'a [&'a str]>,
    /// Current repository name (`"owner/repo"`).
    pub repository: Option<&'a str>,
    /// Current base branch (e.g. `"main"`, `"develop"`).
    pub base_branch: Option<&'a str>,
    /// Raw current version string from the manifest.
    pub current_value: Option<&'a str>,
    /// Proposed new version string (after datasource lookup).
    pub new_value: Option<&'a str>,
    /// Classified update type (available after version lookup).
    pub update_type: Option<UpdateType>,
    /// ISO 8601 release timestamp for the **currently installed** version.
    pub current_version_timestamp: Option<&'a str>,
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
    #[serde(default)]
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
