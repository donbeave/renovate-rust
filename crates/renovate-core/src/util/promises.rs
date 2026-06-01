use std::sync::Arc;
use tokio::sync::Semaphore;

pub async fn run_all_concurrent<T, F, Fut>(tasks: Vec<F>, concurrency: usize) -> Vec<T>
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: std::future::Future<Output = T> + Send,
    T: Send + 'static,
{
    let semaphore = Arc::new(Semaphore::new(concurrency.max(1)));
    let mut handles = Vec::with_capacity(tasks.len());

    for task in tasks {
        let permit = semaphore
            .clone()
            .acquire_owned()
            .await
            .expect("semaphore not closed");
        handles.push(tokio::spawn(async move {
            let _permit = permit;
            task().await
        }));
    }

    let mut results = Vec::with_capacity(handles.len());
    for handle in handles {
        results.push(handle.await.expect("task panicked"));
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[tokio::test]
    async fn run_all_concurrent_runs_all() {
        let counter = Arc::new(AtomicU32::new(0));
        let tasks: Vec<_> = (0..10)
            .map(|_| {
                let c = counter.clone();
                move || {
                    let c = c.clone();
                    async move {
                        c.fetch_add(1, Ordering::SeqCst);
                    }
                }
            })
            .collect();
        let results = run_all_concurrent(tasks, 3).await;
        assert_eq!(results.len(), 10);
        assert_eq!(counter.load(Ordering::SeqCst), 10);
    }

    #[tokio::test]
    #[allow(clippy::type_complexity)]
    async fn run_all_concurrent_empty() {
        let tasks: Vec<
            Box<
                dyn FnOnce() -> std::pin::Pin<Box<dyn std::future::Future<Output = i32> + Send>>
                    + Send,
            >,
        > = vec![];
        let results: Vec<i32> = run_all_concurrent(tasks, 1).await;
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn run_all_concurrent_returns_values() {
        let tasks: Vec<_> = (0..5)
            .map(|i| {
                move || {
                    let i = i;
                    async move { i * 2 }
                }
            })
            .collect();
        let results = run_all_concurrent(tasks, 2).await;
        let mut sorted = results;
        sorted.sort();
        assert_eq!(sorted, vec![0, 2, 4, 6, 8]);
    }

    #[tokio::test]
    async fn run_all_concurrent_zero_concurrency_uses_one() {
        let counter = Arc::new(AtomicU32::new(0));
        let tasks: Vec<_> = (0..3)
            .map(|_| {
                let c = counter.clone();
                move || {
                    let c = c.clone();
                    async move {
                        c.fetch_add(1, Ordering::SeqCst);
                    }
                }
            })
            .collect();
        let results = run_all_concurrent(tasks, 0).await;
        assert_eq!(results.len(), 3);
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }
}
