# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gitea/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gitea/index.spec.ts
**Total tests:** 134 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `initPlatform()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no token | 270 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw if auth fails | 274 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should support default endpoint | 283 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should support custom endpoint | 297 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should support custom endpoint including api path | 316 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use username as author name if full name is missing | 335 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should propagate any other errors | 354 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return an array of repos | 368 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return an filtered array of repos | 386 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should query the organization endpoint for each namespace | 423 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| Sorts repos | 437 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `initRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should propagate API errors | 466 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort when repo is archived | 475 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort when repo is mirrored | 489 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort when repo is empty | 503 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort when repo has insufficient permissions | 517 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort when repo has pulls disabled | 535 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort when repo has no available merge methods | 549 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should select default merge method when it is allowed | 563 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should fall back to merge method as per ordered list when default not allowed | 584 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw if unknown default merge style is configured | 606 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use clone_url of repo if gitUrl is not specified | 621 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use clone_url of repo if gitUrl has value default | 638 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use ssh_url of repo if gitUrl has value ssh | 656 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort when gitUrl has value ssh but ssh_url is empty | 674 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use generated url of repo if gitUrl has value endpoint | 691 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort when clone_url is empty | 711 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use given access token if gitUrl has value endpoint | 730 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use given access token if gitUrl is not specified | 759 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort when clone_url is not valid | 785 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `setBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create a new commit status | 806 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should default to pending state | 834 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should include url if specified | 862 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should gracefully fail with warning | 892 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getBranchStatus`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return yellow for unknown result | 931 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return pending state for pending result | 944 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return green state for success result | 957 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return yellow for all other results | 970 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort when branch status returns 404 | 983 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should propagate any other errors | 996 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should treat internal checks as success | 1009 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not treat internal checks as success | 1031 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getBranchStatusCheck`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null with no results | 1055 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return null with no matching results | 1068 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return yellow with unknown status | 1093 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return green of matching result | 1118 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getPrList`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return list of pull requests | 1145 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should filter list by creator | 1163 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should cache results after first query | 1206 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should update cache results | 1232 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return enriched pull request which exists if open | 1260 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should fallback to direct fetching if cache fails | 1274 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return null for missing pull request | 1291 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should throw temporary error for null pull request | 1307 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `findPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should find pull request without title or state | 1321 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should find pull request with title | 1338 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should find pull request with state | 1358 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not find pull request with inverted state | 1378 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should find pull request with title and state | 1399 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should find pull request with draft | 1421 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should find merged pull request | 1443 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return null for missing pull request | 1461 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| finds pr from other authors using base and head | 1475 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns null if cannot find pr from other author | 1513 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `createPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should use base branch by default | 1557 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use default branch if requested | 1581 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should resolve and apply optional labels to pull request | 1603 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should ensure new pull request gets added to cached pull requests | 1629 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should attempt to resolve 409 conflict error (w/o update) | 1652 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should attempt to resolve 409 conflict error (w/ update) | 1676 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort when response for created pull request is invalid | 1702 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should use platform automerge | 1720 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not use platform automerge on forgejo v7 | 1746 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not use platform automerge on forgejo v7 LTS | 1770 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| continues on platform automerge error | 1794 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| continues if platform automerge is not supported | 1823 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should create PR with repository merge method when automergeStrategy is auto | 1851 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should create PR with mergeStrategy $prMergeStrategy | 1878 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `updatePr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update pull request with title | 1919 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should update pull target branch | 1936 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should update pull request with title and body | 1959 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should update pull request with draft | 1982 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should close pull request | 2005 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should update labels | 2030 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should log a warning if labels could not be looked up | 2069 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `mergePr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true when merging succeeds | 2114 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return false when merging fails | 2132 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getIssueList`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty for disabled issues | 2153 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return the issue | 2165 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return null for disabled issues | 2182 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `findIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return existing open issue | 2194 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not return existing closed issue | 2214 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return null for missing issue | 2231 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `ensureIssue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should create issue if not found | 2247 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should create issue with the correct labels | 2273 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not reopen closed issue by default | 2308 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not update labels when not necessary | 2333 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should update labels when missing | 2370 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should reset labels when others have been set | 2409 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should reopen closed issue if desired | 2449 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should not update existing closed issue if desired | 2475 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should close all open duplicate issues except first one when updating | 2495 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should reset issue cache when creating an issue | 2526 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should gracefully fail with warning | 2550 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return null for disabled issues | 2572 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `ensureIssueClosing`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should close issues with matching title | 2589 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return for disabled issues | 2604 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `deleteLabel`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should delete a label which exists | 2613 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should gracefully fail with warning if label is missing | 2629 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add comment with topic if not found | 2649 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should add comment without topic if not found | 2670 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should update comment with topic if found | 2689 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should skip if comment is up-to-date | 2710 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should gracefully fail with warning | 2727 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should remove existing comment by topic | 2751 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should remove existing comment by content | 2770 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should gracefully fail with warning | 2789 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should abort silently if comment is missing | 2815 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getBranchPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return existing pull request for branch | 2834 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should return null if no pull request exists | 2848 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `addAssignees`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add assignees to the issue | 2862 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `addReviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should assign reviewers | 2877 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| should do nothing for older Gitea versions | 2892 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| catches errors | 2900 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `massageMarkdown`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| replaces pr links | 2920 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| replaces issue links | 2929 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| maxBodyLength | 2939 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

### `getJsonFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns file content | 2944 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns file content from given repo | 2960 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns file content from branch or tag | 2976 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns file content in json5 format | 2992 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws on malformed JSON | 3013 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| returns null on missing content | 3025 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |
| throws on errors | 3035 | not-applicable | Mock framework internals — tests gitea platform via nock HTTP mocks; Rust tests this at different layer | — | Rust PlatformClient implements core methods; orchestration wrappers not in Rust |

---
