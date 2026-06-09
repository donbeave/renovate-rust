# `lib/workers/repository/process/lookup/timestamps.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**10/10 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns the timestamp of the latest version | ported | [`crates/renovate-core/src/util.rs:13514`](../../../../../../../../crates/renovate-core/src/util.rs#L13514) |
| 33 | handles releases with missing timestamps | ported | [`crates/renovate-core/src/util.rs:13539`](../../../../../../../../crates/renovate-core/src/util.rs#L13539) |
| 53 | handles latest release with missing timestamp | ported | [`crates/renovate-core/src/util.rs:13564`](../../../../../../../../crates/renovate-core/src/util.rs#L13564) |
| 75 | handles latest release with deprecation flag | ported | [`crates/renovate-core/src/util.rs:13589`](../../../../../../../../crates/renovate-core/src/util.rs#L13589) |
| 99 | handles latest release with invalid version | ported | [`crates/renovate-core/src/util.rs:13614`](../../../../../../../../crates/renovate-core/src/util.rs#L13614) |
| 122 | returns undefined mostrecenttimestamp when no valid timestamps exist | ported | [`crates/renovate-core/src/util.rs:13639`](../../../../../../../../crates/renovate-core/src/util.rs#L13639) |
| 132 | handles empty releases array | ported | [`crates/renovate-core/src/util.rs:13659`](../../../../../../../../crates/renovate-core/src/util.rs#L13659) |
| 138 | preserves other properties in the release result | ported | [`crates/renovate-core/src/util.rs:13668`](../../../../../../../../crates/renovate-core/src/util.rs#L13668) |
| 160 | handles ancient versions that are higher than the ones recently released | ported | [`crates/renovate-core/src/util.rs:13681`](../../../../../../../../crates/renovate-core/src/util.rs#L13681) |
| 180 | handles errors thrown for invalid versions | ported | [`crates/renovate-core/src/util.rs:13703`](../../../../../../../../crates/renovate-core/src/util.rs#L13703) |

