# `lib/workers/repository/extract/manager-files.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**1/5 in-scope tests ported** (4 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 22 | returns empty of manager is disabled | pending | — |
| 28 | returns empty of manager is not enabled | pending | — |
| 35 | skips files if null content returned | pending | — |
| 46 | returns files with extractpackagefile | ported | [`crates/renovate-core/src/workers/repository/extract/manager_files.rs:223`](../../../../../../../crates/renovate-core/src/workers/repository/extract/manager_files.rs#L223) |
| 66 | returns files with extractallpackagefiles | pending | — |

