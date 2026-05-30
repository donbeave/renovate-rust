# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/npm/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/utils.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 0 | **Status:** done

### `parseLockFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses lockfile string into an object | 16 | ported | `npm.rs` | `npm_parse_lock_file_parses_into_object` | — |
| can deal with invalid lockfiles | 37 | ported | `npm.rs` | `npm_parse_lock_file_invalid_returns_none` | — |

### `composeLockFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| composes lockfile string out of an object | 48 | ported | `npm.rs` | `npm_compose_lock_file_serializes_with_indent` | — |
| adds trailing newline to match npms behavior and avoid diffs | 66 | ported | `npm.rs` | `npm_compose_lock_file_round_trips_fixture` | — |

### `loadPackageJson`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| loads and parses package.json correctly | 81 | ported | `extractors/npm.rs` | `npm_load_package_json_parses_correctly` | — |
| returns empty object when package.json is missing | 100 | ported | `extractors/npm.rs` | `npm_load_package_json_missing_returns_none` | — |
| returns empty object when package.json is invalid | 105 | ported | `extractors/npm.rs` | `npm_load_package_json_invalid_json_returns_none` | — |

---
