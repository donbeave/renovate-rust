# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gitlab/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gitlab/index.spec.ts
**Total tests:** 163 | **Ported:** 0 | **Actionable:** 163 | **Status:** pending

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no token | 78 | pending | — | — | —|
| should throw if auth fails | 82 | pending | — | — | —|
| should default to gitlab.com | 92 | pending | — | — | —|
| should accept custom endpoint | 108 | pending | — | — | —|
| should reuse existing gitAuthor | 129 | pending | — | — | —|

### `getRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw an error if it receives an error | 144 | pending | — | — | —|
| should return an array of repos | 154 | pending | — | — | —|
| should return an array of repos including mirrors | 176 | pending | — | — | —|
| should encode the requested topics into the URL | 198 | pending | — | — | —|
| should query the groups endpoint for each namespace | 216 | pending | — | — | —|
| should consider topics when querying the groups endpoint | 242 | pending | — | — | —|
| should include order and sort query parameters | 263 | pending | — | — | —|

### `initRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should escape all forward slashes in project names | 308 | pending | — | — | —|
| should throw an error if receiving an error | 324 | pending | — | — | —|
| should throw an error if repository is archived | 336 | pending | — | — | —|
| should throw an error if repository is a mirror | 348 | pending | — | — | —|
| should not throw an error if repository is a mirror when includeMirrors option is set | 360 | pending | — | — | —|
| should throw an error if repository access is disabled | 380 | pending | — | — | —|
| should throw an error if MRs are disabled | 392 | pending | — | — | —|
| should throw an error if repository has empty_repo property | 404 | pending | — | — | —|
| should throw an error if repository is empty | 416 | pending | — | — | —|
| should fall back if http_url_to_repo is empty | 428 | pending | — | — | —|
| should use ssh_url_to_repo if gitUrl is set to ssh | 447 | pending | — | — | —|
| should throw if ssh_url_to_repo is not present but gitUrl is set to ssh | 464 | pending | — | — | —|
| should fall back respecting when GITLAB_IGNORE_REPO_URL is set | 480 | pending | — | — | —|

### `getBranchForceRebase`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return false for merge_method=merge | 513 | pending | — | — | —|
| should return true for merge_method=ff | 527 | pending | — | — | —|
| should return false when merge trains are enabled | 541 | pending | — | — | —|

### `getBranchPr(branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null if no PR exists | 558 | pending | — | — | —|
| should return the PR object | 570 | pending | — | — | —|
| should strip draft prefix from title | 609 | pending | — | — | —|
| should strip deprecated draft prefix from title | 648 | pending | — | — | —|

### `getBranchStatus(branchName, ignoreTests)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns pending if no results | 689 | pending | — | — | —|
| returns success if no results with merged results pipeline success | 704 | pending | — | — | —|
| returns success if all are success | 751 | pending | — | — | —|
| returns pending if all are internal success | 769 | pending | — | — | —|
| returns pending if merge request with no pipelines | 787 | pending | — | — | —|
| returns pending if all are internal success with no merged results pipeline | 830 | pending | — | — | —|
| returns success if all are internal success with merged results pipeline success | 880 | pending | — | — | —|
| returns success if optional jobs fail | 930 | pending | — | — | —|
| returns success if all are optional | 948 | pending | — | — | —|
| returns success if job is skipped | 963 | pending | — | — | —|
| returns yellow if there are no jobs expect skipped | 978 | pending | — | — | —|
| returns failure if any mandatory jobs fails and one job is skipped | 993 | pending | — | — | —|
| returns failure if any mandatory jobs fails | 1008 | pending | — | — | —|
| maps custom statuses to yellow | 1027 | pending | — | — | —|
| throws repository-changed | 1042 | pending | — | — | —|

### `getBranchStatusCheck`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no results | 1053 | pending | — | — | —|
| returns null if no matching results | 1067 | pending | — | — | —|
| returns status if name found | 1081 | pending | — | — | —|
| returns yellow if unknown status found | 1099 | pending | — | — | —|

