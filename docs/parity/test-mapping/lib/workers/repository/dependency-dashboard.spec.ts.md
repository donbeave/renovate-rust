# `lib/workers/repository/dependency-dashboard.spec.ts`

[← `worker/repository`](../../../_by-module/worker/repository.md) · [all modules](../../../README.md)

**1/63 in-scope tests ported** (62 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 98 | parses invalid dashboard body without throwing error | pending | — |
| 121 | reads dashboard body | pending | — |
| 148 | reads dashboard body and apply checkedbranches | pending | — |
| 175 | reads dashboard body all pending approval | pending | — |
| 203 | reads dashboard body open all rate-limited | pending | — |
| 231 | reads dashboard body open all awaiting schedule | pending | — |
| 259 | reads dashboard body and config migration checkbox - checked | pending | — |
| 273 | reads dashboard body and config migration checkbox - unchecked | pending | — |
| 287 | reads dashboard body and config migration pr link | pending | — |
| 301 | does not read dashboard body but applies checkedbranches regardless | pending | — |
| 320 | reads dashboard body and group size not met branches | pending | — |
| 345 | does nothing if mode=silent | pending | — |
| 361 | do nothing if dependencydashboard is disabled | pending | — |
| 376 | do nothing if it has no dependencydashboardapproval branches | pending | — |
| 401 | closes dependency dashboard when there is 0 pr opened and dependencydashboardautoclose is true | pending | — |
| 421 | closes dependency dashboard when all branches are automerged and dependencydashboardautoclose is true | pending | — |
| 453 | open or update dependency dashboard when all branches are closed and dependencydashboardautoclose is false | pending | — |
| 475 | open or update dependency dashboard when rules contain approvals | pending | — |
| 511 | checks an issue with 2 pending approvals, 2 not scheduled, 2 pr-hourly-limit-reached, 2 in error, 1 pending automerge and 1 other | pending | — |
| 606 | checks an issue with dependency dashboard categories | pending | — |
| 700 | checks an issue with 2 pr pr-edited | pending | — |
| 742 | checks an issue with 3 pr in progress and rebase all option | pending | — |
| 792 | checks an issue with 2 pr closed / ignored | pending | — |
| 832 | checks an issue with group size not met branches | pending | — |
| 873 | checks an issue with 3 pr in approval | pending | — |
| 928 | adds a checkbox for config migration | ported | [`crates/renovate-core/src/config/migration.rs:470`](../../../../../../crates/renovate-core/src/config/migration.rs#L470) |
| 959 | adds config migration pr link when it exists | pending | — |
| 991 | adds related text when config migration pr has been modified | pending | — |
| 1023 | does not add a config migration checkbox when not needed | pending | — |
| 1052 | contains logged problems | pending | — |
| 1106 | contains logged problems with custom header | pending | — |
| 1143 | dependency dashboard all pending approval | pending | — |
| 1210 | dependency dashboard open all rate-limited | pending | — |
| 1274 | rechecks branches | pending | — |
| 1334 | skips fetching issue if content unchanged | pending | — |
| 1364 | forwards configured labels to the ensure issue call | pending | — |
| 1403 | add detected dependencies to the dependency dashboard body | pending | — |
| 1418 | show default message in issues body when packagefiles is empty | pending | — |
| 1435 | show default message in issues body when when packagefiles is null | pending | — |
| 1452 | shows different combinations of version+digest for a given dependency | pending | — |
| 1468 | shows deprecations | pending | — |
| 1500 | handles missing version/digest values correctly | pending | — |
| 1579 | add detected dependencies to the dependency dashboard body | pending | — |
| 1594 | show default message in issues body when packagefiles is empty | pending | — |
| 1610 | show default message in issues body when when packagefiles is null | pending | — |
| 1626 | truncates the body of a really big repo | pending | — |
| 1658 | dependency lookup warnings message in issues body | pending | — |
| 1694 | does not truncates as there is enough space to fit | pending | — |
| 1704 | removes a branch with no managers | pending | — |
| 1715 | removes a manager with no package files | pending | — |
| 1725 | does nothing when there are no base branches left | pending | — |
| 1730 | removes an entire base branch | pending | — |
| 1740 | ensures original data is unchanged | pending | — |
| 1760 | return empty string if summary is empty | pending | — |
| 1768 | return empty string if summary is set to none | pending | — |
| 1779 | return no data section if summary is set to all and no vulnerabilities | pending | — |
| 1798 | return all vulnerabilities if set to all and disabled osvvulnerabilities | pending | — |
| 1870 | return unresolved vulnerabilities if set to "unresolved" | pending | — |
| 1937 | returns empty string when no abandoned packages exist | pending | — |
| 1954 | returns formatted markdown when abandoned packages exist | pending | — |
| 1985 | handles multiple abandoned packages across different managers | pending | — |
| 2036 | displays "unknown" when mostrecenttimestamp is missing | pending | — |
| 2068 | handles empty deps array | pending | — |

