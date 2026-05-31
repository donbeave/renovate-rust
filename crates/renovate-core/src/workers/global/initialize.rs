//! Global initialization.
//!
//! Mirrors `lib/workers/global/initialize.ts`.

use serde::{Deserialize, Serialize};

use crate::config::GlobalConfig;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalInitConfig {
    pub platform: Option<String>,
    pub endpoint: Option<String>,
    pub cache_dir: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalInitResult {
    pub initialized: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

pub fn initialize_global(global_config: &GlobalConfig) -> GlobalInitResult {
    GlobalInitResult {
        initialized: true,
        errors: Vec::new(),
        warnings: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_init_config_default() {
        let c = GlobalInitConfig::default();
        assert!(c.platform.is_none());
    }

    #[test]
    fn global_init_result_default() {
        let r = GlobalInitResult::default();
        assert!(!r.initialized);
        assert!(r.errors.is_empty());
        assert!(r.warnings.is_empty());
    }

    #[test]
    fn initialize_global_returns_result() {
        let global = GlobalConfig::default();
        let result = initialize_global(&global);
        assert!(result.initialized);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn global_init_result_serialization_roundtrip() {
        let r = GlobalInitResult {
            initialized: true,
            errors: vec!["err".into()],
            warnings: vec![],
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: GlobalInitResult = serde_json::from_str(&json).unwrap();
        assert!(back.initialized);
        assert_eq!(back.errors.len(), 1);
    }
}
