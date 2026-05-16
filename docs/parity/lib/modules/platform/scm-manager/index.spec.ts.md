# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/scm-manager/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/scm-manager/index.spec.ts
**Total tests:** 35 | **Ported:** 0 | **Actionable:** 35 | **Status:** pending

### `initPlatform`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw error, when endpoint is not configured | 77 | pending | — | — | — |
| should throw error, when token is not configured | 83 | pending | — | — | — |
| should throw error, when token is invalid | 89 | pending | — | — | — |
| should init platform | 97 | pending | — | — | — |

### `initRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should init repo | 107 | pending | — | — | — |

### `getRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return all available repos | 144 | pending | — | — | — |

### `getPrList`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty array, because no PR could be found | 169 | pending | — | — | — |
| should return empty array, because API request failed | 186 | pending | — | — | — |
| should return all PRs of a repo | 197 | pending | — | — | — |

### `findPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| search in Pull Request without explicitly setting the state as argument | 234 | pending | — | — | — |
| search within available pull requests for branch name "$branchName", pr title "$prTitle" and state "$state" with result $result | 256 | pending | — | — | — |

### `getBranchPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| search within available pull requests for branch name "$branchName" with result $result | 307 | pending | — | — | — |

### `getPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null, because PR was not found | 342 | pending | — | — | — |
| should return PR from cache | 364 | pending | — | — | — |
| should return fetched pr | 383 | pending | — | — | — |

### `createPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create PR with $draftPR and state $expectedState | 409 | pending | — | — | — |

### `updatePr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update PR with state $state and bdoy $body | 478 | pending | — | — | — |

### `mergePr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return false | 519 | pending | — | — | — |

### `getBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return red | 526 | pending | — | — | — |

### `setBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented | 533 | pending | — | — | — |

### `getBranchStatusCheck`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return null | 546 | pending | — | — | — |

### `addReviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented | 556 | pending | — | — | — |

### `addAssignees`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented | 564 | pending | — | — | — |

### `deleteLabel`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented | 572 | pending | — | — | — |

### `getIssueList`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return empty list | 578 | pending | — | — | — |

### `findIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return null | 585 | pending | — | — | — |

### `ensureIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return null | 592 | pending | — | — | — |

### `ensureIssueClosing`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented | 602 | pending | — | — | — |

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented | 610 | pending | — | — | — |

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented | 622 | pending | — | — | — |

### `massageMarkdown`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should adjust smart link for Pull Requests | 634 | pending | — | — | — |

### `getRepoForceRebase`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return false | 641 | pending | — | — | — |

### `getRawFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return null | 648 | pending | — | — | — |

### `getJsonFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return undefined | 655 | pending | — | — | — |

### `maxBodyLength`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return the max body length allowed for an SCM-Manager request body | 662 | pending | — | — | — |

---

