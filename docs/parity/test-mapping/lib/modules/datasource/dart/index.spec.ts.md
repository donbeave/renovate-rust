# `lib/modules/datasource/dart/index.spec.ts`

[← `datasource/dart`](../../../../_by-module/datasource/dart.md) · [all modules](../../../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 13 | returns null for empty result | ported | `crates/renovate-core/src/datasources/pub_dev.rs:235` |
| 23 | returns null for empty fields | ported | `crates/renovate-core/src/datasources/pub_dev.rs:252` |
| 55 | returns null for 404 | ported | `crates/renovate-core/src/datasources/pub_dev.rs:287` |
| 65 | throws for 5xx | ported | `crates/renovate-core/src/datasources/pub_dev.rs:304` |
| 75 | returns null for unknown error | ported | `crates/renovate-core/src/datasources/pub_dev.rs:319` |
| 85 | processes real data | ported | `crates/renovate-core/src/datasources/pub_dev.rs:331` |

