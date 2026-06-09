# `lib/modules/manager/npm/extract/post/locked-versions.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**16/21 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 57 | uses yarn.lock with yarn v1.22.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7545`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7545) |
| 94 | uses yarn.lock with yarn v2.1.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7975`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7975) |
| 141 | uses yarn.lock with yarn v2.2.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:8012`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8012) |
| 188 | uses yarn.lock with yarn v3.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7988`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7988) |
| 227 | uses yarn.lock but doesn't override extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7733`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7733) |
| 267 | uses package-lock.json with npm v6.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7581`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7581) |
| 298 | uses locked version corresponding to workspace | ported | [`crates/renovate-core/src/extractors/npm.rs:7831`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7831) |
| 348 | does not set locked versions for engines, packagemanager, and volta deps | ported | [`crates/renovate-core/src/extractors/npm.rs:7617`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7617) |
| 457 | does nothing if managerdata is not present | ported | [`crates/renovate-core/src/extractors/npm.rs:7303`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7303) |
| 485 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7694`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7694) |
| 522 | augments v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7333`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7333) |
| 559 | skips augmenting v2 lock file constraint | ported | [`crates/renovate-core/src/extractors/npm.rs:7386`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7386) |
| 596 | appends <7 to npm extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7439`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7439) |
| 641 | skips appending <7 to npm extractedconstraints | ported | [`crates/renovate-core/src/extractors/npm.rs:7492`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7492) |
| 687 | uses pnpm-lock | pending | — |
| 748 | uses pnpm-lock for pnpm.catalog deptype | pending | — |
| 808 | uses pnpm-lock in subfolder | pending | — |
| 869 | uses pnpm-lock with workspaces | pending | — |
| 947 | should log warning if unsupported lockfileversion is found | ported | [`crates/renovate-core/src/extractors/npm.rs:7938`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7938) |
| 978 | uses package-lock.json with npm v9.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7925`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7925) |
| 1019 | uses package-lock.json with npm v7.0.0 | ported | [`crates/renovate-core/src/extractors/npm.rs:7694`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L7694) |

