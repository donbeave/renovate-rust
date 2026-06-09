# `lib/workers/repository/extract/file-match.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**7/8 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns filelist if no includepaths | ported | [`crates/renovate-core/src/managers.rs:2063`](../../../../../../../crates/renovate-core/src/managers.rs#L2063) |
| 13 | returns exact matches | ported | [`crates/renovate-core/src/managers.rs:2071`](../../../../../../../crates/renovate-core/src/managers.rs#L2071) |
| 20 | returns minimatch matches | ported | [`crates/renovate-core/src/managers.rs:2080`](../../../../../../../crates/renovate-core/src/managers.rs#L2080) |
| 29 | returns filelist if no ignoredpaths | ported | [`crates/renovate-core/src/managers.rs:2089`](../../../../../../../crates/renovate-core/src/managers.rs#L2089) |
| 34 | ignores partial matches | ported | [`crates/renovate-core/src/managers.rs:2097`](../../../../../../../crates/renovate-core/src/managers.rs#L2097) |
| 41 | returns minimatch matches | ported | [`crates/renovate-core/src/managers.rs:2080`](../../../../../../../crates/renovate-core/src/managers.rs#L2080) |
| 57 | returns npm files | ported | [`crates/renovate-core/src/managers.rs:2131`](../../../../../../../crates/renovate-core/src/managers.rs#L2131) |
| 64 | deduplicates | ported | [`crates/renovate-core/src/managers.rs:2106`](../../../../../../../crates/renovate-core/src/managers.rs#L2106) |

