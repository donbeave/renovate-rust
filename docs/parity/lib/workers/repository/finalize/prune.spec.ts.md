# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/finalize/prune.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/finalize/prune.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** not-applicable

### `workers/repository/finalize/prune › pruneStaleBranches()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no branchList | 24 | not-applicable | — | — | All tests use git/platform/scm test mocks (from ~test/util.ts) — platform integration infrastructure |
| ignores reconfigure branch | 30 | not-applicable | — | — | Uses git/platform/scm mocks |
| returns if no defaultBranch | 36 | not-applicable | — | — | Uses git/platform/scm mocks |
| returns if no renovate branches | 43 | not-applicable | — | — | Uses git/platform/scm mocks |
| returns if no remaining branches | 51 | not-applicable | — | — | Uses git/platform/scm mocks |
| renames deletes remaining branch | 59 | not-applicable | — | — | Uses git/platform/scm mocks — calls scm.deleteBranch, platform.updatePr |
| skips rename but still deletes branch | 71 | not-applicable | — | — | Uses git/platform/scm mocks |
| deletes with base branches | 87 | not-applicable | — | — | Uses git/platform/scm mocks |
| uses single configured base branch instead of defaultBranch | 124 | not-applicable | — | — | Uses git/platform/scm mocks |
| uses defaultBranch when baseBranchPatterns exist but baseBranches are not computed yet | 145 | not-applicable | — | — | Uses git/platform/scm mocks |
| does nothing on dryRun | 172 | not-applicable | — | — | Uses git/platform/scm mocks |
| does nothing on prune stale branches disabled | 185 | not-applicable | — | — | Uses git/platform/scm mocks |
| notifies via PR changes if someone pushed to PR | 198 | not-applicable | — | — | Uses git/platform/scm mocks — calls platform.updatePr |
| skips appending - abandoned to PR title if already present | 213 | not-applicable | — | — | Uses git/platform/scm mocks |
| skips changes to PR if dry run | 227 | not-applicable | — | — | Uses git/platform/scm mocks |
| dry run delete branch no PR | 243 | not-applicable | — | — | Uses git/platform/scm mocks |
| delete branch no PR | 256 | not-applicable | — | — | Uses git/platform/scm mocks — calls scm.deleteBranch |
| does not delete modified orphan branch | 268 | not-applicable | — | — | Uses git/platform/scm mocks — calls git.branchExists |

---
