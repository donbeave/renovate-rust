# `lib/modules/manager/cargo/artifacts.spec.ts`

[← `manager/cargo`](../../../../_by-module/manager/cargo.md) · [all modules](../../../../README.md)

**9/20 ported** (11 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 44 | returns null if no cargo.lock found | ported | `crates/renovate-core/src/extractors/cargo_artifact_runner.rs:374` |
| 62 | returns null if updateddeps is empty | ported | `crates/renovate-core/src/extractors/cargo_artifact_runner.rs:400` |
| 73 | returns null if unchanged | pending | — |
| 98 | returns updated cargo.lock | ported | `crates/renovate-core/src/extractors/cargo_artifact_runner.rs:419` |
| 122 | returns updated cargo.lock with precise version update | ported | `crates/renovate-core/src/extractors/cargo_artifact_runner.rs:486` |
| 164 | skips precise update when manifest range has changed | ported | `crates/renovate-core/src/extractors/cargo_artifact_runner.rs:546` |
| 199 | handles mixed deps where some have range changes and some do not | ported | `crates/renovate-core/src/extractors/cargo.rs:1926` |
| 247 | returns an artifact error when cargo update fails | ported | `crates/renovate-core/src/extractors/cargo_artifact_runner.rs:601` |
| 284 | returns updated cargo.lock when a preceding dependency triggers an update in a later dependency | pending | — |
| 413 | returns updated cargo.lock when there are no more dependencies to update | ported | `crates/renovate-core/src/extractors/cargo.rs:1952` |
| 434 | updates cargo.lock based on the packagename, when given | pending | — |
| 458 | returns updated workspace cargo.lock | pending | — |
| 488 | returns updated cargo.lock for lockfile maintenance | ported | `crates/renovate-core/src/extractors/cargo_artifact_runner.rs:658` |
| 509 | supports docker mode | pending | — |
| 564 | supports docker mode with credentials | pending | — |
| 661 | supports docker mode with many credentials | pending | — |
| 749 | supports docker mode and ignores non git credentials | pending | — |
| 809 | supports docker mode with cargo specific credential | pending | — |
| 878 | supports install mode | pending | — |
| 929 | catches errors | pending | — |

