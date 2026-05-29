# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/forgejo/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/forgejo/index.spec.ts
**Total tests:** 137 | **Ported:** 0 | **Actionable:** 137 | **Status:** pending

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no token | 278 | pending | — | — | —|
| should throw if auth fails | 282 | pending | — | — | —|
| should support default endpoint | 291 | pending | — | — | —|
| should support custom endpoint | 305 | pending | — | — | —|
| should support custom endpoint including api path | 324 | pending | — | — | —|
| should use username as author name if full name is missing | 343 | pending | — | — | —|

### `getRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should propagate any other errors from getRepos | 362 | pending | — | — | —|
| should return an array of repos | 376 | pending | — | — | —|
| should return an filtered array of repos | 394 | pending | — | — | —|
| should query the organization endpoint for each namespace | 431 | pending | — | — | —|
| Sorts repos | 445 | pending | — | — | —|

### `initRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should propagate API errors | 474 | pending | — | — | —|
| should propagate org API errors | 483 | pending | — | — | —|
| should abort when repo is archived | 496 | pending | — | — | —|
| should abort when repo is mirrored | 510 | pending | — | — | —|
| should abort when repo is empty | 524 | pending | — | — | —|
| should abort when repo has insufficient permissions | 538 | pending | — | — | —|
| should abort when repo has pulls disabled | 556 | pending | — | — | —|
| should abort when repo has no available merge methods | 570 | pending | — | — | —|
| should select default merge method when it is allowed | 584 | pending | — | — | —|
| should fall back to merge method as per ordered list when default not allowed | 607 | pending | — | — | —|
| should throw if unknown default merge style is configured | 630 | pending | — | — | —|
| should use clone_url of repo if gitUrl is not specified | 645 | pending | — | — | —|
| should use clone_url of repo if gitUrl has value default | 664 | pending | — | — | —|
| should use ssh_url of repo if gitUrl has value ssh | 684 | pending | — | — | —|
| should abort when gitUrl has value ssh but ssh_url is empty | 704 | pending | — | — | —|
| should use generated url of repo if gitUrl has value endpoint | 723 | pending | — | — | —|
| should abort when clone_url is empty | 745 | pending | — | — | —|
| should use given access token if gitUrl has value endpoint | 766 | pending | — | — | —|
| should use given access token if gitUrl is not specified | 797 | pending | — | — | —|
| should abort when clone_url is not valid | 825 | pending | — | — | —|

### `setBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create a new commit status | 848 | pending | — | — | —|
| should default to pending state | 876 | pending | — | — | —|
| should include url if specified | 904 | pending | — | — | —|
| should gracefully fail with warning when setting branch status | 934 | pending | — | — | —|

### `getBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return yellow for unknown result | 973 | pending | — | — | —|
| should return pending state for pending result | 986 | pending | — | — | —|
| should return green state for success result | 999 | pending | — | — | —|
| should return yellow for all other results | 1012 | pending | — | — | —|
| should abort when branch status returns 404 | 1025 | pending | — | — | —|
| should propagate any other errors from getBranchStatus | 1038 | pending | — | — | —|
| should treat internal checks as success | 1051 | pending | — | — | —|
| should not treat internal checks as success | 1073 | pending | — | — | —|

### `getBranchStatusCheck`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null with no results | 1097 | pending | — | — | —|
| should return null with no matching results | 1110 | pending | — | — | —|
| should return yellow with unknown status | 1135 | pending | — | — | —|
| should return green of matching result | 1160 | pending | — | — | —|

### `getPrList`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return list of pull requests | 1187 | pending | — | — | —|
| should filter list by creator | 1205 | pending | — | — | —|
| should cache results after first query | 1248 | pending | — | — | —|
| should update cache results | 1274 | pending | — | — | —|

### `getPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return enriched pull request which exists if open | 1302 | pending | — | — | —|
| should fallback to direct fetching if cache fails | 1316 | pending | — | — | —|
| should return null for missing pull request in getPr | 1333 | pending | — | — | —|
| should throw temporary error for null pull request | 1349 | pending | — | — | —|

### `findPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should find pull request without title or state | 1363 | pending | — | — | —|
| should find pull request with title | 1380 | pending | — | — | —|
| should find pull request with state | 1400 | pending | — | — | —|
| should not find pull request with inverted state | 1420 | pending | — | — | —|
| should find pull request with title and state | 1441 | pending | — | — | —|
| should find pull request with draft | 1463 | pending | — | — | —|
| should find merged pull request | 1485 | pending | — | — | —|
| should return null for missing pull request in findPr | 1503 | pending | — | — | —|
| finds pr from other authors using base and head | 1517 | pending | — | — | —|
| returns null if cannot find pr from other author | 1555 | pending | — | — | —|

