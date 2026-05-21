# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/branch/get-updated.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/branch/get-updated.spec.ts
**Total tests:** 53 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/update/branch/get-updated › getUpdatedPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles autoreplace base updated | 64 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| handles autoreplace branch no update | 79 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| handles autoreplace failure | 96 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| handles autoreplace branch needs update | 102 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| handles empty | 119 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| handles null content | 130 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| handles content change | 139 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| handles lock files | 159 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| handles artifact notices | 195 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| handles lockFileMaintenance | 243 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| for updatedArtifacts passes proper lockFiles | 270 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| for nonUpdatedArtifacts passes proper lockFiles | 306 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| for lockFileMaintenance passes proper lockFiles | 345 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| handles isRemediation success | 381 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| handles unsupported isRemediation | 404 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| handles isRemediation rebase | 426 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| handles lockFileMaintenance error | 450 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| handles lock file errors | 470 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| handles git submodules | 492 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| updates lock files in mixed-manager scenarios | 518 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| update artifacts on update-lockfile strategy | 644 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| update artifacts on update-lockfile strategy with no updateLockedDependency | 682 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| does not update artifacts when skipArtifactsUpdate=true | 717 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| updates artifacts when skipArtifactsUpdate=$0 | 788 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| attempts updateLockedDependency and handles unsupported | 862 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| attempts updateLockedDependency and handles already-updated | 885 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| attempts updateLockedDependency and handles updated files with reuse branch | 909 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| bumps versions in updateDependency managers | 935 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| bumps versions in autoReplace managers | 957 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| handles replacement | 981 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| handles package files updated by multiple managers | 997 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |

### `workers/repository/update/branch/get-updated › getUpdatedPackageFiles() › when some artifacts have changed and others have not › updated lockfile + unsupported lockfile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| only writes changed contents | 1071 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |

### `workers/repository/update/branch/get-updated › getUpdatedPackageFiles() › when some artifacts have changed and others have not › unsupported lockfile + updated lockfile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| only writes changed contents | 1086 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |

### `workers/repository/update/branch/get-updated › getUpdatedPackageFiles() › when some artifacts have changed and others have not › lockfile update + non-lockfile update`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| only writes changed contents | 1101 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |

### `workers/repository/update/branch/get-updated › getUpdatedPackageFiles() › when some artifacts have changed and others have not › non-lockfile update + lockfile update`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| only writes changed contents | 1119 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |

### `workers/repository/update/branch/get-updated › getUpdatedPackageFiles() › when some artifacts have changed and others have not › remediation update + lockfile unsupported update`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| only writes changed contents | 1137 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |

### `workers/repository/update/branch/get-updated › getUpdatedPackageFiles() › when some artifacts have changed and others have not › lockfile unsupported update + remediation update`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| only writes changed contents | 1157 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| passes package files to updateArtifacts in the same order they were returned by the manager | 1176 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |

### `workers/repository/update/branch/get-updated › checks if an artifact update introduces a pending version › when artifact update introduces a pending version`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| logs an artifact error | 1242 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| does not add artifact error when no deps match pending versions | 1345 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| does not add artifact error when a different dependency has the same version as the pending version | 1376 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| skips pending version check when upgrade has no pendingVersions | 1411 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| skips pending version check when no artifact results | 1434 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| does not add artifact error when extractPackageFile returns null | 1450 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| adds multiple artifact errors when multiple deps match pending versions | 1474 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| skips pending version check when minimumReleaseAgeBehaviour is not timestamp-required | 1519 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| adds logs a debug log if it fails to re-extract the package file | 1561 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| rejects when an updated dependency has no depName or packageName | 1590 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| adds an artifact error when an updated dependency has no depName, but does have a packageName | 1634 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| rejects when an updated dependency has no new version | 1671 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| rejects when upgrade has no depName | 1719 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| rejects when upgrade has no depName | 1734 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |
| adds artifact error for nonUpdatedPackageFiles (lockfile update scenario) | 1748 | not-applicable | — | — | tests file update pipeline including manager-specific updaters; needs full manager+artifact infrastructure |

---
