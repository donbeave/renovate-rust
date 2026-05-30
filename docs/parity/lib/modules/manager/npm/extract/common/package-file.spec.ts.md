# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/extract/common/package-file.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/extract/common/package-file.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `modules/manager/npm/extract/common/package-file`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true for a valid packageManager with name@version(e.g. pnpm@8.15.4) | 20 | ported | `extractors/npm.rs` | `has_package_manager_valid_version` | Ported as pure function; test passes content directly instead of reading from filesystem |
| returns true for a valid range like npm@^9 | 31 | ported | `extractors/npm.rs` | `has_package_manager_range` | — |
| returns true for yarn classic pin yarn@1.22.19 | 38 | ported | `extractors/npm.rs` | `has_package_manager_yarn_classic` | — |
| returns false when packageManager does not contain '@' (e.g. 'npm') | 45 | ported | `extractors/npm.rs` | `has_package_manager_no_at` | — |
| returns false when packageManager is missing | 52 | ported | `extractors/npm.rs` | `has_package_manager_missing` | — |
| returns false when package.json is invalid | 57 | ported | `extractors/npm.rs` | `has_package_manager_invalid_json` | — |
| returns false if packageManager is an empty string | 62 | ported | `extractors/npm.rs` | `has_package_manager_empty_string` | — |

---
