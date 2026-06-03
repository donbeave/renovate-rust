# `lib/util/cache/package/index.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**8/9 ported** (1 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 23 | returns undefined if not initialized | ported | `crates/renovate-core/src/cache/package.rs:810` |
| 33 | delegates init to backend | ported | `crates/renovate-core/src/cache/package.rs:818` |
| 41 | delegates get to backend | ported | `crates/renovate-core/src/cache/package.rs:555` |
| 51 | delegates set to backend | ported | `crates/renovate-core/src/cache/package.rs:564` |
| 64 | delegates setwithrawttl to backend | pending | — |
| 77 | deduplicates get via memcache | ported | `crates/renovate-core/src/cache/package.rs:865` |
| 89 | setwithrawttl updates memcache | ported | `crates/renovate-core/src/cache/package.rs:881` |
| 99 | delegates cleanup to backend.destroy | ported | `crates/renovate-core/src/cache/package.rs:840` |
| 105 | delegates getcachetype to backend | ported | `crates/renovate-core/src/cache/package.rs:829` |

