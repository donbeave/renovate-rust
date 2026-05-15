//! Shared Gitea/Forgejo platform utility functions.
//!
//! Both Gitea and Forgejo share the same API structure and utility logic.
//! Mirrors:
//! - `lib/modules/platform/forgejo/utils.ts`
//! - `lib/modules/platform/gitea/utils.ts`

use std::sync::LazyLock;

use regex::Regex;

static TRAILING_API_PATH_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"api/v1/?$").unwrap());

/// Strip the `/api/v1[/]` suffix from a Gitea/Forgejo API endpoint URL.
///
/// Mirrors `trimTrailingApiPath` from both `forgejo/utils.ts` and `gitea/utils.ts`.
pub fn trim_trailing_api_path(url: &str) -> String {
    TRAILING_API_PATH_RE.replace(url, "").into_owned()
}

/// Map a merge strategy name to the Gitea/Forgejo API merge method.
///
/// Returns `None` for `"auto"`, `None`/unknown inputs.
/// Mirrors `getMergeMethod` from `forgejo/utils.ts` and `gitea/utils.ts`.
pub fn get_merge_method(strategy: Option<&str>) -> Option<&'static str> {
    match strategy? {
        "fast-forward" => Some("rebase"),
        "merge-commit" => Some("merge"),
        "rebase" => Some("rebase-merge"),
        "squash" => Some("squash"),
        _ => None,
    }
}

/// Repository-permission flags for usability checks.
#[derive(Debug, Clone)]
pub struct RepoPermissions {
    pub pull: bool,
    pub push: bool,
}

/// Check whether a Gitea/Forgejo repository is usable for Renovate.
///
/// Returns `false` if it's a mirror, lacks pull/push permissions, or has pull requests disabled.
/// Mirrors `usableRepo` from `forgejo/utils.ts` and `gitea/utils.ts`.
pub fn usable_repo(
    is_mirror: bool,
    permissions: &RepoPermissions,
    has_pull_requests: bool,
) -> bool {
    if is_mirror {
        return false;
    }
    if !permissions.pull || !permissions.push {
        return false;
    }
    if !has_pull_requests {
        return false;
    }
    true
}

/// Validate that an endpoint string is a valid HTTP/HTTPS URL.
///
/// Returns `Err` if the endpoint does not start with `http://` or `https://`.
/// Used by `getRepoUrl` when `gitUrl === 'endpoint'`.
pub fn validate_endpoint_url(endpoint: &str) -> Result<(), String> {
    if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
        Ok(())
    } else {
        Err("Configuration error: gitUrl endpoint is not a valid URL".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "trimTrailingApiPath" — modules/platform/forgejo/utils.spec.ts line 26
    // (same test exists in modules/platform/gitea/utils.spec.ts line 26)
    #[test]
    fn trim_trailing_api_path_strips_api_v1() {
        assert_eq!(
            trim_trailing_api_path("https://forgejo.renovatebot.com/api/v1"),
            "https://forgejo.renovatebot.com/"
        );
        assert_eq!(
            trim_trailing_api_path("https://forgejo.renovatebot.com/api/v1/"),
            "https://forgejo.renovatebot.com/"
        );
        assert_eq!(
            trim_trailing_api_path("https://forgejo.renovatebot.com/"),
            "https://forgejo.renovatebot.com/"
        );
        assert_eq!(
            trim_trailing_api_path("https://forgejo.renovatebot.com"),
            "https://forgejo.renovatebot.com"
        );
        assert_eq!(
            trim_trailing_api_path("https://forgejo.renovatebot.com/api/forgejo/api/v1"),
            "https://forgejo.renovatebot.com/api/forgejo/"
        );
    }

    // Ported: "should abort when endpoint is not valid" — modules/platform/forgejo/utils.spec.ts line 45
    // (same test exists in modules/platform/gitea/utils.spec.ts line 45)
    #[test]
    fn validate_endpoint_url_invalid_throws() {
        assert!(validate_endpoint_url("abc").is_err());
    }

    // Ported: "getMergeMethod(\"$value\") == \"$expected\"" — modules/platform/forgejo/utils.spec.ts line 53
    // (same test exists in modules/platform/gitea/utils.spec.ts line 53)
    #[test]
    fn get_merge_method_all_cases() {
        assert_eq!(get_merge_method(Some("auto")), None);
        assert_eq!(get_merge_method(None), None);
        assert_eq!(get_merge_method(Some("fast-forward")), Some("rebase"));
        assert_eq!(get_merge_method(Some("merge-commit")), Some("merge"));
        assert_eq!(get_merge_method(Some("rebase")), Some("rebase-merge"));
        assert_eq!(get_merge_method(Some("squash")), Some("squash"));
    }

    fn full_permissions() -> RepoPermissions {
        RepoPermissions {
            pull: true,
            push: true,
        }
    }

    // Ported: "should return true when repo is usable" — modules/platform/forgejo/utils.spec.ts line 66
    #[test]
    fn usable_repo_returns_true_for_usable_repo() {
        assert!(usable_repo(false, &full_permissions(), true));
    }

    // Ported: "should return false when repo lacks permissions" — modules/platform/forgejo/utils.spec.ts line 70
    #[test]
    fn usable_repo_returns_false_without_permissions() {
        // no pull AND push (admin only)
        assert!(!usable_repo(
            false,
            &RepoPermissions {
                pull: false,
                push: false
            },
            true
        ));
        // pull but no push
        assert!(!usable_repo(
            false,
            &RepoPermissions {
                pull: true,
                push: false
            },
            true
        ));
    }

    // Ported: "should return false when repo has disabled pull requests" — modules/platform/forgejo/utils.spec.ts line 85
    #[test]
    fn usable_repo_returns_false_without_pull_requests() {
        assert!(!usable_repo(false, &full_permissions(), false));
    }
}
