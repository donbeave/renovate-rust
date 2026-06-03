# `lib/workers/global/config/parse/host-rules-from-env.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**12/12 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 5 | supports docker username/password | ported | `crates/renovate-core/src/config/host_rules_from_env.rs:261` |
| 19 | supports password-only | ported | `crates/renovate-core/src/config/host_rules_from_env.rs:275` |
| 28 | supports domain and host names with case insensitivity | ported | `crates/renovate-core/src/config/host_rules_from_env.rs:285` |
| 40 | regression test for #10937 | ported | `crates/renovate-core/src/config/host_rules_from_env.rs:303` |
| 55 | support renovate_ prefixed host rules | ported | `crates/renovate-core/src/config/host_rules_from_env.rs:321` |
| 65 | supports renovate in the env variable | ported | `crates/renovate-core/src/config/host_rules_from_env.rs:331` |
| 77 | support https authentication options | ported | `crates/renovate-core/src/config/host_rules_from_env.rs:350` |
| 95 | make sure {{platform}}_token will not be picked up | ported | `crates/renovate-core/src/config/host_rules_from_env.rs:376` |
| 106 | supports datasource env token | ported | `crates/renovate-core/src/config/host_rules_from_env.rs:384` |
| 115 | supports platform env token | ported | `crates/renovate-core/src/config/host_rules_from_env.rs:394` |
| 130 | rejects incomplete datasource env token | ported | `crates/renovate-core/src/config/host_rules_from_env.rs:408` |
| 137 | rejects npm env | ported | `crates/renovate-core/src/config/host_rules_from_env.rs:416` |

