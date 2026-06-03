# `lib/workers/repository/extract/file-match.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**7/8 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns filelist if no includepaths | ported | [`crates/renovate-core/src/managers.rs:2069`](../../../../../../../crates/renovate-core/src/managers.rs#L2069) |
| 13 | returns exact matches | ported | [`crates/renovate-core/src/managers.rs:2077`](../../../../../../../crates/renovate-core/src/managers.rs#L2077) |
| 20 | returns minimatch matches | ported | [`crates/renovate-core/src/managers.rs:2086`](../../../../../../../crates/renovate-core/src/managers.rs#L2086) |
| 29 | returns filelist if no ignoredpaths | ported | [`crates/renovate-core/src/managers.rs:2095`](../../../../../../../crates/renovate-core/src/managers.rs#L2095) |
| 34 | ignores partial matches | ported | [`crates/renovate-core/src/managers.rs:2103`](../../../../../../../crates/renovate-core/src/managers.rs#L2103) |
| 41 | returns minimatch matches | ported | [`crates/renovate-core/src/managers.rs:2086`](../../../../../../../crates/renovate-core/src/managers.rs#L2086) |
| 57 | returns npm files | ported | [`crates/renovate-core/src/managers.rs:2121`](../../../../../../../crates/renovate-core/src/managers.rs#L2121) |
| 64 | deduplicates | ported | [`crates/renovate-core/src/managers.rs:2132`](../../../../../../../crates/renovate-core/src/managers.rs#L2132) |

