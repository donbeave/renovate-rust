# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/global/config/parse/env.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/global/config/parse/env.spec.ts
**Total tests:** 45 | **Ported:** 40 | **Actionable:** 45 | **Status:** partial

### `workers/global/config/parse/env › .getConfig(env)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty env | 11 | ported | `config_env.rs` | `empty_env_returns_default_config` | — |
| supports boolean true | 15 | ported | `config_env.rs` | `config_migration_true_is_parsed` | — |
| supports boolean false | 20 | ported | `config_env.rs` | `config_migration_false_is_parsed` | — |
| throws exception for invalid boolean value | 27 | ported | `config_env.rs` | `config_migration_invalid_boolean_is_rejected` | — |
| supports list single | 40 | ported | `config_env.rs` | `labels_single_value_is_parsed` | — |
| supports list multiple | 45 | ported | `config_env.rs` | `labels_multiple_values_are_parsed` | — |
| supports list multiple without blank items | 50 | ported | `config_env.rs` | `labels_ignore_blank_items` | — |
| supports string | 55 | ported | `config_env.rs` | `token_is_parsed` | — |
| coerces string newlines | 60 | ported | `config_env.rs` | `string_newlines_are_coerced` | — |
| supports custom prefixes | 67 | ported | `config_env.rs` | `custom_prefix_is_supported` | — |
| supports json | 76 | ported | `config_env.rs` | `lock_file_maintenance_json_is_parsed` | — |
| supports arrays of objects | 83 | ported | `config_env.rs` | `host_rules_array_is_parsed` | — |
| "$envArg" -> $config | 91 | ported | `config_env.rs` | `recreate_env_aliases_are_parsed` | — |
| skips misconfigured arrays | 103 | ported | `config_env.rs` | `host_rules_string_value_is_skipped` | — |
| skips garbage array values | 117 | ported | `config_env.rs` | `host_rules_garbage_value_is_skipped` | — |
| supports GitHub token | 131 | ported | `config_env.rs` | `github_token_is_parsed` | — |
| supports GitHub custom endpoint | 140 | ported | `config_env.rs` | `github_endpoint_is_parsed` | — |
| supports GitHub custom endpoint and github.com | 149 | ported | `config_env.rs` | `github_com_token_becomes_host_rule_with_custom_endpoint` | — |
| supports GitHub fine-grained PATs | 168 | ported | `config_env.rs` | `github_fine_grained_pat_becomes_host_rule` | — |
| supports RENOVATE_ prefixed github com token | 185 | ported | `config_env.rs` | `renovate_prefixed_github_com_token_becomes_host_rule` | — |
| GITHUB_COM_TOKEN takes precedence over RENOVATE_GITHUB_COM_TOKEN | 202 | ported | `config_env.rs` | `github_com_token_takes_precedence_over_renovate_prefixed_token` | — |
| supports GitHub custom endpoint and gitlab.com | 220 | ported | `config_env.rs` | `github_custom_endpoint_without_github_com_token_has_no_host_rule` | — |
| supports GitLab token | 231 | ported | `config_env.rs` | `gitlab_token_is_parsed` | — |
| supports GitLab custom endpoint | 242 | ported | `config_env.rs` | `gitlab_custom_endpoint_is_parsed` | — |
| supports Azure DevOps | 255 | ported | `config_env.rs` | `azure_devops_config_is_parsed` | — |
| supports Bitbucket token | 268 | ported | `config_env.rs` | `bitbucket_token_config_is_parsed` | — |
| supports Bitbucket username/password | 283 | ported | `config_env.rs` | `bitbucket_username_password_config_is_parsed` | — |
| merges full config from env | 299 | ported | `config_env.rs` | `renovate_config_merges_with_explicit_env` | — |
| massages converted experimental env vars | 309 | ported | `config_env.rs` | `experimental_env_vars_are_massaged` (+ `converted_experimental_env_current_names_are_parsed`) | — |
| does not migrate empty RENOVATE_X_REPO_CACHE_FORCE_LOCAL | 336 | ported | `config_env.rs` | `empty_repo_cache_force_local_is_not_migrated` | — |

### `workers/global/config/parse/env › .getConfig(env) › RENOVATE_CONFIG tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| crashes | 357 | ported | `config_env.rs` | `invalid_renovate_config_is_rejected` | — |
| migrates RENOVATE_CONFIG | 367 | ported | `config_env.rs` | `renovate_config_automerge_any_is_migrated` | — |
| warns if config in RENOVATE_CONFIG is invalid | 376 | pending | — | — | — |

### `workers/global/config/parse/env › .getConfig(env) › migrations`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| renames migrated variables | 386 | ported | `config_env.rs` | `git_lab_automerge_env_sets_platform_automerge` (+ `renamed_env_vars_map_to_current_options`) | — |

### `workers/global/config/parse/env`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| has no duplicate env names across options | 396 | pending | — | — | — |

### `workers/global/config/parse/env › .getEnvName(definition)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty | 418 | pending | — | — | — |
| returns existing env | 426 | pending | — | — | — |
| generates RENOVATE_ env | 434 | pending | — | — | — |
| dryRun boolean true | 441 | ported | `config_env.rs` | `dry_run_true_maps_to_full` | — |
| dryRun boolean false | 449 | ported | `config_env.rs` | `dry_run_false_disables_dry_run` | — |
| dryRun null | 457 | ported | `config_env.rs` | `dry_run_null_disables_dry_run` | — |
| requireConfig boolean true | 465 | ported | `config_env.rs` | `require_config_true_maps_to_required` | — |
| requireConfig boolean false | 473 | ported | `config_env.rs` | `require_config_false_maps_to_optional` | — |
| platformCommit boolean true | 481 | ported | `config_env.rs` | `platform_commit_true_maps_to_enabled` | — |
| platformCommit boolean false | 489 | ported | `config_env.rs` | `platform_commit_false_maps_to_disabled` | — |

---
