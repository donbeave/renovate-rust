# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/cache/memory/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/cache/memory/index.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 0 | **Status:** done

### `util/cache/memory/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns undefined if not init | 4 | ported | `cache/memory.rs` | `mem_cache_returns_none_when_not_initialized` | — |
| sets and gets repo cache | 8 | ported | `cache/memory.rs` | `mem_cache_sets_and_gets_value` | — |
| resets | 14 | ported | `cache/memory.rs` | `mem_cache_reset_clears_values` | — |

### `util/cache/memory/index › cleanDatasourceKeys`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing if no matching keys exist | 26 | ported | `cache/memory.rs` | `clean_datasource_keys_noop_for_non_matching` | — |
| removes keys that start with datasource-mem:pkg-fetch: | 34 | ported | `cache/memory.rs` | `clean_datasource_keys_removes_pkg_fetch_prefix` | — |
| removes keys that start with datasource-releases | 42 | ported | `cache/memory.rs` | `clean_datasource_keys_removes_releases_prefix` | — |
| removes all matching keys while keeping others | 50 | ported | `cache/memory.rs` | `clean_datasource_keys_removes_all_matching_keeps_others` | — |

---

