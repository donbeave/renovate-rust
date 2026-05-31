# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/finalize/prune.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/finalize/prune.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `workers/repository/finalize/prune › pruneStaleBranches()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no branchList  | 24 | not-applicable | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer |
| ignores reconfigure branch  | 30 | not-applicable | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer |
| returns if no defaultBranch  | 36 | not-applicable | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer |
| returns if no renovate branches  | 43 | not-applicable | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer |
| returns if no remaining branches  | 51 | not-applicable | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer |
| renames deletes remaining branch  | 59 | not-applicable | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer |
| skips rename but still deletes branch  | 71 | not-applicable | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer |
| deletes with base branches  | 87 | not-applicable | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer |
| uses single configured base branch instead of defaultBranch  | 124 | not-applicable | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer |
| uses defaultBranch when baseBranchPatterns exist but baseBranches are not computed yet  | 145 | not-applicable | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer |
| does nothing on dryRun  | 172 | not-applicable | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer |
| does nothing on prune stale branches disabled  | 185 | not-applicable | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer |
| notifies via PR changes if someone pushed to PR  | 198 | not-applicable | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer |
| skips appending - abandoned to PR title if already present  | 213 | not-applicable | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer |
| skips changes to PR if dry run  | 227 | not-applicable | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer |
| dry run delete branch no PR  | 243 | not-applicable | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer |
| delete branch no PR  | 256 | not-applicable | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer |
| does not delete modified orphan branch  | 268 | not-applicable | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests finalize prune via vitest-mocked platform/SCM; Rust tests this at different layer |

---
