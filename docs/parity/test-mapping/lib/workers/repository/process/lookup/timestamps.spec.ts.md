# `lib/workers/repository/process/lookup/timestamps.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**10/10 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns the timestamp of the latest version | ported | [`crates/renovate-core/src/util.rs:13501`](../../../../../../../../crates/renovate-core/src/util.rs#L13501) |
| 33 | handles releases with missing timestamps | ported | [`crates/renovate-core/src/util.rs:13526`](../../../../../../../../crates/renovate-core/src/util.rs#L13526) |
| 53 | handles latest release with missing timestamp | ported | [`crates/renovate-core/src/util.rs:13551`](../../../../../../../../crates/renovate-core/src/util.rs#L13551) |
| 75 | handles latest release with deprecation flag | ported | [`crates/renovate-core/src/util.rs:13576`](../../../../../../../../crates/renovate-core/src/util.rs#L13576) |
| 99 | handles latest release with invalid version | ported | [`crates/renovate-core/src/util.rs:13601`](../../../../../../../../crates/renovate-core/src/util.rs#L13601) |
| 122 | returns undefined mostrecenttimestamp when no valid timestamps exist | ported | [`crates/renovate-core/src/util.rs:13626`](../../../../../../../../crates/renovate-core/src/util.rs#L13626) |
| 132 | handles empty releases array | ported | [`crates/renovate-core/src/util.rs:13646`](../../../../../../../../crates/renovate-core/src/util.rs#L13646) |
| 138 | preserves other properties in the release result | ported | [`crates/renovate-core/src/util.rs:13655`](../../../../../../../../crates/renovate-core/src/util.rs#L13655) |
| 160 | handles ancient versions that are higher than the ones recently released | ported | [`crates/renovate-core/src/util.rs:13668`](../../../../../../../../crates/renovate-core/src/util.rs#L13668) |
| 180 | handles errors thrown for invalid versions | ported | [`crates/renovate-core/src/util.rs:13690`](../../../../../../../../crates/renovate-core/src/util.rs#L13690) |

