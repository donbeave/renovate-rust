//! Auto-discovery of repositories.
//!
//! Mirrors `lib/workers/global/autodiscover.ts`.
//! @parity lib/workers/global/autodiscover.ts partial — local platform special case + proper regex/glob filter via shared match_regex_or_glob_list (the full platform.getRepos + AutodiscoverConfig building + pre-configured repo merge logic is orchestrated at CLI level in the current Rust architecture).

use serde::{Deserialize, Serialize};

use crate::config::GlobalConfig;
use crate::string_match::match_regex_or_glob_list;

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
    // Mirrors the early local platform special case in TS autodiscoverRepositories.
    // In local mode we expect a single synthetic "local" entry (injected by CLI
    // for platform=local when no explicit repositories given).
    if global_config.platform == crate::config::Platform::Local {
        if !global_config.repositories.is_empty() {
            // TS throws here; we return the list but record an error for visibility.
            return AutodiscoverResult {
                discovered: global_config.repositories.clone(),
                filtered: global_config.repositories.clone(),
                errors: vec![
                    "Invalid configuration: repositories list not supported when platform=local"
                        .to_string(),
                ],
            };
        }
        return AutodiscoverResult {
            discovered: vec!["local".to_string()],
            filtered: vec!["local".to_string()],
            errors: Vec::new(),
        };
    }

    let autodiscover = global_config.autodiscover.unwrap_or(false);

    if !autodiscover {
        return AutodiscoverResult {
            discovered: global_config.repositories.clone(),
            filtered: global_config.repositories.clone(),
            errors: Vec::new(),
        };
    }

    let discovered = global_config.repositories.clone();

    // Use Renovate-compatible regex/glob list matching (case-insensitive globs,
    // negation support) instead of naive substring contains.
    let filtered = if let Some(filter) = &global_config.autodiscover_filter {
        discovered
            .into_iter()
            .filter(|repo| match_regex_or_glob_list(repo, filter))
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

    // The single test added for this cycle's changes to match upstream behavior.
    #[test]
    fn autodiscover_uses_glob_filter_and_handles_local() {
        // Ported: filter application + local platform special case from
        // autodiscoverRepositories + applyFilters in lib/workers/global/autodiscover.ts
        use crate::config::Platform;

        // Glob filter (should match using the shared match_regex_or_glob_list)
        let global = GlobalConfig {
            autodiscover: Some(true),
            repositories: vec![
                "org/repo1".into(),
                "org/sub/repo2".into(),
                "other/repo3".into(),
            ],
            autodiscover_filter: Some(vec!["org/*".into()]),
            ..Default::default()
        };
        let result = autodiscover_repositories(&global);
        assert_eq!(result.filtered.len(), 2);
        assert!(result.filtered.iter().any(|r| r.contains("repo1")));
        assert!(result.filtered.iter().any(|r| r.contains("repo2")));

        // Local platform special case
        let mut local_cfg = GlobalConfig::default();
        local_cfg.platform = Platform::Local;
        local_cfg.repositories = vec![]; // should become ["local"]
        let local_res = autodiscover_repositories(&local_cfg);
        assert_eq!(local_res.discovered, vec!["local".to_string()]);
        assert!(local_res.errors.is_empty());

        // Local with explicit repos should surface error (TS throws)
        let mut local_with_repos = GlobalConfig::default();
        local_with_repos.platform = Platform::Local;
        local_with_repos.repositories = vec!["something".into()];
        let local_bad = autodiscover_repositories(&local_with_repos);
        assert!(!local_bad.errors.is_empty());
    }
}
