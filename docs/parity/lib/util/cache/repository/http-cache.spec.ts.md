# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/cache/repository/http-cache.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/cache/repository/http-cache.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 0 | **Status:** done

### `util/cache/repository/http-cache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should not throw if cache is not a valid HttpCache | 12 | ported | `http.rs` | `cleanup_http_cache_noop_for_empty_object` | — |
| should remove expired items from the cache | 16 | ported | `http.rs` | `cleanup_http_cache_removes_expired_entries` | — |
| should remove all items if ttlDays is not configured | 50 | ported | `http.rs` | `cleanup_http_cache_removes_all_when_ttl_is_zero` | — |

---

