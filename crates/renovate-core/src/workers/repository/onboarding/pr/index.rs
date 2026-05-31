//! Onboarding PR management.
//!
//! Mirrors `lib/workers/repository/onboarding/pr/index.ts`.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::config::GlobalConfig;
use crate::workers::repository::common::PackageFile;
use crate::workers::repository::update::branch::types::BranchConfig;
use crate::workers::types::RenovateConfig;

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
    _config: &RenovateConfig,
    global_config: &GlobalConfig,
    package_files: &HashMap<String, Vec<PackageFile>>,
    branches: &[BranchConfig],
) -> String {
    let mut body = String::from("Welcome to [Renovate](https://github.com/renovatebot/renovate)! \
        This is an onboarding PR to help you understand and configure settings before regular Pull Requests begin.\n\n");

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
        assert!(c.dry_run);
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
}
