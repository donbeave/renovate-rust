# `lib/workers/repository/process/extract-update.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**1/18 in-scope tests ported** (17 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 57 | runs with no basebranchpatterns | pending | — |
| 80 | runs with basebranchpatterns | pending | — |
| 99 | uses repository cache | pending | — |
| 122 | fetches vulnerabilities | ported | [`crates/renovate-core/src/workers/repository/process/vulnerabilities.rs:134`](../../../../../../../crates/renovate-core/src/workers/repository/process/vulnerabilities.rs#L134) |
| 141 | handles exception when fetching vulnerabilities | pending | — |
| 159 | skips malicious package updates | pending | — |
| 259 | logs a warning | pending | — |
| 313 | deletes the skipreason and skipstage, to allow the update phase to continue updating | pending | — |
| 361 | when skipreason=malicious-version-in-use, it logs a warning for each skipreason | pending | — |
| 449 | undefined cache | pending | — |
| 454 | returns false if no revision | pending | — |
| 460 | returns false if revision mismatch | pending | — |
| 466 | partial cache | pending | — |
| 471 | sha mismatch | pending | — |
| 481 | config change | pending | — |
| 491 | invalid if no extractionfingerprints | pending | — |
| 508 | invalid if changed fingerprints | pending | — |
| 515 | valid cache and config | pending | — |

