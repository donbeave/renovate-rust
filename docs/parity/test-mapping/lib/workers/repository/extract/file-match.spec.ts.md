# `lib/workers/repository/extract/file-match.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**7/8 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns filelist if no includepaths | ported | [`crates/renovate-core/src/managers.rs:2278`](../../../../../../../crates/renovate-core/src/managers.rs#L2278) |
| 13 | returns exact matches | ported | [`crates/renovate-core/src/managers.rs:2286`](../../../../../../../crates/renovate-core/src/managers.rs#L2286) |
| 20 | returns minimatch matches | ported | [`crates/renovate-core/src/managers.rs:2295`](../../../../../../../crates/renovate-core/src/managers.rs#L2295) |
| 29 | returns filelist if no ignoredpaths | ported | [`crates/renovate-core/src/managers.rs:2304`](../../../../../../../crates/renovate-core/src/managers.rs#L2304) |
| 34 | ignores partial matches | ported | [`crates/renovate-core/src/managers.rs:2312`](../../../../../../../crates/renovate-core/src/managers.rs#L2312) |
| 41 | returns minimatch matches | ported | [`crates/renovate-core/src/managers.rs:2295`](../../../../../../../crates/renovate-core/src/managers.rs#L2295) |
| 57 | returns npm files | ported | [`crates/renovate-core/src/managers.rs:2346`](../../../../../../../crates/renovate-core/src/managers.rs#L2346) |
| 64 | deduplicates | ported | [`crates/renovate-core/src/managers.rs:2321`](../../../../../../../crates/renovate-core/src/managers.rs#L2321) |

