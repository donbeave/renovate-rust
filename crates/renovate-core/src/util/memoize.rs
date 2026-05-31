use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

pub struct Memoizer<K, V> {
    cache: RefCell<HashMap<K, V>>,
}

impl<K, V> Memoizer<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn get_or_compute<F>(&self, key: K, compute: F) -> V
    where
        F: FnOnce() -> V,
    {
        if let Some(value) = self.cache.borrow().get(&key) {
            return value.clone();
        }
        let value = compute();
        self.cache.borrow_mut().insert(key, value.clone());
        value
    }

    pub fn clear(&self) {
        self.cache.borrow_mut().clear();
    }
}

impl<K, V> Default for Memoizer<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> std::fmt::Debug for Memoizer<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Memoizer").finish()
    }
}

pub fn memoize<K, V, F>(cache: &Memoizer<K, V>, key: K, f: F) -> V
where
    K: Eq + Hash + Clone,
    V: Clone,
    F: FnOnce() -> V,
{
    cache.get_or_compute(key, f)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[test]
    fn memoizer_returns_computed_value() {
        let memo = Memoizer::<String, i32>::new();
        let result = memo.get_or_compute("key".to_owned(), || 42);
        assert_eq!(result, 42);
    }

    #[test]
    fn memoizer_caches_result() {
        let memo = Memoizer::<String, i32>::new();
        let calls = AtomicU32::new(0);
        memo.get_or_compute("key".to_owned(), || {
            calls.fetch_add(1, Ordering::SeqCst);
            42
        });
        memo.get_or_compute("key".to_owned(), || {
            calls.fetch_add(1, Ordering::SeqCst);
            99
        });
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn memoizer_different_keys() {
        let memo = Memoizer::<String, i32>::new();
        assert_eq!(memo.get_or_compute("a".to_owned(), || 1), 1);
        assert_eq!(memo.get_or_compute("b".to_owned(), || 2), 2);
    }

    #[test]
    fn memoizer_clear() {
        let memo = Memoizer::<String, i32>::new();
        memo.get_or_compute("key".to_owned(), || 42);
        memo.clear();
        assert_eq!(memo.get_or_compute("key".to_owned(), || 99), 99);
    }

    #[test]
    fn memoize_function() {
        let memo = Memoizer::<String, i32>::new();
        let result = memoize(&memo, "key".to_owned(), || 42);
        assert_eq!(result, 42);
    }
}
