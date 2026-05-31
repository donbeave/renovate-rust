# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/forgejo/forgejo-helper.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/forgejo/forgejo-helper.spec.ts
**Total tests:** 40 | **Ported:** 0 | **Actionable:** 40 | **Status:** pending

### `getCurrentUser`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/user endpoint | 200 | pending | — | — | No corresponding Rust source|

### `getVersion`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/version endpoint | 209 | pending | — | — | No corresponding Rust source|

### `isOrg`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/orgs/[org] endpoint | 220 | pending | — | — | No corresponding Rust source|

### `searchRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/search endpoint | 238 | pending | — | — | No corresponding Rust source|
| should construct proper query parameters | 251 | pending | — | — | No corresponding Rust source|
| should abort if ok flag was not set | 267 | pending | — | — | No corresponding Rust source|

### `orgListRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/orgs/[organization]/repos endpoint | 278 | pending | — | — | No corresponding Rust source|

### `getRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo] endpoint | 287 | pending | — | — | No corresponding Rust source|

### `getRepoContents`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/contents/[file] endpoint | 299 | pending | — | — | No corresponding Rust source|
| should support passing reference by query | 311 | pending | — | — | No corresponding Rust source|
| should properly escape paths | 327 | pending | — | — | No corresponding Rust source|
| should not fail if no content is returned | 342 | pending | — | — | No corresponding Rust source|

### `createPR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls endpoint | 362 | pending | — | — | No corresponding Rust source|

### `updatePR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull] endpoint | 382 | pending | — | — | No corresponding Rust source|

### `closePR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull] endpoint | 407 | pending | — | — | No corresponding Rust source|

### `mergePR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull]/merge endpoint | 418 | pending | — | — | No corresponding Rust source|

### `getPR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull] endpoint | 433 | pending | — | — | No corresponding Rust source|

### `getPRByBranch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[base]/[head] endpoint | 445 | pending | — | — | No corresponding Rust source|
| should return null if pr not found | 461 | pending | — | — | No corresponding Rust source|
| should log error | 477 | pending | — | — | No corresponding Rust source|

### `addReviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull]/requested_reviewers endpoint | 502 | pending | — | — | No corresponding Rust source|

### `createIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues endpoint | 517 | pending | — | — | No corresponding Rust source|

### `updateIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue] endpoint | 534 | pending | — | — | No corresponding Rust source|

### `updateIssueLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue]/labels endpoint | 559 | pending | — | — | No corresponding Rust source|

### `closeIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue] endpoint | 582 | pending | — | — | No corresponding Rust source|

### `searchIssues`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues endpoint | 594 | pending | — | — | No corresponding Rust source|
| should construct proper query parameters | 604 | pending | — | — | No corresponding Rust source|

### `getIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue] endpoint | 618 | pending | — | — | No corresponding Rust source|

### `getRepoLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/labels endpoint | 630 | pending | — | — | No corresponding Rust source|

### `getOrgLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/orgs/[org]/labels endpoint | 642 | pending | — | — | No corresponding Rust source|

### `unassignLabel`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue]/labels/[label] endpoint | 654 | pending | — | — | No corresponding Rust source|

### `createComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue]/comments endpoint | 669 | pending | — | — | No corresponding Rust source|

### `updateComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/comments/[comment] endpoint | 687 | pending | — | — | No corresponding Rust source|

### `deleteComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/comments/[comment] endpoint | 708 | pending | — | — | No corresponding Rust source|

### `getComments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue]/comments endpoint | 722 | pending | — | — | No corresponding Rust source|

### `createCommitStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/statuses/[commit] endpoint | 734 | pending | — | — | No corresponding Rust source|

### `getCombinedCommitStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/commits/[branch]/statuses endpoint | 751 | pending | — | — | No corresponding Rust source|
| should properly determine worst commit status | 765 | pending | — | — | No corresponding Rust source|

### `getBranch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/branches/[branch] endpoint | 838 | pending | — | — | No corresponding Rust source|
| should properly escape branch names | 848 | pending | — | — | No corresponding Rust source|

---

