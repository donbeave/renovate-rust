# `lib/modules/manager/mise/artifacts.spec.ts`

[← `manager/mise`](../../../../_by-module/manager/mise.md) · [all modules](../../../../README.md)

**8/23 ported** (15 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 46 | returns null if lock file does not exist | pending | — |
| 60 | returns null if lock file unchanged after exec | pending | — |
| 81 | returns updated lock file on success | pending | — |
| 112 | returns artifacterror on exec failure with combined output | pending | — |
| 138 | rethrows temporary_error | pending | — |
| 153 | runs mise lock for lockfilemaintenance | pending | — |
| 173 | runs mise lock <tools> for targeted updates | pending | — |
| 193 | injects github_token when host rule found | pending | — |
| 238 | handles empty updateddeps with fallback to full lock | pending | — |
| 258 | handles environment-specific lock files | pending | — |
| 296 | uses --local flag for local config files | pending | — |
| 327 | uses --local flag and mise_env for env-specific local config files | pending | — |
| 354 | uses --local flag for lock file maintenance on local config | pending | — |
| 378 | prevents command injection | pending | — |
| 400 | handles subdirectory package files | pending | — |
| 441 | returns already-updated when version matches | ported | [`crates/renovate-core/src/extractors/mise.rs:1485`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L1485) |
| 454 | returns already-updated for tool with backend prefix | ported | [`crates/renovate-core/src/extractors/mise.rs:1494`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L1494) |
| 467 | returns unsupported when version does not match | ported | [`crates/renovate-core/src/extractors/mise.rs:1503`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L1503) |
| 480 | returns unsupported when tool not in lock file | ported | [`crates/renovate-core/src/extractors/mise.rs:1512`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L1512) |
| 493 | returns unsupported when no lock file content | ported | [`crates/renovate-core/src/extractors/mise.rs:1521`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L1521) |
| 506 | returns unsupported for invalid lock file content | ported | [`crates/renovate-core/src/extractors/mise.rs:1530`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L1530) |
| 519 | returns unsupported when depname is undefined | ported | [`crates/renovate-core/src/extractors/mise.rs:1539`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L1539) |
| 532 | returns update-failed in case of errors | ported | [`crates/renovate-core/src/extractors/mise.rs:1548`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L1548) |

