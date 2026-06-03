# `lib/logger/remap.spec.ts`

[← `logger`](../../_by-module/logger.md) · [all modules](../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 15 | returns null if no remaps are set | ported | `crates/renovate-core/src/util.rs:8485` |
| 24 | performs global remaps | ported | `crates/renovate-core/src/util.rs:8491` |
| 33 | performs repository-level remaps | ported | `crates/renovate-core/src/util.rs:8501` |
| 44 | prioritizes repository-level remaps over global remaps | ported | `crates/renovate-core/src/util.rs:8508` |
| 55 | supports regex patterns | ported | `crates/renovate-core/src/util.rs:8519` |
| 64 | does not match against invalid regex patterns | ported | `crates/renovate-core/src/util.rs:8529` |

