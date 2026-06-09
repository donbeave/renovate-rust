# `lib/modules/manager/npm/update/locked-dependency/package-lock/get-locked.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | handles error | ported | [`crates/renovate-core/src/extractors/npm.rs:5450`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5450) |
| 17 | returns empty if failed to parse | ported | [`crates/renovate-core/src/extractors/npm.rs:5462`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5462) |
| 21 | finds direct dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5474`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5474) |
| 32 | finds indirect dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5489`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5489) |
| 43 | finds any version | ported | [`crates/renovate-core/src/extractors/npm.rs:5498`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5498) |
| 49 | finds bundled dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5506`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5506) |

