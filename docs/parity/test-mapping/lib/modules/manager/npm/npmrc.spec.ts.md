# `lib/modules/manager/npm/npmrc.spec.ts`

[← `manager/npm`](../../../../_by-module/manager/npm.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 19 | returns undefined if no .npmrc exists and no config.npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8795`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8795) |
| 24 | uses config.npmrc if no .npmrc is found | ported | [`crates/renovate-core/src/extractors/npm.rs:8804`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8804) |
| 31 | finds and filters .npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8812`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8812) |
| 53 | uses config.npmrc if .npmrc does exist but npmrcmerge=false | ported | [`crates/renovate-core/src/extractors/npm.rs:8835`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8835) |
| 81 | uses config.npmrc if no .npmrc file is found | ported | [`crates/renovate-core/src/extractors/npm.rs:8849`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8849) |
| 98 | merges config.npmrc and repo .npmrc when npmrcmerge=true | ported | [`crates/renovate-core/src/extractors/npm.rs:8857`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8857) |
| 123 | does not add a newline between config.npmrc and repo .npmrc when npmrcmerge is true, if a newline already exists | ported | [`crates/renovate-core/src/extractors/npm.rs:8871`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8871) |
| 156 | finds and filters .npmrc with variables | ported | [`crates/renovate-core/src/extractors/npm.rs:8888`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8888) |
| 180 | keeps variables when exposeallenv is true | ported | [`crates/renovate-core/src/extractors/npm.rs:8906`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8906) |

