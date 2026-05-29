# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/config/migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migration.spec.ts
**Total tests:** 30 | **Ported:** 29 | **Actionable:** 30 | **Status:** partial

### `config/migration › migrateConfig(config, parentConfig)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| migrates config | 17 | ported | `repo_config.rs` | `broad_config_migration_covers_representable_fields` | Covers the fields represented in Rust's typed config model; raw TS-only migration output remains covered by narrower not-applicable rows below |
| migrates before and after schedules | 184 | ported | `repo_config.rs` | `schedule_before_after_migration_matches_renovate_cases` | — |
| migrates every friday | 205 | ported | `repo_config.rs` | `schedule_every_friday_migrated_to_on_friday` | — |
| migrates semantic prefix with no scope | 215 | ported | `repo_config.rs` | `semantic_prefix_without_scope_migrates_to_type_and_empty_scope` | — |
| does not migrate every weekday | 226 | ported | `repo_config.rs` | `schedule_every_weekday_not_migrated` | — |
| does not migrate multi days | 236 | ported | `repo_config.rs` | `schedule_multi_day_expression_not_migrated` | — |
| does not migrate hour range | 247 | ported | `repo_config.rs` | `schedule_compound_non_straddling_not_split` | — |
| migrates packages | 257 | ported | `repo_config.rs` | `deprecated_packages_field_merged_into_package_rules` | — |
| overrides existing automerge setting | 279 | ported | — | — | — |
| does not migrate config | 297 | ported | `repo_config.rs` | `non_deprecated_config_fields_parse_without_migration_effects` | — |
| migrates subconfig | 308 | ported | `migrate_validate.rs` | `migrates_subconfig` | — |
| migrates packageFiles | 334 | ported | — | — | — |
| migrates more packageFiles | 360 | ported | `migrate_validate.rs` | `migrates_more_package_files` | — |
| removes invalid configs | 389 | ported | — | — | — |
| migrates preset strings to array | 419 | ported | `repo_config.rs` | `extends_string_coerced_to_array` (+ extends_string_js_app_shorthand_normalized, extends_mixed_array_js_app_shorthand_normalized) | — |
| migrates unpublishSafe | 441 | ported | `repo_config.rs` | `unpublish_safe_true_injects_minimum_release_age_preset` (+ unpublish_safe_true_with_existing_extends_appends_preset, unpublish_safe_true_with_empty_extends_injects_preset, unpublish_safe_true_with_multiple_extends_appends_preset, unpublish_safe_false_does_not_inject, unpublish_safe_with_unpublish_safe_preset_already_in_extends_does_not_duplicate, unpublish_safe_with_default_unpublish_safe_preset_does_not_duplicate, unpublish_safe_true_with_disabled_preset_still_injects_preset) | — |
| migrates npm:unpublishSafe | 532 | ported | `repo_config.rs` | `extends_npm_unpublish_safe_normalized` (+ extends_npm_unpublish_safe_normalized_after_existing_preset) | — |
| migrates packageRules | 551 | ported | `repo_config.rs` | `migrates_package_rules_all_deprecated_fields` | — |
| migrates in order of precedence | 593 | ported | `repo_config.rs` | `deprecated_match_file_aliases_obey_precedence` | — |

### `config/migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| migrates nested packageRules | 624 | ported | `repo_config.rs` | `nested_package_rules_are_flattened_with_parent_fields` | — |
| migrates presets | 655 | ported | `repo_config.rs` | `migrate_presets_rewrites_extends_and_drops_empty_replacements` | — |
| migrates customManagers | 671 | ported | `repo_config.rs` | `custom_manager_deprecated_lookup_name_fields_migrate` | — |
| migrates pip-compile | 696 | ported | `migrate_validate.rs` | `migrates_pip_compile` | — |
| migrates gradle-lite | 731 | ported | — | — | — |
| migrates empty requiredStatusChecks | 751 | ported | `repo_config.rs` | `empty_required_status_checks_is_removed` | — |
| migrates azureAutoComplete | 762 | ported | `repo_config.rs` | `azure_auto_complete_migrated_to_platform_automerge` | — |
| migrates gitLabAutomerge | 791 | ported | `repo_config.rs` | `git_lab_automerge_migrated_to_platform_automerge` | — |
| migrates dryRun | 820 | ported | `config_builder.rs` | `dry_run_legacy_true_maps_to_full` (+ dry_run_legacy_false_disables_dry_run) | — |
| migrates baseBranches and baseBranch | 835 | ported | `repo_config.rs` | `base_branches_and_base_branch_migrated_to_patterns` | — |
| logs errors | 844 | pending | — | — | —|

---

