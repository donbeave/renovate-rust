# `lib/workers/global/config/parse/host-rules-from-env.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**12/12 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 5 | supports docker username/password | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:262`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L262) |
| 19 | supports password-only | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:276`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L276) |
| 28 | supports domain and host names with case insensitivity | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:286`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L286) |
| 40 | regression test for #10937 | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:304`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L304) |
| 55 | support renovate_ prefixed host rules | ported | [`crates/renovate-cli/src/config_env.rs:1192`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1192) |
| 65 | supports renovate in the env variable | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:332`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L332) |
| 77 | support https authentication options | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:351`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L351) |
| 95 | make sure {{platform}}_token will not be picked up | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:377`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L377) |
| 106 | supports datasource env token | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:385`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L385) |
| 115 | supports platform env token | ported | [`crates/renovate-cli/src/config_env.rs:1218`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1218) |
| 130 | rejects incomplete datasource env token | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:409`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L409) |
| 137 | rejects npm env | ported | [`crates/renovate-core/src/config/host_rules_from_env.rs:417`](../../../../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L417) |

