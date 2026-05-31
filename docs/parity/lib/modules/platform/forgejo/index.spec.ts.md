# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/forgejo/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/forgejo/index.spec.ts
**Total tests:** 137 | **Ported:** 0 | **Actionable:** 137 | **Status:** pending

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no token | 278 | pending | — | — | No corresponding Rust source|
| should throw if auth fails | 282 | pending | — | — | No corresponding Rust source|
| should support default endpoint | 291 | pending | — | — | No corresponding Rust source|
| should support custom endpoint | 305 | pending | — | — | No corresponding Rust source|
| should support custom endpoint including api path | 324 | pending | — | — | No corresponding Rust source|
| should use username as author name if full name is missing | 343 | pending | — | — | No corresponding Rust source|

### `getRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should propagate any other errors from getRepos | 362 | pending | — | — | No corresponding Rust source|
| should return an array of repos | 376 | pending | — | — | No corresponding Rust source|
| should return an filtered array of repos | 394 | pending | — | — | No corresponding Rust source|
| should query the organization endpoint for each namespace | 431 | pending | — | — | No corresponding Rust source|
| Sorts repos | 445 | pending | — | — | No corresponding Rust source|

### `initRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should propagate API errors | 474 | pending | — | — | No corresponding Rust source|
| should propagate org API errors | 483 | pending | — | — | No corresponding Rust source|
| should abort when repo is archived | 496 | pending | — | — | No corresponding Rust source|
| should abort when repo is mirrored | 510 | pending | — | — | No corresponding Rust source|
| should abort when repo is empty | 524 | pending | — | — | No corresponding Rust source|
| should abort when repo has insufficient permissions | 538 | pending | — | — | No corresponding Rust source|
| should abort when repo has pulls disabled | 556 | pending | — | — | No corresponding Rust source|
| should abort when repo has no available merge methods | 570 | pending | — | — | No corresponding Rust source|
| should select default merge method when it is allowed | 584 | pending | — | — | No corresponding Rust source|
| should fall back to merge method as per ordered list when default not allowed | 607 | pending | — | — | No corresponding Rust source|
| should throw if unknown default merge style is configured | 630 | pending | — | — | No corresponding Rust source|
| should use clone_url of repo if gitUrl is not specified | 645 | pending | — | — | No corresponding Rust source|
| should use clone_url of repo if gitUrl has value default | 664 | pending | — | — | No corresponding Rust source|
| should use ssh_url of repo if gitUrl has value ssh | 684 | pending | — | — | No corresponding Rust source|
| should abort when gitUrl has value ssh but ssh_url is empty | 704 | pending | — | — | No corresponding Rust source|
| should use generated url of repo if gitUrl has value endpoint | 723 | pending | — | — | No corresponding Rust source|
| should abort when clone_url is empty | 745 | pending | — | — | No corresponding Rust source|
| should use given access token if gitUrl has value endpoint | 766 | pending | — | — | No corresponding Rust source|
| should use given access token if gitUrl is not specified | 797 | pending | — | — | No corresponding Rust source|
| should abort when clone_url is not valid | 825 | pending | — | — | No corresponding Rust source|

### `setBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create a new commit status | 848 | pending | — | — | No corresponding Rust source|
| should default to pending state | 876 | pending | — | — | No corresponding Rust source|
| should include url if specified | 904 | pending | — | — | No corresponding Rust source|
| should gracefully fail with warning when setting branch status | 934 | pending | — | — | No corresponding Rust source|

### `getBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return yellow for unknown result | 973 | pending | — | — | No corresponding Rust source|
| should return pending state for pending result | 986 | pending | — | — | No corresponding Rust source|
| should return green state for success result | 999 | pending | — | — | No corresponding Rust source|
| should return yellow for all other results | 1012 | pending | — | — | No corresponding Rust source|
| should abort when branch status returns 404 | 1025 | pending | — | — | No corresponding Rust source|
| should propagate any other errors from getBranchStatus | 1038 | pending | — | — | No corresponding Rust source|
| should treat internal checks as success | 1051 | pending | — | — | No corresponding Rust source|
| should not treat internal checks as success | 1073 | pending | — | — | No corresponding Rust source|

