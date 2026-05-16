# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/branch/get-updated.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/branch/get-updated.spec.ts
**Total tests:** 53 | **Ported:** 0 | **Actionable:** 53 | **Status:** pending

### `workers/repository/update/branch/get-updated › getUpdatedPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles autoreplace base updated | 64 | pending | — | — | — |
| handles autoreplace branch no update | 79 | pending | — | — | — |
| handles autoreplace failure | 96 | pending | — | — | — |
| handles autoreplace branch needs update | 102 | pending | — | — | — |
| handles empty | 119 | pending | — | — | — |
| handles null content | 130 | pending | — | — | — |
| handles content change | 139 | pending | — | — | — |
| handles lock files | 159 | pending | — | — | — |
| handles artifact notices | 195 | pending | — | — | — |
| handles lockFileMaintenance | 243 | pending | — | — | — |
| for updatedArtifacts passes proper lockFiles | 270 | pending | — | — | — |
| for nonUpdatedArtifacts passes proper lockFiles | 306 | pending | — | — | — |
| for lockFileMaintenance passes proper lockFiles | 345 | pending | — | — | — |
| handles isRemediation success | 381 | pending | — | — | — |
| handles unsupported isRemediation | 404 | pending | — | — | — |
| handles isRemediation rebase | 426 | pending | — | — | — |
| handles lockFileMaintenance error | 450 | pending | — | — | — |
| handles lock file errors | 470 | pending | — | — | — |
| handles git submodules | 492 | pending | — | — | — |
| updates lock files in mixed-manager scenarios | 518 | pending | — | — | — |
| update artifacts on update-lockfile strategy | 644 | pending | — | — | — |
| update artifacts on update-lockfile strategy with no updateLockedDependency | 682 | pending | — | — | — |
| does not update artifacts when skipArtifactsUpdate=true | 717 | pending | — | — | — |
| updates artifacts when skipArtifactsUpdate=$0 | 788 | pending | — | — | — |
| attempts updateLockedDependency and handles unsupported | 862 | pending | — | — | — |
| attempts updateLockedDependency and handles already-updated | 885 | pending | — | — | — |
| attempts updateLockedDependency and handles updated files with reuse branch | 909 | pending | — | — | — |
| bumps versions in updateDependency managers | 935 | pending | — | — | — |
| bumps versions in autoReplace managers | 957 | pending | — | — | — |
| handles replacement | 981 | pending | — | — | — |
| handles package files updated by multiple managers | 997 | pending | — | — | — |

### `workers/repository/update/branch/get-updated › getUpdatedPackageFiles() › when some artifacts have changed and others have not › updated lockfile + unsupported lockfile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| only writes changed contents | 1071 | pending | — | — | — |

### `workers/repository/update/branch/get-updated › getUpdatedPackageFiles() › when some artifacts have changed and others have not › unsupported lockfile + updated lockfile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| only writes changed contents | 1086 | pending | — | — | — |

### `workers/repository/update/branch/get-updated › getUpdatedPackageFiles() › when some artifacts have changed and others have not › lockfile update + non-lockfile update`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| only writes changed contents | 1101 | pending | — | — | — |

### `workers/repository/update/branch/get-updated › getUpdatedPackageFiles() › when some artifacts have changed and others have not › non-lockfile update + lockfile update`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| only writes changed contents | 1119 | pending | — | — | — |

### `workers/repository/update/branch/get-updated › getUpdatedPackageFiles() › when some artifacts have changed and others have not › remediation update + lockfile unsupported update`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| only writes changed contents | 1137 | pending | — | — | — |

### `workers/repository/update/branch/get-updated › getUpdatedPackageFiles() › when some artifacts have changed and others have not › lockfile unsupported update + remediation update`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| only writes changed contents | 1157 | pending | — | — | — |
| passes package files to updateArtifacts in the same order they were returned by the manager | 1176 | pending | — | — | — |

### `workers/repository/update/branch/get-updated › checks if an artifact update introduces a pending version › when artifact update introduces a pending version`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| logs an artifact error | 1242 | pending | — | — | — |
| does not add artifact error when no deps match pending versions | 1345 | pending | — | — | — |
| does not add artifact error when a different dependency has the same version as the pending version | 1376 | pending | — | — | — |
| skips pending version check when upgrade has no pendingVersions | 1411 | pending | — | — | — |
| skips pending version check when no artifact results | 1434 | pending | — | — | — |
| does not add artifact error when extractPackageFile returns null | 1450 | pending | — | — | — |
| adds multiple artifact errors when multiple deps match pending versions | 1474 | pending | — | — | — |
| skips pending version check when minimumReleaseAgeBehaviour is not timestamp-required | 1519 | pending | — | — | — |
| adds logs a debug log if it fails to re-extract the package file | 1561 | pending | — | — | — |
| rejects when an updated dependency has no depName or packageName | 1590 | pending | — | — | — |
| adds an artifact error when an updated dependency has no depName, but does have a packageName | 1634 | pending | — | — | — |
| rejects when an updated dependency has no new version | 1671 | pending | — | — | — |
| rejects when upgrade has no depName | 1719 | pending | — | — | — |
| rejects when upgrade has no depName | 1734 | pending | — | — | — |
| adds artifact error for nonUpdatedPackageFiles (lockfile update scenario) | 1748 | pending | — | — | — |

---

