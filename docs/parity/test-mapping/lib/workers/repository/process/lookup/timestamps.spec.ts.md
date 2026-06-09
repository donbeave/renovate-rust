# `lib/workers/repository/process/lookup/timestamps.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**10/10 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns the timestamp of the latest version | ported | [`crates/renovate-core/src/util.rs:13500`](../../../../../../../../crates/renovate-core/src/util.rs#L13500) |
| 33 | handles releases with missing timestamps | ported | [`crates/renovate-core/src/util.rs:13525`](../../../../../../../../crates/renovate-core/src/util.rs#L13525) |
| 53 | handles latest release with missing timestamp | ported | [`crates/renovate-core/src/util.rs:13550`](../../../../../../../../crates/renovate-core/src/util.rs#L13550) |
| 75 | handles latest release with deprecation flag | ported | [`crates/renovate-core/src/util.rs:13575`](../../../../../../../../crates/renovate-core/src/util.rs#L13575) |
| 99 | handles latest release with invalid version | ported | [`crates/renovate-core/src/util.rs:13600`](../../../../../../../../crates/renovate-core/src/util.rs#L13600) |
| 122 | returns undefined mostrecenttimestamp when no valid timestamps exist | ported | [`crates/renovate-core/src/util.rs:13625`](../../../../../../../../crates/renovate-core/src/util.rs#L13625) |
| 132 | handles empty releases array | ported | [`crates/renovate-core/src/util.rs:13645`](../../../../../../../../crates/renovate-core/src/util.rs#L13645) |
| 138 | preserves other properties in the release result | ported | [`crates/renovate-core/src/util.rs:13654`](../../../../../../../../crates/renovate-core/src/util.rs#L13654) |
| 160 | handles ancient versions that are higher than the ones recently released | ported | [`crates/renovate-core/src/util.rs:13667`](../../../../../../../../crates/renovate-core/src/util.rs#L13667) |
| 180 | handles errors thrown for invalid versions | ported | [`crates/renovate-core/src/util.rs:13689`](../../../../../../../../crates/renovate-core/src/util.rs#L13689) |

