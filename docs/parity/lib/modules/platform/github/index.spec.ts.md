# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/github/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/github/index.spec.ts
**Total tests:** 201 | **Ported:** 0 | **Actionable:** 201 | **Status:** pending

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no token | 64 | pending | — | — | — |
| should throw if using fine-grained token with GHE <3.10 | 70 | pending | — | — | — |
| should throw if using fine-grained token with GHE unknown version | 85 | pending | — | — | — |
| should support fine-grained token with GHE >=3.10 | 97 | pending | — | — | — |
| should throw if user failure | 119 | pending | — | — | — |
| should support default endpoint no email access | 124 | pending | — | — | — |
| should support default endpoint no email result | 136 | pending | — | — | — |
| should support gitAuthor and username | 148 | pending | — | — | — |

### `initPlatform() › when using the default gitAuthor › when gitAuthor is not set › when no email access`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| if on GitHub.com, a warning is shown | 161 | pending | — | — | — |
| if on GitHub Enterprise, a warning is not shown | 186 | pending | — | — | — |

### `initPlatform() › when using the default gitAuthor › when gitAuthor is not set › when email access`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no warning is shown | 208 | pending | — | — | — |
| if on GitHub Enterprise, a warning is not shown | 231 | pending | — | — | — |

### `initPlatform() › when using the default gitAuthor › when explicitly set to only email address`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| if on GitHub.com, a warning is shown | 258 | pending | — | — | — |
| if on GitHub Enterprise, a warning is not shown | 278 | pending | — | — | — |

### `initPlatform() › when using the default gitAuthor › when explicitly set to RFC-RFC5322 format`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| if on GitHub.com, a warning is shown | 297 | pending | — | — | — |
| if on GitHub Enterprise, a warning is not shown | 317 | pending | — | — | — |
| should support default endpoint with email | 336 | pending | — | — | — |
| should use public email from user profile when available | 352 | pending | — | — | — |
| should fall back to user/emails when there is no public email | 366 | pending | — | — | — |
| should fall back gracefully when user/emails returns an error (no user:email scope) | 385 | pending | — | — | — |
| should autodetect email/user on default endpoint with GitHub App | 404 | pending | — | — | — |
| should throw error when cant request App information on default endpoint with GitHub App | 494 | pending | — | — | — |
| should autodetect email/user on custom endpoint with GitHub App | 501 | pending | — | — | — |
| should autodetect email/user on GHE Cloud endpoint with GitHub App | 528 | pending | — | — | — |
| should support custom endpoint | 554 | pending | — | — | — |
| should support custom endpoint without version | 578 | pending | — | — | — |

### `getRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return an array of repos | 604 | pending | — | — | — |
| should filters repositories by topics | 627 | pending | — | — | — |
| should return an array of repos when using Github App endpoint | 654 | pending | — | — | — |
| should return an array of repos when using GitHub App Installation Token | 681 | pending | — | — | — |

### `initRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should squash | 792 | pending | — | — | — |
| no token | 800 | pending | — | — | — |
| app token | 808 | pending | — | — | — |
| should fork when using forkToken | 817 | pending | — | — | — |
| should throw if fork needed but forkCreation=false | 835 | pending | — | — | — |
| throws if the repo is a fork | 850 | pending | — | — | — |
| throws when cannot fork due to username error | 864 | pending | — | — | — |
| throws when listing forks with 404 | 879 | pending | — | — | — |
| throws when listing forks with 500 | 892 | pending | — | — | — |
| throws when error creating fork | 905 | pending | — | — | — |
| should update fork when using forkToken and forkOrg | 923 | pending | — | — | — |
| detects fork default branch mismatch | 935 | pending | — | — | — |
| should merge | 951 | pending | — | — | — |
| should rebase | 980 | pending | — | — | — |
| should not guess at merge | 1007 | pending | — | — | — |
| should throw error if archived | 1027 | pending | — | — | — |
| throws not-found | 1051 | pending | — | — | — |
| throws unexpected graphql errors | 1058 | pending | — | — | — |
| throws graphql rate limit error | 1075 | pending | — | — | — |
| should throw error if renamed | 1092 | pending | — | — | — |
| should not be case sensitive | 1115 | pending | — | — | — |

### `getBranchForceRebase`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should detect repoForceRebase | 1142 | pending | — | — | — |
| should handle 404 | 1176 | pending | — | — | — |
| should handle 403 | 1189 | pending | — | — | — |
| should throw 401 | 1202 | pending | — | — | — |
| should return empty object when parentRepo is set | 1216 | pending | — | — | — |
| should ignore non_fast_forward ruleset for determining rebase | 1236 | pending | — | — | — |
| should detect strict required status checks ruleset | 1260 | pending | — | — | — |
| should continue if no expected rulesets have been found | 1279 | pending | — | — | — |
| should abort and throws on internal error | 1300 | pending | — | — | — |
| should fallback to legacy branch protection when rulesets not found | 1311 | pending | — | — | — |
| should return false when no force rebase rules found | 1328 | pending | — | — | — |
| should return cached result on subsequent calls | 1351 | pending | — | — | — |
| should return cached false result on subsequent calls | 1376 | pending | — | — | — |

