# `lib/modules/platform/gitlab/index.spec.ts`

[← `platform/gitlab`](../../../../_by-module/platform/gitlab.md) · [all modules](../../../../README.md)

**59/163 in-scope tests ported** (104 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 78 | should throw if no token | pending | — |
| 82 | should throw if endpoint is not a valid url | ported | [`crates/renovate-core/src/platform/gitlab.rs:1173`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1173) |
| 91 | should throw if auth fails | ported | [`crates/renovate-core/src/platform/gitlab.rs:1195`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1195) |
| 101 | should default to gitlab.com | ported | [`crates/renovate-core/src/platform/gitlab.rs:1210`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1210) |
| 117 | should accept custom endpoint | ported | [`crates/renovate-core/src/platform/gitlab.rs:2523`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2523) |
| 138 | should reuse existing gitauthor | ported | [`crates/renovate-core/src/platform/gitlab.rs:2352`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2352) |
| 153 | should throw an error if it receives an error | ported | [`crates/renovate-core/src/platform/gitlab.rs:1230`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1230) |
| 163 | should return an array of repos | ported | [`crates/renovate-core/src/platform/gitlab.rs:1118`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1118) |
| 185 | should return an array of repos including mirrors | ported | [`crates/renovate-core/src/platform/gitlab.rs:1274`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1274) |
| 207 | should encode the requested topics into the url | ported | [`crates/renovate-core/src/platform/gitlab.rs:1302`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1302) |
| 225 | should query the groups endpoint for each namespace | ported | [`crates/renovate-core/src/platform/gitlab.rs:1322`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1322) |
| 251 | should consider topics when querying the groups endpoint | ported | [`crates/renovate-core/src/platform/gitlab.rs:2316`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2316) |
| 272 | should include order and sort query parameters | ported | [`crates/renovate-core/src/platform/gitlab.rs:2334`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2334) |
| 317 | should escape all forward slashes in project names | ported | [`crates/renovate-core/src/platform/gitlab.rs:2502`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2502) |
| 333 | should throw an error if receiving an error | ported | [`crates/renovate-core/src/platform/gitlab.rs:1019`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1019) |
| 345 | should throw an error if repository is archived | ported | [`crates/renovate-core/src/platform/gitlab.rs:915`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L915) |
| 357 | should throw an error if repository is a mirror | ported | [`crates/renovate-core/src/platform/gitlab.rs:941`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L941) |
| 369 | should not throw an error if repository is a mirror when includemirrors option is set | pending | — |
| 389 | should throw an error if repository access is disabled | ported | [`crates/renovate-core/src/platform/gitlab.rs:1066`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1066) |
| 401 | should throw an error if mrs are disabled | ported | [`crates/renovate-core/src/platform/gitlab.rs:1092`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1092) |
| 413 | should throw an error if repository has empty_repo property | ported | [`crates/renovate-core/src/platform/gitlab.rs:967`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L967) |
| 425 | should throw an error if repository is empty | ported | [`crates/renovate-core/src/platform/gitlab.rs:993`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L993) |
| 437 | should fall back if http_url_to_repo is empty | ported | [`crates/renovate-core/src/platform/gitlab.rs:1039`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1039) |
| 456 | should use ssh_url_to_repo if giturl is set to ssh | ported | [`crates/renovate-core/src/platform/gitlab.rs:2727`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2727) |
| 473 | should throw if ssh_url_to_repo is not present but giturl is set to ssh | ported | [`crates/renovate-core/src/platform/gitlab.rs:2744`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2744) |
| 489 | should fall back respecting when gitlab_ignore_repo_url is set | pending | — |
| 522 | should return false for merge_method=merge | ported | [`crates/renovate-core/src/platform/gitlab.rs:2195`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2195) |
| 536 | should return true for merge_method=ff | ported | [`crates/renovate-core/src/platform/gitlab.rs:2222`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2222) |
| 550 | should return false when merge trains are enabled | pending | — |
| 567 | should return null if no pr exists | ported | [`crates/renovate-core/src/platform/gitlab.rs:1910`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1910) |
| 579 | should return the pr object | ported | [`crates/renovate-core/src/platform/gitlab.rs:1862`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1862) |
| 618 | should strip draft prefix from title | ported | [`crates/renovate-core/src/platform/gitlab.rs:1930`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1930) |
| 657 | should strip deprecated draft prefix from title | ported | [`crates/renovate-core/src/platform/gitlab.rs:1978`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1978) |
| 698 | returns pending if no results | ported | [`crates/renovate-core/src/platform/gitlab.rs:1520`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1520) |
| 713 | returns success if no results with merged results pipeline success | pending | — |
| 760 | returns success if all are success | pending | — |
| 778 | returns pending if all are internal success | pending | — |
| 796 | returns pending if merge request with no pipelines | pending | — |
| 839 | returns pending if all are internal success with no merged results pipeline | pending | — |
| 889 | returns success if all are internal success with merged results pipeline success | pending | — |
| 939 | returns success if optional jobs fail | pending | — |
| 957 | returns success if all are optional | pending | — |
| 972 | returns success if job is skipped | pending | — |
| 987 | returns yellow if there are no jobs expect skipped | pending | — |
| 1002 | returns failure if any mandatory jobs fails and one job is skipped | pending | — |
| 1017 | returns failure if any mandatory jobs fails | pending | — |
| 1036 | maps custom statuses to yellow | pending | — |
| 1051 | throws repository-changed | pending | — |
| 1062 | returns null if no results | ported | [`crates/renovate-core/src/platform/gitlab.rs:1799`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1799) |
| 1076 | returns null if no matching results | ported | [`crates/renovate-core/src/platform/gitlab.rs:2370`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2370) |
| 1090 | returns status if name found | pending | — |
| 1108 | returns yellow if unknown status found | pending | — |
| 1130 | should log message that branch commit sha not found | pending | — |
| 1145 | should log message that failed to retrieve commit pipeline | pending | — |
| 1177 | _(it.each / template — verify manually)_ | ? | — |
| 1205 | _(it.each / template — verify manually)_ | ? | — |
| 1233 | does not skip setting branch status when renovate_x_gitlab_skip_status_without_pipeline is not true | pending | — |
| 1266 | sets branch status when renovate_x_gitlab_skip_status_without_pipeline is true and pipeline is found | pending | — |
| 1302 | waits for 1000ms by default | pending | — |
| 1331 | set branch status with pipeline_id | pending | — |
| 1366 | waits for renovate_x_gitlab_branch_status_delay ms when set | pending | — |
| 1411 | do renovate_x_gitlab_branch_status_check_attempts attemps when set | pending | — |
| 1446 | returns null if no issue | pending | — |
| 1466 | finds issue | pending | — |
| 1490 | creates issue | ported | [`crates/renovate-core/src/platform/gitlab.rs:1546`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1546) |
| 1515 | sets issue labels | pending | — |
| 1532 | updates issue | ported | [`crates/renovate-core/src/platform/gitlab.rs:1574`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1574) |
| 1559 | updates issue with labels | ported | [`crates/renovate-core/src/platform/gitlab.rs:1534`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1534) |
| 1587 | skips update if unchanged | ported | [`crates/renovate-core/src/platform/gitlab.rs:1396`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1396) |
| 1612 | creates confidential issue | pending | — |
| 1638 | updates confidential issue | pending | — |
| 1669 | closes issue | pending | — |
| 1692 | should add the given assignee to the issue | pending | — |
| 1702 | should add the given assignees to the issue | pending | — |
| 1718 | should swallow error | pending | — |
| 1732 | should log message for each assignee that could not be found | pending | — |
| 1766 | should not be supported in too low version | pending | — |
| 1787 | should fail to get existing reviewers | pending | — |
| 1799 | should not fail if some reviewers are unknown | pending | — |
| 1821 | should warn and return early if new reviewers ids could be fetched | pending | — |
| 1844 | should add gitlab group members as reviewers to mr | pending | — |
| 1866 | should fail to add reviewers to the mr | pending | — |
| 1886 | should add the given reviewers to the mr | pending | — |
| 1906 | should only add reviewers if necessary | pending | — |
| 1927 | add comment if not found | pending | — |
| 1943 | add updates comment if necessary | pending | — |
| 1959 | skips comment | pending | — |
| 1973 | handles comment with no description | pending | — |
| 1989 | deletes comment by topic if found | pending | — |
| 2005 | deletes comment by content if found | pending | — |
| 2023 | returns true if no title and all state | pending | — |
| 2047 | returns true if not open | pending | — |
| 2072 | returns true if open and with title | pending | — |
| 2098 | returns true with title | pending | — |
| 2123 | returns true with draft prefix title | ported | [`crates/renovate-core/src/platform/gitlab.rs:2385`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2385) |
| 2148 | returns true with deprecated draft prefix title | ported | [`crates/renovate-core/src/platform/gitlab.rs:2432`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2432) |
| 2173 | finds pr from other authors | ported | [`crates/renovate-core/src/platform/gitlab.rs:2249`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2249) |
| 2205 | returns null if no pr found - (includeotherauthors) | ported | [`crates/renovate-core/src/platform/gitlab.rs:2296`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2296) |
| 2245 | returns the pr | ported | [`crates/renovate-core/src/platform/gitlab.rs:1749`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1749) |
| 2277 | uses default branch | ported | [`crates/renovate-core/src/platform/gitlab.rs:1360`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1360) |
| 2309 | supports draftpr on < 13.2 | ported | [`crates/renovate-core/src/platform/gitlab.rs:2072`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2072) |
| 2341 | supports draftpr on >= 13.2 | pending | — |
| 2373 | auto-accepts the mr when requested | pending | — |
| 2416 | adds the mr to a merge train when merge trains are enabled on the project | pending | — |
| 2468 | falls back to /merge endpoint when merge trains enabled but gitlab < 17.11 | pending | — |
| 2521 | retries the merge_trains endpoint on transient failure | pending | — |
| 2572 | should parse merge_status attribute if detailed_merge_status is not set (on < 15.6) | pending | — |
| 2637 | should parse detailed_merge_status attribute on >= 15.6 | pending | — |
| 2695 | should retry auto merge creation on 405 method not allowed | pending | — |
| 2773 | should not retry if mr is mergeable and pipeline is running | pending | — |
| 2817 | raises with squash enabled when repository squash option is default_on | pending | — |
| 2860 | raises with squash enabled when repository squash option is always | pending | — |
| 2903 | adds approval rule to ignore all approvals | pending | — |
| 2957 | adds approval rule to ignore all approvals when platformautomerge is false | pending | — |
| 3005 | will modify a rule of type any_approvers, if such a rule exists | pending | — |
| 3067 | will remove rules of type regular, if such rules exist | pending | — |
| 3140 | does not try to remove "report_approver" and "code_owner" approval rules | pending | — |
| 3223 | does not try to create already existing approval rule | pending | — |
| 3277 | silently ignores approval rules adding errors | pending | — |
| 3331 | auto-approves when enabled | pending | — |
| 3368 | auto-approve with different user | pending | — |
| 3407 | should swallow an error on auto-approve | pending | — |
| 3442 | returns the pr | ported | [`crates/renovate-core/src/platform/gitlab.rs:1749`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1749) |
| 3466 | removes draft prefix from returned title | ported | [`crates/renovate-core/src/platform/gitlab.rs:1808`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1808) |
| 3490 | removes deprecated draft prefix from returned title | ported | [`crates/renovate-core/src/platform/gitlab.rs:1834`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1834) |
| 3514 | returns the mergeable pr | ported | [`crates/renovate-core/src/platform/gitlab.rs:1784`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1784) |
| 3539 | returns the pr with nonexisting branch | pending | — |
| 3567 | returns the pr with reviewers | pending | — |
| 3610 | updates the pr | ported | [`crates/renovate-core/src/platform/gitlab.rs:1408`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1408) |
| 3643 | retains draft status when draft uses current prefix | ported | [`crates/renovate-core/src/platform/gitlab.rs:1467`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1467) |
| 3676 | retains draft status when draft uses deprecated prefix | ported | [`crates/renovate-core/src/platform/gitlab.rs:2106`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2106) |
| 3709 | updates target branch of the pr | ported | [`crates/renovate-core/src/platform/gitlab.rs:2154`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2154) |
| 3748 | auto-approves when enabled | pending | — |
| 3791 | closes the pr | ported | [`crates/renovate-core/src/platform/gitlab.rs:1441`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L1441) |
| 3830 | adds and removes labels | pending | — |
| 3880 | should set automatic merge | pending | — |
| 3903 | should skip retries when merge_when_pipeline_succeeds is already enabled | pending | — |
| 3925 | merges the pr | pending | — |
| 3950 | strips invalid unicode null characters | ported | [`crates/renovate-core/src/platform/gitlab.rs:2673`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2673) |
| 3958 | replaces pr with mr including pluralization | ported | [`crates/renovate-core/src/platform/gitlab.rs:2682`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2682) |
| 3966 | replaces pr reference with mr reference | ported | [`crates/renovate-core/src/platform/gitlab.rs:2691`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2691) |
| 3972 | replaces pr relative link with mr reference | ported | [`crates/renovate-core/src/platform/gitlab.rs:2700`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2700) |
| 3980 | replaces issues relative link with issue reference | ported | [`crates/renovate-core/src/platform/gitlab.rs:2709`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2709) |
| 3988 | avoids false positives when replacing pr with mr | ported | [`crates/renovate-core/src/platform/gitlab.rs:2720`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2720) |
| 3993 | returns updated pr body | ported | [`crates/renovate-core/src/platform/gitlab.rs:2758`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2758) |
| 4002 | truncates description if too low api version | pending | — |
| 4012 | truncates description for api version gt 13.4 | pending | — |
| 4024 | should delete the label | pending | — |
| 4049 | returns null | ported | [`crates/renovate-core/src/platform/gitlab.rs:2479`](../../../../../../../crates/renovate-core/src/platform/gitlab.rs#L2479) |
| 4062 | returns file content | pending | — |
| 4076 | returns file content in json5 format | pending | — |
| 4095 | returns file content from given repo | pending | — |
| 4109 | returns file content from branch or tag | pending | — |
| 4127 | throws on malformed json | pending | — |
| 4139 | throws on errors | pending | — |
| 4151 | filters users that are busy | pending | — |
| 4169 | keeps users with missing availability | pending | — |
| 4178 | keeps users with failing requests | pending | — |
| 4189 | expands group members for groups with members | pending | — |
| 4209 | users are not expanded when 404 | pending | — |
| 4218 | users are not expanded when non 404 | pending | — |
| 4234 | groups with no members expand into empty list | pending | — |
| 4245 | includes email in final result | pending | — |

