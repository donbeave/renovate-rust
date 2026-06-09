# `lib/modules/manager/npm/update/locked-dependency/package-lock/get-locked.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | handles error | ported | [`crates/renovate-core/src/extractors/npm.rs:5447`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5447) |
| 17 | returns empty if failed to parse | ported | [`crates/renovate-core/src/extractors/npm.rs:5459`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5459) |
| 21 | finds direct dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5471`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5471) |
| 32 | finds indirect dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5486`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5486) |
| 43 | finds any version | ported | [`crates/renovate-core/src/extractors/npm.rs:5495`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5495) |
| 49 | finds bundled dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5503`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5503) |

