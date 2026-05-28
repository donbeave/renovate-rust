# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/forgejo/forgejo-helper.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/forgejo/forgejo-helper.spec.ts
**Total tests:** 40 | **Ported:** 0 | **Actionable:** 40 | **Status:** done

### `getCurrentUser`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/user endpoint | 200 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getVersion`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/version endpoint | 209 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `isOrg`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/orgs/[org] endpoint | 220 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `searchRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/search endpoint | 238 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should construct proper query parameters | 251 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should abort if ok flag was not set | 267 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `orgListRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/orgs/[organization]/repos endpoint | 278 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo] endpoint | 287 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getRepoContents`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/contents/[file] endpoint | 299 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should support passing reference by query | 311 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should properly escape paths | 327 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should not fail if no content is returned | 342 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `createPR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls endpoint | 362 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `updatePR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull] endpoint | 382 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `closePR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull] endpoint | 407 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `mergePR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull]/merge endpoint | 418 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getPR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull] endpoint | 433 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getPRByBranch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[base]/[head] endpoint | 445 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should return null if pr not found | 461 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should log error | 477 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `addReviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull]/requested_reviewers endpoint | 502 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `createIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues endpoint | 517 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `updateIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue] endpoint | 534 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `updateIssueLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue]/labels endpoint | 559 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `closeIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue] endpoint | 582 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `searchIssues`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues endpoint | 594 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should construct proper query parameters | 604 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue] endpoint | 618 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getRepoLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/labels endpoint | 630 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getOrgLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/orgs/[org]/labels endpoint | 642 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `unassignLabel`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue]/labels/[label] endpoint | 654 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `createComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue]/comments endpoint | 669 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `updateComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/comments/[comment] endpoint | 687 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `deleteComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/comments/[comment] endpoint | 708 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getComments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue]/comments endpoint | 722 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `createCommitStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/statuses/[commit] endpoint | 734 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getCombinedCommitStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/commits/[branch]/statuses endpoint | 751 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should properly determine worst commit status | 765 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getBranch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/branches/[branch] endpoint | 838 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should properly escape branch names | 848 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

---

