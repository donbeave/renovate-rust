# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/write.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/write.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 16 | **Status:** not-applicable

### `workers/repository/process/write › writeUpdates()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| stops after automerge | 48 | not-applicable | — | — | tests write-back of update results via git/platform operations; requires git2 + platform layer |
| increments branch counter | 106 | not-applicable | — | — | tests write-back of update results via git/platform operations; requires git2 + platform layer |
| return no-work if branch fingerprint is not different | 147 | not-applicable | — | — | tests write-back of update results via git/platform operations; requires git2 + platform layer |
| updates branch fingerprint when new commit is made | 176 | not-applicable | — | — | tests write-back of update results via git/platform operations; requires git2 + platform layer |
| caches same fingerprint when no commit is made and branch cache existed | 219 | not-applicable | — | — | tests write-back of update results via git/platform operations; requires git2 + platform layer |
| caches same fingerprint when no commit is made | 264 | not-applicable | — | — | tests write-back of update results via git/platform operations; requires git2 + platform layer |
| creates new branchCache when cache is not enabled | 306 | not-applicable | — | — | tests write-back of update results via git/platform operations; requires git2 + platform layer |

### `workers/repository/process/write › canSkipBranchUpdateCheck()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if no cache | 357 | not-applicable | — | — | tests write-back of update results via git/platform operations; requires git2 + platform layer |
| returns false when fingerprints are not same | 368 | not-applicable | — | — | tests write-back of update results via git/platform operations; requires git2 + platform layer |
| returns true | 378 | not-applicable | — | — | tests write-back of update results via git/platform operations; requires git2 + platform layer |

### `workers/repository/process/write › syncBranchState()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates minimal branch state when cache is not populated | 390 | not-applicable | — | — | tests write-back of update results via git/platform operations; requires git2 + platform layer |
| when base branch name is different updates it and invalidates related cache | 405 | not-applicable | — | — | tests write-back of update results via git/platform operations; requires git2 + platform layer |
| when base branch sha is different updates it and invalidates related values | 438 | not-applicable | — | — | tests write-back of update results via git/platform operations; requires git2 + platform layer |
| when branch sha is different updates it and invalidates related values | 473 | not-applicable | — | — | tests write-back of update results via git/platform operations; requires git2 + platform layer |
| when branch sha is different updates it and sets commitTimestamp | 509 | not-applicable | — | — | tests write-back of update results via git/platform operations; requires git2 + platform layer |
| no change if all parameters are same | 548 | not-applicable | — | — | tests write-back of update results via git/platform operations; requires git2 + platform layer |

---

