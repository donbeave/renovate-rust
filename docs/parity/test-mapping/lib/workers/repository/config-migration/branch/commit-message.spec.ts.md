# `lib/workers/repository/config-migration/branch/commit-message.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | creates semantic commit message | ported | [`crates/renovate-core/src/branch.rs:2206`](../../../../../../../../crates/renovate-core/src/branch.rs#L2206) |
| 19 | creates semantic pr title | ported | [`crates/renovate-core/src/branch.rs:2215`](../../../../../../../../crates/renovate-core/src/branch.rs#L2215) |
| 30 | creates non-semantic commit message | ported | [`crates/renovate-core/src/branch.rs:2224`](../../../../../../../../crates/renovate-core/src/branch.rs#L2224) |
| 41 | creates non-semantic pr title | ported | [`crates/renovate-core/src/branch.rs:2233`](../../../../../../../../crates/renovate-core/src/branch.rs#L2233) |
| 50 | returns default values when commitmessage template string is empty | ported | [`crates/renovate-core/src/branch.rs:2242`](../../../../../../../../crates/renovate-core/src/branch.rs#L2242) |

