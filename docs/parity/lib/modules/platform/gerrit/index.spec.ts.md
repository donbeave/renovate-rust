# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gerrit/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gerrit/index.spec.ts
**Total tests:** 63 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no endpoint | 59 | not-applicable | — | — | No corresponding Rust source|
| should throw if no username/password | 64 | not-applicable | — | — | No corresponding Rust source|
| should init | 71 | not-applicable | — | — | No corresponding Rust source|
| should throw if auth fails | 81 | not-applicable | — | — | No corresponding Rust source|
| should throw if version is unparseable | 92 | not-applicable | — | — | No corresponding Rust source|

### `getRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns repos | 105 | not-applicable | — | — | No corresponding Rust source|
| initRepo() - inactive | 111 | not-applicable | — | — | No corresponding Rust source|

### `initRepo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| initRepo() - active | 133 | not-applicable | — | — | No corresponding Rust source|
| initRepo() - passes cloneSubmodules | 146 | not-applicable | — | — | No corresponding Rust source|
| initRepo() - abandon rejected changes | 163 | not-applicable | — | — | No corresponding Rust source|

### `findPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| findPr() - no results | 193 | not-applicable | — | — | No corresponding Rust source|
| findPr() - found | 214 | not-applicable | — | — | No corresponding Rust source|

### `getPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getPr() - found | 237 | not-applicable | — | — | No corresponding Rust source|
| getPr() - not found | 251 | not-applicable | — | — | No corresponding Rust source|
| getPr() - other error | 256 | not-applicable | — | — | No corresponding Rust source|

### `updatePr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updatePr() - closed => abandon the change | 267 | not-applicable | — | — | No corresponding Rust source|
| updatePr() - body set => add as message if needed | 278 | not-applicable | — | — | No corresponding Rust source|
| updatePr() - with addLabels => add hashtags | 295 | not-applicable | — | — | No corresponding Rust source|
| updatePr() - with removeLabels => remove hashtags | 308 | not-applicable | — | — | No corresponding Rust source|
| updatePr() - with addLabels and removeLabels => update hashtags in single call | 321 | not-applicable | — | — | No corresponding Rust source|
| updatePr() - targetBranch set => move the change | 337 | not-applicable | — | — | No corresponding Rust source|

### `createPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| createPr() - creates change by pushing to refs/for/ | 355 | not-applicable | — | — | No corresponding Rust source|
| createPr() - with autoApprove | 388 | not-applicable | — | — | No corresponding Rust source|
| createPr() - with labels | 424 | not-applicable | — | — | No corresponding Rust source|
| createPr() - no change found after push => rejects | 463 | not-applicable | — | — | No corresponding Rust source|
| createPr() - push fails => rejects | 478 | not-applicable | — | — | No corresponding Rust source|

### `getBranchPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getBranchPr() - no result | 494 | not-applicable | — | — | No corresponding Rust source|
| getBranchPr() - found | 509 | not-applicable | — | — | No corresponding Rust source|
| getBranchPr() - found even without targetBranch | 535 | not-applicable | — | — | No corresponding Rust source|

### `getPrList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getPrList() - empty list | 563 | not-applicable | — | — | No corresponding Rust source|
| getPrList() - multiple results | 575 | not-applicable | — | — | No corresponding Rust source|

### `mergePr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| mergePr() - blocker by Verified | 591 | not-applicable | — | — | No corresponding Rust source|
| mergePr() - success | 600 | not-applicable | — | — | No corresponding Rust source|
| mergePr() - other errors | 607 | not-applicable | — | — | No corresponding Rust source|

### `getBranchStatus()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getBranchStatus() - change not found => yellow | 616 | not-applicable | — | — | No corresponding Rust source|
| getBranchStatus() - change found, submittable and not hasProblems => green | 623 | not-applicable | — | — | No corresponding Rust source|
| getBranchStatus() - change found, submittable but hasProblems => red | 633 | not-applicable | — | — | No corresponding Rust source|
| getBranchStatus() - change found and hasProblems => red | 650 | not-applicable | — | — | No corresponding Rust source|
| getBranchStatus() - changes found and hasBlockingLabels but no problems => red | 667 | not-applicable | — | — | No corresponding Rust source|

### `getBranchStatusCheck() › GerritLabel is not available`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| unknownCtx | 694 | not-applicable | — | — | No corresponding Rust source|

### `getBranchStatusCheck() › GerritLabel is available`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $ctx/$labels | 718 | not-applicable | — | — | No corresponding Rust source|

### `setBranchStatus() › GerritLabel is not configured in Renovate`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| setBranchStatus(renovate/stability-days) | 766 | not-applicable | — | — | No corresponding Rust source|
| setBranchStatus(renovate/merge-confidence) | 778 | not-applicable | — | — | No corresponding Rust source|

### `setBranchStatus() › GerritLabel is configured in Renovate`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $ctx/$branchState | 803 | not-applicable | — | — | No corresponding Rust source|
| no change found | 855 | not-applicable | — | — | No corresponding Rust source|
| does not call setLabel() if label does not exist in change | 868 | not-applicable | — | — | No corresponding Rust source|

### `deleteLabel()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deleteLabel() - deletes a label | 893 | not-applicable | — | — | No corresponding Rust source|

### `addReviewers()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| addReviewers() - add reviewers | 903 | not-applicable | — | — | No corresponding Rust source|

### `addAssignees()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| addAssignees() - set assignee | 916 | not-applicable | — | — | No corresponding Rust source|

### `ensureComment()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ensureComment() - without tag | 929 | not-applicable | — | — | No corresponding Rust source|
| ensureComment() - with tag | 942 | not-applicable | — | — | No corresponding Rust source|

### `getRawFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getRawFile() - repo and branch | 961 | not-applicable | — | — | No corresponding Rust source|
| getRawFile() - repo/branch from config | 972 | not-applicable | — | — | No corresponding Rust source|
| getRawFile() - branch defaults | 986 | not-applicable | — | — | No corresponding Rust source|
| getRawFile() - no repo | 1000 | not-applicable | — | — | No corresponding Rust source|

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getJsonFile() | 1013 | not-applicable | — | — | No corresponding Rust source|

### `massageMarkdown()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| massageMarkdown() | 1022 | not-applicable | — | — | No corresponding Rust source|

### `currently unused/not-implemented functions`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deleteLabel() | 1053 | not-applicable | — | — | No corresponding Rust source|
| ensureCommentRemoval() | 1059 | not-applicable | — | — | No corresponding Rust source|
| ensureIssueClosing() | 1069 | not-applicable | — | — | No corresponding Rust source|
| ensureIssue() | 1073 | not-applicable | — | — | No corresponding Rust source|
| findIssue() | 1079 | not-applicable | — | — | No corresponding Rust source|
| getIssueList() | 1083 | not-applicable | — | — | No corresponding Rust source|

---