### `getPrList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fetches single page | 1450 | pending | — | — | — |
| fetches multiple pages | 1461 | pending | — | — | — |
| synchronizes cache | 1480 | pending | — | — | — |

### `getPrList() › Body compaction`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| compacts body from response | 1533 | pending | — | — | — |

### `getPrList() › PR author filtering`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| filters PRs by renovate username when no forkToken or ignorePrAuthor | 1578 | pending | — | — | — |
| fetches all PRs when forkToken is set | 1593 | pending | — | — | — |
| fetches all PRs when ignorePrAuthor is set | 1620 | pending | — | — | — |
| stops sync early when non-Renovate PRs dominate | 1639 | pending | — | — | — |
| advances watermark from unfiltered page so next sync is cheaper | 1685 | pending | — | — | — |
| derives cutoff from cached items when lastModified is missing | 1742 | pending | — | — | — |
| stops at max sync pages | 1808 | pending | — | — | — |
| stops at custom max sync pages | 1852 | pending | — | — | — |
| reconciles mixed pages with both Renovate and non-Renovate PRs | 1897 | pending | — | — | — |
| continues past timestamp tie at page boundary | 1948 | pending | — | — | — |

### `getBranchPr(branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null if no PR exists | 1998 | pending | — | — | — |
| should cache and return the PR object | 2012 | pending | — | — | — |

### `tryReuseAutoclosedPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should reopen autoclosed PR | 2047 | pending | — | — | — |
| force pushes when local SHA differs from PR SHA | 2082 | pending | — | — | — |
| aborts reopening if branch recreation fails | 2126 | pending | — | — | — |
| aborts reopening if PR reopening fails | 2155 | pending | — | — | — |

### `getBranchStatus()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should pass through success | 2179 | pending | — | — | — |
| should not consider internal statuses as success | 2195 | pending | — | — | — |
| should pass through failed | 2217 | pending | — | — | — |
| defaults to pending | 2233 | pending | — | — | — |
| should fail if a check run has failed | 2248 | pending | — | — | — |
| should succeed if no status and all passed check runs | 2280 | pending | — | — | — |
| should fail if a check run is pending | 2318 | pending | — | — | — |

### `getBranchStatusCheck`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns state if found | 2351 | pending | — | — | — |
| returns null | 2380 | pending | — | — | — |
| returns yellow if state not present in context object | 2406 | pending | — | — | — |

### `setBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if already set | 2425 | pending | — | — | — |
| sets branch status | 2450 | pending | — | — | — |

### `getIssue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if issues disabled | 2496 | pending | — | — | — |
| returns issue | 2504 | pending | — | — | — |
| returns null if issue not found | 2524 | pending | — | — | — |
| logs debug message if issue deleted | 2533 | pending | — | — | — |

### `findIssue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no issue | 2548 | pending | — | — | — |
| finds issue | 2585 | pending | — | — | — |

### `ensureIssue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates issue | 2638 | pending | — | — | — |
| creates issue if not ensuring only once | 2688 | pending | — | — | — |
| does not create issue if ensuring only once | 2732 | pending | — | — | — |
| creates issue with labels | 2774 | pending | — | — | — |
| closes others if ensuring only once | 2810 | pending | — | — | — |
| updates issue | 2863 | pending | — | — | — |
| updates issue with labels | 2922 | pending | — | — | — |
| skips update if unchanged | 2982 | pending | — | — | — |
| deletes if duplicate | 3026 | pending | — | — | — |
| creates issue if reopen flag false and issue is not open | 3070 | pending | — | — | — |
| does not create issue if reopen flag false and issue is already open | 3123 | pending | — | — | — |

### `ensureIssueClosing()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| closes issue | 3170 | pending | — | — | — |
| swallows 410 Gone when the issue was deleted on the platform | 3214 | pending | — | — | — |
| swallows 404 Not Found when the issue was deleted on the platform | 3245 | pending | — | — | — |
| rethrows non-deletion errors | 3276 | pending | — | — | — |

### `deleteLabel(issueNo, label)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should delete the label | 3309 | pending | — | — | — |

### `addAssignees(issueNo, assignees)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add the given assignees to the issue | 3319 | pending | — | — | — |
| should retry on 404 and succeed | 3335 | pending | — | — | — |
| should throw after 3 consecutive 404 responses | 3355 | pending | — | — | — |
| should throw immediately on non-404 errors | 3365 | pending | — | — | — |

### `addReviewers(issueNo, reviewers)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add the given reviewers to the PR | 3377 | pending | — | — | — |

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add comment if not found | 3389 | pending | — | — | — |
| adds comment if found in closed PR list | 3408 | pending | — | — | — |
| add updates comment if necessary | 3436 | pending | — | — | — |
| skips comment | 3455 | pending | — | — | — |
| handles comment with no description | 3472 | pending | — | — | — |

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes comment by topic if found | 3491 | pending | — | — | — |
| deletes comment by content if found | 3510 | pending | — | — | — |

