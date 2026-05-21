# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/mise/lockfile.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/mise/lockfile.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `modules/manager/mise/lockfile › getConfigType()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns isLocal=$isLocal env=$env for $configPath | 10 | ported | `mise.rs` | `get_config_type_parses_all_variants` | — |

### `modules/manager/mise/lockfile › getLockFileName()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns $expected for $configPath | 27 | ported | `mise.rs` | `get_lock_file_name_derives_correct_path` | — |

### `modules/manager/mise/lockfile › getLockedVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns $expected for $depName | 55 | ported | `mise.rs` | `get_locked_version_returns_correct_version` | — |
| returns first version when multiple versions exist | 70 | ported | `mise.rs` | `get_locked_version_returns_first_when_multiple` | — |
| handles tools with bracket options in name | 74 | ported | `mise.rs` | `get_locked_version_handles_bracket_options_in_name` | — |
