# `lib/modules/manager/npm/update/locked-dependency/package-lock/get-locked.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | handles error | ported | [`crates/renovate-core/src/extractors/npm.rs:5455`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5455) |
| 17 | returns empty if failed to parse | ported | [`crates/renovate-core/src/extractors/npm.rs:5467`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5467) |
| 21 | finds direct dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5479`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5479) |
| 32 | finds indirect dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5494`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5494) |
| 43 | finds any version | ported | [`crates/renovate-core/src/extractors/npm.rs:5503`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5503) |
| 49 | finds bundled dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5511`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5511) |

