use std::collections::HashMap;
use std::time::Duration;

use super::key::generate_cache_key;

#[derive(Debug, Clone)]
pub struct PackageCache {
    store: HashMap<String, (String, Option<std::time::Instant>)>,
    ttl: Option<Duration>,
}

impl PackageCache {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            ttl: None,
        }
    }

    pub fn with_ttl(ttl: Duration) -> Self {
        Self {
            store: HashMap::new(),
            ttl: Some(ttl),
        }
    }

    pub fn get(&mut self, namespace: &str, key: &str) -> Option<String> {
        let cache_key = cache_key(namespace, key);
        let (value, expires_at) = self.store.get(&cache_key)?;
        if let Some(exp) = expires_at
            && std::time::Instant::now() > *exp
        {
            self.store.remove(&cache_key);
            return None;
        }
        Some(value.clone())
    }

    pub fn set(&mut self, namespace: &str, key: &str, value: &str) {
        let cache_key = cache_key(namespace, key);
        let expires_at = self.ttl.map(|ttl| std::time::Instant::now() + ttl);
        self.store.insert(cache_key, (value.to_owned(), expires_at));
    }

    pub fn clear(&mut self) {
        self.store.clear();
    }
}

impl Default for PackageCache {
    fn default() -> Self {
        Self::new()
    }
}

pub fn cache_key(namespace: &str, key: &str) -> String {
    generate_cache_key(namespace, key)
}

pub async fn with_cache<T>(
    cache: &mut PackageCache,
    namespace: &str,
    key: &str,
    f: impl std::future::Future<Output = T>,
) -> T
where
    T: Clone + std::fmt::Debug,
{
    let _ = (cache, namespace, key);
    f.await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_key_combines_namespace_and_key() {
        assert_eq!(cache_key("npm", "lodash"), "npm:lodash");
    }

    #[test]
    fn get_returns_none_when_empty() {
        let mut cache = PackageCache::new();
        assert_eq!(cache.get("ns", "key"), None);
    }

    #[test]
    fn set_then_get_returns_value() {
        let mut cache = PackageCache::new();
        cache.set("ns", "key", "value");
        assert_eq!(cache.get("ns", "key"), Some("value".to_owned()));
    }

    #[test]
    fn set_overwrites_previous() {
        let mut cache = PackageCache::new();
        cache.set("ns", "key", "v1");
        cache.set("ns", "key", "v2");
        assert_eq!(cache.get("ns", "key"), Some("v2".to_owned()));
    }

    #[test]
    fn clear_removes_all() {
        let mut cache = PackageCache::new();
        cache.set("ns", "key", "value");
        cache.clear();
        assert_eq!(cache.get("ns", "key"), None);
    }

    #[test]
    fn different_namespaces_dont_collide() {
        let mut cache = PackageCache::new();
        cache.set("ns1", "key", "v1");
        cache.set("ns2", "key", "v2");
        assert_eq!(cache.get("ns1", "key"), Some("v1".to_owned()));
        assert_eq!(cache.get("ns2", "key"), Some("v2".to_owned()));
    }
}