### `createPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should use base branch by default | 1599 | pending | — | — | —|
| should use default branch if requested | 1623 | pending | — | — | —|
| should resolve and apply optional repo and org labels to pull request | 1645 | pending | — | — | —|
| should resolve and apply optional repo labels to pull request | 1671 | pending | — | — | —|
| should ensure new pull request gets added to cached pull requests | 1695 | pending | — | — | —|
| should attempt to resolve 409 conflict error (w/o update) | 1718 | pending | — | — | —|
| should attempt to resolve 409 conflict error (w/ update) | 1742 | pending | — | — | —|
| should abort when response for created pull request is invalid | 1768 | pending | — | — | —|
| should use platform automerge | 1786 | pending | — | — | —|
| should not use platform automerge on forgejo v7 | 1812 | pending | — | — | —|
| should not use platform automerge on forgejo v7 LTS | 1836 | pending | — | — | —|
| continues on platform automerge error | 1860 | pending | — | — | —|
| continues if platform automerge is not supported | 1889 | pending | — | — | —|
| should create PR with repository merge method when automergeStrategy is auto | 1917 | pending | — | — | —|
| should create PR with mergeStrategy $prMergeStrategy | 1944 | pending | — | — | —|

### `updatePr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update pull request with title | 1985 | pending | — | — | —|
| should update pull target branch | 2002 | pending | — | — | —|
| should update pull request with title and body | 2025 | pending | — | — | —|
| should update pull request with draft | 2048 | pending | — | — | —|
| should close pull request | 2071 | pending | — | — | —|
| should update labels | 2096 | pending | — | — | —|
| should log a warning if labels could not be looked up | 2135 | pending | — | — | —|

### `mergePr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true when merging succeeds | 2180 | pending | — | — | —|
| should return false when merging fails | 2198 | pending | — | — | —|

### `getIssueList`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty for disabled issues | 2219 | pending | — | — | —|

### `getIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return the issue | 2231 | pending | — | — | —|
| should return null for disabled issues in getIssue | 2248 | pending | — | — | —|
| should return null on error | 2258 | pending | — | — | —|

### `findIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return existing open issue | 2273 | pending | — | — | —|
| should not return existing closed issue | 2293 | pending | — | — | —|
| should return null for missing issue | 2310 | pending | — | — | —|

### `ensureIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create issue if not found | 2326 | pending | — | — | —|
| should create issue with the correct labels | 2352 | pending | — | — | —|
| should not reopen closed issue by default | 2387 | pending | — | — | —|
| should not update labels when not necessary | 2412 | pending | — | — | —|
| should update labels when missing | 2449 | pending | — | — | —|
| should reset labels when others have been set | 2488 | pending | — | — | —|
| should reopen closed issue if desired | 2528 | pending | — | — | —|
| should not update existing closed issue if desired | 2554 | pending | — | — | —|
| should close all open duplicate issues except first one when updating | 2574 | pending | — | — | —|
| should reset issue cache when creating an issue | 2605 | pending | — | — | —|
| should gracefully fail with warning when ensuring issue | 2629 | pending | — | — | —|
| should return null for disabled issues in ensureIssue | 2651 | pending | — | — | —|

### `ensureIssueClosing`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should close issues with matching title | 2668 | pending | — | — | —|
| should return for disabled issues | 2683 | pending | — | — | —|

### `deleteLabel`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should delete a label which exists | 2692 | pending | — | — | —|
| should gracefully fail with warning if label is missing | 2708 | pending | — | — | —|

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add comment with topic if not found | 2728 | pending | — | — | —|
| should add comment without topic if not found | 2749 | pending | — | — | —|
| should update comment with topic if found | 2768 | pending | — | — | —|
| should skip if comment is up-to-date | 2789 | pending | — | — | —|
| should gracefully fail with warning when ensuring comment | 2806 | pending | — | — | —|

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should remove existing comment by topic | 2830 | pending | — | — | —|
| should remove existing comment by content | 2849 | pending | — | — | —|
| should gracefully fail with warning when removing comment | 2868 | pending | — | — | —|
| should abort silently if comment is missing | 2894 | pending | — | — | —|

### `getBranchPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return existing pull request for branch | 2913 | pending | — | — | —|
| should return null if no pull request exists | 2927 | pending | — | — | —|

### `addAssignees`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add assignees to the issue | 2941 | pending | — | — | —|

### `addReviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should assign user and team reviewers | 2956 | pending | — | — | —|
| should assign user reviewers | 2974 | pending | — | — | —|
| catches errors | 2989 | pending | — | — | —|

### `massageMarkdown`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| replaces pr links | 3008 | pending | — | — | —|
| replaces issue links | 3017 | pending | — | — | —|
| maxBodyLength | 3027 | pending | — | — | —|

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns file content | 3032 | pending | — | — | —|
| returns file content from given repo | 3048 | pending | — | — | —|
| returns file content from branch or tag | 3064 | pending | — | — | —|
| returns file content in json5 format | 3080 | pending | — | — | —|
| throws on malformed JSON | 3101 | pending | — | — | —|
| returns null on missing content | 3113 | pending | — | — | —|
| throws on errors | 3123 | pending | — | — | —|

---
