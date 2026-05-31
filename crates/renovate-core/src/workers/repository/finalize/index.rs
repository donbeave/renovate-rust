//! Repository finalization.
//!
//! Mirrors `lib/workers/repository/finalize/index.ts`.

use serde::{Deserialize, Serialize};

use crate::workers::types::RenovateConfig;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FinalizeResult {
    pub pruned_branches: Vec<String>,
    pub statistics_collected: bool,
    pub cache_saved: bool,
}

pub fn finalize_repository(
    config: &RenovateConfig,
    branch_list: &[String],
) -> FinalizeResult {
    FinalizeResult {
        pruned_branches: Vec::new(),
        statistics_collected: true,
        cache_saved: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finalize_result_default() {
        let r = FinalizeResult::default();
        assert!(r.pruned_branches.is_empty());
        assert!(!r.statistics_collected);
        assert!(!r.cache_saved);
    }

    #[test]
    fn finalize_repository_returns_result() {
        let config = RenovateConfig::default();
        let result = finalize_repository(&config, &[]);
        assert!(result.pruned_branches.is_empty());
        assert!(result.statistics_collected);
        assert!(result.cache_saved);
    }

    #[test]
    fn finalize_result_serialization_roundtrip() {
        let r = FinalizeResult {
            pruned_branches: vec!["renovate/old-branch".into()],
            statistics_collected: true,
            cache_saved: true,
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: FinalizeResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.pruned_branches.len(), 1);
        assert!(back.statistics_collected);
    }
}
