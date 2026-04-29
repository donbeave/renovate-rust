//! Renovate branch name generation.
//!
//! Computes the expected git branch name for a proposed dependency update,
//! mirroring Renovate's default naming logic.
//!
//! Renovate reference:
//! - `lib/workers/repository/updates/flatten.ts` — `sanitizeDepName()`
//! - `lib/workers/repository/updates/branch-name.ts` — `generateBranchName()`,
//!   `cleanBranchName()`
//! - `lib/config/options/index.ts` — `branchName`, `branchTopic` defaults

use std::sync::LazyLock;

use regex::Regex;
use sha2::{Digest as _, Sha512};

static MULTI_DASH: LazyLock<Regex> = LazyLock::new(|| Regex::new("-{2,}").unwrap());

/// Sanitize a dependency name for use in a git branch name.
///
/// Mirrors Renovate's `sanitizeDepName` from
/// `lib/workers/repository/updates/flatten.ts`:
/// - Strips `@types/` prefix (TypeScript type packages)
/// - Removes `@` (npm scope character)
/// - Replaces `/` with `-`
/// - Replaces whitespace and `:` with `-`
/// - Collapses consecutive `-` into one
/// - Lowercases the result
///
/// # Examples
///
/// ```
/// # use renovate_core::branch::sanitize_dep_name;
/// assert_eq!(sanitize_dep_name("@angular/core"), "angular-core");
/// assert_eq!(sanitize_dep_name("@types/lodash"), "lodash");
/// assert_eq!(sanitize_dep_name("react"), "react");
/// assert_eq!(sanitize_dep_name("github.com/user/repo"), "github.com-user-repo");
/// ```
pub fn sanitize_dep_name(name: &str) -> String {
    let s = name
        .replace("@types/", "")
        .replace('@', "")
        .replace('/', "-")
        .replace(|c: char| c.is_whitespace(), "-")
        .replace(':', "-")
        .to_lowercase();
    MULTI_DASH.replace_all(&s, "-").into_owned()
}

/// Compute the default `branchTopic` for a single-dep (non-grouped) update.
///
/// Mirrors the default `branchTopic` template from Renovate's options:
/// ```text
/// {depNameSanitized}-{newMajor}[.{newMinor}].x
/// ```
///
/// - Default: `{sanitized}-{major}.x` — all minor/patch updates for the same
///   major share a single branch.
/// - When `separate_minor_patch = true` and the update is a patch: includes
///   the minor component → `{sanitized}-{major}.{minor}.x`.
///
/// When `separate_multiple_minor` is `true` and this is a minor update, the
/// minor component is included so that different minor versions get separate
/// branches → `{sanitized}-{major}.{minor}.x`.
///
/// # Parameters
///
/// - `dep_name` — raw dep name (will be sanitized)
/// - `new_major` — major component of the proposed new version
/// - `new_minor` — minor component (used when `separate_minor_patch` + `is_patch`,
///   or when `separate_multiple_minor` + `is_minor`)
/// - `is_patch` — whether this is a patch-level update
/// - `is_minor` — whether this is a minor-level update
/// - `separate_minor_patch` — value of the `separateMinorPatch` config option
/// - `separate_multiple_minor` — value of the `separateMultipleMinor` config option
///
/// # Examples
///
/// ```
/// # use renovate_core::branch::branch_topic;
/// // Default: all 4.x lodash updates share one branch.
/// assert_eq!(branch_topic("lodash", 4, 17, true, false, false, false), "lodash-4.x");
/// // separateMinorPatch=true: patch gets its own branch.
/// assert_eq!(branch_topic("lodash", 4, 17, true, false, true, false), "lodash-4.17.x");
/// // separateMultipleMinor=true: minor update gets its own branch.
/// assert_eq!(branch_topic("lodash", 4, 17, false, true, false, true), "lodash-4.17.x");
/// // Major update.
/// assert_eq!(branch_topic("react", 18, 0, false, false, false, false), "react-18.x");
/// // Scoped npm package.
/// assert_eq!(branch_topic("@angular/core", 17, 0, false, false, false, false), "angular-core-17.x");
/// ```
pub fn branch_topic(
    dep_name: &str,
    new_major: u64,
    new_minor: u64,
    is_patch: bool,
    is_minor: bool,
    separate_minor_patch: bool,
    separate_multiple_minor: bool,
) -> String {
    let sanitized = sanitize_dep_name(dep_name);
    if (separate_minor_patch && is_patch) || (separate_multiple_minor && is_minor) {
        format!("{sanitized}-{new_major}.{new_minor}.x")
    } else {
        format!("{sanitized}-{new_major}.x")
    }
}

