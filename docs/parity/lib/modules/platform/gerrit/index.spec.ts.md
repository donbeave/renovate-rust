# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gerrit/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gerrit/index.spec.ts
**Total tests:** 63 | **Ported:** 0 | **Actionable:** 63 | **Status:** pending

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no endpoint | 59 | pending | — | — | No corresponding Rust source|
| should throw if no username/password | 64 | pending | — | — | No corresponding Rust source|
| should init | 71 | pending | — | — | No corresponding Rust source|
| should throw if auth fails | 81 | pending | — | — | No corresponding Rust source|
| should throw if version is unparseable | 92 | pending | — | — | No corresponding Rust source|

### `getRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns repos | 105 | pending | — | — | No corresponding Rust source|
| initRepo() - inactive | 111 | pending | — | — | No corresponding Rust source|

### `initRepo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| initRepo() - active | 133 | pending | — | — | No corresponding Rust source|
| initRepo() - passes cloneSubmodules | 146 | pending | — | — | No corresponding Rust source|
| initRepo() - abandon rejected changes | 163 | pending | — | — | No corresponding Rust source|

### `findPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| findPr() - no results | 193 | pending | — | — | No corresponding Rust source|
| findPr() - found | 214 | pending | — | — | No corresponding Rust source|

### `getPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getPr() - found | 237 | pending | — | — | No corresponding Rust source|
| getPr() - not found | 251 | pending | — | — | No corresponding Rust source|
| getPr() - other error | 256 | pending | — | — | No corresponding Rust source|

### `updatePr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updatePr() - closed => abandon the change | 267 | pending | — | — | No corresponding Rust source|
| updatePr() - body set => add as message if needed | 278 | pending | — | — | No corresponding Rust source|
| updatePr() - with addLabels => add hashtags | 295 | pending | — | — | No corresponding Rust source|
| updatePr() - with removeLabels => remove hashtags | 308 | pending | — | — | No corresponding Rust source|
| updatePr() - with addLabels and removeLabels => update hashtags in single call | 321 | pending | — | — | No corresponding Rust source|
| updatePr() - targetBranch set => move the change | 337 | pending | — | — | No corresponding Rust source|

### `createPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| createPr() - creates change by pushing to refs/for/ | 355 | pending | — | — | No corresponding Rust source|
| createPr() - with autoApprove | 388 | pending | — | — | No corresponding Rust source|
| createPr() - with labels | 424 | pending | — | — | No corresponding Rust source|
| createPr() - no change found after push => rejects | 463 | pending | — | — | No corresponding Rust source|
| createPr() - push fails => rejects | 478 | pending | — | — | No corresponding Rust source|

### `getBranchPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getBranchPr() - no result | 494 | pending | — | — | No corresponding Rust source|
| getBranchPr() - found | 509 | pending | — | — | No corresponding Rust source|
| getBranchPr() - found even without targetBranch | 535 | pending | — | — | No corresponding Rust source|

### `getPrList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getPrList() - empty list | 563 | pending | — | — | No corresponding Rust source|
| getPrList() - multiple results | 575 | pending | — | — | No corresponding Rust source|

### `mergePr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| mergePr() - blocker by Verified | 591 | pending | — | — | No corresponding Rust source|
| mergePr() - success | 600 | pending | — | — | No corresponding Rust source|
| mergePr() - other errors | 607 | pending | — | — | No corresponding Rust source|

### `getBranchStatus()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getBranchStatus() - change not found => yellow | 616 | pending | — | — | No corresponding Rust source|
| getBranchStatus() - change found, submittable and not hasProblems => green | 623 | pending | — | — | No corresponding Rust source|
| getBranchStatus() - change found, submittable but hasProblems => red | 633 | pending | — | — | No corresponding Rust source|
| getBranchStatus() - change found and hasProblems => red | 650 | pending | — | — | No corresponding Rust source|
| getBranchStatus() - changes found and hasBlockingLabels but no problems => red | 667 | pending | — | — | No corresponding Rust source|

### `getBranchStatusCheck() › GerritLabel is not available`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| unknownCtx | 694 | pending | — | — | No corresponding Rust source|

### `getBranchStatusCheck() › GerritLabel is available`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $ctx/$labels | 718 | pending | — | — | No corresponding Rust source|

### `setBranchStatus() › GerritLabel is not configured in Renovate`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| setBranchStatus(renovate/stability-days) | 766 | pending | — | — | No corresponding Rust source|
| setBranchStatus(renovate/merge-confidence) | 778 | pending | — | — | No corresponding Rust source|

### `setBranchStatus() › GerritLabel is configured in Renovate`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $ctx/$branchState | 803 | pending | — | — | No corresponding Rust source|
| no change found | 855 | pending | — | — | No corresponding Rust source|
| does not call setLabel() if label does not exist in change | 868 | pending | — | — | No corresponding Rust source|

### `deleteLabel()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deleteLabel() - deletes a label | 893 | pending | — | — | No corresponding Rust source|

### `addReviewers()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| addReviewers() - add reviewers | 903 | pending | — | — | No corresponding Rust source|

### `addAssignees()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| addAssignees() - set assignee | 916 | pending | — | — | No corresponding Rust source|

### `ensureComment()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ensureComment() - without tag | 929 | pending | — | — | No corresponding Rust source|
| ensureComment() - with tag | 942 | pending | — | — | No corresponding Rust source|

### `getRawFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getRawFile() - repo and branch | 961 | pending | — | — | No corresponding Rust source|
| getRawFile() - repo/branch from config | 972 | pending | — | — | No corresponding Rust source|
| getRawFile() - branch defaults | 986 | pending | — | — | No corresponding Rust source|
| getRawFile() - no repo | 1000 | pending | — | — | No corresponding Rust source|

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getJsonFile() | 1013 | pending | — | — | No corresponding Rust source|

### `massageMarkdown()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| massageMarkdown() | 1022 | pending | — | — | No corresponding Rust source|

### `currently unused/not-implemented functions`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deleteLabel() | 1053 | pending | — | — | No corresponding Rust source|
| ensureCommentRemoval() | 1059 | pending | — | — | No corresponding Rust source|
| ensureIssueClosing() | 1069 | pending | — | — | No corresponding Rust source|
| ensureIssue() | 1073 | pending | — | — | No corresponding Rust source|
| findIssue() | 1079 | pending | — | — | No corresponding Rust source|
| getIssueList() | 1083 | pending | — | — | No corresponding Rust source|

---
