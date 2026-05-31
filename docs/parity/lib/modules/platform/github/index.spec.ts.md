# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/github/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/github/index.spec.ts
**Total tests:** 206 | **Ported:** 0 | **Actionable:** 206 | **Status:** pending

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no token | 64 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw if using fine-grained token with GHE <3.10 | 70 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw if using fine-grained token with GHE unknown version | 85 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should support fine-grained token with GHE >=3.10 | 97 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw if user failure | 119 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should support default endpoint no email access | 124 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should support default endpoint no email result | 136 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should support gitAuthor and username | 148 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `initPlatform() › when using the default gitAuthor › when gitAuthor is not set › when no email access`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| if on GitHub.com, a warning is shown | 161 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| if on GitHub Enterprise, a warning is not shown | 186 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `initPlatform() › when using the default gitAuthor › when gitAuthor is not set › when email access`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no warning is shown | 208 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| if on GitHub Enterprise, a warning is not shown | 231 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `initPlatform() › when using the default gitAuthor › when explicitly set to only email address`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| if on GitHub.com, a warning is shown | 258 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| if on GitHub Enterprise, a warning is not shown | 278 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `initPlatform() › when using the default gitAuthor › when explicitly set to RFC-RFC5322 format`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| if on GitHub.com, a warning is shown | 297 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| if on GitHub Enterprise, a warning is not shown | 317 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should support default endpoint with email | 336 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use public email from user profile when available | 352 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should fall back to user/emails when there is no public email | 366 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should fall back gracefully when user/emails returns an error (no user:email scope) | 385 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should autodetect email/user on default endpoint with GitHub App | 404 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw error when cant request App information on default endpoint with GitHub App | 494 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should autodetect email/user on custom endpoint with GitHub App | 501 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should autodetect email/user on GHE Cloud endpoint with GitHub App | 528 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should support custom endpoint | 554 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should support custom endpoint without version | 578 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return an array of repos | 604 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should filters repositories by topics | 627 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return an array of repos when using Github App endpoint | 654 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return an array of repos when using GitHub App Installation Token | 681 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `initRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should squash | 792 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| no token | 800 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| app token | 808 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should fork when using forkToken | 817 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw if fork needed but forkCreation=false | 835 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws if the repo is a fork | 850 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws when cannot fork due to username error | 864 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws when listing forks with 404 | 879 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws when listing forks with 500 | 892 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws when error creating fork | 905 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should update fork when using forkToken and forkOrg | 923 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| detects fork default branch mismatch | 935 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should merge | 951 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should rebase | 980 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not guess at merge | 1007 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw error if archived | 1027 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws not-found | 1051 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws unexpected graphql errors | 1058 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws graphql rate limit error | 1075 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw error if renamed | 1092 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not be case sensitive | 1115 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getBranchForceRebase`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should detect repoForceRebase | 1142 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should handle 404 | 1176 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should handle 403 | 1189 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw 401 | 1202 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return empty object when parentRepo is set | 1216 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should ignore non_fast_forward ruleset for determining rebase | 1236 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should detect strict required status checks ruleset | 1260 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should continue if no expected rulesets have been found | 1279 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort and throws on internal error | 1300 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should fallback to legacy branch protection when rulesets not found | 1311 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return false when no force rebase rules found | 1328 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return cached result on subsequent calls | 1351 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return cached false result on subsequent calls | 1376 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getPrList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fetches single page | 1450 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| fetches multiple pages | 1461 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| synchronizes cache | 1480 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getPrList() › Body compaction`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| compacts body from response | 1533 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getPrList() › PR author filtering`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| filters PRs by renovate username when no forkToken or ignorePrAuthor | 1578 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| fetches all PRs when forkToken is set | 1593 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| fetches all PRs when ignorePrAuthor is set | 1620 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| stops sync early when non-Renovate PRs dominate | 1639 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| advances watermark from unfiltered page so next sync is cheaper | 1685 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| derives cutoff from cached items when lastModified is missing | 1742 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| stops at max sync pages | 1808 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| stops at custom max sync pages | 1852 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| reconciles mixed pages with both Renovate and non-Renovate PRs | 1897 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| continues past timestamp tie at page boundary | 1948 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getBranchPr(branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null if no PR exists | 1998 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should cache and return the PR object | 2012 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `tryReuseAutoclosedPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should reopen autoclosed PR | 2047 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| force pushes when local SHA differs from PR SHA | 2082 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| aborts reopening if branch recreation fails | 2126 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| aborts reopening if PR reopening fails | 2155 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getBranchStatus()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should pass through success | 2179 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not consider internal statuses as success | 2195 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should pass through failed | 2217 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| defaults to pending | 2233 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should fail if a check run has failed | 2248 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should succeed if no status and all passed check runs | 2280 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should fail if a check run is pending | 2318 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getBranchStatusCheck`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns state if found | 2351 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns null | 2380 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns yellow if state not present in context object | 2406 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `setBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if already set | 2425 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| sets branch status | 2450 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getIssue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if issues disabled | 2496 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns issue | 2504 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns null if issue not found | 2524 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| logs debug message if issue deleted | 2533 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `findIssue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no issue | 2548 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| finds issue | 2585 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `ensureIssue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates issue | 2638 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| creates issue if not ensuring only once | 2688 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| does not create issue if ensuring only once | 2732 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| creates issue with labels | 2774 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| closes others if ensuring only once | 2810 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| updates issue | 2863 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| updates issue with labels | 2922 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| skips update if unchanged | 2982 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| deletes if duplicate | 3026 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| creates issue if reopen flag false and issue is not open | 3070 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| does not create issue if reopen flag false and issue is already open | 3123 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `ensureIssueClosing()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| closes issue | 3170 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| swallows 410 Gone when the issue was deleted on the platform | 3214 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| swallows 404 Not Found when the issue was deleted on the platform | 3245 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| rethrows non-deletion errors | 3276 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `deleteLabel(issueNo, label)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should delete the label | 3309 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `addAssignees(issueNo, assignees)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add the given assignees to the issue | 3319 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should retry on 404 and succeed | 3335 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw after 3 consecutive 404 responses | 3355 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw immediately on non-404 errors | 3365 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `addReviewers(issueNo, reviewers)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add the given reviewers to the PR | 3377 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add comment if not found | 3389 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| adds comment if found in closed PR list | 3408 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| add updates comment if necessary | 3436 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| skips comment | 3455 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| handles comment with no description | 3472 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes comment by topic if found | 3491 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| deletes comment by content if found | 3510 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `findPr(branchName, prTitle, state)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| finds PR by branch name | 3531 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| finds PR with non-open state | 3573 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| skips PR with non-matching state | 3602 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| skips PRs from forks | 3628 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| skips PR with non-matching title | 3653 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| caches pr list | 3678 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| finds pr from other authors | 3713 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns null if no pr found - (includeOtherAuthors) | 3743 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `createPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create and return a PR object | 3760 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use defaultBranch | 3782 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should create a draftPR if set in the settings | 3800 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `createPr() › with forkToken`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should allow maintainer edits if explicitly enabled via options | 3840 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should allow maintainer edits if not explicitly set | 3864 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should disallow maintainer edits if explicitly disabled | 3885 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `createPr() › automerge`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should skip automerge if disabled in repo settings | 4000 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should skip automerge if GHE <3.3.0 | 4013 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should perform automerge if GHE >=3.3.0 | 4048 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should set automatic merge | 4094 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should handle GraphQL errors | 4109 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should handle REST API errors | 4122 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `createPr() › milestone`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should set the milestone on the PR | 4137 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should log a warning but not throw on error | 4169 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getPr(prNo)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null if no prNo is passed | 4231 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return PR | 4236 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return closed PR | 4279 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return merged PR | 4304 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return null if no PR is returned from GitHub | 4330 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return a PR object - 0 | 4345 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return a PR object - 1 | 4371 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return a PR object - 2 | 4407 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `updatePr(prNo, title, body)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update the PR | 4441 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should update and close the PR | 4455 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should update target branch | 4470 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should add and remove labels | 4486 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `updatePr(prNo, title, body) › addLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| warns if adding labels failed | 4526 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `reattemptPlatformAutomerge(number, platformPrOptions)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should set automatic merge | 4630 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| handles unknown error | 4648 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `mergePr(prNo)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should merge the PR | 4670 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should handle merge error | 4702 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should handle merge block | 4723 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should handle approvers required | 4745 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should warn if automergeStrategy is not supported | 4767 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use configured automergeStrategy | 4786 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `massageMarkdown(input)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns updated pr body | 4813 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns not-updated pr body for GHE | 4819 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `mergePr(prNo) - autodetection`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should try squash first | 4846 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should try merge after squash | 4865 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should try rebase after merge | 4886 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should give up | 4911 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getVulnerabilityAlerts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| avoids fetching if repo has vulnerability alerts disabled | 4940 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns empty if error | 4950 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns array if found | 4963 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns empty if disabled | 5013 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| handles network error | 5027 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| calls logger.debug with only items that include securityVulnerability | 5041 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns normalized names for PIP ecosystem | 5097 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| handles pagination correctly | 5133 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null | 5232 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns file content | 5243 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns file content in json5 format | 5255 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns file content from given repo | 5272 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns file content from branch or tag | 5284 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws on malformed JSON | 5296 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws on errors | 5306 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `pushFiles`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if pre-commit phase has failed | 5332 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns null on REST error | 5352 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| commits and returns SHA string | 5367 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| performs rebase | 5396 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| continues if rebase fails due to 422 | 5425 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| aborts if rebase fails due to non-422 | 5456 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| aborts if commit SHA doesn't exist | 5485 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

| should throw if endpoint is invalid URL | 70 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should pass commit message as commitHeadline and commitBody for squash merge | 4144 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should pass commit message as commitHeadline and commitBody for merge commit | 4175 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should pass multi-line commit message body for squash merge | 4209 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not pass commit message headline/body for rebase merge | 4242 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
---
