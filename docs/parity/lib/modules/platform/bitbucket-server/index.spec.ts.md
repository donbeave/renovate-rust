# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/bitbucket-server/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/bitbucket-server/index.spec.ts
**Total tests:** 141 | **Ported:** 0 | **Actionable:** 2 | **Status:** pending

### `endpoint with path › initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no endpoint | 240 | not-applicable | — | — | No corresponding Rust source|
| should throw if no username/password/token | 245 | not-applicable | — | — | No corresponding Rust source|
| should throw if password and token is set | 252 | not-applicable | — | — | No corresponding Rust source|
| should not throw if username/password | 264 | not-applicable | — | — | No corresponding Rust source|
| should not throw if token | 275 | not-applicable | — | — | No corresponding Rust source|
| should throw if version could not be fetched | 285 | not-applicable | — | — | No corresponding Rust source|
| should not throw if user info fetch fails | 307 | not-applicable | — | — | No corresponding Rust source|
| should skip users api call when gitAuthor is configured | 333 | not-applicable | — | — | No corresponding Rust source|
| should skip users api call when no username | 351 | not-applicable | — | — | No corresponding Rust source|
| should fetch user info if token with username | 367 | not-applicable | — | — | No corresponding Rust source|
| should collect username from headers if token with no username | 389 | not-applicable | — | — | No corresponding Rust source|
| should use fallback gitAuthor if user info has empty email address | 411 | not-applicable | — | — | No corresponding Rust source|
| should init | 442 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › getRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns repos | 463 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › initRepo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 482 | not-applicable | — | — | No corresponding Rust source|
| no git url | 501 | not-applicable | — | — | No corresponding Rust source|
| gitUrl ssh returns ssh url | 524 | not-applicable | — | — | No corresponding Rust source|
| gitURL endpoint returns generates endpoint URL | 553 | not-applicable | — | — | No corresponding Rust source|
| gitUrl default returns http from API with injected auth | 586 | not-applicable | — | — | No corresponding Rust source|
| uses ssh url from API if http not in API response | 620 | not-applicable | — | — | No corresponding Rust source|
| uses http url from API with injected auth if http url in API response | 644 | not-applicable | — | — | No corresponding Rust source|
| generates URL if API does not contain clone links | 673 | not-applicable | — | — | No corresponding Rust source|
| throws REPOSITORY_EMPTY if there is no default branch | 701 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › repoForceRebase()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false on missing mergeConfig | 720 | not-applicable | — | — | No corresponding Rust source|
| returns false on missing defaultStrategy | 734 | not-applicable | — | — | No corresponding Rust source|
| return true if %s strategy is enabled | 750 | not-applicable | — | — | No corresponding Rust source|
| return false if %s strategy is enabled | 771 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › addAssignees()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not throw | 794 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › addReviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not throw | 801 | not-applicable | — | — | No corresponding Rust source|
| sends the reviewer name as a reviewer | 817 | not-applicable | — | — | No corresponding Rust source|
| throws not-found 1 | 834 | not-applicable | — | — | No corresponding Rust source|
| throws not-found 2 | 841 | not-applicable | — | — | No corresponding Rust source|
| throws not-found 3 | 854 | not-applicable | — | — | No corresponding Rust source|
| does not throws repository-changed after 1 try | 871 | not-applicable | — | — | No corresponding Rust source|
| does not throws repository-changed after 2 tries | 890 | not-applicable | — | — | No corresponding Rust source|
| throws repository-changed after 3 tries | 910 | not-applicable | — | — | No corresponding Rust source|
| deals with invalid reviewers correctly | 928 | not-applicable | — | — | No corresponding Rust source|
| aborts instead of infinite recursion when invalid reviewers cannot be filtered | 984 | not-applicable | — | — | No corresponding Rust source|
| deals correctly with resolving reviewers | 1023 | not-applicable | — | — | No corresponding Rust source|
| throws | 1074 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › getUsernamesByEmail`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws when lookup fails | 1092 | not-applicable | — | — | No corresponding Rust source|
| return empty array when no results found | 1113 | not-applicable | — | — | No corresponding Rust source|
| return only active users | 1131 | not-applicable | — | — | No corresponding Rust source|
| only returns exact matches | 1156 | not-applicable | — | — | No corresponding Rust source|
| returns multiple exact matches | 1187 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › deleteLAbel()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not throw | 1223 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › ensureComment()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not throw | 1229 | not-applicable | — | — | No corresponding Rust source|
| add comment if not found 1 | 1244 | not-applicable | — | — | No corresponding Rust source|
| add comment if not found 2 | 1287 | not-applicable | — | — | No corresponding Rust source|
| add updates comment if necessary 1 | 1330 | not-applicable | — | — | No corresponding Rust source|
| add updates comment if necessary 2 | 1379 | not-applicable | — | — | No corresponding Rust source|
| skips comment 1 | 1422 | not-applicable | — | — | No corresponding Rust source|
| skips comment 2 | 1461 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › ensureCommentRemoval()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not throw | 1501 | not-applicable | — | — | No corresponding Rust source|
| deletes comment by topic if found | 1539 | not-applicable | — | — | No corresponding Rust source|
| deletes comment by content if found | 1588 | not-applicable | — | — | No corresponding Rust source|
| deletes nothing | 1637 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › getPrList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| has pr | 1678 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › getBranchPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| has pr | 1693 | not-applicable | — | — | No corresponding Rust source|
| has no pr | 1713 | not-applicable | — | — | No corresponding Rust source|
| has no existing pr | 1729 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › findPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| has pr | 1747 | not-applicable | — | — | No corresponding Rust source|
| has no pr | 1767 | not-applicable | — | — | No corresponding Rust source|
| finds pr from other authors | 1787 | not-applicable | — | — | No corresponding Rust source|
| returns null if no pr found - (includeOtherAuthors) | 1812 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › createPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| posts PR | 1833 | not-applicable | — | — | No corresponding Rust source|
| posts PR default branch | 1866 | not-applicable | — | — | No corresponding Rust source|
| should use platform automerge | 1900 | not-applicable | — | — | No corresponding Rust source|
| platform-native automerge returns early if usePlatformAutomerge is false | 1939 | not-applicable | — | — | No corresponding Rust source|
| platform-native automerge returns early if Bitbucket Server <= 8.15.0 is used | 1970 | not-applicable | — | — | No corresponding Rust source|
| platform-native automerge catches errors gracefully | 2004 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › reattemptPlatformAutomerge()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should reattempt automerge | 2049 | not-applicable | — | — | No corresponding Rust source|
| handles unknown error | 2071 | not-applicable | — | — | No corresponding Rust source|
| handles missing prNo | 2087 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › getPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for no prNo | 2102 | not-applicable | — | — | No corresponding Rust source|
| gets a PR | 2107 | not-applicable | — | — | No corresponding Rust source|
| canRebase | 2118 | not-applicable | — | — | No corresponding Rust source|
| gets a closed PR | 2138 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › updatePr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| puts PR | 2158 | not-applicable | — | — | No corresponding Rust source|
| closes PR | 2194 | not-applicable | — | — | No corresponding Rust source|
| re-opens PR | 2231 | not-applicable | — | — | No corresponding Rust source|
| throws not-found 1 | 2268 | not-applicable | — | — | No corresponding Rust source|
| throws not-found 2 | 2279 | not-applicable | — | — | No corresponding Rust source|
| throws not-found 3 | 2291 | not-applicable | — | — | No corresponding Rust source|
| handles invalid users gracefully by retrying without invalid reviewers | 2308 | not-applicable | — | — | No corresponding Rust source|
| throws repository-changed | 2364 | not-applicable | — | — | No corresponding Rust source|
| throws | 2381 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › mergePr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| posts Merge | 2400 | not-applicable | — | — | No corresponding Rust source|
| throws not-found 1 | 2420 | not-applicable | — | — | No corresponding Rust source|
| throws not-found 2 | 2429 | not-applicable | — | — | No corresponding Rust source|
| throws not-found 3 | 2445 | not-applicable | — | — | No corresponding Rust source|
| throws conflicted | 2465 | not-applicable | — | — | No corresponding Rust source|
| unknown error | 2485 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › massageMarkdown()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns diff files | 2507 | not-applicable | — | — | No corresponding Rust source|
| sanitizes HTML comments in the body | 2515 | not-applicable | — | — | No corresponding Rust source|
| resizes mend.io merge confidence badges | 2530 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › getBranchStatus()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should be success | 2539 | not-applicable | — | — | No corresponding Rust source|
| should be pending | 2554 | not-applicable | — | — | No corresponding Rust source|
| should be failed | 2581 | not-applicable | — | — | No corresponding Rust source|
| throws repository-changed | 2604 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › getBranchStatusCheck()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should be success | 2614 | not-applicable | — | — | No corresponding Rust source|
| should be pending | 2636 | not-applicable | — | — | No corresponding Rust source|
| should be failure | 2658 | not-applicable | — | — | No corresponding Rust source|
| should be null | 2680 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › setBranchStatus()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should be success 1 | 2708 | not-applicable | — | — | No corresponding Rust source|
| should be success 2 | 2738 | not-applicable | — | — | No corresponding Rust source|
| should be success 3 | 2768 | not-applicable | — | — | No corresponding Rust source|
| should be success 4 | 2798 | not-applicable | — | — | No corresponding Rust source|
| should be success 5 | 2828 | not-applicable | — | — | No corresponding Rust source|
| should be success 6 | 2853 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns file content | 2876 | not-applicable | — | — | No corresponding Rust source|
| returns file content in json5 format | 2891 | not-applicable | — | — | No corresponding Rust source|
| returns file content from given repo | 2911 | not-applicable | — | — | No corresponding Rust source|
| returns file content from branch or tag | 2926 | not-applicable | — | — | No corresponding Rust source|
| throws on malformed JSON | 2945 | not-applicable | — | — | No corresponding Rust source|
| throws on long content | 2958 | not-applicable | — | — | No corresponding Rust source|
| throws on errors | 2971 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › modules/platform/bitbucket-server/code-owners`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ignores comments and empty lines | 2982 | not-applicable | — | — | No corresponding Rust source|
| parses usernames with escaped spaces | 2992 | not-applicable | — | — | No corresponding Rust source|
| parses groups with escaped spaces | 3000 | not-applicable | — | — | No corresponding Rust source|
| supports reviewer groups with modifiers) | 3013 | not-applicable | — | — | No corresponding Rust source|
| matches paths correctly using glob patterns | 3027 | not-applicable | — | — | No corresponding Rust source|
| respects bottom-to-top rule precedence | 3044 | not-applicable | — | — | No corresponding Rust source|
| supports rules with no owners (ownership ignored) | 3054 | not-applicable | — | — | No corresponding Rust source|
| unescapes multiple escaped spaces correctly | 3064 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with path › expandGroupMembers()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns input when it is not a group | 3073 | not-applicable | — | — | No corresponding Rust source|
| returns only active users from the matching reviewer group | 3080 | not-applicable | — | — | No corresponding Rust source|
| returns empty array if group is not found | 3124 | not-applicable | — | — | No corresponding Rust source|
| returns empty array if API call fails | 3153 | not-applicable | — | — | No corresponding Rust source|
| returns empty array if all users in group are inactive | 3167 | not-applicable | — | — | No corresponding Rust source|
| prefers repository-level reviewer group over project-level group with same name | 3200 | not-applicable | — | — | No corresponding Rust source|
| uses project-level group when repository-level group is not available | 3247 | not-applicable | — | — | No corresponding Rust source|
| deals with not found groups correctly | 3280 | not-applicable | — | — | No corresponding Rust source|
| handles random without number correctly | 3306 | not-applicable | — | — | No corresponding Rust source|
| handles random with number correctly | 3353 | not-applicable | — | — | No corresponding Rust source|
| handles non-existent modifier correctly | 3402 | not-applicable | — | — | No corresponding Rust source|
| handles paginated responses and finds matching group in next page | 3451 | not-applicable | — | — | No corresponding Rust source|

### `endpoint with no path › initRepo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gitURL endpoint generates URL without endpoint path | 3559 | not-applicable | — | — | No corresponding Rust source|
| generates URL without endpoint path if API does not contain clone links | 3584 | not-applicable | — | — | No corresponding Rust source|

---
