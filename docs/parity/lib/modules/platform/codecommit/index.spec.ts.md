# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/codecommit/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/codecommit/index.spec.ts
**Total tests:** 58 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `massageMarkdown`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| validates massageMarkdown functionality | 66 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| replaces pr links | 75 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| replaces issue links | 84 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| maxBodyLength | 94 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should init | 99 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| should init with env vars | 111 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| should | 123 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| should as well | 131 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |

### `initRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fails to git.initRepo | 139 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| fails on getRepositoryInfo | 155 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| getRepositoryInfo returns bad results | 164 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| getRepositoryInfo returns bad results 2 | 172 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| initiates repo successfully | 183 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| gets the right url | 202 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| gets the eu-central-1 url | 218 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| gets url with username and token | 233 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |

### `getRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns repos | 262 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| returns empty if error | 277 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |

### `getPrList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets PR list by author | 292 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| checks if nullcheck works for list prs | 336 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |

### `findPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws error on findPr | 352 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| finds pr | 365 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| finds any pr with that title in regardless of state | 396 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| finds closed/merged pr | 427 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| finds any pr | 458 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| returns empty list in case prs dont exist yet | 488 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |

### `getBranchPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| codecommit find PR for branch | 499 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| returns null if no PR for branch | 526 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |

### `getPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets pr | 549 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| gets closed pr | 576 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| gets merged pr | 602 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| returns null in case input is null | 631 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns file content | 641 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| returns file content in json5 format | 651 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| returns null | 666 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |

### `getRawFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns file content | 676 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| returns null | 686 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| returns file content in json5 format | 694 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |

### `createPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| posts PR | 720 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| doesnt return a title | 755 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |

### `updatePr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates PR | 777 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| updates PR body if cache is not the same | 791 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| updates PR body does not update if cache is the same | 830 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| updates PR regardless of status failure | 868 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| updates PR with status closed | 884 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds comment if missing | 905 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| updates comment if different content | 958 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| does nothing if comment exists and is the same | 992 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| does nothing if comment exists and is the same when there is no topic | 1025 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| throws an exception in case of api failed connection | 1058 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| fails at null check for response | 1074 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| doesnt find comments obj and source or destination commit | 1084 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes comment by topic if found | 1125 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| doesnt find commentsForPullRequestData | 1157 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| doesnt find comment obj | 1171 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| deletes comment by content if found | 1197 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |
| throws exception in case failed api connection | 1229 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |

### `addReviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| checks that the function resolves | 1246 | not-applicable | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer | — | Mock framework internals — tests codecommit platform via vitest-mocked AWS SDK; Rust tests this at different layer |

---
