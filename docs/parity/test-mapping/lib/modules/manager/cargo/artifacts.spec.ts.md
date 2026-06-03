# `lib/modules/manager/cargo/artifacts.spec.ts`

[← `manager/cargo`](../../../../_by-module/manager/cargo.md) · [all modules](../../../../README.md)

**13/20 in-scope tests ported** (7 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 44 | returns null if no cargo.lock found | ported | [`crates/renovate-core/src/extractors/cargo_artifact_runner.rs:378`](../../../../../../../crates/renovate-core/src/extractors/cargo_artifact_runner.rs#L378) |
| 62 | returns null if updateddeps is empty | ported | [`crates/renovate-core/src/extractors/cargo_artifact_runner.rs:404`](../../../../../../../crates/renovate-core/src/extractors/cargo_artifact_runner.rs#L404) |
| 73 | returns null if unchanged | ported | [`crates/renovate-core/src/extractors/cargo_artifact_runner.rs:423`](../../../../../../../crates/renovate-core/src/extractors/cargo_artifact_runner.rs#L423) |
| 98 | returns updated cargo.lock | ported | [`crates/renovate-core/src/extractors/cargo_artifact_runner.rs:480`](../../../../../../../crates/renovate-core/src/extractors/cargo_artifact_runner.rs#L480) |
| 122 | returns updated cargo.lock with precise version update | ported | [`crates/renovate-core/src/extractors/cargo_artifact_runner.rs:662`](../../../../../../../crates/renovate-core/src/extractors/cargo_artifact_runner.rs#L662) |
| 164 | skips precise update when manifest range has changed | ported | [`crates/renovate-core/src/extractors/cargo_artifact_runner.rs:722`](../../../../../../../crates/renovate-core/src/extractors/cargo_artifact_runner.rs#L722) |
| 199 | handles mixed deps where some have range changes and some do not | ported | [`crates/renovate-core/src/extractors/cargo.rs:1926`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L1926) |
| 247 | returns an artifact error when cargo update fails | ported | [`crates/renovate-core/src/extractors/cargo_artifact_runner.rs:777`](../../../../../../../crates/renovate-core/src/extractors/cargo_artifact_runner.rs#L777) |
| 284 | returns updated cargo.lock when a preceding dependency triggers an update in a later dependency | ported | [`crates/renovate-core/src/extractors/cargo_artifact_runner.rs:882`](../../../../../../../crates/renovate-core/src/extractors/cargo_artifact_runner.rs#L882) |
| 413 | returns updated cargo.lock when there are no more dependencies to update | ported | [`crates/renovate-core/src/extractors/cargo.rs:1952`](../../../../../../../crates/renovate-core/src/extractors/cargo.rs#L1952) |
| 434 | updates cargo.lock based on the packagename, when given | ported | [`crates/renovate-core/src/extractors/cargo_artifact_runner.rs:547`](../../../../../../../crates/renovate-core/src/extractors/cargo_artifact_runner.rs#L547) |
| 458 | returns updated workspace cargo.lock | ported | [`crates/renovate-core/src/extractors/cargo_artifact_runner.rs:604`](../../../../../../../crates/renovate-core/src/extractors/cargo_artifact_runner.rs#L604) |
| 488 | returns updated cargo.lock for lockfile maintenance | ported | [`crates/renovate-core/src/extractors/cargo_artifact_runner.rs:834`](../../../../../../../crates/renovate-core/src/extractors/cargo_artifact_runner.rs#L834) |
| 509 | supports docker mode | pending | — |
| 564 | supports docker mode with credentials | pending | — |
| 661 | supports docker mode with many credentials | pending | — |
| 749 | supports docker mode and ignores non git credentials | pending | — |
| 809 | supports docker mode with cargo specific credential | pending | — |
| 878 | supports install mode | pending | — |
| 929 | catches errors | pending | — |

