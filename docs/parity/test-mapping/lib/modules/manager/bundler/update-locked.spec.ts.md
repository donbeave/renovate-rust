# `lib/modules/manager/bundler/update-locked.spec.ts`

[← `manager/bundler`](../../../../_by-module/manager/bundler.md) · [all modules](../../../../README.md)

**5/5 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 9 | detects already updated | ported | [`crates/renovate-core/src/extractors/bundler.rs:967`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L967) |
| 21 | returns unsupported for empty lockfile | ported | [`crates/renovate-core/src/extractors/bundler.rs:975`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L975) |
| 32 | returns unsupported for empty depname | ported | [`crates/renovate-core/src/extractors/bundler.rs:982`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L982) |
| 44 | returns unsupported | ported | [`crates/renovate-core/src/extractors/bundler.rs:989`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L989) |
| 56 | returns update-failed in case of errors | ported | [`crates/renovate-core/src/extractors/bundler.rs:997`](../../../../../../../crates/renovate-core/src/extractors/bundler.rs#L997) |

