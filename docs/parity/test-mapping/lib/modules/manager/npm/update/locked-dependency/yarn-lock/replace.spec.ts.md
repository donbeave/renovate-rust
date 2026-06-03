# `lib/modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 11 | returns same if yarn 2+ | ported | `crates/renovate-core/src/extractors/npm.rs:5008` |
| 21 | replaces without dependencies | ported | `crates/renovate-core/src/extractors/npm.rs:5015` |
| 46 | replaces with dependencies | ported | `crates/renovate-core/src/extractors/npm.rs:5039` |
| 71 | replaces constraint too | ported | `crates/renovate-core/src/extractors/npm.rs:5060` |
| 99 | handles escaped constraints | ported | `crates/renovate-core/src/extractors/npm.rs:5076` |
| 124 | handles quoted | ported | `crates/renovate-core/src/extractors/npm.rs:5132` |

