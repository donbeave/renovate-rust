# `lib/workers/repository/process/lookup/timestamps.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**10/10 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns the timestamp of the latest version | ported | [`crates/renovate-core/src/util.rs:13510`](../../../../../../../../crates/renovate-core/src/util.rs#L13510) |
| 33 | handles releases with missing timestamps | ported | [`crates/renovate-core/src/util.rs:13535`](../../../../../../../../crates/renovate-core/src/util.rs#L13535) |
| 53 | handles latest release with missing timestamp | ported | [`crates/renovate-core/src/util.rs:13560`](../../../../../../../../crates/renovate-core/src/util.rs#L13560) |
| 75 | handles latest release with deprecation flag | ported | [`crates/renovate-core/src/util.rs:13585`](../../../../../../../../crates/renovate-core/src/util.rs#L13585) |
| 99 | handles latest release with invalid version | ported | [`crates/renovate-core/src/util.rs:13610`](../../../../../../../../crates/renovate-core/src/util.rs#L13610) |
| 122 | returns undefined mostrecenttimestamp when no valid timestamps exist | ported | [`crates/renovate-core/src/util.rs:13635`](../../../../../../../../crates/renovate-core/src/util.rs#L13635) |
| 132 | handles empty releases array | ported | [`crates/renovate-core/src/util.rs:13655`](../../../../../../../../crates/renovate-core/src/util.rs#L13655) |
| 138 | preserves other properties in the release result | ported | [`crates/renovate-core/src/util.rs:13664`](../../../../../../../../crates/renovate-core/src/util.rs#L13664) |
| 160 | handles ancient versions that are higher than the ones recently released | ported | [`crates/renovate-core/src/util.rs:13677`](../../../../../../../../crates/renovate-core/src/util.rs#L13677) |
| 180 | handles errors thrown for invalid versions | ported | [`crates/renovate-core/src/util.rs:13699`](../../../../../../../../crates/renovate-core/src/util.rs#L13699) |

