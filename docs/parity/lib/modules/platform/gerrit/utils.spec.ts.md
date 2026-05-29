# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gerrit/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gerrit/utils.spec.ts
**Total tests:** 23 | **Ported:** 0 | **Actionable:** 23 | **Status:** not-applicable

### `getGerritRepoUrl() › no gitUrl provided`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| create a git url with username/password | 28 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|
| create a git url without username/password | 37 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|
| throws on invalid endpoint | 44 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|

### `getGerritRepoUrl() › default gitUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| create a git url with username/password | 51 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|

### `getGerritRepoUrl() › endpoint gitUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| create a git url with username/password | 61 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|

### `getGerritRepoUrl() › ssh gitUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| create a simple url | 71 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|
| create a url with trailing slash | 80 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|
| create a url when base has context | 93 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|

### `mapPrStateToGerritFilter()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| maps pr state %p to gerrit filter %p | 109 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|

### `mapGerritChangeStateToPrState()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| maps gerrit change state %p to PrState %p | 125 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|

### `mapGerritChangeToPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| map a gerrit change to to Pr | 139 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|
| map a gerrit change without reviewers to Pr | 191 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|
| does not map a gerrit change without source branch to Pr | 222 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|
| does not reject a broken commit message if knownProperties.sourceBranch is passed | 240 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|
| avoids iterating through change messages knownProperties.prBody is passed | 274 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|

### `extractSourceBranch()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no commit message | 310 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|
| commit message with no footer | 315 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|
| commit message with footer | 327 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|

### `findPullRequestBody()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| find pull-request-body | 342 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|
| no pull-request-body message found | 364 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|

### `mapBranchStatusToLabel()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Label with +1/-1 map branchState=%p to %p | 385 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|
| Label with +2/-2, map branchState=%p to %p | 409 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|

### `convertGerritDateToISO()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| converts Gerrit date format to ISO format | 424 | not-applicable | — | — | mocking framework internals — vi.mock on host-rules; TypeScript Gerrit utility functions with mocked auth|

---

