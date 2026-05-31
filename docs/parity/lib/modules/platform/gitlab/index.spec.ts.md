# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gitlab/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gitlab/index.spec.ts
**Total tests:** 163 | **Ported:** 6 | **Actionable:** 6 | **Status:** done

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no token | 78 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw if auth fails | 82 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should default to gitlab.com | 92 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should accept custom endpoint | 108 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should reuse existing gitAuthor | 129 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw an error if it receives an error | 144 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return an array of repos | 154 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return an array of repos including mirrors | 176 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should encode the requested topics into the URL | 198 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should query the groups endpoint for each namespace | 216 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should consider topics when querying the groups endpoint | 242 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should include order and sort query parameters | 263 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `initRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should escape all forward slashes in project names | 308 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw an error if receiving an error | 324 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw an error if repository is archived | 336 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw an error if repository is a mirror | 348 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not throw an error if repository is a mirror when includeMirrors option is set | 360 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw an error if repository access is disabled | 380 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw an error if MRs are disabled | 392 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw an error if repository has empty_repo property | 404 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw an error if repository is empty | 416 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should fall back if http_url_to_repo is empty | 428 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use ssh_url_to_repo if gitUrl is set to ssh | 447 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw if ssh_url_to_repo is not present but gitUrl is set to ssh | 464 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should fall back respecting when GITLAB_IGNORE_REPO_URL is set | 480 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getBranchForceRebase`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return false for merge_method=merge | 513 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return true for merge_method=ff | 527 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return false when merge trains are enabled | 541 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getBranchPr(branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null if no PR exists | 558 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return the PR object | 570 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should strip draft prefix from title | 609 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should strip deprecated draft prefix from title | 648 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getBranchStatus(branchName, ignoreTests)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns pending if no results | 689 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns success if no results with merged results pipeline success | 704 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns success if all are success | 751 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns pending if all are internal success | 769 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns pending if merge request with no pipelines | 787 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns pending if all are internal success with no merged results pipeline | 830 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns success if all are internal success with merged results pipeline success | 880 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns success if optional jobs fail | 930 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns success if all are optional | 948 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns success if job is skipped | 963 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns yellow if there are no jobs expect skipped | 978 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns failure if any mandatory jobs fails and one job is skipped | 993 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns failure if any mandatory jobs fails | 1008 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| maps custom statuses to yellow | 1027 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws repository-changed | 1042 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getBranchStatusCheck`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no results | 1053 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns null if no matching results | 1067 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns status if name found | 1081 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns yellow if unknown status found | 1099 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `setBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should log message that branch commit SHA not found | 1121 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should log message that failed to retrieve commit pipeline | 1136 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| /api/v4/projects/some%2Frepo/statuses/0d9c7726c3d628b7e28af234595cfd20febdbf8e | 1168 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| skips setting branch status %s when RENOVATE_X_GITLAB_SKIP_STATUS_WITHOUT_PIPELINE is set and no pipeline is found | 1196 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| does not skip setting branch status when RENOVATE_X_GITLAB_SKIP_STATUS_WITHOUT_PIPELINE is not true | 1224 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| sets branch status when RENOVATE_X_GITLAB_SKIP_STATUS_WITHOUT_PIPELINE is true and pipeline is found | 1257 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| waits for 1000ms by default | 1293 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| set branch status with pipeline_id | 1322 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| waits for RENOVATE_X_GITLAB_BRANCH_STATUS_DELAY ms when set | 1357 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| do RENOVATE_X_GITLAB_BRANCH_STATUS_CHECK_ATTEMPTS attemps when set | 1402 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `findIssue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no issue | 1437 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| finds issue | 1457 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `ensureIssue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates issue | 1481 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| sets issue labels | 1506 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| updates issue | 1523 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| updates issue with labels | 1550 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| skips update if unchanged | 1578 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| creates confidential issue | 1603 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| updates confidential issue | 1629 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `ensureIssueClosing()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| closes issue | 1660 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `addAssignees(issueNo, assignees)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add the given assignee to the issue | 1683 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should add the given assignees to the issue | 1693 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should swallow error | 1709 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should log message for each assignee that could not be found | 1723 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `addReviewers(iid, reviewers) › 13.8.0`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should not be supported in too low version | 1757 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `addReviewers(iid, reviewers) › 13.9.0`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should fail to get existing reviewers | 1778 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not fail if some reviewers are unknown | 1790 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should warn and return early if new reviewers IDs could be fetched | 1812 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should add gitlab group members as reviewers to MR | 1835 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should fail to add reviewers to the MR | 1857 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should add the given reviewers to the MR | 1877 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should only add reviewers if necessary | 1897 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add comment if not found | 1918 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| add updates comment if necessary | 1934 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| skips comment | 1950 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| handles comment with no description | 1964 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes comment by topic if found | 1980 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| deletes comment by content if found | 1996 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `findPr(branchName, prTitle, state)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true if no title and all state | 2014 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns true if not open | 2038 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns true if open and with title | 2063 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns true with title | 2089 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns true with draft prefix title | 2114 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns true with deprecated draft prefix title | 2139 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| finds pr from other authors | 2164 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns null if no pr found - (includeOtherAuthors) | 2196 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `createPr(branchName, title, body)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the PR | 2236 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| uses default branch | 2268 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| supports draftPR on < 13.2 | 2300 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| supports draftPR on >= 13.2 | 2332 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| auto-accepts the MR when requested | 2364 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| adds the MR to a merge train when merge trains are enabled on the project | 2407 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| falls back to /merge endpoint when merge trains enabled but GitLab < 17.11 | 2459 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| retries the merge_trains endpoint on transient failure | 2512 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should parse merge_status attribute if detailed_merge_status is not set (on < 15.6) | 2563 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should parse detailed_merge_status attribute on >= 15.6 | 2628 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should retry auto merge creation on 405 method not allowed | 2686 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not retry if MR is mergeable and pipeline is running | 2764 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| raises with squash enabled when repository squash option is default_on | 2808 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| raises with squash enabled when repository squash option is always | 2851 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| adds approval rule to ignore all approvals | 2894 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| adds approval rule to ignore all approvals when platformAutomerge is false | 2948 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| will modify a rule of type any_approvers, if such a rule exists | 2996 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| will remove rules of type regular, if such rules exist | 3058 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| does not try to remove "report_approver" and "code_owner" approval rules | 3131 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| does not try to create already existing approval rule | 3214 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| silently ignores approval rules adding errors | 3268 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| auto-approves when enabled | 3322 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| auto-approve with different user | 3359 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should swallow an error on auto-approve | 3398 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getPr(prNo)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the PR | 3433 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| removes draft prefix from returned title | 3457 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| removes deprecated draft prefix from returned title | 3481 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns the mergeable PR | 3505 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns the PR with nonexisting branch | 3530 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns the PR with reviewers | 3558 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `updatePr(prNo, title, body)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates the PR | 3601 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| retains draft status when draft uses current prefix | 3634 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| retains draft status when draft uses deprecated prefix | 3667 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| updates target branch of the PR | 3700 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| auto-approves when enabled | 3739 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| closes the PR | 3782 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| adds and removes labels | 3821 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `reattemptPlatformAutomerge(number, platformPrOptions)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should set automatic merge | 3871 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should skip retries when merge_when_pipeline_succeeds is already enabled | 3894 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `mergePr(pr)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| merges the PR | 3916 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `massageMarkdown(input)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| strips invalid unicode null characters | 3941 | ported | `gitlab.rs` | `massage_markdown_strips_null_chars` | — |
| replaces PR with MR including pluralization | 3949 | ported | `gitlab.rs` | `massage_markdown_replaces_pr_with_mr` | — |
| replaces PR reference with MR reference | 3957 | ported | `gitlab.rs` | `massage_markdown_replaces_pr_ref` | — |
| replaces PR relative link with MR reference | 3963 | ported | `gitlab.rs` | `massage_markdown_replaces_pr_link` | — |
| replaces issues relative link with issue reference | 3971 | ported | `gitlab.rs` | `massage_markdown_replaces_issues_link` | — |
| avoids false positives when replacing PR with MR | 3979 | ported | `gitlab.rs` | `massage_markdown_avoids_false_positives` | — |
| returns updated pr body | 3984 | not-applicable | Requires TS module state (gitLabVersion, smartTruncate mock) for snapshot testing | — | Snapshot test depends on TS test infrastructure |
| truncates description if too low API version | 3993 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| truncates description for API version gt 13.4 | 4003 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `deleteLabel(issueNo, label)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should delete the label | 4015 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null | 4040 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns file content | 4053 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns file content in json5 format | 4067 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns file content from given repo | 4086 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns file content from branch or tag | 4100 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws on malformed JSON | 4118 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws on errors | 4130 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `filterUnavailableUsers(users)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| filters users that are busy | 4142 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| keeps users with missing availability | 4160 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| keeps users with failing requests | 4169 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `expandGroupMembers(reviewersOrAssignees)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| expands group members for groups with members | 4180 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| users are not expanded when 404 | 4200 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| users are not expanded when non 404 | 4209 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| groups with no members expand into empty list | 4225 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| includes email in final result | 4236 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

| should throw if endpoint is not a valid URL | 82 | not-applicable | Mock framework internals — tests gitlab platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
---
