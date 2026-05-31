//! Main repository worker.
//!
//! Mirrors `lib/workers/repository/index.ts`.

use serde::{Deserialize, Serialize};

use crate::config::GlobalConfig;
use crate::workers::repository::result::{ProcessResult, ProcessStatus, RepositoryResult, process_result};
use crate::workers::types::RenovateConfig;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RepositoryWorker {
    pub repository: String,
}

impl RepositoryWorker {
    pub fn new(repository: &str) -> Self {
        Self {
            repository: repository.to_owned(),
        }
    }

    pub fn process(&self, config: &RenovateConfig, global_config: &GlobalConfig) -> ProcessResult {
        process_repository(config, global_config)
    }
}

pub fn process_repository(
    config: &RenovateConfig,
    _global_config: &GlobalConfig,
) -> ProcessResult {
    if config.enabled == Some(false) {
        return ProcessResult {
            result: RepositoryResult::Disabled,
            status: ProcessStatus::Disabled,
            enabled: Some(false),
            onboarded: None,
        };
    }

    process_result(config, RepositoryResult::Done)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repository_worker_default() {
        let w = RepositoryWorker::default();
        assert!(w.repository.is_empty());
    }

    #[test]
    fn repository_worker_new() {
        let w = RepositoryWorker::new("org/repo");
        assert_eq!(w.repository, "org/repo");
    }

    #[test]
    fn repository_worker_process() {
        let w = RepositoryWorker::new("org/repo");
        let result = w.process(&RenovateConfig::default(), &GlobalConfig::default());
        assert_eq!(result.result, RepositoryResult::Done);
    }

    #[test]
    fn process_repository_disabled() {
        let config = RenovateConfig {
            enabled: Some(false),
            ..Default::default()
        };
        let result = process_repository(&config, &GlobalConfig::default());
        assert_eq!(result.result, RepositoryResult::Disabled);
        assert_eq!(result.status, ProcessStatus::Disabled);
    }

    #[test]
    fn process_repository_enabled() {
        let config = RenovateConfig::default();
        let result = process_repository(&config, &GlobalConfig::default());
        assert_eq!(result.result, RepositoryResult::Done);
        assert_eq!(result.enabled, Some(true));
    }

    #[test]
    fn repository_worker_serialization_roundtrip() {
        let w = RepositoryWorker {
            repository: "org/repo".into(),
        };
        let json = serde_json::to_string(&w).unwrap();
        let back: RepositoryWorker = serde_json::from_str(&json).unwrap();
        assert_eq!(back.repository, "org/repo");
    }
}
