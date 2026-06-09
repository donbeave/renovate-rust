# `lib/modules/manager/bundler/artifacts.spec.ts`

[← `manager/bundler`](../../../../_by-module/manager/bundler.md) · [all modules](../../../../README.md)

**12/20 in-scope tests ported** (8 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 66 | returns null by default | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:258`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L258) |
| 77 | returns null if gemfile.lock was not changed | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:276`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L276) |
| 100 | executes commands from lockfile path | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:595`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L595) |
| 123 | works for default binarysource | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:317`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L317) |
| 149 | works explicit global binarysource | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:318`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L318) |
| 176 | supports conservative mode and updatetype option | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:571`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L571) |
| 217 | supports install mode | pending | — |
| 259 | .ruby-version | pending | — |
| 306 | constraints options | pending | — |
| 365 | invalid constraints options | pending | — |
| 426 | injects bundler host configuration environment variables | pending | — |
| 488 | returns error when failing in lockfilemaintenance true | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:497`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L497) |
| 517 | performs lockfilemaintenance | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:362`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L362) |
| 543 | returns error when failing in lockfilemaintenance true | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:497`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L497) |
| 577 | rethrows for temporary error | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:690`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L690) |
| 599 | handles "could not parse object" error | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:645`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L645) |
| 621 | throws on authentication errors | pending | — |
| 643 | handles recursive resolved dependencies | pending | — |
| 678 | updates the gemfile.lock when upgrading ruby | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:406`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L406) |
| 699 | updates the gemfile.lock when upgrading bundler | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:452`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L452) |

