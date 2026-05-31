//! Main repository worker.
//!
//! Mirrors `lib/workers/repository/index.ts`.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::config::GlobalConfig;
use crate::workers::repository::common::PackageFile;
use crate::workers::repository::result::{ProcessResult, ProcessStatus, RepositoryResult, process_result};
use crate::workers::types::RenovateConfig;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RepositoryWorker {
    pub repository: String,
    pub config: RenovateConfig,
    pub global_config: GlobalConfig,
}

impl RepositoryWorker {
    pub fn new(repository: &str, config: RenovateConfig, global_config: GlobalConfig) -> Self {
        Self {
            repository: repository.to_owned(),
            config,
            global_config,
        }
    }

    pub fn process(&self) -> ProcessResult {
        process_repository(&self.config, &self.global_config)
    }
}

pub fn process_repository(
    config: &RenovateConfig,
    global_config: &GlobalConfig,
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
        let w = RepositoryWorker::new(
            "org/repo",
            RenovateConfig::default(),
            GlobalConfig::default(),
        );
        assert_eq!(w.repository, "org/repo");
    }

    #[test]
    fn repository_worker_process() {
        let w = RepositoryWorker::new(
            "org/repo",
            RenovateConfig::default(),
            GlobalConfig::default(),
        );
        let result = w.process();
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
            config: RenovateConfig::default(),
            global_config: GlobalConfig::default(),
        };
        let json = serde_json::to_string(&w).unwrap();
        let back: RepositoryWorker = serde_json::from_str(&json).unwrap();
        assert_eq!(back.repository, "org/repo");
    }
}
