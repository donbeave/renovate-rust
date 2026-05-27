use std::collections::HashMap;

use serde_json::Value;

/// In-memory key-value cache for a single run.
///
/// Mirrors `lib/util/cache/memory/index.ts`. The cache must be
/// explicitly initialized with `init()` before use; `get` returns
/// `None` while uninitialized. Call `reset()` to destroy all entries.
#[derive(Debug, Default)]
pub struct MemCache {
    data: Option<HashMap<String, Value>>,
}

impl MemCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init(&mut self) {
        self.data = Some(HashMap::new());
    }

    pub fn reset(&mut self) {
        self.data = None;
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.as_ref()?.get(key)
    }

    pub fn set(&mut self, key: &str, value: Value) {
        if let Some(map) = &mut self.data {
            map.insert(key.to_string(), value);
        }
    }

    /// Removes all keys that start with `datasource-mem:pkg-fetch:` or
    /// `datasource-mem:releases:`.
    pub fn clean_datasource_keys(&mut self) {
        if let Some(map) = &mut self.data {
            map.retain(|k, _| {
                !k.starts_with("datasource-mem:pkg-fetch:")
                    && !k.starts_with("datasource-mem:releases:")
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "returns undefined if not init" — util/cache/memory/index.spec.ts line 3
    #[test]
    fn mem_cache_returns_none_when_not_initialized() {
        let cache = MemCache::new();
        assert_eq!(cache.get("key1"), None);
    }

    // Ported: "sets and gets repo cache" — util/cache/memory/index.spec.ts line 7
    #[test]
    fn mem_cache_sets_and_gets_value() {
        let mut cache = MemCache::new();
        cache.init();
        cache.set("key2", Value::String("value".to_string()));
        assert_eq!(cache.get("key2"), Some(&Value::String("value".to_string())));
    }

    // Ported: "resets" — util/cache/memory/index.spec.ts line 13
    #[test]
    fn mem_cache_reset_clears_values() {
        let mut cache = MemCache::new();
        cache.init();
        cache.set("key3", Value::String("value".to_string()));
        cache.reset();
        assert_eq!(cache.get("key3"), None);
    }

    // Ported: "does nothing if no matching keys exist" — util/cache/memory/index.spec.ts line 21
    #[test]
    fn clean_datasource_keys_noop_for_non_matching() {
        let mut cache = MemCache::new();
        cache.init();
        cache.set("normal-key", Value::String("value".to_string()));
        cache.set("another-key", Value::String("data".to_string()));
        cache.clean_datasource_keys();
        assert_eq!(
            cache.get("normal-key"),
            Some(&Value::String("value".to_string()))
        );
        assert_eq!(
            cache.get("another-key"),
            Some(&Value::String("data".to_string()))
        );
    }

    // Ported: "removes keys that start with datasource-mem:pkg-fetch:" — util/cache/memory/index.spec.ts line 28
    #[test]
    fn clean_datasource_keys_removes_pkg_fetch_prefix() {
        let mut cache = MemCache::new();
        cache.init();
        cache.set(
            "datasource-mem:pkg-fetch:test",
            Value::String("value".to_string()),
        );
        cache.set("normal-key", Value::String("data".to_string()));
        cache.clean_datasource_keys();
        assert_eq!(cache.get("datasource-mem:pkg-fetch:test"), None);
        assert_eq!(
            cache.get("normal-key"),
            Some(&Value::String("data".to_string()))
        );
    }

    // Ported: "removes keys that start with datasource-releases" — util/cache/memory/index.spec.ts line 36
    #[test]
    fn clean_datasource_keys_removes_releases_prefix() {
        let mut cache = MemCache::new();
        cache.init();
        cache.set(
            "datasource-mem:releases:npm",
            Value::String("value".to_string()),
        );
        cache.set("normal-key", Value::String("data".to_string()));
        cache.clean_datasource_keys();
        assert_eq!(cache.get("datasource-mem:releases:npm"), None);
        assert_eq!(
            cache.get("normal-key"),
            Some(&Value::String("data".to_string()))
        );
    }

    // Ported: "removes all matching keys while keeping others" — util/cache/memory/index.spec.ts line 44
    #[test]
    fn clean_datasource_keys_removes_all_matching_keeps_others() {
        let mut cache = MemCache::new();
        cache.init();
        cache.set(
            "datasource-mem:pkg-fetch:test1",
            Value::String("value1".to_string()),
        );
        cache.set(
            "datasource-mem:pkg-fetch:test2",
            Value::String("value2".to_string()),
        );
        cache.set(
            "datasource-mem:pkg-fetch:npm",
            Value::String("npm-data".to_string()),
        );
        cache.set(
            "datasource-mem:pkg-fetch:docker",
            Value::String("docker-data".to_string()),
        );
        cache.set("normal-key1", Value::String("normal1".to_string()));
        cache.set("normal-key2", Value::String("normal2".to_string()));
        cache.clean_datasource_keys();
        assert_eq!(cache.get("datasource-mem:pkg-fetch:test1"), None);
        assert_eq!(cache.get("datasource-mem:pkg-fetch:test2"), None);
        assert_eq!(cache.get("datasource-mem:pkg-fetch:npm"), None);
        assert_eq!(cache.get("datasource-mem:pkg-fetch:docker"), None);
        assert_eq!(
            cache.get("normal-key1"),
            Some(&Value::String("normal1".to_string()))
        );
        assert_eq!(
            cache.get("normal-key2"),
            Some(&Value::String("normal2".to_string()))
        );
    }
}
