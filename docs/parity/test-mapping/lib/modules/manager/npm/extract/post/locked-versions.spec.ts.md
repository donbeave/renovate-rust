# `lib/modules/manager/npm/extract/post/locked-versions.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**16/21 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 57 | uses yarn.lock with yarn v1.22.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7539`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7539) |
| 94 | uses yarn.lock with yarn v2.1.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7969`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7969) |
| 141 | uses yarn.lock with yarn v2.2.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:8006`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8006) |
| 188 | uses yarn.lock with yarn v3.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7982`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7982) |
| 227 | uses yarn.lock but doesn't override extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7727`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7727) |
| 267 | uses package-lock.json with npm v6.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7575`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7575) |
| 298 | uses locked version corresponding to workspace | ported | [`crates/renovate-core/src/extractors/npm.rs:7825`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7825) |
| 348 | does not set locked versions for engines, packagemanager, and volta deps | ported | [`crates/renovate-core/src/extractors/npm.rs:7611`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7611) |
| 457 | does nothing if managerdata is not present | ported | [`crates/renovate-core/src/extractors/npm.rs:7297`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7297) |
| 485 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7688`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7688) |
| 522 | augments v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7327`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7327) |
| 559 | skips augmenting v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7380`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7380) |
| 596 | appends <7 to npm extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7433`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7433) |
| 641 | skips appending <7 to npm extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7486`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7486) |
| 687 | uses pnpm-lock | pending | — |
| 748 | uses pnpm-lock for pnpm.catalog deptype | pending | — |
| 808 | uses pnpm-lock in subfolder | pending | — |
| 869 | uses pnpm-lock with workspaces | pending | — |
| 947 | should log warning if unsupported lockfileversion is found | ported | [`crates/renovate-core/src/extractors/npm.rs:7932`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7932) |
| 978 | uses package-lock.json with npm v9.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7919`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7919) |
| 1019 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7688`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7688) |

