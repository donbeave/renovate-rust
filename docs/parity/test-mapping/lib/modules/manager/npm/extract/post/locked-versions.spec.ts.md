# `lib/modules/manager/npm/extract/post/locked-versions.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**12/21 ported** (9 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 57 | uses yarn.lock with yarn v1.22.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7287`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7287) |
| 94 | uses yarn.lock with yarn v2.1.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7717`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7717) |
| 141 | uses yarn.lock with yarn v2.2.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7754`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7754) |
| 188 | uses yarn.lock with yarn v3.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7730`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7730) |
| 227 | uses yarn.lock but doesn't override extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7475`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7475) |
| 267 | uses package-lock.json with npm v6.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7323`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7323) |
| 298 | uses locked version corresponding to workspace | ported | [`crates/renovate-core/src/extractors/npm.rs:7573`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7573) |
| 348 | does not set locked versions for engines, packagemanager, and volta deps | ported | [`crates/renovate-core/src/extractors/npm.rs:7359`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7359) |
| 457 | does nothing if managerdata is not present | ported | [`crates/renovate-core/src/extractors/npm.rs:7257`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7257) |
| 485 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7436`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7436) |
| 522 | augments v2 lock file constraint | pending | — |
| 559 | skips augmenting v2 lock file constraint | pending | — |
| 596 | appends <7 to npm extractedconstraints | pending | — |
| 641 | skips appending <7 to npm extractedconstraints | pending | — |
| 687 | uses pnpm-lock | pending | — |
| 748 | uses pnpm-lock for pnpm.catalog deptype | pending | — |
| 808 | uses pnpm-lock in subfolder | pending | — |
| 869 | uses pnpm-lock with workspaces | pending | — |
| 947 | should log warning if unsupported lockfileversion is found | ported | [`crates/renovate-core/src/extractors/npm.rs:7680`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7680) |
| 978 | uses package-lock.json with npm v9.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7667`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7667) |
| 1019 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7436`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7436) |

