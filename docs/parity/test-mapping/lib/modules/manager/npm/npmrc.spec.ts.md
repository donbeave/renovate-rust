# `lib/modules/manager/npm/npmrc.spec.ts`

[← `manager/npm`](../../../../_by-module/manager/npm.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 19 | returns undefined if no .npmrc exists and no config.npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8798`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8798) |
| 24 | uses config.npmrc if no .npmrc is found | ported | [`crates/renovate-core/src/extractors/npm.rs:8807`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8807) |
| 31 | finds and filters .npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8815`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8815) |
| 53 | uses config.npmrc if .npmrc does exist but npmrcmerge=false | ported | [`crates/renovate-core/src/extractors/npm.rs:8838`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8838) |
| 81 | uses config.npmrc if no .npmrc file is found | ported | [`crates/renovate-core/src/extractors/npm.rs:8852`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8852) |
| 98 | merges config.npmrc and repo .npmrc when npmrcmerge=true | ported | [`crates/renovate-core/src/extractors/npm.rs:8860`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8860) |
| 123 | does not add a newline between config.npmrc and repo .npmrc when npmrcmerge is true, if a newline already exists | ported | [`crates/renovate-core/src/extractors/npm.rs:8874`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8874) |
| 156 | finds and filters .npmrc with variables | ported | [`crates/renovate-core/src/extractors/npm.rs:8891`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8891) |
| 180 | keeps variables when exposeallenv is true | ported | [`crates/renovate-core/src/extractors/npm.rs:8909`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8909) |

