//! PR caching.
//!
//! Mirrors `lib/workers/repository/update/pr/pr-cache.ts`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrCache {
    pub body_fingerprint: Option<String>,
    pub last_edited: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct PrCacheStore {
    caches: HashMap<String, PrCache>,
}

impl PrCacheStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_cached_pr(&self, branch_name: &str) -> Option<&PrCache> {
        self.caches.get(branch_name)
    }

    pub fn set_cached_pr(
        &mut self,
        branch_name: &str,
        body_fingerprint: String,
        pr_modified: bool,
    ) {
        let cache = self.caches.entry(branch_name.to_owned()).or_default();

        let last_edited = if pr_modified || cache.last_edited.is_none() {
            Some(
                chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            )
        } else {
            cache.last_edited.clone()
        };

        cache.body_fingerprint = Some(body_fingerprint);
        cache.last_edited = last_edited;
    }

    pub fn invalidate(&mut self, branch_name: &str) {
        self.caches.remove(branch_name);
    }

    pub fn is_empty(&self) -> bool {
        self.caches.is_empty()
    }
}

pub fn get_cached_pr<'a>(store: &'a PrCacheStore, branch_name: &str) -> Option<&'a PrCache> {
    store.get_cached_pr(branch_name)
}

pub fn set_cached_pr(
    store: &mut PrCacheStore,
    branch_name: &str,
    body_fingerprint: String,
    pr_modified: bool,
) {
    store.set_cached_pr(branch_name, body_fingerprint, pr_modified);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pr_cache_default() {
        let cache = PrCache::default();
        assert!(cache.body_fingerprint.is_none());
        assert!(cache.last_edited.is_none());
    }

    #[test]
    fn pr_cache_store_default() {
        let store = PrCacheStore::default();
        assert!(store.is_empty());
    }

    #[test]
    fn pr_cache_store_new() {
        let store = PrCacheStore::new();
        assert!(store.is_empty());
    }

    #[test]
    fn set_and_get_cached_pr() {
        let mut store = PrCacheStore::new();
        set_cached_pr(&mut store, "renovate/lodash-4.x", "fp123".to_owned(), true);

        let cache = get_cached_pr(&store, "renovate/lodash-4.x").unwrap();
        assert_eq!(cache.body_fingerprint, Some("fp123".to_owned()));
        assert!(cache.last_edited.is_some());
    }

    #[test]
    fn get_cached_pr_not_found() {
        let store = PrCacheStore::new();
        assert!(get_cached_pr(&store, "nonexistent").is_none());
    }

    #[test]
    fn set_cached_pr_update_preserves_timestamp() {
        let mut store = PrCacheStore::new();
        set_cached_pr(&mut store, "branch", "fp1".to_owned(), true);
        let first_edited = store.get_cached_pr("branch").unwrap().last_edited.clone();

        set_cached_pr(&mut store, "branch", "fp2".to_owned(), false);
        let cache = store.get_cached_pr("branch").unwrap();
        assert_eq!(cache.body_fingerprint, Some("fp2".to_owned()));
        assert_eq!(cache.last_edited, first_edited);
    }

    #[test]
    fn set_cached_pr_update_modified_updates_timestamp() {
        let mut store = PrCacheStore::new();
        set_cached_pr(&mut store, "branch", "fp1".to_owned(), true);

        set_cached_pr(&mut store, "branch", "fp2".to_owned(), true);
        let cache = store.get_cached_pr("branch").unwrap();
        assert_eq!(cache.body_fingerprint, Some("fp2".to_owned()));
    }

    #[test]
    fn invalidate_cached_pr() {
        let mut store = PrCacheStore::new();
        set_cached_pr(&mut store, "branch", "fp".to_owned(), true);
        assert!(store.get_cached_pr("branch").is_some());

        store.invalidate("branch");
        assert!(store.get_cached_pr("branch").is_none());
    }

    #[test]
    fn pr_cache_serialization() {
        let cache = PrCache {
            body_fingerprint: Some("fp123".into()),
            last_edited: Some("2024-01-01T00:00:00Z".into()),
        };
        let json = serde_json::to_string(&cache).unwrap();
        let back: PrCache = serde_json::from_str(&json).unwrap();
        assert_eq!(back.body_fingerprint, Some("fp123".into()));
        assert_eq!(back.last_edited, Some("2024-01-01T00:00:00Z".into()));
    }

    #[test]
    fn multiple_branches() {
        let mut store = PrCacheStore::new();
        set_cached_pr(&mut store, "branch-a", "fp-a".to_owned(), true);
        set_cached_pr(&mut store, "branch-b", "fp-b".to_owned(), true);

        assert_eq!(
            store.get_cached_pr("branch-a").unwrap().body_fingerprint,
            Some("fp-a".into())
        );
        assert_eq!(
            store.get_cached_pr("branch-b").unwrap().body_fingerprint,
            Some("fp-b".into())
        );
    }
}
