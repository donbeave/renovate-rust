# `lib/workers/global/config/parse/host-rules-from-env.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**12/12 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 5 | supports docker username/password | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:188`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L188) |
| 19 | supports password-only | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:202`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L202) |
| 28 | supports domain and host names with case insensitivity | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:212`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L212) |
| 40 | regression test for #10937 | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:230`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L230) |
| 55 | support renovate_ prefixed host rules | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:248`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L248) |
| 65 | supports renovate in the env variable | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:258`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L258) |
| 77 | support https authentication options | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:277`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L277) |
| 95 | make sure {{platform}}_token will not be picked up | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:303`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L303) |
| 106 | supports datasource env token | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:311`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L311) |
| 115 | supports platform env token | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:331`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L331) |
| 130 | rejects incomplete datasource env token | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:345`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L345) |
| 137 | rejects npm env | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:353`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L353) |

