# `lib/workers/repository/process/lookup/timestamps.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**10/10 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns the timestamp of the latest version | ported | [`crates/renovate-core/src/util.rs:13498`](../../../../../../../../crates/renovate-core/src/util.rs#L13498) |
| 33 | handles releases with missing timestamps | ported | [`crates/renovate-core/src/util.rs:13523`](../../../../../../../../crates/renovate-core/src/util.rs#L13523) |
| 53 | handles latest release with missing timestamp | ported | [`crates/renovate-core/src/util.rs:13548`](../../../../../../../../crates/renovate-core/src/util.rs#L13548) |
| 75 | handles latest release with deprecation flag | ported | [`crates/renovate-core/src/util.rs:13573`](../../../../../../../../crates/renovate-core/src/util.rs#L13573) |
| 99 | handles latest release with invalid version | ported | [`crates/renovate-core/src/util.rs:13598`](../../../../../../../../crates/renovate-core/src/util.rs#L13598) |
| 122 | returns undefined mostrecenttimestamp when no valid timestamps exist | ported | [`crates/renovate-core/src/util.rs:13623`](../../../../../../../../crates/renovate-core/src/util.rs#L13623) |
| 132 | handles empty releases array | ported | [`crates/renovate-core/src/util.rs:13643`](../../../../../../../../crates/renovate-core/src/util.rs#L13643) |
| 138 | preserves other properties in the release result | ported | [`crates/renovate-core/src/util.rs:13652`](../../../../../../../../crates/renovate-core/src/util.rs#L13652) |
| 160 | handles ancient versions that are higher than the ones recently released | ported | [`crates/renovate-core/src/util.rs:13665`](../../../../../../../../crates/renovate-core/src/util.rs#L13665) |
| 180 | handles errors thrown for invalid versions | ported | [`crates/renovate-core/src/util.rs:13687`](../../../../../../../../crates/renovate-core/src/util.rs#L13687) |

