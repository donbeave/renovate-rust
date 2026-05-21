# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/azure/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/azure/index.spec.ts
**Total tests:** 79 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no endpoint | 102 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should throw if no token nor a username and password | 107 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should throw if a username but no password | 116 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should throw if a password but no username | 126 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should init | 136 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `getRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return an array of repos | 147 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `initRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should initialise the config for a repo | 201 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| throws if repo is disabled | 209 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| throws if repo is not in repos list | 217 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `findPr(branchName, prTitle, state, targetBranch)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns pr if found it open | 227 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| returns pr if found not open | 269 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| returns pr if found it close | 311 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| returns pr if found it all state | 353 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| returns pr if found matches targetBranch | 394 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| returns first pr if found does not match targetBranch | 442 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| catches errors | 490 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `getPrList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty array | 505 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `getBranchPr(branchName, targetBranch)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null if no PR exists | 517 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should return the pr | 528 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `getBranchStatusCheck(branchName, context)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return green if status is succeeded | 568 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should return green if status is not applicable | 590 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should return red if status is failed | 612 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should return red if context status is error | 634 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should return yellow if status is pending | 656 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should return yellow if status is not set | 678 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should return yellow if status is unknown | 700 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should return null if status not found | 722 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `getBranchStatus(branchName, ignoreTests)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should pass through success | 746 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should not treat internal checks as success | 765 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should pass through failed | 784 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should pass through pending | 797 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should fall back to yellow if no statuses returned | 810 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `getPr(prNo)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null if no prNo is passed | 825 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should return null if no PR is returned from azure | 830 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should return a pr in the right format | 842 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `createPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create and return a PR object | 875 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should create and return a PR object from base branch | 897 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `createPr() › when usePlatformAutomerge is set`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create and return a PR object with auto-complete set | 920 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should only call getMergeMethod once per run when automergeStrategy is auto | 960 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should not call getMergeMethod when automergeStrategy is $automergeStrategy | 1043 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should create PR with mergeStrategy $prMergeStrategy | 1097 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should create and return an approved PR object | 1158 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `updatePr(prNo, title, body, platformPrOptions)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update the PR | 1198 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should update the PR including cache | 1216 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should update the PR without description | 1254 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should close the PR | 1270 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should reopen the PR | 1288 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should re-approve the PR | 1306 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds comment if missing | 1346 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| updates comment if missing | 1368 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| does nothing if comment exists and is the same | 1394 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| does nothing if comment exists and is the same when there is no topic | 1420 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| passes comment through massageMarkdown | 1442 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes comment by topic if found | 1494 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| deletes comment by content if found | 1510 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| comment not found | 1526 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `Assignees`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| addAssignees | 1539 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `Reviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| addReviewers one valid | 1567 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| addReviewers all valid | 1593 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `massageMarkdown(input)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns updated pr body | 1621 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| returns updated comment content | 1630 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `setBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should build and call the create status api properly | 1641 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should build and call the create status api properly with a complex context | 1673 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `mergePr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should complete the PR | 1707 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should complete PR with mergeStrategy $prMergeStrategy | 1754 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should return false if the PR does not update successfully | 1809 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should cache the mergeMethod for subsequent merges | 1838 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should refetch the PR if the update response has not yet been set to completed | 1869 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should log a warning after retrying if the PR has still not yet been set to completed | 1901 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `deleteLabel()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Should delete a label | 1938 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns file content | 1956 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| returns null when file not found | 1969 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| returns file content in json5 format | 1979 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| returns file content from branch or tag | 1995 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| throws on malformed JSON | 2008 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| throws on errors | 2017 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| supports fetch from another repo | 2028 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| returns null | 2048 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| getRawFile should check tag first and then return branch if tag was not found | 2059 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

---

