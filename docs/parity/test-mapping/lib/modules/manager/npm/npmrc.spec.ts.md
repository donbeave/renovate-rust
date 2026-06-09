# `lib/modules/manager/npm/npmrc.spec.ts`

[← `manager/npm`](../../../../_by-module/manager/npm.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 19 | returns undefined if no .npmrc exists and no config.npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8799`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8799) |
| 24 | uses config.npmrc if no .npmrc is found | ported | [`crates/renovate-core/src/extractors/npm.rs:8808`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8808) |
| 31 | finds and filters .npmrc | ported | [`crates/renovate-core/src/extractors/npm.rs:8816`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8816) |
| 53 | uses config.npmrc if .npmrc does exist but npmrcmerge=false | ported | [`crates/renovate-core/src/extractors/npm.rs:8839`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8839) |
| 81 | uses config.npmrc if no .npmrc file is found | ported | [`crates/renovate-core/src/extractors/npm.rs:8853`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8853) |
| 98 | merges config.npmrc and repo .npmrc when npmrcmerge=true | ported | [`crates/renovate-core/src/extractors/npm.rs:8861`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8861) |
| 123 | does not add a newline between config.npmrc and repo .npmrc when npmrcmerge is true, if a newline already exists | ported | [`crates/renovate-core/src/extractors/npm.rs:8875`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8875) |
| 156 | finds and filters .npmrc with variables | ported | [`crates/renovate-core/src/extractors/npm.rs:8892`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8892) |
| 180 | keeps variables when exposeallenv is true | ported | [`crates/renovate-core/src/extractors/npm.rs:8910`](../../../../../../../crates/renovate-core/src/extractors/npm.rs#L8910) |

