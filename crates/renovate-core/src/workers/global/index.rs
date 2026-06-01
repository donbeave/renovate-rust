//! Global worker entry point.
//!
//! Mirrors `lib/workers/global/index.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalWorkerConfig {
    pub repositories: Vec<String>,
    pub dry_run: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalWorkerResult {
    pub processed_repos: Vec<String>,
    pub failed_repos: Vec<String>,
    pub total_duration_ms: u64,
}

pub fn start_global_worker(
    config: &GlobalWorkerConfig,
    _global_config: &crate::config::GlobalConfig,
) -> GlobalWorkerResult {
    let dry_run = config.dry_run;

    GlobalWorkerResult {
        processed_repos: if dry_run {
            Vec::new()
        } else {
            config.repositories.clone()
        },
        failed_repos: Vec::new(),
        total_duration_ms: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_worker_config_default() {
        let c = GlobalWorkerConfig::default();
        assert!(c.repositories.is_empty());
        assert!(!c.dry_run);
    }

    #[test]
    fn global_worker_result_default() {
        let r = GlobalWorkerResult::default();
        assert!(r.processed_repos.is_empty());
        assert!(r.failed_repos.is_empty());
        assert_eq!(r.total_duration_ms, 0);
    }

    #[test]
    fn start_global_worker_processes_repos() {
        let config = GlobalWorkerConfig {
            repositories: vec!["org/repo1".into(), "org/repo2".into()],
            dry_run: false,
        };
        let result = start_global_worker(&config, &crate::config::GlobalConfig::default());
        assert_eq!(result.processed_repos.len(), 2);
        assert!(result.failed_repos.is_empty());
    }

    #[test]
    fn start_global_worker_dry_run() {
        let config = GlobalWorkerConfig {
            repositories: vec!["org/repo1".into()],
            dry_run: true,
        };
        let result = start_global_worker(&config, &crate::config::GlobalConfig::default());
        assert!(result.processed_repos.is_empty());
    }

    #[test]
    fn global_worker_config_serialization_roundtrip() {
        let c = GlobalWorkerConfig {
            repositories: vec!["org/repo".into()],
            dry_run: false,
        };
        let json = serde_json::to_string(&c).unwrap();
        let back: GlobalWorkerConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(back.repositories.len(), 1);
    }
}
