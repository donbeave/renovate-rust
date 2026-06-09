# `lib/workers/repository/extract/manager-files.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**4/4 in-scope tests ported** (0 pending, 1 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 22 | returns empty of manager is disabled | ported | [`crates/renovate-core/src/workers/repository/extract/manager_files.rs:223`](../../../../../../../crates/renovate-core/src/workers/repository/extract/manager_files.rs#L223) |
| 28 | returns empty of manager is not enabled | opt-out | the enabledManagers list filter (when the list is set in config, a manager not present in it returns empty even if its local .enabled=true and fileList non-empty) is decided when building the list of ManagerFile entries in the extract orchestrator (getEnabledManagersList, skips_non_enabled_managers test in managers.rs, and the loop in workers/repository/extract); this unit's get_manager_package_files only sees the effective per-ManagerFile .enabled (already ported as 'returns empty of manager is disabled'); the list interaction test in this spec has no additional unique business logic in the isolated fn to port beyond the caller coverage. |
| 35 | skips files if null content returned | ported | [`crates/renovate-core/src/workers/repository/extract/manager_files.rs:239`](../../../../../../../crates/renovate-core/src/workers/repository/extract/manager_files.rs#L239) |
| 46 | returns files with extractpackagefile | ported | [`crates/renovate-core/src/workers/repository/extract/manager_files.rs:257`](../../../../../../../crates/renovate-core/src/workers/repository/extract/manager_files.rs#L257) |
| 66 | returns files with extractallpackagefiles | ported | [`crates/renovate-core/src/workers/repository/extract/manager_files.rs:278`](../../../../../../../crates/renovate-core/src/workers/repository/extract/manager_files.rs#L278) |

