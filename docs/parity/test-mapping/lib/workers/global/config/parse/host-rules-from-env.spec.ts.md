# `lib/workers/global/config/parse/host-rules-from-env.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**12/12 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 5 | supports docker username/password | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:261`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L261) |
| 19 | supports password-only | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:275`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L275) |
| 28 | supports domain and host names with case insensitivity | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:285`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L285) |
| 40 | regression test for #10937 | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:303`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L303) |
| 55 | support renovate_ prefixed host rules | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:321`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L321) |
| 65 | supports renovate in the env variable | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:331`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L331) |
| 77 | support https authentication options | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:350`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L350) |
| 95 | make sure {{platform}}_token will not be picked up | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:376`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L376) |
| 106 | supports datasource env token | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:384`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L384) |
| 115 | supports platform env token | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:394`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L394) |
| 130 | rejects incomplete datasource env token | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:408`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L408) |
| 137 | rejects npm env | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:416`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L416) |

