# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/codecommit/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/codecommit/index.spec.ts
**Total tests:** 58 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `massageMarkdown`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| validates massageMarkdown functionality | 66 | not-applicable | — | — | — |
| replaces pr links | 75 | not-applicable | — | — | — |
| replaces issue links | 84 | not-applicable | — | — | — |
| maxBodyLength | 94 | not-applicable | — | — | — |

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should init | 99 | not-applicable | — | — | — |
| should init with env vars | 111 | not-applicable | — | — | — |
| should | 123 | not-applicable | — | — | — |
| should as well | 131 | not-applicable | — | — | — |

### `initRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fails to git.initRepo | 139 | not-applicable | — | — | — |
| fails on getRepositoryInfo | 155 | not-applicable | — | — | — |
| getRepositoryInfo returns bad results | 164 | not-applicable | — | — | — |
| getRepositoryInfo returns bad results 2 | 172 | not-applicable | — | — | — |
| initiates repo successfully | 183 | not-applicable | — | — | — |
| gets the right url | 202 | not-applicable | — | — | — |
| gets the eu-central-1 url | 218 | not-applicable | — | — | — |
| gets url with username and token | 233 | not-applicable | — | — | — |

### `getRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns repos | 262 | not-applicable | — | — | — |
| returns empty if error | 277 | not-applicable | — | — | — |

### `getPrList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets PR list by author | 292 | not-applicable | — | — | — |
| checks if nullcheck works for list prs | 336 | not-applicable | — | — | — |

### `findPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws error on findPr | 352 | not-applicable | — | — | — |
| finds pr | 365 | not-applicable | — | — | — |
| finds any pr with that title in regardless of state | 396 | not-applicable | — | — | — |
| finds closed/merged pr | 427 | not-applicable | — | — | — |
| finds any pr | 458 | not-applicable | — | — | — |
| returns empty list in case prs dont exist yet | 488 | not-applicable | — | — | — |

### `getBranchPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| codecommit find PR for branch | 499 | not-applicable | — | — | — |
| returns null if no PR for branch | 526 | not-applicable | — | — | — |

### `getPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets pr | 549 | not-applicable | — | — | — |
| gets closed pr | 576 | not-applicable | — | — | — |
| gets merged pr | 602 | not-applicable | — | — | — |
| returns null in case input is null | 631 | not-applicable | — | — | — |

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns file content | 641 | not-applicable | — | — | — |
| returns file content in json5 format | 651 | not-applicable | — | — | — |
| returns null | 666 | not-applicable | — | — | — |

### `getRawFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns file content | 676 | not-applicable | — | — | — |
| returns null | 686 | not-applicable | — | — | — |
| returns file content in json5 format | 694 | not-applicable | — | — | — |

### `createPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| posts PR | 720 | not-applicable | — | — | — |
| doesnt return a title | 755 | not-applicable | — | — | — |

### `updatePr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates PR | 777 | not-applicable | — | — | — |
| updates PR body if cache is not the same | 791 | not-applicable | — | — | — |
| updates PR body does not update if cache is the same | 830 | not-applicable | — | — | — |
| updates PR regardless of status failure | 868 | not-applicable | — | — | — |
| updates PR with status closed | 884 | not-applicable | — | — | — |

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds comment if missing | 905 | not-applicable | — | — | — |
| updates comment if different content | 958 | not-applicable | — | — | — |
| does nothing if comment exists and is the same | 992 | not-applicable | — | — | — |
| does nothing if comment exists and is the same when there is no topic | 1025 | not-applicable | — | — | — |
| throws an exception in case of api failed connection | 1058 | not-applicable | — | — | — |
| fails at null check for response | 1074 | not-applicable | — | — | — |
| doesnt find comments obj and source or destination commit | 1084 | not-applicable | — | — | — |

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes comment by topic if found | 1125 | not-applicable | — | — | — |
| doesnt find commentsForPullRequestData | 1157 | not-applicable | — | — | — |
| doesnt find comment obj | 1171 | not-applicable | — | — | — |
| deletes comment by content if found | 1197 | not-applicable | — | — | — |
| throws exception in case failed api connection | 1229 | not-applicable | — | — | — |

### `addReviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| checks that the function resolves | 1246 | not-applicable | — | — | — |

---
