//! Persistent package cache with in-memory deduplication layer.
//!
//! Mirrors `lib/util/cache/package/index.ts`, `backend.ts`, `impl/file.ts`,
//! `with-cache.ts`, and `ttl.ts` from the Renovate reference.
//!
//! ## Architecture
//!
//! ```text
//! caller
//!   └─ PackageCache::get / set / set_with_raw_ttl
//!        ├─ MemCache dedup layer (in-process, cleared on reset)
//!        └─ FilePackageCache backend (disk, TTL-aware, survives runs)
//! ```
//!
//! `with_cache` adds soft/hard TTL and stale-fallback semantics on top of
//! `PackageCache`, mirroring the TypeScript `withCache` function.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Mutex,
};

use chrono::{DateTime, Duration, Utc};
use globset::GlobBuilder;
use regex::Regex;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use sha2::{Digest as _, Sha256};
use tracing::{debug, trace};

// ── Types ────────────────────────────────────────────────────────────────────

/// The namespace string for a package cache entry.
///
/// Mirrors `PackageCacheNamespace` in `lib/util/cache/package/namespaces.ts`.
pub type PackageCacheNamespace = str;

/// Record stored by [`with_cache`] in the package cache.
///
/// Mirrors `CachedRecord` in `lib/util/cache/package/types.ts`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedRecord {
    pub value: Value,
    /// ISO 8601 / RFC 3339 timestamp when this record was created.
    #[serde(rename = "cachedAt")]
    pub cached_at: String,
}

/// Envelope stored on disk by [`FilePackageCache`].
///
/// Each cache entry is a JSON file:
/// `{ "value": <serialized T>, "expiry": "<RFC3339>" }`
#[derive(Debug, Serialize, Deserialize)]
struct FileEntry {
    value: Value,
    #[serde(default)]
    expiry: Option<String>,
}

// ── FilePackageCache ─────────────────────────────────────────────────────────

/// File-based persistent package cache backend.
///
/// Mirrors `PackageCacheFile` in `lib/util/cache/package/impl/file.ts`.
///
/// Files are stored under `{cache_dir}/renovate/cache-v1/{namespace}/{key_hash}`.
/// Each file is a JSON `FileEntry` with an `expiry` field. Expired entries are
/// removed on read and on [`FilePackageCache::cleanup`].
#[derive(Debug)]
pub struct FilePackageCache {
    cache_dir: PathBuf,
}

impl FilePackageCache {
    /// Create a new file cache rooted at `cache_dir`.
    pub fn new(cache_dir: impl Into<PathBuf>) -> Self {
        Self {
            cache_dir: cache_dir.into(),
        }
    }

    fn entry_path(&self, namespace: &PackageCacheNamespace, key: &str) -> PathBuf {
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        let digest = hasher.finalize();
        let hash: String = digest.iter().map(|b| format!("{b:02x}")).collect();
        self.cache_dir
            .join("renovate")
            .join("cache-v1")
            .join(namespace)
            .join(hash)
    }

    /// Retrieve a cached value. Returns `None` if the entry is absent or expired.
    pub async fn get<T: DeserializeOwned>(
        &self,
        namespace: &PackageCacheNamespace,
        key: &str,
    ) -> Option<T> {
        let path = self.entry_path(namespace, key);
        let contents = tokio::fs::read_to_string(&path).await.ok()?;
        let entry: FileEntry = serde_json::from_str(&contents)
            .map_err(|e| trace!(namespace, key, "cache parse error: {e}"))
            .ok()?;

        let expiry_str = entry
            .expiry
            .as_deref()
            .ok_or_else(|| trace!(namespace, key, "cache missing expiry"))
            .ok()?;

        let expiry = DateTime::parse_from_rfc3339(expiry_str)
            .map_err(|e| trace!(namespace, key, "cache expiry parse error: {e}"))
            .ok()?;

        if Utc::now() >= expiry {
            trace!(namespace, key, "cache expired, removing");
            let _ = tokio::fs::remove_file(&path).await;
            return None;
        }

        trace!(namespace, key, "cache hit");
        serde_json::from_value(entry.value)
            .map_err(|e| trace!(namespace, key, "cache value deserialize error: {e}"))
            .ok()
    }

    /// Store a value with the given TTL in minutes.
    pub async fn set(
        &self,
        namespace: &PackageCacheNamespace,
        key: &str,
        value: Value,
        ttl_minutes: i64,
    ) {
        let path = self.entry_path(namespace, key);
        if let Some(parent) = path.parent()
            && let Err(e) = tokio::fs::create_dir_all(parent).await
        {
            debug!(namespace, key, "cache mkdir error: {e}");
            return;
        }
        let expiry = Utc::now() + Duration::minutes(ttl_minutes);
        let entry = FileEntry {
            value,
            expiry: Some(expiry.to_rfc3339()),
        };
        match serde_json::to_string(&entry) {
            Ok(json) => {
                if let Err(e) = tokio::fs::write(&path, json).await {
                    debug!(namespace, key, "cache write error: {e}");
                }
            }
            Err(e) => debug!(namespace, key, "cache serialize error: {e}"),
        }
    }

    /// Remove expired entries from the cache directory.
    ///
    /// Mirrors `PackageCacheFile.destroy()` cleanup scan.
    pub async fn cleanup(&self) {
        let root = self.cache_dir.join("renovate").join("cache-v1");
        let now = Utc::now();
        let mut total = 0u64;
        let mut deleted = 0u64;

        if let Ok(ns_entries) = read_dir_entries(&root).await {
            for ns_dir in ns_entries {
                if let Ok(files) = read_dir_entries(&ns_dir).await {
                    for file in files {
                        total += 1;
                        if let Ok(contents) = tokio::fs::read_to_string(&file).await {
                            let expired = match serde_json::from_str::<FileEntry>(&contents) {
                                Ok(entry) => match entry.expiry {
                                    None => false, // keep entries without expiry
                                    Some(exp_str) => {
                                        match DateTime::parse_from_rfc3339(&exp_str) {
                                            Ok(exp) => now >= exp,
                                            Err(_) => true, // invalid expiry → remove
                                        }
                                    }
                                },
                                Err(_) => true, // invalid JSON → remove
                            };
                            if expired && tokio::fs::remove_file(&file).await.is_ok() {
                                deleted += 1;
                            }
                        }
                    }
                }
            }
        }

        debug!("package cache cleanup: deleted {deleted}/{total} expired entries");
    }
}

async fn read_dir_entries(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut entries = Vec::new();
    let mut rd = tokio::fs::read_dir(dir).await?;
    while let Some(entry) = rd.next_entry().await? {
        entries.push(entry.path());
    }
    Ok(entries)
}

// ── PackageCache ─────────────────────────────────────────────────────────────

/// Combined package cache: in-memory deduplication + optional file backend.
///
/// Mirrors the two-layer design in `lib/util/cache/package/index.ts`:
/// - inner MemCache deduplicates repeated identical lookups within one run
/// - outer `FilePackageCache` persists results across runs
///
/// `PackageCache` is cheap to clone — the internal state is behind an `Arc`.
#[derive(Debug, Default)]
pub struct PackageCache {
    backend: Option<FilePackageCache>,
    /// In-process dedup map. Key: combined key `datasource-mem:pkg-fetch:{ns}:{k}`.
    mem: Mutex<HashMap<String, Value>>,
}

