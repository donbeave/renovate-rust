# `lib/modules/manager/npm/update/locked-dependency/package-lock/get-locked.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | handles error | ported | [`crates/renovate-core/src/extractors/npm.rs:5449`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5449) |
| 17 | returns empty if failed to parse | ported | [`crates/renovate-core/src/extractors/npm.rs:5461`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5461) |
| 21 | finds direct dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5473`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5473) |
| 32 | finds indirect dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5488`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5488) |
| 43 | finds any version | ported | [`crates/renovate-core/src/extractors/npm.rs:5497`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5497) |
| 49 | finds bundled dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5505`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5505) |

