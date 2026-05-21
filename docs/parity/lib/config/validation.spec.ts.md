# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/config/validation.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/validation.spec.ts
**Total tests:** 131 | **Ported:** 131 | **Actionable:** 131 | **Status:** ported

### `config/validation › validateConfig(config)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns custom deprecation warnings for %s | 10 | ported | `migrate_validate.rs` | `validate_config_returns_custom_deprecation_warnings` | — |
| returns the deprecationMsg for `dnsCache` as a warning | 26 | ported | `migrate_validate.rs` | `validate_config_warns_for_dns_cache_deprecation` | — |
| allow enabled field in vulnerabilityAlerts | 47 | ported | `migrate_validate.rs` | `validate_config_allows_vulnerability_alerts_enabled` | — |
| catches global options in repo config | 61 | ported | `migrate_validate.rs` | `validate_config_warns_for_global_options_in_repo_config` | — |
| catches global options in inherit config | 86 | ported | `migrate_validate.rs` | `validate_config_warns_for_global_options_in_inherit_config` | — |
| only warns for actual globals in repo config | 107 | ported | `migrate_validate.rs` | `validate_config_ignores_host_rule_credentials` | — |
| does not warn for valid inheritConfig | 124 | ported | `migrate_validate.rs` | `validate_config_allows_inherited_onboarding` | — |
| does not warn for valid platformConfig | 135 | ported | `migrate_validate.rs` | `validate_config_allows_auto_platform_config` | — |
| warns for invalid platformConfig | 147 | ported | `migrate_validate.rs` | `validate_config_errors_for_invalid_platform_config` | — |
| catches invalid templates | 156 | ported | `migrate_validate.rs` | `validate_config_catches_invalid_templates` | — |
| catches invalid jsonata expressions | 165 | ported | `migrate_validate.rs` | `validate_config_catches_invalid_jsonata_expressions` | — |
| catches invalid allowedVersions regex | 179 | ported | `migrate_validate.rs` | `validate_config_catches_invalid_allowed_versions_regex` | — |
| catches invalid matchCurrentValue | 209 | ported | `migrate_validate.rs` | `validate_config_catches_invalid_match_current_value_regex` | — |
| catches invalid matchNewValue | 243 | ported | `migrate_validate.rs` | `validate_config_catches_invalid_match_new_value_regex` | — |
| validates matchBaseBranches | 277 | ported | `migrate_validate.rs` | `validate_config_validates_match_base_branches` | — |
| catches invalid matchBaseBranches when baseBranchPatterns is not defined | 295 | ported | `migrate_validate.rs` | `validate_config_warns_for_match_base_branches_without_base_branch_patterns` | — |
| catches invalid matchCurrentVersion regex | 312 | ported | `migrate_validate.rs` | `validate_config_catches_invalid_match_current_version_regex` | — |
| catches invalid customDatasources content | 347 | ported | `migrate_validate.rs` | `validate_config_catches_invalid_custom_datasources_content` | — |
| validates invalid statusCheckNames | 384 | ported | `migrate_validate.rs` | `validate_config_validates_invalid_status_check_names` | — |
| catches invalid customDatasources record type | 408 | ported | `migrate_validate.rs` | `validate_config_catches_invalid_custom_datasources_record_type` | — |
| catches invalid baseBranchPatterns regex | 423 | ported | `migrate_validate.rs` | `validate_config_catches_invalid_base_branch_patterns_regex` | — |
| returns nested errors | 436 | ported | `migrate_validate.rs` | `validate_config_returns_nested_errors` | — |
| included managers of the wrong type | 466 | ported | `migrate_validate.rs` | `validate_config_errors_for_match_managers_wrong_type` | — |
| empty configuration | 484 | ported | `migrate_validate.rs` | `validate_config_allows_empty_configuration` | — |
| single not supported manager | 503 | ported | `migrate_validate.rs` | `validate_config_errors_for_unsupported_enabled_managers` | — |
| errors for all types | 523 | ported | `migrate_validate.rs` | `validate_config_errors_for_all_types` | — |
| selectors outside packageRules array trigger errors | 558 | ported | `migrate_validate.rs` | `validate_config_errors_for_selectors_outside_package_rules` | — |
| ignore packageRule nesting validation for presets | 588 | ported | `migrate_validate.rs` | `validate_config_ignores_package_rule_nesting_for_presets` | — |
| errors for unsafe managerFilePatterns | 608 | ported | `migrate_validate.rs` | `validate_config_errors_for_unsafe_manager_file_patterns` | — |
| validates regEx for each managerFilePatterns of format regex | 627 | ported | `migrate_validate.rs` | `validate_config_validates_custom_manager_file_pattern_regex` | — |
| errors if customManager has empty managerFilePatterns | 649 | ported | `migrate_validate.rs` | `validate_config_errors_for_empty_custom_manager_file_patterns` | — |
| errors if no customManager customType | 675 | ported | `migrate_validate.rs` | `validate_config_errors_for_missing_custom_manager_type` | — |
| errors if invalid customManager customType | 703 | ported | `migrate_validate.rs` | `validate_config_errors_for_invalid_custom_manager_type` | — |
| errors if empty customManager matchStrings | 732 | ported | `migrate_validate.rs` | `validate_config_errors_for_empty_custom_manager_match_strings` | — |
| errors if no customManager managerFilePatterns | 774 | ported | `migrate_validate.rs` | `validate_config_errors_for_custom_manager_without_manager_file_patterns` | — |
| validates regEx for each matchStrings | 793 | ported | `migrate_validate.rs` | `validate_config_validates_custom_manager_match_string_regex` | — |
| error if no fileFormat in custom JSONata manager | 815 | ported | `migrate_validate.rs` | `validate_config_errors_for_jsonata_manager_missing_file_format` | — |
| validates JSONata query for each matchStrings | 841 | ported | `migrate_validate.rs` | `validate_config_validates_jsonata_manager_queries` | — |
| validates all possible regex manager options | 871 | ported | `migrate_validate.rs` | `validate_config_validates_all_regex_custom_manager_options` | — |
| passes if customManager fields are present | 890 | ported | `migrate_validate.rs` | `validate_config_allows_valid_custom_managers` | — |
| errors if extra customManager fields are present | 922 | ported | `migrate_validate.rs` | `validate_config_errors_for_extra_custom_manager_fields` | — |
| errors if customManager fields are missing | 945 | ported | `migrate_validate.rs` | `validate_config_errors_for_missing_regex_custom_manager_fields` | — |
| errors if customManager fields are missing: JSONataManager | 967 | ported | `migrate_validate.rs` | `validate_config_errors_for_missing_jsonata_custom_manager_fields` | — |
| ignore keys | 1000 | ported | `migrate_validate.rs` | `validate_config_ignores_schema_key` | — |
| validates timezone preset | 1013 | ported | `migrate_validate.rs` | `validate_config_allows_timezone_presets` | — |

