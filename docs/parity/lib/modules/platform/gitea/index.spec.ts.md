# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gitea/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gitea/index.spec.ts
**Total tests:** 137 | **Ported:** 0 | **Actionable:** 137 | **Status:** pending

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no token | 270 | pending | — | — | —|
| should throw if auth fails | 274 | pending | — | — | —|
| should support default endpoint | 283 | pending | — | — | —|
| should support custom endpoint | 297 | pending | — | — | —|
| should support custom endpoint including api path | 316 | pending | — | — | —|
| should use username as author name if full name is missing | 335 | pending | — | — | —|

### `getRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should propagate any other errors | 354 | pending | — | — | —|
| should return an array of repos | 368 | pending | — | — | —|
| should return an filtered array of repos | 386 | pending | — | — | —|
| should query the organization endpoint for each namespace | 423 | pending | — | — | —|
| Sorts repos | 437 | pending | — | — | —|

### `initRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should propagate API errors | 466 | pending | — | — | —|
| should abort when repo is archived | 475 | pending | — | — | —|
| should abort when repo is mirrored | 489 | pending | — | — | —|
| should abort when repo is empty | 503 | pending | — | — | —|
| should abort when repo has insufficient permissions | 517 | pending | — | — | —|
| should abort when repo has pulls disabled | 535 | pending | — | — | —|
| should abort when repo has no available merge methods | 549 | pending | — | — | —|
| should select default merge method when it is allowed | 563 | pending | — | — | —|
| should fall back to merge method as per ordered list when default not allowed | 584 | pending | — | — | —|
| should throw if unknown default merge style is configured | 606 | pending | — | — | —|
| should use clone_url of repo if gitUrl is not specified | 621 | pending | — | — | —|
| should use clone_url of repo if gitUrl has value default | 638 | pending | — | — | —|
| should use ssh_url of repo if gitUrl has value ssh | 656 | pending | — | — | —|
| should abort when gitUrl has value ssh but ssh_url is empty | 674 | pending | — | — | —|
| should use generated url of repo if gitUrl has value endpoint | 691 | pending | — | — | —|
| should abort when clone_url is empty | 711 | pending | — | — | —|
| should use given access token if gitUrl has value endpoint | 730 | pending | — | — | —|
| should use given access token if gitUrl is not specified | 759 | pending | — | — | —|
| should abort when clone_url is not valid | 785 | pending | — | — | —|

### `setBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create a new commit status | 806 | pending | — | — | —|
| should default to pending state | 834 | pending | — | — | —|
| should include url if specified | 862 | pending | — | — | —|
| should gracefully fail with warning | 892 | pending | — | — | —|

### `getBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return yellow for unknown result | 931 | pending | — | — | —|
| should return pending state for pending result | 944 | pending | — | — | —|
| should return green state for success result | 957 | pending | — | — | —|
| should return yellow for all other results | 970 | pending | — | — | —|
| should abort when branch status returns 404 | 983 | pending | — | — | —|
| should propagate any other errors | 996 | pending | — | — | —|
| should treat internal checks as success | 1009 | pending | — | — | —|
| should not treat internal checks as success | 1031 | pending | — | — | —|

### `getBranchStatusCheck`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null with no results | 1055 | pending | — | — | —|
| should return null with no matching results | 1068 | pending | — | — | —|
| should return yellow with unknown status | 1093 | pending | — | — | —|
| should return green of matching result | 1118 | pending | — | — | —|

### `getPrList`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return list of pull requests | 1145 | pending | — | — | —|
| should filter list by creator | 1163 | pending | — | — | —|
| should cache results after first query | 1206 | pending | — | — | —|
| should update cache results | 1232 | pending | — | — | —|

### `getPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return enriched pull request which exists if open | 1260 | pending | — | — | —|
| should fallback to direct fetching if cache fails | 1274 | pending | — | — | —|
| should return null for missing pull request | 1291 | pending | — | — | —|
| should throw temporary error for null pull request | 1307 | pending | — | — | —|

### `findPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should find pull request without title or state | 1321 | pending | — | — | —|
| should find pull request with title | 1338 | pending | — | — | —|
| should find pull request with state | 1358 | pending | — | — | —|
| should not find pull request with inverted state | 1378 | pending | — | — | —|
| should find pull request with title and state | 1399 | pending | — | — | —|
| should find pull request with draft | 1421 | pending | — | — | —|
| should find merged pull request | 1443 | pending | — | — | —|
| should return null for missing pull request | 1461 | pending | — | — | —|
| finds pr from other authors using base and head | 1475 | pending | — | — | —|
| returns null if cannot find pr from other author | 1513 | pending | — | — | —|

