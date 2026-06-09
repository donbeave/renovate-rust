# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns same if yarn 2+ | ported | [`crates/renovate-core/src/extractors/npm.rs:5056`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5056) |
| 21 | replaces without dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5063`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5063) |
| 46 | replaces with dependencies | ported | [`crates/renovate-core/src/extractors/npm.rs:5087`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5087) |
| 71 | replaces constraint too | ported | [`crates/renovate-core/src/extractors/npm.rs:5108`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5108) |
| 99 | handles escaped constraints | ported | [`crates/renovate-core/src/extractors/npm.rs:5124`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5124) |
| 124 | handles quoted | ported | [`crates/renovate-core/src/extractors/npm.rs:5180`](../../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L5180) |