### `setBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should log message that branch commit SHA not found | 1121 | pending | — | — | —|
| should log message that failed to retrieve commit pipeline | 1136 | pending | — | — | —|
| /api/v4/projects/some%2Frepo/statuses/0d9c7726c3d628b7e28af234595cfd20febdbf8e | 1168 | pending | — | — | —|
| skips setting branch status %s when RENOVATE_X_GITLAB_SKIP_STATUS_WITHOUT_PIPELINE is set and no pipeline is found | 1196 | pending | — | — | —|
| does not skip setting branch status when RENOVATE_X_GITLAB_SKIP_STATUS_WITHOUT_PIPELINE is not true | 1224 | pending | — | — | —|
| sets branch status when RENOVATE_X_GITLAB_SKIP_STATUS_WITHOUT_PIPELINE is true and pipeline is found | 1257 | pending | — | — | —|
| waits for 1000ms by default | 1293 | pending | — | — | —|
| set branch status with pipeline_id | 1322 | pending | — | — | —|
| waits for RENOVATE_X_GITLAB_BRANCH_STATUS_DELAY ms when set | 1357 | pending | — | — | —|
| do RENOVATE_X_GITLAB_BRANCH_STATUS_CHECK_ATTEMPTS attemps when set | 1402 | pending | — | — | —|

### `findIssue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no issue | 1437 | pending | — | — | —|
| finds issue | 1457 | pending | — | — | —|

### `ensureIssue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates issue | 1481 | pending | — | — | —|
| sets issue labels | 1506 | pending | — | — | —|
| updates issue | 1523 | pending | — | — | —|
| updates issue with labels | 1550 | pending | — | — | —|
| skips update if unchanged | 1578 | pending | — | — | —|
| creates confidential issue | 1603 | pending | — | — | —|
| updates confidential issue | 1629 | pending | — | — | —|

### `ensureIssueClosing()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| closes issue | 1660 | pending | — | — | —|

### `addAssignees(issueNo, assignees)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add the given assignee to the issue | 1683 | pending | — | — | —|
| should add the given assignees to the issue | 1693 | pending | — | — | —|
| should swallow error | 1709 | pending | — | — | —|
| should log message for each assignee that could not be found | 1723 | pending | — | — | —|

### `addReviewers(iid, reviewers) › 13.8.0`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should not be supported in too low version | 1757 | pending | — | — | —|

### `addReviewers(iid, reviewers) › 13.9.0`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should fail to get existing reviewers | 1778 | pending | — | — | —|
| should not fail if some reviewers are unknown | 1790 | pending | — | — | —|
| should warn and return early if new reviewers IDs could be fetched | 1812 | pending | — | — | —|
| should add gitlab group members as reviewers to MR | 1835 | pending | — | — | —|
| should fail to add reviewers to the MR | 1857 | pending | — | — | —|
| should add the given reviewers to the MR | 1877 | pending | — | — | —|
| should only add reviewers if necessary | 1897 | pending | — | — | —|

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add comment if not found | 1918 | pending | — | — | —|
| add updates comment if necessary | 1934 | pending | — | — | —|
| skips comment | 1950 | pending | — | — | —|
| handles comment with no description | 1964 | pending | — | — | —|

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes comment by topic if found | 1980 | pending | — | — | —|
| deletes comment by content if found | 1996 | pending | — | — | —|

### `findPr(branchName, prTitle, state)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true if no title and all state | 2014 | pending | — | — | —|
| returns true if not open | 2038 | pending | — | — | —|
| returns true if open and with title | 2063 | pending | — | — | —|
| returns true with title | 2089 | pending | — | — | —|
| returns true with draft prefix title | 2114 | pending | — | — | —|
| returns true with deprecated draft prefix title | 2139 | pending | — | — | —|
| finds pr from other authors | 2164 | pending | — | — | —|
| returns null if no pr found - (includeOtherAuthors) | 2196 | pending | — | — | —|

