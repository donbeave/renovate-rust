# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/util/cache/package/impl/file.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/cache/package/impl/file.spec.ts
**Total tests:** 16 | **Ported:** 13 | **Actionable:** 14 | **Status:** done

### `util/cache/package/impl/file › basic operations`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sets and gets | 26 | ported | cache/package.rs | `file_cache_set_and_get_roundtrip` | — |
| stores payload with value and expiry | 34 | ported | cache/package.rs | `file_cache_set_and_get_roundtrip` | FileEntry stores expiry |

### `util/cache/package/impl/file › get`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns undefined on cache miss | 47 | ported | cache/package.rs | `file_cache_get_returns_none_for_missing_key` | — |
| expires cached entries | 53 | ported | cache/package.rs | `file_cache_returns_none_for_expired_entry` | — |
| returns undefined for null cached value | 65 | ported | cache/package.rs | `file_cache_returns_none_for_null_value` | — |
| returns undefined for invalid JSON | 73 | ported | cache/package.rs | `file_cache_returns_none_for_invalid_json` | — |
| returns undefined for corrupted cache payload | 81 | ported | cache/package.rs | `file_cache_returns_none_for_corrupted_payload` | — |
| returns undefined for missing expiry | 93 | ported | cache/package.rs | `file_cache_returns_none_for_missing_expiry` | — |
| returns undefined for invalid expiry | 102 | ported | cache/package.rs | `file_cache_returns_none_for_invalid_expiry` | — |
| retrieves value from cache payload | 114 | ported | cache/package.rs | `file_cache_set_and_get_roundtrip` | — |

### `util/cache/package/impl/file › destroy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| removes expired and invalid entries | 127 | ported | cache/package.rs | `file_cache_cleanup_removes_expired_and_invalid` | — |
| keeps entries without expiry field | 148 | ported | cache/package.rs | `file_cache_cleanup_keeps_entries_without_expiry` | — |
| removes entries with invalid expiry | 158 | ported | cache/package.rs | `file_cache_cleanup_removes_invalid_expiry` | — |
| continues on cleanup errors | 171 | ported | cache/package.rs | `file_cache_cleanup_continues_on_errors` | — |
| skips disk read for entry written this run | 183 | not-applicable | — | — | LRU expiry map optimization not ported |
| skips disk read for expired entry written this run | 197 | not-applicable | — | — | LRU expiry map optimization not ported |

---
