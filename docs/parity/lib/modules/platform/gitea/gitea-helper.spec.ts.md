# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gitea/gitea-helper.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gitea/gitea-helper.spec.ts
**Total tests:** 39 | **Ported:** 0 | **Actionable:** 39 | **Status:** pending

### `getCurrentUser`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/user endpoint | 199 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getVersion`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/version endpoint | 208 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `searchRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/search endpoint | 219 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should construct proper query parameters | 232 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should abort if ok flag was not set | 248 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `orgListRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/orgs/[organization]/repos endpoint | 259 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo] endpoint | 268 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getRepoContents`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/contents/[file] endpoint | 280 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should support passing reference by query | 292 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should properly escape paths | 308 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should not fail if no content is returned | 323 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `createPR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls endpoint | 343 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `updatePR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull] endpoint | 363 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `closePR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull] endpoint | 388 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `mergePR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull]/merge endpoint | 399 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getPR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull] endpoint | 414 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getPRByBranch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[base]/[head] endpoint | 426 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should return null if pr not found | 442 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should log error | 458 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `addReviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull]/requested_reviewers endpoint | 483 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `createIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues endpoint | 498 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `updateIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue] endpoint | 515 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `updateIssueLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue]/labels endpoint | 540 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `closeIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue] endpoint | 563 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `searchIssues`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues endpoint | 575 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should construct proper query parameters | 585 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue] endpoint | 599 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getRepoLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/labels endpoint | 611 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getOrgLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/orgs/[org]/labels endpoint | 623 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `unassignLabel`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue]/labels/[label] endpoint | 635 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `createComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue]/comments endpoint | 650 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `updateComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/comments/[comment] endpoint | 668 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `deleteComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/comments/[comment] endpoint | 689 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getComments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue]/comments endpoint | 703 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `createCommitStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/statuses/[commit] endpoint | 715 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getCombinedCommitStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/commits/[branch]/statuses endpoint | 732 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should properly determine worst commit status | 746 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getBranch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/branches/[branch] endpoint | 819 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should properly escape branch names | 829 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

---

