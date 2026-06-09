# `lib/workers/repository/extract/file-match.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**7/8 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns filelist if no includepaths | ported | [`crates/renovate-core/src/managers.rs:2271`](../../../../../../../crates/renovate-core/src/managers.rs#L2271) |
| 13 | returns exact matches | ported | [`crates/renovate-core/src/managers.rs:2279`](../../../../../../../crates/renovate-core/src/managers.rs#L2279) |
| 20 | returns minimatch matches | ported | [`crates/renovate-core/src/managers.rs:2288`](../../../../../../../crates/renovate-core/src/managers.rs#L2288) |
| 29 | returns filelist if no ignoredpaths | ported | [`crates/renovate-core/src/managers.rs:2297`](../../../../../../../crates/renovate-core/src/managers.rs#L2297) |
| 34 | ignores partial matches | ported | [`crates/renovate-core/src/managers.rs:2305`](../../../../../../../crates/renovate-core/src/managers.rs#L2305) |
| 41 | returns minimatch matches | ported | [`crates/renovate-core/src/managers.rs:2288`](../../../../../../../crates/renovate-core/src/managers.rs#L2288) |
| 57 | returns npm files | ported | [`crates/renovate-core/src/managers.rs:2339`](../../../../../../../crates/renovate-core/src/managers.rs#L2339) |
| 64 | deduplicates | ported | [`crates/renovate-core/src/managers.rs:2314`](../../../../../../../crates/renovate-core/src/managers.rs#L2314) |

