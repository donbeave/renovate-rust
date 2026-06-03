# `lib/modules/datasource/node-version/index.spec.ts`

[← `datasource/node-version`](../../../../_by-module/datasource/node-version.md) · [all modules](../../../../README.md)

**2/4 ported** (2 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 9 | throws for 500 | pending | — |
| 19 | returns null for error | pending | — |
| 32 | returns null for empty 200 ok | ported | `crates/renovate-core/src/datasources/node_version.rs:121` |
| 42 | processes real data | ported | `crates/renovate-core/src/datasources/node_version.rs:129` |