### `config/validation › validateConfig(config) › constraints`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| can contain a valid tool name for Containerbase | 1027 | ported | `migrate_validate.rs` | `validate_config_allows_containerbase_constraint_tool` | — |
| can contain a constraint for a non-Containerbase tool | 1042 | ported | `migrate_validate.rs` | `validate_config_allows_non_containerbase_constraint_tool` | — |
| warns if an unsupported constraint is specified | 1057 | ported | `migrate_validate.rs` | `validate_config_warns_for_unsupported_constraint` | — |
| warns if a constraint is not valid | 1079 | ported | `migrate_validate.rs` | `validate_config_warns_for_invalid_constraint_value` | — |
| errors if constraints is a malformed object | 1100 | ported | `migrate_validate.rs` | `validate_config_errors_for_malformed_constraints_object` | — |
| errors if constraints is a malformed array | 1120 | ported | `migrate_validate.rs` | `validate_config_errors_for_malformed_constraints_array` | — |

### `config/validation › validateConfig(config) › constraintsVersioning`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| cannot contain a valid tool name for Containerbase | 1142 | ported | `migrate_validate.rs` | `validate_config_errors_for_containerbase_tool_constraints_versioning` | — |
| can contain a constraint for a non-Containerbase tool | 1164 | ported | `migrate_validate.rs` | `validate_config_allows_non_containerbase_constraints_versioning` | — |
| cannot contain an additional constraint name with an invalid versioning scheme | 1179 | ported | `migrate_validate.rs` | `validate_config_errors_for_invalid_constraints_versioning_scheme` | — |
| can contain an additional constraint name with a regex versioning scheme | 1200 | ported | `migrate_validate.rs` | `validate_config_allows_regex_constraints_versioning_scheme` | — |
| cannot contain an unsupported constraint | 1216 | ported | `migrate_validate.rs` | `validate_config_errors_for_unknown_constraints_versioning_name` | — |
| errors if constraintsVersioning is a malformed object | 1238 | ported | `migrate_validate.rs` | `validate_config_errors_for_malformed_constraints_versioning_object` | — |
| errors if constraintsVersioning is a malformed array | 1260 | ported | `migrate_validate.rs` | `validate_config_errors_for_malformed_constraints_versioning_array` | — |

