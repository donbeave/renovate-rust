# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gerrit/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gerrit/utils.spec.ts
**Total tests:** 23 | **Ported:** 0 | **Actionable:** 23 | **Status:** pending

### `getGerritRepoUrl() › no gitUrl provided`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| create a git url with username/password | 28 | pending | — | — | —|
| create a git url without username/password | 37 | pending | — | — | —|
| throws on invalid endpoint | 44 | pending | — | — | —|

### `getGerritRepoUrl() › default gitUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| create a git url with username/password | 51 | pending | — | — | —|

### `getGerritRepoUrl() › endpoint gitUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| create a git url with username/password | 61 | pending | — | — | —|

### `getGerritRepoUrl() › ssh gitUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| create a simple url | 71 | pending | — | — | —|
| create a url with trailing slash | 80 | pending | — | — | —|
| create a url when base has context | 93 | pending | — | — | —|

### `mapPrStateToGerritFilter()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| maps pr state %p to gerrit filter %p | 109 | pending | — | — | —|

### `mapGerritChangeStateToPrState()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| maps gerrit change state %p to PrState %p | 125 | pending | — | — | —|

### `mapGerritChangeToPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| map a gerrit change to to Pr | 139 | pending | — | — | —|
| map a gerrit change without reviewers to Pr | 191 | pending | — | — | —|
| does not map a gerrit change without source branch to Pr | 222 | pending | — | — | —|
| does not reject a broken commit message if knownProperties.sourceBranch is passed | 240 | pending | — | — | —|
| avoids iterating through change messages knownProperties.prBody is passed | 274 | pending | — | — | —|

### `extractSourceBranch()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no commit message | 310 | pending | — | — | —|
| commit message with no footer | 315 | pending | — | — | —|
| commit message with footer | 327 | pending | — | — | —|

### `findPullRequestBody()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| find pull-request-body | 342 | pending | — | — | —|
| no pull-request-body message found | 364 | pending | — | — | —|

### `mapBranchStatusToLabel()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Label with +1/-1 map branchState=%p to %p | 385 | pending | — | — | —|
| Label with +2/-2, map branchState=%p to %p | 409 | pending | — | — | —|

### `convertGerritDateToISO()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| converts Gerrit date format to ISO format | 424 | pending | — | — | —|

---

