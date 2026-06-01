//! SCM Manager platform utilities.
//!
//! Mirrors:
//! - `lib/modules/platform/scm-manager/mapper.ts`
//! - `lib/modules/platform/scm-manager/utils.ts`

/// State of a pull request in SCM Manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScmPrState {
    Open,
    Draft,
    Rejected,
    Merged,
}

/// Renovate-normalized PR state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenovatePr {
    pub state: &'static str,
    pub is_draft: bool,
}

/// Map an SCM Manager PR state to the Renovate-normalized representation.
///
/// Mirrors `mapPrFromScmToRenovate` from `lib/modules/platform/scm-manager/mapper.ts`.
pub fn map_pr_state(scm_state: ScmPrState) -> (&'static str, bool) {
    match scm_state {
        ScmPrState::Draft => ("open", true),
        ScmPrState::Open => ("open", false),
        ScmPrState::Rejected => ("closed", false),
        ScmPrState::Merged => ("merged", false),
    }
}

/// Map an SCM Manager merge strategy to the API's PrMergeMethod value.
///
/// Mirrors `getMergeMethod` from `lib/modules/platform/scm-manager/utils.ts`.
pub fn get_scm_merge_method(strategy: Option<&str>) -> Option<&'static str> {
    match strategy? {
        "fast-forward" => Some("FAST_FORWARD_ONLY"),
        "merge-commit" => Some("MERGE_COMMIT"),
        "rebase" => Some("REBASE"),
        "squash" => Some("SQUASH"),
        _ => None,
    }
}

/// Replace `](../pull/` with `](pulls/` in PR body text for SCM Manager smart links.
///
/// Mirrors `smartLinks` from `lib/modules/platform/scm-manager/utils.ts`.
pub fn smart_links(body: &str) -> String {
    body.replace("](../pull/", "](pulls/")
}

/// Check if a Renovate PR matches a given state filter.
///
/// Mirrors `matchPrState` from `lib/modules/platform/scm-manager/utils.ts`.
pub fn match_pr_state(pr_state: &str, filter: &str) -> bool {
    if filter == "all" {
        return true;
    }
    if filter == "!open" {
        return pr_state == "closed" || pr_state == "merged";
    }
    filter == pr_state
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "should correctly map the scm-manager type of a PR with the $scmPrState to the Renovate PR type" — modules/platform/scm-manager/mapper.spec.ts line 12
    //         — modules/platform/scm-manager/mapper.spec.ts line 5
    #[test]
    fn map_pr_state_all_cases() {
        assert_eq!(map_pr_state(ScmPrState::Open), ("open", false));
        assert_eq!(map_pr_state(ScmPrState::Draft), ("open", true));
        assert_eq!(map_pr_state(ScmPrState::Rejected), ("closed", false));
        assert_eq!(map_pr_state(ScmPrState::Merged), ("merged", false));
    }

    // Ported: "map merge strategy $strategy on PR merge method $method" — modules/platform/scm-manager/utils.spec.ts line 25
    //         — modules/platform/scm-manager/utils.spec.ts line 16
    #[test]
    fn get_scm_merge_method_all_cases() {
        assert_eq!(get_scm_merge_method(None), None);
        assert_eq!(get_scm_merge_method(Some("auto")), None);
        assert_eq!(
            get_scm_merge_method(Some("fast-forward")),
            Some("FAST_FORWARD_ONLY")
        );
        assert_eq!(
            get_scm_merge_method(Some("merge-commit")),
            Some("MERGE_COMMIT")
        );
        assert_eq!(get_scm_merge_method(Some("rebase")), Some("REBASE"));
        assert_eq!(get_scm_merge_method(Some("squash")), Some("SQUASH"));
    }

    // Ported: "adjust $body to smart link $result" — modules/platform/scm-manager/utils.spec.ts line 39
    #[test]
    fn smart_links_replaces_pull_links() {
        assert_eq!(smart_links(""), "");
        assert_eq!(smart_links("](../pull/"), "](pulls/");
    }

    // Ported: "match scm pr state $pr.state to renovate pr state $state" — modules/platform/scm-manager/utils.spec.ts line 76
    //         — modules/platform/scm-manager/utils.spec.ts line 61
    #[test]
    fn match_pr_state_all_cases() {
        // filter = 'all' → always true
        assert!(match_pr_state("open", "all"));
        assert!(match_pr_state("merged", "all"));
        assert!(match_pr_state("closed", "all"));

        // filter = 'open'
        assert!(match_pr_state("open", "open"));
        assert!(!match_pr_state("merged", "open"));
        assert!(!match_pr_state("closed", "open"));

        // filter = '!open'
        assert!(!match_pr_state("open", "!open"));
        assert!(match_pr_state("merged", "!open"));
        assert!(match_pr_state("closed", "!open"));

        // filter = 'closed'
        assert!(!match_pr_state("open", "closed"));
        assert!(!match_pr_state("merged", "closed"));
        assert!(match_pr_state("closed", "closed"));
    }
}

// ---------------------------------------------------------------------------
// getRepoUrl — lib/modules/platform/scm-manager/utils.ts
// ---------------------------------------------------------------------------

