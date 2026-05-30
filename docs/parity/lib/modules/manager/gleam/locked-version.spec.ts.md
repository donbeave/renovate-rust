# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gleam/locked-version.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gleam/locked-version.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 0 | **Status:** done

### `extractLockFileVersions()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for missing lock file | 19 | ported | `gleam.rs` | `extract_versions_missing_file_returns_none` | — |
| returns null for invalid lock file | 23 | ported | `gleam.rs` | `gleam_lock_returns_none_for_invalid` | — |
| returns empty map for lock file without packages | 28 | ported | `gleam.rs` | `gleam_lock_returns_empty_map_for_no_packages` | — |
| returns a map of package versions | 33 | ported | `gleam.rs` | `gleam_lock_returns_map_of_package_versions` | — |

### `parseLockFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses lockfile string into an object | 45 | ported | `gleam.rs` | `gleam_lock_parses_into_object` | — |
| can deal with invalid lockfiles | 63 | ported | `gleam.rs` | `gleam_lock_handles_invalid_lockfile` | — |

---

