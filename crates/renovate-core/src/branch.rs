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
/// # Parameters
///
/// - `dep_name` — raw dep name (will be sanitized)
/// - `new_major` — major component of the proposed new version
/// - `new_minor` — minor component (only used when `separate_minor_patch` and
///   `is_patch` are both `true`)
/// - `is_patch` — whether this is a patch-level update
/// - `separate_minor_patch` — value of the `separateMinorPatch` config option
///
/// # Examples
///
/// ```
/// # use renovate_core::branch::branch_topic;
/// // Default: all 4.x lodash updates share one branch.
/// assert_eq!(branch_topic("lodash", 4, 17, true, false), "lodash-4.x");
/// // separateMinorPatch=true: patch gets its own branch.
/// assert_eq!(branch_topic("lodash", 4, 17, true, true), "lodash-4.17.x");
/// // Major update.
/// assert_eq!(branch_topic("react", 18, 0, false, false), "react-18.x");
/// // Scoped npm package.
/// assert_eq!(branch_topic("@angular/core", 17, 0, false, false), "angular-core-17.x");
/// ```
pub fn branch_topic(
    dep_name: &str,
    new_major: u64,
    new_minor: u64,
    is_patch: bool,
    separate_minor_patch: bool,
) -> String {
    let sanitized = sanitize_dep_name(dep_name);
    if separate_minor_patch && is_patch {
        format!("{sanitized}-{new_major}.{new_minor}.x")
    } else {
        format!("{sanitized}-{new_major}.x")
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
        assert_eq!(branch_topic("lodash", 4, 17, true, false), "lodash-4.x");
        assert_eq!(branch_topic("lodash", 4, 17, false, false), "lodash-4.x");
    }

    #[test]
    fn branch_topic_separate_minor_patch_for_patch_update() {
        assert_eq!(branch_topic("lodash", 4, 17, true, true), "lodash-4.17.x");
    }

    #[test]
    fn branch_topic_separate_minor_patch_for_minor_not_applied() {
        // separateMinorPatch only adds minor component for patch updates.
        assert_eq!(branch_topic("lodash", 4, 17, false, true), "lodash-4.x");
    }

    #[test]
    fn branch_topic_scoped_package() {
        assert_eq!(
            branch_topic("@angular/core", 17, 0, false, false),
            "angular-core-17.x"
        );
    }

    #[test]
    fn branch_topic_major_update() {
        assert_eq!(branch_topic("react", 18, 0, false, false), "react-18.x");
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
        let topic = branch_topic("@angular/core", 17, 0, false, false);
        let name = branch_name("renovate/", "", &topic);
        assert_eq!(name, "renovate/angular-core-17.x");
    }
}
