# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/package-rules/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/package-rules/index.spec.ts
**Total tests:** 73 | **Ported:** 62 | **Actionable:** 62 | **Status:** ported

### `util/package-rules/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| applies | 38 | ported | `repo_config.rs` | `applies_comprehensive_integration` | — |
| applies both rules for a | 71 | ported | `repo_config.rs` | `package_rules_index_fixture_name_matching_cases` | — |
| applies both rules for b | 81 | ported | `repo_config.rs` | `package_rules_index_fixture_name_matching_cases` | — |
| applies the second rule | 91 | ported | `repo_config.rs` | `package_rules_index_fixture_name_matching_cases` | — |
| applies matchPackageNames | 101 | ported | `repo_config.rs` | `package_rules_index_fixture_name_matching_cases` | — |
| applies the second second rule | 109 | ported | `repo_config.rs` | `package_rules_index_fixture_name_matching_cases` | — |
| excludes package name | 118 | ported | `repo_config.rs` | `package_rules_index_fixture_name_matching_cases` | — |
| excludes package pattern | 127 | ported | `repo_config.rs` | `package_rules_index_fixture_name_matching_cases` | — |
| ignores patterns if lock file maintenance | 136 | not-applicable | — | — | Rust update-type model does not represent Renovate's lockFileMaintenance artifact-maintenance flow |
| do apply rule with matchPackageName | 152 | not-applicable | — | — | Rust update-type model does not represent Renovate's lockFileMaintenance artifact-maintenance flow |
| sets skipReason=package-rules if enabled=false | 169 | ported | `repo_config.rs` | `enabled_false_rule_blocks_dependency` | Rust tracks the equivalent blocked state, not the worker-layer skipReason fields |
| unsets skipReason=package-rules if enabled=true | 184 | ported | `repo_config.rs` | `enabled_true_later_rule_overrides_earlier_enabled_false` | Rust tracks the equivalent unblocked state, not the worker-layer skipReason fields |
| does not set skipReason=package-rules if the last packageRule has force.enabled=true | 202 | ported | `repo_config.rs` | `force_enabled_true_overrides_enabled_false` | Rust tracks the equivalent unblocked state, not the worker-layer skipReason fields |
| does not set skipReason=package-rules if the last packageRule has force.enabled=true (if config.enabled=false) | 223 | not-applicable | — | — | Rust package-rule effects do not model Renovate worker skipReason/skipStage output or config-level disabled dependency objects |
| does not set skipReason=package-rules if the last packageRule has enabled=true (if config.force.enabled=false) | 245 | ported | `repo_config.rs` | `force_enabled_true_on_ctx_clears_block` | Rust verifies the equivalent merged force.enabled effect |
| sets skipReason=package-rules if the last packageRule has force.enabled=false (if config.force.enabled=false) | 267 | not-applicable | — | — | Rust package-rule effects do not model Renovate worker skipReason/skipStage output or config-level force dependency objects |
| sets skipReason=package-rules if the last packageRule has force.enabled=false | 292 | ported | `repo_config.rs` | `force_enabled_false_overrides_enabled_true` | Rust tracks the equivalent blocked state, not the worker-layer skipReason fields |
| skips skipReason=package-rules if enabled=true | 312 | not-applicable | — | — | Rust package-rule effects do not model Renovate worker skipReason/skipStage emission |
| matches anything if missing inclusive rules | 326 | ported | `repo_config.rs` | `match_package_names_negation` | — |
| supports inclusive or | 348 | ported | `repo_config.rs` | `match_package_names_supports_inclusive_or` | — |
| filters requested depType | 370 | ported | `repo_config.rs` | `match_dep_types_multiple_types_in_list` | — |
| filters from list of requested depTypes | 389 | ported | `repo_config.rs` | `match_dep_types_plural_array_any_matches` | — |
| returns false if no depTypes | 408 | ported | `repo_config.rs` | `match_dep_types_no_dep_type_rule_does_not_fire` | — |
| filters managers with matching manager | 426 | ported | `repo_config.rs` | `match_managers_matching_manager_applies_rule` | — |
| filters managers with non-matching manager | 446 | ported | `repo_config.rs` | `match_managers_non_matching_manager_skips_rule` | — |
| filters categories with matching category | 468 | ported | `repo_config.rs` | `match_categories_dep_provided_categories_override_manager_derived` | — |
| filters categories with non-matching category | 489 | ported | `repo_config.rs` | `match_categories_dep_provided_categories_non_matching` | — |
| filters categories with undefined category | 510 | ported | `repo_config.rs` | `needs_categories_to_match_rule_does_not_fire_without_it` | — |
| filters datasources with matching datasource | 529 | ported | `repo_config.rs` | `match_datasources_matching_datasource_applies_rule` | — |
| filters branches with matching branch | 554 | ported | `repo_config.rs` | `match_base_branches_multiple_entries` | — |
| filters datasources with non-matching datasource | 573 | ported | `repo_config.rs` | `match_datasources_missing_datasource_skips_rule` | — |
| filters branches with non-matching branch | 591 | ported | `repo_config.rs` | `match_base_branches_multiple_entries` | — |
| filters branches with matching branch regex | 609 | ported | `repo_config.rs` | `match_base_branches_regex_matches_release_branch_only` | — |
| filters branches with non-matching branch regex | 628 | ported | `repo_config.rs` | `match_base_branches_regex_matches_release_branch_only` | — |
| filters updateType | 647 | ported | `repo_config.rs` | `match_update_types_patch_matches_patch_minor_rule_only` | — |
| matches matchSourceUrls with glob | 672 | ported | `repo_config.rs` | `match_source_urls_with_double_star_glob` | — |
| non-matches matchSourceUrls with globs | 695 | ported | `repo_config.rs` | `match_source_urls_with_double_star_glob` | — |
| handles matchSourceUrls when missing sourceUrl | 718 | ported | `repo_config.rs` | `match_source_urls_missing_returns_false` | — |
| matches matchSourceUrls | 740 | ported | `repo_config.rs` | `match_source_urls_exact_disables_dep` | — |
| non-matches matchSourceUrls | 763 | ported | `repo_config.rs` | `match_source_urls_exact_disables_dep` | — |
| handles matchRegistryUrls when missing registryUrls | 786 | ported | `repo_config.rs` | `match_registry_urls_no_dep_urls_fails_when_constraint_set` | — |
| matches matchRegistryUrls | 808 | ported | `repo_config.rs` | `match_registry_urls_exact_hit` | — |
| non-matches matchRegistryUrls | 831 | ported | `repo_config.rs` | `match_registry_urls_exact_hit` | — |

