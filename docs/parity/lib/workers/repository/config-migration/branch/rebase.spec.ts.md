# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/config-migration/branch/rebase.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/config-migration/branch/rebase.spec.ts
**Total tests:** 5 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `workers/repository/config-migration/branch/rebase › rebaseMigrationBranch()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing if branch is up to date (%s) | 48 | not-applicable | — | — | SCM git operations / prettier formatting |
| rebases migration branch (%s) | 66 | not-applicable | — | — | SCM git operations / prettier formatting |
| applies prettier formatting when rebasing the migration branch (%s) | 83 | not-applicable | — | — | SCM git operations / prettier formatting |
| does not rebases migration branch when in dryRun is on (%s) | 118 | not-applicable | — | — | SCM git operations / prettier formatting |

### `workers/repository/config-migration/branch/rebase › jsonStripWhiteSpaces()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should strip white spaces from json | 140 | ported | `json_writer.rs` | `json_strip_whitespaces_removes_formatting` | — |

---

