# `lib/modules/manager/npm/update/locked-dependency/package-lock/get-locked.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | handles error | ported | [`crates/renovate-core/src/extractors/npm.rs:5459`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5459) |
| 17 | returns empty if failed to parse | ported | [`crates/renovate-core/src/extractors/npm.rs:5471`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5471) |
| 21 | finds direct dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5483`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5483) |
| 32 | finds indirect dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5498`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5498) |
| 43 | finds any version | ported | [`crates/renovate-core/src/extractors/npm.rs:5507`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5507) |
| 49 | finds bundled dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5515`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5515) |