/// Compute the branch topic for a grouped update.
///
/// Mirrors Renovate's `slugify(groupName, { lower: true })`:
/// - Lowercases the group name
/// - Replaces spaces, slashes, and other non-alphanumeric characters with `-`
/// - Collapses multiple consecutive `-` into one
/// - Strips leading/trailing `-`
///
/// Renovate reference:
/// `lib/workers/repository/updates/branch-name.ts` — `update.groupSlug`
///
/// # Examples
///
/// ```
/// # use renovate_core::branch::group_branch_topic;
/// assert_eq!(group_branch_topic("All Dependencies"), "all-dependencies");
/// assert_eq!(group_branch_topic("@angular/**"), "angular");
/// assert_eq!(group_branch_topic("Python packages"), "python-packages");
/// ```
pub fn group_branch_topic(group_name: &str) -> String {
    let lower = group_name.to_lowercase();
    let slug: String = lower
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect();
    // Collapse runs of hyphens, strip leading/trailing hyphens.
    let mut result = String::with_capacity(slug.len());
    let mut last_was_hyphen = true; // treat start as if preceded by hyphen to trim leading
    for ch in slug.chars() {
        if ch == '-' {
            if !last_was_hyphen {
                result.push('-');
                last_was_hyphen = true;
            }
        } else {
            result.push(ch);
            last_was_hyphen = false;
        }
    }
    // Trim trailing hyphen.
    result.trim_end_matches('-').to_owned()
}

/// Apply Renovate's `separateMajorMinor` / `separateMultipleMajor` prefix to a
/// computed group slug.
///
/// Mirrors the logic in `branch-name.ts` `generateBranchName`:
/// ```text
/// if (updateType === 'major' && separateMajorMinor) {
///   if (separateMultipleMajor) groupSlug = `major-${newMajor}-${groupSlug}`;
///   else                       groupSlug = `major-${groupSlug}`;
/// }
/// ```
///
/// Note: this is applied **only to computed slugs** (derived from `groupName`).
/// Explicit `groupSlug` overrides from `packageRules` bypass this logic.
///
/// # Examples
///
/// ```
/// # use renovate_core::branch::major_group_slug;
/// // Default: separateMajorMinor=true, separateMultipleMajor=false.
/// assert_eq!(major_group_slug("all-deps", true, false, true, 5), "major-all-deps");
/// // separateMultipleMajor=true → include major version number.
/// assert_eq!(major_group_slug("all-deps", true, true, true, 5), "major-5-all-deps");
/// // separateMajorMinor=false → no prefix.
/// assert_eq!(major_group_slug("all-deps", false, false, true, 5), "all-deps");
/// // Minor update → no prefix regardless of config.
/// assert_eq!(major_group_slug("all-deps", true, false, false, 5), "all-deps");
/// ```
pub fn major_group_slug(
    base: &str,
    separate_major_minor: bool,
    separate_multiple_major: bool,
    is_major: bool,
    new_major: u64,
) -> String {
    if is_major && separate_major_minor {
        if separate_multiple_major {
            format!("major-{new_major}-{base}")
        } else {
            format!("major-{base}")
        }
    } else {
        base.to_owned()
    }
}

/// Compute the full Renovate branch name.
///
/// Mirrors the default `branchName` template:
/// ```text
/// {branchPrefix}{additionalBranchPrefix}{branchTopic}
/// ```
///
/// The result is passed through `clean_branch_name` to strip characters that
/// are invalid in git ref names.
///
/// # Examples
///
/// ```
/// # use renovate_core::branch::branch_name;
/// assert_eq!(branch_name("renovate/", "", "lodash-4.x"), "renovate/lodash-4.x");
/// assert_eq!(branch_name("deps/", "", "react-18.x"), "deps/react-18.x");
/// ```
pub fn branch_name(branch_prefix: &str, additional_prefix: &str, topic: &str) -> String {
    let raw = format!("{branch_prefix}{additional_prefix}{topic}");
    clean_branch_name(&raw)
}

