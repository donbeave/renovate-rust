# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/finalize/prune.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/finalize/prune.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/finalize/prune › pruneStaleBranches()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no branchList | 24 | not-applicable | — | — | tests branch pruning via platform API (delete branches/PRs); platform interactions out of scope |
| ignores reconfigure branch | 30 | not-applicable | — | — | tests branch pruning via platform API (delete branches/PRs); platform interactions out of scope |
| returns if no defaultBranch | 36 | not-applicable | — | — | tests branch pruning via platform API (delete branches/PRs); platform interactions out of scope |
| returns if no renovate branches | 43 | not-applicable | — | — | tests branch pruning via platform API (delete branches/PRs); platform interactions out of scope |
| returns if no remaining branches | 51 | not-applicable | — | — | tests branch pruning via platform API (delete branches/PRs); platform interactions out of scope |
| renames deletes remaining branch | 59 | not-applicable | — | — | tests branch pruning via platform API (delete branches/PRs); platform interactions out of scope |
| skips rename but still deletes branch | 71 | not-applicable | — | — | tests branch pruning via platform API (delete branches/PRs); platform interactions out of scope |
| deletes with base branches | 87 | not-applicable | — | — | tests branch pruning via platform API (delete branches/PRs); platform interactions out of scope |
| uses single configured base branch instead of defaultBranch | 124 | not-applicable | — | — | tests branch pruning via platform API (delete branches/PRs); platform interactions out of scope |
| uses defaultBranch when baseBranchPatterns exist but baseBranches are not computed yet | 145 | not-applicable | — | — | tests branch pruning via platform API (delete branches/PRs); platform interactions out of scope |
| does nothing on dryRun | 172 | not-applicable | — | — | tests branch pruning via platform API (delete branches/PRs); platform interactions out of scope |
| does nothing on prune stale branches disabled | 185 | not-applicable | — | — | tests branch pruning via platform API (delete branches/PRs); platform interactions out of scope |
| notifies via PR changes if someone pushed to PR | 198 | not-applicable | — | — | tests branch pruning via platform API (delete branches/PRs); platform interactions out of scope |
| skips appending - abandoned to PR title if already present | 213 | not-applicable | — | — | tests branch pruning via platform API (delete branches/PRs); platform interactions out of scope |
| skips changes to PR if dry run | 227 | not-applicable | — | — | tests branch pruning via platform API (delete branches/PRs); platform interactions out of scope |
| dry run delete branch no PR | 243 | not-applicable | — | — | tests branch pruning via platform API (delete branches/PRs); platform interactions out of scope |
| delete branch no PR | 256 | not-applicable | — | — | tests branch pruning via platform API (delete branches/PRs); platform interactions out of scope |
| does not delete modified orphan branch | 268 | not-applicable | — | — | tests branch pruning via platform API (delete branches/PRs); platform interactions out of scope |

---
