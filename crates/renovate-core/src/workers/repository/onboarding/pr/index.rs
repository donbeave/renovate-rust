//! Onboarding PR management.
//!
//! Mirrors `lib/workers/repository/onboarding/pr/index.ts`.
//! @parity `lib/workers/repository/onboarding/pr/index.ts` partial — ensureOnboardingPr (early returns for onboarded/cache/checkbox, title, body with config/base/sections, dry handling); single test ported. Full platform PR create/update, auto-close, full template placeholders, pr-list/errors wiring pending other units. (architectural return of OnboardingPrConfig instead of void side-effect).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::config::GlobalConfig;
use crate::workers::repository::common::PackageFile;
use crate::workers::repository::update::branch::types::BranchConfig;
use crate::workers::types::RenovateConfig;
// wiring for siblings ported in recent cycles (config desc, base branch, state, cache, errors)
use crate::branch::get_base_branch_desc;
use crate::workers::repository::errors_warnings::{
    get_dep_warnings_onboarding_pr, get_errors, get_warnings,
};
use crate::workers::repository::onboarding::branch::onboarding_branch_cache;
use crate::workers::repository::onboarding::common::OnboardingState;
use crate::workers::repository::onboarding::pr::config_description::get_config_desc;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OnboardingPrConfig {
    pub pr_title: String,
    pub pr_body: String,
    pub source_branch: String,
    pub target_branch: String,
    pub labels: Vec<String>,
    pub dry_run: bool,
}

pub fn ensure_onboarding_pr(
    config: &RenovateConfig,
    global_config: &GlobalConfig,
    package_files: &HashMap<String, Vec<PackageFile>>,
    branches: &[BranchConfig],
) -> Option<OnboardingPrConfig> {
    let onboarding_branch = global_config
        .onboarding_branch
        .as_deref()
        .unwrap_or("renovate/configure");

    if config.enabled == Some(false) {
        return None;
    }

    // early returns matching TS ensureOnboardingPr (using OnboardingState from common, repoIsOnboarded)
    if config.repo_is_onboarded == Some(true) {
        return None;
    }
    if config.onboarding_rebase_checkbox.unwrap_or(false) && !OnboardingState::pr_update_requested()
    {
        return None;
    }
    if OnboardingState::onboarding_cache_valid() {
        return None;
    }

    // conflicted example (simplified; full uses await isOnboardingBranchConflicted + ensureComment)
    // if onboarding_branch_cache::is_onboarding_branch_conflicted(...) { return None; }

    let pr_title = if config.semantic_commits.as_deref() == Some("enabled") {
        let commit_type = config.semantic_commit_type.as_deref().unwrap_or("chore");
        let onboarding_title = global_config
            .onboarding_pr_title
            .as_deref()
            .unwrap_or("Configure Renovate");
        format!("{commit_type}: {onboarding_title}")
    } else {
        global_config
            .onboarding_pr_title
            .clone()
            .unwrap_or_else(|| "Configure Renovate".to_owned())
    };

    let pr_body = build_onboarding_pr_body(config, global_config, package_files, branches);

    if global_config.dry_run.is_some() {
        return Some(OnboardingPrConfig {
            pr_title,
            pr_body,
            source_branch: onboarding_branch.to_owned(),
            target_branch: "main".to_owned(),
            labels: Vec::new(),
            dry_run: true,
        });
    }

    Some(OnboardingPrConfig {
        pr_title,
        pr_body,
        source_branch: onboarding_branch.to_owned(),
        target_branch: "main".to_owned(),
        labels: Vec::new(),
        dry_run: false,
    })
}

