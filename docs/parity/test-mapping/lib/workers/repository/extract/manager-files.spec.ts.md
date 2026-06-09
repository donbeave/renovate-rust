# `lib/workers/repository/extract/manager-files.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**3/5 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 22 | returns empty of manager is disabled | ported | [`crates/renovate-core/src/workers/repository/extract/manager_files.rs:223`](../../../../../../../crates/renovate-core/src/workers/repository/extract/manager_files.rs#L223) |
| 28 | returns empty of manager is not enabled | pending | — |
| 35 | skips files if null content returned | ported | [`crates/renovate-core/src/workers/repository/extract/manager_files.rs:239`](../../../../../../../crates/renovate-core/src/workers/repository/extract/manager_files.rs#L239) |
| 46 | returns files with extractpackagefile | ported | [`crates/renovate-core/src/workers/repository/extract/manager_files.rs:257`](../../../../../../../crates/renovate-core/src/workers/repository/extract/manager_files.rs#L257) |
| 66 | returns files with extractallpackagefiles | pending | — |

