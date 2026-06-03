# `lib/modules/manager/pep723/utils.spec.ts`

[← `manager/pep723`](../../../../_by-module/manager/pep723.md) · [all modules](../../../../README.md)

**5/5 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 6 | should extract dependencies | ported | `crates/renovate-core/src/extractors/pep723.rs:337` |
| 42 | should skip invalid dependencies | ported | `crates/renovate-core/src/extractors/pep723.rs:352` |
| 71 | should return null on missing dependencies | ported | `crates/renovate-core/src/extractors/pep723.rs:362` |
| 84 | should return null on invalid toml | ported | `crates/renovate-core/src/extractors/pep723.rs:369` |
| 101 | should return null if there is no pep 723 metadata | ported | `crates/renovate-core/src/extractors/pep723.rs:376` |

