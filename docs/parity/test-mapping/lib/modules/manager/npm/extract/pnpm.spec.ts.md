# `lib/modules/manager/npm/extract/pnpm.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**5/16 ported** (11 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 28 | detects errors in pnpm-workspace.yml file structure | pending | — |
| 46 | detects errors when opening pnpm-workspace.yml file | pending | — |
| 65 | detects missing pnpm-workspace.yaml | pending | — |
| 78 | detects missing pnpm-lock.yaml when pnpm-workspace.yaml was already found | pending | — |
| 114 | uses pnpm workspaces | pending | — |
| 203 | skips when pnpm shrinkwrap file has already been provided | pending | — |
| 220 | filters none matching packages | pending | — |
| 266 | returns empty if failed to parse | pending | — |
| 272 | extracts version from monorepo | pending | — |
| 279 | extracts version from normal repo | pending | — |
| 289 | extracts version from catalogs | pending | — |
| 341 | returns empty if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:3799`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3799) |
| 349 | handles empty catalog entries | ported | [`crates/renovate-core/src/extractors/npm.rs:3807`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3807) |
| 360 | parses valid pnpm-workspace.yaml file | ported | [`crates/renovate-core/src/extractors/npm.rs:3815`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3815) |
| 395 | parses overrides in pnpm-workspace.yaml file | ported | [`crates/renovate-core/src/extractors/npm.rs:3863`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3863) |
| 466 | finds relevant lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3899`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3899) |

