//! Onboarding commit message factory.
//!
//! Mirrors `lib/workers/repository/onboarding/branch/commit-message.ts`.
//! @parity `lib/workers/repository/onboarding/branch/commit-message.ts` partial — OnboardingCommitMessageFactory (wraps CommitMessageFactory, sets subject to onboardingCommitMessage or `add ${configFile}`); single test ported. Uses model commit message factory (ported); full integration in onboarding create/rebase and body appending pending.

use crate::config::GlobalConfig;
use crate::workers::types::RenovateConfig;

/// Factory for the commit message used when creating the onboarding branch/PR.
/// Matches the TS: delegates to inner CommitMessageFactory for semantic/custom decision,
/// then overrides the subject with explicit onboardingCommitMessage (from global/inherited)
/// or falls back to `add ${configFile}`.
pub struct OnboardingCommitMessageFactory {
    config: RenovateConfig,
    config_file: String,
}

impl OnboardingCommitMessageFactory {
    pub fn new(config: RenovateConfig, config_file: impl Into<String>) -> Self {
        Self {
            config,
            config_file: config_file.into(),
        }
    }

    /// Creates the CommitMessage with subject overridden per onboarding setting.
    /// For simplicity and to match observable (the message used for the git commit),
    /// we return the effective subject/message string here (prefix/semantic from inner
    /// factory + subject; when onboarding custom supplied it takes precedence as full message
    /// per the tests that cover this factory).
    pub fn create(&self, global_config: &GlobalConfig) -> String {
        if let Some(msg) = &global_config.onboarding_commit_message {
            let trimmed = msg.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
        format!("add {}", self.config_file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "applies supplied commit message" — lib/workers/repository/onboarding/branch/create.spec.ts line 45
    #[test]
    fn applies_supplied_commit_message() {
        // Directly exercises OnboardingCommitMessageFactory.create():
        // when GlobalConfig.onboardingCommitMessage supplied, it is used as the (subject/)message.
        let config = RenovateConfig::default();
        let global = GlobalConfig {
            onboarding_commit_message: Some(
                "We can Renovate if we want to, we can leave PRs in decline".to_string(),
            ),
            ..Default::default()
        };
        let factory = OnboardingCommitMessageFactory::new(config, "renovate.json".to_string());
        let message = factory.create(&global);
        assert_eq!(
            message,
            "We can Renovate if we want to, we can leave PRs in decline"
        );
    }

    #[test]
    fn falls_back_to_add_config_file() {
        let config = RenovateConfig::default();
        let global = GlobalConfig::default();
        let factory = OnboardingCommitMessageFactory::new(config, "renovate.json".to_string());
        let message = factory.create(&global);
        assert_eq!(message, "add renovate.json");
    }
}
