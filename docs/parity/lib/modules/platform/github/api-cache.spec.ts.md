# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/github/api-cache.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/github/api-cache.spec.ts
**Total tests:** 15 | **Ported:** 15 | **Actionable:** 0 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| stores and retrieves items | 12 | ported | `platform/github_api_cache.rs` | `stores_and_retrieves_items` | — |

### `getItems`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| maps items | 29 | ported | `platform/github_api_cache.rs` | `get_items_maps_items` | — |
| resets cache on item update | 46 | ported | `platform/github_api_cache.rs` | `get_items_resets_on_item_update` | — |
| resets cache on page reconcile | 69 | ported | `platform/github_api_cache.rs` | `get_items_resets_on_page_reconcile` | — |

### `getLastModified`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns undefined when no lastModified in cache | 94 | ported | `platform/github_api_cache.rs` | `get_last_modified_returns_none_when_not_set` | — |
| returns stored value when present | 100 | ported | `platform/github_api_cache.rs` | `get_last_modified_returns_stored_value` | — |
| returns updated value after reconcile | 106 | ported | `platform/github_api_cache.rs` | `get_last_modified_returns_updated_after_reconcile` | — |

### `updateLastModified`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sets lastModified when not present | 116 | ported | `platform/github_api_cache.rs` | `update_last_modified_sets_when_absent` | — |
| advances lastModified to newer timestamp | 124 | ported | `platform/github_api_cache.rs` | `update_last_modified_advances_to_newer` | — |
| does not regress lastModified to older timestamp | 132 | ported | `platform/github_api_cache.rs` | `update_last_modified_does_not_regress` | — |

### `reconcile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false for empty page | 142 | ported | `platform/github_api_cache.rs` | `reconcile_returns_false_for_empty_page` | — |
| appends new items | 152 | ported | `platform/github_api_cache.rs` | `reconcile_appends_new_items` | — |
| handles updated items | 175 | ported | `platform/github_api_cache.rs` | `reconcile_handles_updated_items` | — |
| ignores page overlap | 199 | ported | `platform/github_api_cache.rs` | `reconcile_ignores_page_overlap` | — |
| does not require new page if all items are old | 226 | ported | `platform/github_api_cache.rs` | `reconcile_does_not_require_next_page_if_all_old` | — |

---

