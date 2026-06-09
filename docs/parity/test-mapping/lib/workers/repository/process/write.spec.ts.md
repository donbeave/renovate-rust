# `lib/workers/repository/process/write.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**2/16 in-scope tests ported** (14 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 48 | stops after automerge | pending | — |
| 106 | increments branch counter | pending | — |
| 147 | return no-work if branch fingerprint is not different | ported | [`crates/renovate-core/src/workers/repository/process/fingerprint_fields.rs:31`](../../../../../../../crates/renovate-core/src/workers/repository/process/fingerprint_fields.rs#L31) |
| 176 | updates branch fingerprint when new commit is made | pending | — |
| 219 | caches same fingerprint when no commit is made and branch cache existed | ported | [`crates/renovate-core/src/workers/repository/cache.rs:142`](../../../../../../../crates/renovate-core/src/workers/repository/cache.rs#L142) |
| 264 | caches same fingerprint when no commit is made | pending | — |
| 306 | creates new branchcache when cache is not enabled | pending | — |
| 357 | returns false if no cache | pending | — |
| 368 | returns false when fingerprints are not same | pending | — |
| 378 | returns true | pending | — |
| 390 | creates minimal branch state when cache is not populated | pending | — |
| 405 | when base branch name is different updates it and invalidates related cache | pending | — |
| 438 | when base branch sha is different updates it and invalidates related values | pending | — |
| 473 | when branch sha is different updates it and invalidates related values | pending | — |
| 509 | when branch sha is different updates it and sets committimestamp | pending | — |
| 548 | no change if all parameters are same | pending | — |

