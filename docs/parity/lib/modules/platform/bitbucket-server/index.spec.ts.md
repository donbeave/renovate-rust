# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/bitbucket-server/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/bitbucket-server/index.spec.ts
**Total tests:** 139 | **Ported:** 0 | **Actionable:** 139 | **Status:** not-applicable

### `endpoint with path › initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no endpoint | 240 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw if no username/password/token | 245 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw if password and token is set | 252 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should not throw if username/password | 264 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should not throw if token | 275 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should throw if version could not be fetched | 285 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should not throw if user info fetch fails | 307 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should skip users api call when gitAuthor is configured | 333 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should skip users api call when no username | 351 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should fetch user info if token with username | 367 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should collect username from headers if token with no username | 389 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should use fallback gitAuthor if user info has empty email address | 411 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should init | 442 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › getRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns repos | 463 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › initRepo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 482 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| no git url | 501 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| gitUrl ssh returns ssh url | 524 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| gitURL endpoint returns generates endpoint URL | 553 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| gitUrl default returns http from API with injected auth | 586 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| uses ssh url from API if http not in API response | 620 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| uses http url from API with injected auth if http url in API response | 644 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| generates URL if API does not contain clone links | 673 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws REPOSITORY_EMPTY if there is no default branch | 701 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › repoForceRebase()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false on missing mergeConfig | 720 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns false on missing defaultStrategy | 734 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| return true if %s strategy is enabled | 750 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| return false if %s strategy is enabled | 771 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › addAssignees()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not throw | 794 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › addReviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not throw | 801 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| sends the reviewer name as a reviewer | 817 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws not-found 1 | 834 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws not-found 2 | 841 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws not-found 3 | 854 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| does not throws repository-changed after 1 try | 871 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| does not throws repository-changed after 2 tries | 890 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws repository-changed after 3 tries | 910 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| deals with invalid reviewers correctly | 928 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| aborts instead of infinite recursion when invalid reviewers cannot be filtered | 984 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| deals correctly with resolving reviewers | 1023 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws | 1074 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › getUsernamesByEmail`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws when lookup fails | 1092 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| return empty array when no results found | 1113 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| return only active users | 1131 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| only returns exact matches | 1156 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns multiple exact matches | 1187 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › deleteLAbel()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not throw | 1223 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › ensureComment()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not throw | 1229 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| add comment if not found 1 | 1244 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| add comment if not found 2 | 1287 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| add updates comment if necessary 1 | 1330 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| add updates comment if necessary 2 | 1379 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| skips comment 1 | 1422 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| skips comment 2 | 1461 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › ensureCommentRemoval()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not throw | 1501 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| deletes comment by topic if found | 1539 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| deletes comment by content if found | 1588 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| deletes nothing | 1637 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › getPrList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| has pr | 1678 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › getBranchPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| has pr | 1693 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| has no pr | 1713 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| has no existing pr | 1729 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › findPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| has pr | 1747 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| has no pr | 1767 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| finds pr from other authors | 1787 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns null if no pr found - (includeOtherAuthors) | 1812 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › createPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| posts PR | 1833 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| posts PR default branch | 1866 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should use platform automerge | 1900 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| platform-native automerge returns early if usePlatformAutomerge is false | 1939 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| platform-native automerge returns early if Bitbucket Server <= 8.15.0 is used | 1970 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| platform-native automerge catches errors gracefully | 2004 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › reattemptPlatformAutomerge()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should reattempt automerge | 2049 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| handles unknown error | 2071 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| handles missing prNo | 2087 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › getPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for no prNo | 2102 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| gets a PR | 2107 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| canRebase | 2118 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| gets a closed PR | 2138 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › updatePr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| puts PR | 2158 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| closes PR | 2194 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| re-opens PR | 2231 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws not-found 1 | 2268 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws not-found 2 | 2279 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws not-found 3 | 2291 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| handles invalid users gracefully by retrying without invalid reviewers | 2308 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws repository-changed | 2364 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws | 2381 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › mergePr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| posts Merge | 2400 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws not-found 1 | 2420 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws not-found 2 | 2429 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws not-found 3 | 2445 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws conflicted | 2465 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| unknown error | 2485 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › massageMarkdown()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns diff files | 2507 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| sanitizes HTML comments in the body | 2515 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| resizes mend.io merge confidence badges | 2530 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › getBranchStatus()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should be success | 2539 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should be pending | 2554 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should be failed | 2581 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws repository-changed | 2604 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › getBranchStatusCheck()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should be success | 2614 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should be pending | 2636 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should be failure | 2658 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should be null | 2680 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › setBranchStatus()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should be success 1 | 2708 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should be success 2 | 2738 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should be success 3 | 2768 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should be success 4 | 2798 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should be success 5 | 2828 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| should be success 6 | 2853 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns file content | 2876 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns file content in json5 format | 2891 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns file content from given repo | 2911 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns file content from branch or tag | 2926 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws on malformed JSON | 2945 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws on long content | 2958 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| throws on errors | 2971 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › modules/platform/bitbucket-server/code-owners`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ignores comments and empty lines | 2982 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| parses usernames with escaped spaces | 2992 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| parses groups with escaped spaces | 3000 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| supports reviewer groups with modifiers) | 3013 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| matches paths correctly using glob patterns | 3027 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| respects bottom-to-top rule precedence | 3044 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| supports rules with no owners (ownership ignored) | 3054 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| unescapes multiple escaped spaces correctly | 3064 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with path › expandGroupMembers()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns input when it is not a group | 3073 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns only active users from the matching reviewer group | 3080 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns empty array if group is not found | 3124 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns empty array if API call fails | 3153 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| returns empty array if all users in group are inactive | 3167 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| prefers repository-level reviewer group over project-level group with same name | 3200 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| uses project-level group when repository-level group is not available | 3247 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| deals with not found groups correctly | 3280 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| handles random without number correctly | 3306 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| handles random with number correctly | 3353 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| handles non-existent modifier correctly | 3402 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| handles paginated responses and finds matching group in next page | 3451 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

### `endpoint with no path › initRepo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gitURL endpoint generates URL without endpoint path | 3559 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |
| generates URL without endpoint path if API does not contain clone links | 3584 | not-applicable | — | — | Uses HTTP mocking / platform API mocking / fs mocking / git mocking; infrastructure tests not portable to Rust |

---
