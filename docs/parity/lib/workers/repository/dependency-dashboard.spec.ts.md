# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/workers/repository/dependency-dashboard.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/dependency-dashboard.spec.ts
**Total tests:** 63 | **Ported:** 0 | **Actionable:** 63 | **Status:** pending

### `workers/repository/dependency-dashboard › readDashboardBody()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses invalid dashboard body without throwing error | 98 | pending | — | — | — |
| reads dashboard body | 121 | pending | — | — | — |
| reads dashboard body and apply checkedBranches | 149 | pending | — | — | — |
| reads dashboard body all pending approval | 176 | pending | — | — | — |
| reads dashboard body open all rate-limited | 204 | pending | — | — | — |
| reads dashboard body open all awaiting schedule | 232 | pending | — | — | — |
| reads dashboard body and config migration checkbox - checked | 260 | pending | — | — | — |
| reads dashboard body and config migration checkbox - unchecked | 274 | pending | — | — | — |
| reads dashboard body and config migration pr link | 288 | pending | — | — | — |
| does not read dashboard body but applies checkedBranches regardless | 302 | pending | — | — | — |
| reads dashboard body and group size not met branches | 321 | pending | — | — | — |

### `workers/repository/dependency-dashboard › ensureDependencyDashboard()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing if mode=silent | 346 | pending | — | — | — |
| do nothing if dependencyDashboard is disabled | 362 | pending | — | — | — |
| do nothing if it has no dependencyDashboardApproval branches | 377 | pending | — | — | — |
| closes Dependency Dashboard when there is 0 PR opened and dependencyDashboardAutoclose is true | 402 | pending | — | — | — |
| closes Dependency Dashboard when all branches are automerged and dependencyDashboardAutoclose is true | 422 | pending | — | — | — |
| open or update Dependency Dashboard when all branches are closed and dependencyDashboardAutoclose is false | 454 | pending | — | — | — |
| open or update Dependency Dashboard when rules contain approvals | 476 | pending | — | — | — |
| checks an issue with 2 Pending Approvals, 2 not scheduled, 2 pr-hourly-limit-reached, 2 in error, 1 pending automerge and 1 other | 512 | pending | — | — | — |
| checks an issue with dependency dashboard categories | 607 | pending | — | — | — |
| checks an issue with 2 PR pr-edited | 701 | pending | — | — | — |
| checks an issue with 3 PR in progress and rebase all option | 743 | pending | — | — | — |
| checks an issue with 2 PR closed / ignored | 793 | pending | — | — | — |
| checks an issue with group size not met branches | 833 | pending | — | — | — |
| checks an issue with 3 PR in approval | 874 | pending | — | — | — |
| adds a checkbox for config migration | 929 | pending | — | — | — |
| adds config migration pr link when it exists | 960 | pending | — | — | — |
| adds related text when config migration pr has been modified | 992 | pending | — | — | — |
| does not add a config migration checkbox when not needed | 1024 | pending | — | — | — |
| contains logged problems | 1053 | pending | — | — | — |
| contains logged problems with custom header | 1107 | pending | — | — | — |
| dependency Dashboard All Pending Approval | 1144 | pending | — | — | — |
| dependency Dashboard Open All rate-limited | 1211 | pending | — | — | — |
| rechecks branches | 1275 | pending | — | — | — |
| skips fetching issue if content unchanged | 1335 | pending | — | — | — |
| forwards configured labels to the ensure issue call | 1365 | pending | — | — | — |

### `workers/repository/dependency-dashboard › ensureDependencyDashboard() › checks detected dependencies section › single base branch repo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add detected dependencies to the Dependency Dashboard body | 1404 | pending | — | — | — |
| show default message in issues body when packageFiles is empty | 1419 | pending | — | — | — |
| show default message in issues body when when packageFiles is null | 1436 | pending | — | — | — |
| shows different combinations of version+digest for a given dependency | 1453 | pending | — | — | — |
| shows deprecations | 1469 | pending | — | — | — |
| handles missing version/digest values correctly | 1501 | pending | — | — | — |

### `workers/repository/dependency-dashboard › ensureDependencyDashboard() › checks detected dependencies section › multi base branch repo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add detected dependencies to the Dependency Dashboard body | 1580 | pending | — | — | — |
| show default message in issues body when packageFiles is empty | 1595 | pending | — | — | — |
| show default message in issues body when when packageFiles is null | 1611 | pending | — | — | — |
| truncates the body of a really big repo | 1627 | pending | — | — | — |

### `workers/repository/dependency-dashboard › ensureDependencyDashboard() › checks detected dependencies section › dependency dashboard lookup warnings`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Dependency Lookup Warnings message in issues body | 1659 | pending | — | — | — |

### `workers/repository/dependency-dashboard › ensureDependencyDashboard() › checks detected dependencies section › PackageFiles.getDashboardMarkdown()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not truncates as there is enough space to fit | 1695 | pending | — | — | — |
| removes a branch with no managers | 1705 | pending | — | — | — |
| removes a manager with no package files | 1716 | pending | — | — | — |
| does nothing when there are no base branches left | 1726 | pending | — | — | — |
| removes an entire base branch | 1731 | pending | — | — | — |
| ensures original data is unchanged | 1741 | pending | — | — | — |

### `workers/repository/dependency-dashboard › getDashboardMarkdownVulnerabilities()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return empty string if summary is empty | 1761 | pending | — | — | — |
| return empty string if summary is set to none | 1769 | pending | — | — | — |
| return no data section if summary is set to all and no vulnerabilities | 1780 | pending | — | — | — |
| return all vulnerabilities if set to all and disabled osvVulnerabilities | 1799 | pending | — | — | — |
| return unresolved vulnerabilities if set to "unresolved" | 1871 | pending | — | — | — |

### `workers/repository/dependency-dashboard › getAbandonedPackagesMd()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty string when no abandoned packages exist | 1938 | pending | — | — | — |
| returns formatted markdown when abandoned packages exist | 1955 | pending | — | — | — |
| handles multiple abandoned packages across different managers | 1986 | pending | — | — | — |
| displays "unknown" when mostRecentTimestamp is missing | 2037 | pending | — | — | — |
| handles empty deps array | 2069 | pending | — | — | — |

---

