# `lib/util/cache/memory/index.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**7/7 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | returns undefined if not init | ported | `crates/renovate-core/src/cache/memory.rs:54` |
| 8 | sets and gets repo cache | ported | `crates/renovate-core/src/cache/memory.rs:61` |
| 14 | resets | ported | `crates/renovate-core/src/cache/memory.rs:70` |
| 26 | does nothing if no matching keys exist | ported | `crates/renovate-core/src/cache/memory.rs:80` |
| 34 | removes keys that start with datasource-mem:pkg-fetch: | ported | `crates/renovate-core/src/cache/memory.rs:98` |
| 42 | removes keys that start with datasource-releases | ported | `crates/renovate-core/src/cache/memory.rs:116` |
| 50 | removes all matching keys while keeping others | ported | `crates/renovate-core/src/cache/memory.rs:134` |