### `util/package-rules/index › matchConfidence`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| matches matchConfidence | 865 | not-applicable | — | — | Rust core does not implement Renovate's merge-confidence service matcher or hostRules authentication path |
| non-matches matchConfidence | 884 | not-applicable | — | — | Rust core does not implement Renovate's merge-confidence service matcher or hostRules authentication path |
| does not match matchConfidence when there is no mergeConfidenceLevel | 903 | not-applicable | — | — | Rust core does not implement Renovate's merge-confidence service matcher or hostRules authentication path |
| throws when unauthenticated | 922 | not-applicable | — | — | Rust core does not implement Renovate's merge-confidence service matcher or hostRules authentication path |

### `util/package-rules/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| filters naked depType | 950 | ported | `repo_config.rs` | `match_dep_types_naked_dep_type_matches` | — |
| filters out unrequested depType | 968 | ported | `repo_config.rs` | `match_dep_types_out_of_requested_does_not_match` | — |
| checks if matchCurrentVersion selector is valid and satisfies the condition on range overlap | 987 | ported | `repo_config.rs` | `match_current_version_range_uses_current_version_field` | — |
| checks if matchCurrentVersion selector is valid and satisfies the condition on pinned to range overlap | 1026 | ported | `repo_config.rs` | `match_current_version_index_spec_pinned_satisfies_range` | — |
| checks if matchCurrentVersion selector is a version and matches if currentValue is a range | 1049 | ported | `repo_config.rs` | `match_current_version_index_spec_version_matches_range` | — |
| checks if matchCurrentVersion selector works with static values | 1079 | ported | `repo_config.rs` | `match_current_version_index_spec_static_value` | — |
| checks if matchCurrentVersion selector works with regular expressions | 1101 | ported | `repo_config.rs` | `match_current_version_index_spec_regex_matches` | — |
| checks if matchCurrentVersion selector works with negated regular expressions | 1132 | ported | `repo_config.rs` | `match_current_version_index_spec_negated_regex` | — |
| matches packageFiles | 1163 | ported | `repo_config.rs` | `match_file_names_exact_match` | — |
| matches lock files | 1187 | ported | `repo_config.rs` | `match_file_names_matches_lock_files` | — |
| matches paths | 1203 | ported | `repo_config.rs` | `match_file_names_matches_paths` | — |
| empty rules | 1233 | ported | `repo_config.rs` | `package_rules_null_is_treated_as_empty_rules` | — |
| creates groupSlug if necessary | 1242 | ported | `repo_config.rs` | `group_slug_auto_generated_from_group_name_when_prior_slug_exists` | — |
| matches matchSourceUrls with patterns (case-insensitive) | 1261 | ported | `repo_config.rs` | `match_source_urls_case_insensitive` | — |
| matches matchSourceUrls(case-insensitive) | 1284 | ported | `repo_config.rs` | `match_source_urls_case_insensitive` | — |
| needs language to match | 1307 | ported | `repo_config.rs` | `needs_categories_to_match_rule_does_not_fire_without_it` | — |
| needs baseBranch to match | 1325 | ported | `repo_config.rs` | `needs_base_branch_to_match_rule_does_not_fire_without_it` | — |
| needs manager to match | 1343 | ported | `repo_config.rs` | `needs_manager_to_match_rule_does_not_fire_without_it` | — |
| matches matchDepNames(depName) | 1361 | ported | `repo_config.rs` | `match_dep_names_exact_disables_dep` | — |
| matches if there are no matchers | 1386 | ported | `repo_config.rs` | `package_rule_without_matchers_applies_to_any_dep` | — |
| overrides | 1404 | not-applicable | — | — | Rust package-rule effects do not mutate dependency identity/datasource through Renovate overrideDepName/overridePackageName/overrideDatasource |
| overrides with templates | 1447 | not-applicable | — | — | Rust package-rule effects do not mutate dependency identity through Renovate overrideDepName templates |
| propagates fetchChangeLogs from matching packageRule | 1464 | ported | `repo_config.rs` | `package_rule_fetch_change_logs_applies_when_rule_matches` | — |
| does not set fetchChangeLogs when packageRule does not match | 1479 | ported | `repo_config.rs` | `package_rule_fetch_change_logs_skipped_when_rule_does_not_match` | — |
| compiles sourceUrl with template helper functions | 1494 | ported | `repo_config.rs` | `package_rule_source_url_template_replace_helper` | — |
| compiles sourceUrl with template variables | 1513 | ported | `repo_config.rs` | `package_rule_source_url_template_package_name_variable` | — |

---

