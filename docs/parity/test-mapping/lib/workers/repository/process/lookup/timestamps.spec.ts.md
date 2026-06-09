# `lib/workers/repository/process/lookup/timestamps.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**10/10 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns the timestamp of the latest version | ported | [`crates/renovate-core/src/util.rs:13633`](../../../../../../../../crates/renovate-core/src/util.rs#L13633) |
| 33 | handles releases with missing timestamps | ported | [`crates/renovate-core/src/util.rs:13658`](../../../../../../../../crates/renovate-core/src/util.rs#L13658) |
| 53 | handles latest release with missing timestamp | ported | [`crates/renovate-core/src/util.rs:13683`](../../../../../../../../crates/renovate-core/src/util.rs#L13683) |
| 75 | handles latest release with deprecation flag | ported | [`crates/renovate-core/src/util.rs:13708`](../../../../../../../../crates/renovate-core/src/util.rs#L13708) |
| 99 | handles latest release with invalid version | ported | [`crates/renovate-core/src/util.rs:13733`](../../../../../../../../crates/renovate-core/src/util.rs#L13733) |
| 122 | returns undefined mostrecenttimestamp when no valid timestamps exist | ported | [`crates/renovate-core/src/util.rs:13758`](../../../../../../../../crates/renovate-core/src/util.rs#L13758) |
| 132 | handles empty releases array | ported | [`crates/renovate-core/src/util.rs:13778`](../../../../../../../../crates/renovate-core/src/util.rs#L13778) |
| 138 | preserves other properties in the release result | ported | [`crates/renovate-core/src/util.rs:13787`](../../../../../../../../crates/renovate-core/src/util.rs#L13787) |
| 160 | handles ancient versions that are higher than the ones recently released | ported | [`crates/renovate-core/src/util.rs:13800`](../../../../../../../../crates/renovate-core/src/util.rs#L13800) |
| 180 | handles errors thrown for invalid versions | ported | [`crates/renovate-core/src/util.rs:13822`](../../../../../../../../crates/renovate-core/src/util.rs#L13822) |

