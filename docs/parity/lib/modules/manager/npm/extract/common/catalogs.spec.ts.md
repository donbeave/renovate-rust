# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/extract/common/catalogs.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/extract/common/catalogs.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns correct dependencies for pnpm | 5 | ported | `npm.rs` | `catalog_deps_for_pnpm` | — |
| returns correct dependencies for yarn | 39 | ported | `npm.rs` | `catalog_deps_for_yarn` | — |
| handles empty catalogs list | 73 | ported | `npm.rs` | `catalog_deps_empty_list` | — |
| handles catalog with no dependencies | 80 | ported | `npm.rs` | `catalog_deps_empty_dependencies` | — |

---

