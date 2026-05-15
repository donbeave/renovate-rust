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

    // Ported: "should correctly map the scm-manager type of a PR with the $scmPrState to the Renovate PR type"
    //         — modules/platform/scm-manager/mapper.spec.ts line 5
    #[test]
    fn map_pr_state_all_cases() {
        assert_eq!(map_pr_state(ScmPrState::Open), ("open", false));
        assert_eq!(map_pr_state(ScmPrState::Draft), ("open", true));
        assert_eq!(map_pr_state(ScmPrState::Rejected), ("closed", false));
        assert_eq!(map_pr_state(ScmPrState::Merged), ("merged", false));
    }

    // Ported: "map merge strategy $strategy on PR merge method $method"
    //         — modules/platform/scm-manager/utils.spec.ts line 16
    #[test]
    fn get_scm_merge_method_all_cases() {
        assert_eq!(get_scm_merge_method(None), None);
        assert_eq!(get_scm_merge_method(Some("auto")), None);
        assert_eq!(get_scm_merge_method(Some("fast-forward")), Some("FAST_FORWARD_ONLY"));
        assert_eq!(get_scm_merge_method(Some("merge-commit")), Some("MERGE_COMMIT"));
        assert_eq!(get_scm_merge_method(Some("rebase")), Some("REBASE"));
        assert_eq!(get_scm_merge_method(Some("squash")), Some("SQUASH"));
    }

    // Ported: "adjust $body to smart link $result" — modules/platform/scm-manager/utils.spec.ts line 39
    #[test]
    fn smart_links_replaces_pull_links() {
        assert_eq!(smart_links(""), "");
        assert_eq!(smart_links("](../pull/"), "](pulls/");
    }

    // Ported: "match scm pr state $pr.state to renovate pr state $state"
    //         — modules/platform/scm-manager/utils.spec.ts line 60
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
