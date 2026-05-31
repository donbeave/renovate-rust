# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gitlab/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gitlab/index.spec.ts
**Total tests:** 163 | **Ported:** 0 | **Actionable:** 163 | **Status:** pending

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no token | 78 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw if auth fails | 82 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should default to gitlab.com | 92 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should accept custom endpoint | 108 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should reuse existing gitAuthor | 129 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw an error if it receives an error | 144 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return an array of repos | 154 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return an array of repos including mirrors | 176 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should encode the requested topics into the URL | 198 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should query the groups endpoint for each namespace | 216 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should consider topics when querying the groups endpoint | 242 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should include order and sort query parameters | 263 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `initRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should escape all forward slashes in project names | 308 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw an error if receiving an error | 324 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw an error if repository is archived | 336 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw an error if repository is a mirror | 348 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not throw an error if repository is a mirror when includeMirrors option is set | 360 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw an error if repository access is disabled | 380 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw an error if MRs are disabled | 392 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw an error if repository has empty_repo property | 404 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw an error if repository is empty | 416 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should fall back if http_url_to_repo is empty | 428 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use ssh_url_to_repo if gitUrl is set to ssh | 447 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw if ssh_url_to_repo is not present but gitUrl is set to ssh | 464 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should fall back respecting when GITLAB_IGNORE_REPO_URL is set | 480 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getBranchForceRebase`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return false for merge_method=merge | 513 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return true for merge_method=ff | 527 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return false when merge trains are enabled | 541 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getBranchPr(branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null if no PR exists | 558 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return the PR object | 570 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should strip draft prefix from title | 609 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should strip deprecated draft prefix from title | 648 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getBranchStatus(branchName, ignoreTests)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns pending if no results | 689 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns success if no results with merged results pipeline success | 704 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns success if all are success | 751 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns pending if all are internal success | 769 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns pending if merge request with no pipelines | 787 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns pending if all are internal success with no merged results pipeline | 830 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns success if all are internal success with merged results pipeline success | 880 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns success if optional jobs fail | 930 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns success if all are optional | 948 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns success if job is skipped | 963 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns yellow if there are no jobs expect skipped | 978 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns failure if any mandatory jobs fails and one job is skipped | 993 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns failure if any mandatory jobs fails | 1008 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| maps custom statuses to yellow | 1027 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws repository-changed | 1042 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getBranchStatusCheck`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no results | 1053 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns null if no matching results | 1067 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns status if name found | 1081 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns yellow if unknown status found | 1099 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `setBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should log message that branch commit SHA not found | 1121 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should log message that failed to retrieve commit pipeline | 1136 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| /api/v4/projects/some%2Frepo/statuses/0d9c7726c3d628b7e28af234595cfd20febdbf8e | 1168 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| skips setting branch status %s when RENOVATE_X_GITLAB_SKIP_STATUS_WITHOUT_PIPELINE is set and no pipeline is found | 1196 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| does not skip setting branch status when RENOVATE_X_GITLAB_SKIP_STATUS_WITHOUT_PIPELINE is not true | 1224 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| sets branch status when RENOVATE_X_GITLAB_SKIP_STATUS_WITHOUT_PIPELINE is true and pipeline is found | 1257 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| waits for 1000ms by default | 1293 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| set branch status with pipeline_id | 1322 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| waits for RENOVATE_X_GITLAB_BRANCH_STATUS_DELAY ms when set | 1357 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| do RENOVATE_X_GITLAB_BRANCH_STATUS_CHECK_ATTEMPTS attemps when set | 1402 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `findIssue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no issue | 1437 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| finds issue | 1457 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `ensureIssue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates issue | 1481 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| sets issue labels | 1506 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| updates issue | 1523 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| updates issue with labels | 1550 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| skips update if unchanged | 1578 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| creates confidential issue | 1603 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| updates confidential issue | 1629 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `ensureIssueClosing()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| closes issue | 1660 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `addAssignees(issueNo, assignees)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add the given assignee to the issue | 1683 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should add the given assignees to the issue | 1693 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should swallow error | 1709 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should log message for each assignee that could not be found | 1723 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `addReviewers(iid, reviewers) › 13.8.0`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should not be supported in too low version | 1757 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `addReviewers(iid, reviewers) › 13.9.0`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should fail to get existing reviewers | 1778 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not fail if some reviewers are unknown | 1790 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should warn and return early if new reviewers IDs could be fetched | 1812 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should add gitlab group members as reviewers to MR | 1835 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should fail to add reviewers to the MR | 1857 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should add the given reviewers to the MR | 1877 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should only add reviewers if necessary | 1897 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add comment if not found | 1918 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| add updates comment if necessary | 1934 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| skips comment | 1950 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| handles comment with no description | 1964 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes comment by topic if found | 1980 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| deletes comment by content if found | 1996 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `findPr(branchName, prTitle, state)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true if no title and all state | 2014 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns true if not open | 2038 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns true if open and with title | 2063 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns true with title | 2089 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns true with draft prefix title | 2114 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns true with deprecated draft prefix title | 2139 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| finds pr from other authors | 2164 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns null if no pr found - (includeOtherAuthors) | 2196 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `createPr(branchName, title, body)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the PR | 2236 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| uses default branch | 2268 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| supports draftPR on < 13.2 | 2300 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| supports draftPR on >= 13.2 | 2332 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| auto-accepts the MR when requested | 2364 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| adds the MR to a merge train when merge trains are enabled on the project | 2407 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| falls back to /merge endpoint when merge trains enabled but GitLab < 17.11 | 2459 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| retries the merge_trains endpoint on transient failure | 2512 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should parse merge_status attribute if detailed_merge_status is not set (on < 15.6) | 2563 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should parse detailed_merge_status attribute on >= 15.6 | 2628 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should retry auto merge creation on 405 method not allowed | 2686 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not retry if MR is mergeable and pipeline is running | 2764 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| raises with squash enabled when repository squash option is default_on | 2808 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| raises with squash enabled when repository squash option is always | 2851 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| adds approval rule to ignore all approvals | 2894 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| adds approval rule to ignore all approvals when platformAutomerge is false | 2948 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| will modify a rule of type any_approvers, if such a rule exists | 2996 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| will remove rules of type regular, if such rules exist | 3058 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| does not try to remove "report_approver" and "code_owner" approval rules | 3131 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| does not try to create already existing approval rule | 3214 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| silently ignores approval rules adding errors | 3268 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| auto-approves when enabled | 3322 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| auto-approve with different user | 3359 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should swallow an error on auto-approve | 3398 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getPr(prNo)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the PR | 3433 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| removes draft prefix from returned title | 3457 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| removes deprecated draft prefix from returned title | 3481 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns the mergeable PR | 3505 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns the PR with nonexisting branch | 3530 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns the PR with reviewers | 3558 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `updatePr(prNo, title, body)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates the PR | 3601 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| retains draft status when draft uses current prefix | 3634 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| retains draft status when draft uses deprecated prefix | 3667 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| updates target branch of the PR | 3700 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| auto-approves when enabled | 3739 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| closes the PR | 3782 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| adds and removes labels | 3821 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `reattemptPlatformAutomerge(number, platformPrOptions)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should set automatic merge | 3871 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should skip retries when merge_when_pipeline_succeeds is already enabled | 3894 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `mergePr(pr)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| merges the PR | 3916 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `massageMarkdown(input)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| strips invalid unicode null characters | 3941 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| replaces PR with MR including pluralization | 3949 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| replaces PR reference with MR reference | 3957 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| replaces PR relative link with MR reference | 3963 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| replaces issues relative link with issue reference | 3971 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| avoids false positives when replacing PR with MR | 3979 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns updated pr body | 3984 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| truncates description if too low API version | 3993 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| truncates description for API version gt 13.4 | 4003 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `deleteLabel(issueNo, label)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should delete the label | 4015 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null | 4040 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns file content | 4053 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns file content in json5 format | 4067 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns file content from given repo | 4086 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns file content from branch or tag | 4100 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws on malformed JSON | 4118 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws on errors | 4130 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `filterUnavailableUsers(users)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| filters users that are busy | 4142 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| keeps users with missing availability | 4160 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| keeps users with failing requests | 4169 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `expandGroupMembers(reviewersOrAssignees)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| expands group members for groups with members | 4180 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| users are not expanded when 404 | 4200 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| users are not expanded when non 404 | 4209 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| groups with no members expand into empty list | 4225 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| includes email in final result | 4236 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

| should throw if endpoint is not a valid URL | 82 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
---
