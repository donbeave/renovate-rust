# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns same if yarn 2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5062`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5062) |
| 21 | replaces without dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5069`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5069) |
| 46 | replaces with dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5093`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5093) |
| 71 | replaces constraint too | ported | [`crates/renovate-core/src/extractors/npm.rs:5114`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5114) |
| 99 | handles escaped constraints | ported | [`crates/renovate-core/src/extractors/npm.rs:5130`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5130) |
| 124 | handles quoted | ported | [`crates/renovate-core/src/extractors/npm.rs:5186`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5186) |

