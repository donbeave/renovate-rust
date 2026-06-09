# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns same if yarn 2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5048`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5048) |
| 21 | replaces without dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5055`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5055) |
| 46 | replaces with dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5079`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5079) |
| 71 | replaces constraint too | ported | [`crates/renovate-core/src/extractors/npm.rs:5100`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5100) |
| 99 | handles escaped constraints | ported | [`crates/renovate-core/src/extractors/npm.rs:5116`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5116) |
| 124 | handles quoted | ported | [`crates/renovate-core/src/extractors/npm.rs:5172`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5172) |

