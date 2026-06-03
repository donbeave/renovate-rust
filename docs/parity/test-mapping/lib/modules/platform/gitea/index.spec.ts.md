# `lib/modules/platform/gitea/index.spec.ts`

[← `platform/gitea`](../../../../_by-module/platform/gitea.md) · [all modules](../../../../README.md)

**0/134 ported** (134 pending) · status: pending

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 271 | should throw if no token | pending | — |
| 275 | should throw if auth fails | pending | — |
| 284 | should support default endpoint | pending | — |
| 298 | should support custom endpoint | pending | — |
| 317 | should support custom endpoint including api path | pending | — |
| 336 | should use username as author name if full name is missing | pending | — |
| 355 | should propagate any other errors | pending | — |
| 369 | should return an array of repos | pending | — |
| 387 | should return an filtered array of repos | pending | — |
| 424 | should query the organization endpoint for each namespace | pending | — |
| 438 | sorts repos | pending | — |
| 467 | should propagate api errors | pending | — |
| 476 | should abort when repo is archived | pending | — |
| 490 | should abort when repo is mirrored | pending | — |
| 504 | should abort when repo is empty | pending | — |
| 518 | should abort when repo has insufficient permissions | pending | — |
| 536 | should abort when repo has pulls disabled | pending | — |
| 550 | should abort when repo has no available merge methods | pending | — |
| 564 | should select default merge method when it is allowed | pending | — |
| 585 | should fall back to merge method as per ordered list when default not allowed | pending | — |
| 607 | should throw if unknown default merge style is configured | pending | — |
| 622 | should use clone_url of repo if giturl is not specified | pending | — |
| 639 | should use clone_url of repo if giturl has value default | pending | — |
| 657 | should use ssh_url of repo if giturl has value ssh | pending | — |
| 675 | should abort when giturl has value ssh but ssh_url is empty | pending | — |
| 692 | should use generated url of repo if giturl has value endpoint | pending | — |
| 712 | should abort when clone_url is empty | pending | — |
| 731 | should use given access token if giturl has value endpoint | pending | — |
| 760 | should use given access token if giturl is not specified | pending | — |
| 786 | should abort when clone_url is not valid | pending | — |
| 807 | should create a new commit status | pending | — |
| 835 | should default to pending state | pending | — |
| 863 | should include url if specified | pending | — |
| 893 | should gracefully fail with warning | pending | — |
| 932 | should return yellow for unknown result | pending | — |
| 945 | should return pending state for pending result | pending | — |
| 958 | should return green state for success result | pending | — |
| 971 | should return yellow for all other results | pending | — |
| 984 | should abort when branch status returns 404 | pending | — |
| 997 | should propagate any other errors | pending | — |
| 1010 | should treat internal checks as success | pending | — |
| 1032 | should not treat internal checks as success | pending | — |
| 1056 | should return null with no results | pending | — |
| 1069 | should return null with no matching results | pending | — |
| 1094 | should return yellow with unknown status | pending | — |
| 1119 | should return green of matching result | pending | — |
| 1146 | should return list of pull requests | pending | — |
| 1164 | should filter list by creator | pending | — |
| 1207 | should cache results after first query | pending | — |
| 1233 | should update cache results | pending | — |
| 1261 | should return enriched pull request which exists if open | pending | — |
| 1275 | should fallback to direct fetching if cache fails | pending | — |
| 1292 | should return null for missing pull request | pending | — |
| 1308 | should throw temporary error for null pull request | pending | — |
| 1322 | should find pull request without title or state | pending | — |
| 1339 | should find pull request with title | pending | — |
| 1359 | should find pull request with state | pending | — |
| 1379 | should not find pull request with inverted state | pending | — |
| 1400 | should find pull request with title and state | pending | — |
| 1422 | should find pull request with draft | pending | — |
| 1444 | should find merged pull request | pending | — |
| 1462 | should return null for missing pull request | pending | — |
| 1476 | finds pr from other authors using base and head | pending | — |
| 1514 | returns null if cannot find pr from other author | pending | — |
| 1558 | should use base branch by default | pending | — |
| 1582 | should use default branch if requested | pending | — |
| 1604 | should resolve and apply optional labels to pull request | pending | — |
| 1630 | should ensure new pull request gets added to cached pull requests | pending | — |
| 1653 | should attempt to resolve 409 conflict error (w/o update) | pending | — |
| 1677 | should attempt to resolve 409 conflict error (w/ update) | pending | — |
| 1703 | should abort when response for created pull request is invalid | pending | — |
| 1721 | should use platform automerge | pending | — |
| 1747 | should not use platform automerge on forgejo v7 | pending | — |
| 1771 | should not use platform automerge on forgejo v7 lts | pending | — |
| 1795 | continues on platform automerge error | pending | — |
| 1824 | continues if platform automerge is not supported | pending | — |
| 1852 | should create pr with repository merge method when automergestrategy is auto | pending | — |
| 1879 | _(it.each / template — verify manually)_ | ? | — |
| 1920 | should update pull request with title | pending | — |
| 1937 | should update pull target branch | pending | — |
| 1960 | should update pull request with title and body | pending | — |
| 1983 | should update pull request with draft | pending | — |
| 2006 | should close pull request | pending | — |
| 2031 | should update labels | pending | — |
| 2070 | should log a warning if labels could not be looked up | pending | — |
| 2115 | should return true when merging succeeds | pending | — |
| 2133 | should return false when merging fails | pending | — |
| 2154 | should return empty for disabled issues | pending | — |
| 2166 | should return the issue | pending | — |
| 2183 | should return null for disabled issues | pending | — |
| 2195 | should return existing open issue | pending | — |
| 2215 | should not return existing closed issue | pending | — |
| 2232 | should return null for missing issue | pending | — |
| 2248 | should create issue if not found | pending | — |
| 2274 | should create issue with the correct labels | pending | — |
| 2309 | should not reopen closed issue by default | pending | — |
| 2334 | should not update labels when not necessary | pending | — |
| 2371 | should update labels when missing | pending | — |
| 2410 | should reset labels when others have been set | pending | — |
| 2450 | should reopen closed issue if desired | pending | — |
| 2476 | should not update existing closed issue if desired | pending | — |
| 2496 | should close all open duplicate issues except first one when updating | pending | — |
| 2527 | should reset issue cache when creating an issue | pending | — |
| 2551 | should gracefully fail with warning | pending | — |
| 2573 | should return null for disabled issues | pending | — |
| 2590 | should close issues with matching title | pending | — |
| 2605 | should return for disabled issues | pending | — |
| 2614 | should delete a label which exists | pending | — |
| 2630 | should gracefully fail with warning if label is missing | pending | — |
| 2650 | should add comment with topic if not found | pending | — |
| 2671 | should add comment without topic if not found | pending | — |
| 2690 | should update comment with topic if found | pending | — |
| 2711 | should skip if comment is up-to-date | pending | — |
| 2728 | should gracefully fail with warning | pending | — |
| 2752 | should remove existing comment by topic | pending | — |
| 2771 | should remove existing comment by content | pending | — |
| 2790 | should gracefully fail with warning | pending | — |
| 2816 | should abort silently if comment is missing | pending | — |
| 2835 | should return existing pull request for branch | pending | — |
| 2849 | should return null if no pull request exists | pending | — |
| 2863 | should add assignees to the issue | pending | — |
| 2878 | should assign reviewers | pending | — |
| 2893 | should do nothing for older gitea versions | pending | — |
| 2901 | catches errors | pending | — |
| 2921 | replaces pr links | pending | — |
| 2930 | replaces issue links | pending | — |
| 2940 | maxbodylength | pending | — |
| 2945 | returns file content | pending | — |
| 2961 | returns file content from given repo | pending | — |
| 2977 | returns file content from branch or tag | pending | — |
| 2993 | returns file content in json5 format | pending | — |
| 3014 | throws on malformed json | pending | — |
| 3026 | returns null on missing content | pending | — |
| 3036 | throws on errors | pending | — |

