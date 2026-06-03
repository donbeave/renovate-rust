# `lib/modules/datasource/jsr/index.spec.ts`

[← `datasource/jsr`](../../../../_by-module/datasource/jsr.md) · [all modules](../../../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 24 | should return null for invalid package name | ported | `crates/renovate-core/src/datasources/jsr.rs:276` |
| 32 | should return null for no versions | ported | `crates/renovate-core/src/datasources/jsr.rs:286` |
| 46 | should fetch package info from jsr | ported | `crates/renovate-core/src/datasources/jsr.rs:305` |
| 74 | contains yanked versions | ported | `crates/renovate-core/src/datasources/jsr.rs:341` |
| 102 | should return null if lookup fails | ported | `crates/renovate-core/src/datasources/jsr.rs:370` |
| 115 | should throw error for unparseable | ported | `crates/renovate-core/src/datasources/jsr.rs:386` |

