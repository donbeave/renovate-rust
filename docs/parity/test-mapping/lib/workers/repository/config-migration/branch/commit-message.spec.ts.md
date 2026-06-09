# `lib/workers/repository/config-migration/branch/commit-message.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | creates semantic commit message | ported | [`crates/renovate-core/src/branch.rs:2192`](../../../../../../../../crates/renovate-core/src/branch.rs#L2192) |
| 19 | creates semantic pr title | ported | [`crates/renovate-core/src/branch.rs:2201`](../../../../../../../../crates/renovate-core/src/branch.rs#L2201) |
| 30 | creates non-semantic commit message | ported | [`crates/renovate-core/src/branch.rs:2210`](../../../../../../../../crates/renovate-core/src/branch.rs#L2210) |
| 41 | creates non-semantic pr title | ported | [`crates/renovate-core/src/branch.rs:2219`](../../../../../../../../crates/renovate-core/src/branch.rs#L2219) |
| 50 | returns default values when commitmessage template string is empty | ported | [`crates/renovate-core/src/branch.rs:2228`](../../../../../../../../crates/renovate-core/src/branch.rs#L2228) |

