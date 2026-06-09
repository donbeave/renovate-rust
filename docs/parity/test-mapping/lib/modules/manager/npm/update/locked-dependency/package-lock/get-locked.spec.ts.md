# `lib/modules/manager/npm/update/locked-dependency/package-lock/get-locked.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | handles error | ported | [`crates/renovate-core/src/extractors/npm.rs:5453`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5453) |
| 17 | returns empty if failed to parse | ported | [`crates/renovate-core/src/extractors/npm.rs:5465`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5465) |
| 21 | finds direct dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5477`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5477) |
| 32 | finds indirect dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5492`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5492) |
| 43 | finds any version | ported | [`crates/renovate-core/src/extractors/npm.rs:5501`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5501) |
| 49 | finds bundled dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5509`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5509) |

