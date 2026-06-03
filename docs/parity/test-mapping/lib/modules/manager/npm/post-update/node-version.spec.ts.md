# `lib/modules/manager/npm/post-update/node-version.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**11/11 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 18 | returns from user constraints | ported | [`crates/renovate-core/src/extractors/npm_post_update/node_version.rs:112`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/node_version.rs#L112) |
| 29 | returns .node-version value | ported | [`crates/renovate-core/src/extractors/npm_post_update/node_version.rs:131`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/node_version.rs#L131) |
| 41 | returns .nvmrc value | ported | [`crates/renovate-core/src/extractors/npm_post_update/node_version.rs:121`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/node_version.rs#L121) |
| 52 | ignores unusable ranges in dotfiles | ported | [`crates/renovate-core/src/extractors/npm_post_update/node_version.rs:221`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/node_version.rs#L221) |
| 64 | returns from package.json | ported | [`crates/renovate-core/src/extractors/npm_post_update/node_version.rs:140`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/node_version.rs#L140) |
| 74 | returns from package.json volta | ported | [`crates/renovate-core/src/extractors/npm_post_update/node_version.rs:198`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/node_version.rs#L198) |
| 84 | prefers volta over engines | ported | [`crates/renovate-core/src/extractors/npm_post_update/node_version.rs:208`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/node_version.rs#L208) |
| 101 | returns version | ported | [`crates/renovate-core/src/extractors/npm.rs:4886`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4886) |
| 107 | returns undefined | ported | [`crates/renovate-core/src/extractors/npm.rs:4893`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L4893) |
| 113 | returns getnodeupdate | ported | [`crates/renovate-core/src/extractors/npm_post_update/node_version.rs:185`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/node_version.rs#L185) |
| 127 | returns getnodeconstraint | ported | [`crates/renovate-core/src/extractors/npm_post_update/node_version.rs:232`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/node_version.rs#L232) |

