# `lib/modules/datasource/jsr/util.spec.ts`

[← `datasource/jsr`](../../../../_by-module/datasource/jsr.md) · [all modules](../../../../README.md)

**9/9 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | should extract package name | ported | `crates/renovate-core/src/datasources/jsr.rs:216` |
| 12 | should return null for invalid name | ported | `crates/renovate-core/src/datasources/jsr.rs:224` |
| 17 | should return null for below scope min length | ported | `crates/renovate-core/src/datasources/jsr.rs:230` |
| 22 | should return null for exceed scope max length | ported | `crates/renovate-core/src/datasources/jsr.rs:236` |
| 27 | should return null for invalid scope name | ported | `crates/renovate-core/src/datasources/jsr.rs:243` |
| 32 | should return null for invalid package name starting with @ | ported | `crates/renovate-core/src/datasources/jsr.rs:249` |
| 37 | should return null for exceed package max length | ported | `crates/renovate-core/src/datasources/jsr.rs:255` |
| 42 | should return null for invalid package name | ported | `crates/renovate-core/src/datasources/jsr.rs:262` |
| 47 | should return null for invalid package name starting with - | ported | `crates/renovate-core/src/datasources/jsr.rs:268` |

