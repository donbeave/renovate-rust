# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns same if yarn 2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5050`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5050) |
| 21 | replaces without dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5057`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5057) |
| 46 | replaces with dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5081`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5081) |
| 71 | replaces constraint too | ported | [`crates/renovate-core/src/extractors/npm.rs:5102`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5102) |
| 99 | handles escaped constraints | ported | [`crates/renovate-core/src/extractors/npm.rs:5118`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5118) |
| 124 | handles quoted | ported | [`crates/renovate-core/src/extractors/npm.rs:5174`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5174) |

