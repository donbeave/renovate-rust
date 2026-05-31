//! Auto-discovery of repositories.
//!
//! Mirrors `lib/workers/global/autodiscover.ts`.

use serde::{Deserialize, Serialize};

use crate::config::GlobalConfig;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutodiscoverConfig {
    pub autodiscover: bool,
    pub filter: Vec<String>,
    pub namespaces: Vec<String>,
    pub projects: Vec<String>,
    pub topics: Vec<String>,
    pub repo_sort: Option<String>,
    pub repo_order: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutodiscoverResult {
    pub discovered: Vec<String>,
    pub filtered: Vec<String>,
    pub errors: Vec<String>,
}

pub fn autodiscover_repositories(global_config: &GlobalConfig) -> AutodiscoverResult {
    let autodiscover = global_config.autodiscover.unwrap_or(false);

    if !autodiscover {
        return AutodiscoverResult {
            discovered: global_config.repositories.clone(),
            filtered: global_config.repositories.clone(),
            errors: Vec::new(),
        };
    }

    let discovered = global_config.repositories.clone();

    let filtered = if let Some(filter) = &global_config.autodiscover_filter {
        discovered
            .into_iter()
            .filter(|repo| filter.iter().any(|f| repo.contains(f)))
            .collect()
    } else {
        discovered
    };

    AutodiscoverResult {
        discovered: global_config.repositories.clone(),
        filtered,
        errors: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn autodiscover_config_default() {
        let c = AutodiscoverConfig::default();
        assert!(!c.autodiscover);
        assert!(c.filter.is_empty());
    }

    #[test]
    fn autodiscover_result_default() {
        let r = AutodiscoverResult::default();
        assert!(r.discovered.is_empty());
        assert!(r.filtered.is_empty());
        assert!(r.errors.is_empty());
    }

    #[test]
    fn autodiscover_repositories_disabled() {
        let global = GlobalConfig::default();
        let result = autodiscover_repositories(&global);
        assert!(result.discovered.is_empty());
    }

    #[test]
    fn autodiscover_repositories_with_repos() {
        let global = GlobalConfig {
            repositories: vec!["org/repo1".into(), "org/repo2".into()],
            ..Default::default()
        };
        let result = autodiscover_repositories(&global);
        assert_eq!(result.filtered.len(), 2);
    }

    #[test]
    fn autodiscover_repositories_with_filter() {
        let global = GlobalConfig {
            autodiscover: Some(true),
            repositories: vec!["org/repo1".into(), "org/repo2".into(), "other/repo3".into()],
            autodiscover_filter: Some(vec!["org/".into()]),
            ..Default::default()
        };
        let result = autodiscover_repositories(&global);
        assert_eq!(result.filtered.len(), 2);
        assert!(result.filtered.contains(&"org/repo1".to_owned()));
    }

    #[test]
    fn autodiscover_result_serialization_roundtrip() {
        let r = AutodiscoverResult {
            discovered: vec!["org/repo".into()],
            filtered: vec!["org/repo".into()],
            errors: vec![],
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: AutodiscoverResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.discovered.len(), 1);
    }
}