### `config/validation › validateConfig(config)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| validates object with ignored children | 1281 | ported | `migrate_validate.rs` | `validate_config_allows_object_with_ignored_children` | — |
| validates valid registryAlias objects | 1294 | ported | `migrate_validate.rs` | `validate_config_allows_valid_registry_aliases` | — |
| errors if registryAliases depth is more than 1 | 1309 | ported | `migrate_validate.rs` | `validate_config_errors_for_nested_registry_aliases` | — |
| errors if registryAliases have invalid value | 1331 | ported | `migrate_validate.rs` | `validate_config_errors_for_invalid_registry_alias_value` | — |
| errors if managerFilePatterns has wrong parent | 1352 | ported | `migrate_validate.rs` | `validate_config_warns_for_wrong_manager_file_patterns_parent` | — |
| errors if manager objects are nested | 1395 | ported | `migrate_validate.rs` | `validate_config_errors_for_nested_manager_objects` | — |
| warns if hostType has the wrong parent | 1415 | ported | `migrate_validate.rs` | `validate_config_warns_for_host_type_wrong_parent` | — |
| validates preset values | 1429 | ported | `migrate_validate.rs` | `validate_config_errors_for_non_string_preset_values` | — |
| errors on invalid preset syntax | 1442 | ported | `migrate_validate.rs` | `validate_config_errors_for_invalid_preset_syntax` | — |
| warns if only selectors in packageRules | 1459 | ported | `migrate_validate.rs` | `validate_config_warns_for_selector_only_package_rules` | — |
| errors if invalid combinations in packageRules | 1473 | ported | `migrate_validate.rs` | `validate_config_errors_for_invalid_package_rule_combinations` | — |
| warns when registryUrls is set at the top level of repo config | 1492 | ported | `migrate_validate.rs` | `validate_config_warns_for_top_level_registry_urls` | — |
| warns when defaultRegistryUrls is set at the top level of repo config | 1507 | ported | `migrate_validate.rs` | `validate_config_warns_for_top_level_default_registry_urls` | — |
| warns on nested group packageRules | 1522 | ported | `migrate_validate.rs` | `validate_config_warns_on_nested_group_package_rules` | — |
| does not error on use of `global:` presets in `globalExtends` | 1541 | ported | `migrate_validate.rs` | `validate_config_allows_global_presets_in_global_extends` | — |
| does not error on use of `global:` presets in global `extends` | 1554 | ported | `migrate_validate.rs` | `validate_config_allows_global_presets_in_global_extends_field` | — |
| errors on use of `global:` presets in inherit `extends` | 1567 | ported | `migrate_validate.rs` | `validate_config_errors_for_global_presets_in_inherit_extends` | — |
| errors on use of `global:` presets in repo `extends` | 1580 | ported | `migrate_validate.rs` | `validate_config_errors_for_global_presets_in_repo_extends` | — |
| warns if customEnvVariables are found in repo config | 1594 | ported | `migrate_validate.rs` | `validate_config_warns_for_custom_env_variables_in_repo_config` | — |
| errors if schedule is cron and has no * minutes | 1613 | ported | `migrate_validate.rs` | `validate_config_errors_for_cron_schedule_without_wildcard_minutes` | — |
| errors if invalid matchHost values in hostRules | 1631 | ported | `migrate_validate.rs` | `validate_config_errors_for_invalid_host_rule_match_host_values` | — |
| errors if forbidden header in hostRules | 1673 | ported | `migrate_validate.rs` | `validate_config_errors_for_forbidden_host_rule_header` | — |
| errors if headers values are not string | 1701 | ported | `migrate_validate.rs` | `validate_config_errors_for_non_string_host_rule_header_values` | — |
| errors if allowedHeaders is empty or not defined | 1728 | ported | `migrate_validate.rs` | `validate_config_errors_for_headers_without_allowed_headers` | — |
| catches invalid variable name in env config option | 1755 | ported | `migrate_validate.rs` | `validate_config_catches_invalid_env_variable_name_and_value` | — |
| catches env config option if configured inside a parent | 1783 | ported | `migrate_validate.rs` | `validate_config_catches_nested_env_config` | — |
| catches when * or ** is combined with others patterns in a regexOrGlob option | 1820 | ported | `migrate_validate.rs` | `validate_config_catches_match_all_combined_with_other_patterns` | — |
| catches when negative number is used for integer type | 1848 | ported | `migrate_validate.rs` | `validate_config_catches_negative_integer_options` | — |
| validates prPriority | 1862 | ported | `migrate_validate.rs` | `validate_config_allows_negative_pr_priority` | — |
| errors if no bumpVersion filePattern is provided | 1883 | ported | `migrate_validate.rs` | `validate_config_errors_for_bump_version_without_file_patterns` | — |
| errors if no matchStrings are provided for bumpVersion | 1909 | ported | `migrate_validate.rs` | `validate_config_errors_for_bump_version_without_match_strings` | — |
| allow bumpVersion | 1933 | ported | `migrate_validate.rs` | `validate_config_matches_upstream_bump_version_allow_case` | — |

