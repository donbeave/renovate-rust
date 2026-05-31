//! Cache initialization.
//!
//! Mirrors `lib/workers/repository/init/cache.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheInitResult {
    pub cache_dir: Option<String>,
    pub initialized: bool,
}

pub fn init_cache(cache_dir: Option<&str>) -> CacheInitResult {
    CacheInitResult {
        cache_dir: cache_dir.map(|s| s.to_owned()),
        initialized: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_init_result_default() {
        let r = CacheInitResult::default();
        assert!(r.cache_dir.is_none());
        assert!(!r.initialized);
    }

    #[test]
    fn init_cache_with_dir() {
        let result = init_cache(Some("/tmp/cache"));
        assert_eq!(result.cache_dir, Some("/tmp/cache".to_owned()));
        assert!(result.initialized);
    }

    #[test]
    fn init_cache_without_dir() {
        let result = init_cache(None);
        assert!(result.cache_dir.is_none());
        assert!(result.initialized);
    }

    #[test]
    fn cache_init_result_serialization_roundtrip() {
        let r = CacheInitResult {
            cache_dir: Some("/tmp/cache".into()),
            initialized: true,
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: CacheInitResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.cache_dir, Some("/tmp/cache".into()));
    }
}
