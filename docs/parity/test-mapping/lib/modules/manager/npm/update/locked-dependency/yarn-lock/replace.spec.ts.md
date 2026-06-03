# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns same if yarn 2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5008`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5008) |
| 21 | replaces without dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5015`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5015) |
| 46 | replaces with dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5039`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5039) |
| 71 | replaces constraint too | ported | [`crates/renovate-core/src/extractors/npm.rs:5060`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5060) |
| 99 | handles escaped constraints | ported | [`crates/renovate-core/src/extractors/npm.rs:5076`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5076) |
| 124 | handles quoted | ported | [`crates/renovate-core/src/extractors/npm.rs:5132`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5132) |

