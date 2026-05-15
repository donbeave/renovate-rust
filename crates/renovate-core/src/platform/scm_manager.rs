//! SCM Manager platform utilities.
//!
//! Mirrors:
//! - `lib/modules/platform/scm-manager/mapper.ts`

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
}
