# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/finalize/prune.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/finalize/prune.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** pending

### `workers/repository/finalize/prune › pruneStaleBranches()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no branchList | 24 | pending | — | — | — |
| ignores reconfigure branch | 30 | pending | — | — | — |
| returns if no defaultBranch | 36 | pending | — | — | — |
| returns if no renovate branches | 43 | pending | — | — | — |
| returns if no remaining branches | 51 | pending | — | — | — |
| renames deletes remaining branch | 59 | pending | — | — | — |
| skips rename but still deletes branch | 71 | pending | — | — | — |
| deletes with base branches | 87 | pending | — | — | — |
| uses single configured base branch instead of defaultBranch | 124 | pending | — | — | — |
| uses defaultBranch when baseBranchPatterns exist but baseBranches are not computed yet | 145 | pending | — | — | — |
| does nothing on dryRun | 172 | pending | — | — | — |
| does nothing on prune stale branches disabled | 185 | pending | — | — | — |
| notifies via PR changes if someone pushed to PR | 198 | pending | — | — | — |
| skips appending - abandoned to PR title if already present | 213 | pending | — | — | — |
| skips changes to PR if dry run | 227 | pending | — | — | — |
| dry run delete branch no PR | 243 | pending | — | — | — |
| delete branch no PR | 256 | pending | — | — | — |
| does not delete modified orphan branch | 268 | pending | — | — | — |

---
