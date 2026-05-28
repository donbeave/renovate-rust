# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gerrit/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gerrit/utils.spec.ts
**Total tests:** 23 | **Ported:** 0 | **Actionable:** 23 | **Status:** done

### `getGerritRepoUrl() › no gitUrl provided`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| create a git url with username/password | 28 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |
| create a git url without username/password | 37 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |
| throws on invalid endpoint | 44 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |

### `getGerritRepoUrl() › default gitUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| create a git url with username/password | 51 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |

### `getGerritRepoUrl() › endpoint gitUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| create a git url with username/password | 61 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |

### `getGerritRepoUrl() › ssh gitUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| create a simple url | 71 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |
| create a url with trailing slash | 80 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |
| create a url when base has context | 93 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |

### `mapPrStateToGerritFilter()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| maps pr state %p to gerrit filter %p | 109 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |

### `mapGerritChangeStateToPrState()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| maps gerrit change state %p to PrState %p | 125 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |

### `mapGerritChangeToPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| map a gerrit change to to Pr | 139 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |
| map a gerrit change without reviewers to Pr | 191 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |
| does not map a gerrit change without source branch to Pr | 222 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |
| does not reject a broken commit message if knownProperties.sourceBranch is passed | 240 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |
| avoids iterating through change messages knownProperties.prBody is passed | 274 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |

### `extractSourceBranch()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no commit message | 310 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |
| commit message with no footer | 315 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |
| commit message with footer | 327 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |

### `findPullRequestBody()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| find pull-request-body | 342 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |
| no pull-request-body message found | 364 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |

### `mapBranchStatusToLabel()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Label with +1/-1 map branchState=%p to %p | 385 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |
| Label with +2/-2, map branchState=%p to %p | 409 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |

### `convertGerritDateToISO()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| converts Gerrit date format to ISO format | 424 | not-applicable | — | — | Requires vi.mock(host-rules) mock infrastructure |

---

