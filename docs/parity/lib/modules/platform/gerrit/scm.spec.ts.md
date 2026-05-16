# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gerrit/scm.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gerrit/scm.spec.ts
**Total tests:** 29 | **Ported:** 0 | **Actionable:** 29 | **Status:** pending

### `isBranchBehindBase()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no open change for with branchname found -> isBehind == true | 29 | pending | — | — | — |
| open change found for branchname, rebase action is available -> isBehind == true | 46 | pending | — | — | — |
| open change found for branch name, but rebase action is not available -> isBehind == false | 65 | pending | — | — | — |

### `isBranchModified()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no open change for with branchname found -> not modified | 84 | pending | — | — | — |
| open change found for branchname, but not modified | 101 | pending | — | — | — |
| open change found for branchname, but modified from other user | 116 | pending | — | — | — |

### `isBranchConflicted()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no open change with branch name found -> return true | 133 | pending | — | — | — |
| open change found for branch name/baseBranch and its mergeable | 149 | pending | — | — | — |
| open change found for branch name/baseBranch and its NOT mergeable | 164 | pending | — | — | — |

### `branchExists()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no change found for branch name -> return result from git.branchExists | 181 | pending | — | — | — |
| open change found for branch name -> return true | 196 | pending | — | — | — |

### `getBranchCommit()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no change found for branch name -> return result from git.getBranchCommit | 207 | pending | — | — | — |
| open change found for branchname -> return true | 224 | pending | — | — | — |

### `getBranchUpdateDate()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no change found for branch name -> return result from git.getBranchUpdateDate | 234 | pending | — | — | — |
| open change found for branchname -> return DateTime from Gerrit change | 258 | pending | — | — | — |

### `pushForReview()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| pushes to refs/for/<targetBranch> and returns true on success | 280 | pending | — | — | — |
| adds hashtag push options for each label | 297 | pending | — | — | — |
| clears pending change branch on success | 320 | pending | — | — | — |
| keeps pending change branch when push fails | 333 | pending | — | — | — |

### `deleteBranch()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes local branch | 348 | pending | — | — | — |
| clears pending change branch | 355 | pending | — | — | — |

### `mergeToLocal()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no change exists | 363 | pending | — | — | — |
| uses local merge when there is a pending change branch | 383 | pending | — | — | — |
| change exists | 394 | pending | — | — | — |

### `commitAndPush()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| commitAndPush() - empty commit | 424 | pending | — | — | — |
| commitAndPush() - create first commit but does not push | 448 | pending | — | — | — |
| commitAndPush() - existing change keeps original target branch | 482 | pending | — | — | — |
| commitAndPush() - existing change without new changes | 531 | pending | — | — | — |
| commitAndPush() - existing change with new changes - auto-approve | 575 | pending | — | — | — |

---

