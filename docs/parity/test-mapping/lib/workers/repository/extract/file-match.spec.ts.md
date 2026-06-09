# `lib/workers/repository/extract/file-match.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**7/8 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns filelist if no includepaths | ported | [`crates/renovate-core/src/managers.rs:2276`](../../../../../../../crates/renovate-core/src/managers.rs#L2276) |
| 13 | returns exact matches | ported | [`crates/renovate-core/src/managers.rs:2284`](../../../../../../../crates/renovate-core/src/managers.rs#L2284) |
| 20 | returns minimatch matches | ported | [`crates/renovate-core/src/managers.rs:2293`](../../../../../../../crates/renovate-core/src/managers.rs#L2293) |
| 29 | returns filelist if no ignoredpaths | ported | [`crates/renovate-core/src/managers.rs:2302`](../../../../../../../crates/renovate-core/src/managers.rs#L2302) |
| 34 | ignores partial matches | ported | [`crates/renovate-core/src/managers.rs:2310`](../../../../../../../crates/renovate-core/src/managers.rs#L2310) |
| 41 | returns minimatch matches | ported | [`crates/renovate-core/src/managers.rs:2293`](../../../../../../../crates/renovate-core/src/managers.rs#L2293) |
| 57 | returns npm files | ported | [`crates/renovate-core/src/managers.rs:2344`](../../../../../../../crates/renovate-core/src/managers.rs#L2344) |
| 64 | deduplicates | ported | [`crates/renovate-core/src/managers.rs:2319`](../../../../../../../crates/renovate-core/src/managers.rs#L2319) |

