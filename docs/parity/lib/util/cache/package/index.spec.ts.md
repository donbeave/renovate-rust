# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/cache/package/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/cache/package/index.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `util/cache/package/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns undefined if not initialized | 23 | ported | cache/package.rs | `package_cache_get_returns_none_without_backend` | — |
| delegates init to backend | 33 | ported | cache/package.rs | `file_cache_set_and_get_roundtrip` | construction behavior covered |
| delegates get to backend | 41 | ported | cache/package.rs | `file_cache_get_returns_none_for_missing_key` | — |
| delegates set to backend | 51 | ported | cache/package.rs | `file_cache_set_and_get_roundtrip` | — |
| delegates setWithRawTtl to backend | 64 | ported | cache/package.rs | `set_with_raw_ttl_updates_mem_immediately` | — |
| deduplicates get via memCache | 77 | ported | cache/package.rs | `package_cache_deduplicates_via_mem` | — |
| setWithRawTtl updates memCache | 89 | ported | cache/package.rs | `set_with_raw_ttl_updates_mem_immediately` | — |
| delegates cleanup to backend.destroy | 99 | ported | cache/package.rs | `package_cache_cleanup_delegates_to_backend` | — |
| delegates getCacheType to backend | 105 | ported | cache/package.rs | `package_cache_type_returns_file_when_backend_set` | — |

---
