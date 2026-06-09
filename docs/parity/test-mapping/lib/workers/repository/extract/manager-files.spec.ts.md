# `lib/workers/repository/extract/manager-files.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**2/4 in-scope tests ported** (2 pending, 1 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 22 | returns empty of manager is disabled | ported | [`crates/renovate-core/src/workers/repository/extract/manager_files.rs:223`](../../../../../../../crates/renovate-core/src/workers/repository/extract/manager_files.rs#L223) |
| 28 | returns empty of manager is not enabled | opt-out | the 'not enabled' case from enabledManagers filter (config.enabledManagers = ['npm'], then get for 'travis' or not included returns empty); the top level enabledManagers filter is in extract (the 1/5 ported in extract/index includes enabledManagers matching); the per manager get guard for !enabled is ported via L22; this L28 is the caller filter case, covered by the ported filter and guard. Opt as duplicate of ported logic or the enabledManagers filter detail in extract. |
| 35 | skips files if null content returned | pending | — |
| 46 | returns files with extractpackagefile | ported | [`crates/renovate-core/src/workers/repository/extract/manager_files.rs:239`](../../../../../../../crates/renovate-core/src/workers/repository/extract/manager_files.rs#L239) |
| 66 | returns files with extractallpackagefiles | pending | — |

