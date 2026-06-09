//! Onboarding branch cache management (set/delete, has changed, is modified/conflicted, config details).
//!
//! Mirrors `lib/workers/repository/onboarding/branch/onboarding-branch-cache.ts`.
//! @parity `lib/workers/repository/onboarding/branch/onboarding-branch-cache.ts` partial — setOnboardingCache, deleteOnboardingCache, hasOnboardingBranchChanged, isOnboardingBranchModified, isOnboardingBranchConflicted, getOnboardingFileNameFromCache/getOnboardingConfigFromCache/setOnboardingConfigDetails; single test ported. Uses repo cache (via static for unit, full util/workers integration pending); callers in index pending other units.

use serde::{Deserialize, Serialize};
use std::sync::{Mutex, OnceLock};

use tracing::debug;

/// Mirrors OnboardingBranchCache from the TS (selected fields for the cache).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingBranchCache {
    pub default_branch_sha: String,
    pub onboarding_branch_sha: String,
    pub is_conflicted: bool,
    pub is_modified: bool,
    pub config_file_name: Option<String>,
    pub config_file_parsed: Option<String>,
}

static ONBOARDING_CACHE: OnceLock<Mutex<Option<OnboardingBranchCache>>> = OnceLock::new();

fn get_onboarding_cache() -> &'static Mutex<Option<OnboardingBranchCache>> {
    ONBOARDING_CACHE.get_or_init(|| Mutex::new(None))
}

pub fn set_onboarding_cache(
    default_branch_sha: &str,
    onboarding_branch_sha: &str,
    is_conflicted: bool,
    is_modified: bool,
) {
    if !( !default_branch_sha.is_empty() && !onboarding_branch_sha.is_empty() ) {
        debug!("Onboarding cache not updated");
        return;
    }

    let cache = get_onboarding_cache();
    let mut lock = cache.lock().unwrap();
    let onboarding_cache = OnboardingBranchCache {
        default_branch_sha: default_branch_sha.to_string(),
        onboarding_branch_sha: onboarding_branch_sha.to_string(),
        is_conflicted,
        is_modified,
        config_file_name: None,
        config_file_parsed: None,
    };
    if lock.is_some() {
        debug!(?onboarding_cache, "Update Onboarding Cache");
    } else {
        debug!(?onboarding_cache, "Create Onboarding Cache");
    }
    *lock = Some(onboarding_cache);
}

pub fn delete_onboarding_cache() {
    let cache = get_onboarding_cache();
    let mut lock = cache.lock().unwrap();
    if lock.is_some() {
        debug!("Delete Onboarding Cache");
        *lock = None;
    }
}

// checks if onboarding branch has been modified since last run
// return true if cache isn't present
pub fn has_onboarding_branch_changed(onboarding_branch: &str) -> bool {
    let cache = get_onboarding_cache();
    let lock = cache.lock().unwrap();
    if let Some(c) = lock.as_ref() {
        // getBranchCommit(onboardingBranch)
        let onboarding_sha = crate::util::git::get_branch_commit(onboarding_branch).unwrap_or_default();
        return onboarding_sha != c.onboarding_branch_sha;
    }
    true
}

// checks if onboarding branch has been modified by user
// once set to true it stays true as we do not rebase onboarding branches anymore (this feature will be added in future though)
pub async fn is_onboarding_branch_modified(
    onboarding_branch: &str,
    default_branch: &str,
) -> bool {
    let cache = get_onboarding_cache();
    let lock = cache.lock().unwrap();
    let onboarding_cache = lock.as_ref();
    // getBranchCommit
    let onboarding_sha = crate::util::git::get_branch_commit(onboarding_branch).unwrap_or_default();
    let mut is_modified = false;

    if let Some(c) = onboarding_cache {
        if onboarding_sha == c.onboarding_branch_sha && !c.is_modified {  // simplified
            return c.is_modified;
        }
    } else {
        // scm.isBranchModified(onboardingBranch, defaultBranch)
        // use the git storage or platform
        is_modified = false; // stub for unit; full in platform/git
    }

    is_modified
}

pub fn get_onboarding_file_name_from_cache() -> Option<String> {
    let cache = get_onboarding_cache();
    let lock = cache.lock().unwrap();
    lock.as_ref().and_then(|c| c.config_file_name.clone())
}

pub fn get_onboarding_config_from_cache() -> Option<String> {
    let cache = get_onboarding_cache();
    let lock = cache.lock().unwrap();
    lock.as_ref().and_then(|c| c.config_file_parsed.clone())
}

pub fn set_onboarding_config_details(config_file_name: &str, config_file_parsed: &str) {
    let cache = get_onboarding_cache();
    let mut lock = cache.lock().unwrap();
    if let Some(c) = lock.as_mut() {
        c.config_file_name = Some(config_file_name.to_string());
        c.config_file_parsed = Some(config_file_parsed.to_string());
    }
}

pub async fn is_onboarding_branch_conflicted(
    default_branch: &str,
    onboarding_branch: &str,
) -> bool {
    let cache = get_onboarding_cache();
    let lock = cache.lock().unwrap();
    let onboarding_cache = lock.as_ref();
    let onboarding_sha = crate::util::git::get_branch_commit(onboarding_branch).unwrap_or_default();
    let default_branch_sha = crate::util::git::get_branch_commit(default_branch).unwrap_or_default();
    let mut is_conflicted = false;

    if let Some(c) = onboarding_cache {
        if default_branch_sha == c.default_branch_sha && onboarding_sha == c.onboarding_branch_sha && !c.is_conflicted {
            return c.is_conflicted;
        }
    } else {
        // scm.isBranchConflicted(defaultBranch, onboardingBranch)
        is_conflicted = false; // stub; full in git/platform
    }

    is_conflicted
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "sets new cache" — lib/workers/repository/onboarding/branch/onboarding-branch-cache.spec.ts line 31
    #[test]
    fn sets_new_cache() {
        // Exercises setOnboardingCache (the core set with shas and flags, matching the TS cache structure).
        set_onboarding_cache("default-sha", "onboarding-sha", false, false);
        let cache = get_onboarding_cache().lock().unwrap();
        let expected = OnboardingBranchCache {
            default_branch_sha: "default-sha".to_string(),
            onboarding_branch_sha: "onboarding-sha".to_string(),
            is_conflicted: false,
            is_modified: false,
            config_file_name: None,
            config_file_parsed: None,
        };
        assert_eq!(cache.as_ref().unwrap(), &expected);
    }

    #[test]
    fn deletes_cache() {
        set_onboarding_cache("default-sha", "onboarding-sha", false, false);
        delete_onboarding_cache();
        let cache = get_onboarding_cache().lock().unwrap();
        assert!(cache.is_none());
    }

    #[test]
    fn returns_true_if_cache_is_absent() {
        // for hasOnboardingBranchChanged
        // clear
        *get_onboarding_cache().lock().unwrap() = None;
        assert!(has_onboarding_branch_changed("configure/renovate"));
    }
}