/// Minimum hash length (in hex chars) after subtracting the prefix length.
///
/// Mirrors Renovate's `MIN_HASH_LENGTH = 6`.
const MIN_HASH_LENGTH: u32 = 6;

/// Compute a length-bounded branch name using SHA-512.
///
/// When `hashedBranchLength` is configured, Renovate replaces the branch topic
/// with a hash of `additionalBranchPrefix + branchTopic` so the full branch
/// name is exactly `hashed_branch_length` characters long.
///
/// Mirrors `lib/workers/repository/updates/branch-name.ts`:
/// ```text
/// hash_len = hashedBranchLength - len(branchPrefix)
/// hashInput = additionalBranchPrefix + branchTopic
/// branchName = branchPrefix + sha512(hashInput).slice(0, hash_len)
/// ```
///
/// If `hashed_branch_length <= len(branch_prefix) + MIN_HASH_LENGTH`, the
/// minimum `MIN_HASH_LENGTH` hex chars are used (matching Renovate's warning
/// fallback).
///
/// # Examples
///
/// ```
/// # use renovate_core::branch::hashed_branch_name;
/// // 20-char limit: "renovate/" (9) + 11 hash chars
/// let name = hashed_branch_name("renovate/", "", "lodash-4.x", 20);
/// assert_eq!(name.len(), 20);
/// assert!(name.starts_with("renovate/"));
/// ```
pub fn hashed_branch_name(
    branch_prefix: &str,
    additional_prefix: &str,
    topic: &str,
    hashed_branch_length: u32,
) -> String {
    let prefix_len = branch_prefix.len() as u32;
    let hash_len = if hashed_branch_length > prefix_len + MIN_HASH_LENGTH {
        hashed_branch_length - prefix_len
    } else {
        MIN_HASH_LENGTH
    } as usize;

    let hash_input = format!("{additional_prefix}{topic}");
    let digest = Sha512::digest(hash_input.as_bytes());
    // GenericArray doesn't implement LowerHex — format byte-by-byte.
    let hex: String = digest.iter().map(|b| format!("{b:02x}")).collect();

    format!("{branch_prefix}{}", &hex[..hash_len.min(hex.len())])
}

/// Configuration for PR title / commit message generation.
///
/// Mirrors the `commitMessage*` options from Renovate's config schema.
///
/// Renovate reference:
/// - `lib/config/options/index.ts` — `commitMessageAction`, `commitMessageTopic`,
///   `commitMessageExtra`, `commitMessageSuffix`, `commitMessagePrefix`,
///   `semanticCommits`, `semanticCommitType`, `semanticCommitScope`.
#[derive(Debug, Default)]
pub struct PrTitleConfig<'a> {
    /// `"enabled"` adds semantic-commit prefix; anything else skips it.
    pub semantic_commits: Option<&'a str>,
    /// Action verb override (e.g. `"Pin"`).  `None` → `"Update"`.
    pub action: Option<&'a str>,
    /// Explicit prefix (e.g. `"fix(deps):"`).  When set, overrides semantic prefix.
    pub custom_prefix: Option<&'a str>,
    /// Custom topic template.  Supports `{{depName}}` / `{{{depName}}}`.
    /// `None` → `"dependency {dep_name}"`.
    pub commit_message_topic: Option<&'a str>,
    /// Semantic-commit type (default `"chore"`).
    pub semantic_commit_type: &'a str,
    /// Semantic-commit scope (default `"deps"`).  Empty string → no parentheses.
    pub semantic_commit_scope: &'a str,
    /// Overrides the "to {{newVersion}}" extra segment.
    /// Supports `{{newVersion}}`, `{{currentVersion}}`, `{{depName}}` substitution.
    /// `None` → default `"to {new_version}"`.
    pub commit_message_extra: Option<&'a str>,
    /// Free-form suffix appended after the complete commit message.
    /// `None` → no suffix.
    pub commit_message_suffix: Option<&'a str>,
    /// Currently installed version — used for `{{currentVersion}}` template substitution
    /// in `commitMessageExtra` and `commitMessageTopic`.  `None` when not available.
    pub current_version: Option<&'a str>,
}

impl<'a> PrTitleConfig<'a> {
    /// Return a config with the default semantic-commit type/scope.
    pub fn with_defaults() -> Self {
        Self {
            semantic_commit_type: "chore",
            semantic_commit_scope: "deps",
            ..Default::default()
        }
    }
}

