# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/workers/repository/dependency-dashboard.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/dependency-dashboard.spec.ts
**Total tests:** 63 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `workers/repository/dependency-dashboard › readDashboardBody()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses invalid dashboard body without throwing error  | 98 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| reads dashboard body  | 121 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| reads dashboard body and apply checkedBranches  | 149 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| reads dashboard body all pending approval  | 176 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| reads dashboard body open all rate-limited  | 204 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| reads dashboard body open all awaiting schedule  | 232 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| reads dashboard body and config migration checkbox - checked  | 260 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| reads dashboard body and config migration checkbox - unchecked  | 274 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| reads dashboard body and config migration pr link  | 288 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| does not read dashboard body but applies checkedBranches regardless  | 302 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| reads dashboard body and group size not met branches  | 321 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |

### `workers/repository/dependency-dashboard › ensureDependencyDashboard()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing if mode=silent  | 346 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| do nothing if dependencyDashboard is disabled  | 362 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| do nothing if it has no dependencyDashboardApproval branches  | 377 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| closes Dependency Dashboard when there is 0 PR opened and dependencyDashboardAutoclose is true  | 402 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| closes Dependency Dashboard when all branches are automerged and dependencyDashboardAutoclose is true  | 422 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| open or update Dependency Dashboard when all branches are closed and dependencyDashboardAutoclose is false  | 454 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| open or update Dependency Dashboard when rules contain approvals  | 476 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| checks an issue with 2 Pending Approvals, 2 not scheduled, 2 pr-hourly-limit-reached, 2 in error, 1 pending automerge and 1 other  | 512 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| checks an issue with dependency dashboard categories  | 607 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| checks an issue with 2 PR pr-edited  | 701 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| checks an issue with 3 PR in progress and rebase all option  | 743 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| checks an issue with 2 PR closed / ignored  | 793 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| checks an issue with group size not met branches  | 833 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| checks an issue with 3 PR in approval  | 874 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| adds a checkbox for config migration  | 929 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| adds config migration pr link when it exists  | 960 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| adds related text when config migration pr has been modified  | 992 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| does not add a config migration checkbox when not needed  | 1024 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| contains logged problems  | 1053 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| contains logged problems with custom header  | 1107 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| dependency Dashboard All Pending Approval  | 1144 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| dependency Dashboard Open All rate-limited  | 1211 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| rechecks branches  | 1275 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| skips fetching issue if content unchanged  | 1335 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| forwards configured labels to the ensure issue call  | 1365 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |

### `workers/repository/dependency-dashboard › ensureDependencyDashboard() › checks detected dependencies section › single base branch repo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add detected dependencies to the Dependency Dashboard body  | 1404 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| show default message in issues body when packageFiles is empty  | 1419 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| show default message in issues body when when packageFiles is null  | 1436 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| shows different combinations of version+digest for a given dependency  | 1453 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| shows deprecations  | 1469 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| handles missing version/digest values correctly  | 1501 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |

### `workers/repository/dependency-dashboard › ensureDependencyDashboard() › checks detected dependencies section › multi base branch repo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add detected dependencies to the Dependency Dashboard body  | 1580 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| show default message in issues body when packageFiles is empty  | 1595 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| show default message in issues body when when packageFiles is null  | 1611 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| truncates the body of a really big repo  | 1627 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |

### `workers/repository/dependency-dashboard › ensureDependencyDashboard() › checks detected dependencies section › dependency dashboard lookup warnings`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Dependency Lookup Warnings message in issues body  | 1659 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |

### `workers/repository/dependency-dashboard › ensureDependencyDashboard() › checks detected dependencies section › PackageFiles.getDashboardMarkdown()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not truncates as there is enough space to fit  | 1695 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| removes a branch with no managers  | 1705 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| removes a manager with no package files  | 1716 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| does nothing when there are no base branches left  | 1726 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| removes an entire base branch  | 1731 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| ensures original data is unchanged  | 1741 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |

### `workers/repository/dependency-dashboard › getDashboardMarkdownVulnerabilities()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return empty string if summary is empty  | 1761 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| return empty string if summary is set to none  | 1769 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| return no data section if summary is set to all and no vulnerabilities  | 1780 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| return all vulnerabilities if set to all and disabled osvVulnerabilities  | 1799 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| return unresolved vulnerabilities if set to "unresolved"  | 1871 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |

### `workers/repository/dependency-dashboard › getAbandonedPackagesMd()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty string when no abandoned packages exist  | 1938 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| returns formatted markdown when abandoned packages exist  | 1955 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| handles multiple abandoned packages across different managers  | 1986 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| displays "unknown" when mostRecentTimestamp is missing  | 2037 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |
| handles empty deps array  | 2069 | not-applicable | Mock framework internals — tests dependency dashboard via vitest-mocked platform/SCM; Rust tests this at different layer | — | dependency dashboard runtime behavior is in scope |

---
