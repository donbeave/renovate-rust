//! Base branch pattern validation.
//!
//! Renovate reference: `lib/config/validation-helpers/match-base-branches.ts`.

use super::types::{CheckBaseBranchesArgs, ValidationMessage};

/// Check that matchBaseBranches is only used when baseBranchPatterns are configured.
///
/// Mirrors `check()` from `lib/config/validation-helpers/match-base-branches.ts`.
pub fn check_match_base_branches(args: &CheckBaseBranchesArgs) -> Vec<ValidationMessage> {
    let mut warnings = Vec::new();

    if let Some(rule) = args.resolved_rule.as_object()
        && let Some(match_base_branches) = rule.get("matchBaseBranches")
        && match_base_branches.is_array()
    {
        let has_base_branches = args
            .base_branch_patterns
            .is_some_and(|patterns| !patterns.is_empty());

        if !has_base_branches {
            warnings.push(ValidationMessage {
                topic: "Configuration Error".to_owned(),
                message: format!(
                    "{}: You must configure baseBranchPatterns in order to use them inside matchBaseBranches.",
                    args.current_path
                ),
            });
        }
    }

    warnings
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn check_passes_when_no_base_branches() {
        let rule = json!({"matchPackageNames": ["foo"]});
        let args = CheckBaseBranchesArgs {
            resolved_rule: &rule,
            current_path: "packageRules.0".to_owned(),
            base_branch_patterns: None,
        };
        assert!(check_match_base_branches(&args).is_empty());
    }

    #[test]
    fn check_warns_when_match_base_branches_without_config() {
        let rule = json!({"matchBaseBranches": ["main"]});
        let args = CheckBaseBranchesArgs {
            resolved_rule: &rule,
            current_path: "packageRules.0".to_owned(),
            base_branch_patterns: None,
        };
        let warnings = check_match_base_branches(&args);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].message.contains("baseBranchPatterns"));
    }

    #[test]
    fn check_passes_with_configured_base_branches() {
        let rule = json!({"matchBaseBranches": ["main"]});
        let patterns = vec!["main".to_owned()];
        let args = CheckBaseBranchesArgs {
            resolved_rule: &rule,
            current_path: "packageRules.0".to_owned(),
            base_branch_patterns: Some(&patterns),
        };
        assert!(check_match_base_branches(&args).is_empty());
    }
}