### `createPr(branchName, title, body)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the PR | 2236 | pending | — | — | —|
| uses default branch | 2268 | pending | — | — | —|
| supports draftPR on < 13.2 | 2300 | pending | — | — | —|
| supports draftPR on >= 13.2 | 2332 | pending | — | — | —|
| auto-accepts the MR when requested | 2364 | pending | — | — | —|
| adds the MR to a merge train when merge trains are enabled on the project | 2407 | pending | — | — | —|
| falls back to /merge endpoint when merge trains enabled but GitLab < 17.11 | 2459 | pending | — | — | —|
| retries the merge_trains endpoint on transient failure | 2512 | pending | — | — | —|
| should parse merge_status attribute if detailed_merge_status is not set (on < 15.6) | 2563 | pending | — | — | —|
| should parse detailed_merge_status attribute on >= 15.6 | 2628 | pending | — | — | —|
| should retry auto merge creation on 405 method not allowed | 2686 | pending | — | — | —|
| should not retry if MR is mergeable and pipeline is running | 2764 | pending | — | — | —|
| raises with squash enabled when repository squash option is default_on | 2808 | pending | — | — | —|
| raises with squash enabled when repository squash option is always | 2851 | pending | — | — | —|
| adds approval rule to ignore all approvals | 2894 | pending | — | — | —|
| adds approval rule to ignore all approvals when platformAutomerge is false | 2948 | pending | — | — | —|
| will modify a rule of type any_approvers, if such a rule exists | 2996 | pending | — | — | —|
| will remove rules of type regular, if such rules exist | 3058 | pending | — | — | —|
| does not try to remove "report_approver" and "code_owner" approval rules | 3131 | pending | — | — | —|
| does not try to create already existing approval rule | 3214 | pending | — | — | —|
| silently ignores approval rules adding errors | 3268 | pending | — | — | —|
| auto-approves when enabled | 3322 | pending | — | — | —|
| auto-approve with different user | 3359 | pending | — | — | —|
| should swallow an error on auto-approve | 3398 | pending | — | — | —|

### `getPr(prNo)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the PR | 3433 | pending | — | — | —|
| removes draft prefix from returned title | 3457 | pending | — | — | —|
| removes deprecated draft prefix from returned title | 3481 | pending | — | — | —|
| returns the mergeable PR | 3505 | pending | — | — | —|
| returns the PR with nonexisting branch | 3530 | pending | — | — | —|
| returns the PR with reviewers | 3558 | pending | — | — | —|

### `updatePr(prNo, title, body)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates the PR | 3601 | pending | — | — | —|
| retains draft status when draft uses current prefix | 3634 | pending | — | — | —|
| retains draft status when draft uses deprecated prefix | 3667 | pending | — | — | —|
| updates target branch of the PR | 3700 | pending | — | — | —|
| auto-approves when enabled | 3739 | pending | — | — | —|
| closes the PR | 3782 | pending | — | — | —|
| adds and removes labels | 3821 | pending | — | — | —|

### `reattemptPlatformAutomerge(number, platformPrOptions)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should set automatic merge | 3871 | pending | — | — | —|
| should skip retries when merge_when_pipeline_succeeds is already enabled | 3894 | pending | — | — | —|

### `mergePr(pr)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| merges the PR | 3916 | pending | — | — | —|

### `massageMarkdown(input)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| strips invalid unicode null characters | 3941 | pending | — | — | —|
| replaces PR with MR including pluralization | 3949 | pending | — | — | —|
| replaces PR reference with MR reference | 3957 | pending | — | — | —|
| replaces PR relative link with MR reference | 3963 | pending | — | — | —|
| replaces issues relative link with issue reference | 3971 | pending | — | — | —|
| avoids false positives when replacing PR with MR | 3979 | pending | — | — | —|
| returns updated pr body | 3984 | pending | — | — | —|
| truncates description if too low API version | 3993 | pending | — | — | —|
| truncates description for API version gt 13.4 | 4003 | pending | — | — | —|

### `deleteLabel(issueNo, label)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should delete the label | 4015 | pending | — | — | —|

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null | 4040 | pending | — | — | —|
| returns file content | 4053 | pending | — | — | —|
| returns file content in json5 format | 4067 | pending | — | — | —|
| returns file content from given repo | 4086 | pending | — | — | —|
| returns file content from branch or tag | 4100 | pending | — | — | —|
| throws on malformed JSON | 4118 | pending | — | — | —|
| throws on errors | 4130 | pending | — | — | —|

### `filterUnavailableUsers(users)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| filters users that are busy | 4142 | pending | — | — | —|
| keeps users with missing availability | 4160 | pending | — | — | —|
| keeps users with failing requests | 4169 | pending | — | — | —|

### `expandGroupMembers(reviewersOrAssignees)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| expands group members for groups with members | 4180 | pending | — | — | —|
| users are not expanded when 404 | 4200 | pending | — | — | —|
| users are not expanded when non 404 | 4209 | pending | — | — | —|
| groups with no members expand into empty list | 4225 | pending | — | — | —|
| includes email in final result | 4236 | pending | — | — | —|

| should throw if endpoint is not a valid URL | 82 | pending | — | — | —|
---
