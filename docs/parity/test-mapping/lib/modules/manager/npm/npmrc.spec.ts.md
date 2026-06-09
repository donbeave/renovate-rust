# `lib/modules/manager/npm/npmrc.spec.ts`

[← `manager/npm`](../../../../_by-module/manager/npm.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 19 | returns undefined if no .npmrc exists and no config.npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8797`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8797) |
| 24 | uses config.npmrc if no .npmrc is found | ported | [`crates/renovate-core/src/extractors/npm.rs:8806`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8806) |
| 31 | finds and filters .npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8814`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8814) |
| 53 | uses config.npmrc if .npmrc does exist but npmrcmerge=false | ported | [`crates/renovate-core/src/extractors/npm.rs:8837`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8837) |
| 81 | uses config.npmrc if no .npmrc file is found | ported | [`crates/renovate-core/src/extractors/npm.rs:8851`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8851) |
| 98 | merges config.npmrc and repo .npmrc when npmrcmerge=true | ported | [`crates/renovate-core/src/extractors/npm.rs:8859`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8859) |
| 123 | does not add a newline between config.npmrc and repo .npmrc when npmrcmerge is true, if a newline already exists | ported | [`crates/renovate-core/src/extractors/npm.rs:8873`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8873) |
| 156 | finds and filters .npmrc with variables | ported | [`crates/renovate-core/src/extractors/npm.rs:8890`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8890) |
| 180 | keeps variables when exposeallenv is true | ported | [`crates/renovate-core/src/extractors/npm.rs:8908`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8908) |

