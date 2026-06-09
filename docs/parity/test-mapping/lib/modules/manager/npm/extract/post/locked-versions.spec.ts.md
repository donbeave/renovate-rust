# `lib/modules/manager/npm/extract/post/locked-versions.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**16/21 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 57 | uses yarn.lock with yarn v1.22.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7549`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7549) |
| 94 | uses yarn.lock with yarn v2.1.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7979`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7979) |
| 141 | uses yarn.lock with yarn v2.2.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:8016`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8016) |
| 188 | uses yarn.lock with yarn v3.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7992`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7992) |
| 227 | uses yarn.lock but doesn't override extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7737`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7737) |
| 267 | uses package-lock.json with npm v6.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7585`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7585) |
| 298 | uses locked version corresponding to workspace | ported | [`crates/renovate-core/src/extractors/npm.rs:7835`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7835) |
| 348 | does not set locked versions for engines, packagemanager, and volta deps | ported | [`crates/renovate-core/src/extractors/npm.rs:7621`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7621) |
| 457 | does nothing if managerdata is not present | ported | [`crates/renovate-core/src/extractors/npm.rs:7307`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7307) |
| 485 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7698`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7698) |
| 522 | augments v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7337`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7337) |
| 559 | skips augmenting v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7390`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7390) |
| 596 | appends <7 to npm extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7443`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7443) |
| 641 | skips appending <7 to npm extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7496`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7496) |
| 687 | uses pnpm-lock | pending | — |
| 748 | uses pnpm-lock for pnpm.catalog deptype | pending | — |
| 808 | uses pnpm-lock in subfolder | pending | — |
| 869 | uses pnpm-lock with workspaces | pending | — |
| 947 | should log warning if unsupported lockfileversion is found | ported | [`crates/renovate-core/src/extractors/npm.rs:7942`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7942) |
| 978 | uses package-lock.json with npm v9.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7929`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7929) |
| 1019 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7698`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7698) |

