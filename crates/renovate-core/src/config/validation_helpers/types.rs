//! Validation helper types.
//!
//! Renovate reference: `lib/config/validation-helpers/types.ts`.

use serde_json::Value;

/// A single validation message (error or warning).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationMessage {
    pub topic: String,
    pub message: String,
}

/// Arguments for checking base branch patterns.
#[derive(Debug, Clone)]
pub struct CheckBaseBranchesArgs<'a> {
    pub resolved_rule: &'a Value,
    pub current_path: String,
    pub base_branch_patterns: Option<&'a [String]>,
}

/// Arguments for checking regex/glob matchers.
#[derive(Debug, Clone)]
pub struct CheckMatcherArgs {
    pub val: Value,
    pub current_path: String,
}

/// Types of validation to perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationType {
    Error,
    Warning,
}
