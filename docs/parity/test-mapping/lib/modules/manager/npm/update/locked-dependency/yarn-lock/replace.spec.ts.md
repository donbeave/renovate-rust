# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns same if yarn 2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5055`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5055) |
| 21 | replaces without dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5062`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5062) |
| 46 | replaces with dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5086`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5086) |
| 71 | replaces constraint too | ported | [`crates/renovate-core/src/extractors/npm.rs:5107`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5107) |
| 99 | handles escaped constraints | ported | [`crates/renovate-core/src/extractors/npm.rs:5123`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5123) |
| 124 | handles quoted | ported | [`crates/renovate-core/src/extractors/npm.rs:5179`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5179) |

