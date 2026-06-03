# `lib/workers/repository/process/lookup/timestamps.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**10/10 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 10 | returns the timestamp of the latest version | ported | `crates/renovate-core/src/util.rs:11775` |
| 33 | handles releases with missing timestamps | ported | `crates/renovate-core/src/util.rs:11800` |
| 53 | handles latest release with missing timestamp | ported | `crates/renovate-core/src/util.rs:11825` |
| 75 | handles latest release with deprecation flag | ported | `crates/renovate-core/src/util.rs:11850` |
| 99 | handles latest release with invalid version | ported | `crates/renovate-core/src/util.rs:11875` |
| 122 | returns undefined mostrecenttimestamp when no valid timestamps exist | ported | `crates/renovate-core/src/util.rs:11900` |
| 132 | handles empty releases array | ported | `crates/renovate-core/src/util.rs:11920` |
| 138 | preserves other properties in the release result | ported | `crates/renovate-core/src/util.rs:11929` |
| 160 | handles ancient versions that are higher than the ones recently released | ported | `crates/renovate-core/src/util.rs:11942` |
| 180 | handles errors thrown for invalid versions | ported | `crates/renovate-core/src/util.rs:11964` |

