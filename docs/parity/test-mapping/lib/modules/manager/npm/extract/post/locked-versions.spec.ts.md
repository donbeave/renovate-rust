# `lib/modules/manager/npm/extract/post/locked-versions.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**16/21 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 57 | uses yarn.lock with yarn v1.22.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7548`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7548) |
| 94 | uses yarn.lock with yarn v2.1.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7978`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7978) |
| 141 | uses yarn.lock with yarn v2.2.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:8015`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8015) |
| 188 | uses yarn.lock with yarn v3.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7991`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7991) |
| 227 | uses yarn.lock but doesn't override extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7736`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7736) |
| 267 | uses package-lock.json with npm v6.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7584`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7584) |
| 298 | uses locked version corresponding to workspace | ported | [`crates/renovate-core/src/extractors/npm.rs:7834`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7834) |
| 348 | does not set locked versions for engines, packagemanager, and volta deps | ported | [`crates/renovate-core/src/extractors/npm.rs:7620`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7620) |
| 457 | does nothing if managerdata is not present | ported | [`crates/renovate-core/src/extractors/npm.rs:7306`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7306) |
| 485 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7697`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7697) |
| 522 | augments v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7336`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7336) |
| 559 | skips augmenting v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7389`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7389) |
| 596 | appends <7 to npm extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7442`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7442) |
| 641 | skips appending <7 to npm extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7495`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7495) |
| 687 | uses pnpm-lock | pending | — |
| 748 | uses pnpm-lock for pnpm.catalog deptype | pending | — |
| 808 | uses pnpm-lock in subfolder | pending | — |
| 869 | uses pnpm-lock with workspaces | pending | — |
| 947 | should log warning if unsupported lockfileversion is found | ported | [`crates/renovate-core/src/extractors/npm.rs:7941`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7941) |
| 978 | uses package-lock.json with npm v9.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7928`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7928) |
| 1019 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7697`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7697) |

