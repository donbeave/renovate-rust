# `lib/modules/manager/npm/extract/post/locked-versions.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**16/21 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 57 | uses yarn.lock with yarn v1.22.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7543`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7543) |
| 94 | uses yarn.lock with yarn v2.1.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7973`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7973) |
| 141 | uses yarn.lock with yarn v2.2.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:8010`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8010) |
| 188 | uses yarn.lock with yarn v3.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7986`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7986) |
| 227 | uses yarn.lock but doesn't override extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7731`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7731) |
| 267 | uses package-lock.json with npm v6.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7579`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7579) |
| 298 | uses locked version corresponding to workspace | ported | [`crates/renovate-core/src/extractors/npm.rs:7829`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7829) |
| 348 | does not set locked versions for engines, packagemanager, and volta deps | ported | [`crates/renovate-core/src/extractors/npm.rs:7615`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7615) |
| 457 | does nothing if managerdata is not present | ported | [`crates/renovate-core/src/extractors/npm.rs:7301`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7301) |
| 485 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7692`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7692) |
| 522 | augments v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7331`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7331) |
| 559 | skips augmenting v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7384`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7384) |
| 596 | appends <7 to npm extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7437`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7437) |
| 641 | skips appending <7 to npm extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7490`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7490) |
| 687 | uses pnpm-lock | pending | — |
| 748 | uses pnpm-lock for pnpm.catalog deptype | pending | — |
| 808 | uses pnpm-lock in subfolder | pending | — |
| 869 | uses pnpm-lock with workspaces | pending | — |
| 947 | should log warning if unsupported lockfileversion is found | ported | [`crates/renovate-core/src/extractors/npm.rs:7936`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7936) |
| 978 | uses package-lock.json with npm v9.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7923`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7923) |
| 1019 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7692`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7692) |

