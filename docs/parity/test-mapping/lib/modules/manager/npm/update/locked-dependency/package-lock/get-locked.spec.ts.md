# `lib/modules/manager/npm/update/locked-dependency/package-lock/get-locked.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | handles error | ported | [`crates/renovate-core/src/extractors/npm.rs:5448`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5448) |
| 17 | returns empty if failed to parse | ported | [`crates/renovate-core/src/extractors/npm.rs:5460`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5460) |
| 21 | finds direct dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5472`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5472) |
| 32 | finds indirect dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5487`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5487) |
| 43 | finds any version | ported | [`crates/renovate-core/src/extractors/npm.rs:5496`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5496) |
| 49 | finds bundled dependency | ported | [`crates/renovate-core/src/extractors/npm.rs:5504`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5504) |

