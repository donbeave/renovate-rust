# `lib/workers/repository/finalize/prune.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**3/18 in-scope tests ported** (15 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 24 | returns if no branchlist | ported | [`crates/renovate-core/src/workers/repository/finalize/prune.rs:108`](../../../../../../../crates/renovate-core/src/workers/repository/finalize/prune.rs#L108) |
| 30 | ignores reconfigure branch | pending | — |
| 36 | returns if no defaultbranch | pending | — |
| 43 | returns if no renovate branches | ported | [`crates/renovate-core/src/workers/repository/finalize/prune.rs:118`](../../../../../../../crates/renovate-core/src/workers/repository/finalize/prune.rs#L118) |
| 51 | returns if no remaining branches | ported | [`crates/renovate-core/src/workers/repository/finalize/prune.rs:97`](../../../../../../../crates/renovate-core/src/workers/repository/finalize/prune.rs#L97) |
| 59 | renames deletes remaining branch | pending | — |
| 71 | skips rename but still deletes branch | pending | — |
| 87 | deletes with base branches | pending | — |
| 124 | uses single configured base branch instead of defaultbranch | pending | — |
| 145 | uses defaultbranch when basebranchpatterns exist but basebranches are not computed yet | pending | — |
| 172 | does nothing on dryrun | pending | — |
| 185 | does nothing on prune stale branches disabled | pending | — |
| 198 | notifies via pr changes if someone pushed to pr | pending | — |
| 213 | skips appending - abandoned to pr title if already present | pending | — |
| 227 | skips changes to pr if dry run | pending | — |
| 243 | dry run delete branch no pr | pending | — |
| 256 | delete branch no pr | pending | — |
| 268 | does not delete modified orphan branch | pending | — |