fn build_onboarding_pr_body(
    config: &RenovateConfig,
    global_config: &GlobalConfig,
    package_files: &HashMap<String, Vec<PackageFile>>,
    branches: &[BranchConfig],
) -> String {
    // enhanced to wire ported siblings (config desc, base branch) and closer to TS template
    // (full placeholders, rebase checkbox, warnings/errors, pr list would use pr_list + errors_warnings siblings)
    let mut body = String::from(
        "Welcome to [Renovate](https://github.com/renovatebot/renovate)! \
        This is an onboarding PR to help you understand and configure settings before regular Pull Requests begin.\n\n",
    );

    // traffic light / require config note (simplified from TS)
    if global_config.require_config == crate::config::RequireConfig::Required {
        body.push_str(":vertical_traffic_light: To activate Renovate, merge this Pull Request. To disable Renovate, simply close this Pull Request unmerged.\n\n");
    } else {
        body.push_str(":vertical_traffic_light: Renovate will begin keeping your dependencies up-to-date only once you merge or close this Pull Request.\n\n");
    }

    body.push_str(":books: See our [Reading List](https://docs.renovatebot.com/reading-list/) for relevant documentation you may be interested in reading.\n\n");

    let config_file = global_config
        .onboarding_config_file_name
        .as_deref()
        .unwrap_or("renovate.json");
    body.push_str(&format!(
        ":abcd: Do you want to change how Renovate upgrades your dependencies? Add your custom config to `{}` in this branch{}.\n\n",
        config_file,
        if config.onboarding_rebase_checkbox.unwrap_or(false) { " and select the Retry/Rebase checkbox below" } else { "" }
    ));

    if !package_files.is_empty() {
        body.push_str("### Detected Package Files\n\n");
        for (manager, files) in package_files {
            for file in files {
                body.push_str(&format!(" * `{}` ({})\n", file.package_file, manager));
            }
        }
        body.push('\n');
    }

    if !branches.is_empty() {
        body.push_str(&format!(
            "### What to Expect\n\nWith your current configuration, Renovate will create {} Pull Request(s).\n\n",
            branches.len()
        ));
        for branch in branches {
            body.push_str(&format!(" - `{}`\n", branch.branch_name));
        }
    }

    // CONFIG section (from recently ported sibling)
    let config_section = get_config_desc(config, None);
    if !config_section.is_empty() {
        body.push_str(&config_section);
    }

    // BASEBRANCH section (from ported in branch.rs)
    let base_patterns: Vec<&str> = config
        .base_branch_patterns
        .as_ref()
        .map_or(vec![], |v| v.iter().map(|s| s.as_str()).collect());
    let base_section = get_base_branch_desc(&base_patterns);
    if !base_section.is_empty() {
        body.push_str(&base_section);
    }

    // TODO for full: append PRLIST (pr_list sibling), WARNINGS/ERRORS/DEP WARNINGS (errors_warnings sibling), rebase checkbox

    let _ = global_config;
    body
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn onboarding_pr_config_default() {
        let c = OnboardingPrConfig::default();
        assert!(c.pr_title.is_empty());
        assert!(c.pr_body.is_empty());
        assert!(c.source_branch.is_empty());
    }

    #[test]
    fn ensure_onboarding_pr_returns_config() {
        let config = RenovateConfig::default();
        let global = GlobalConfig::default();
        let package_files = HashMap::new();
        let branches = vec![];
        let result = ensure_onboarding_pr(&config, &global, &package_files, &branches);
        assert!(result.is_some());
        let pr = result.unwrap();
        assert!(!pr.pr_title.is_empty());
    }

    #[test]
    fn ensure_onboarding_pr_disabled() {
        let config = RenovateConfig {
            enabled: Some(false),
            ..Default::default()
        };
        let global = GlobalConfig::default();
        let result = ensure_onboarding_pr(&config, &global, &HashMap::new(), &[]);
        assert!(result.is_none());
    }

    #[test]
    fn ensure_onboarding_pr_semantic_commits() {
        let config = RenovateConfig {
            semantic_commits: Some("enabled".to_owned()),
            semantic_commit_type: Some("chore".to_owned()),
            ..Default::default()
        };
        let global = GlobalConfig::default();
        let result = ensure_onboarding_pr(&config, &global, &HashMap::new(), &[]);
        let pr = result.unwrap();
        assert!(pr.pr_title.starts_with("chore:"));
    }

    #[test]
    fn ensure_onboarding_pr_dry_run() {
        let config = RenovateConfig::default();
        let global = GlobalConfig {
            dry_run: Some(crate::config::DryRun::Full),
            ..Default::default()
        };
        let result = ensure_onboarding_pr(&config, &global, &HashMap::new(), &[]);
        let pr = result.unwrap();
        assert!(pr.dry_run);
    }

    #[test]
    fn ensure_onboarding_pr_with_package_files() {
        let config = RenovateConfig::default();
        let global = GlobalConfig::default();
        let mut pf = HashMap::new();
        pf.insert(
            "npm".to_owned(),
            vec![PackageFile {
                package_file: "package.json".to_owned(),
                deps: vec![],
                ..Default::default()
            }],
        );
        let result = ensure_onboarding_pr(&config, &global, &pf, &[]);
        let pr = result.unwrap();
        assert!(pr.pr_body.contains("Detected Package Files"));
        assert!(pr.pr_body.contains("package.json"));
    }

    #[test]
    fn onboarding_pr_config_serialization_roundtrip() {
        let c = OnboardingPrConfig {
            pr_title: "Configure Renovate".into(),
            pr_body: "body".into(),
            source_branch: "renovate/configure".into(),
            target_branch: "main".into(),
            labels: vec!["dependencies".into()],
            dry_run: false,
        };
        let json = serde_json::to_string(&c).unwrap();
        let back: OnboardingPrConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(back.pr_title, c.pr_title);
        assert_eq!(back.labels.len(), 1);
    }

    // Ported: "returns if onboarded cache is valid" — lib/workers/repository/onboarding/pr/index.spec.ts line 56
    #[test]
    fn returns_if_onboarded_cache_is_valid() {
        // Exercises the early return for OnboardingState.onboardingCacheValid in ensureOnboardingPr (TS index).
        // Proves the state wiring + cache valid skip path for this unit (matching the it() that sets the state and expects no create/update).
        OnboardingState::set_onboarding_cache_valid(true);
        let config = RenovateConfig::default();
        let global = GlobalConfig::default();
        let result = ensure_onboarding_pr(&config, &global, &HashMap::new(), &[]);
        // in recipe style we return None for the skip case (no PR ensure)
        assert!(result.is_none() || result.as_ref().map_or(true, |p| p.pr_body.is_empty()));
        OnboardingState::set_onboarding_cache_valid(false);
    }
}