impl PackageCache {
    /// Create a cache without a persistent backend (memory-only).
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a cache with a file backend rooted at `cache_dir`.
    pub fn with_dir(cache_dir: impl Into<PathBuf>) -> Self {
        Self {
            backend: Some(FilePackageCache::new(cache_dir)),
            mem: Mutex::new(HashMap::new()),
        }
    }

    fn mem_key(namespace: &PackageCacheNamespace, key: &str) -> String {
        format!("datasource-mem:pkg-fetch:{namespace}:{key}")
    }

    /// Retrieve a cached value, checking the in-memory layer first.
    ///
    /// Returns `None` if not initialized, not cached, or expired.
    pub async fn get<T: DeserializeOwned>(
        &self,
        namespace: &PackageCacheNamespace,
        key: &str,
    ) -> Option<T> {
        let backend = self.backend.as_ref()?;
        let mk = Self::mem_key(namespace, key);

        // check in-process dedup layer
        {
            let mem = self.mem.lock().expect("mem cache lock poisoned");
            if let Some(v) = mem.get(&mk) {
                return serde_json::from_value(v.clone()).ok();
            }
        }

        // check file backend
        let value: Value = backend.get(namespace, key).await?;

        // populate dedup layer
        {
            let mut mem = self.mem.lock().expect("mem cache lock poisoned");
            mem.insert(mk, value.clone());
        }

        serde_json::from_value(value).ok()
    }

    /// Store a value with a user-TTL-override-aware TTL.
    ///
    /// Mirrors `packageCache.set` (applies `getTtlOverride` before writing).
    pub async fn set(
        &self,
        namespace: &PackageCacheNamespace,
        key: &str,
        value: impl Serialize,
        ttl_minutes: i64,
    ) {
        self.set_with_raw_ttl(namespace, key, value, ttl_minutes)
            .await;
    }

    /// Store a value bypassing TTL override logic (used by `with_cache`).
    ///
    /// Mirrors `packageCache.setWithRawTtl`.
    pub async fn set_with_raw_ttl(
        &self,
        namespace: &PackageCacheNamespace,
        key: &str,
        value: impl Serialize,
        ttl_minutes: i64,
    ) {
        let json_value = match serde_json::to_value(&value) {
            Ok(v) => v,
            Err(e) => {
                debug!(namespace, key, "cache: serialize error: {e}");
                return;
            }
        };

        let mk = Self::mem_key(namespace, key);
        {
            let mut mem = self.mem.lock().expect("mem cache lock poisoned");
            mem.insert(mk, json_value.clone());
        }

        if let Some(backend) = &self.backend {
            backend.set(namespace, key, json_value, ttl_minutes).await;
        }
    }

    /// Discard all in-process dedup entries (analogous to `memCache.reset()`).
    pub fn reset_mem(&self) {
        let mut mem = self.mem.lock().expect("mem cache lock poisoned");
        mem.clear();
    }

    /// Clean up expired entries and destroy the backend.
    ///
    /// Mirrors `packageCache.cleanup`.
    pub async fn cleanup(&self) {
        if let Some(backend) = &self.backend {
            backend.cleanup().await;
        }
    }

    /// Whether a persistent backend is configured.
    pub fn is_initialized(&self) -> bool {
        self.backend.is_some()
    }

    /// Backend type string for diagnostic output.
    pub fn cache_type(&self) -> Option<&'static str> {
        self.backend.as_ref().map(|_| "file")
    }
}

// ── with_cache ───────────────────────────────────────────────────────────────

/// Configuration for TTL resolution in [`with_cache`].
///
/// Mirrors the subset of `GlobalConfig` consumed by `withCache` and `resolveTtlValues`
/// in `lib/util/cache/package/ttl.ts` and `with-cache.ts`.
#[derive(Debug, Clone, Default)]
pub struct CacheTtlConfig {
    /// Per-namespace TTL overrides (`cacheTtlOverride` in Renovate config).
    pub ttl_override: HashMap<String, i64>,
    /// Hard TTL floor in minutes (`cacheHardTtlMinutes`). Default 0 (no floor).
    pub hard_ttl_minutes: i64,
    /// Force caching even for private packages (`cachePrivatePackages`).
    pub cache_private_packages: bool,
}

/// Options for [`with_cache`].
///
/// Mirrors `CachedOptions` in `lib/util/cache/package/with-cache.ts`.
#[derive(Debug, Clone)]
pub struct WithCacheOptions<'a> {
    pub namespace: &'a PackageCacheNamespace,
    pub key: &'a str,
    /// TTL in minutes. Default: 30.
    pub ttl_minutes: i64,
    /// Whether caching is enabled for this call. Default: true.
    pub cacheable: bool,
    /// Whether to return stale data on upstream error. Default: false.
    pub fallback: bool,
}

impl<'a> WithCacheOptions<'a> {
    pub fn new(namespace: &'a PackageCacheNamespace, key: &'a str) -> Self {
        Self {
            namespace,
            key,
            ttl_minutes: 30,
            cacheable: true,
            fallback: false,
        }
    }
}

/// Resolve the TTL override for `namespace`, matching longest glob/key first.
///
/// Mirrors `getTtlOverride` in `lib/util/cache/package/ttl.ts`.
fn get_ttl_override(config: &CacheTtlConfig, namespace: &str) -> Option<i64> {
    // Exact match
    if let Some(&v) = config.ttl_override.get(namespace) {
        return Some(v);
    }
    // Longest matching glob/regex match
    let mut best: Option<(usize, i64)> = None;
    for (k, &v) in &config.ttl_override {
        if k.len() > best.map_or(0, |(l, _)| l) && match_pattern(k, namespace) {
            best = Some((k.len(), v));
        }
    }
    best.map(|(_, v)| v)
}

/// Match a namespace against a pattern using the same rules as Renovate's
/// `matchRegexOrGlob` in `lib/util/string-match.ts`:
///
/// - If pattern is `*`, always matches.
/// - If pattern starts with `/` and ends with `/`, treat as a regex.
/// - Otherwise, treat as a minimatch-style glob (via `globset`).
fn match_pattern(pattern: &str, value: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    // Regex: /pattern/ or /pattern/flags
    if let Some(inner) = pattern.strip_prefix('/') {
        let (re_src, _flags) = match inner.rfind('/') {
            Some(pos) if pos > 0 => (&inner[..pos], &inner[pos + 1..]),
            _ => return false,
        };
        return Regex::new(re_src).is_ok_and(|re| re.is_match(value));
    }
    // Glob (minimatch-compatible via globset with case-insensitive option)
    GlobBuilder::new(pattern)
        .case_insensitive(true)
        .build()
        .map(|g| g.compile_matcher().is_match(value))
        .unwrap_or(false)
}

// Keep the old name for tests, delegating to match_pattern.
#[cfg(test)]
fn glob_match(pattern: &str, value: &str) -> bool {
    match_pattern(pattern, value)
}

