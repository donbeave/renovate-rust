# `lib/modules/manager/npm/npmrc.spec.ts`

[← `manager/npm`](../../../../_by-module/manager/npm.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 19 | returns undefined if no .npmrc exists and no config.npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8800`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8800) |
| 24 | uses config.npmrc if no .npmrc is found | ported | [`crates/renovate-core/src/extractors/npm.rs:8809`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8809) |
| 31 | finds and filters .npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8817`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8817) |
| 53 | uses config.npmrc if .npmrc does exist but npmrcmerge=false | ported | [`crates/renovate-core/src/extractors/npm.rs:8840`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8840) |
| 81 | uses config.npmrc if no .npmrc file is found | ported | [`crates/renovate-core/src/extractors/npm.rs:8854`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8854) |
| 98 | merges config.npmrc and repo .npmrc when npmrcmerge=true | ported | [`crates/renovate-core/src/extractors/npm.rs:8862`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8862) |
| 123 | does not add a newline between config.npmrc and repo .npmrc when npmrcmerge is true, if a newline already exists | ported | [`crates/renovate-core/src/extractors/npm.rs:8876`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8876) |
| 156 | finds and filters .npmrc with variables | ported | [`crates/renovate-core/src/extractors/npm.rs:8893`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8893) |
| 180 | keeps variables when exposeallenv is true | ported | [`crates/renovate-core/src/extractors/npm.rs:8911`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8911) |

