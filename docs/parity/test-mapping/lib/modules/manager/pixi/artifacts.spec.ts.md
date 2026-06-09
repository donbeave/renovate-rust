# `lib/modules/manager/pixi/artifacts.spec.ts`

[← `manager/pixi`](../../../../_by-module/manager/pixi.md) · [all modules](../../../../README.md)

**7/10 in-scope tests ported** (3 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 70 | returns null if no pixi.lock found | ported | [`crates/renovate-core/src/extractors/pixi_artifact_runner.rs:191`](../../../../../../../crates/renovate-core/src/extractors/pixi_artifact_runner.rs#L191) |
| 83 | returns null if updateddeps is empty | ported | [`crates/renovate-core/src/extractors/pixi_artifact_runner.rs:210`](../../../../../../../crates/renovate-core/src/extractors/pixi_artifact_runner.rs#L210) |
| 96 | returns null if unchanged | ported | [`crates/renovate-core/src/extractors/pixi_artifact_runner.rs:228`](../../../../../../../crates/renovate-core/src/extractors/pixi_artifact_runner.rs#L228) |
| 122 | handle temporary_error | ported | [`crates/renovate-core/src/extractors/pixi_artifact_runner.rs:403`](../../../../../../../crates/renovate-core/src/extractors/pixi_artifact_runner.rs#L403) |
| 140 | returns updated pixi.lock using docker | ported | [`crates/renovate-core/src/extractors/pixi_artifact_runner.rs:271`](../../../../../../../crates/renovate-core/src/extractors/pixi_artifact_runner.rs#L271) |
| 196 | returns updated pixi.lock using install mode | pending | — |
| 235 | returns updated pixi.lock using install mode for old version lock file | pending | — |
| 273 | returns pixi version defined in requires-pixi | pending | — |
| 328 | catches errors | ported | [`crates/renovate-core/src/extractors/pixi_artifact_runner.rs:359`](../../../../../../../crates/renovate-core/src/extractors/pixi_artifact_runner.rs#L359) |
| 348 | returns updated pixi.lock when doing lockfile maintenance | ported | [`crates/renovate-core/src/extractors/pixi_artifact_runner.rs:315`](../../../../../../../crates/renovate-core/src/extractors/pixi_artifact_runner.rs#L315) |

