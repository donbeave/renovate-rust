# `lib/workers/repository/process/lookup/abandonment.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 27 | returns the original release result when no abandonment threshold is provided | ported | [`crates/renovate-core/src/util.rs:9756`](../../../../../../../../crates/renovate-core/src/util.rs#L9756) |
| 39 | returns the original release result when abandonment threshold is invalid | ported | [`crates/renovate-core/src/util.rs:9763`](../../../../../../../../crates/renovate-core/src/util.rs#L9763) |
| 54 | returns the original release result when no mostrecenttimestamp timestamp is available | ported | [`crates/renovate-core/src/util.rs:9774`](../../../../../../../../crates/renovate-core/src/util.rs#L9774) |
| 69 | marks a package as abandoned when mostrecenttimestamp plus threshold is before now | ported | [`crates/renovate-core/src/util.rs:9781`](../../../../../../../../crates/renovate-core/src/util.rs#L9781) |
| 83 | does not mark a package as abandoned when mostrecenttimestamp plus threshold is after now | ported | [`crates/renovate-core/src/util.rs:9793`](../../../../../../../../crates/renovate-core/src/util.rs#L9793) |
| 97 | preserves other properties in the release result | ported | [`crates/renovate-core/src/util.rs:9805`](../../../../../../../../crates/renovate-core/src/util.rs#L9805) |
| 117 | handles exactly at the threshold boundary | ported | [`crates/renovate-core/src/util.rs:9818`](../../../../../../../../crates/renovate-core/src/util.rs#L9818) |

