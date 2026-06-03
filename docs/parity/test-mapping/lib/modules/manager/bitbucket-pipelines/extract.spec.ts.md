# `lib/modules/manager/bitbucket-pipelines/extract.spec.ts`

[← `manager/bitbucket-pipelines`](../../../../_by-module/manager/bitbucket-pipelines.md) · [all modules](../../../../README.md)

**4/4 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 6 | returns null for empty | ported | `crates/renovate-core/src/extractors/bitbucket_pipelines.rs:344` |
| 12 | returns null for malformed | ported | `crates/renovate-core/src/extractors/bitbucket_pipelines.rs:350` |
| 22 | extracts dependencies | ported | `crates/renovate-core/src/extractors/bitbucket_pipelines.rs:233` |
| 82 | extracts dependencies with registryalias | ported | `crates/renovate-core/src/extractors/bitbucket_pipelines.rs:435` |

