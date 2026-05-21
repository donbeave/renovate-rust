# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gerrit/scm.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gerrit/scm.spec.ts
**Total tests:** 29 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `isBranchBehindBase()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no open change for with branchname found -> isBehind == true | 29 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| open change found for branchname, rebase action is available -> isBehind == true | 46 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| open change found for branch name, but rebase action is not available -> isBehind == false | 65 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `isBranchModified()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no open change for with branchname found -> not modified | 84 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| open change found for branchname, but not modified | 101 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| open change found for branchname, but modified from other user | 116 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `isBranchConflicted()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no open change with branch name found -> return true | 133 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| open change found for branch name/baseBranch and its mergeable | 149 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| open change found for branch name/baseBranch and its NOT mergeable | 164 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `branchExists()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no change found for branch name -> return result from git.branchExists | 181 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| open change found for branch name -> return true | 196 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `getBranchCommit()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no change found for branch name -> return result from git.getBranchCommit | 207 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| open change found for branchname -> return true | 224 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `getBranchUpdateDate()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no change found for branch name -> return result from git.getBranchUpdateDate | 234 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| open change found for branchname -> return DateTime from Gerrit change | 258 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `pushForReview()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| pushes to refs/for/<targetBranch> and returns true on success | 280 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| adds hashtag push options for each label | 297 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| clears pending change branch on success | 320 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| keeps pending change branch when push fails | 333 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `deleteBranch()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes local branch | 348 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| clears pending change branch | 355 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `mergeToLocal()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no change exists | 363 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| uses local merge when there is a pending change branch | 383 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| change exists | 394 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `commitAndPush()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| commitAndPush() - empty commit | 424 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| commitAndPush() - create first commit but does not push | 448 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| commitAndPush() - existing change keeps original target branch | 482 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| commitAndPush() - existing change without new changes | 531 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| commitAndPush() - existing change with new changes - auto-approve | 575 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

---

