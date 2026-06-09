# `lib/modules/manager/bundler/artifacts.spec.ts`

[← `manager/bundler`](../../../../_by-module/manager/bundler.md) · [all modules](../../../../README.md)

**10/20 in-scope tests ported** (10 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 66 | returns null by default | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:251`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L251) |
| 77 | returns null if gemfile.lock was not changed | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:269`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L269) |
| 100 | executes commands from lockfile path | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:587`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L587) |
| 123 | works for default binarysource | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:310`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L310) |
| 149 | works explicit global binarysource | pending | — |
| 176 | supports conservative mode and updatetype option | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:563`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L563) |
| 217 | supports install mode | pending | — |
| 259 | .ruby-version | pending | — |
| 306 | constraints options | pending | — |
| 365 | invalid constraints options | pending | — |
| 426 | injects bundler host configuration environment variables | pending | — |
| 488 | returns error when failing in lockfilemaintenance true | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:489`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L489) |
| 517 | performs lockfilemaintenance | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:354`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L354) |
| 543 | returns error when failing in lockfilemaintenance true | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:489`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L489) |
| 577 | rethrows for temporary error | pending | — |
| 599 | handles "could not parse object" error | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:637`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L637) |
| 621 | throws on authentication errors | pending | — |
| 643 | handles recursive resolved dependencies | pending | — |
| 678 | updates the gemfile.lock when upgrading ruby | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:398`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L398) |
| 699 | updates the gemfile.lock when upgrading bundler | ported | [`crates/renovate-core/src/extractors/bundler_artifact_runner.rs:444`](../../../../../../../crates/renovate-core/src/extractors/bundler_artifact_runner.rs#L444) |

