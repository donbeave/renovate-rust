# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gerrit/scm.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gerrit/scm.spec.ts
**Total tests:** 29 | **Ported:** 0 | **Actionable:** 29 | **Status:** done

### `isBranchBehindBase()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no open change for with branchname found -> isBehind == true | 29 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |
| open change found for branchname, rebase action is available -> isBehind == true | 46 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |
| open change found for branch name, but rebase action is not available -> isBehind == false | 65 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |

### `isBranchModified()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no open change for with branchname found -> not modified | 84 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |
| open change found for branchname, but not modified | 101 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |
| open change found for branchname, but modified from other user | 116 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |

### `isBranchConflicted()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no open change with branch name found -> return true | 133 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |
| open change found for branch name/baseBranch and its mergeable | 149 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |
| open change found for branch name/baseBranch and its NOT mergeable | 164 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |

### `branchExists()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no change found for branch name -> return result from git.branchExists | 181 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |
| open change found for branch name -> return true | 196 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |

### `getBranchCommit()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no change found for branch name -> return result from git.getBranchCommit | 207 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |
| open change found for branchname -> return true | 224 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |

### `getBranchUpdateDate()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no change found for branch name -> return result from git.getBranchUpdateDate | 234 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |
| open change found for branchname -> return DateTime from Gerrit change | 258 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |

### `pushForReview()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| pushes to refs/for/<targetBranch> and returns true on success | 280 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |
| adds hashtag push options for each label | 297 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |
| clears pending change branch on success | 320 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |
| keeps pending change branch when push fails | 333 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |

### `deleteBranch()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes local branch | 348 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |
| clears pending change branch | 355 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |

### `mergeToLocal()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no change exists | 363 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |
| uses local merge when there is a pending change branch | 383 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |
| change exists | 394 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |

### `commitAndPush()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| commitAndPush() - empty commit | 424 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |
| commitAndPush() - create first commit but does not push | 448 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |
| commitAndPush() - existing change keeps original target branch | 482 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |
| commitAndPush() - existing change without new changes | 531 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |
| commitAndPush() - existing change with new changes - auto-approve | 575 | not-applicable | — | — | Requires git mock + gerrit client mock infrastructure |

---

