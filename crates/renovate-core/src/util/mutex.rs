use std::sync::{Arc, Mutex};

pub struct ScopedMutex<T> {
    inner: Arc<Mutex<T>>,
}

impl<T> ScopedMutex<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(value)),
        }
    }

    pub fn lock<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut guard = self.inner.lock().expect("mutex not poisoned");
        f(&mut guard)
    }

    pub fn with_scoped_lock<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        let guard = self.inner.lock().expect("mutex not poisoned");
        f(&guard)
    }
}

impl<T> Clone for ScopedMutex<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for ScopedMutex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScopedMutex").finish()
    }
}

impl<T: Default> Default for ScopedMutex<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scoped_mutex_lock_mutates() {
        let mutex = ScopedMutex::new(0i32);
        mutex.lock(|v| *v = 42);
        assert_eq!(mutex.with_scoped_lock(|v| *v), 42);
    }

    #[test]
    fn scoped_mutex_returns_value() {
        let mutex = ScopedMutex::new("hello".to_owned());
        let val = mutex.with_scoped_lock(|v| v.clone());
        assert_eq!(val, "hello");
    }

    #[test]
    fn scoped_mutex_clone_shares_state() {
        let mutex = ScopedMutex::new(0i32);
        let clone = mutex.clone();
        mutex.lock(|v| *v = 99);
        assert_eq!(clone.with_scoped_lock(|v| *v), 99);
    }

    #[test]
    fn scoped_mutex_default() {
        let mutex: ScopedMutex<i32> = ScopedMutex::default();
        assert_eq!(mutex.with_scoped_lock(|v| *v), 0);
    }
}
