# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/forgejo/forgejo-helper.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/forgejo/forgejo-helper.spec.ts
**Total tests:** 40 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `getCurrentUser`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/user endpoint | 200 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `getVersion`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/version endpoint | 209 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `isOrg`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/orgs/[org] endpoint | 220 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `searchRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/search endpoint | 238 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should construct proper query parameters | 251 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should abort if ok flag was not set | 267 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `orgListRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/orgs/[organization]/repos endpoint | 278 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `getRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo] endpoint | 287 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `getRepoContents`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/contents/[file] endpoint | 299 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should support passing reference by query | 311 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should properly escape paths | 327 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should not fail if no content is returned | 342 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `createPR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls endpoint | 362 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `updatePR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull] endpoint | 382 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `closePR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull] endpoint | 407 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `mergePR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull]/merge endpoint | 418 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `getPR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull] endpoint | 433 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `getPRByBranch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[base]/[head] endpoint | 445 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should return null if pr not found | 461 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should log error | 477 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `addReviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/pulls/[pull]/requested_reviewers endpoint | 502 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `createIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues endpoint | 517 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `updateIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue] endpoint | 534 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `updateIssueLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue]/labels endpoint | 559 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `closeIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue] endpoint | 582 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `searchIssues`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues endpoint | 594 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should construct proper query parameters | 604 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `getIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue] endpoint | 618 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `getRepoLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/labels endpoint | 630 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `getOrgLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/orgs/[org]/labels endpoint | 642 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `unassignLabel`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue]/labels/[label] endpoint | 654 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `createComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue]/comments endpoint | 669 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `updateComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/comments/[comment] endpoint | 687 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `deleteComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/comments/[comment] endpoint | 708 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `getComments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/issues/[issue]/comments endpoint | 722 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `createCommitStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/statuses/[commit] endpoint | 734 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `getCombinedCommitStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/commits/[branch]/statuses endpoint | 751 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should properly determine worst commit status | 765 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `getBranch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call /api/v1/repos/[repo]/branches/[branch] endpoint | 838 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should properly escape branch names | 848 | not-applicable | Mock framework internals — tests forgejo helper via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

---

