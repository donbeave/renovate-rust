# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns same if yarn 2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5052`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5052) |
| 21 | replaces without dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5059`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5059) |
| 46 | replaces with dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5083`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5083) |
| 71 | replaces constraint too | ported | [`crates/renovate-core/src/extractors/npm.rs:5104`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5104) |
| 99 | handles escaped constraints | ported | [`crates/renovate-core/src/extractors/npm.rs:5120`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5120) |
| 124 | handles quoted | ported | [`crates/renovate-core/src/extractors/npm.rs:5176`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5176) |

