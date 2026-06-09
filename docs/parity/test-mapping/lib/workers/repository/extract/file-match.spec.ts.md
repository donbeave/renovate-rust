# `lib/workers/repository/extract/file-match.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**7/8 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns filelist if no includepaths | ported | [`crates/renovate-core/src/managers.rs:2064`](../../../../../../../crates/renovate-core/src/managers.rs#L2064) |
| 13 | returns exact matches | ported | [`crates/renovate-core/src/managers.rs:2072`](../../../../../../../crates/renovate-core/src/managers.rs#L2072) |
| 20 | returns minimatch matches | ported | [`crates/renovate-core/src/managers.rs:2081`](../../../../../../../crates/renovate-core/src/managers.rs#L2081) |
| 29 | returns filelist if no ignoredpaths | ported | [`crates/renovate-core/src/managers.rs:2090`](../../../../../../../crates/renovate-core/src/managers.rs#L2090) |
| 34 | ignores partial matches | ported | [`crates/renovate-core/src/managers.rs:2098`](../../../../../../../crates/renovate-core/src/managers.rs#L2098) |
| 41 | returns minimatch matches | ported | [`crates/renovate-core/src/managers.rs:2081`](../../../../../../../crates/renovate-core/src/managers.rs#L2081) |
| 57 | returns npm files | ported | [`crates/renovate-core/src/managers.rs:2132`](../../../../../../../crates/renovate-core/src/managers.rs#L2132) |
| 64 | deduplicates | ported | [`crates/renovate-core/src/managers.rs:2107`](../../../../../../../crates/renovate-core/src/managers.rs#L2107) |

