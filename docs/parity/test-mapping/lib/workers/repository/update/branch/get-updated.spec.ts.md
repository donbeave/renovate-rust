# `lib/workers/repository/update/branch/get-updated.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**1/53 in-scope tests ported** (52 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 64 | handles autoreplace base updated | pending | — |
| 79 | handles autoreplace branch no update | pending | — |
| 96 | handles autoreplace failure | pending | — |
| 102 | handles autoreplace branch needs update | pending | — |
| 119 | handles empty | ported | [`crates/renovate-core/src/workers/repository/update/branch/get_updated.rs:373`](../../../../../../../../crates/renovate-core/src/workers/repository/update/branch/get_updated.rs#L373) |
| 130 | handles null content | pending | — |
| 139 | handles content change | pending | — |
| 159 | handles lock files | pending | — |
| 195 | handles artifact notices | pending | — |
| 243 | handles lockfilemaintenance | pending | — |
| 270 | for updatedartifacts passes proper lockfiles | pending | — |
| 306 | for nonupdatedartifacts passes proper lockfiles | pending | — |
| 345 | for lockfilemaintenance passes proper lockfiles | pending | — |
| 381 | handles isremediation success | pending | — |
| 404 | handles unsupported isremediation | pending | — |
| 426 | handles isremediation rebase | pending | — |
| 450 | handles lockfilemaintenance error | pending | — |
| 470 | handles lock file errors | pending | — |
| 492 | handles git submodules | pending | — |
| 518 | updates lock files in mixed-manager scenarios | pending | — |
| 644 | update artifacts on update-lockfile strategy | pending | — |
| 682 | update artifacts on update-lockfile strategy with no updatelockeddependency | pending | — |
| 717 | does not update artifacts when skipartifactsupdate=true | pending | — |
| 788 | _(it.each / template — verify manually)_ | ? | — |
| 862 | attempts updatelockeddependency and handles unsupported | pending | — |
| 885 | attempts updatelockeddependency and handles already-updated | pending | — |
| 909 | attempts updatelockeddependency and handles updated files with reuse branch | pending | — |
| 935 | bumps versions in updatedependency managers | pending | — |
| 957 | bumps versions in autoreplace managers | pending | — |
| 981 | handles replacement | pending | — |
| 997 | handles package files updated by multiple managers | pending | — |
| 1071 | only writes changed contents | pending | — |
| 1086 | only writes changed contents | pending | — |
| 1101 | only writes changed contents | pending | — |
| 1119 | only writes changed contents | pending | — |
| 1137 | only writes changed contents | pending | — |
| 1157 | only writes changed contents | pending | — |
| 1176 | passes package files to updateartifacts in the same order they were returned by the manager | pending | — |
| 1242 | logs an artifact error | pending | — |
| 1345 | does not add artifact error when no deps match pending versions | pending | — |
| 1376 | does not add artifact error when a different dependency has the same version as the pending version | pending | — |
| 1411 | skips pending version check when upgrade has no pendingversions | pending | — |
| 1434 | skips pending version check when no artifact results | pending | — |
| 1450 | does not add artifact error when extractpackagefile returns null | pending | — |
| 1474 | adds multiple artifact errors when multiple deps match pending versions | pending | — |
| 1519 | skips pending version check when minimumreleaseagebehaviour is not timestamp-required | pending | — |
| 1561 | adds logs a debug log if it fails to re-extract the package file | pending | — |
| 1590 | rejects when an updated dependency has no depname or packagename | pending | — |
| 1634 | adds an artifact error when an updated dependency has no depname, but does have a packagename | pending | — |
| 1670 | skips the pending-version check when re-extracted dep has no resolvable version | pending | — |
| 1715 | rejects when upgrade has no depname | pending | — |
| 1730 | rejects when upgrade has no depname | pending | — |
| 1744 | adds artifact error for nonupdatedpackagefiles (lockfile update scenario) | pending | — |

