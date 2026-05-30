# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gerrit/scm.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gerrit/scm.spec.ts
**Total tests:** 33 | **Ported:** 0 | **Actionable:** 4 | **Status:** pending

### `isBranchBehindBase()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no open change for with branchname found -> isBehind == true | 29 | not-applicable | — | — | — |
| open change found for branchname, rebase action is available -> isBehind == true | 46 | not-applicable | — | — | — |
| open change found for branch name, but rebase action is not available -> isBehind == false | 65 | not-applicable | — | — | — |

### `isBranchModified()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no open change for with branchname found -> not modified | 84 | not-applicable | — | — | — |
| open change found for branchname, but not modified | 101 | not-applicable | — | — | — |
| open change found for branchname, but modified from other user | 116 | not-applicable | — | — | — |

### `isBranchConflicted()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no open change with branch name found -> return true | 133 | not-applicable | — | — | — |
| open change found for branch name/baseBranch and its mergeable | 149 | not-applicable | — | — | — |
| open change found for branch name/baseBranch and its NOT mergeable | 164 | not-applicable | — | — | — |

### `branchExists()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no change found for branch name -> return result from git.branchExists | 181 | not-applicable | — | — | — |
| open change found for branch name -> return true | 196 | not-applicable | — | — | — |

### `getBranchCommit()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no change found for branch name -> return result from git.getBranchCommit | 207 | not-applicable | — | — | — |
| open change found for branchname -> return true | 224 | not-applicable | — | — | — |

### `getBranchUpdateDate()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no change found for branch name -> return result from git.getBranchUpdateDate | 234 | not-applicable | — | — | — |
| open change found for branchname -> return DateTime from Gerrit change | 258 | not-applicable | — | — | — |

### `pushForReview()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| pushes to refs/for/<targetBranch> and returns true on success | 280 | not-applicable | — | — | — |
| adds hashtag push options for each label | 297 | not-applicable | — | — | — |
| clears pending change branch on success | 320 | not-applicable | — | — | — |
| keeps pending change branch when push fails | 333 | not-applicable | — | — | — |

### `deleteBranch()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes local branch | 348 | not-applicable | — | — | — |
| clears pending change branch | 355 | not-applicable | — | — | — |

### `mergeToLocal()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no change exists | 363 | not-applicable | — | — | — |
| uses local merge when there is a pending change branch | 383 | not-applicable | — | — | — |
| change exists | 394 | not-applicable | — | — | — |

### `commitAndPush()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| commitAndPush() - empty commit | 424 | not-applicable | — | — | — |
| commitAndPush() - create first commit but does not push | 448 | not-applicable | — | — | — |
| commitAndPush() - existing change keeps original target branch | 482 | not-applicable | — | — | — |
| commitAndPush() - existing change without new changes | 531 | not-applicable | — | — | — |
| commitAndPush() - existing change with new changes - auto-approve | 575 | not-applicable | — | — | — |

---

