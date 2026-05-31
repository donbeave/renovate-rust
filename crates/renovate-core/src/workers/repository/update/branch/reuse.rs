//! Branch reuse logic.
//!
//! Mirrors `lib/workers/repository/update/branch/reuse.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReuseResult {
    CanReuse,
    CannotReuse,
    NeedsRebase,
}

#[derive(Debug, Clone, Default)]
pub struct ReuseConfig {
    pub branch_exists: bool,
    pub branch_name: String,
    pub base_branch: String,
    pub is_behind_base: bool,
    pub is_modified: bool,
    pub is_conflicted: bool,
    pub rebase_when: Option<String>,
    pub automerge: bool,
    pub keep_updated: bool,
}

pub fn should_reuse_branch(config: &ReuseConfig) -> (ReuseResult, bool) {
    if !config.branch_exists {
        return (ReuseResult::CannotReuse, false);
    }

    if config.is_behind_base {
        if config.is_modified {
            return (ReuseResult::CanReuse, true);
        }
        return (ReuseResult::NeedsRebase, false);
    }

    if config.is_conflicted {
        if !config.is_modified {
            let rebase_when = config.rebase_when.as_deref().unwrap_or("auto");
            if rebase_when == "never" && !config.keep_updated {
                return (ReuseResult::CanReuse, false);
            }
            return (ReuseResult::NeedsRebase, false);
        }
        return (ReuseResult::CannotReuse, true);
    }

    (ReuseResult::CanReuse, false)
}

pub fn compare_branch_content(
    existing_content: &str,
    new_content: &str,
) -> bool {
    existing_content == new_content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reuse_result_variants() {
        assert_ne!(ReuseResult::CanReuse, ReuseResult::CannotReuse);
        assert_ne!(ReuseResult::CannotReuse, ReuseResult::NeedsRebase);
    }

    #[test]
    fn reuse_config_default() {
        let c = ReuseConfig::default();
        assert!(!c.branch_exists);
        assert!(c.branch_name.is_empty());
        assert!(c.base_branch.is_empty());
        assert!(!c.is_behind_base);
        assert!(!c.is_modified);
        assert!(!c.is_conflicted);
    }

    #[test]
    fn should_reuse_branch_not_exists() {
        let config = ReuseConfig {
            branch_exists: false,
            ..Default::default()
        };
        let (result, _) = should_reuse_branch(&config);
        assert_eq!(result, ReuseResult::CannotReuse);
    }

    #[test]
    fn should_reuse_branch_exists_not_behind() {
        let config = ReuseConfig {
            branch_exists: true,
            ..Default::default()
        };
        let (result, modified) = should_reuse_branch(&config);
        assert_eq!(result, ReuseResult::CanReuse);
        assert!(!modified);
    }

    #[test]
    fn should_reuse_branch_behind_modified() {
        let config = ReuseConfig {
            branch_exists: true,
            is_behind_base: true,
            is_modified: true,
            ..Default::default()
        };
        let (result, modified) = should_reuse_branch(&config);
        assert_eq!(result, ReuseResult::CanReuse);
        assert!(modified);
    }

    #[test]
    fn should_reuse_branch_behind_not_modified() {
        let config = ReuseConfig {
            branch_exists: true,
            is_behind_base: true,
            is_modified: false,
            ..Default::default()
        };
        let (result, _) = should_reuse_branch(&config);
        assert_eq!(result, ReuseResult::NeedsRebase);
    }

    #[test]
    fn should_reuse_branch_conflicted_not_modified() {
        let config = ReuseConfig {
            branch_exists: true,
            is_conflicted: true,
            is_modified: false,
            ..Default::default()
        };
        let (result, _) = should_reuse_branch(&config);
        assert_eq!(result, ReuseResult::NeedsRebase);
    }

    #[test]
    fn should_reuse_branch_conflicted_never() {
        let config = ReuseConfig {
            branch_exists: true,
            is_conflicted: true,
            is_modified: false,
            rebase_when: Some("never".into()),
            ..Default::default()
        };
        let (result, _) = should_reuse_branch(&config);
        assert_eq!(result, ReuseResult::CanReuse);
    }

    #[test]
    fn should_reuse_branch_conflicted_modified() {
        let config = ReuseConfig {
            branch_exists: true,
            is_conflicted: true,
            is_modified: true,
            ..Default::default()
        };
        let (result, modified) = should_reuse_branch(&config);
        assert_eq!(result, ReuseResult::CannotReuse);
        assert!(modified);
    }

    #[test]
    fn compare_branch_content_same() {
        assert!(compare_branch_content("hello", "hello"));
    }

    #[test]
    fn compare_branch_content_different() {
        assert!(!compare_branch_content("hello", "world"));
    }

    #[test]
    fn compare_branch_content_empty() {
        assert!(compare_branch_content("", ""));
    }
}
