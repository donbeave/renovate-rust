# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns same if yarn 2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5057`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5057) |
| 21 | replaces without dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5064`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5064) |
| 46 | replaces with dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5088`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5088) |
| 71 | replaces constraint too | ported | [`crates/renovate-core/src/extractors/npm.rs:5109`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5109) |
| 99 | handles escaped constraints | ported | [`crates/renovate-core/src/extractors/npm.rs:5125`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5125) |
| 124 | handles quoted | ported | [`crates/renovate-core/src/extractors/npm.rs:5181`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5181) |

