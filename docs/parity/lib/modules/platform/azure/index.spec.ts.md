# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/azure/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/azure/index.spec.ts
**Total tests:** 79 | **Ported:** 0 | **Actionable:** 79 | **Status:** pending

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no endpoint | 102 | pending | — | — | — |
| should throw if no token nor a username and password | 107 | pending | — | — | — |
| should throw if a username but no password | 116 | pending | — | — | — |
| should throw if a password but no username | 126 | pending | — | — | — |
| should init | 136 | pending | — | — | — |

### `getRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return an array of repos | 147 | pending | — | — | — |

### `initRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should initialise the config for a repo | 201 | pending | — | — | — |
| throws if repo is disabled | 209 | pending | — | — | — |
| throws if repo is not in repos list | 217 | pending | — | — | — |

### `findPr(branchName, prTitle, state, targetBranch)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns pr if found it open | 227 | pending | — | — | — |
| returns pr if found not open | 269 | pending | — | — | — |
| returns pr if found it close | 311 | pending | — | — | — |
| returns pr if found it all state | 353 | pending | — | — | — |
| returns pr if found matches targetBranch | 394 | pending | — | — | — |
| returns first pr if found does not match targetBranch | 442 | pending | — | — | — |
| catches errors | 490 | pending | — | — | — |

### `getPrList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty array | 505 | pending | — | — | — |

### `getBranchPr(branchName, targetBranch)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null if no PR exists | 517 | pending | — | — | — |
| should return the pr | 528 | pending | — | — | — |

### `getBranchStatusCheck(branchName, context)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return green if status is succeeded | 568 | pending | — | — | — |
| should return green if status is not applicable | 590 | pending | — | — | — |
| should return red if status is failed | 612 | pending | — | — | — |
| should return red if context status is error | 634 | pending | — | — | — |
| should return yellow if status is pending | 656 | pending | — | — | — |
| should return yellow if status is not set | 678 | pending | — | — | — |
| should return yellow if status is unknown | 700 | pending | — | — | — |
| should return null if status not found | 722 | pending | — | — | — |

### `getBranchStatus(branchName, ignoreTests)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should pass through success | 746 | pending | — | — | — |
| should not treat internal checks as success | 765 | pending | — | — | — |
| should pass through failed | 784 | pending | — | — | — |
| should pass through pending | 797 | pending | — | — | — |
| should fall back to yellow if no statuses returned | 810 | pending | — | — | — |

### `getPr(prNo)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null if no prNo is passed | 825 | pending | — | — | — |
| should return null if no PR is returned from azure | 830 | pending | — | — | — |
| should return a pr in the right format | 842 | pending | — | — | — |

### `createPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create and return a PR object | 875 | pending | — | — | — |
| should create and return a PR object from base branch | 897 | pending | — | — | — |

### `createPr() › when usePlatformAutomerge is set`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create and return a PR object with auto-complete set | 920 | pending | — | — | — |
| should only call getMergeMethod once per run when automergeStrategy is auto | 960 | pending | — | — | — |
| should not call getMergeMethod when automergeStrategy is $automergeStrategy | 1043 | pending | — | — | — |
| should create PR with mergeStrategy $prMergeStrategy | 1097 | pending | — | — | — |
| should create and return an approved PR object | 1158 | pending | — | — | — |

### `updatePr(prNo, title, body, platformPrOptions)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update the PR | 1198 | pending | — | — | — |
| should update the PR including cache | 1216 | pending | — | — | — |
| should update the PR without description | 1254 | pending | — | — | — |
| should close the PR | 1270 | pending | — | — | — |
| should reopen the PR | 1288 | pending | — | — | — |
| should re-approve the PR | 1306 | pending | — | — | — |

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds comment if missing | 1346 | pending | — | — | — |
| updates comment if missing | 1368 | pending | — | — | — |
| does nothing if comment exists and is the same | 1394 | pending | — | — | — |
| does nothing if comment exists and is the same when there is no topic | 1420 | pending | — | — | — |
| passes comment through massageMarkdown | 1442 | pending | — | — | — |

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes comment by topic if found | 1494 | pending | — | — | — |
| deletes comment by content if found | 1510 | pending | — | — | — |
| comment not found | 1526 | pending | — | — | — |

### `Assignees`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| addAssignees | 1539 | pending | — | — | — |

### `Reviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| addReviewers one valid | 1567 | pending | — | — | — |
| addReviewers all valid | 1593 | pending | — | — | — |

### `massageMarkdown(input)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns updated pr body | 1621 | pending | — | — | — |
| returns updated comment content | 1630 | pending | — | — | — |

### `setBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should build and call the create status api properly | 1641 | pending | — | — | — |
| should build and call the create status api properly with a complex context | 1673 | pending | — | — | — |

### `mergePr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should complete the PR | 1707 | pending | — | — | — |
| should complete PR with mergeStrategy $prMergeStrategy | 1754 | pending | — | — | — |
| should return false if the PR does not update successfully | 1809 | pending | — | — | — |
| should cache the mergeMethod for subsequent merges | 1838 | pending | — | — | — |
| should refetch the PR if the update response has not yet been set to completed | 1869 | pending | — | — | — |
| should log a warning after retrying if the PR has still not yet been set to completed | 1901 | pending | — | — | — |

### `deleteLabel()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Should delete a label | 1938 | pending | — | — | — |

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns file content | 1956 | pending | — | — | — |
| returns null when file not found | 1969 | pending | — | — | — |
| returns file content in json5 format | 1979 | pending | — | — | — |
| returns file content from branch or tag | 1995 | pending | — | — | — |
| throws on malformed JSON | 2008 | pending | — | — | — |
| throws on errors | 2017 | pending | — | — | — |
| supports fetch from another repo | 2028 | pending | — | — | — |
| returns null | 2048 | pending | — | — | — |
| getRawFile should check tag first and then return branch if tag was not found | 2059 | pending | — | — | — |

---

