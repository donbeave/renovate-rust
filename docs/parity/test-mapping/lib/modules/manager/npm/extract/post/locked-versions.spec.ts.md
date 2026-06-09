# `lib/modules/manager/npm/extract/post/locked-versions.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**16/21 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 57 | uses yarn.lock with yarn v1.22.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7553`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7553) |
| 94 | uses yarn.lock with yarn v2.1.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7983`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7983) |
| 141 | uses yarn.lock with yarn v2.2.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:8020`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8020) |
| 188 | uses yarn.lock with yarn v3.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7996`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7996) |
| 227 | uses yarn.lock but doesn't override extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7741`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7741) |
| 267 | uses package-lock.json with npm v6.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7589`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7589) |
| 298 | uses locked version corresponding to workspace | ported | [`crates/renovate-core/src/extractors/npm.rs:7839`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7839) |
| 348 | does not set locked versions for engines, packagemanager, and volta deps | ported | [`crates/renovate-core/src/extractors/npm.rs:7625`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7625) |
| 457 | does nothing if managerdata is not present | ported | [`crates/renovate-core/src/extractors/npm.rs:7311`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7311) |
| 485 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7702`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7702) |
| 522 | augments v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7341`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7341) |
| 559 | skips augmenting v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7394`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7394) |
| 596 | appends <7 to npm extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7447`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7447) |
| 641 | skips appending <7 to npm extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7500`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7500) |
| 687 | uses pnpm-lock | pending | — |
| 748 | uses pnpm-lock for pnpm.catalog deptype | pending | — |
| 808 | uses pnpm-lock in subfolder | pending | — |
| 869 | uses pnpm-lock with workspaces | pending | — |
| 947 | should log warning if unsupported lockfileversion is found | ported | [`crates/renovate-core/src/extractors/npm.rs:7946`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7946) |
| 978 | uses package-lock.json with npm v9.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7933`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7933) |
| 1019 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7702`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7702) |

