use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct RepositoryCache {
    data: HashMap<String, serde_json::Value>,
    modified: bool,
}

impl RepositoryCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }

    pub fn set(&mut self, key: &str, value: serde_json::Value) {
        self.data.insert(key.to_owned(), value);
        self.modified = true;
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn reset_modified(&mut self) {
        self.modified = false;
    }

    pub fn data(&self) -> &HashMap<String, serde_json::Value> {
        &self.data
    }
}

pub fn get_repo_cache(cache: &RepositoryCache, key: &str) -> Option<serde_json::Value> {
    cache.get(key).cloned()
}

pub fn set_repo_cache(cache: &mut RepositoryCache, key: &str, value: serde_json::Value) {
    cache.set(key, value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_returns_none_when_empty() {
        let cache = RepositoryCache::new();
        assert!(get_repo_cache(&cache, "key").is_none());
    }

    #[test]
    fn set_then_get_returns_value() {
        let mut cache = RepositoryCache::new();
        set_repo_cache(&mut cache, "key", serde_json::json!("value"));
        assert_eq!(
            get_repo_cache(&cache, "key"),
            Some(serde_json::json!("value"))
        );
    }

    #[test]
    fn set_marks_modified() {
        let mut cache = RepositoryCache::new();
        assert!(!cache.is_modified());
        cache.set("key", serde_json::json!("value"));
        assert!(cache.is_modified());
    }

    #[test]
    fn reset_modified_clears_flag() {
        let mut cache = RepositoryCache::new();
        cache.set("key", serde_json::json!("value"));
        cache.reset_modified();
        assert!(!cache.is_modified());
    }

    #[test]
    fn overwrites_previous_value() {
        let mut cache = RepositoryCache::new();
        cache.set("key", serde_json::json!("v1"));
        cache.set("key", serde_json::json!("v2"));
        assert_eq!(cache.get("key"), Some(&serde_json::json!("v2")));
    }
}