### `createPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should use base branch by default | 1557 | pending | — | — | —|
| should use default branch if requested | 1581 | pending | — | — | —|
| should resolve and apply optional labels to pull request | 1603 | pending | — | — | —|
| should ensure new pull request gets added to cached pull requests | 1629 | pending | — | — | —|
| should attempt to resolve 409 conflict error (w/o update) | 1652 | pending | — | — | —|
| should attempt to resolve 409 conflict error (w/ update) | 1676 | pending | — | — | —|
| should abort when response for created pull request is invalid | 1702 | pending | — | — | —|
| should use platform automerge | 1720 | pending | — | — | —|
| should not use platform automerge on forgejo v7 | 1746 | pending | — | — | —|
| should not use platform automerge on forgejo v7 LTS | 1770 | pending | — | — | —|
| continues on platform automerge error | 1794 | pending | — | — | —|
| continues if platform automerge is not supported | 1823 | pending | — | — | —|
| should create PR with repository merge method when automergeStrategy is auto | 1851 | pending | — | — | —|
| should create PR with mergeStrategy $prMergeStrategy | 1878 | pending | — | — | —|

### `updatePr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update pull request with title | 1919 | pending | — | — | —|
| should update pull target branch | 1936 | pending | — | — | —|
| should update pull request with title and body | 1959 | pending | — | — | —|
| should update pull request with draft | 1982 | pending | — | — | —|
| should close pull request | 2005 | pending | — | — | —|
| should update labels | 2030 | pending | — | — | —|
| should log a warning if labels could not be looked up | 2069 | pending | — | — | —|

### `mergePr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true when merging succeeds | 2114 | pending | — | — | —|
| should return false when merging fails | 2132 | pending | — | — | —|

### `getIssueList`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty for disabled issues | 2153 | pending | — | — | —|

### `getIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return the issue | 2165 | pending | — | — | —|
| should return null for disabled issues | 2182 | pending | — | — | —|

### `findIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return existing open issue | 2194 | pending | — | — | —|
| should not return existing closed issue | 2214 | pending | — | — | —|
| should return null for missing issue | 2231 | pending | — | — | —|

### `ensureIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create issue if not found | 2247 | pending | — | — | —|
| should create issue with the correct labels | 2273 | pending | — | — | —|
| should not reopen closed issue by default | 2308 | pending | — | — | —|
| should not update labels when not necessary | 2333 | pending | — | — | —|
| should update labels when missing | 2370 | pending | — | — | —|
| should reset labels when others have been set | 2409 | pending | — | — | —|
| should reopen closed issue if desired | 2449 | pending | — | — | —|
| should not update existing closed issue if desired | 2475 | pending | — | — | —|
| should close all open duplicate issues except first one when updating | 2495 | pending | — | — | —|
| should reset issue cache when creating an issue | 2526 | pending | — | — | —|
| should gracefully fail with warning | 2550 | pending | — | — | —|
| should return null for disabled issues | 2572 | pending | — | — | —|

### `ensureIssueClosing`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should close issues with matching title | 2589 | pending | — | — | —|
| should return for disabled issues | 2604 | pending | — | — | —|

### `deleteLabel`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should delete a label which exists | 2613 | pending | — | — | —|
| should gracefully fail with warning if label is missing | 2629 | pending | — | — | —|

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add comment with topic if not found | 2649 | pending | — | — | —|
| should add comment without topic if not found | 2670 | pending | — | — | —|
| should update comment with topic if found | 2689 | pending | — | — | —|
| should skip if comment is up-to-date | 2710 | pending | — | — | —|
| should gracefully fail with warning | 2727 | pending | — | — | —|

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should remove existing comment by topic | 2751 | pending | — | — | —|
| should remove existing comment by content | 2770 | pending | — | — | —|
| should gracefully fail with warning | 2789 | pending | — | — | —|
| should abort silently if comment is missing | 2815 | pending | — | — | —|

### `getBranchPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return existing pull request for branch | 2834 | pending | — | — | —|
| should return null if no pull request exists | 2848 | pending | — | — | —|

### `addAssignees`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add assignees to the issue | 2862 | pending | — | — | —|

### `addReviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should assign reviewers | 2877 | pending | — | — | —|
| should do nothing for older Gitea versions | 2892 | pending | — | — | —|
| catches errors | 2900 | pending | — | — | —|

### `massageMarkdown`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| replaces pr links | 2920 | pending | — | — | —|
| replaces issue links | 2929 | pending | — | — | —|
| maxBodyLength | 2939 | pending | — | — | —|

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns file content | 2944 | pending | — | — | —|
| returns file content from given repo | 2960 | pending | — | — | —|
| returns file content from branch or tag | 2976 | pending | — | — | —|
| returns file content in json5 format | 2992 | pending | — | — | —|
| throws on malformed JSON | 3013 | pending | — | — | —|
| returns null on missing content | 3025 | pending | — | — | —|
| throws on errors | 3035 | pending | — | — | —|

---
