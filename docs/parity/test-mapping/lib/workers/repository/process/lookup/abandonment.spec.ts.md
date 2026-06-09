# `lib/workers/repository/process/lookup/abandonment.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 27 | returns the original release result when no abandonment threshold is provided | ported | [`crates/renovate-core/src/util.rs:9743`](../../../../../../../../crates/renovate-core/src/util.rs#L9743) |
| 39 | returns the original release result when abandonment threshold is invalid | ported | [`crates/renovate-core/src/util.rs:9750`](../../../../../../../../crates/renovate-core/src/util.rs#L9750) |
| 54 | returns the original release result when no mostrecenttimestamp timestamp is available | ported | [`crates/renovate-core/src/util.rs:9761`](../../../../../../../../crates/renovate-core/src/util.rs#L9761) |
| 69 | marks a package as abandoned when mostrecenttimestamp plus threshold is before now | ported | [`crates/renovate-core/src/util.rs:9768`](../../../../../../../../crates/renovate-core/src/util.rs#L9768) |
| 83 | does not mark a package as abandoned when mostrecenttimestamp plus threshold is after now | ported | [`crates/renovate-core/src/util.rs:9780`](../../../../../../../../crates/renovate-core/src/util.rs#L9780) |
| 97 | preserves other properties in the release result | ported | [`crates/renovate-core/src/util.rs:9792`](../../../../../../../../crates/renovate-core/src/util.rs#L9792) |
| 117 | handles exactly at the threshold boundary | ported | [`crates/renovate-core/src/util.rs:9805`](../../../../../../../../crates/renovate-core/src/util.rs#L9805) |

