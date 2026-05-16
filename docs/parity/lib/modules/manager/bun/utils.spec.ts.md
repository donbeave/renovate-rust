# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bun/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bun/utils.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `fileMatchesWorkspaces`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return false when fileName does not start with pwd | 7 | ported | `npm.rs` | `bun_file_matches_workspaces_false_when_different_pwd` | — |
| should correctly evaluate fileName when it starts with pwd | 14 | ported | `npm.rs` | `bun_file_matches_workspaces_true_when_starts_with_pwd` | — |

### `filesMatchingWorkspaces`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should filter files matching workspaces and pwd | 30 | ported | `npm.rs` | `bun_files_matching_workspaces_filters_correctly` | — |

---

