# `lib/workers/repository/process/lookup/timestamps.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**10/10 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns the timestamp of the latest version | ported | [`crates/renovate-core/src/util.rs:13499`](../../../../../../../../crates/renovate-core/src/util.rs#L13499) |
| 33 | handles releases with missing timestamps | ported | [`crates/renovate-core/src/util.rs:13524`](../../../../../../../../crates/renovate-core/src/util.rs#L13524) |
| 53 | handles latest release with missing timestamp | ported | [`crates/renovate-core/src/util.rs:13549`](../../../../../../../../crates/renovate-core/src/util.rs#L13549) |
| 75 | handles latest release with deprecation flag | ported | [`crates/renovate-core/src/util.rs:13574`](../../../../../../../../crates/renovate-core/src/util.rs#L13574) |
| 99 | handles latest release with invalid version | ported | [`crates/renovate-core/src/util.rs:13599`](../../../../../../../../crates/renovate-core/src/util.rs#L13599) |
| 122 | returns undefined mostrecenttimestamp when no valid timestamps exist | ported | [`crates/renovate-core/src/util.rs:13624`](../../../../../../../../crates/renovate-core/src/util.rs#L13624) |
| 132 | handles empty releases array | ported | [`crates/renovate-core/src/util.rs:13644`](../../../../../../../../crates/renovate-core/src/util.rs#L13644) |
| 138 | preserves other properties in the release result | ported | [`crates/renovate-core/src/util.rs:13653`](../../../../../../../../crates/renovate-core/src/util.rs#L13653) |
| 160 | handles ancient versions that are higher than the ones recently released | ported | [`crates/renovate-core/src/util.rs:13666`](../../../../../../../../crates/renovate-core/src/util.rs#L13666) |
| 180 | handles errors thrown for invalid versions | ported | [`crates/renovate-core/src/util.rs:13688`](../../../../../../../../crates/renovate-core/src/util.rs#L13688) |