### `config/validation › validateConfig() -> globaOnly options`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns errors for invalid options | 1959 | ported | `migrate_validate.rs` | `validate_config_global_errors_for_invalid_options` | — |
| validates hostRules.headers | 1981 | ported | `migrate_validate.rs` | `validate_config_global_validates_host_rule_headers` | — |
| errors if hostRules.headers is defined but allowedHeaders is not | 2001 | ported | `migrate_validate.rs` | `validate_config_global_errors_for_headers_without_allowed_headers` | — |
| validates env | 2025 | ported | `migrate_validate.rs` | `validate_config_global_validates_env` | — |
| handles prefixed onboardingConfigFileName | 2040 | ported | `migrate_validate.rs` | `validate_config_global_allows_prefixed_onboarding_config_file_name` | — |
| allows unique onboardingConfigFileName if it is set in configFileNames | 2054 | ported | `migrate_validate.rs` | `validate_config_global_allows_unique_onboarding_config_file_name_in_config_file_names` | — |
| errors if env object is defined but allowedEnv is empty or undefined | 2067 | ported | `migrate_validate.rs` | `validate_config_global_errors_for_env_without_allowed_env` | — |
| validates env against the allowedEnv regex | 2086 | ported | `migrate_validate.rs` | `validate_config_global_validates_env_against_allowed_env_regex` | — |
| validates options with different type but defaultValue=null | 2101 | ported | `migrate_validate.rs` | `validate_config_allows_default_null_options` | — |

### `config/validation › validate globalOptions()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| binarySource=docker is deprecated | 2137 | ported | `migrate_validate.rs` | `validate_config_global_warns_for_deprecated_docker_binary_source` | — |
| binarySource | 2154 | ported | `migrate_validate.rs` | `validate_config_global_warns_for_invalid_binary_source` | — |

### `config/validation › validate globalOptions() › validates string type options`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| binarySource | 2172 | ported | `migrate_validate.rs` | `validate_config_global_string_options_binary_source` | — |
| baseDir | 2189 | ported | `migrate_validate.rs` | `validate_config_global_string_options_base_dir` | — |
| requireConfig | 2205 | ported | `migrate_validate.rs` | `validate_config_global_string_options_require_config` | — |
| dryRun | 2222 | ported | `migrate_validate.rs` | `validate_config_global_string_options_dry_run` | — |
| repositoryCache | 2239 | ported | `migrate_validate.rs` | `validate_config_global_string_options_repository_cache` | — |
| onboardingConfigFileName | 2256 | ported | `migrate_validate.rs` | `validate_config_global_string_options_onboarding_config_file_name` | — |
| onboardingConfig | 2272 | ported | `migrate_validate.rs` | `validate_config_global_string_options_onboarding_config` | — |
| force | 2299 | ported | `migrate_validate.rs` | `validate_config_global_string_options_force` | — |
| gitUrl | 2324 | ported | `migrate_validate.rs` | `validate_config_global_string_options_git_url` | — |

