# `lib/workers/repository/config-migration/branch/commit-message.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**5/5 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 8 | creates semantic commit message | ported | `crates/renovate-core/src/branch.rs:2168` |
| 19 | creates semantic pr title | ported | `crates/renovate-core/src/branch.rs:2177` |
| 30 | creates non-semantic commit message | ported | `crates/renovate-core/src/branch.rs:2186` |
| 41 | creates non-semantic pr title | ported | `crates/renovate-core/src/branch.rs:2195` |
| 50 | returns default values when commitmessage template string is empty | ported | `crates/renovate-core/src/branch.rs:2204` |

