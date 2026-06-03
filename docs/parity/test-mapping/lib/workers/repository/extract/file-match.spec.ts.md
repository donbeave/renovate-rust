# `lib/workers/repository/extract/file-match.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**7/8 ported** (1 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 8 | returns filelist if no includepaths | ported | `crates/renovate-core/src/managers.rs:2069` |
| 13 | returns exact matches | ported | `crates/renovate-core/src/managers.rs:2077` |
| 20 | returns minimatch matches | ported | `crates/renovate-core/src/managers.rs:2086` |
| 29 | returns filelist if no ignoredpaths | ported | `crates/renovate-core/src/managers.rs:2095` |
| 34 | ignores partial matches | ported | `crates/renovate-core/src/managers.rs:2103` |
| 41 | returns minimatch matches | ported | `crates/renovate-core/src/managers.rs:2086` |
| 57 | returns npm files | ported | `crates/renovate-core/src/managers.rs:2121` |
| 64 | deduplicates | ported | `crates/renovate-core/src/managers.rs:2132` |

