# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns same if yarn 2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5035`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5035) |
| 21 | replaces without dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5042`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5042) |
| 46 | replaces with dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5066`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5066) |
| 71 | replaces constraint too | ported | [`crates/renovate-core/src/extractors/npm.rs:5087`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5087) |
| 99 | handles escaped constraints | ported | [`crates/renovate-core/src/extractors/npm.rs:5103`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5103) |
| 124 | handles quoted | ported | [`crates/renovate-core/src/extractors/npm.rs:5159`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5159) |

