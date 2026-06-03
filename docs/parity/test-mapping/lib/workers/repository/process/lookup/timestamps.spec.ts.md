# `lib/workers/repository/process/lookup/timestamps.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**10/10 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 10 | returns the timestamp of the latest version | ported | [`crates/renovate-core/src/util.rs:11775`](../../../../../../../../crates/renovate-core/src/util.rs#L11775) |
| 33 | handles releases with missing timestamps | ported | [`crates/renovate-core/src/util.rs:11800`](../../../../../../../../crates/renovate-core/src/util.rs#L11800) |
| 53 | handles latest release with missing timestamp | ported | [`crates/renovate-core/src/util.rs:11825`](../../../../../../../../crates/renovate-core/src/util.rs#L11825) |
| 75 | handles latest release with deprecation flag | ported | [`crates/renovate-core/src/util.rs:11850`](../../../../../../../../crates/renovate-core/src/util.rs#L11850) |
| 99 | handles latest release with invalid version | ported | [`crates/renovate-core/src/util.rs:11875`](../../../../../../../../crates/renovate-core/src/util.rs#L11875) |
| 122 | returns undefined mostrecenttimestamp when no valid timestamps exist | ported | [`crates/renovate-core/src/util.rs:11900`](../../../../../../../../crates/renovate-core/src/util.rs#L11900) |
| 132 | handles empty releases array | ported | [`crates/renovate-core/src/util.rs:11920`](../../../../../../../../crates/renovate-core/src/util.rs#L11920) |
| 138 | preserves other properties in the release result | ported | [`crates/renovate-core/src/util.rs:11929`](../../../../../../../../crates/renovate-core/src/util.rs#L11929) |
| 160 | handles ancient versions that are higher than the ones recently released | ported | [`crates/renovate-core/src/util.rs:11942`](../../../../../../../../crates/renovate-core/src/util.rs#L11942) |
| 180 | handles errors thrown for invalid versions | ported | [`crates/renovate-core/src/util.rs:11964`](../../../../../../../../crates/renovate-core/src/util.rs#L11964) |

