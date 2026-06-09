# `lib/workers/repository/process/lookup/timestamps.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**10/10 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns the timestamp of the latest version | ported | [`crates/renovate-core/src/util.rs:13507`](../../../../../../../../crates/renovate-core/src/util.rs#L13507) |
| 33 | handles releases with missing timestamps | ported | [`crates/renovate-core/src/util.rs:13532`](../../../../../../../../crates/renovate-core/src/util.rs#L13532) |
| 53 | handles latest release with missing timestamp | ported | [`crates/renovate-core/src/util.rs:13557`](../../../../../../../../crates/renovate-core/src/util.rs#L13557) |
| 75 | handles latest release with deprecation flag | ported | [`crates/renovate-core/src/util.rs:13582`](../../../../../../../../crates/renovate-core/src/util.rs#L13582) |
| 99 | handles latest release with invalid version | ported | [`crates/renovate-core/src/util.rs:13607`](../../../../../../../../crates/renovate-core/src/util.rs#L13607) |
| 122 | returns undefined mostrecenttimestamp when no valid timestamps exist | ported | [`crates/renovate-core/src/util.rs:13632`](../../../../../../../../crates/renovate-core/src/util.rs#L13632) |
| 132 | handles empty releases array | ported | [`crates/renovate-core/src/util.rs:13652`](../../../../../../../../crates/renovate-core/src/util.rs#L13652) |
| 138 | preserves other properties in the release result | ported | [`crates/renovate-core/src/util.rs:13661`](../../../../../../../../crates/renovate-core/src/util.rs#L13661) |
| 160 | handles ancient versions that are higher than the ones recently released | ported | [`crates/renovate-core/src/util.rs:13674`](../../../../../../../../crates/renovate-core/src/util.rs#L13674) |
| 180 | handles errors thrown for invalid versions | ported | [`crates/renovate-core/src/util.rs:13696`](../../../../../../../../crates/renovate-core/src/util.rs#L13696) |

