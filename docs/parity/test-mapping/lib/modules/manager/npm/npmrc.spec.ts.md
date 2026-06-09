# `lib/modules/manager/npm/npmrc.spec.ts`

[← `manager/npm`](../../../../_by-module/manager/npm.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 19 | returns undefined if no .npmrc exists and no config.npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8673`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8673) |
| 24 | uses config.npmrc if no .npmrc is found | ported | [`crates/renovate-core/src/extractors/npm.rs:8682`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8682) |
| 31 | finds and filters .npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8690`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8690) |
| 53 | uses config.npmrc if .npmrc does exist but npmrcmerge=false | ported | [`crates/renovate-core/src/extractors/npm.rs:8713`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8713) |
| 81 | uses config.npmrc if no .npmrc file is found | ported | [`crates/renovate-core/src/extractors/npm.rs:8727`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8727) |
| 98 | merges config.npmrc and repo .npmrc when npmrcmerge=true | ported | [`crates/renovate-core/src/extractors/npm.rs:8735`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8735) |
| 123 | does not add a newline between config.npmrc and repo .npmrc when npmrcmerge is true, if a newline already exists | ported | [`crates/renovate-core/src/extractors/npm.rs:8749`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8749) |
| 156 | finds and filters .npmrc with variables | ported | [`crates/renovate-core/src/extractors/npm.rs:8766`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8766) |
| 180 | keeps variables when exposeallenv is true | ported | [`crates/renovate-core/src/extractors/npm.rs:8784`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8784) |

