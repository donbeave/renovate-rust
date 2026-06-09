# `lib/modules/manager/npm/extract/post/locked-versions.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**16/21 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 57 | uses yarn.lock with yarn v1.22.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7544`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7544) |
| 94 | uses yarn.lock with yarn v2.1.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7974`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7974) |
| 141 | uses yarn.lock with yarn v2.2.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:8011`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8011) |
| 188 | uses yarn.lock with yarn v3.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7987`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7987) |
| 227 | uses yarn.lock but doesn't override extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7732`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7732) |
| 267 | uses package-lock.json with npm v6.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7580`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7580) |
| 298 | uses locked version corresponding to workspace | ported | [`crates/renovate-core/src/extractors/npm.rs:7830`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7830) |
| 348 | does not set locked versions for engines, packagemanager, and volta deps | ported | [`crates/renovate-core/src/extractors/npm.rs:7616`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7616) |
| 457 | does nothing if managerdata is not present | ported | [`crates/renovate-core/src/extractors/npm.rs:7302`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7302) |
| 485 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7693`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7693) |
| 522 | augments v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7332`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7332) |
| 559 | skips augmenting v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7385`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7385) |
| 596 | appends <7 to npm extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7438`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7438) |
| 641 | skips appending <7 to npm extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7491`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7491) |
| 687 | uses pnpm-lock | pending | — |
| 748 | uses pnpm-lock for pnpm.catalog deptype | pending | — |
| 808 | uses pnpm-lock in subfolder | pending | — |
| 869 | uses pnpm-lock with workspaces | pending | — |
| 947 | should log warning if unsupported lockfileversion is found | ported | [`crates/renovate-core/src/extractors/npm.rs:7937`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7937) |
| 978 | uses package-lock.json with npm v9.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7924`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7924) |
| 1019 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7693`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7693) |

