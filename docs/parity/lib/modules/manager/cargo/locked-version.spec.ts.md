# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/cargo/locked-version.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/cargo/locked-version.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 8 | **Status:** ported

### `extractLockFileVersions()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for missing lock file | 19 | ported | `cargo.rs` | `extract_versions_missing_file_returns_none` | — |
| returns null for invalid lock file | 23 | ported | `cargo.rs` | `extract_versions_invalid_content_returns_none` | — |
| returns empty map for lock file without packages | 28 | ported | `cargo.rs` | `extract_versions_no_packages_returns_empty` | — |
| returns a map of package versions | 33 | ported | `cargo.rs` | `extract_versions_returns_map_of_package_versions` | — |

### `parseLockFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses v1 lockfile string into an object | 51 | ported | `cargo.rs` | `parse_lock_file_v1` | — |
| parses v2 lockfile string into an object | 70 | ported | `cargo.rs` | `parse_lock_file_v2` | — |
| parses v3 lockfile string into an object | 88 | ported | `cargo.rs` | `parse_lock_file_v3` | — |
| can deal with invalid lockfiles | 106 | ported | `cargo.rs` | `parse_lock_file_invalid` | — |

---

