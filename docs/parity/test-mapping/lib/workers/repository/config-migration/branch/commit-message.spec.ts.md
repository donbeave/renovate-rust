# `lib/workers/repository/config-migration/branch/commit-message.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | creates semantic commit message | ported | [`crates/renovate-core/src/branch.rs:2306`](../../../../../../../../crates/renovate-core/src/branch.rs#L2306) |
| 19 | creates semantic pr title | ported | [`crates/renovate-core/src/branch.rs:2315`](../../../../../../../../crates/renovate-core/src/branch.rs#L2315) |
| 30 | creates non-semantic commit message | ported | [`crates/renovate-core/src/branch.rs:2324`](../../../../../../../../crates/renovate-core/src/branch.rs#L2324) |
| 41 | creates non-semantic pr title | ported | [`crates/renovate-core/src/branch.rs:2333`](../../../../../../../../crates/renovate-core/src/branch.rs#L2333) |
| 50 | returns default values when commitmessage template string is empty | ported | [`crates/renovate-core/src/branch.rs:2342`](../../../../../../../../crates/renovate-core/src/branch.rs#L2342) |

