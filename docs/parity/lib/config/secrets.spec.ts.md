# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/config/secrets.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/secrets.spec.ts
**Total tests:** 13 | **Ported:** 13 | **Actionable:** 13 | **Status:** ported

### `config/secrets › validateConfigSecretsAndVariables(config)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works with default config | 14 | ported | `secrets.rs` | `validate_config_secrets_and_variables_works_with_default_config` | — |
| returns if no secrets/variables | 20 | ported | `secrets.rs` | `validate_config_secrets_and_variables_returns_without_entries` | — |
| throws for invalid secret name | 24 | ported | `secrets.rs` | `validate_config_secrets_and_variables_rejects_invalid_secret_name` | — |
| throws for invalid variable name | 32 | ported | `secrets.rs` | `validate_config_secrets_and_variables_rejects_invalid_variable_name` | — |
| throws for secrets in repositories | 40 | ported | `secrets.rs` | `validate_config_secrets_and_variables_rejects_repository_secrets` | — |
| throws for variables in repositories | 48 | ported | `secrets.rs` | `validate_config_secrets_and_variables_rejects_repository_variables` | — |

### `config/secrets › applySecretsAndVariablesToConfig(config)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| replaces both secrets and variables | 58 | ported | `secrets.rs` | `apply_secrets_and_variables_replaces_both` | — |
| replaces all secrets and variables | 75 | ported | `secrets.rs` | `apply_secrets_and_variables_replaces_all` | — |
| handles a mix of space characters around the curly braces | 94 | ported | `secrets.rs` | `apply_secrets_and_variables_handles_spaces_around_braces` | — |
| does not handle non-space characters around the curly braces | 111 | ported | `secrets.rs` | `apply_secrets_and_variables_does_not_handle_non_space_characters` | — |
| preserves secrets and variables if delete flags are false | 128 | ported | `secrets.rs` | `apply_secrets_and_variables_preserves_sources_when_delete_flags_are_false` | — |
| throws if secret is missing | 151 | ported | `secrets.rs` | `apply_secrets_and_variables_errors_if_secret_missing` | — |
| throws if variable is missing | 160 | ported | `secrets.rs` | `apply_secrets_and_variables_errors_if_variable_missing` | — |

---

