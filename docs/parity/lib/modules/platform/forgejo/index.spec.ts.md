# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/forgejo/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/forgejo/index.spec.ts
**Total tests:** 137 | **Ported:** 3 | **Actionable:** 3 | **Status:** done

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no token | 278 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should throw if auth fails | 282 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should support default endpoint | 291 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should support custom endpoint | 305 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should support custom endpoint including api path | 324 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should use username as author name if full name is missing | 343 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `getRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should propagate any other errors from getRepos | 362 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should return an array of repos | 376 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should return an filtered array of repos | 394 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should query the organization endpoint for each namespace | 431 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| Sorts repos | 445 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `initRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should propagate API errors | 474 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should propagate org API errors | 483 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should abort when repo is archived | 496 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should abort when repo is mirrored | 510 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should abort when repo is empty | 524 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should abort when repo has insufficient permissions | 538 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should abort when repo has pulls disabled | 556 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should abort when repo has no available merge methods | 570 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should select default merge method when it is allowed | 584 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should fall back to merge method as per ordered list when default not allowed | 607 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should throw if unknown default merge style is configured | 630 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should use clone_url of repo if gitUrl is not specified | 645 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should use clone_url of repo if gitUrl has value default | 664 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should use ssh_url of repo if gitUrl has value ssh | 684 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should abort when gitUrl has value ssh but ssh_url is empty | 704 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should use generated url of repo if gitUrl has value endpoint | 723 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should abort when clone_url is empty | 745 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should use given access token if gitUrl has value endpoint | 766 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should use given access token if gitUrl is not specified | 797 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should abort when clone_url is not valid | 825 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `setBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create a new commit status | 848 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should default to pending state | 876 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should include url if specified | 904 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should gracefully fail with warning when setting branch status | 934 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `getBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return yellow for unknown result | 973 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should return pending state for pending result | 986 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should return green state for success result | 999 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should return yellow for all other results | 1012 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should abort when branch status returns 404 | 1025 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should propagate any other errors from getBranchStatus | 1038 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should treat internal checks as success | 1051 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should not treat internal checks as success | 1073 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `getBranchStatusCheck`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null with no results | 1097 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should return null with no matching results | 1110 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should return yellow with unknown status | 1135 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should return green of matching result | 1160 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `getPrList`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return list of pull requests | 1187 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should filter list by creator | 1205 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should cache results after first query | 1248 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should update cache results | 1274 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `getPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return enriched pull request which exists if open | 1302 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should fallback to direct fetching if cache fails | 1316 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should return null for missing pull request in getPr | 1333 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should throw temporary error for null pull request | 1349 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `findPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should find pull request without title or state | 1363 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should find pull request with title | 1380 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should find pull request with state | 1400 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should not find pull request with inverted state | 1420 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should find pull request with title and state | 1441 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should find pull request with draft | 1463 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should find merged pull request | 1485 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should return null for missing pull request in findPr | 1503 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| finds pr from other authors using base and head | 1517 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| returns null if cannot find pr from other author | 1555 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `createPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should use base branch by default | 1599 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should use default branch if requested | 1623 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should resolve and apply optional repo and org labels to pull request | 1645 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should resolve and apply optional repo labels to pull request | 1671 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should ensure new pull request gets added to cached pull requests | 1695 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should attempt to resolve 409 conflict error (w/o update) | 1718 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should attempt to resolve 409 conflict error (w/ update) | 1742 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should abort when response for created pull request is invalid | 1768 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should use platform automerge | 1786 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should not use platform automerge on forgejo v7 | 1812 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should not use platform automerge on forgejo v7 LTS | 1836 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| continues on platform automerge error | 1860 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| continues if platform automerge is not supported | 1889 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should create PR with repository merge method when automergeStrategy is auto | 1917 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should create PR with mergeStrategy $prMergeStrategy | 1944 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `updatePr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update pull request with title | 1985 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should update pull target branch | 2002 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should update pull request with title and body | 2025 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should update pull request with draft | 2048 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should close pull request | 2071 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should update labels | 2096 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should log a warning if labels could not be looked up | 2135 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `mergePr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true when merging succeeds | 2180 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should return false when merging fails | 2198 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `getIssueList`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty for disabled issues | 2219 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `getIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return the issue | 2231 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should return null for disabled issues in getIssue | 2248 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should return null on error | 2258 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `findIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return existing open issue | 2273 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should not return existing closed issue | 2293 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should return null for missing issue | 2310 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `ensureIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create issue if not found | 2326 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should create issue with the correct labels | 2352 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should not reopen closed issue by default | 2387 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should not update labels when not necessary | 2412 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should update labels when missing | 2449 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should reset labels when others have been set | 2488 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should reopen closed issue if desired | 2528 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should not update existing closed issue if desired | 2554 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should close all open duplicate issues except first one when updating | 2574 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should reset issue cache when creating an issue | 2605 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should gracefully fail with warning when ensuring issue | 2629 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should return null for disabled issues in ensureIssue | 2651 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `ensureIssueClosing`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should close issues with matching title | 2668 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should return for disabled issues | 2683 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `deleteLabel`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should delete a label which exists | 2692 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should gracefully fail with warning if label is missing | 2708 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add comment with topic if not found | 2728 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should add comment without topic if not found | 2749 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should update comment with topic if found | 2768 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should skip if comment is up-to-date | 2789 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should gracefully fail with warning when ensuring comment | 2806 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should remove existing comment by topic | 2830 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should remove existing comment by content | 2849 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should gracefully fail with warning when removing comment | 2868 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should abort silently if comment is missing | 2894 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `getBranchPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return existing pull request for branch | 2913 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should return null if no pull request exists | 2927 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `addAssignees`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add assignees to the issue | 2941 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `addReviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should assign user and team reviewers | 2956 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| should assign user reviewers | 2974 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| catches errors | 2989 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

### `massageMarkdown`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| replaces pr links | 3009 | ported | `gitea_forgejo_utils.rs` | `smart_links_replaces_pr_links` | — |
| replaces issue links | 3018 | ported | `gitea_forgejo_utils.rs` | `smart_links_replaces_issue_links` | — |
| maxBodyLength | 3028 | ported | `gitea_forgejo_utils.rs` | `max_body_length_is_1_000_000` | — |

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns file content | 3032 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| returns file content from given repo | 3048 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| returns file content from branch or tag | 3064 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| returns file content in json5 format | 3080 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| throws on malformed JSON | 3101 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| returns null on missing content | 3113 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| throws on errors | 3123 | not-applicable | Mock framework internals — tests forgejo platform via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

---