/// Generate the PR title / commit message for a dependency update.
///
/// Mirrors Renovate's default `commitMessage` template:
/// ```text
/// {commitMessagePrefix} {commitMessageAction} {commitMessageTopic} {commitMessageExtra} {commitMessageSuffix}
/// ```
///
/// # Examples
///
/// ```
/// # use renovate_core::branch::{pr_title, PrTitleConfig};
/// // Default plain title.
/// assert_eq!(
///     pr_title("express", "4.18.2", false, &PrTitleConfig::with_defaults()),
///     "Update dependency express to 4.18.2",
/// );
///
/// // Semantic commits enabled.
/// let cfg = PrTitleConfig { semantic_commits: Some("enabled"), ..PrTitleConfig::with_defaults() };
/// assert_eq!(
///     pr_title("express", "4.18.2", false, &cfg),
///     "chore(deps): Update dependency express to 4.18.2",
/// );
/// ```
///
/// Renovate reference:
/// - `lib/config/options/index.ts` — `commitMessage`, `commitMessageAction`,
///   `commitMessageTopic`, `commitMessageExtra`, `commitMessagePrefix`
/// - `lib/workers/repository/updates/commit.ts` — commit body generation
pub fn pr_title(
    dep_name: &str,
    new_version: &str,
    is_major: bool,
    cfg: &PrTitleConfig<'_>,
) -> String {
    let action = cfg.action.unwrap_or("Update");
    let topic = if let Some(t) = cfg.commit_message_topic {
        let cur = cfg.current_version.unwrap_or("");
        // Basic Handlebars substitution.
        t.replace("{{{depName}}}", dep_name)
            .replace("{{depName}}", dep_name)
            .replace("{{{currentVersion}}}", cur)
            .replace("{{currentVersion}}", cur)
            .replace("{{{newVersion}}}", new_version)
            .replace("{{newVersion}}", new_version)
    } else {
        format!("dependency {dep_name}")
    };

    // Build `extra` — either from the template or the default "to {version}".
    let extra = if let Some(tmpl) = cfg.commit_message_extra {
        if tmpl.is_empty() {
            String::new()
        } else {
            let cur = cfg.current_version.unwrap_or("");
            tmpl.replace("{{{newVersion}}}", new_version)
                .replace("{{newVersion}}", new_version)
                .replace("{{{currentVersion}}}", cur)
                .replace("{{currentVersion}}", cur)
                .replace("{{{depName}}}", dep_name)
                .replace("{{depName}}", dep_name)
        }
    } else {
        format!("to {new_version}")
    };

    let body = if extra.is_empty() {
        format!("{action} {topic}")
    } else {
        format!("{action} {topic} {extra}")
    };

    let full = if let Some(prefix) = cfg.custom_prefix {
        // Explicit prefix overrides semantic prefix entirely.
        format!("{prefix} {body}")
    } else {
        match cfg.semantic_commits {
            Some("enabled") => {
                let breaking = if is_major { "!" } else { "" };
                let scope = if cfg.semantic_commit_scope.is_empty() {
                    String::new()
                } else {
                    format!("({})", cfg.semantic_commit_scope)
                };
                format!("{}{scope}{breaking}: {body}", cfg.semantic_commit_type)
            }
            _ => body,
        }
    };

    match cfg.commit_message_suffix {
        Some(suffix) if !suffix.is_empty() => format!("{full} {suffix}"),
        _ => full,
    }
}

/// Thin wrapper around `pr_title` for callers that need a simple call with
/// minimal configuration, kept for internal convenience.
///
/// Uses the old 9-parameter signature internally by building a `PrTitleConfig`.
#[allow(clippy::too_many_arguments)]
pub fn pr_title_full(
    dep_name: &str,
    new_version: &str,
    is_major: bool,
    semantic_commits: Option<&str>,
    action: Option<&str>,
    custom_prefix: Option<&str>,
    commit_message_topic: Option<&str>,
    semantic_commit_type: &str,
    semantic_commit_scope: &str,
) -> String {
    pr_title(
        dep_name,
        new_version,
        is_major,
        &PrTitleConfig {
            semantic_commits,
            action,
            custom_prefix,
            commit_message_topic,
            semantic_commit_type,
            semantic_commit_scope,
            commit_message_extra: None,
            commit_message_suffix: None,
            current_version: None,
        },
    )
}

