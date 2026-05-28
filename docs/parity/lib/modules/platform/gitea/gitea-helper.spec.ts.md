# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gitea/gitea-helper.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gitea/gitea-helper.spec.ts
**Total tests:** 39 | **Ported:** 0 | **Actionable:** 39 | **Status:** done

### `getCurrentUser`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/user endpoint | 199 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getVersion`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/version endpoint | 208 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `searchRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/search endpoint | 219 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should construct proper query parameters | 232 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should abort if ok flag was not set | 248 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `orgListRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/orgs/[organization]/repos endpoint | 259 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo] endpoint | 268 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getRepoContents`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/contents/[file] endpoint | 280 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should support passing reference by query | 292 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should properly escape paths | 308 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should not fail if no content is returned | 323 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `createPR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls endpoint | 343 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `updatePR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull] endpoint | 363 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `closePR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull] endpoint | 388 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `mergePR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull]/merge endpoint | 399 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getPR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull] endpoint | 414 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getPRByBranch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[base]/[head] endpoint | 426 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should return null if pr not found | 442 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should log error | 458 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `addReviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull]/requested_reviewers endpoint | 483 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `createIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues endpoint | 498 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `updateIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue] endpoint | 515 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `updateIssueLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue]/labels endpoint | 540 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `closeIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue] endpoint | 563 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `searchIssues`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues endpoint | 575 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should construct proper query parameters | 585 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue] endpoint | 599 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getRepoLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/labels endpoint | 611 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getOrgLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/orgs/[org]/labels endpoint | 623 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `unassignLabel`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue]/labels/[label] endpoint | 635 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `createComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue]/comments endpoint | 650 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `updateComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/comments/[comment] endpoint | 668 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `deleteComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/comments/[comment] endpoint | 689 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getComments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue]/comments endpoint | 703 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `createCommitStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/statuses/[commit] endpoint | 715 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getCombinedCommitStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/commits/[branch]/statuses endpoint | 732 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should properly determine worst commit status | 746 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getBranch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/branches/[branch] endpoint | 819 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should properly escape branch names | 829 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

---

