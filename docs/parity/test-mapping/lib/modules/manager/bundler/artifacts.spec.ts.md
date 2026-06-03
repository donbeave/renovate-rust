# `lib/modules/manager/bundler/artifacts.spec.ts`

[← `manager/bundler`](../../../../_by-module/manager/bundler.md) · [all modules](../../../../README.md)

**9/20 ported** (11 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 66 | returns null by default | ported | `crates/renovate-core/src/extractors/bundler_artifact_runner.rs:251` |
| 77 | returns null if gemfile.lock was not changed | ported | `crates/renovate-core/src/extractors/bundler_artifact_runner.rs:269` |
| 100 | executes commands from lockfile path | ported | `crates/renovate-core/src/extractors/bundler_artifact_runner.rs:586` |
| 123 | works for default binarysource | pending | — |
| 149 | works explicit global binarysource | pending | — |
| 176 | supports conservative mode and updatetype option | ported | `crates/renovate-core/src/extractors/bundler_artifact_runner.rs:562` |
| 217 | supports install mode | pending | — |
| 259 | .ruby-version | pending | — |
| 306 | constraints options | pending | — |
| 365 | invalid constraints options | pending | — |
| 426 | injects bundler host configuration environment variables | pending | — |
| 488 | returns error when failing in lockfilemaintenance true | ported | `crates/renovate-core/src/extractors/bundler_artifact_runner.rs:488` |
| 517 | performs lockfilemaintenance | ported | `crates/renovate-core/src/extractors/bundler_artifact_runner.rs:353` |
| 543 | returns error when failing in lockfilemaintenance true | ported | `crates/renovate-core/src/extractors/bundler_artifact_runner.rs:488` |
| 577 | rethrows for temporary error | pending | — |
| 599 | handles "could not parse object" error | ported | `crates/renovate-core/src/extractors/bundler_artifact_runner.rs:636` |
| 621 | throws on authentication errors | pending | — |
| 643 | handles recursive resolved dependencies | pending | — |
| 678 | updates the gemfile.lock when upgrading ruby | ported | `crates/renovate-core/src/extractors/bundler_artifact_runner.rs:397` |
| 699 | updates the gemfile.lock when upgrading bundler | ported | `crates/renovate-core/src/extractors/bundler_artifact_runner.rs:443` |

