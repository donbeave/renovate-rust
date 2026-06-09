# `lib/modules/manager/npm/extract/post/locked-versions.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**16/21 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 57 | uses yarn.lock with yarn v1.22.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7547`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7547) |
| 94 | uses yarn.lock with yarn v2.1.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7977`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7977) |
| 141 | uses yarn.lock with yarn v2.2.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:8014`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8014) |
| 188 | uses yarn.lock with yarn v3.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7990`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7990) |
| 227 | uses yarn.lock but doesn't override extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7735`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7735) |
| 267 | uses package-lock.json with npm v6.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7583`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7583) |
| 298 | uses locked version corresponding to workspace | ported | [`crates/renovate-core/src/extractors/npm.rs:7833`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7833) |
| 348 | does not set locked versions for engines, packagemanager, and volta deps | ported | [`crates/renovate-core/src/extractors/npm.rs:7619`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7619) |
| 457 | does nothing if managerdata is not present | ported | [`crates/renovate-core/src/extractors/npm.rs:7305`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7305) |
| 485 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7696`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7696) |
| 522 | augments v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7335`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7335) |
| 559 | skips augmenting v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7388`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7388) |
| 596 | appends <7 to npm extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7441`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7441) |
| 641 | skips appending <7 to npm extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7494`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7494) |
| 687 | uses pnpm-lock | pending | — |
| 748 | uses pnpm-lock for pnpm.catalog deptype | pending | — |
| 808 | uses pnpm-lock in subfolder | pending | — |
| 869 | uses pnpm-lock with workspaces | pending | — |
| 947 | should log warning if unsupported lockfileversion is found | ported | [`crates/renovate-core/src/extractors/npm.rs:7940`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7940) |
| 978 | uses package-lock.json with npm v9.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7927`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7927) |
| 1019 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7696`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7696) |