/// Remove characters that are invalid or disruptive in git branch names.
///
/// Mirrors Renovate's `cleanBranchName` (default mode, `branchNameStrict=false`):
/// - Strips `~`, `^`, `?`, `[`, `\`, leading `.`, trailing `.`
/// - Collapses multiple consecutive `-` into one
/// - Trims leading/trailing dashes from each path component
///
/// Note: `clean-git-ref` in the original implementation is more exhaustive.
/// This covers the common cases.
fn clean_branch_name(name: &str) -> String {
    let cleaned = name
        .chars()
        .filter(|c| !matches!(c, '~' | '^' | '?' | '[' | '\\' | '\x00'..='\x1f'))
        .collect::<String>();

    cleaned
        .trim_start_matches('.')
        .trim_end_matches('.')
        .split('/')
        .map(|segment| {
            // Trim leading/trailing dashes from each segment.
            segment
                .trim_start_matches('-')
                .trim_end_matches('-')
                .to_owned()
        })
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── sanitize_dep_name ────────────────────────────────────────────────────

    #[test]
    fn sanitize_plain_name() {
        assert_eq!(sanitize_dep_name("lodash"), "lodash");
        assert_eq!(sanitize_dep_name("react"), "react");
    }

    #[test]
    fn sanitize_scoped_npm_package() {
        assert_eq!(sanitize_dep_name("@angular/core"), "angular-core");
        assert_eq!(sanitize_dep_name("@aws-sdk/client-s3"), "aws-sdk-client-s3");
    }

    #[test]
    fn sanitize_types_prefix_stripped() {
        assert_eq!(sanitize_dep_name("@types/lodash"), "lodash");
        assert_eq!(sanitize_dep_name("@types/react"), "react");
    }

    #[test]
    fn sanitize_url_style_dep() {
        // https:// → replace "/" with "-" (×2) and ":" with "-", then collapse:
        // "https://..." → "https---..." → "https-..."
        assert_eq!(
            sanitize_dep_name("https://some.host.name/a/path/to.git"),
            "https-some.host.name-a-path-to.git"
        );
    }

    #[test]
    fn sanitize_go_module() {
        assert_eq!(
            sanitize_dep_name("github.com/user/repo"),
            "github.com-user-repo"
        );
    }

    #[test]
    fn sanitize_colon_replaced() {
        assert_eq!(sanitize_dep_name("foo:bar"), "foo-bar");
    }

    #[test]
    fn sanitize_consecutive_dashes_collapsed() {
        // Multiple special chars in sequence should collapse to one dash.
        assert_eq!(sanitize_dep_name("@org/foo/bar"), "org-foo-bar");
    }

    #[test]
    fn sanitize_lowercased() {
        assert_eq!(sanitize_dep_name("ReactJS"), "reactjs");
        assert_eq!(sanitize_dep_name("@Angular/Core"), "angular-core");
    }

    // ── branch_topic ─────────────────────────────────────────────────────────

    #[test]
    fn branch_topic_default_no_minor_component() {
        // Default: patch/minor updates share {dep}-{major}.x branch.
        assert_eq!(
            branch_topic("lodash", 4, 17, true, false, false, false),
            "lodash-4.x"
        );
        assert_eq!(
            branch_topic("lodash", 4, 17, false, true, false, false),
            "lodash-4.x"
        );
    }

    #[test]
    fn branch_topic_separate_minor_patch_for_patch_update() {
        assert_eq!(
            branch_topic("lodash", 4, 17, true, false, true, false),
            "lodash-4.17.x"
        );
    }

    #[test]
    fn branch_topic_separate_minor_patch_for_minor_not_applied() {
        // separateMinorPatch only adds minor component for patch updates.
        assert_eq!(
            branch_topic("lodash", 4, 17, false, true, true, false),
            "lodash-4.x"
        );
    }

    #[test]
    fn branch_topic_separate_multiple_minor_for_minor_update() {
        // separateMultipleMinor adds minor component for minor updates.
        assert_eq!(
            branch_topic("lodash", 4, 17, false, true, false, true),
            "lodash-4.17.x"
        );
    }

    #[test]
    fn branch_topic_separate_multiple_minor_not_applied_to_patch() {
        // separateMultipleMinor does not affect patch updates.
        assert_eq!(
            branch_topic("lodash", 4, 17, true, false, false, true),
            "lodash-4.x"
        );
    }

    #[test]
    fn branch_topic_scoped_package() {
        assert_eq!(
            branch_topic("@angular/core", 17, 0, false, false, false, false),
            "angular-core-17.x"
        );
    }

    #[test]
    fn branch_topic_major_update() {
        assert_eq!(
            branch_topic("react", 18, 0, false, false, false, false),
            "react-18.x"
        );
    }

    // ── group_branch_topic ────────────────────────────────────────────────────

    #[test]
    fn group_branch_topic_spaces_to_hyphens() {
        assert_eq!(group_branch_topic("All Dependencies"), "all-dependencies");
    }

    #[test]
    fn group_branch_topic_special_chars_stripped() {
        assert_eq!(group_branch_topic("@angular/**"), "angular");
    }

    #[test]
    fn group_branch_topic_no_trailing_hyphen() {
        assert_eq!(group_branch_topic("Python packages"), "python-packages");
    }

    #[test]
    fn group_branch_topic_already_clean() {
        assert_eq!(group_branch_topic("lodash"), "lodash");
    }

    // ── branch_name ──────────────────────────────────────────────────────────

    #[test]
    fn branch_name_default_prefix() {
        assert_eq!(
            branch_name("renovate/", "", "lodash-4.x"),
            "renovate/lodash-4.x"
        );
    }

    #[test]
    fn branch_name_custom_prefix() {
        assert_eq!(branch_name("deps/", "", "react-18.x"), "deps/react-18.x");
    }

    #[test]
    fn branch_name_with_additional_prefix() {
        assert_eq!(
            branch_name("renovate/", "chore-", "lodash-4.x"),
            "renovate/chore-lodash-4.x"
        );
    }

    #[test]
    fn branch_name_roundtrip() {
        let topic = branch_topic("@angular/core", 17, 0, false, false, false, false);
        let name = branch_name("renovate/", "", &topic);
        assert_eq!(name, "renovate/angular-core-17.x");
    }

    // ── pr_title ─────────────────────────────────────────────────────────────

    #[test]
    fn pr_title_plain_minor() {
        assert_eq!(
            pr_title("express", "4.18.2", false, &PrTitleConfig::with_defaults()),
            "Update dependency express to 4.18.2"
        );
    }

    #[test]
    fn pr_title_plain_major() {
        assert_eq!(
            pr_title("lodash", "5.0.0", true, &PrTitleConfig::with_defaults()),
            "Update dependency lodash to 5.0.0"
        );
    }

    #[test]
    fn pr_title_semantic_minor() {
        let cfg = PrTitleConfig {
            semantic_commits: Some("enabled"),
            ..PrTitleConfig::with_defaults()
        };
        assert_eq!(
            pr_title("express", "4.18.2", false, &cfg),
            "chore(deps): Update dependency express to 4.18.2"
        );
    }

    #[test]
    fn pr_title_semantic_major_breaking() {
        let cfg = PrTitleConfig {
            semantic_commits: Some("enabled"),
            ..PrTitleConfig::with_defaults()
        };
        assert_eq!(
            pr_title("lodash", "5.0.0", true, &cfg),
            "chore(deps)!: Update dependency lodash to 5.0.0"
        );
    }

    #[test]
    fn pr_title_semantic_disabled() {
        // "disabled" semantic_commits → no prefix
        let cfg = PrTitleConfig {
            semantic_commits: Some("disabled"),
            ..PrTitleConfig::with_defaults()
        };
        assert_eq!(
            pr_title("react", "18.0.0", true, &cfg),
            "Update dependency react to 18.0.0"
        );
    }

    #[test]
    fn pr_title_scoped_package() {
        let cfg = PrTitleConfig {
            semantic_commits: Some("enabled"),
            ..PrTitleConfig::with_defaults()
        };
        assert_eq!(
            pr_title("@angular/core", "17.1.0", false, &cfg),
            "chore(deps): Update dependency @angular/core to 17.1.0"
        );
    }

    #[test]
    fn pr_title_custom_action() {
        // commitMessageAction: "Bump" → custom action verb
        let cfg = PrTitleConfig {
            action: Some("Bump"),
            ..PrTitleConfig::with_defaults()
        };
        assert_eq!(
            pr_title("lodash", "4.17.21", false, &cfg),
            "Bump dependency lodash to 4.17.21"
        );
    }

    #[test]
    fn pr_title_full_custom_type_and_scope() {
        // semanticCommitType: "fix", semanticCommitScope: "security"
        assert_eq!(
            pr_title_full(
                "express",
                "4.18.2",
                false,
                Some("enabled"),
                None,
                None,
                None,
                "fix",
                "security"
            ),
            "fix(security): Update dependency express to 4.18.2"
        );
    }

    #[test]
    fn pr_title_full_empty_scope() {
        // semanticCommitScope: "" → no parentheses
        assert_eq!(
            pr_title_full(
                "lodash",
                "5.0.0",
                true,
                Some("enabled"),
                None,
                None,
                None,
                "chore",
                ""
            ),
            "chore!: Update dependency lodash to 5.0.0"
        );
    }

    #[test]
    fn pr_title_custom_prefix_overrides_semantic() {
        // commitMessagePrefix: "fix(deps):" overrides chore(deps) even with semantic enabled
        let cfg = PrTitleConfig {
            semantic_commits: Some("enabled"),
            custom_prefix: Some("fix(deps):"),
            ..PrTitleConfig::with_defaults()
        };
        assert_eq!(
            pr_title("express", "4.18.2", false, &cfg),
            "fix(deps): Update dependency express to 4.18.2"
        );
    }

    #[test]
    fn pr_title_custom_prefix_and_action() {
        let cfg = PrTitleConfig {
            semantic_commits: Some("enabled"),
            action: Some("Pin"),
            custom_prefix: Some("build(deps):"),
            ..PrTitleConfig::with_defaults()
        };
        assert_eq!(
            pr_title("react", "18.0.0", true, &cfg),
            "build(deps): Pin dependency react to 18.0.0"
        );
    }

    // ── pr_title commitMessageTopic ──────────────────────────────────────────

    #[test]
    fn pr_title_custom_topic_literal() {
        let cfg = PrTitleConfig {
            commit_message_topic: Some("Docker image nginx"),
            ..PrTitleConfig::with_defaults()
        };
        assert_eq!(
            pr_title("nginx", "1.25", false, &cfg),
            "Update Docker image nginx to 1.25"
        );
    }

    #[test]
    fn pr_title_custom_topic_with_dep_name_template() {
        let cfg = PrTitleConfig {
            commit_message_topic: Some("Docker image {{depName}}"),
            ..PrTitleConfig::with_defaults()
        };
        assert_eq!(
            pr_title("nginx", "1.25", false, &cfg),
            "Update Docker image nginx to 1.25"
        );
    }

    #[test]
    fn pr_title_custom_topic_triple_brace() {
        let cfg = PrTitleConfig {
            commit_message_topic: Some("Docker image {{{depName}}}"),
            ..PrTitleConfig::with_defaults()
        };
        assert_eq!(
            pr_title("nginx", "1.25", false, &cfg),
            "Update Docker image nginx to 1.25"
        );
    }

    #[test]
    fn pr_title_default_topic_when_none() {
        // None → uses default "dependency {dep_name}"
        assert_eq!(
            pr_title("nginx", "1.25", false, &PrTitleConfig::with_defaults()),
            "Update dependency nginx to 1.25"
        );
    }

    #[test]
    fn pr_title_semantic_with_custom_topic() {
        let cfg = PrTitleConfig {
            semantic_commits: Some("enabled"),
            commit_message_topic: Some("Helm chart {{depName}}"),
            ..PrTitleConfig::with_defaults()
        };
        assert_eq!(
            pr_title("nginx", "1.25", true, &cfg),
            "chore(deps)!: Update Helm chart nginx to 1.25"
        );
    }

    // ── pr_title commitMessageExtra / commitMessageSuffix ────────────────────

    #[test]
    fn pr_title_custom_extra_with_version_template() {
        let cfg = PrTitleConfig {
            commit_message_extra: Some("({{newVersion}})"),
            ..PrTitleConfig::with_defaults()
        };
        assert_eq!(
            pr_title("lodash", "4.17.21", false, &cfg),
            "Update dependency lodash (4.17.21)"
        );
    }

    #[test]
    fn pr_title_empty_extra_omits_version_segment() {
        let cfg = PrTitleConfig {
            commit_message_extra: Some(""),
            ..PrTitleConfig::with_defaults()
        };
        assert_eq!(
            pr_title("lodash", "4.17.21", false, &cfg),
            "Update dependency lodash"
        );
    }

    #[test]
    fn pr_title_commit_message_suffix_appended() {
        let cfg = PrTitleConfig {
            commit_message_suffix: Some("[skip ci]"),
            ..PrTitleConfig::with_defaults()
        };
        assert_eq!(
            pr_title("express", "4.18.2", false, &cfg),
            "Update dependency express to 4.18.2 [skip ci]"
        );
    }

    #[test]
    fn pr_title_suffix_with_semantic_commits() {
        let cfg = PrTitleConfig {
            semantic_commits: Some("enabled"),
            commit_message_suffix: Some("[skip ci]"),
            ..PrTitleConfig::with_defaults()
        };
        assert_eq!(
            pr_title("express", "4.18.2", false, &cfg),
            "chore(deps): Update dependency express to 4.18.2 [skip ci]"
        );
    }

    #[test]
    fn pr_title_current_version_template_in_extra() {
        let cfg = PrTitleConfig {
            commit_message_extra: Some("{{currentVersion}} → {{newVersion}}"),
            current_version: Some("4.0.0"),
            ..PrTitleConfig::with_defaults()
        };
        assert_eq!(
            pr_title("lodash", "4.17.21", false, &cfg),
            "Update dependency lodash 4.0.0 → 4.17.21"
        );
    }

    #[test]
    fn pr_title_current_version_in_topic_template() {
        let cfg = PrTitleConfig {
            commit_message_topic: Some("{{depName}} from {{currentVersion}}"),
            current_version: Some("1.0.0"),
            ..PrTitleConfig::with_defaults()
        };
        assert_eq!(
            pr_title("express", "2.0.0", false, &cfg),
            "Update express from 1.0.0 to 2.0.0"
        );
    }

    // ── hashed_branch_name ───────────────────────────────────────────────────

    #[test]
    fn hashed_branch_length_produces_exact_length() {
        let name = hashed_branch_name("renovate/", "", "lodash-4.x", 20);
        assert_eq!(name.len(), 20, "branch name should be exactly 20 chars");
        assert!(name.starts_with("renovate/"));
    }

    #[test]
    fn hashed_branch_length_different_topics_differ() {
        let a = hashed_branch_name("renovate/", "", "lodash-4.x", 30);
        let b = hashed_branch_name("renovate/", "", "react-18.x", 30);
        assert_ne!(a, b, "different topics must produce different hashes");
    }

    #[test]
    fn hashed_branch_length_too_small_uses_min() {
        // hashedBranchLength=10 minus prefix "renovate/"(9) = 1, below MIN_HASH_LENGTH(6)
        // → use MIN_HASH_LENGTH(6), so result = "renovate/" + 6 hex chars = 15 chars
        let name = hashed_branch_name("renovate/", "", "lodash-4.x", 10);
        assert_eq!(name.len(), 9 + 6, "should use minimum 6 hex chars");
        assert!(name.starts_with("renovate/"));
    }

    #[test]
    fn hashed_branch_length_deterministic() {
        let a = hashed_branch_name("renovate/", "", "lodash-4.x", 30);
        let b = hashed_branch_name("renovate/", "", "lodash-4.x", 30);
        assert_eq!(a, b, "same inputs must produce same output");
    }

    #[test]
    fn hashed_branch_length_with_additional_prefix() {
        let without = hashed_branch_name("renovate/", "", "lodash-4.x", 30);
        let with_prefix = hashed_branch_name("renovate/", "chore-", "lodash-4.x", 30);
        assert_ne!(
            without, with_prefix,
            "additionalBranchPrefix changes hash input"
        );
        assert_eq!(with_prefix.len(), 30);
        // Both must start with branch_prefix, not additionalBranchPrefix
        assert!(with_prefix.starts_with("renovate/"));
    }

    #[test]
    fn hashed_branch_is_hex_only() {
        let name = hashed_branch_name("r/", "", "dep-1.x", 20);
        let hash_part = &name[2..]; // strip "r/"
        assert!(
            hash_part.chars().all(|c| c.is_ascii_hexdigit()),
            "hash part must be lowercase hex: {hash_part}"
        );
    }
}