### `getBranchStatusCheck`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null with no results | 1097 | pending | — | — | No corresponding Rust source|
| should return null with no matching results | 1110 | pending | — | — | No corresponding Rust source|
| should return yellow with unknown status | 1135 | pending | — | — | No corresponding Rust source|
| should return green of matching result | 1160 | pending | — | — | No corresponding Rust source|

### `getPrList`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return list of pull requests | 1187 | pending | — | — | No corresponding Rust source|
| should filter list by creator | 1205 | pending | — | — | No corresponding Rust source|
| should cache results after first query | 1248 | pending | — | — | No corresponding Rust source|
| should update cache results | 1274 | pending | — | — | No corresponding Rust source|

### `getPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return enriched pull request which exists if open | 1302 | pending | — | — | No corresponding Rust source|
| should fallback to direct fetching if cache fails | 1316 | pending | — | — | No corresponding Rust source|
| should return null for missing pull request in getPr | 1333 | pending | — | — | No corresponding Rust source|
| should throw temporary error for null pull request | 1349 | pending | — | — | No corresponding Rust source|

### `findPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should find pull request without title or state | 1363 | pending | — | — | No corresponding Rust source|
| should find pull request with title | 1380 | pending | — | — | No corresponding Rust source|
| should find pull request with state | 1400 | pending | — | — | No corresponding Rust source|
| should not find pull request with inverted state | 1420 | pending | — | — | No corresponding Rust source|
| should find pull request with title and state | 1441 | pending | — | — | No corresponding Rust source|
| should find pull request with draft | 1463 | pending | — | — | No corresponding Rust source|
| should find merged pull request | 1485 | pending | — | — | No corresponding Rust source|
| should return null for missing pull request in findPr | 1503 | pending | — | — | No corresponding Rust source|
| finds pr from other authors using base and head | 1517 | pending | — | — | No corresponding Rust source|
| returns null if cannot find pr from other author | 1555 | pending | — | — | No corresponding Rust source|

### `createPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should use base branch by default | 1599 | pending | — | — | No corresponding Rust source|
| should use default branch if requested | 1623 | pending | — | — | No corresponding Rust source|
| should resolve and apply optional repo and org labels to pull request | 1645 | pending | — | — | No corresponding Rust source|
| should resolve and apply optional repo labels to pull request | 1671 | pending | — | — | No corresponding Rust source|
| should ensure new pull request gets added to cached pull requests | 1695 | pending | — | — | No corresponding Rust source|
| should attempt to resolve 409 conflict error (w/o update) | 1718 | pending | — | — | No corresponding Rust source|
| should attempt to resolve 409 conflict error (w/ update) | 1742 | pending | — | — | No corresponding Rust source|
| should abort when response for created pull request is invalid | 1768 | pending | — | — | No corresponding Rust source|
| should use platform automerge | 1786 | pending | — | — | No corresponding Rust source|
| should not use platform automerge on forgejo v7 | 1812 | pending | — | — | No corresponding Rust source|
| should not use platform automerge on forgejo v7 LTS | 1836 | pending | — | — | No corresponding Rust source|
| continues on platform automerge error | 1860 | pending | — | — | No corresponding Rust source|
| continues if platform automerge is not supported | 1889 | pending | — | — | No corresponding Rust source|
| should create PR with repository merge method when automergeStrategy is auto | 1917 | pending | — | — | No corresponding Rust source|
| should create PR with mergeStrategy $prMergeStrategy | 1944 | pending | — | — | No corresponding Rust source|

### `updatePr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update pull request with title | 1985 | pending | — | — | No corresponding Rust source|
| should update pull target branch | 2002 | pending | — | — | No corresponding Rust source|
| should update pull request with title and body | 2025 | pending | — | — | No corresponding Rust source|
| should update pull request with draft | 2048 | pending | — | — | No corresponding Rust source|
| should close pull request | 2071 | pending | — | — | No corresponding Rust source|
| should update labels | 2096 | pending | — | — | No corresponding Rust source|
| should log a warning if labels could not be looked up | 2135 | pending | — | — | No corresponding Rust source|

### `mergePr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true when merging succeeds | 2180 | pending | — | — | No corresponding Rust source|
| should return false when merging fails | 2198 | pending | — | — | No corresponding Rust source|

### `getIssueList`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty for disabled issues | 2219 | pending | — | — | No corresponding Rust source|

