# `lib/workers/repository/process/lookup/abandonment.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**7/7 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 27 | returns the original release result when no abandonment threshold is provided | ported | `crates/renovate-core/src/util.rs:8198` |
| 39 | returns the original release result when abandonment threshold is invalid | ported | `crates/renovate-core/src/util.rs:8205` |
| 54 | returns the original release result when no mostrecenttimestamp timestamp is available | ported | `crates/renovate-core/src/util.rs:8216` |
| 69 | marks a package as abandoned when mostrecenttimestamp plus threshold is before now | ported | `crates/renovate-core/src/util.rs:8223` |
| 83 | does not mark a package as abandoned when mostrecenttimestamp plus threshold is after now | ported | `crates/renovate-core/src/util.rs:8235` |
| 97 | preserves other properties in the release result | ported | `crates/renovate-core/src/util.rs:8247` |
| 117 | handles exactly at the threshold boundary | ported | `crates/renovate-core/src/util.rs:8260` |

