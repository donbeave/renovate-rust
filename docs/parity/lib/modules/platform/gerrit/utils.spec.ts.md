# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gerrit/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gerrit/utils.spec.ts
**Total tests:** 23 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `getGerritRepoUrl() › no gitUrl provided`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| create a git url with username/password | 28 | not-applicable | — | — | — |
| create a git url without username/password | 37 | not-applicable | — | — | — |
| throws on invalid endpoint | 44 | not-applicable | — | — | — |

### `getGerritRepoUrl() › default gitUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| create a git url with username/password | 51 | not-applicable | — | — | — |

### `getGerritRepoUrl() › endpoint gitUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| create a git url with username/password | 61 | not-applicable | — | — | — |

### `getGerritRepoUrl() › ssh gitUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| create a simple url | 71 | not-applicable | — | — | — |
| create a url with trailing slash | 80 | not-applicable | — | — | — |
| create a url when base has context | 93 | not-applicable | — | — | — |

### `mapPrStateToGerritFilter()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| maps pr state %p to gerrit filter %p | 109 | not-applicable | — | — | — |

### `mapGerritChangeStateToPrState()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| maps gerrit change state %p to PrState %p | 125 | not-applicable | — | — | — |

### `mapGerritChangeToPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| map a gerrit change to to Pr | 139 | not-applicable | — | — | — |
| map a gerrit change without reviewers to Pr | 191 | not-applicable | — | — | — |
| does not map a gerrit change without source branch to Pr | 222 | not-applicable | — | — | — |
| does not reject a broken commit message if knownProperties.sourceBranch is passed | 240 | not-applicable | — | — | — |
| avoids iterating through change messages knownProperties.prBody is passed | 274 | not-applicable | — | — | — |

### `extractSourceBranch()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no commit message | 310 | not-applicable | — | — | — |
| commit message with no footer | 315 | not-applicable | — | — | — |
| commit message with footer | 327 | not-applicable | — | — | — |

### `findPullRequestBody()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| find pull-request-body | 342 | not-applicable | — | — | — |
| no pull-request-body message found | 364 | not-applicable | — | — | — |

### `mapBranchStatusToLabel()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Label with +1/-1 map branchState=%p to %p | 385 | not-applicable | — | — | — |
| Label with +2/-2, map branchState=%p to %p | 409 | not-applicable | — | — | — |

### `convertGerritDateToISO()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| converts Gerrit date format to ISO format | 424 | not-applicable | — | — | — |

---

