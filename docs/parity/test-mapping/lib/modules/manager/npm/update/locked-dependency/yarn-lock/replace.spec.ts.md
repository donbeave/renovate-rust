# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns same if yarn 2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5054`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5054) |
| 21 | replaces without dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5061`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5061) |
| 46 | replaces with dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5085`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5085) |
| 71 | replaces constraint too | ported | [`crates/renovate-core/src/extractors/npm.rs:5106`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5106) |
| 99 | handles escaped constraints | ported | [`crates/renovate-core/src/extractors/npm.rs:5122`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5122) |
| 124 | handles quoted | ported | [`crates/renovate-core/src/extractors/npm.rs:5178`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5178) |

