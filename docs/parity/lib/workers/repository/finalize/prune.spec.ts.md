# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/finalize/prune.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/finalize/prune.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** not-applicable

### `workers/repository/finalize/prune › pruneStaleBranches()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no branchList | 24 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript branch pruning pipeline|
| ignores reconfigure branch | 30 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript branch pruning pipeline|
| returns if no defaultBranch | 36 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript branch pruning pipeline|
| returns if no renovate branches | 43 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript branch pruning pipeline|
| returns if no remaining branches | 51 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript branch pruning pipeline|
| renames deletes remaining branch | 59 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript branch pruning pipeline|
| skips rename but still deletes branch | 71 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript branch pruning pipeline|
| deletes with base branches | 87 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript branch pruning pipeline|
| uses single configured base branch instead of defaultBranch | 124 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript branch pruning pipeline|
| uses defaultBranch when baseBranchPatterns exist but baseBranches are not computed yet | 145 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript branch pruning pipeline|
| does nothing on dryRun | 172 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript branch pruning pipeline|
| does nothing on prune stale branches disabled | 185 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript branch pruning pipeline|
| notifies via PR changes if someone pushed to PR | 198 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript branch pruning pipeline|
| skips appending - abandoned to PR title if already present | 213 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript branch pruning pipeline|
| skips changes to PR if dry run | 227 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript branch pruning pipeline|
| dry run delete branch no PR | 243 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript branch pruning pipeline|
| delete branch no PR | 256 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript branch pruning pipeline|
| does not delete modified orphan branch | 268 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript branch pruning pipeline|

---
