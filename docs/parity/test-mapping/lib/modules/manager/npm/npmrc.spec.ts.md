# `lib/modules/manager/npm/npmrc.spec.ts`

[← `manager/npm`](../../../../_by-module/manager/npm.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 19 | returns undefined if no .npmrc exists and no config.npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8796`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8796) |
| 24 | uses config.npmrc if no .npmrc is found | ported | [`crates/renovate-core/src/extractors/npm.rs:8805`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8805) |
| 31 | finds and filters .npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8813`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8813) |
| 53 | uses config.npmrc if .npmrc does exist but npmrcmerge=false | ported | [`crates/renovate-core/src/extractors/npm.rs:8836`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8836) |
| 81 | uses config.npmrc if no .npmrc file is found | ported | [`crates/renovate-core/src/extractors/npm.rs:8850`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8850) |
| 98 | merges config.npmrc and repo .npmrc when npmrcmerge=true | ported | [`crates/renovate-core/src/extractors/npm.rs:8858`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8858) |
| 123 | does not add a newline between config.npmrc and repo .npmrc when npmrcmerge is true, if a newline already exists | ported | [`crates/renovate-core/src/extractors/npm.rs:8872`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8872) |
| 156 | finds and filters .npmrc with variables | ported | [`crates/renovate-core/src/extractors/npm.rs:8889`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8889) |
| 180 | keeps variables when exposeallenv is true | ported | [`crates/renovate-core/src/extractors/npm.rs:8907`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8907) |

