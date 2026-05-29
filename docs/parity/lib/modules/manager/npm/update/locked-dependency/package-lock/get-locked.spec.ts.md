# Renovate Test Detail

[Back to test map](../../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/update/locked-dependency/package-lock/get-locked.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/update/locked-dependency/package-lock/get-locked.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** done

### `getLockedDependencies()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles error | 11 | ported | `extractors/npm.rs` | `pkg_lock_get_locked_handles_null` | — |
| returns empty if failed to parse | 17 | ported | `extractors/npm.rs` | `pkg_lock_get_locked_returns_empty_for_no_deps` | — |
| finds direct dependency | 21 | ported | `extractors/npm.rs` | `pkg_lock_get_locked_finds_direct` | — |
| finds indirect dependency | 32 | ported | `extractors/npm.rs` | `pkg_lock_get_locked_finds_indirect` | — |
| finds any version | 43 | ported | `extractors/npm.rs` | `pkg_lock_get_locked_finds_any_version` | — |
| finds bundled dependency | 49 | ported | `extractors/npm.rs` | `pkg_lock_get_locked_finds_bundled` | — |

---
