# `lib/modules/manager/npm/extract/post/locked-versions.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**16/21 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 57 | uses yarn.lock with yarn v1.22.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7546`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7546) |
| 94 | uses yarn.lock with yarn v2.1.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7976`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7976) |
| 141 | uses yarn.lock with yarn v2.2.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:8013`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8013) |
| 188 | uses yarn.lock with yarn v3.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7989`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7989) |
| 227 | uses yarn.lock but doesn't override extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7734`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7734) |
| 267 | uses package-lock.json with npm v6.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7582`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7582) |
| 298 | uses locked version corresponding to workspace | ported | [`crates/renovate-core/src/extractors/npm.rs:7832`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7832) |
| 348 | does not set locked versions for engines, packagemanager, and volta deps | ported | [`crates/renovate-core/src/extractors/npm.rs:7618`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7618) |
| 457 | does nothing if managerdata is not present | ported | [`crates/renovate-core/src/extractors/npm.rs:7304`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7304) |
| 485 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7695`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7695) |
| 522 | augments v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7334`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7334) |
| 559 | skips augmenting v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7387`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7387) |
| 596 | appends <7 to npm extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7440`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7440) |
| 641 | skips appending <7 to npm extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7493`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7493) |
| 687 | uses pnpm-lock | pending | — |
| 748 | uses pnpm-lock for pnpm.catalog deptype | pending | — |
| 808 | uses pnpm-lock in subfolder | pending | — |
| 869 | uses pnpm-lock with workspaces | pending | — |
| 947 | should log warning if unsupported lockfileversion is found | ported | [`crates/renovate-core/src/extractors/npm.rs:7939`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7939) |
| 978 | uses package-lock.json with npm v9.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7926`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7926) |
| 1019 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7695`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7695) |

