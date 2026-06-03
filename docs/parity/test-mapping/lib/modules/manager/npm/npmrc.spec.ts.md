# `lib/modules/manager/npm/npmrc.spec.ts`

[← `manager/npm`](../../../../_by-module/manager/npm.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 19 | returns undefined if no .npmrc exists and no config.npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8540`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8540) |
| 24 | uses config.npmrc if no .npmrc is found | ported | [`crates/renovate-core/src/extractors/npm.rs:8549`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8549) |
| 31 | finds and filters .npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8557`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8557) |
| 53 | uses config.npmrc if .npmrc does exist but npmrcmerge=false | ported | [`crates/renovate-core/src/extractors/npm.rs:8580`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8580) |
| 81 | uses config.npmrc if no .npmrc file is found | ported | [`crates/renovate-core/src/extractors/npm.rs:8594`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8594) |
| 98 | merges config.npmrc and repo .npmrc when npmrcmerge=true | ported | [`crates/renovate-core/src/extractors/npm.rs:8602`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8602) |
| 123 | does not add a newline between config.npmrc and repo .npmrc when npmrcmerge is true, if a newline already exists | ported | [`crates/renovate-core/src/extractors/npm.rs:8616`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8616) |
| 156 | finds and filters .npmrc with variables | ported | [`crates/renovate-core/src/extractors/npm.rs:8633`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8633) |
| 180 | keeps variables when exposeallenv is true | ported | [`crates/renovate-core/src/extractors/npm.rs:8651`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8651) |

