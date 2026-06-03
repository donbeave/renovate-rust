# `lib/modules/datasource/hexpm-bob/index.spec.ts`

[← `datasource/hexpm-bob`](../../../../_by-module/datasource/hexpm-bob.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | throws for error | ported | [`crates/renovate-core/src/datasources/hexpm_bob.rs:184`](../../../../../../../crates/renovate-core/src/datasources/hexpm_bob.rs#L184) |
| 22 | returns null for 404 | ported | [`crates/renovate-core/src/datasources/hexpm_bob.rs:199`](../../../../../../../crates/renovate-core/src/datasources/hexpm_bob.rs#L199) |
| 35 | returns null for empty result | ported | [`crates/renovate-core/src/datasources/hexpm_bob.rs:216`](../../../../../../../crates/renovate-core/src/datasources/hexpm_bob.rs#L216) |
| 48 | returns empty list for empty 200 ok | ported | [`crates/renovate-core/src/datasources/hexpm_bob.rs:233`](../../../../../../../crates/renovate-core/src/datasources/hexpm_bob.rs#L233) |
| 61 | throws for 5xx | ported | [`crates/renovate-core/src/datasources/hexpm_bob.rs:250`](../../../../../../../crates/renovate-core/src/datasources/hexpm_bob.rs#L250) |
| 74 | processes real data | ported | [`crates/renovate-core/src/datasources/hexpm_bob.rs:265`](../../../../../../../crates/renovate-core/src/datasources/hexpm_bob.rs#L265) |
| 122 | processes real data (erlang / ubuntu 20.04) | ported | [`crates/renovate-core/src/datasources/hexpm_bob.rs:319`](../../../../../../../crates/renovate-core/src/datasources/hexpm_bob.rs#L319) |
| 155 | can override registry url | ported | [`crates/renovate-core/src/datasources/hexpm_bob.rs:362`](../../../../../../../crates/renovate-core/src/datasources/hexpm_bob.rs#L362) |
| 172 | returns empty list for invalid package name | ported | [`crates/renovate-core/src/datasources/hexpm_bob.rs:387`](../../../../../../../crates/renovate-core/src/datasources/hexpm_bob.rs#L387) |

