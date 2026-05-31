# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/azure/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/azure/index.spec.ts
**Total tests:** 79 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no endpoint | 102 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw if no token nor a username and password | 107 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw if a username but no password | 116 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw if a password but no username | 126 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should init | 136 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return an array of repos | 147 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `initRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should initialise the config for a repo | 201 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws if repo is disabled | 209 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws if repo is not in repos list | 217 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `findPr(branchName, prTitle, state, targetBranch)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns pr if found it open | 227 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns pr if found not open | 269 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns pr if found it close | 311 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns pr if found it all state | 353 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns pr if found matches targetBranch | 394 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns first pr if found does not match targetBranch | 442 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| catches errors | 490 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getPrList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty array | 505 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getBranchPr(branchName, targetBranch)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null if no PR exists | 517 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return the pr | 528 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getBranchStatusCheck(branchName, context)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return green if status is succeeded | 568 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return green if status is not applicable | 590 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return red if status is failed | 612 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return red if context status is error | 634 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return yellow if status is pending | 656 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return yellow if status is not set | 678 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return yellow if status is unknown | 700 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return null if status not found | 722 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getBranchStatus(branchName, ignoreTests)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should pass through success | 746 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not treat internal checks as success | 765 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should pass through failed | 784 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should pass through pending | 797 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should fall back to yellow if no statuses returned | 810 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getPr(prNo)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null if no prNo is passed | 825 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return null if no PR is returned from azure | 830 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return a pr in the right format | 842 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `createPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create and return a PR object | 875 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should create and return a PR object from base branch | 897 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `createPr() › when usePlatformAutomerge is set`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create and return a PR object with auto-complete set | 920 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should only call getMergeMethod once per run when automergeStrategy is auto | 960 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not call getMergeMethod when automergeStrategy is $automergeStrategy | 1043 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should create PR with mergeStrategy $prMergeStrategy | 1097 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should create and return an approved PR object | 1158 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `updatePr(prNo, title, body, platformPrOptions)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update the PR | 1198 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should update the PR including cache | 1216 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should update the PR without description | 1254 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should close the PR | 1270 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should reopen the PR | 1288 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should re-approve the PR | 1306 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds comment if missing | 1346 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| updates comment if missing | 1368 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| does nothing if comment exists and is the same | 1394 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| does nothing if comment exists and is the same when there is no topic | 1420 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| passes comment through massageMarkdown | 1442 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes comment by topic if found | 1494 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| deletes comment by content if found | 1510 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| comment not found | 1526 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `Assignees`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| addAssignees | 1539 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `Reviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| addReviewers one valid | 1567 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| addReviewers all valid | 1593 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `massageMarkdown(input)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns updated pr body | 1621 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns updated comment content | 1630 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `setBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should build and call the create status api properly | 1641 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should build and call the create status api properly with a complex context | 1673 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `mergePr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should complete the PR | 1707 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should complete PR with mergeStrategy $prMergeStrategy | 1754 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return false if the PR does not update successfully | 1809 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should cache the mergeMethod for subsequent merges | 1838 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should refetch the PR if the update response has not yet been set to completed | 1869 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should log a warning after retrying if the PR has still not yet been set to completed | 1901 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `deleteLabel()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Should delete a label | 1938 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns file content | 1956 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns null when file not found | 1969 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns file content in json5 format | 1979 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns file content from branch or tag | 1995 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws on malformed JSON | 2008 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws on errors | 2017 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| supports fetch from another repo | 2028 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns null | 2048 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| getRawFile should check tag first and then return branch if tag was not found | 2059 | not-applicable | Mock framework internals — tests azure platform via vitest-mocked Azure DevOps API; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

---