### `config/validation › validate globalOptions()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| validates boolean type options | 2343 | ported | `migrate_validate.rs` | `validate_config_global_validates_boolean_type_options` | — |
| validates integer type options | 2363 | ported | `migrate_validate.rs` | `validate_config_global_validates_integer_type_options` | — |
| validates array type options | 2383 | ported | `migrate_validate.rs` | `validate_config_global_validates_array_type_options` | — |
| validates object type options | 2414 | ported | `migrate_validate.rs` | `validate_config_global_validates_object_type_options` | — |
| warns if negative number is used for integer type | 2444 | ported | `migrate_validate.rs` | `validate_config_global_warns_for_negative_integer_options` | — |
| warns on invalid customEnvVariables objects | 2461 | ported | `migrate_validate.rs` | `validate_config_global_warns_for_invalid_custom_env_variables` | — |
| validates valid customEnvVariables objects | 2482 | ported | `migrate_validate.rs` | `validate_config_global_allows_valid_custom_env_variables` | — |
| validates options with different type but defaultValue=null | 2497 | ported | `migrate_validate.rs` | `validate_config_global_allows_default_null_options` | — |
| fails for missing reportPath if reportType is "s3" | 2517 | ported | `migrate_validate.rs` | `validate_config_global_errors_for_missing_s3_report_path` | — |
| validates reportPath if reportType is "s3" | 2529 | ported | `migrate_validate.rs` | `validate_config_global_allows_s3_report_path` | — |
| fails for missing reportPath if reportType is "file" | 2542 | ported | `migrate_validate.rs` | `validate_config_global_errors_for_missing_file_report_path` | — |
| validates reportPath if reportType is "file" | 2554 | ported | `migrate_validate.rs` | `validate_config_global_allows_file_report_path` | — |
| warns when registryUrls is set at the top level of global config | 2567 | ported | `migrate_validate.rs` | `validate_config_global_warns_for_top_level_registry_urls` | — |
| warns when defaultRegistryUrls is set at the top level of global config | 2582 | ported | `migrate_validate.rs` | `validate_config_global_warns_for_top_level_default_registry_urls` | — |
| validates postUpgradeTasks.installTools tool names | 2597 | ported | `migrate_validate.rs` | `validate_config_global_validates_post_upgrade_install_tools` | — |
| rejects invalid postUpgradeTasks.installTools tool names | 2615 | ported | `migrate_validate.rs` | `validate_config_global_rejects_invalid_post_upgrade_install_tools` | — |
| catches when * or ** is combined with others patterns in a regexOrGlob option | 2639 | ported | `migrate_validate.rs` | `validate_config_global_catches_match_all_combined_with_other_patterns` | — |

| accepts templates referencing runtime-only fields | 165 | ported | `migrate_validate.rs` | `validate_config_accepts_templates_referencing_runtime_only_fields` | — |
| skips preset syntax validation for templates | 1472 | ported | `migrate_validate.rs` | `validate_config_skips_preset_syntax_validation_for_templates` | — |
| errors when using an invalid cache namespace | 2706 | ported | `migrate_validate.rs` | `validate_config_errors_for_invalid_cache_namespace` | — |
| allows a valid cache namespace | 2729 | ported | `migrate_validate.rs` | `validate_config_allows_valid_cache_namespace` | — |
---

## Config specs (`lib/config/`)

| Renovate spec file | Renovate tests | Rust file | Rust tests | Status |
|--------------------|---------------|-----------|------------|--------|
<!-- config/defaults.spec.ts converted to per-test format above -->
<!-- config/app-strings.spec.ts converted to per-test format above -->
<!-- config/parse.spec.ts converted to per-test format above -->
<!-- config/global.spec.ts converted to per-test format above -->
<!-- config/validation.spec.ts converted to per-test format above -->
<!-- config/migration.spec.ts converted to per-test format above -->
<!-- config/migrate-validate.spec.ts converted to per-test format above -->
<!-- config/massage.spec.ts converted to per-test format above -->
<!-- config/secrets.spec.ts converted to per-test format above -->
<!-- config/inherit.spec.ts converted to per-test format above -->
<!-- config/index.spec.ts converted to per-test format above -->
<!-- config/decrypt.spec.ts converted to per-test format above -->

---