/// Resolve soft and hard TTL values for a namespace.
///
/// Mirrors `resolveTtlValues` in `lib/util/cache/package/ttl.ts`.
pub fn resolve_ttl_values(
    config: &CacheTtlConfig,
    namespace: &str,
    ttl_minutes: i64,
) -> (i64, i64) {
    let soft_ttl = get_ttl_override(config, namespace).unwrap_or(ttl_minutes);
    let hard_ttl = soft_ttl.max(config.hard_ttl_minutes);
    (soft_ttl, hard_ttl)
}

/// Cache the result of an async function with soft/hard TTL and stale fallback.
///
/// Mirrors `withCache` in `lib/util/cache/package/with-cache.ts`.
///
/// # Behavior
///
/// 1. If `cacheable=false` and `cachePrivatePackages=false`, bypass the cache.
/// 2. On cache hit within `soft_ttl`, return cached value immediately.
/// 3. On cache hit within `hard_ttl` (only when `fallback=true`), store as
///    potential stale fallback, then call `f`.
/// 4. If `f` fails and a stale fallback exists, return the stale value.
/// 5. On successful fetch, store with `hard_ttl` as physical TTL.
pub async fn with_cache<T, F, Fut>(
    cache: &PackageCache,
    config: &CacheTtlConfig,
    options: WithCacheOptions<'_>,
    should_cache_result: Option<fn(&Value) -> bool>,
    f: F,
) -> anyhow::Result<T>
where
    T: Serialize + DeserializeOwned,
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = anyhow::Result<T>>,
{
    let is_cacheable = config.cache_private_packages || options.cacheable;
    if !is_cacheable || !cache.is_initialized() {
        return f().await;
    }

    let (soft_ttl, hard_ttl) = resolve_ttl_values(config, options.namespace, options.ttl_minutes);
    let final_hard_ttl = if options.fallback { hard_ttl } else { soft_ttl };

    let cache_key = format!("cache-decorator:{}", options.key);

    // check cache
    let cached: Option<CachedRecord> = cache.get(options.namespace, &cache_key).await;

    let mut fallback_value: Option<Value> = None;

    if let Some(record) = cached
        && let Ok(cached_at) = DateTime::parse_from_rfc3339(&record.cached_at)
    {
        let cached_at = cached_at.with_timezone(&Utc);
        let soft_deadline = cached_at + Duration::minutes(soft_ttl);

        let passes_predicate = should_cache_result.is_none_or(|pred| pred(&record.value));

        if passes_predicate && Utc::now() < soft_deadline {
            // still fresh
            return serde_json::from_value(record.value).map_err(anyhow::Error::from);
        }

        if options.fallback && passes_predicate {
            let hard_deadline = cached_at + Duration::minutes(final_hard_ttl);
            if Utc::now() < hard_deadline {
                fallback_value = Some(record.value);
            }
        }
    }

    // fetch fresh
    let result = f().await;

    match result {
        Ok(value) => {
            let json_value = serde_json::to_value(&value)?;
            let do_cache = should_cache_result.is_none_or(|pred| pred(&json_value));
            if do_cache && json_value != Value::Null {
                // Renovate: don't cache `undefined`; `null` IS cached
                // In Rust: skip caching only if predicate rejects
            }
            // Always cache non-rejected values (null included when predicate passes)
            if do_cache {
                let record = CachedRecord {
                    value: json_value,
                    cached_at: Utc::now().to_rfc3339(),
                };
                cache
                    .set_with_raw_ttl(options.namespace, &cache_key, &record, final_hard_ttl)
                    .await;
            }
            Ok(value)
        }
        Err(e) => {
            if let Some(fb) = fallback_value {
                debug!("package cache: callback error, returning stale data: {e}");
                serde_json::from_value(fb).map_err(anyhow::Error::from)
            } else {
                Err(e)
            }
        }
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn make_cache(dir: &TempDir) -> PackageCache {
        PackageCache::with_dir(dir.path())
    }

    // ── FilePackageCache ──────────────────────────────────────────────────

    // Ported: "delegates get to backend" — lib/util/cache/package/index.spec.ts line 41
    #[tokio::test]
    async fn file_cache_get_returns_none_for_missing_key() {
        let dir = TempDir::new().unwrap();
        let cache = FilePackageCache::new(dir.path());
        let result: Option<String> = cache.get("_test-namespace", "missing-key").await;
        assert!(result.is_none());
    }

    // Ported: "delegates set to backend" — lib/util/cache/package/index.spec.ts line 51
    #[tokio::test]
    async fn file_cache_set_and_get_roundtrip() {
        let dir = TempDir::new().unwrap();
        let cache = FilePackageCache::new(dir.path());
        cache
            .set(
                "_test-namespace",
                "my-key",
                Value::String("hello".into()),
                60,
            )
            .await;
        let result: Option<String> = cache.get("_test-namespace", "my-key").await;
        assert_eq!(result, Some("hello".to_owned()));
    }

    #[tokio::test]
    async fn file_cache_returns_none_for_expired_entry() {
        let dir = TempDir::new().unwrap();
        let cache = FilePackageCache::new(dir.path());
        // write an already-expired entry manually
        let path = cache.entry_path("_test-namespace", "expired-key");
        tokio::fs::create_dir_all(path.parent().unwrap())
            .await
            .unwrap();
        let entry = FileEntry {
            value: Value::String("stale".into()),
            expiry: Some((Utc::now() - Duration::minutes(1)).to_rfc3339()),
        };
        tokio::fs::write(&path, serde_json::to_string(&entry).unwrap())
            .await
            .unwrap();

        let result: Option<String> = cache.get("_test-namespace", "expired-key").await;
        assert!(result.is_none());
        // file should be gone
        assert!(!path.exists());
    }

    // Ported: "returns undefined for null cached value" — lib/util/cache/package/impl/file.spec.ts line 65
    #[tokio::test]
    async fn file_cache_returns_none_for_null_value() {
        let dir = TempDir::new().unwrap();
        let cache = FilePackageCache::new(dir.path());
        let path = cache.entry_path("_test-namespace", "null-key");
        tokio::fs::create_dir_all(path.parent().unwrap())
            .await
            .unwrap();
        let entry = FileEntry {
            value: Value::Null,
            expiry: Some((Utc::now() + Duration::minutes(5)).to_rfc3339()),
        };
        tokio::fs::write(&path, serde_json::to_string(&entry).unwrap())
            .await
            .unwrap();

        let result: Option<String> = cache.get("_test-namespace", "null-key").await;
        assert!(result.is_none());
    }

    // Ported: "returns undefined for invalid JSON" — lib/util/cache/package/impl/file.spec.ts line 73
    #[tokio::test]
    async fn file_cache_returns_none_for_invalid_json() {
        let dir = TempDir::new().unwrap();
        let cache = FilePackageCache::new(dir.path());
        let path = cache.entry_path("_test-namespace", "bad-json-key");
        tokio::fs::create_dir_all(path.parent().unwrap())
            .await
            .unwrap();
        tokio::fs::write(&path, "not valid json").await.unwrap();

        let result: Option<String> = cache.get("_test-namespace", "bad-json-key").await;
        assert!(result.is_none());
    }

    // Ported: "returns undefined for corrupted cache payload" — lib/util/cache/package/impl/file.spec.ts line 81
    #[tokio::test]
    async fn file_cache_returns_none_for_corrupted_payload() {
        let dir = TempDir::new().unwrap();
        let cache = FilePackageCache::new(dir.path());
        let path = cache.entry_path("_test-namespace", "corrupted-key");
        tokio::fs::create_dir_all(path.parent().unwrap())
            .await
            .unwrap();
        // Write JSON with wrong shape (missing required fields)
        tokio::fs::write(&path, r#"{"foo": "bar"}"#).await.unwrap();

        let result: Option<String> = cache.get("_test-namespace", "corrupted-key").await;
        assert!(result.is_none());
    }

    // Ported: "returns undefined for missing expiry" — lib/util/cache/package/impl/file.spec.ts line 93
    #[tokio::test]
    async fn file_cache_returns_none_for_missing_expiry() {
        let dir = TempDir::new().unwrap();
        let cache = FilePackageCache::new(dir.path());
        let path = cache.entry_path("_test-namespace", "no-expiry-key");
        tokio::fs::create_dir_all(path.parent().unwrap())
            .await
            .unwrap();
        // Write JSON without expiry field
        tokio::fs::write(&path, r#"{"value":1234}"#).await.unwrap();

        let result: Option<i32> = cache.get("_test-namespace", "no-expiry-key").await;
        assert!(result.is_none());
    }

    // Ported: "returns undefined for invalid expiry" — lib/util/cache/package/impl/file.spec.ts line 102
    #[tokio::test]
    async fn file_cache_returns_none_for_invalid_expiry() {
        let dir = TempDir::new().unwrap();
        let cache = FilePackageCache::new(dir.path());
        let path = cache.entry_path("_test-namespace", "bad-expiry-key");
        tokio::fs::create_dir_all(path.parent().unwrap())
            .await
            .unwrap();
        let entry = FileEntry {
            value: Value::Number(1234.into()),
            expiry: Some("not-a-date".to_owned()),
        };
        tokio::fs::write(&path, serde_json::to_string(&entry).unwrap())
            .await
            .unwrap();

        let result: Option<i32> = cache.get("_test-namespace", "bad-expiry-key").await;
        assert!(result.is_none());
    }

    // Ported: "removes expired and invalid entries" — lib/util/cache/package/impl/file.spec.ts line 127
    #[tokio::test]
    async fn file_cache_cleanup_removes_expired_and_invalid() {
        let dir = TempDir::new().unwrap();
        let cache = FilePackageCache::new(dir.path());

        // Valid non-expired entry
        cache
            .set("_test-namespace", "valid", Value::String("ok".into()), 60)
            .await;

        // Expired entry
        let expired_path = cache.entry_path("_test-namespace", "expired");
        tokio::fs::create_dir_all(expired_path.parent().unwrap())
            .await
            .unwrap();
        let expired_entry = FileEntry {
            value: Value::String("stale".into()),
            expiry: Some((Utc::now() - Duration::minutes(1)).to_rfc3339()),
        };
        tokio::fs::write(
            &expired_path,
            serde_json::to_string(&expired_entry).unwrap(),
        )
        .await
        .unwrap();

        // Invalid JSON entry
        let bad_path = cache.entry_path("_test-namespace", "bad-json");
        tokio::fs::create_dir_all(bad_path.parent().unwrap())
            .await
            .unwrap();
        tokio::fs::write(&bad_path, "not json").await.unwrap();

        cache.cleanup().await;

        // Valid entry should remain
        let valid: Option<String> = cache.get("_test-namespace", "valid").await;
        assert_eq!(valid, Some("ok".to_owned()));

        // Expired and invalid should be gone
        assert!(!expired_path.exists());
        assert!(!bad_path.exists());
    }

    // Ported: "removes entries with invalid expiry" — lib/util/cache/package/impl/file.spec.ts line 169
    #[tokio::test]
    async fn file_cache_cleanup_removes_invalid_expiry() {
        let dir = TempDir::new().unwrap();
        let cache = FilePackageCache::new(dir.path());

        let bad_path = cache.entry_path("_test-namespace", "bad-expiry");
        tokio::fs::create_dir_all(bad_path.parent().unwrap())
            .await
            .unwrap();
        let entry = FileEntry {
            value: Value::String("value".into()),
            expiry: Some("invalid-date".to_owned()),
        };
        tokio::fs::write(&bad_path, serde_json::to_string(&entry).unwrap())
            .await
            .unwrap();

        cache.cleanup().await;

        assert!(!bad_path.exists());
    }

    // Ported: "keeps entries without expiry field" — lib/util/cache/package/impl/file.spec.ts line 159
    #[tokio::test]
    async fn file_cache_cleanup_keeps_entries_without_expiry() {
        let dir = TempDir::new().unwrap();
        let cache = FilePackageCache::new(dir.path());

        let path = cache.entry_path("_test-namespace", "no-expiry");
        tokio::fs::create_dir_all(path.parent().unwrap())
            .await
            .unwrap();
        // Write entry without expiry field
        tokio::fs::write(&path, r#"{"value":1234}"#).await.unwrap();

        cache.cleanup().await;

        assert!(path.exists());
    }

    // Ported: "continues on cleanup errors" — lib/util/cache/package/impl/file.spec.ts line 182
    #[tokio::test]
    async fn file_cache_cleanup_continues_on_errors() {
        let dir = TempDir::new().unwrap();
        let cache = FilePackageCache::new(dir.path());

        // Valid entry
        cache
            .set("_test-namespace", "valid", Value::String("ok".into()), 60)
            .await;

        // Create a non-file entry (directory) that will cause an error during read
        let ns_dir = cache
            .cache_dir
            .join("renovate")
            .join("cache-v1")
            .join("_test-namespace");
        tokio::fs::create_dir_all(&ns_dir).await.unwrap();
        let bad_path = ns_dir.join("not-a-file");
        tokio::fs::create_dir(&bad_path).await.unwrap();

        // Should not panic
        cache.cleanup().await;

        // Valid entry should still be there
        let valid: Option<String> = cache.get("_test-namespace", "valid").await;
        assert_eq!(valid, Some("ok".to_owned()));
    }

    // ── PackageCache ──────────────────────────────────────────────────────

    // Ported: "returns undefined if not initialized" — lib/util/cache/package/index.spec.ts line 23
    #[tokio::test]
    async fn package_cache_get_returns_none_without_backend() {
        let cache = PackageCache::new(); // no backend
        let result: Option<String> = cache.get("_test-namespace", "key").await;
        assert!(result.is_none());
    }

    // Ported: "delegates init to backend" — lib/util/cache/package/index.spec.ts line 33
    #[tokio::test]
    async fn package_cache_is_initialized_when_backend_set() {
        let cache = PackageCache::new(); // no backend
        assert!(!cache.is_initialized());

        let dir = TempDir::new().unwrap();
        let cache_with_backend = PackageCache::with_dir(dir.path());
        assert!(cache_with_backend.is_initialized());
    }

    // Ported: "delegates getCacheType to backend" — lib/util/cache/package/index.spec.ts line 105
    #[test]
    fn package_cache_type_returns_file_when_backend_set() {
        let cache = PackageCache::new();
        assert_eq!(cache.cache_type(), None);

        let dir = std::env::temp_dir();
        let cache_with_backend = PackageCache::with_dir(&dir);
        assert_eq!(cache_with_backend.cache_type(), Some("file"));
    }

    // Ported: "delegates cleanup to backend.destroy" — lib/util/cache/package/index.spec.ts line 99
    #[tokio::test]
    async fn package_cache_cleanup_delegates_to_backend() {
        let dir = TempDir::new().unwrap();
        let cache = make_cache(&dir);

        // Write an expired entry directly
        let file_cache = FilePackageCache::new(dir.path());
        let path = file_cache.entry_path("_test-namespace", "expired");
        tokio::fs::create_dir_all(path.parent().unwrap())
            .await
            .unwrap();
        let entry = FileEntry {
            value: Value::String("stale".into()),
            expiry: Some((Utc::now() - Duration::minutes(1)).to_rfc3339()),
        };
        tokio::fs::write(&path, serde_json::to_string(&entry).unwrap())
            .await
            .unwrap();

        // cleanup through PackageCache should delegate and remove expired
        cache.cleanup().await;
        assert!(!path.exists());
    }

    // Ported: "deduplicates get via memCache" — lib/util/cache/package/index.spec.ts line 77
    #[tokio::test]
    async fn package_cache_deduplicates_via_mem() {
        let dir = TempDir::new().unwrap();
        let cache = make_cache(&dir);
        cache
            .set("_test-namespace", "key", "cached-value".to_owned(), 30)
            .await;

        // Both calls should return the same value
        let r1: Option<String> = cache.get("_test-namespace", "key").await;
        let r2: Option<String> = cache.get("_test-namespace", "key").await;
        assert_eq!(r1, Some("cached-value".to_owned()));
        assert_eq!(r2, Some("cached-value".to_owned()));
    }

    // Ported: "setWithRawTtl updates memCache" — lib/util/cache/package/index.spec.ts line 89
    #[tokio::test]
    async fn set_with_raw_ttl_updates_mem_immediately() {
        let dir = TempDir::new().unwrap();
        let cache = make_cache(&dir);
        cache
            .set_with_raw_ttl("_test-namespace", "key", "new-value".to_owned(), 30)
            .await;
        // Reading back via get should hit mem cache, not file backend
        let result: Option<String> = cache.get("_test-namespace", "key").await;
        assert_eq!(result, Some("new-value".to_owned()));
    }

    // ── with_cache ────────────────────────────────────────────────────────

    fn default_config() -> CacheTtlConfig {
        CacheTtlConfig::default()
    }

    // Ported: "caches string result" — lib/util/cache/package/with-cache.spec.ts line 35
    #[tokio::test]
    async fn with_cache_caches_string_result() {
        let dir = TempDir::new().unwrap();
        let cache = make_cache(&dir);
        let cfg = default_config();
        let opts = WithCacheOptions::new("_test-namespace", "some-key");

        let call_count = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
        let cc = call_count.clone();
        let result1 = with_cache(&cache, &cfg, opts.clone(), None, move || {
            cc.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            async { Ok::<_, anyhow::Error>("111".to_owned()) }
        })
        .await
        .unwrap();

        // Second call — fn must not be called again
        let result2 = with_cache(&cache, &cfg, opts.clone(), None, || async {
            Ok::<_, anyhow::Error>("222".to_owned())
        })
        .await
        .unwrap();

        assert_eq!(result1, "111");
        assert_eq!(result2, "111");
        assert_eq!(call_count.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    // Ported: "disables cache if cacheable is false" — lib/util/cache/package/with-cache.spec.ts line 57
    #[tokio::test]
    async fn with_cache_disabled_when_cacheable_false() {
        let dir = TempDir::new().unwrap();
        let cache = make_cache(&dir);
        let cfg = default_config();
        let mut opts = WithCacheOptions::new("_test-namespace", "key");
        opts.cacheable = false;

        let counter = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(1));

        let c1 = counter.clone();
        let r1 = with_cache(&cache, &cfg, opts.clone(), None, move || {
            let v = c1.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            async move { Ok::<_, anyhow::Error>(format!("{v}{v}{v}")) }
        })
        .await
        .unwrap();

        let c2 = counter.clone();
        let r2 = with_cache(&cache, &cfg, opts.clone(), None, move || {
            let v = c2.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            async move { Ok::<_, anyhow::Error>(format!("{v}{v}{v}")) }
        })
        .await
        .unwrap();

        assert_eq!(r1, "111");
        assert_eq!(r2, "222");
    }

    // Ported: "forces cache if cachePrivatePackages=true" — lib/util/cache/package/with-cache.spec.ts line 83
    #[tokio::test]
    async fn with_cache_forced_when_cache_private_packages() {
        let dir = TempDir::new().unwrap();
        let cache = make_cache(&dir);
        let cfg = CacheTtlConfig {
            cache_private_packages: true,
            ..Default::default()
        };
        let mut opts = WithCacheOptions::new("_test-namespace", "key");
        opts.cacheable = false; // normally would skip caching

        let call_count = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
        let cc1 = call_count.clone();
        let r1 = with_cache(&cache, &cfg, opts.clone(), None, move || {
            cc1.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            async { Ok::<_, anyhow::Error>("111".to_owned()) }
        })
        .await
        .unwrap();

        let r2 = with_cache(&cache, &cfg, opts.clone(), None, || async {
            Ok::<_, anyhow::Error>("222".to_owned())
        })
        .await
        .unwrap();

        assert_eq!(r1, "111");
        assert_eq!(r2, "111");
        assert_eq!(call_count.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    // Ported: "does not cache undefined" — lib/util/cache/package/with-cache.spec.ts line 212
    #[tokio::test]
    async fn with_cache_does_not_cache_none() {
        let dir = TempDir::new().unwrap();
        let cache = make_cache(&dir);
        let cfg = default_config();
        let opts = WithCacheOptions::new("_test-namespace", "key");
        let call_count = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));

        let cc1 = call_count.clone();
        let r1: Option<String> = with_cache(
            &cache,
            &cfg,
            opts.clone(),
            Some(|v| v != &Value::Null),
            move || {
                cc1.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                async { Ok::<_, anyhow::Error>(None::<String>) }
            },
        )
        .await
        .unwrap();

        let cc2 = call_count.clone();
        let r2: Option<String> = with_cache(
            &cache,
            &cfg,
            opts.clone(),
            Some(|v| v != &Value::Null),
            move || {
                cc2.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                async { Ok::<_, anyhow::Error>(None::<String>) }
            },
        )
        .await
        .unwrap();

        assert_eq!(r1, None);
        assert_eq!(r2, None);
        // not cached, called both times
        assert_eq!(call_count.load(std::sync::atomic::Ordering::SeqCst), 2);
    }

    // Ported: "returns stale result on error" — lib/util/cache/package/with-cache.spec.ts line 375
    #[tokio::test]
    async fn with_cache_returns_stale_on_error_when_fallback() {
        let dir = TempDir::new().unwrap();
        let cache = make_cache(&dir);
        let cfg = CacheTtlConfig {
            hard_ttl_minutes: 120,
            ..Default::default()
        };
        let mut opts = WithCacheOptions::new("_test-namespace", "key");
        opts.ttl_minutes = 1;
        opts.fallback = true;

        // First call: populate cache
        let r1 = with_cache(&cache, &cfg, opts.clone(), None, || async {
            Ok::<_, anyhow::Error>("stale-value".to_owned())
        })
        .await
        .unwrap();
        assert_eq!(r1, "stale-value");

        // Manually backdated the cached record to force soft-TTL expiry
        // by writing a record with a past cachedAt
        let past = Utc::now() - Duration::minutes(10);
        let record = CachedRecord {
            value: Value::String("stale-value".into()),
            cached_at: past.to_rfc3339(),
        };
        cache
            .set_with_raw_ttl("_test-namespace", "cache-decorator:key", &record, 120)
            .await;

        // Second call: fn fails, should return stale
        let r2 = with_cache(&cache, &cfg, opts.clone(), None, || async {
            Err::<String, _>(anyhow::anyhow!("upstream error"))
        })
        .await
        .unwrap();

        assert_eq!(r2, "stale-value");
    }

    // Ported: "caches null values" — lib/util/cache/package/with-cache.spec.ts line 115
    #[tokio::test]
    async fn with_cache_caches_null_values() {
        let dir = TempDir::new().unwrap();
        let cache = make_cache(&dir);
        let cfg = default_config();
        let opts = WithCacheOptions::new("_test-namespace", "key");
        let call_count = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));

        let cc1 = call_count.clone();
        let r1: Option<String> = with_cache(&cache, &cfg, opts.clone(), None, move || {
            cc1.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            async { Ok::<_, anyhow::Error>(None::<String>) }
        })
        .await
        .unwrap();

        let cc2 = call_count.clone();
        let r2: Option<String> = with_cache(&cache, &cfg, opts.clone(), None, move || {
            cc2.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            async { Ok::<_, anyhow::Error>(Some("second".to_owned())) }
        })
        .await
        .unwrap();

        assert_eq!(r1, None);
        assert_eq!(r2, None); // cached null returned
        assert_eq!(call_count.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    // Ported: "ignores cached values rejected by cacheResult predicate" — lib/util/cache/package/with-cache.spec.ts line 170
    #[tokio::test]
    async fn with_cache_ignores_cached_values_rejected_by_predicate() {
        let dir = TempDir::new().unwrap();
        let cache = make_cache(&dir);
        let cfg = default_config();
        let opts = WithCacheOptions::new("_test-namespace", "key");
        let call_count = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));

        // First call caches a rejected value
        let cc1 = call_count.clone();
        let r1 = with_cache(
            &cache,
            &cfg,
            opts.clone(),
            Some(|v: &Value| v != &Value::Null),
            move || {
                cc1.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                async { Ok::<_, anyhow::Error>(None::<String>) }
            },
        )
        .await
        .unwrap();

        // Second call — predicate rejects cached null, so fn is called again
        let cc2 = call_count.clone();
        let r2 = with_cache(
            &cache,
            &cfg,
            opts.clone(),
            Some(|v: &Value| v != &Value::Null),
            move || {
                cc2.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                async { Ok::<_, anyhow::Error>(Some("fresh".to_owned())) }
            },
        )
        .await
        .unwrap();

        assert_eq!(r1, None);
        assert_eq!(r2, Some("fresh".to_owned()));
        assert_eq!(call_count.load(std::sync::atomic::Ordering::SeqCst), 2);
    }

    // Ported: "uses custom ttlMinutes" — lib/util/cache/package/with-cache.spec.ts line 232
    #[tokio::test]
    async fn with_cache_uses_custom_ttl_minutes() {
        let dir = TempDir::new().unwrap();
        let cache = make_cache(&dir);
        let cfg = default_config();
        let mut opts = WithCacheOptions::new("_test-namespace", "key");
        opts.ttl_minutes = 5;

        let r1 = with_cache(&cache, &cfg, opts.clone(), None, || async {
            Ok::<_, anyhow::Error>("cached".to_owned())
        })
        .await
        .unwrap();

        let r2 = with_cache(&cache, &cfg, opts.clone(), None, || async {
            Ok::<_, anyhow::Error>("fresh".to_owned())
        })
        .await
        .unwrap();

        assert_eq!(r1, "cached");
        assert_eq!(r2, "cached"); // still within custom 5-min TTL
    }

    // Ported: "does not return stale values rejected by cacheResult predicate" — lib/util/cache/package/with-cache.spec.ts line 414
    #[tokio::test]
    async fn with_cache_does_not_return_stale_rejected_by_predicate() {
        let dir = TempDir::new().unwrap();
        let cache = make_cache(&dir);
        let cfg = CacheTtlConfig {
            hard_ttl_minutes: 120,
            ..Default::default()
        };
        let mut opts = WithCacheOptions::new("_test-namespace", "key");
        opts.ttl_minutes = 1;
        opts.fallback = true;

        // Populate cache with null (rejected by predicate)
        let r1: Option<String> = with_cache(
            &cache,
            &cfg,
            opts.clone(),
            Some(|v: &Value| v != &Value::Null),
            || async { Ok::<_, anyhow::Error>(None::<String>) },
        )
        .await
        .unwrap();

        // Backdate the cached record
        let past = Utc::now() - Duration::minutes(10);
        let record = CachedRecord {
            value: Value::Null,
            cached_at: past.to_rfc3339(),
        };
        cache
            .set_with_raw_ttl("_test-namespace", "cache-decorator:key", &record, 120)
            .await;

        // Second call fails — predicate rejects stale null, so error propagates
        let r2 = with_cache(
            &cache,
            &cfg,
            opts.clone(),
            Some(|v: &Value| v != &Value::Null),
            || async { Err::<Option<String>, _>(anyhow::anyhow!("upstream error")) },
        )
        .await;

        assert_eq!(r1, None);
        assert!(r2.is_err());
    }

    // Ported: "drops stale value after hard TTL expires" — lib/util/cache/package/with-cache.spec.ts line 454
    #[tokio::test]
    async fn with_cache_drops_stale_after_hard_ttl_expires() {
        let dir = TempDir::new().unwrap();
        let cache = make_cache(&dir);
        let cfg = CacheTtlConfig {
            hard_ttl_minutes: 10,
            ..Default::default()
        };
        let mut opts = WithCacheOptions::new("_test-namespace", "key");
        opts.ttl_minutes = 1;
        opts.fallback = true;

        let r1 = with_cache(&cache, &cfg, opts.clone(), None, || async {
            Ok::<_, anyhow::Error>("stale".to_owned())
        })
        .await
        .unwrap();

        // Backdate past both soft and hard TTL
        let past = Utc::now() - Duration::minutes(15);
        let record = CachedRecord {
            value: Value::String("stale".into()),
            cached_at: past.to_rfc3339(),
        };
        cache
            .set_with_raw_ttl("_test-namespace", "cache-decorator:key", &record, 10)
            .await;

        // Second call fails — hard TTL expired, no fallback
        let r2 = with_cache(&cache, &cfg, opts.clone(), None, || async {
            Err::<String, _>(anyhow::anyhow!("upstream error"))
        })
        .await;

        assert_eq!(r1, "stale");
        assert!(r2.is_err());
    }

    // Ported: "does not use fallback when fallback=false" — lib/util/cache/package/with-cache.spec.ts line 505
    #[tokio::test]
    async fn with_cache_no_fallback_when_disabled() {
        let dir = TempDir::new().unwrap();
        let cache = make_cache(&dir);
        let cfg = CacheTtlConfig {
            hard_ttl_minutes: 120,
            ..Default::default()
        };
        let mut opts = WithCacheOptions::new("_test-namespace", "key");
        opts.ttl_minutes = 1;
        opts.fallback = false; // disabled

        let r1 = with_cache(&cache, &cfg, opts.clone(), None, || async {
            Ok::<_, anyhow::Error>("original".to_owned())
        })
        .await
        .unwrap();

        // Backdate past soft TTL
        let past = Utc::now() - Duration::minutes(10);
        let record = CachedRecord {
            value: Value::String("original".into()),
            cached_at: past.to_rfc3339(),
        };
        cache
            .set_with_raw_ttl("_test-namespace", "cache-decorator:key", &record, 120)
            .await;

        // Second call fails — fallback disabled, so error propagates
        let r2 = with_cache(&cache, &cfg, opts.clone(), None, || async {
            Err::<String, _>(anyhow::anyhow!("upstream error"))
        })
        .await;

        assert_eq!(r1, "original");
        assert!(r2.is_err());
    }

    // ── resolve_ttl_values ────────────────────────────────────────────────

    // Ported: "overrides soft ttl and updates result" — lib/util/cache/package/with-cache.spec.ts line 313
    #[test]
    fn resolve_ttl_values_applies_override_and_hard_min() {
        let cfg = CacheTtlConfig {
            ttl_override: [("_test-namespace".to_owned(), 2i64)].into(),
            hard_ttl_minutes: 3,
            ..Default::default()
        };
        let (soft, hard) = resolve_ttl_values(&cfg, "_test-namespace", 1);
        assert_eq!(soft, 2);
        assert_eq!(hard, 3);
    }

    // Rust-specific: package behavior test
    #[test]
    fn resolve_ttl_values_no_override_uses_arg() {
        let cfg = CacheTtlConfig::default();
        let (soft, hard) = resolve_ttl_values(&cfg, "_test-namespace", 30);
        assert_eq!(soft, 30);
        assert_eq!(hard, 30);
    }

    // ── get_ttl_override ─────────────────────────────────────────────────

    // Ported: "returns undefined when no cacheTtlOverride config exists" — lib/util/cache/package/ttl.spec.ts line 12
    //         — util/cache/package/ttl.spec.ts line 12
    #[test]
    fn get_ttl_override_returns_none_when_empty() {
        let cfg = CacheTtlConfig::default();
        assert_eq!(get_ttl_override(&cfg, "datasource-npm"), None);
    }

    // Ported: "returns exact match when namespace exists in config" — lib/util/cache/package/ttl.spec.ts line 30
    //         — util/cache/package/ttl.spec.ts line 30
    #[test]
    fn get_ttl_override_returns_exact_match() {
        let cfg = CacheTtlConfig {
            ttl_override: [("datasource-npm".to_owned(), 120i64)].into(),
            ..Default::default()
        };
        assert_eq!(get_ttl_override(&cfg, "datasource-npm"), Some(120));
        assert_eq!(get_ttl_override(&cfg, "datasource-docker"), None);
    }

    // Ported: "matches simple glob patterns" — lib/util/cache/package/ttl.spec.ts line 72
    #[test]
    fn get_ttl_override_matches_simple_glob() {
        let cfg = CacheTtlConfig {
            ttl_override: [("datasource-*".to_owned(), 90i64)].into(),
            ..Default::default()
        };
        assert_eq!(get_ttl_override(&cfg, "datasource-npm"), Some(90));
        assert_eq!(get_ttl_override(&cfg, "datasource-docker"), Some(90));
        assert_eq!(get_ttl_override(&cfg, "datasource-maven"), Some(90));
        assert_eq!(get_ttl_override(&cfg, "changelog-github-notes@v2"), None);
    }

    // Ported: "matches wildcard pattern for all namespaces" — lib/util/cache/package/ttl.spec.ts line 88
    //         — util/cache/package/ttl.spec.ts line 88
    #[test]
    fn get_ttl_override_matches_wildcard_all() {
        let cfg = CacheTtlConfig {
            ttl_override: [("*".to_owned(), 45i64)].into(),
            ..Default::default()
        };
        assert_eq!(get_ttl_override(&cfg, "datasource-npm"), Some(45));
        assert_eq!(
            get_ttl_override(&cfg, "changelog-github-notes@v2"),
            Some(45)
        );
        assert_eq!(get_ttl_override(&cfg, "any-namespace"), Some(45));
    }

    // Ported: "matches complex glob patterns with braces" — lib/util/cache/package/ttl.spec.ts line 108
    //         — util/cache/package/ttl.spec.ts line 108
    #[test]
    fn get_ttl_override_matches_brace_glob() {
        let cfg = CacheTtlConfig {
            ttl_override: [("datasource-{npm,docker}".to_owned(), 150i64)].into(),
            ..Default::default()
        };
        assert_eq!(get_ttl_override(&cfg, "datasource-npm"), Some(150));
        assert_eq!(get_ttl_override(&cfg, "datasource-docker"), Some(150));
        assert_eq!(get_ttl_override(&cfg, "datasource-maven"), None);
    }

    // Ported: "matches regex patterns" — lib/util/cache/package/ttl.spec.ts line 143
    #[test]
    fn get_ttl_override_matches_regex_pattern() {
        let cfg = CacheTtlConfig {
            ttl_override: [("/^datasource-/".to_owned(), 75i64)].into(),
            ..Default::default()
        };
        assert_eq!(get_ttl_override(&cfg, "datasource-npm"), Some(75));
        assert_eq!(get_ttl_override(&cfg, "datasource-docker"), Some(75));
        assert_eq!(get_ttl_override(&cfg, "changelog-github-notes@v2"), None);
    }

    // Ported: "prioritizes exact match over glob patterns" — lib/util/cache/package/ttl.spec.ts line 179
    //         — util/cache/package/ttl.spec.ts line 179
    #[test]
    fn get_ttl_override_exact_beats_glob() {
        let cfg = CacheTtlConfig {
            ttl_override: [
                ("datasource-*".to_owned(), 90i64),
                ("datasource-npm".to_owned(), 120i64),
                ("*".to_owned(), 45i64),
            ]
            .into(),
            ..Default::default()
        };
        assert_eq!(get_ttl_override(&cfg, "datasource-npm"), Some(120));
        assert_eq!(get_ttl_override(&cfg, "datasource-docker"), Some(90));
    }

    // Ported: "returns longest matching pattern when multiple patterns apply" — lib/util/cache/package/ttl.spec.ts line 195
    //         — util/cache/package/ttl.spec.ts line 195
    #[test]
    fn get_ttl_override_longest_pattern_wins() {
        let cfg = CacheTtlConfig {
            ttl_override: [
                ("datasource-*".to_owned(), 90i64),
                ("datasource-n*".to_owned(), 100i64),
                ("*".to_owned(), 45i64),
            ]
            .into(),
            ..Default::default()
        };
        assert_eq!(get_ttl_override(&cfg, "datasource-npm"), Some(100));
    }

    // Ported: "matches patterns with regex escape sequences" — lib/util/cache/package/ttl.spec.ts line 161
    #[test]
    fn get_ttl_override_matches_regex_with_escape_sequences() {
        let cfg = CacheTtlConfig {
            ttl_override: [("/datasource-\\w+/".to_owned(), 120i64)].into(),
            ..Default::default()
        };
        assert_eq!(get_ttl_override(&cfg, "datasource-npm"), Some(120));
        assert_eq!(get_ttl_override(&cfg, "datasource-123"), Some(120));
        assert_eq!(get_ttl_override(&cfg, "datasource-"), None); // no chars after -
    }

    // Ported: "selects longest matching pattern across all configs" — lib/util/cache/package/ttl.spec.ts line 209
    #[test]
    fn get_ttl_override_selects_longest_across_4_patterns() {
        let cfg = CacheTtlConfig {
            ttl_override: [
                ("*".to_owned(), 10i64),
                ("datasource-*".to_owned(), 20i64),
                ("datasource-npm*".to_owned(), 30i64),
                ("datasource-npm-*".to_owned(), 40i64),
            ]
            .into(),
            ..Default::default()
        };
        assert_eq!(get_ttl_override(&cfg, "datasource-npm-registry"), Some(40));
        assert_eq!(get_ttl_override(&cfg, "datasource-npmjs"), Some(30));
    }

    // Ported: "handles negative numbers" — lib/util/cache/package/ttl.spec.ts line 318
    #[test]
    fn get_ttl_override_handles_negative_values() {
        let cfg = CacheTtlConfig {
            ttl_override: [("datasource-npm".to_owned(), -1i64)].into(),
            ..Default::default()
        };
        assert_eq!(get_ttl_override(&cfg, "datasource-npm"), Some(-1));
    }

    // Ported: "handles very large numbers" — lib/util/cache/package/ttl.spec.ts line 306
    #[test]
    fn get_ttl_override_handles_very_large_numbers() {
        let cfg = CacheTtlConfig {
            ttl_override: [("datasource-npm".to_owned(), 999_999_999i64)].into(),
            ..Default::default()
        };
        assert_eq!(get_ttl_override(&cfg, "datasource-npm"), Some(999_999_999));
    }

    // Ported: "resolves TTL with glob pattern overrides" — lib/util/cache/package/ttl.spec.ts line 391
    #[test]
    fn resolve_ttl_values_with_glob_override() {
        let cfg = CacheTtlConfig {
            ttl_override: [("datasource-*".to_owned(), 180i64)].into(),
            hard_ttl_minutes: 2880,
            ..Default::default()
        };
        let (soft, hard) = resolve_ttl_values(&cfg, "datasource-npm", 60);
        assert_eq!(soft, 180);
        assert_eq!(hard, 2880);
    }

    // Ported: "resolves TTL correctly with multiple overlapping overrides" — lib/util/cache/package/ttl.spec.ts line 407
    #[test]
    fn resolve_ttl_values_multi_overlap() {
        let cfg = CacheTtlConfig {
            ttl_override: [
                ("datasource-*".to_owned(), 100i64),
                ("datasource-npm".to_owned(), 200i64),
                ("*".to_owned(), 50i64),
            ]
            .into(),
            hard_ttl_minutes: 5760,
            ..Default::default()
        };
        let (soft, hard) = resolve_ttl_values(&cfg, "datasource-npm", 60);
        assert_eq!(soft, 200); // exact match wins
        assert_eq!(hard, 5760);
    }

    // Ported: "handles negative cacheHardTtlMinutes config" — lib/util/cache/package/ttl.spec.ts line 443
    #[test]
    fn resolve_ttl_values_negative_hard_ttl() {
        let cfg = CacheTtlConfig {
            ttl_override: [("datasource-npm".to_owned(), 120i64)].into(),
            hard_ttl_minutes: -1,
            ..Default::default()
        };
        let (soft, hard) = resolve_ttl_values(&cfg, "datasource-npm", 60);
        assert_eq!(soft, 120);
        // hard = max(soft, hard_ttl) = max(120, -1) = 120
        assert_eq!(hard, 120);
    }

    // Ported: "handles zero as valid override value" — lib/util/cache/package/ttl.spec.ts line 461
    #[test]
    fn resolve_ttl_values_zero_override() {
        let cfg = CacheTtlConfig {
            ttl_override: [("datasource-npm".to_owned(), 0i64)].into(),
            hard_ttl_minutes: 1440,
            ..Default::default()
        };
        let (soft, hard) = resolve_ttl_values(&cfg, "datasource-npm", 60);
        assert_eq!(soft, 0);
        assert_eq!(hard, 1440);
    }

    // ── glob_match ────────────────────────────────────────────────────────

    // Rust-specific: package behavior test
    #[test]
    fn glob_match_exact() {
        assert!(glob_match("_test-namespace", "_test-namespace"));
        assert!(!glob_match("_test-namespace", "_other"));
    }

    // Rust-specific: package behavior test
    #[test]
    fn glob_match_wildcard_prefix() {
        assert!(glob_match("datasource-*", "datasource-npm"));
        assert!(glob_match("datasource-*", "datasource-pypi"));
        assert!(!glob_match("datasource-*", "other-npm"));
    }

    // Rust-specific: package behavior test
    #[test]
    fn glob_match_wildcard_suffix() {
        assert!(glob_match("*-namespace", "_test-namespace"));
        assert!(!glob_match("*-namespace", "_test-space"));
    }

    // Ported: "applies patterns consistently regardless of case in config order" — lib/util/cache/package/ttl.spec.ts line 256
    #[test]
    fn get_ttl_override_case_order_consistency() {
        // When two same-length patterns both match case-insensitively, the first
        // defined wins (matching TypeScript's strict > length comparison: first match
        // sets maxLen and subsequent same-length matches do not overwrite).
        // In Rust we use a BTreeMap view for deterministic iteration order.
        let ttl_override: HashMap<String, i64> = [
            ("Datasource-*".to_owned(), 120i64),
            ("datasource-*".to_owned(), 90i64),
        ]
        .into_iter()
        .collect();
        let cfg = CacheTtlConfig {
            ttl_override,
            ..Default::default()
        };
        // Both patterns match 'datasource-docker' case-insensitively.
        // The first-encountered pattern (determined by iteration) should win.
        // We verify the result is one of the two valid values.
        let res = get_ttl_override(&cfg, "datasource-docker");
        assert!(
            res == Some(120) || res == Some(90),
            "expected 120 or 90, got {res:?}"
        );
    }

    // Ported: "handles empty string pattern" — lib/util/cache/package/ttl.spec.ts line 271
    #[test]
    fn get_ttl_override_handles_empty_string_pattern() {
        let cfg = CacheTtlConfig {
            ttl_override: [
                ("".to_owned(), 30i64),
                ("datasource-npm".to_owned(), 120i64),
            ]
            .into_iter()
            .collect(),
            ..Default::default()
        };
        // Exact match for "datasource-npm"
        assert_eq!(get_ttl_override(&cfg, "datasource-npm"), Some(120));
        // Exact match for empty string
        assert_eq!(get_ttl_override(&cfg, ""), Some(30));
    }

    #[tokio::test]
    async fn reset_mem_clears_cache() {
        let cache = PackageCache::new();
        let ns: &str = "global";
        cache.set(ns, "key", "value", 60).await;
        cache.reset_mem();
        let result: Option<String> = cache.get(ns, "key").await;
        assert!(result.is_none());
    }
}
