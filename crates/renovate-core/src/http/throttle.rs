use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};

use tokio::sync::{Mutex as AsyncMutex, Semaphore};

use super::rate_limits::{
    ConcurrencyLimitRule, ThrottleLimitRule, get_concurrent_requests_limit,
    get_throttle_interval_ms,
};

type SharedSemaphore = Arc<AsyncMutex<Semaphore>>;
type SharedTimestamp = Arc<AsyncMutex<tokio::time::Instant>>;

static HOST_QUEUES: LazyLock<Mutex<HashMap<String, Option<SharedSemaphore>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static HOST_THROTTLES: LazyLock<Mutex<HashMap<String, Option<SharedTimestamp>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static HOST_RULES_CONCURRENCY: LazyLock<Mutex<Vec<ConcurrencyLimitRule>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

static HOST_RULES_THROTTLE: LazyLock<Mutex<Vec<ThrottleLimitRule>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

pub fn set_http_rate_limits(
    concurrency_rules: Vec<ConcurrencyLimitRule>,
    throttle_rules: Vec<ThrottleLimitRule>,
) {
    if let Ok(mut c) = HOST_RULES_CONCURRENCY.lock() {
        *c = concurrency_rules;
    }
    if let Ok(mut t) = HOST_RULES_THROTTLE.lock() {
        *t = throttle_rules;
    }
}

fn get_concurrency_rules() -> Vec<ConcurrencyLimitRule> {
    HOST_RULES_CONCURRENCY
        .lock()
        .map(|r| r.clone())
        .unwrap_or_default()
}

fn get_throttle_rules() -> Vec<ThrottleLimitRule> {
    HOST_RULES_THROTTLE
        .lock()
        .map(|r| r.clone())
        .unwrap_or_default()
}

pub async fn get_queue(url: &str) -> Option<SharedSemaphore> {
    let host = url::Url::parse(url)
        .ok()
        .and_then(|u| u.host_str().map(|h| h.to_owned()))?;

    {
        let queues = HOST_QUEUES.lock().unwrap();
        if let Some(entry) = queues.get(&host) {
            return entry.clone();
        }
    }

    let concurrency_rules = get_concurrency_rules();
    let limit = get_concurrent_requests_limit(url, &concurrency_rules)?;

    let sem = Arc::new(AsyncMutex::new(Semaphore::new(limit)));

    {
        let mut queues = HOST_QUEUES.lock().unwrap();
        queues.insert(host, Some(sem.clone()));
    }

    Some(sem)
}

pub async fn get_throttle(url: &str) -> Option<SharedTimestamp> {
    let host = url::Url::parse(url)
        .ok()
        .and_then(|u| u.host_str().map(|h| h.to_owned()))?;

    {
        let throttles = HOST_THROTTLES.lock().unwrap();
        if let Some(entry) = throttles.get(&host) {
            return entry.clone();
        }
    }

    let throttle_rules = get_throttle_rules();
    let interval_ms = get_throttle_interval_ms(url, &throttle_rules)?;

    let timestamp = Arc::new(AsyncMutex::new(
        tokio::time::Instant::now() - tokio::time::Duration::from_millis(interval_ms),
    ));

    {
        let mut throttles = HOST_THROTTLES.lock().unwrap();
        throttles.insert(host, Some(timestamp.clone()));
    }

    Some(timestamp)
}

pub async fn apply_throttle(url: &str) {
    let throttle_rules = get_throttle_rules();
    let Some(interval_ms) = get_throttle_interval_ms(url, &throttle_rules) else {
        return;
    };

    if let Some(timestamp) = get_throttle(url).await {
        let mut last = timestamp.lock().await;
        let now = tokio::time::Instant::now();
        let elapsed = now.duration_since(*last);
        let required = tokio::time::Duration::from_millis(interval_ms);
        if elapsed < required {
            tokio::time::sleep(required - elapsed).await;
        }
        *last = tokio::time::Instant::now();
    }
}

pub fn clear() {
    if let Ok(mut queues) = HOST_QUEUES.lock() {
        queues.clear();
    }
    if let Ok(mut throttles) = HOST_THROTTLES.lock() {
        throttles.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Rust-specific: throttle behavior test
    #[test]
    fn set_and_get_concurrency_rules() {
        let rules = vec![ConcurrencyLimitRule {
            match_host: "example.com".to_owned(),
            concurrency: 5,
        }];
        set_http_rate_limits(rules, vec![]);
        let retrieved = get_concurrency_rules();
        assert_eq!(retrieved.len(), 1);
        assert_eq!(retrieved[0].concurrency, 5);
    }

    #[tokio::test]
    async fn get_queue_for_known_host() {
        clear();
        set_http_rate_limits(
            vec![ConcurrencyLimitRule {
                match_host: "api.github.com".to_owned(),
                concurrency: 2,
            }],
            vec![],
        );
        let queue = get_queue("https://api.github.com/repos").await;
        assert!(queue.is_some());
    }

    #[tokio::test]
    async fn apply_throttle_no_op_for_unknown() {
        clear();
        apply_throttle("https://unknown-host.example.com/api").await;
    }

    #[test]
    fn set_and_get_throttle_rules() {
        let rules = vec![ThrottleLimitRule {
            match_host: "example.com".to_owned(),
            throttle_ms: 200,
        }];
        set_http_rate_limits(vec![], rules);
        let retrieved = get_throttle_rules();
        assert_eq!(retrieved.len(), 1);
        assert_eq!(retrieved[0].throttle_ms, 200);
        // Restore empty state for other tests
        set_http_rate_limits(vec![], vec![]);
    }

    #[tokio::test]
    async fn get_throttle_for_known_host() {
        clear();
        set_http_rate_limits(
            vec![],
            vec![ThrottleLimitRule {
                match_host: "api.github.com".to_owned(),
                throttle_ms: 100,
            }],
        );
        let throttle = get_throttle("https://api.github.com/repos").await;
        assert!(throttle.is_some());
        // Restore empty state
        set_http_rate_limits(vec![], vec![]);
    }

    #[tokio::test]
    async fn get_throttle_unknown_host() {
        clear();
        let throttle = get_throttle("https://unknown.example.com").await;
        assert!(throttle.is_none());
    }

    #[tokio::test]
    async fn clear_removes_runtime_queues_and_throttles() {
        set_http_rate_limits(
            vec![ConcurrencyLimitRule {
                match_host: "example.com".to_owned(),
                concurrency: 2,
            }],
            vec![ThrottleLimitRule {
                match_host: "example.com".to_owned(),
                throttle_ms: 100,
            }],
        );
        // Prime the queue and throttle
        let _ = get_queue("https://example.com/api").await;
        let _ = get_throttle("https://example.com/api").await;
        clear();
        // After clear, the runtime state is gone but rules remain.
        // The queue should be recreated on next access because rules are still there.
        let queue = get_queue("https://example.com/api").await;
        assert!(queue.is_some());
    }
}
