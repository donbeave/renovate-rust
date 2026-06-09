# `lib/workers/repository/config-migration/branch/index.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**1/12 in-scope tests ported** (11 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 31 | does nothing when migration disabled and checkbox unchecked | pending | — |
| 50 | creates migration branch when migration disabled but checkbox checked | ported | [`crates/renovate-core/src/branch.rs:2798`](../../../../../../../../crates/renovate-core/src/branch.rs#L2798) |
| 71 | does not create a branch if migration branch is modified | pending | — |
| 102 | updates migration branch & refreshes pr when migration disabled but open pr exists | pending | — |
| 134 | creates migration branch when migration enabled but no pr exists | pending | — |
| 157 | updates migration branch & refresh pr when migration enabled and open pr exists | pending | — |
| 184 | dry runs update migration branch | pending | — |
| 209 | dry runs create migration pr | pending | — |
| 236 | does not create a branch when migration is disabled but needed and a closed pr exists | pending | — |
| 255 | deletes old branch and creates new migration branch when migration is disabled but needed, a closed pr exists and checkbox is checked | pending | — |
| 280 | does not create a branch when migration is enabled and a closed pr exists | pending | — |
| 299 | dry run:deletes old branch and creates new migration branch when migration is disabled but needed, a closed pr exists and checkbox is checked | pending | — |