/// A protocol link with name and href from `repo._links.protocol`.
#[derive(Debug, Clone)]
pub struct ProtocolLink {
    pub name: String,
    pub href: String,
}

/// Error variants from `getRepoUrl`.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum GetRepoUrlError {
    #[error("Missing protocol links.")]
    MissingProtocolLinks,
    #[error("Expected protocol links to be an array of links.")]
    NotAnArray,
    #[error("MISSING_SSH_LINKS")]
    MissingSshLink,
    #[error("MISSING_HTTP_LINK")]
    MissingHttpLink,
    #[error("MALFORMED_HTTP_LINK")]
    MalformedHttpLink,
}

/// Resolve the repository URL for a given git-URL option.
///
/// `protocol_links` is the `repo._links.protocol` array from SCM Manager.
/// `git_url` is one of `Some("ssh")`, `Some("default")`, `Some("endpoint")`,
/// or `None` (both `None` and `"default"` and `"endpoint"` use HTTP).
/// `username` / `token` are the SCM Manager credentials to embed in the HTTP URL.
///
/// Mirrors `getRepoUrl` from `lib/modules/platform/scm-manager/utils.ts`.
pub fn get_repo_url(
    protocol_links: &[ProtocolLink],
    git_url: Option<&str>,
    username: Option<&str>,
    token: Option<&str>,
) -> Result<String, GetRepoUrlError> {
    if git_url == Some("ssh") {
        let ssh_url = protocol_links
            .iter()
            .find(|l| l.name == "ssh")
            .map(|l| l.href.clone())
            .ok_or(GetRepoUrlError::MissingSshLink)?;
        return Ok(ssh_url);
    }

    let http_url = protocol_links
        .iter()
        .find(|l| l.name == "http")
        .map(|l| l.href.clone())
        .ok_or(GetRepoUrlError::MissingHttpLink)?;

    let mut parsed = url::Url::parse(&http_url).map_err(|_| GetRepoUrlError::MalformedHttpLink)?;

    let _ = parsed.set_username(username.unwrap_or(""));
    let _ = parsed.set_password(if token.is_some_and(|t| !t.is_empty()) {
        token
    } else {
        None
    });

    Ok(parsed.to_string())
}

#[cfg(test)]
mod get_repo_url_tests {
    use super::*;

    fn link(name: &str, href: &str) -> ProtocolLink {
        ProtocolLink {
            name: name.to_owned(),
            href: href.to_owned(),
        }
    }

    const GIT_HTTP: &str = "http://localhost:8081/scm/repo/default/repo";
    const GIT_SSH: &str = "ssh://localhost:2222/scm/repo/default/repo";

    // Ported: "should use the provided ssh link" — scm-manager/utils.spec.ts line 158
    #[test]
    fn get_repo_url_uses_ssh_link() {
        let links = vec![link("http", GIT_HTTP), link("ssh", GIT_SSH)];
        let result = get_repo_url(&links, Some("ssh"), None, None).unwrap();
        assert_eq!(result, GIT_SSH);
    }

    // Ported: "should throw error because of missing SSH link" — scm-manager/utils.spec.ts line 132
    #[test]
    fn get_repo_url_errors_missing_ssh() {
        let links = vec![link("http", GIT_HTTP)];
        let err = get_repo_url(&links, Some("ssh"), None, None).unwrap_err();
        assert_eq!(err, GetRepoUrlError::MissingSshLink);
    }

    // Ported: "should throw error for option $gitUrl, because protocol links are missing" — scm-manager/utils.spec.ts line 117
    #[test]
    fn get_repo_url_errors_no_http_link() {
        let links = vec![link("ssh", GIT_SSH)]; // no http link
        let err = get_repo_url(&links, None, None, None).unwrap_err();
        assert_eq!(err, GetRepoUrlError::MissingHttpLink);
    }

    // Ported: "should throw error because of malformed HTTP link with option $gitUrl" — scm-manager/utils.spec.ts line 192
    #[test]
    fn get_repo_url_errors_malformed_http_link() {
        let links = vec![link("http", "invalid url")];
        let err = get_repo_url(&links, None, None, None).unwrap_err();
        assert_eq!(err, GetRepoUrlError::MalformedHttpLink);
    }

    // Ported: "should use empty string, because username was not provided with option $gitUrl" — scm-manager/utils.spec.ts line 213
    #[test]
    fn get_repo_url_no_username_gives_plain_url() {
        let links = vec![link("http", GIT_HTTP)];
        let result = get_repo_url(&links, None, None, None).unwrap();
        assert_eq!(result, GIT_HTTP);
    }

    // Ported: "should provide the HTTP link with username, for option $gitUrl" — scm-manager/utils.spec.ts line 258
    #[test]
    fn get_repo_url_with_username_and_token() {
        let links = vec![link("http", GIT_HTTP)];
        let result = get_repo_url(&links, None, Some("tzerr"), Some("token")).unwrap();
        assert!(result.contains("tzerr"));
        assert!(result.contains("localhost"));
    }
}
