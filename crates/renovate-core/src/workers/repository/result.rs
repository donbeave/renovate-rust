//! Repository result types.
//!
//! Mirrors `lib/workers/repository/result.ts`.

use serde::{Deserialize, Serialize};

use crate::workers::types::RenovateConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ProcessStatus {
    Disabled,
    Onboarded,
    Activated,
    Onboarding,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum RepositoryResult {
    #[default]
    Done,
    Automerged,
    Error,
    Disabled,
    ConfigValidation,
    ExternalHostError,
    MissingApiCredentials,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcessResult {
    pub result: RepositoryResult,
    pub status: ProcessStatus,
    pub enabled: Option<bool>,
    pub onboarded: Option<bool>,
}

pub fn process_result(
    config: &RenovateConfig,
    result: RepositoryResult,
) -> ProcessResult {
    let (status, enabled, onboarded) = match result {
        RepositoryResult::Disabled | RepositoryResult::Error => {
            (ProcessStatus::Disabled, Some(false), None)
        }
        _ => {
            if config.enabled == Some(false) {
                (ProcessStatus::Disabled, Some(false), None)
            } else {
                (
                    ProcessStatus::Onboarded,
                    Some(true),
                    Some(true),
                )
            }
        }
    };

    ProcessResult {
        result,
        status,
        enabled,
        onboarded,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_status_variants() {
        assert_ne!(ProcessStatus::Disabled, ProcessStatus::Onboarded);
        assert_ne!(ProcessStatus::Activated, ProcessStatus::Unknown);
    }

    #[test]
    fn repository_result_variants() {
        assert_ne!(RepositoryResult::Done, RepositoryResult::Error);
        assert_ne!(RepositoryResult::Automerged, RepositoryResult::Disabled);
    }

    #[test]
    fn process_result_default() {
        let r = ProcessResult::default();
        assert!(r.enabled.is_none());
        assert!(r.onboarded.is_none());
    }

    #[test]
    fn process_result_done() {
        let config = RenovateConfig::default();
        let result = process_result(&config, RepositoryResult::Done);
        assert_eq!(result.result, RepositoryResult::Done);
        assert_eq!(result.status, ProcessStatus::Onboarded);
        assert_eq!(result.enabled, Some(true));
    }

    #[test]
    fn process_result_disabled() {
        let config = RenovateConfig::default();
        let result = process_result(&config, RepositoryResult::Disabled);
        assert_eq!(result.status, ProcessStatus::Disabled);
        assert_eq!(result.enabled, Some(false));
    }

    #[test]
    fn process_result_error() {
        let config = RenovateConfig::default();
        let result = process_result(&config, RepositoryResult::Error);
        assert_eq!(result.status, ProcessStatus::Disabled);
    }

    #[test]
    fn process_result_serialization_roundtrip() {
        let r = ProcessResult {
            result: RepositoryResult::Done,
            status: ProcessStatus::Onboarded,
            enabled: Some(true),
            onboarded: Some(true),
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: ProcessResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.result, RepositoryResult::Done);
    }
}
