# `lib/modules/manager/poetry/update-locked.spec.ts`

[← `manager/poetry`](../../../../_by-module/manager/poetry.md) · [all modules](../../../../README.md)

**3/3 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 11 | detects already updated | ported | `crates/renovate-core/src/extractors/poetry.rs:1638` |
| 23 | returns unsupported | ported | `crates/renovate-core/src/extractors/poetry.rs:1646` |
| 35 | returns unsupported for mising locked content | ported | `crates/renovate-core/src/extractors/poetry.rs:1654` |

