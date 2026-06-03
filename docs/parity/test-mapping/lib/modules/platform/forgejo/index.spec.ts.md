# `lib/modules/platform/forgejo/index.spec.ts`

[← `platform/forgejo`](../../../../_by-module/platform/forgejo.md) · [all modules](../../../../README.md)

**3/137 ported** (134 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 279 | should throw if no token | pending | — |
| 283 | should throw if auth fails | pending | — |
| 292 | should support default endpoint | pending | — |
| 306 | should support custom endpoint | pending | — |
| 325 | should support custom endpoint including api path | pending | — |
| 344 | should use username as author name if full name is missing | pending | — |
| 363 | should propagate any other errors from getrepos | pending | — |
| 377 | should return an array of repos | pending | — |
| 395 | should return an filtered array of repos | pending | — |
| 432 | should query the organization endpoint for each namespace | pending | — |
| 446 | sorts repos | pending | — |
| 475 | should propagate api errors | pending | — |
| 484 | should propagate org api errors | pending | — |
| 497 | should abort when repo is archived | pending | — |
| 511 | should abort when repo is mirrored | pending | — |
| 525 | should abort when repo is empty | pending | — |
| 539 | should abort when repo has insufficient permissions | pending | — |
| 557 | should abort when repo has pulls disabled | pending | — |
| 571 | should abort when repo has no available merge methods | pending | — |
| 585 | should select default merge method when it is allowed | pending | — |
| 608 | should fall back to merge method as per ordered list when default not allowed | pending | — |
| 631 | should throw if unknown default merge style is configured | pending | — |
| 646 | should use clone_url of repo if giturl is not specified | pending | — |
| 665 | should use clone_url of repo if giturl has value default | pending | — |
| 685 | should use ssh_url of repo if giturl has value ssh | pending | — |
| 705 | should abort when giturl has value ssh but ssh_url is empty | pending | — |
| 724 | should use generated url of repo if giturl has value endpoint | pending | — |
| 746 | should abort when clone_url is empty | pending | — |
| 767 | should use given access token if giturl has value endpoint | pending | — |
| 798 | should use given access token if giturl is not specified | pending | — |
| 826 | should abort when clone_url is not valid | pending | — |
| 849 | should create a new commit status | pending | — |
| 877 | should default to pending state | pending | — |
| 905 | should include url if specified | pending | — |
| 935 | should gracefully fail with warning when setting branch status | pending | — |
| 974 | should return yellow for unknown result | pending | — |
| 987 | should return pending state for pending result | pending | — |
| 1000 | should return green state for success result | pending | — |
| 1013 | should return yellow for all other results | pending | — |
| 1026 | should abort when branch status returns 404 | pending | — |
| 1039 | should propagate any other errors from getbranchstatus | pending | — |
| 1052 | should treat internal checks as success | pending | — |
| 1074 | should not treat internal checks as success | pending | — |
| 1098 | should return null with no results | pending | — |
| 1111 | should return null with no matching results | pending | — |
| 1136 | should return yellow with unknown status | pending | — |
| 1161 | should return green of matching result | pending | — |
| 1188 | should return list of pull requests | pending | — |
| 1206 | should filter list by creator | pending | — |
| 1249 | should cache results after first query | pending | — |
| 1275 | should update cache results | pending | — |
| 1303 | should return enriched pull request which exists if open | pending | — |
| 1317 | should fallback to direct fetching if cache fails | pending | — |
| 1334 | should return null for missing pull request in getpr | pending | — |
| 1350 | should throw temporary error for null pull request | pending | — |
| 1364 | should find pull request without title or state | pending | — |
| 1381 | should find pull request with title | pending | — |
| 1401 | should find pull request with state | pending | — |
| 1421 | should not find pull request with inverted state | pending | — |
| 1442 | should find pull request with title and state | pending | — |
| 1464 | should find pull request with draft | pending | — |
| 1486 | should find merged pull request | pending | — |
| 1504 | should return null for missing pull request in findpr | pending | — |
| 1518 | finds pr from other authors using base and head | pending | — |
| 1556 | returns null if cannot find pr from other author | pending | — |
| 1600 | should use base branch by default | pending | — |
| 1624 | should use default branch if requested | pending | — |
| 1646 | should resolve and apply optional repo and org labels to pull request | pending | — |
| 1672 | should resolve and apply optional repo labels to pull request | pending | — |
| 1696 | should ensure new pull request gets added to cached pull requests | pending | — |
| 1719 | should attempt to resolve 409 conflict error (w/o update) | pending | — |
| 1743 | should attempt to resolve 409 conflict error (w/ update) | pending | — |
| 1769 | should abort when response for created pull request is invalid | pending | — |
| 1787 | should use platform automerge | pending | — |
| 1813 | should not use platform automerge on forgejo v7 | pending | — |
| 1837 | should not use platform automerge on forgejo v7 lts | pending | — |
| 1861 | continues on platform automerge error | pending | — |
| 1890 | continues if platform automerge is not supported | pending | — |
| 1918 | should create pr with repository merge method when automergestrategy is auto | pending | — |
| 1945 | _(it.each / template — verify manually)_ | ? | — |
| 1986 | should update pull request with title | pending | — |
| 2003 | should update pull target branch | pending | — |
| 2026 | should update pull request with title and body | pending | — |
| 2049 | should update pull request with draft | pending | — |
| 2072 | should close pull request | pending | — |
| 2097 | should update labels | pending | — |
| 2136 | should log a warning if labels could not be looked up | pending | — |
| 2181 | should return true when merging succeeds | pending | — |
| 2199 | should return false when merging fails | pending | — |
| 2220 | should return empty for disabled issues | pending | — |
| 2232 | should return the issue | pending | — |
| 2249 | should return null for disabled issues in getissue | pending | — |
| 2259 | should return null on error | pending | — |
| 2274 | should return existing open issue | pending | — |
| 2294 | should not return existing closed issue | pending | — |
| 2311 | should return null for missing issue | pending | — |
| 2327 | should create issue if not found | pending | — |
| 2353 | should create issue with the correct labels | pending | — |
| 2388 | should not reopen closed issue by default | pending | — |
| 2413 | should not update labels when not necessary | pending | — |
| 2450 | should update labels when missing | pending | — |
| 2489 | should reset labels when others have been set | pending | — |
| 2529 | should reopen closed issue if desired | pending | — |
| 2555 | should not update existing closed issue if desired | pending | — |
| 2575 | should close all open duplicate issues except first one when updating | pending | — |
| 2606 | should reset issue cache when creating an issue | pending | — |
| 2630 | should gracefully fail with warning when ensuring issue | pending | — |
| 2652 | should return null for disabled issues in ensureissue | pending | — |
| 2669 | should close issues with matching title | pending | — |
| 2684 | should return for disabled issues | pending | — |
| 2693 | should delete a label which exists | pending | — |
| 2709 | should gracefully fail with warning if label is missing | pending | — |
| 2729 | should add comment with topic if not found | pending | — |
| 2750 | should add comment without topic if not found | pending | — |
| 2769 | should update comment with topic if found | pending | — |
| 2790 | should skip if comment is up-to-date | pending | — |
| 2807 | should gracefully fail with warning when ensuring comment | pending | — |
| 2831 | should remove existing comment by topic | pending | — |
| 2850 | should remove existing comment by content | pending | — |
| 2869 | should gracefully fail with warning when removing comment | pending | — |
| 2895 | should abort silently if comment is missing | pending | — |
| 2914 | should return existing pull request for branch | pending | — |
| 2928 | should return null if no pull request exists | pending | — |
| 2942 | should add assignees to the issue | pending | — |
| 2957 | should assign user and team reviewers | pending | — |
| 2975 | should assign user reviewers | pending | — |
| 2990 | catches errors | pending | — |
| 3009 | replaces pr links | ported | `crates/renovate-core/src/platform/gitea_forgejo_utils.rs:208` |
| 3018 | replaces issue links | ported | `crates/renovate-core/src/platform/gitea_forgejo_utils.rs:219` |
| 3028 | maxbodylength | ported | `crates/renovate-core/src/platform/gitea_forgejo_utils.rs:230` |
| 3033 | returns file content | pending | — |
| 3049 | returns file content from given repo | pending | — |
| 3065 | returns file content from branch or tag | pending | — |
| 3081 | returns file content in json5 format | pending | — |
| 3102 | throws on malformed json | pending | — |
| 3114 | returns null on missing content | pending | — |
| 3124 | throws on errors | pending | — |

