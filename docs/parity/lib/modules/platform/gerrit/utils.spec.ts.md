# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gerrit/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gerrit/utils.spec.ts
**Total tests:** 23 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `getGerritRepoUrl() › no gitUrl provided`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| create a git url with username/password | 28 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| create a git url without username/password | 37 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| throws on invalid endpoint | 44 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `getGerritRepoUrl() › default gitUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| create a git url with username/password | 51 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `getGerritRepoUrl() › endpoint gitUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| create a git url with username/password | 61 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `getGerritRepoUrl() › ssh gitUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| create a simple url | 71 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| create a url with trailing slash | 80 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| create a url when base has context | 93 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `mapPrStateToGerritFilter()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| maps pr state %p to gerrit filter %p | 109 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `mapGerritChangeStateToPrState()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| maps gerrit change state %p to PrState %p | 125 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `mapGerritChangeToPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| map a gerrit change to to Pr | 139 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| map a gerrit change without reviewers to Pr | 191 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| does not map a gerrit change without source branch to Pr | 222 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| does not reject a broken commit message if knownProperties.sourceBranch is passed | 240 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| avoids iterating through change messages knownProperties.prBody is passed | 274 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `extractSourceBranch()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no commit message | 310 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| commit message with no footer | 315 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| commit message with footer | 327 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `findPullRequestBody()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| find pull-request-body | 342 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| no pull-request-body message found | 364 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `mapBranchStatusToLabel()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Label with +1/-1 map branchState=%p to %p | 385 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| Label with +2/-2, map branchState=%p to %p | 409 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `convertGerritDateToISO()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| converts Gerrit date format to ISO format | 424 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

---

