# `lib/workers/repository/process/fetch.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**7/13 in-scope tests ported** (6 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 21 | handles empty deps | ported | [`crates/renovate-core/src/workers/repository/process/fetch.rs:197`](../../../../../../../crates/renovate-core/src/workers/repository/process/fetch.rs#L197) |
| 31 | handles ignored, skipped and disabled | pending | — |
| 85 | fetches updates | ported | [`crates/renovate-core/src/workers/repository/process/fetch.rs:171`](../../../../../../../crates/renovate-core/src/workers/repository/process/fetch.rs#L171) |
| 119 | is merged from packagefile with config | pending | — |
| 147 | is set from packagefile if only set on packagefile | pending | — |
| 168 | is not set if neither config nor packagefile are set | pending | — |
| 189 | is set if config is set | pending | — |
| 211 | skips deps with empty names | ported | [`crates/renovate-core/src/workers/repository/process/fetch.rs:219`](../../../../../../../crates/renovate-core/src/workers/repository/process/fetch.rs#L219) |
| 238 | skips internal deps by default | ported | [`crates/renovate-core/src/workers/repository/process/fetch.rs:271`](../../../../../../../crates/renovate-core/src/workers/repository/process/fetch.rs#L271) |
| 261 | fetch updates for internal deps if updateinternaldeps is true | pending | — |
| 283 | throws lookup errors for onboarded repos | ported | [`crates/renovate-core/src/workers/repository/process/fetch.rs:332`](../../../../../../../crates/renovate-core/src/workers/repository/process/fetch.rs#L332) |
| 300 | throws lookup errors for not onboarded repos | ported | [`crates/renovate-core/src/workers/repository/process/fetch.rs:358`](../../../../../../../crates/renovate-core/src/workers/repository/process/fetch.rs#L358) |
| 317 | produces external host warnings for not onboarded repos | ported | [`crates/renovate-core/src/workers/repository/process/fetch.rs:297`](../../../../../../../crates/renovate-core/src/workers/repository/process/fetch.rs#L297) |

