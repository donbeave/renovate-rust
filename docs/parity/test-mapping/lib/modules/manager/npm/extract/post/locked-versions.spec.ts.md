# `lib/modules/manager/npm/extract/post/locked-versions.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**16/21 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 57 | uses yarn.lock with yarn v1.22.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7542`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7542) |
| 94 | uses yarn.lock with yarn v2.1.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7972`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7972) |
| 141 | uses yarn.lock with yarn v2.2.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:8009`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8009) |
| 188 | uses yarn.lock with yarn v3.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7985`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7985) |
| 227 | uses yarn.lock but doesn't override extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7730`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7730) |
| 267 | uses package-lock.json with npm v6.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7578`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7578) |
| 298 | uses locked version corresponding to workspace | ported | [`crates/renovate-core/src/extractors/npm.rs:7828`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7828) |
| 348 | does not set locked versions for engines, packagemanager, and volta deps | ported | [`crates/renovate-core/src/extractors/npm.rs:7614`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7614) |
| 457 | does nothing if managerdata is not present | ported | [`crates/renovate-core/src/extractors/npm.rs:7300`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7300) |
| 485 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7691`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7691) |
| 522 | augments v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7330`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7330) |
| 559 | skips augmenting v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7383`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7383) |
| 596 | appends <7 to npm extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7436`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7436) |
| 641 | skips appending <7 to npm extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7489`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7489) |
| 687 | uses pnpm-lock | pending | — |
| 748 | uses pnpm-lock for pnpm.catalog deptype | pending | — |
| 808 | uses pnpm-lock in subfolder | pending | — |
| 869 | uses pnpm-lock with workspaces | pending | — |
| 947 | should log warning if unsupported lockfileversion is found | ported | [`crates/renovate-core/src/extractors/npm.rs:7935`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7935) |
| 978 | uses package-lock.json with npm v9.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7922`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7922) |
| 1019 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7691`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7691) |

