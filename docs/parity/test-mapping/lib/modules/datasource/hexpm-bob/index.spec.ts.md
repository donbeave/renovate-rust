# `lib/modules/datasource/hexpm-bob/index.spec.ts`

[← `datasource/hexpm-bob`](../../../../_by-module/datasource/hexpm-bob.md) · [all modules](../../../../README.md)

**9/9 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 9 | throws for error | ported | `crates/renovate-core/src/datasources/hexpm_bob.rs:184` |
| 22 | returns null for 404 | ported | `crates/renovate-core/src/datasources/hexpm_bob.rs:199` |
| 35 | returns null for empty result | ported | `crates/renovate-core/src/datasources/hexpm_bob.rs:216` |
| 48 | returns empty list for empty 200 ok | ported | `crates/renovate-core/src/datasources/hexpm_bob.rs:233` |
| 61 | throws for 5xx | ported | `crates/renovate-core/src/datasources/hexpm_bob.rs:250` |
| 74 | processes real data | ported | `crates/renovate-core/src/datasources/hexpm_bob.rs:265` |
| 122 | processes real data (erlang / ubuntu 20.04) | ported | `crates/renovate-core/src/datasources/hexpm_bob.rs:319` |
| 155 | can override registry url | ported | `crates/renovate-core/src/datasources/hexpm_bob.rs:362` |
| 172 | returns empty list for invalid package name | ported | `crates/renovate-core/src/datasources/hexpm_bob.rs:387` |

