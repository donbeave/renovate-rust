# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/global/config/parse/cli.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/global/config/parse/cli.spec.ts
**Total tests:** 30 | **Ported:** 28 | **Actionable:** 28 | **Status:** ported

### `workers/global/config/parse/cli › .getCliName(definition)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| generates CLI value | 15 | not-applicable | — | — | TypeScript option-definition helper; Rust CLI names are static `clap` attributes |
| generates returns empty if CLI false | 22 | not-applicable | — | — | TypeScript option-definition helper; Rust has no runtime `cli: false` option metadata |

### `workers/global/config/parse/cli › .getConfig(argv)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty argv | 32 | ported | `config_builder.rs` | `default_cli_produces_default_config` | — |
| supports boolean no value | 36 | ported | `config_builder.rs` | `config_migration_bare_sets_true` | — |
| supports boolean space true | 42 | ported | `config_builder.rs` | `config_migration_space_true_sets_true` | — |
| throws exception for invalid boolean value | 48 | ported | `config_builder.rs` | `config_migration_invalid_boolean_is_rejected` | — |
| supports boolean space false | 58 | ported | `config_builder.rs` | `config_migration_space_false_sets_false` | — |
| supports boolean equals true | 64 | ported | `config_builder.rs` | `config_migration_equals_true_sets_true` | — |
| supports boolean equals false | 69 | ported | `config_builder.rs` | `config_migration_equals_false_sets_false` | — |
| supports list single | 74 | ported | `config_builder.rs` | `labels_single_value_is_set` | — |
| supports list multiple | 79 | ported | `config_builder.rs` | `labels_comma_separated_values_are_set` | — |
| supports string | 84 | ported | `config_builder.rs` | `token_is_set` | — |
| supports repositories | 89 | ported | `config_builder.rs` | `repositories_are_set` | — |
| parses json lists correctly | 95 | ported | `config_builder.rs` | `host_rules_json_list_is_parsed` | — |
| parses [] correctly as empty list of hostRules | 111 | ported | `config_builder.rs` | `host_rules_empty_array_is_parsed` | — |
| parses an empty string correctly as empty list of hostRules | 118 | ported | `config_builder.rs` | `host_rules_empty_string_is_parsed` | — |
| "$arg" -> $config | 125 | ported | `config_builder.rs` | `migrated_cli_aliases_produce_expected_config` | — |
| parses json object correctly when empty | 145 | ported | `config_builder.rs` | `onboarding_config_empty_string_is_parsed` | — |
| parses json {} object correctly | 152 | ported | `config_builder.rs` | `onboarding_config_empty_object_is_parsed` | — |
| parses json object correctly | 159 | ported | `config_builder.rs` | `onboarding_config_object_is_parsed` | — |
| throws exception for invalid json object | 168 | ported | `config_builder.rs` | `onboarding_config_invalid_json_is_rejected` | — |
| dryRun boolean true | 175 | ported | `config_builder.rs` | `dry_run_legacy_true_maps_to_full` | — |
| dryRun no value | 180 | ported | `cli.rs` | `dry_run_bare_is_accepted_via_migrate` | — |
| dryRun boolean false | 185 | ported | `config_builder.rs` | `dry_run_legacy_false_disables_dry_run` | — |
| dryRun  null | 190 | ported | `config_builder.rs` | `dry_run_legacy_null_disables_dry_run` | — |
| requireConfig boolean true | 195 | ported | `config_builder.rs` | `require_config_legacy_true_maps_to_required` | — |
| requireConfig no value | 200 | ported | `cli.rs` | `require_config_bare_is_accepted_via_migrate` | — |
| requireConfig boolean false | 205 | ported | `config_builder.rs` | `require_config_legacy_false_maps_to_optional` | — |

### `workers/global/config/parse/cli › .parseEarlyFlags(argv)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| prints version and exits when --version is passed | 212 | ported | `cli.rs` | `version_long_flag_prints_bare_version` | — |
| does not error when --dry-run is the last argument | 229 | ported | `cli.rs` | `dry_run_last_argument_after_repository_is_accepted` | — |

---

