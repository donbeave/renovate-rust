# `lib/modules/manager/npm/post-update/node-version.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**11/11 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 18 | returns from user constraints | ported | `crates/renovate-core/src/extractors/npm_post_update/node_version.rs:112` |
| 29 | returns .node-version value | ported | `crates/renovate-core/src/extractors/npm_post_update/node_version.rs:131` |
| 41 | returns .nvmrc value | ported | `crates/renovate-core/src/extractors/npm_post_update/node_version.rs:121` |
| 52 | ignores unusable ranges in dotfiles | ported | `crates/renovate-core/src/extractors/npm_post_update/node_version.rs:221` |
| 64 | returns from package.json | ported | `crates/renovate-core/src/extractors/npm_post_update/node_version.rs:140` |
| 74 | returns from package.json volta | ported | `crates/renovate-core/src/extractors/npm_post_update/node_version.rs:198` |
| 84 | prefers volta over engines | ported | `crates/renovate-core/src/extractors/npm_post_update/node_version.rs:208` |
| 101 | returns version | ported | `crates/renovate-core/src/extractors/npm.rs:4886` |
| 107 | returns undefined | ported | `crates/renovate-core/src/extractors/npm.rs:4893` |
| 113 | returns getnodeupdate | ported | `crates/renovate-core/src/extractors/npm_post_update/node_version.rs:185` |
| 127 | returns getnodeconstraint | ported | `crates/renovate-core/src/extractors/npm_post_update/node_version.rs:232` |

