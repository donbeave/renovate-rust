# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/config-migration/branch/rebase.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/config-migration/branch/rebase.spec.ts
**Total tests:** 5 | **Ported:** 1 | **Actionable:** 5 | **Status:** done

### `workers/repository/config-migration/branch/rebase › rebaseMigrationBranch()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing if branch is up to date (%s) | 48 | not-applicable | — | — | Uses git.getFile mock and scm.checkoutBranch/commitAndPush mocks (git repo mock); not portable |
| rebases migration branch (%s) | 66 | not-applicable | — | — | Uses scm.isBranchBehindBase mock, vi.spyOn(prettierSpy), scm mocks; not portable |
| applies prettier formatting when rebasing the migration branch (%s) | 83 | not-applicable | — | — | Uses vi.spyOn(prettierSpy) and scm mocks (git repo mock); not portable |
| does not rebases migration branch when in dryRun is on (%s) | 118 | not-applicable | — | — | Uses scm.isBranchBehindBase mock and scm.checkoutBranch/commitAndPush mocks; not portable |

### `workers/repository/config-migration/branch/rebase › jsonStripWhiteSpaces()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should strip white spaces from json | 140 | ported | `json_writer.rs` | `json_strip_whitespaces_removes_formatting` | — |

---

