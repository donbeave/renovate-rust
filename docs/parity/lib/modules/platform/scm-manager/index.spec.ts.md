# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/scm-manager/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/scm-manager/index.spec.ts
**Total tests:** 35 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `initPlatform`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw error, when endpoint is not configured | 77 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should throw error, when token is not configured | 83 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should throw error, when token is invalid | 89 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should init platform | 97 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `initRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should init repo | 107 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return all available repos | 144 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getPrList`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty array, because no PR could be found | 169 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should return empty array, because API request failed | 186 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should return all PRs of a repo | 197 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `findPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| search in Pull Request without explicitly setting the state as argument | 234 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| search within available pull requests for branch name "$branchName", pr title "$prTitle" and state "$state" with result $result | 256 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getBranchPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| search within available pull requests for branch name "$branchName" with result $result | 307 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null, because PR was not found | 342 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should return PR from cache | 364 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should return fetched pr | 383 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `createPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create PR with $draftPR and state $expectedState | 409 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `updatePr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update PR with state $state and bdoy $body | 478 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `mergePr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return false | 519 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return red | 526 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `setBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented | 533 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getBranchStatusCheck`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return null | 546 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `addReviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented | 556 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `addAssignees`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented | 564 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `deleteLabel`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented | 572 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getIssueList`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return empty list | 578 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `findIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return null | 585 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `ensureIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return null | 592 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `ensureIssueClosing`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented | 602 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented | 610 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented | 622 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `massageMarkdown`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should adjust smart link for Pull Requests | 634 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getRepoForceRebase`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return false | 641 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getRawFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return null | 648 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getJsonFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should Not implemented and return undefined | 655 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `maxBodyLength`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return the max body length allowed for an SCM-Manager request body | 662 | not-applicable | — | — | Helper/orchestration functions not implemented in Rust platform layer |

---

