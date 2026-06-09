//! Rebase onboarding branch (update config file on branch if the computed config hash changed).
//!
//! Mirrors `lib/workers/repository/onboarding/branch/rebase.ts`.
//! @parity `lib/workers/repository/onboarding/branch/rebase.ts` partial — rebaseOnboardingBranch (platform support check for github/gitea/gitlab, get contents and hash, skip if same as previous or dryRun, use OnboardingCommitMessageFactory for message, prTitle, scm.commitAndPush); single test ported. Uses siblings for contents/factory; full scm/platform execution and higher caller (index) pending other units.

use crate::config::GlobalConfig;
use crate::workers::types::RenovateConfig;
use crate::workers::repository::onboarding::branch::commit_message::OnboardingCommitMessageFactory;

/// Mirrors the rebaseOnboardingBranch.
/// In the Rust arch, this prepares the rebase "commit" data (message etc) and returns it if rebase needed, similar to the create stub (the actual push is in higher or the result is used).
/// The hash is simplified for the unit (real would use toSha256 of the contents from the config sibling).
pub fn rebase_onboarding_branch(
    config: &RenovateConfig,
    global_config: &GlobalConfig,
    previous_config_hash: Option<&str>,
) -> Option<String> {
    let platform = global_config.platform.as_deref().unwrap_or("");
    if !["github", "gitea", "gitlab"].contains(&platform) {
        return None;
    }

    let config_file = global_config
        .onboarding_config_file_name
        .as_deref()
        .unwrap_or("renovate.json");

    // contents from sibling (the ported config)
    let contents = crate::workers::repository::onboarding::branch::config::get_onboarding_config_content(
        global_config,
        Some(&config.repository.clone().unwrap_or_default()),
        platform,
    );

    // simple hash for unit (real: toSha256(contents))
    let current_hash = format!("hash-{}", contents.len());

    if previous_config_hash == Some(current_hash.as_str()) {
        return None;
    }

    if global_config.dry_run.is_some() {
        return None;
    }

    let factory = OnboardingCommitMessageFactory::new(config.clone(), config_file.to_string());
    let commit_message = factory.create(global_config);

    let pr_title = if config.semantic_commits.as_deref() == Some("enabled") {
        // getSemanticCommitPrTitle stub
        "chore(deps): Configure Renovate".to_string()
    } else {
        global_config
            .onboarding_pr_title
            .clone()
            .unwrap_or_else(|| "Configure Renovate".to_string())
    };

    // In real, the scm.commitAndPush with the contents for the path, message, prTitle.
    // For the stub (like create), return the message as the "commit" indicator.
    Some(commit_message)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "rebases onboarding branch" — lib/workers/repository/onboarding/branch/rebase.spec.ts line 48
    #[test]
    fn rebases_onboarding_branch() {
        // Exercises the core: previous hash different -> rebase (use factory for message, prTitle).
        // The TS calls scm.commitAndPush; the Rust stub returns the message to indicate rebase happened.
        let config = RenovateConfig::default();
        let mut global = GlobalConfig::default();
        global.platform = Some("github".to_string());
        let result = rebase_onboarding_branch(&config, &global, Some("old-hash"));
        assert!(result.is_some());
        let msg = result.unwrap();
        // the message from factory default or the prTitle in other path
        assert!(!msg.is_empty());
    }
}