# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gomod/artifacts-extra.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gomod/artifacts-extra.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 0 | **Status:** done

### `getExtraDeps`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| detects extra dependencies | 34 | ported | `gomod.rs` | `get_extra_deps_detects_changes` | — |

### `extraDepsTable`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| generates a table | 55 | ported | `gomod.rs` | `extra_deps_table_generates_aligned_markdown` | — |

### `getExtraDepsNotice`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when one of files is missing | 83 | ported | `gomod.rs` | `get_extra_deps_notice_returns_none_for_missing_files` | — |
| returns null when all dependencies are excluded | 88 | ported | `gomod.rs` | `get_extra_deps_notice_returns_none_when_all_excluded` | — |
| returns a notice when there is an extra dependency | 94 | ported | `gomod.rs` | `get_extra_deps_notice_single_dep` | — |
| returns a notice when there are extra dependencies | 117 | ported | `gomod.rs` | `get_extra_deps_notice_multiple_deps` | — |
| adds special notice for updated `go` version | 141 | ported | `gomod.rs` | `get_extra_deps_notice_go_version_updated` | — |
| correctly identifies toolchain updates vs go version updates | 166 | ported | `gomod.rs` | `get_extra_deps_notice_toolchain_update` | — |
| correctly identifies and distinguishes toolchain updates vs go version updates when both are present | 215 | ported | `gomod.rs` | `get_extra_deps_notice_both_go_and_toolchain` | — |
| correctly handles the introduction of a toolchain directive by not indicating a change | 266 | ported | `gomod.rs` | `get_extra_deps_notice_new_toolchain_directive` | — |

---

