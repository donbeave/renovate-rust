//! Onboarding common utilities.
//!
//! Mirrors `lib/workers/repository/onboarding/common.ts`.

use crate::config::GlobalConfig;
use crate::workers::types::RenovateConfig;

pub fn get_onboarding_pr_title(config: &RenovateConfig, global_config: &GlobalConfig) -> String {
    if config.semantic_commits.as_deref() == Some("enabled") {
        let commit_type = config
            .semantic_commit_type
            .as_deref()
            .unwrap_or("chore");
        let title = global_config
            .onboarding_pr_title
            .as_deref()
            .unwrap_or("Configure Renovate");
        format!("{commit_type}: {title}")
    } else {
        global_config
            .onboarding_pr_title
            .clone()
            .unwrap_or_else(|| "Configure Renovate".to_owned())
    }
}

pub fn get_onboarding_pr_body(
    _config: &RenovateConfig,
    global_config: &GlobalConfig,
    has_package_files: bool,
) -> String {
    let mut body = String::from(
        "Welcome to [Renovate](https://github.com/renovatebot/renovate)! \
         This is an onboarding PR to help you understand and configure settings \
         before regular Pull Requests begin.\n\n",
    );

    body.push_str(
        "To activate Renovate, merge this Pull Request. \
         To disable Renovate, simply close this Pull Request unmerged.\n\n",
    );

    if has_package_files {
        body.push_str("Detected package files are listed above.\n\n");
    }

    let _ = global_config;
    body
}

pub fn get_default_config_file_name(global_config: &GlobalConfig) -> String {
    let config_file_names = get_config_file_names(global_config);
    if let Some(name) = &global_config.onboarding_config_file_name
        && config_file_names.contains(&name.as_str())
    {
        return name.clone();
    }
    config_file_names.first().unwrap_or(&"renovate.json").to_string()
}

pub fn get_config_file_names(_global_config: &GlobalConfig) -> Vec<&'static str> {
    vec![
        "renovate.json",
        "renovate.json5",
        ".github/renovate.json",
        ".github/renovate.json5",
        ".gitlab/renovate.json",
        ".gitlab/renovate.json5",
        ".renovaterc",
        ".renovaterc.json",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_onboarding_pr_title_default() {
        let config = RenovateConfig::default();
        let global = GlobalConfig::default();
        let title = get_onboarding_pr_title(&config, &global);
        assert_eq!(title, "Configure Renovate");
    }

    #[test]
    fn get_onboarding_pr_title_semantic() {
        let config = RenovateConfig {
            semantic_commits: Some("enabled".to_owned()),
            semantic_commit_type: Some("chore".to_owned()),
            ..Default::default()
        };
        let global = GlobalConfig::default();
        let title = get_onboarding_pr_title(&config, &global);
        assert_eq!(title, "chore: Configure Renovate");
    }

    #[test]
    fn get_onboarding_pr_title_custom() {
        let config = RenovateConfig::default();
        let global = GlobalConfig {
            onboarding_pr_title: Some("Set up Renovate".to_owned()),
            ..Default::default()
        };
        let title = get_onboarding_pr_title(&config, &global);
        assert_eq!(title, "Set up Renovate");
    }

    #[test]
    fn get_onboarding_pr_body_no_files() {
        let config = RenovateConfig::default();
        let global = GlobalConfig::default();
        let body = get_onboarding_pr_body(&config, &global, false);
        assert!(body.contains("[Renovate]"));
        assert!(!body.contains("Detected package files"));
    }

    #[test]
    fn get_onboarding_pr_body_with_files() {
        let config = RenovateConfig::default();
        let global = GlobalConfig::default();
        let body = get_onboarding_pr_body(&config, &global, true);
        assert!(body.contains("Detected package files"));
    }

    #[test]
    fn get_default_config_file_name_default() {
        let global = GlobalConfig::default();
        let name = get_default_config_file_name(&global);
        assert_eq!(name, "renovate.json");
    }

    #[test]
    fn get_default_config_file_name_custom() {
        let global = GlobalConfig {
            onboarding_config_file_name: Some("renovate.json5".to_owned()),
            ..Default::default()
        };
        let name = get_default_config_file_name(&global);
        assert_eq!(name, "renovate.json5");
    }

    #[test]
    fn get_config_file_names_default() {
        let global = GlobalConfig::default();
        let names = get_config_file_names(&global);
        assert!(names.contains(&"renovate.json"));
        assert!(names.contains(&".renovaterc"));
    }
}
