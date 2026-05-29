# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/codecommit/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/codecommit/index.spec.ts
**Total tests:** 58 | **Ported:** 0 | **Actionable:** 58 | **Status:** not-applicable

### `massageMarkdown`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| validates massageMarkdown functionality | 66 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| replaces pr links | 75 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| replaces issue links | 84 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| maxBodyLength | 94 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should init | 99 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| should init with env vars | 111 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| should | 123 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| should as well | 131 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|

### `initRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fails to git.initRepo | 139 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| fails on getRepositoryInfo | 155 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| getRepositoryInfo returns bad results | 164 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| getRepositoryInfo returns bad results 2 | 172 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| initiates repo successfully | 183 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| gets the right url | 202 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| gets the eu-central-1 url | 218 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| gets url with username and token | 233 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|

### `getRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns repos | 262 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| returns empty if error | 277 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|

### `getPrList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets PR list by author | 292 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| checks if nullcheck works for list prs | 336 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|

### `findPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws error on findPr | 352 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| finds pr | 365 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| finds any pr with that title in regardless of state | 396 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| finds closed/merged pr | 427 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| finds any pr | 458 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| returns empty list in case prs dont exist yet | 488 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|

### `getBranchPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| codecommit find PR for branch | 499 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| returns null if no PR for branch | 526 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|

### `getPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets pr | 549 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| gets closed pr | 576 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| gets merged pr | 602 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| returns null in case input is null | 631 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns file content | 641 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| returns file content in json5 format | 651 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| returns null | 666 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|

### `getRawFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns file content | 676 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| returns null | 686 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| returns file content in json5 format | 694 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|

### `createPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| posts PR | 720 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| doesnt return a title | 755 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|

### `updatePr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates PR | 777 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| updates PR body if cache is not the same | 791 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| updates PR body does not update if cache is the same | 830 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| updates PR regardless of status failure | 868 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| updates PR with status closed | 884 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds comment if missing | 905 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| updates comment if different content | 958 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| does nothing if comment exists and is the same | 992 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| does nothing if comment exists and is the same when there is no topic | 1025 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| throws an exception in case of api failed connection | 1058 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| fails at null check for response | 1074 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| doesnt find comments obj and source or destination commit | 1084 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes comment by topic if found | 1125 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| doesnt find commentsForPullRequestData | 1157 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| doesnt find comment obj | 1171 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| deletes comment by content if found | 1197 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|
| throws exception in case failed api connection | 1229 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|

### `addReviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| checks that the function resolves | 1246 | not-applicable | — | — | TS-library-specific; uses aws-sdk-client-mock to mock CodeCommit client; TypeScript AWS SDK mock pipeline|

---
