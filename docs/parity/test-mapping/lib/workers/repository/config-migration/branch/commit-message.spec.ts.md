# `lib/workers/repository/config-migration/branch/commit-message.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | creates semantic commit message | ported | [`crates/renovate-core/src/branch.rs:2189`](../../../../../../../../crates/renovate-core/src/branch.rs#L2189) |
| 19 | creates semantic pr title | ported | [`crates/renovate-core/src/branch.rs:2198`](../../../../../../../../crates/renovate-core/src/branch.rs#L2198) |
| 30 | creates non-semantic commit message | ported | [`crates/renovate-core/src/branch.rs:2207`](../../../../../../../../crates/renovate-core/src/branch.rs#L2207) |
| 41 | creates non-semantic pr title | ported | [`crates/renovate-core/src/branch.rs:2216`](../../../../../../../../crates/renovate-core/src/branch.rs#L2216) |
| 50 | returns default values when commitmessage template string is empty | ported | [`crates/renovate-core/src/branch.rs:2225`](../../../../../../../../crates/renovate-core/src/branch.rs#L2225) |

