# `lib/modules/manager/npm/update/locked-dependency/package-lock/get-locked.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | handles error | ported | [`crates/renovate-core/src/extractors/npm.rs:5454`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5454) |
| 17 | returns empty if failed to parse | ported | [`crates/renovate-core/src/extractors/npm.rs:5466`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5466) |
| 21 | finds direct dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5478`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5478) |
| 32 | finds indirect dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5493`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5493) |
| 43 | finds any version | ported | [`crates/renovate-core/src/extractors/npm.rs:5502`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5502) |
| 49 | finds bundled dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5510`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5510) |

