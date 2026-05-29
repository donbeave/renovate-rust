# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/azure/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/azure/index.spec.ts
**Total tests:** 79 | **Ported:** 0 | **Actionable:** 79 | **Status:** not-applicable

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no endpoint | 102 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should throw if no token nor a username and password | 107 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should throw if a username but no password | 116 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should throw if a password but no username | 126 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should init | 136 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|

### `getRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return an array of repos | 147 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|

### `initRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should initialise the config for a repo | 201 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| throws if repo is disabled | 209 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| throws if repo is not in repos list | 217 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|

### `findPr(branchName, prTitle, state, targetBranch)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns pr if found it open | 227 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| returns pr if found not open | 269 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| returns pr if found it close | 311 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| returns pr if found it all state | 353 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| returns pr if found matches targetBranch | 394 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| returns first pr if found does not match targetBranch | 442 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| catches errors | 490 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|

### `getPrList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty array | 505 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|

### `getBranchPr(branchName, targetBranch)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null if no PR exists | 517 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should return the pr | 528 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|

### `getBranchStatusCheck(branchName, context)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return green if status is succeeded | 568 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should return green if status is not applicable | 590 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should return red if status is failed | 612 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should return red if context status is error | 634 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should return yellow if status is pending | 656 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should return yellow if status is not set | 678 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should return yellow if status is unknown | 700 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should return null if status not found | 722 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|

### `getBranchStatus(branchName, ignoreTests)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should pass through success | 746 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should not treat internal checks as success | 765 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should pass through failed | 784 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should pass through pending | 797 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should fall back to yellow if no statuses returned | 810 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|

### `getPr(prNo)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null if no prNo is passed | 825 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should return null if no PR is returned from azure | 830 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should return a pr in the right format | 842 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|

### `createPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create and return a PR object | 875 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should create and return a PR object from base branch | 897 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|

### `createPr() › when usePlatformAutomerge is set`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create and return a PR object with auto-complete set | 920 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should only call getMergeMethod once per run when automergeStrategy is auto | 960 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should not call getMergeMethod when automergeStrategy is $automergeStrategy | 1043 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should create PR with mergeStrategy $prMergeStrategy | 1097 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should create and return an approved PR object | 1158 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|

### `updatePr(prNo, title, body, platformPrOptions)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update the PR | 1198 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should update the PR including cache | 1216 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should update the PR without description | 1254 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should close the PR | 1270 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should reopen the PR | 1288 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should re-approve the PR | 1306 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds comment if missing | 1346 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| updates comment if missing | 1368 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| does nothing if comment exists and is the same | 1394 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| does nothing if comment exists and is the same when there is no topic | 1420 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| passes comment through massageMarkdown | 1442 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes comment by topic if found | 1494 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| deletes comment by content if found | 1510 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| comment not found | 1526 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|

### `Assignees`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| addAssignees | 1539 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|

### `Reviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| addReviewers one valid | 1567 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| addReviewers all valid | 1593 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|

### `massageMarkdown(input)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns updated pr body | 1621 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| returns updated comment content | 1630 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|

### `setBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should build and call the create status api properly | 1641 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should build and call the create status api properly with a complex context | 1673 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|

### `mergePr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should complete the PR | 1707 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should complete PR with mergeStrategy $prMergeStrategy | 1754 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should return false if the PR does not update successfully | 1809 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should cache the mergeMethod for subsequent merges | 1838 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should refetch the PR if the update response has not yet been set to completed | 1869 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| should log a warning after retrying if the PR has still not yet been set to completed | 1901 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|

### `deleteLabel()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Should delete a label | 1938 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns file content | 1956 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| returns null when file not found | 1969 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| returns file content in json5 format | 1979 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| returns file content from branch or tag | 1995 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| throws on malformed JSON | 2008 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| throws on errors | 2017 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| supports fetch from another repo | 2028 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| returns null | 2048 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|
| getRawFile should check tag first and then return branch if tag was not found | 2059 | not-applicable | — | — | mocking framework internals — mockDeep on Azure DevOps SDK wrapper/helper; TypeScript Azure DevOps SDK pipeline|

---
