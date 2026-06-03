# `lib/modules/manager/cake/index.spec.ts`

[← `manager/cake`](../../../../_by-module/manager/cake.md) · [all modules](../../../../README.md)

**3/5 ported** (2 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 21 | extracts | ported | `crates/renovate-core/src/extractors/cake.rs:231` |
| 45 | extracts dotnet tools from single sdk style build file | ported | `crates/renovate-core/src/extractors/cake.rs:294` |
| 101 | skips invalid entries in installtools | ported | `crates/renovate-core/src/extractors/cake.rs:338` |
| 124 | calls applyregistries to honor nuget.config files if present for .cake files | pending | — |
| 141 | calls applyregistries to honor nuget.config files if present for installtools | pending | — |

