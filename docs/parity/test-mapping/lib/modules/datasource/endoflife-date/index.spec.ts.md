# `lib/modules/datasource/endoflife-date/index.spec.ts`

[← `datasource/endoflife-date`](../../../../_by-module/datasource/endoflife-date.md) · [all modules](../../../../README.md)

**7/7 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 22 | processes real data | ported | `crates/renovate-core/src/datasources/endoflife.rs:174` |
| 83 | returns null without registryurl | ported | `crates/renovate-core/src/datasources/endoflife.rs:213` |
| 92 | returns null for 404 | ported | `crates/renovate-core/src/datasources/endoflife.rs:221` |
| 102 | returns null for empty result | ported | `crates/renovate-core/src/datasources/endoflife.rs:238` |
| 112 | throws for 5xx | ported | `crates/renovate-core/src/datasources/endoflife.rs:255` |
| 122 | detects boolean discontinuation | ported | `crates/renovate-core/src/datasources/endoflife.rs:270` |
| 158 | detects date discontinuation | ported | `crates/renovate-core/src/datasources/endoflife.rs:322` |

