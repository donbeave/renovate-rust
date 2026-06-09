# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns same if yarn 2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5053`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5053) |
| 21 | replaces without dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5060`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5060) |
| 46 | replaces with dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5084`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5084) |
| 71 | replaces constraint too | ported | [`crates/renovate-core/src/extractors/npm.rs:5105`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5105) |
| 99 | handles escaped constraints | ported | [`crates/renovate-core/src/extractors/npm.rs:5121`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5121) |
| 124 | handles quoted | ported | [`crates/renovate-core/src/extractors/npm.rs:5177`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5177) |

