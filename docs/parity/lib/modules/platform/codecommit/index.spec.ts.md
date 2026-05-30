# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/codecommit/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/codecommit/index.spec.ts
**Total tests:** 58 | **Ported:** 0 | **Actionable:** 58 | **Status:** pending

### `massageMarkdown`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| validates massageMarkdown functionality | 66 | pending | — | — | — |
| replaces pr links | 75 | pending | — | — | — |
| replaces issue links | 84 | pending | — | — | — |
| maxBodyLength | 94 | pending | — | — | — |

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should init | 99 | pending | — | — | — |
| should init with env vars | 111 | pending | — | — | — |
| should | 123 | pending | — | — | — |
| should as well | 131 | pending | — | — | — |

### `initRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fails to git.initRepo | 139 | pending | — | — | — |
| fails on getRepositoryInfo | 155 | pending | — | — | — |
| getRepositoryInfo returns bad results | 164 | pending | — | — | — |
| getRepositoryInfo returns bad results 2 | 172 | pending | — | — | — |
| initiates repo successfully | 183 | pending | — | — | — |
| gets the right url | 202 | pending | — | — | — |
| gets the eu-central-1 url | 218 | pending | — | — | — |
| gets url with username and token | 233 | pending | — | — | — |

### `getRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns repos | 262 | pending | — | — | — |
| returns empty if error | 277 | pending | — | — | — |

### `getPrList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets PR list by author | 292 | pending | — | — | — |
| checks if nullcheck works for list prs | 336 | pending | — | — | — |

### `findPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws error on findPr | 352 | pending | — | — | — |
| finds pr | 365 | pending | — | — | — |
| finds any pr with that title in regardless of state | 396 | pending | — | — | — |
| finds closed/merged pr | 427 | pending | — | — | — |
| finds any pr | 458 | pending | — | — | — |
| returns empty list in case prs dont exist yet | 488 | pending | — | — | — |

### `getBranchPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| codecommit find PR for branch | 499 | pending | — | — | — |
| returns null if no PR for branch | 526 | pending | — | — | — |

### `getPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets pr | 549 | pending | — | — | — |
| gets closed pr | 576 | pending | — | — | — |
| gets merged pr | 602 | pending | — | — | — |
| returns null in case input is null | 631 | pending | — | — | — |

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns file content | 641 | pending | — | — | — |
| returns file content in json5 format | 651 | pending | — | — | — |
| returns null | 666 | pending | — | — | — |

### `getRawFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns file content | 676 | pending | — | — | — |
| returns null | 686 | pending | — | — | — |
| returns file content in json5 format | 694 | pending | — | — | — |

### `createPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| posts PR | 720 | pending | — | — | — |
| doesnt return a title | 755 | pending | — | — | — |

### `updatePr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates PR | 777 | pending | — | — | — |
| updates PR body if cache is not the same | 791 | pending | — | — | — |
| updates PR body does not update if cache is the same | 830 | pending | — | — | — |
| updates PR regardless of status failure | 868 | pending | — | — | — |
| updates PR with status closed | 884 | pending | — | — | — |

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds comment if missing | 905 | pending | — | — | — |
| updates comment if different content | 958 | pending | — | — | — |
| does nothing if comment exists and is the same | 992 | pending | — | — | — |
| does nothing if comment exists and is the same when there is no topic | 1025 | pending | — | — | — |
| throws an exception in case of api failed connection | 1058 | pending | — | — | — |
| fails at null check for response | 1074 | pending | — | — | — |
| doesnt find comments obj and source or destination commit | 1084 | pending | — | — | — |

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes comment by topic if found | 1125 | pending | — | — | — |
| doesnt find commentsForPullRequestData | 1157 | pending | — | — | — |
| doesnt find comment obj | 1171 | pending | — | — | — |
| deletes comment by content if found | 1197 | pending | — | — | — |
| throws exception in case failed api connection | 1229 | pending | — | — | — |

### `addReviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| checks that the function resolves | 1246 | pending | — | — | — |

---