### `findPr(branchName, prTitle, state)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| finds PR by branch name | 3531 | pending | — | — | — |
| finds PR with non-open state | 3573 | pending | — | — | — |
| skips PR with non-matching state | 3602 | pending | — | — | — |
| skips PRs from forks | 3628 | pending | — | — | — |
| skips PR with non-matching title | 3653 | pending | — | — | — |
| caches pr list | 3678 | pending | — | — | — |
| finds pr from other authors | 3713 | pending | — | — | — |
| returns null if no pr found - (includeOtherAuthors) | 3743 | pending | — | — | — |

### `createPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create and return a PR object | 3760 | pending | — | — | — |
| should use defaultBranch | 3782 | pending | — | — | — |
| should create a draftPR if set in the settings | 3800 | pending | — | — | — |

### `createPr() › with forkToken`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should allow maintainer edits if explicitly enabled via options | 3840 | pending | — | — | — |
| should allow maintainer edits if not explicitly set | 3864 | pending | — | — | — |
| should disallow maintainer edits if explicitly disabled | 3885 | pending | — | — | — |

### `createPr() › automerge`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should skip automerge if disabled in repo settings | 4000 | pending | — | — | — |
| should skip automerge if GHE <3.3.0 | 4013 | pending | — | — | — |
| should perform automerge if GHE >=3.3.0 | 4048 | pending | — | — | — |
| should set automatic merge | 4094 | pending | — | — | — |
| should handle GraphQL errors | 4109 | pending | — | — | — |
| should handle REST API errors | 4122 | pending | — | — | — |

### `createPr() › milestone`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should set the milestone on the PR | 4137 | pending | — | — | — |
| should log a warning but not throw on error | 4169 | pending | — | — | — |

### `getPr(prNo)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null if no prNo is passed | 4231 | pending | — | — | — |
| should return PR | 4236 | pending | — | — | — |
| should return closed PR | 4279 | pending | — | — | — |
| should return merged PR | 4304 | pending | — | — | — |
| should return null if no PR is returned from GitHub | 4330 | pending | — | — | — |
| should return a PR object - 0 | 4345 | pending | — | — | — |
| should return a PR object - 1 | 4371 | pending | — | — | — |
| should return a PR object - 2 | 4407 | pending | — | — | — |

### `updatePr(prNo, title, body)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update the PR | 4441 | pending | — | — | — |
| should update and close the PR | 4455 | pending | — | — | — |
| should update target branch | 4470 | pending | — | — | — |
| should add and remove labels | 4486 | pending | — | — | — |

### `updatePr(prNo, title, body) › addLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| warns if adding labels failed | 4526 | pending | — | — | — |

### `reattemptPlatformAutomerge(number, platformPrOptions)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should set automatic merge | 4630 | pending | — | — | — |
| handles unknown error | 4648 | pending | — | — | — |

### `mergePr(prNo)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should merge the PR | 4670 | pending | — | — | — |
| should handle merge error | 4702 | pending | — | — | — |
| should handle merge block | 4723 | pending | — | — | — |
| should handle approvers required | 4745 | pending | — | — | — |
| should warn if automergeStrategy is not supported | 4767 | pending | — | — | — |
| should use configured automergeStrategy | 4786 | pending | — | — | — |

### `massageMarkdown(input)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns updated pr body | 4813 | pending | — | — | — |
| returns not-updated pr body for GHE | 4819 | pending | — | — | — |

### `mergePr(prNo) - autodetection`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should try squash first | 4846 | pending | — | — | — |
| should try merge after squash | 4865 | pending | — | — | — |
| should try rebase after merge | 4886 | pending | — | — | — |
| should give up | 4911 | pending | — | — | — |

### `getVulnerabilityAlerts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| avoids fetching if repo has vulnerability alerts disabled | 4940 | pending | — | — | — |
| returns empty if error | 4950 | pending | — | — | — |
| returns array if found | 4963 | pending | — | — | — |
| returns empty if disabled | 5013 | pending | — | — | — |
| handles network error | 5027 | pending | — | — | — |
| calls logger.debug with only items that include securityVulnerability | 5041 | pending | — | — | — |
| returns normalized names for PIP ecosystem | 5097 | pending | — | — | — |
| handles pagination correctly | 5133 | pending | — | — | — |

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null | 5232 | pending | — | — | — |
| returns file content | 5243 | pending | — | — | — |
| returns file content in json5 format | 5255 | pending | — | — | — |
| returns file content from given repo | 5272 | pending | — | — | — |
| returns file content from branch or tag | 5284 | pending | — | — | — |
| throws on malformed JSON | 5296 | pending | — | — | — |
| throws on errors | 5306 | pending | — | — | — |

### `pushFiles`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if pre-commit phase has failed | 5332 | pending | — | — | — |
| returns null on REST error | 5352 | pending | — | — | — |
| commits and returns SHA string | 5367 | pending | — | — | — |
| performs rebase | 5396 | pending | — | — | — |
| continues if rebase fails due to 422 | 5425 | pending | — | — | — |
| aborts if rebase fails due to non-422 | 5456 | pending | — | — | — |
| aborts if commit SHA doesn't exist | 5485 | pending | — | — | — |

---

