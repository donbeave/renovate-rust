# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gerrit/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gerrit/index.spec.ts
**Total tests:** 63 | **Ported:** 0 | **Actionable:** 63 | **Status:** pending

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no endpoint | 59 | pending | — | — | —|
| should throw if no username/password | 64 | pending | — | — | —|
| should init | 71 | pending | — | — | —|
| should throw if auth fails | 81 | pending | — | — | —|
| should throw if version is unparseable | 92 | pending | — | — | —|

### `getRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns repos | 105 | pending | — | — | —|
| initRepo() - inactive | 111 | pending | — | — | —|

### `initRepo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| initRepo() - active | 133 | pending | — | — | —|
| initRepo() - passes cloneSubmodules | 146 | pending | — | — | —|
| initRepo() - abandon rejected changes | 163 | pending | — | — | —|

### `findPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| findPr() - no results | 193 | pending | — | — | —|
| findPr() - found | 214 | pending | — | — | —|

### `getPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getPr() - found | 237 | pending | — | — | —|
| getPr() - not found | 251 | pending | — | — | —|
| getPr() - other error | 256 | pending | — | — | —|

### `updatePr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updatePr() - closed => abandon the change | 267 | pending | — | — | —|
| updatePr() - body set => add as message if needed | 278 | pending | — | — | —|
| updatePr() - with addLabels => add hashtags | 295 | pending | — | — | —|
| updatePr() - with removeLabels => remove hashtags | 308 | pending | — | — | —|
| updatePr() - with addLabels and removeLabels => update hashtags in single call | 321 | pending | — | — | —|
| updatePr() - targetBranch set => move the change | 337 | pending | — | — | —|

### `createPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| createPr() - creates change by pushing to refs/for/ | 355 | pending | — | — | —|
| createPr() - with autoApprove | 388 | pending | — | — | —|
| createPr() - with labels | 424 | pending | — | — | —|
| createPr() - no change found after push => rejects | 463 | pending | — | — | —|
| createPr() - push fails => rejects | 478 | pending | — | — | —|

### `getBranchPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getBranchPr() - no result | 494 | pending | — | — | —|
| getBranchPr() - found | 509 | pending | — | — | —|
| getBranchPr() - found even without targetBranch | 535 | pending | — | — | —|

### `getPrList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getPrList() - empty list | 563 | pending | — | — | —|
| getPrList() - multiple results | 575 | pending | — | — | —|

### `mergePr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| mergePr() - blocker by Verified | 591 | pending | — | — | —|
| mergePr() - success | 600 | pending | — | — | —|
| mergePr() - other errors | 607 | pending | — | — | —|

### `getBranchStatus()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getBranchStatus() - change not found => yellow | 616 | pending | — | — | —|
| getBranchStatus() - change found, submittable and not hasProblems => green | 623 | pending | — | — | —|
| getBranchStatus() - change found, submittable but hasProblems => red | 633 | pending | — | — | —|
| getBranchStatus() - change found and hasProblems => red | 650 | pending | — | — | —|
| getBranchStatus() - changes found and hasBlockingLabels but no problems => red | 667 | pending | — | — | —|

### `getBranchStatusCheck() › GerritLabel is not available`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| unknownCtx | 694 | pending | — | — | —|

### `getBranchStatusCheck() › GerritLabel is available`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $ctx/$labels | 718 | pending | — | — | —|

### `setBranchStatus() › GerritLabel is not configured in Renovate`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| setBranchStatus(renovate/stability-days) | 766 | pending | — | — | —|
| setBranchStatus(renovate/merge-confidence) | 778 | pending | — | — | —|

### `setBranchStatus() › GerritLabel is configured in Renovate`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $ctx/$branchState | 803 | pending | — | — | —|
| no change found | 855 | pending | — | — | —|
| does not call setLabel() if label does not exist in change | 868 | pending | — | — | —|

### `deleteLabel()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deleteLabel() - deletes a label | 893 | pending | — | — | —|

### `addReviewers()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| addReviewers() - add reviewers | 903 | pending | — | — | —|

### `addAssignees()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| addAssignees() - set assignee | 916 | pending | — | — | —|

### `ensureComment()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ensureComment() - without tag | 929 | pending | — | — | —|
| ensureComment() - with tag | 942 | pending | — | — | —|

### `getRawFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getRawFile() - repo and branch | 961 | pending | — | — | —|
| getRawFile() - repo/branch from config | 972 | pending | — | — | —|
| getRawFile() - branch defaults | 986 | pending | — | — | —|
| getRawFile() - no repo | 1000 | pending | — | — | —|

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getJsonFile() | 1013 | pending | — | — | —|

### `massageMarkdown()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| massageMarkdown() | 1022 | pending | — | — | —|

### `currently unused/not-implemented functions`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deleteLabel() | 1053 | pending | — | — | —|
| ensureCommentRemoval() | 1059 | pending | — | — | —|
| ensureIssueClosing() | 1069 | pending | — | — | —|
| ensureIssue() | 1073 | pending | — | — | —|
| findIssue() | 1079 | pending | — | — | —|
| getIssueList() | 1083 | pending | — | — | —|

---
