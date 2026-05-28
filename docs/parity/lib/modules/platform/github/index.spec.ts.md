# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/github/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/github/index.spec.ts
**Total tests:** 206 | **Ported:** 0 | **Actionable:** 206 | **Status:** not-applicable

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no token | 64 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw if using fine-grained token with GHE <3.10 | 70 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw if using fine-grained token with GHE unknown version | 85 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should support fine-grained token with GHE >=3.10 | 97 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw if user failure | 119 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should support default endpoint no email access | 124 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should support default endpoint no email result | 136 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should support gitAuthor and username | 148 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `initPlatform() › when using the default gitAuthor › when gitAuthor is not set › when no email access`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| if on GitHub.com, a warning is shown | 161 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| if on GitHub Enterprise, a warning is not shown | 186 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `initPlatform() › when using the default gitAuthor › when gitAuthor is not set › when email access`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no warning is shown | 208 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| if on GitHub Enterprise, a warning is not shown | 231 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `initPlatform() › when using the default gitAuthor › when explicitly set to only email address`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| if on GitHub.com, a warning is shown | 258 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| if on GitHub Enterprise, a warning is not shown | 278 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `initPlatform() › when using the default gitAuthor › when explicitly set to RFC-RFC5322 format`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| if on GitHub.com, a warning is shown | 297 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| if on GitHub Enterprise, a warning is not shown | 317 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should support default endpoint with email | 336 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should use public email from user profile when available | 352 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should fall back to user/emails when there is no public email | 366 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should fall back gracefully when user/emails returns an error (no user:email scope) | 385 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should autodetect email/user on default endpoint with GitHub App | 404 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw error when cant request App information on default endpoint with GitHub App | 494 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should autodetect email/user on custom endpoint with GitHub App | 501 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should autodetect email/user on GHE Cloud endpoint with GitHub App | 528 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should support custom endpoint | 554 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should support custom endpoint without version | 578 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `getRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return an array of repos | 604 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should filters repositories by topics | 627 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should return an array of repos when using Github App endpoint | 654 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should return an array of repos when using GitHub App Installation Token | 681 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `initRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should squash | 792 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| no token | 800 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| app token | 808 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should fork when using forkToken | 817 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw if fork needed but forkCreation=false | 835 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws if the repo is a fork | 850 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws when cannot fork due to username error | 864 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws when listing forks with 404 | 879 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws when listing forks with 500 | 892 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws when error creating fork | 905 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should update fork when using forkToken and forkOrg | 923 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| detects fork default branch mismatch | 935 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should merge | 951 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should rebase | 980 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should not guess at merge | 1007 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw error if archived | 1027 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws not-found | 1051 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws unexpected graphql errors | 1058 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws graphql rate limit error | 1075 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw error if renamed | 1092 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should not be case sensitive | 1115 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `getBranchForceRebase`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should detect repoForceRebase | 1142 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should handle 404 | 1176 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should handle 403 | 1189 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw 401 | 1202 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should return empty object when parentRepo is set | 1216 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should ignore non_fast_forward ruleset for determining rebase | 1236 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should detect strict required status checks ruleset | 1260 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should continue if no expected rulesets have been found | 1279 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should abort and throws on internal error | 1300 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should fallback to legacy branch protection when rulesets not found | 1311 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should return false when no force rebase rules found | 1328 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should return cached result on subsequent calls | 1351 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should return cached false result on subsequent calls | 1376 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `getPrList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fetches single page | 1450 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| fetches multiple pages | 1461 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| synchronizes cache | 1480 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `getPrList() › Body compaction`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| compacts body from response | 1533 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `getPrList() › PR author filtering`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| filters PRs by renovate username when no forkToken or ignorePrAuthor | 1578 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| fetches all PRs when forkToken is set | 1593 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| fetches all PRs when ignorePrAuthor is set | 1620 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| stops sync early when non-Renovate PRs dominate | 1639 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| advances watermark from unfiltered page so next sync is cheaper | 1685 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| derives cutoff from cached items when lastModified is missing | 1742 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| stops at max sync pages | 1808 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| stops at custom max sync pages | 1852 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| reconciles mixed pages with both Renovate and non-Renovate PRs | 1897 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| continues past timestamp tie at page boundary | 1948 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `getBranchPr(branchName)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null if no PR exists | 1998 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should cache and return the PR object | 2012 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `tryReuseAutoclosedPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should reopen autoclosed PR | 2047 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| force pushes when local SHA differs from PR SHA | 2082 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| aborts reopening if branch recreation fails | 2126 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| aborts reopening if PR reopening fails | 2155 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `getBranchStatus()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should pass through success | 2179 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should not consider internal statuses as success | 2195 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should pass through failed | 2217 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| defaults to pending | 2233 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should fail if a check run has failed | 2248 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should succeed if no status and all passed check runs | 2280 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should fail if a check run is pending | 2318 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `getBranchStatusCheck`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns state if found | 2351 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns null | 2380 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns yellow if state not present in context object | 2406 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `setBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if already set | 2425 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| sets branch status | 2450 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `getIssue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if issues disabled | 2496 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns issue | 2504 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns null if issue not found | 2524 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| logs debug message if issue deleted | 2533 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `findIssue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no issue | 2548 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| finds issue | 2585 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `ensureIssue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates issue | 2638 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| creates issue if not ensuring only once | 2688 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| does not create issue if ensuring only once | 2732 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| creates issue with labels | 2774 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| closes others if ensuring only once | 2810 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| updates issue | 2863 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| updates issue with labels | 2922 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| skips update if unchanged | 2982 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| deletes if duplicate | 3026 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| creates issue if reopen flag false and issue is not open | 3070 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| does not create issue if reopen flag false and issue is already open | 3123 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `ensureIssueClosing()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| closes issue | 3170 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| swallows 410 Gone when the issue was deleted on the platform | 3214 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| swallows 404 Not Found when the issue was deleted on the platform | 3245 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| rethrows non-deletion errors | 3276 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `deleteLabel(issueNo, label)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should delete the label | 3309 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `addAssignees(issueNo, assignees)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add the given assignees to the issue | 3319 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should retry on 404 and succeed | 3335 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw after 3 consecutive 404 responses | 3355 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw immediately on non-404 errors | 3365 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `addReviewers(issueNo, reviewers)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add the given reviewers to the PR | 3377 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add comment if not found | 3389 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| adds comment if found in closed PR list | 3408 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| add updates comment if necessary | 3436 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| skips comment | 3455 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| handles comment with no description | 3472 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes comment by topic if found | 3491 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| deletes comment by content if found | 3510 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `findPr(branchName, prTitle, state)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| finds PR by branch name | 3531 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| finds PR with non-open state | 3573 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| skips PR with non-matching state | 3602 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| skips PRs from forks | 3628 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| skips PR with non-matching title | 3653 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| caches pr list | 3678 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| finds pr from other authors | 3713 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns null if no pr found - (includeOtherAuthors) | 3743 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `createPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create and return a PR object | 3760 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should use defaultBranch | 3782 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should create a draftPR if set in the settings | 3800 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `createPr() › with forkToken`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should allow maintainer edits if explicitly enabled via options | 3840 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should allow maintainer edits if not explicitly set | 3864 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should disallow maintainer edits if explicitly disabled | 3885 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `createPr() › automerge`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should skip automerge if disabled in repo settings | 4000 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should skip automerge if GHE <3.3.0 | 4013 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should perform automerge if GHE >=3.3.0 | 4048 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should set automatic merge | 4094 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should handle GraphQL errors | 4109 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should handle REST API errors | 4122 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `createPr() › milestone`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should set the milestone on the PR | 4137 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should log a warning but not throw on error | 4169 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `getPr(prNo)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null if no prNo is passed | 4231 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should return PR | 4236 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should return closed PR | 4279 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should return merged PR | 4304 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should return null if no PR is returned from GitHub | 4330 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should return a PR object - 0 | 4345 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should return a PR object - 1 | 4371 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should return a PR object - 2 | 4407 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `updatePr(prNo, title, body)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update the PR | 4441 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should update and close the PR | 4455 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should update target branch | 4470 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should add and remove labels | 4486 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `updatePr(prNo, title, body) › addLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| warns if adding labels failed | 4526 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `reattemptPlatformAutomerge(number, platformPrOptions)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should set automatic merge | 4630 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| handles unknown error | 4648 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `mergePr(prNo)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should merge the PR | 4670 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should handle merge error | 4702 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should handle merge block | 4723 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should handle approvers required | 4745 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should warn if automergeStrategy is not supported | 4767 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should use configured automergeStrategy | 4786 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `massageMarkdown(input)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns updated pr body | 4813 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns not-updated pr body for GHE | 4819 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `mergePr(prNo) - autodetection`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should try squash first | 4846 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should try merge after squash | 4865 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should try rebase after merge | 4886 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should give up | 4911 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `getVulnerabilityAlerts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| avoids fetching if repo has vulnerability alerts disabled | 4940 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns empty if error | 4950 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns array if found | 4963 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns empty if disabled | 5013 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| handles network error | 5027 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| calls logger.debug with only items that include securityVulnerability | 5041 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns normalized names for PIP ecosystem | 5097 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| handles pagination correctly | 5133 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null | 5232 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns file content | 5243 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns file content in json5 format | 5255 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns file content from given repo | 5272 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns file content from branch or tag | 5284 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws on malformed JSON | 5296 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws on errors | 5306 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `pushFiles`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if pre-commit phase has failed | 5332 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns null on REST error | 5352 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| commits and returns SHA string | 5367 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| performs rebase | 5396 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| continues if rebase fails due to 422 | 5425 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| aborts if rebase fails due to non-422 | 5456 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| aborts if commit SHA doesn't exist | 5485 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

| should throw if endpoint is invalid URL | 70 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should pass commit message as commitHeadline and commitBody for squash merge | 4144 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should pass commit message as commitHeadline and commitBody for merge commit | 4175 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should pass multi-line commit message body for squash merge | 4209 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should not pass commit message headline/body for rebase merge | 4242 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
---