### `getIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return the issue | 2231 | pending | — | — | No corresponding Rust source|
| should return null for disabled issues in getIssue | 2248 | pending | — | — | No corresponding Rust source|
| should return null on error | 2258 | pending | — | — | No corresponding Rust source|

### `findIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return existing open issue | 2273 | pending | — | — | No corresponding Rust source|
| should not return existing closed issue | 2293 | pending | — | — | No corresponding Rust source|
| should return null for missing issue | 2310 | pending | — | — | No corresponding Rust source|

### `ensureIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create issue if not found | 2326 | pending | — | — | No corresponding Rust source|
| should create issue with the correct labels | 2352 | pending | — | — | No corresponding Rust source|
| should not reopen closed issue by default | 2387 | pending | — | — | No corresponding Rust source|
| should not update labels when not necessary | 2412 | pending | — | — | No corresponding Rust source|
| should update labels when missing | 2449 | pending | — | — | No corresponding Rust source|
| should reset labels when others have been set | 2488 | pending | — | — | No corresponding Rust source|
| should reopen closed issue if desired | 2528 | pending | — | — | No corresponding Rust source|
| should not update existing closed issue if desired | 2554 | pending | — | — | No corresponding Rust source|
| should close all open duplicate issues except first one when updating | 2574 | pending | — | — | No corresponding Rust source|
| should reset issue cache when creating an issue | 2605 | pending | — | — | No corresponding Rust source|
| should gracefully fail with warning when ensuring issue | 2629 | pending | — | — | No corresponding Rust source|
| should return null for disabled issues in ensureIssue | 2651 | pending | — | — | No corresponding Rust source|

### `ensureIssueClosing`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should close issues with matching title | 2668 | pending | — | — | No corresponding Rust source|
| should return for disabled issues | 2683 | pending | — | — | No corresponding Rust source|

### `deleteLabel`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should delete a label which exists | 2692 | pending | — | — | No corresponding Rust source|
| should gracefully fail with warning if label is missing | 2708 | pending | — | — | No corresponding Rust source|

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add comment with topic if not found | 2728 | pending | — | — | No corresponding Rust source|
| should add comment without topic if not found | 2749 | pending | — | — | No corresponding Rust source|
| should update comment with topic if found | 2768 | pending | — | — | No corresponding Rust source|
| should skip if comment is up-to-date | 2789 | pending | — | — | No corresponding Rust source|
| should gracefully fail with warning when ensuring comment | 2806 | pending | — | — | No corresponding Rust source|

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should remove existing comment by topic | 2830 | pending | — | — | No corresponding Rust source|
| should remove existing comment by content | 2849 | pending | — | — | No corresponding Rust source|
| should gracefully fail with warning when removing comment | 2868 | pending | — | — | No corresponding Rust source|
| should abort silently if comment is missing | 2894 | pending | — | — | No corresponding Rust source|

### `getBranchPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return existing pull request for branch | 2913 | pending | — | — | No corresponding Rust source|
| should return null if no pull request exists | 2927 | pending | — | — | No corresponding Rust source|

### `addAssignees`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add assignees to the issue | 2941 | pending | — | — | No corresponding Rust source|

### `addReviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should assign user and team reviewers | 2956 | pending | — | — | No corresponding Rust source|
| should assign user reviewers | 2974 | pending | — | — | No corresponding Rust source|
| catches errors | 2989 | pending | — | — | No corresponding Rust source|

### `massageMarkdown`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| replaces pr links | 3008 | pending | — | — | No corresponding Rust source|
| replaces issue links | 3017 | pending | — | — | No corresponding Rust source|
| maxBodyLength | 3027 | pending | — | — | No corresponding Rust source|

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns file content | 3032 | pending | — | — | No corresponding Rust source|
| returns file content from given repo | 3048 | pending | — | — | No corresponding Rust source|
| returns file content from branch or tag | 3064 | pending | — | — | No corresponding Rust source|
| returns file content in json5 format | 3080 | pending | — | — | No corresponding Rust source|
| throws on malformed JSON | 3101 | pending | — | — | No corresponding Rust source|
| returns null on missing content | 3113 | pending | — | — | No corresponding Rust source|
| throws on errors | 3123 | pending | — | — | No corresponding Rust source|

---
