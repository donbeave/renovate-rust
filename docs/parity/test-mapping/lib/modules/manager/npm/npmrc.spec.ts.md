# `lib/modules/manager/npm/npmrc.spec.ts`

[← `manager/npm`](../../../../_by-module/manager/npm.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 19 | returns undefined if no .npmrc exists and no config.npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8794`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8794) |
| 24 | uses config.npmrc if no .npmrc is found | ported | [`crates/renovate-core/src/extractors/npm.rs:8803`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8803) |
| 31 | finds and filters .npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8811`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8811) |
| 53 | uses config.npmrc if .npmrc does exist but npmrcmerge=false | ported | [`crates/renovate-core/src/extractors/npm.rs:8834`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8834) |
| 81 | uses config.npmrc if no .npmrc file is found | ported | [`crates/renovate-core/src/extractors/npm.rs:8848`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8848) |
| 98 | merges config.npmrc and repo .npmrc when npmrcmerge=true | ported | [`crates/renovate-core/src/extractors/npm.rs:8856`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8856) |
| 123 | does not add a newline between config.npmrc and repo .npmrc when npmrcmerge is true, if a newline already exists | ported | [`crates/renovate-core/src/extractors/npm.rs:8870`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8870) |
| 156 | finds and filters .npmrc with variables | ported | [`crates/renovate-core/src/extractors/npm.rs:8887`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8887) |
| 180 | keeps variables when exposeallenv is true | ported | [`crates/renovate-core/src/extractors/npm.rs:8905`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8905) |

