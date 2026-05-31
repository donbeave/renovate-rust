# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/scm-manager/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/scm-manager/index.spec.ts
**Total tests:** 35 | **Ported:** 0 | **Actionable:** 35 | **Status:** pending

### `initPlatform`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw error, when endpoint is not configured | 77 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should throw error, when token is not configured | 83 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should throw error, when token is invalid | 89 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should init platform | 97 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `initRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should init repo | 107 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return all available repos | 144 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getPrList`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty array, because no PR could be found | 169 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should return empty array, because API request failed | 186 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should return all PRs of a repo | 197 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `findPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| search in Pull Request without explicitly setting the state as argument | 234 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| search within available pull requests for branch name "$branchName", pr title "$prTitle" and state "$state" with result $result | 256 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getBranchPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| search within available pull requests for branch name "$branchName" with result $result | 307 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null, because PR was not found | 342 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should return PR from cache | 364 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should return fetched pr | 383 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `createPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create PR with $draftPR and state $expectedState | 409 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `updatePr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update PR with state $state and bdoy $body | 478 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `mergePr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return false | 519 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return red | 526 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `setBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented | 533 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getBranchStatusCheck`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return null | 546 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `addReviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented | 556 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `addAssignees`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented | 564 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `deleteLabel`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented | 572 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getIssueList`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return empty list | 578 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `findIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return null | 585 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `ensureIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return null | 592 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `ensureIssueClosing`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented | 602 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented | 610 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented | 622 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `massageMarkdown`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should adjust smart link for Pull Requests | 634 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getRepoForceRebase`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return false | 641 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getRawFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return null | 648 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getJsonFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return undefined | 655 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `maxBodyLength`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return the max body length allowed for an SCM-Manager request body | 662 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

---

