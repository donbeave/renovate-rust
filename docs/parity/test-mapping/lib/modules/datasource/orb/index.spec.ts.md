# `lib/modules/datasource/orb/index.spec.ts`

[← `datasource/orb`](../../../../_by-module/datasource/orb.md) · [all modules](../../../../README.md)

**7/7 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 32 | returns null for empty result | ported | `crates/renovate-core/src/datasources/orb.rs:263` |
| 42 | returns null for missing orb | ported | `crates/renovate-core/src/datasources/orb.rs:284` |
| 55 | returns null for 404 | ported | `crates/renovate-core/src/datasources/orb.rs:305` |
| 65 | returns null for unknown error | ported | `crates/renovate-core/src/datasources/orb.rs:326` |
| 75 | processes real data | ported | `crates/renovate-core/src/datasources/orb.rs:340` |
| 85 | processes homeurl | ported | `crates/renovate-core/src/datasources/orb.rs:384` |
| 96 | supports other registries | ported | `crates/renovate-core/src/datasources/orb.rs:408` |

