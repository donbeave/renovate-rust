//! Config validation helpers module.
//!
//! Renovate reference: `lib/config/validation-helpers/`.

pub mod match_base_branches;
pub mod regex_glob_matchers;
pub mod types;
pub mod utils;

pub use match_base_branches::check_match_base_branches;
pub use regex_glob_matchers::check_regex_glob_matchers;
pub use types::{CheckBaseBranchesArgs, CheckMatcherArgs, ValidationMessage, ValidationType};
pub use utils::{
    get_parent_name, get_validation_message, is_false_global, validate_number,
    validate_plain_object,
};
