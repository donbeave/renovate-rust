# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gitea/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gitea/index.spec.ts
**Total tests:** 134 | **Ported:** 0 | **Actionable:** 134 | **Status:** pending

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no token | 270 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw if auth fails | 274 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should support default endpoint | 283 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should support custom endpoint | 297 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should support custom endpoint including api path | 316 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use username as author name if full name is missing | 335 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should propagate any other errors | 354 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return an array of repos | 368 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return an filtered array of repos | 386 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should query the organization endpoint for each namespace | 423 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| Sorts repos | 437 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `initRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should propagate API errors | 466 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort when repo is archived | 475 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort when repo is mirrored | 489 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort when repo is empty | 503 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort when repo has insufficient permissions | 517 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort when repo has pulls disabled | 535 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort when repo has no available merge methods | 549 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should select default merge method when it is allowed | 563 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should fall back to merge method as per ordered list when default not allowed | 584 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw if unknown default merge style is configured | 606 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use clone_url of repo if gitUrl is not specified | 621 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use clone_url of repo if gitUrl has value default | 638 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use ssh_url of repo if gitUrl has value ssh | 656 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort when gitUrl has value ssh but ssh_url is empty | 674 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use generated url of repo if gitUrl has value endpoint | 691 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort when clone_url is empty | 711 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use given access token if gitUrl has value endpoint | 730 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use given access token if gitUrl is not specified | 759 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort when clone_url is not valid | 785 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `setBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create a new commit status | 806 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should default to pending state | 834 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should include url if specified | 862 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should gracefully fail with warning | 892 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return yellow for unknown result | 931 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return pending state for pending result | 944 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return green state for success result | 957 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return yellow for all other results | 970 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort when branch status returns 404 | 983 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should propagate any other errors | 996 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should treat internal checks as success | 1009 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not treat internal checks as success | 1031 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getBranchStatusCheck`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null with no results | 1055 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return null with no matching results | 1068 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return yellow with unknown status | 1093 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return green of matching result | 1118 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getPrList`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return list of pull requests | 1145 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should filter list by creator | 1163 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should cache results after first query | 1206 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should update cache results | 1232 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return enriched pull request which exists if open | 1260 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should fallback to direct fetching if cache fails | 1274 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return null for missing pull request | 1291 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw temporary error for null pull request | 1307 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `findPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should find pull request without title or state | 1321 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should find pull request with title | 1338 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should find pull request with state | 1358 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not find pull request with inverted state | 1378 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should find pull request with title and state | 1399 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should find pull request with draft | 1421 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should find merged pull request | 1443 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return null for missing pull request | 1461 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| finds pr from other authors using base and head | 1475 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns null if cannot find pr from other author | 1513 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `createPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should use base branch by default | 1557 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use default branch if requested | 1581 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should resolve and apply optional labels to pull request | 1603 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should ensure new pull request gets added to cached pull requests | 1629 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should attempt to resolve 409 conflict error (w/o update) | 1652 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should attempt to resolve 409 conflict error (w/ update) | 1676 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort when response for created pull request is invalid | 1702 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use platform automerge | 1720 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not use platform automerge on forgejo v7 | 1746 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not use platform automerge on forgejo v7 LTS | 1770 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| continues on platform automerge error | 1794 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| continues if platform automerge is not supported | 1823 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should create PR with repository merge method when automergeStrategy is auto | 1851 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should create PR with mergeStrategy $prMergeStrategy | 1878 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `updatePr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update pull request with title | 1919 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should update pull target branch | 1936 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should update pull request with title and body | 1959 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should update pull request with draft | 1982 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should close pull request | 2005 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should update labels | 2030 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should log a warning if labels could not be looked up | 2069 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `mergePr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true when merging succeeds | 2114 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return false when merging fails | 2132 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getIssueList`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty for disabled issues | 2153 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return the issue | 2165 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return null for disabled issues | 2182 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `findIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return existing open issue | 2194 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not return existing closed issue | 2214 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return null for missing issue | 2231 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `ensureIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create issue if not found | 2247 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should create issue with the correct labels | 2273 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not reopen closed issue by default | 2308 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not update labels when not necessary | 2333 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should update labels when missing | 2370 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should reset labels when others have been set | 2409 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should reopen closed issue if desired | 2449 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not update existing closed issue if desired | 2475 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should close all open duplicate issues except first one when updating | 2495 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should reset issue cache when creating an issue | 2526 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should gracefully fail with warning | 2550 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return null for disabled issues | 2572 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `ensureIssueClosing`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should close issues with matching title | 2589 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return for disabled issues | 2604 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `deleteLabel`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should delete a label which exists | 2613 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should gracefully fail with warning if label is missing | 2629 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add comment with topic if not found | 2649 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should add comment without topic if not found | 2670 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should update comment with topic if found | 2689 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should skip if comment is up-to-date | 2710 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should gracefully fail with warning | 2727 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should remove existing comment by topic | 2751 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should remove existing comment by content | 2770 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should gracefully fail with warning | 2789 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort silently if comment is missing | 2815 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getBranchPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return existing pull request for branch | 2834 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return null if no pull request exists | 2848 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `addAssignees`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add assignees to the issue | 2862 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `addReviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should assign reviewers | 2877 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should do nothing for older Gitea versions | 2892 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| catches errors | 2900 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `massageMarkdown`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| replaces pr links | 2920 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| replaces issue links | 2929 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| maxBodyLength | 2939 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns file content | 2944 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns file content from given repo | 2960 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns file content from branch or tag | 2976 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns file content in json5 format | 2992 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws on malformed JSON | 3013 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns null on missing content | 3025 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws on errors | 3035 | pending | — | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

---
