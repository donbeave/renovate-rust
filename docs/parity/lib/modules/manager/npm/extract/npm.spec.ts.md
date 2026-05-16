# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/extract/npm.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/extract/npm.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `modules/manager/npm/extract/npm › .getNpmLock()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if failed to parse | 9 | ported | `npm.rs` | `npm_lock_returns_empty_if_failed_to_parse` | — |
| extracts | 15 | ported | `npm.rs` | `npm_lock_extracts_v1_dependencies` | — |
| extracts npm 7 lockfile | 34 | ported | `npm.rs` | `npm_lock_extracts_v2_packages` | — |
| extracts npm 9 lockfile | 53 | ported | `npm.rs` | `npm_lock_extracts_v3_packages` | — |
| returns null if no deps | 72 | ported | `npm.rs` | `npm_lock_returns_empty_if_no_deps` | — |
| returns null on read error | 78 | ported | `npm.rs` | `npm_lock_returns_empty_on_read_error` | — |

---

