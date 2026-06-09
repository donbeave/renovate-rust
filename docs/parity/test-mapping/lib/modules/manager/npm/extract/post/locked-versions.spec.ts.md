# `lib/modules/manager/npm/extract/post/locked-versions.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**14/21 in-scope tests ported** (7 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 57 | uses yarn.lock with yarn v1.22.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7420`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7420) |
| 94 | uses yarn.lock with yarn v2.1.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7850`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7850) |
| 141 | uses yarn.lock with yarn v2.2.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7887`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7887) |
| 188 | uses yarn.lock with yarn v3.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7863`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7863) |
| 227 | uses yarn.lock but doesn't override extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7608`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7608) |
| 267 | uses package-lock.json with npm v6.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7456`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7456) |
| 298 | uses locked version corresponding to workspace | ported | [`crates/renovate-core/src/extractors/npm.rs:7706`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7706) |
| 348 | does not set locked versions for engines, packagemanager, and volta deps | ported | [`crates/renovate-core/src/extractors/npm.rs:7492`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7492) |
| 457 | does nothing if managerdata is not present | ported | [`crates/renovate-core/src/extractors/npm.rs:7284`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7284) |
| 485 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7569`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7569) |
| 522 | augments v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7314`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7314) |
| 559 | skips augmenting v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7367`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7367) |
| 596 | appends <7 to npm extractedconstraints | pending | — |
| 641 | skips appending <7 to npm extractedconstraints | pending | — |
| 687 | uses pnpm-lock | pending | — |
| 748 | uses pnpm-lock for pnpm.catalog deptype | pending | — |
| 808 | uses pnpm-lock in subfolder | pending | — |
| 869 | uses pnpm-lock with workspaces | pending | — |
| 947 | should log warning if unsupported lockfileversion is found | ported | [`crates/renovate-core/src/extractors/npm.rs:7813`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7813) |
| 978 | uses package-lock.json with npm v9.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7800`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7800) |
| 1019 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7569`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7569) |

