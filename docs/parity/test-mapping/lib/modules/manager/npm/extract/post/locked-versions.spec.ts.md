# `lib/modules/manager/npm/extract/post/locked-versions.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**16/21 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 57 | uses yarn.lock with yarn v1.22.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7541`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7541) |
| 94 | uses yarn.lock with yarn v2.1.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7971`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7971) |
| 141 | uses yarn.lock with yarn v2.2.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:8008`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8008) |
| 188 | uses yarn.lock with yarn v3.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7984`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7984) |
| 227 | uses yarn.lock but doesn't override extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7729`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7729) |
| 267 | uses package-lock.json with npm v6.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7577`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7577) |
| 298 | uses locked version corresponding to workspace | ported | [`crates/renovate-core/src/extractors/npm.rs:7827`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7827) |
| 348 | does not set locked versions for engines, packagemanager, and volta deps | ported | [`crates/renovate-core/src/extractors/npm.rs:7613`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7613) |
| 457 | does nothing if managerdata is not present | ported | [`crates/renovate-core/src/extractors/npm.rs:7299`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7299) |
| 485 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7690`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7690) |
| 522 | augments v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7329`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7329) |
| 559 | skips augmenting v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7382`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7382) |
| 596 | appends <7 to npm extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7435`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7435) |
| 641 | skips appending <7 to npm extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7488`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7488) |
| 687 | uses pnpm-lock | pending | — |
| 748 | uses pnpm-lock for pnpm.catalog deptype | pending | — |
| 808 | uses pnpm-lock in subfolder | pending | — |
| 869 | uses pnpm-lock with workspaces | pending | — |
| 947 | should log warning if unsupported lockfileversion is found | ported | [`crates/renovate-core/src/extractors/npm.rs:7934`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7934) |
| 978 | uses package-lock.json with npm v9.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7921`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7921) |
| 1019 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7690`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7690) |

