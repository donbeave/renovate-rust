# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns same if yarn 2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5058`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5058) |
| 21 | replaces without dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5065`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5065) |
| 46 | replaces with dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5089`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5089) |
| 71 | replaces constraint too | ported | [`crates/renovate-core/src/extractors/npm.rs:5110`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5110) |
| 99 | handles escaped constraints | ported | [`crates/renovate-core/src/extractors/npm.rs:5126`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5126) |
| 124 | handles quoted | ported | [`crates/renovate-core/src/extractors/npm.rs:5182`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5182) |

