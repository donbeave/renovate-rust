# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/global/config/parse/host-rules-from-env.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/global/config/parse/host-rules-from-env.spec.ts
**Total tests:** 12 | **Ported:** 12 | **Actionable:** 0 | **Status:** done

### `workers/global/config/parse/host-rules-from-env`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports docker username/password | 5 | ported | `config/host_rules_from_env.rs` | `host_rules_docker_user_pass` | — |
| supports password-only | 19 | ported | `config/host_rules_from_env.rs` | `host_rules_npm_password_only` | — |
| supports domain and host names with case insensitivity | 28 | ported | `config/host_rules_from_env.rs` | `host_rules_domain_and_host` | — |
| regression test for #10937 | 40 | ported | `config/host_rules_from_env.rs` | `host_rules_regression_10937` | — |
| support RENOVATE_ prefixed host rules | 55 | ported | `config/host_rules_from_env.rs` | `host_rules_renovate_prefix` | — |
| supports renovate in the env variable | 65 | ported | `config/host_rules_from_env.rs` | `host_rules_renovate_in_var` | — |
| support https authentication options | 77 | ported | `config/host_rules_from_env.rs` | `host_rules_https_auth` | — |
| make sure {{PLATFORM}}_TOKEN will not be picked up | 95 | ported | `config/host_rules_from_env.rs` | `host_rules_platform_token_skipped` | — |
| supports datasource env token | 106 | ported | `config/host_rules_from_env.rs` | `host_rules_datasource_token` | — |
| supports platform env token | 115 | ported | `config/host_rules_from_env.rs` | `host_rules_platform_token` | — |
| rejects incomplete datasource env token | 130 | ported | `config/host_rules_from_env.rs` | `host_rules_incomplete_token` | — |
| rejects npm env | 137 | ported | `config/host_rules_from_env.rs` | `host_rules_npm_env_skipped` | — |

---
