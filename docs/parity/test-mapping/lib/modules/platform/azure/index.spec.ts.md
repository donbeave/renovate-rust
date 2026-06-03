# `lib/modules/platform/azure/index.spec.ts`

[← `platform/azure`](../../../../_by-module/platform/azure.md) · [all modules](../../../../README.md)

**2/79 in-scope tests ported** (77 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 102 | should throw if no endpoint | pending | — |
| 107 | should throw if no token nor a username and password | pending | — |
| 116 | should throw if a username but no password | pending | — |
| 126 | should throw if a password but no username | pending | — |
| 136 | should init | pending | — |
| 147 | should return an array of repos | pending | — |
| 201 | should initialise the config for a repo | pending | — |
| 209 | throws if repo is disabled | pending | — |
| 217 | throws if repo is not in repos list | pending | — |
| 227 | returns pr if found it open | pending | — |
| 269 | returns pr if found not open | pending | — |
| 311 | returns pr if found it close | pending | — |
| 353 | returns pr if found it all state | pending | — |
| 394 | returns pr if found matches targetbranch | pending | — |
| 442 | returns first pr if found does not match targetbranch | pending | — |
| 490 | catches errors | pending | — |
| 505 | returns empty array | pending | — |
| 517 | should return null if no pr exists | pending | — |
| 528 | should return the pr | pending | — |
| 568 | should return green if status is succeeded | pending | — |
| 590 | should return green if status is not applicable | pending | — |
| 612 | should return red if status is failed | pending | — |
| 634 | should return red if context status is error | pending | — |
| 656 | should return yellow if status is pending | pending | — |
| 678 | should return yellow if status is not set | pending | — |
| 700 | should return yellow if status is unknown | pending | — |
| 722 | should return null if status not found | pending | — |
| 746 | should pass through success | pending | — |
| 765 | should not treat internal checks as success | pending | — |
| 784 | should pass through failed | pending | — |
| 797 | should pass through pending | pending | — |
| 810 | should fall back to yellow if no statuses returned | pending | — |
| 825 | should return null if no prno is passed | pending | — |
| 830 | should return null if no pr is returned from azure | pending | — |
| 842 | should return a pr in the right format | pending | — |
| 875 | should create and return a pr object | pending | — |
| 897 | should create and return a pr object from base branch | pending | — |
| 920 | should create and return a pr object with auto-complete set | pending | — |
| 960 | should only call getmergemethod once per run when automergestrategy is auto | pending | — |
| 1043 | _(it.each / template — verify manually)_ | ? | — |
| 1097 | _(it.each / template — verify manually)_ | ? | — |
| 1158 | should create and return an approved pr object | pending | — |
| 1198 | should update the pr | pending | — |
| 1216 | should update the pr including cache | pending | — |
| 1254 | should update the pr without description | pending | — |
| 1270 | should close the pr | pending | — |
| 1288 | should reopen the pr | pending | — |
| 1306 | should re-approve the pr | pending | — |
| 1346 | adds comment if missing | pending | — |
| 1368 | updates comment if missing | pending | — |
| 1394 | does nothing if comment exists and is the same | pending | — |
| 1420 | does nothing if comment exists and is the same when there is no topic | pending | — |
| 1442 | passes comment through massagemarkdown | pending | — |
| 1494 | deletes comment by topic if found | pending | — |
| 1510 | deletes comment by content if found | pending | — |
| 1526 | comment not found | pending | — |
| 1539 | addassignees | pending | — |
| 1567 | addreviewers one valid | pending | — |
| 1593 | addreviewers all valid | pending | — |
| 1621 | returns updated pr body | ported | [`crates/renovate-core/src/platform/azure_utils.rs:456`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L456) |
| 1630 | returns updated comment content | ported | [`crates/renovate-core/src/platform/azure_utils.rs:466`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L466) |
| 1641 | should build and call the create status api properly | pending | — |
| 1673 | should build and call the create status api properly with a complex context | pending | — |
| 1707 | should complete the pr | pending | — |
| 1754 | _(it.each / template — verify manually)_ | ? | — |
| 1809 | should return false if the pr does not update successfully | pending | — |
| 1838 | should cache the mergemethod for subsequent merges | pending | — |
| 1869 | should refetch the pr if the update response has not yet been set to completed | pending | — |
| 1901 | should log a warning after retrying if the pr has still not yet been set to completed | pending | — |
| 1938 | should delete a label | pending | — |
| 1956 | returns file content | pending | — |
| 1969 | returns null when file not found | pending | — |
| 1979 | returns file content in json5 format | pending | — |
| 1995 | returns file content from branch or tag | pending | — |
| 2008 | throws on malformed json | pending | — |
| 2017 | throws on errors | pending | — |
| 2028 | supports fetch from another repo | pending | — |
| 2048 | returns null | pending | — |
| 2059 | getrawfile should check tag first and then return branch if tag was not found | pending | — |

