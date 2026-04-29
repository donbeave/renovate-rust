# Renovate Test Map

Maps Renovate (TypeScript/vitest) test cases to equivalent Rust test cases in
this workspace. Use this file to (a) see which Renovate behaviors have Rust
coverage and (b) plan which Renovate tests to port next.

## Mapping philosophy

This is **not** a one-to-one structural copy. Logical equivalence is the goal:
- One Renovate `it()` may map to multiple Rust `#[test]` functions when splitting
  into focused tests improves clarity.
- Multiple Renovate `it()` blocks may collapse into one Rust test when the
  fixture difference is trivial.
- Test organization follows Rust module conventions, not the Renovate file tree.
- Fixtures are recreated as Rust literals or inline strings, not copied verbatim.

**Status values:** `ported` · `partial` · `pending` · `not-applicable`

---

## CLI flags and argument parsing

| Renovate test file | Line | Renovate test | Rust location | Rust test | Status |
|--------------------|------|---------------|---------------|-----------|--------|
| `lib/workers/global/config/parse/cli.spec.ts` | 22 | generates returns empty if CLI false | `crates/renovate-cli/tests/cli.rs` | `version_long_flag_prints_bare_version`, `version_short_flag_matches_long_flag` | ported |
| `lib/workers/global/config/parse/cli.spec.ts` | — | `--help` shows help text | `crates/renovate-cli/tests/cli.rs` | `help_flag_succeeds_and_mentions_repositories` | partial |
| `lib/workers/global/config/parse/cli.spec.ts` | 125–208 | `migrateArgs` rewrites legacy flags | `crates/renovate-cli/src/migrate.rs` | 22 migration test cases + `git_fs_legacy_flags_are_silently_dropped` | ported |
| `lib/workers/global/config/parse/cli.spec.ts` | — | `parseEarlyFlags` ignores unknown flags | — | — | pending |

---

## Config discovery (`repo_config.rs`)

| Renovate test file | Line | Renovate test | Rust location | Rust test | Status |
|--------------------|------|---------------|---------------|-----------|--------|
| `lib/config/app-strings.ts` | — | `configFileNames` list | `crates/renovate-core/src/repo_config.rs` | `CONFIG_FILE_CANDIDATES` array | ported |
| `lib/workers/repository/init/merge.ts` | — | `detectConfigFile` finds `renovate.json` first | `crates/renovate-core/src/repo_config.rs` | `finds_renovate_json_first` (async) | ported |
| `lib/workers/repository/init/merge.ts` | — | NeedsOnboarding when no config and `requireConfig=Required` | `crates/renovate-core/src/repo_config.rs` | `returns_needs_onboarding_when_no_config_and_required` | ported |
| `lib/workers/repository/init/merge.ts` | — | NotFound when no config and `requireConfig=Optional` | `crates/renovate-core/src/repo_config.rs` | `returns_not_found_when_optional` | ported |
| `lib/workers/repository/init/merge.ts` | — | `package.json` `renovate` key used when dedicated files absent | `crates/renovate-core/src/repo_config.rs` | `discovers_renovate_key_in_package_json`, `package_json_without_renovate_key_triggers_onboarding` | ported |
| `lib/workers/repository/init/merge.ts` | — | `package.json` without `renovate` key → not a config source | `crates/renovate-core/src/repo_config.rs` | `parse_from_package_json_returns_none_when_no_key` | ported |

---

## `RepoConfig` field parsing

| Renovate test file | Line | Renovate test | Rust location | Rust test | Status |
|--------------------|------|---------------|---------------|-----------|--------|
| `lib/config/defaults.ts` | — | `enabled` defaults to `true` | `crates/renovate-core/src/repo_config.rs` | `defaults_when_empty` | ported |
| `lib/config/defaults.ts` | — | `branchPrefix` default `"renovate/"` | `crates/renovate-core/src/repo_config.rs` | `branch_prefix_default` | ported |
| `lib/config/defaults.ts` | — | `prHourlyLimit` default `2` | `crates/renovate-core/src/repo_config.rs` | `pr_hourly_limit_default` | ported |
| `lib/config/defaults.ts` | — | `separateMajorMinor` default `true` | `crates/renovate-core/src/repo_config.rs` | `separate_major_minor_default_true` | ported |
| `lib/config/defaults.ts` | — | `separateMinorPatch` default `false` | `crates/renovate-core/src/repo_config.rs` | `separate_minor_patch_default_false` | ported |
| `lib/config/options/index.ts` | — | `schedule`, `timezone` config fields | `crates/renovate-core/src/repo_config.rs` | `schedule_parsed_into_repo_config`, `timezone_parsed` | ported |
| `lib/config/options/index.ts` | — | `automerge` defaults false | `crates/renovate-core/src/repo_config.rs` | `automerge_defaults_false`, `automerge_parsed_true` | ported |
| `lib/config/options/index.ts` | — | `labels`, `assignees`, `reviewers` | `crates/renovate-core/src/repo_config.rs` | `labels_parsed`, `reviewers_and_assignees_parsed` | ported |
| `lib/config/options/index.ts` | — | `groupName` at repo and rule level | `crates/renovate-core/src/repo_config.rs` | `group_name_parsed_at_repo_level`, `package_rule_group_name_parsed` | ported |
| `lib/config/options/index.ts` | — | `baseBranches` list | `crates/renovate-core/src/repo_config.rs` | `base_branches_parsed` | ported |
| `lib/config/options/index.ts` | — | `additionalBranchPrefix` parsed + wired to branch_name | `crates/renovate-core/src/repo_config.rs` | `additional_branch_prefix_default_empty`, `additional_branch_prefix_parsed` | ported |
| `lib/config/options/index.ts` | — | `labels` + `addLabels` seed rule effects | `crates/renovate-core/src/repo_config.rs` | `repo_level_labels_seed_effects`, `add_labels_merged_with_labels`, `rule_labels_append_to_repo_labels` | ported |

---

## `packageRules` matchers

| Renovate test file | Line | Renovate test | Rust location | Rust test | Status |
|--------------------|------|---------------|---------------|-----------|--------|
| `lib/util/package-rules/index.spec.ts` | — | `matchPackageNames` exact | `crates/renovate-core/src/repo_config.rs` | `enabled_managers_parsed` | partial |
| `lib/util/package-rules/index.spec.ts` | — | `matchPackageNames` regex `/pattern/` | `crates/renovate-core/src/repo_config.rs` | `match_dep_names_regex_disables_dep` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `matchPackageNames` glob `@aws-sdk/**` | `crates/renovate-core/src/repo_config.rs` | `match_dep_names_glob_disables_dep` | ported |
| `lib/util/package-rules/dep-names.ts` | — | `matchDepNames` matches `depName` | `crates/renovate-core/src/repo_config.rs` | `match_dep_names_exact_disables_dep`, `match_dep_names_regex_disables_dep`, `match_dep_names_glob_disables_dep` | ported |
| `lib/util/package-rules/dep-names.spec.ts` | — | `matchDepNames` negation regex `!/^@opentelemetry/` | `crates/renovate-core/src/repo_config.rs` | `match_dep_names_negation_regex` | ported |
| `lib/util/package-rules/dep-names.spec.ts` | — | `matchDepNames` negation glob `!@opentelemetry/**` | `crates/renovate-core/src/repo_config.rs` | `match_dep_names_negation_glob` | ported |
| `lib/util/package-rules/dep-names.spec.ts` | — | `matchDepNames` positive regex include | `crates/renovate-core/src/repo_config.rs` | `match_dep_names_regex_includes` | ported |
| `lib/util/package-rules/repositories.spec.ts` | — | `matchRepositories` negation glob | `crates/renovate-core/src/repo_config.rs` | `match_repositories_negation` | ported |
| `lib/util/package-rules/sourceurls.ts` | — | `matchSourceUrls` exact | `crates/renovate-core/src/repo_config.rs` | `source_url_tests::match_source_urls_exact_disables_dep` | ported |
| `lib/util/package-rules/sourceurls.ts` | — | `matchSourceUrls` glob | `crates/renovate-core/src/repo_config.rs` | `source_url_tests::match_source_urls_glob` | ported |
| `lib/util/package-rules/sourceurls.ts` | — | `matchSourceUrls` regex | `crates/renovate-core/src/repo_config.rs` | `source_url_tests::match_source_urls_regex` | ported |
| `lib/util/package-rules/sourceurls.ts` | — | empty `matchSourceUrls` matches all | `crates/renovate-core/src/repo_config.rs` | `source_url_tests::match_source_urls_empty_matches_all` | ported |
| `lib/util/package-rules/current-value.ts` | — | `matchCurrentValue` regex match | `crates/renovate-core/src/repo_config.rs` | `source_url_tests::match_current_value_regex` | ported |
| `lib/util/package-rules/current-value.ts` | — | `matchCurrentValue` exact match | `crates/renovate-core/src/repo_config.rs` | `source_url_tests::match_current_value_exact` | ported |
| `lib/util/package-rules/current-value.spec.ts` | — | `matchCurrentValue` case-insensitive `/^"v/i` | `crates/renovate-core/src/repo_config.rs` | `match_current_value_regex_with_flags` | ported |
| `lib/util/package-rules/current-value.spec.ts` | — | `matchCurrentValue` glob match `1.2.*` | `crates/renovate-core/src/repo_config.rs` | `source_url_tests::match_current_value_regex` | ported |
| `lib/util/package-rules/new-value.ts` | — | `matchNewValue` glob match | `crates/renovate-core/src/repo_config.rs` | `source_url_tests::match_new_value_glob` | ported |
| `lib/util/package-rules/new-value.spec.ts` | — | `matchNewValue` exact, glob, regex, flags | `crates/renovate-core/src/repo_config.rs` | `source_url_tests::match_new_value_glob`, `match_current_value_regex_with_flags` | ported |
| `lib/util/package-rules/current-version.spec.ts` | — | `matchCurrentVersion` regex against currentValue | `crates/renovate-core/src/repo_config.rs` | `match_current_version_regex_against_current_value` | ported |
| `lib/util/package-rules/current-version.spec.ts` | — | `matchCurrentVersion` semver range match | `crates/renovate-core/src/repo_config.rs` | `match_current_version_blocks_when_below_range` | ported |
| `lib/util/package-rules/current-version.spec.ts` | — | `matchCurrentVersion` caret range + currentValue | `crates/renovate-core/src/repo_config.rs` | `match_current_version_with_caret_range_current` | ported |
| `lib/util/package-rules/current-version.spec.ts` | — | `matchCurrentVersion` absent → matches all | `crates/renovate-core/src/repo_config.rs` | `match_current_version_absent_matches_all` | ported |
| `lib/util/package-rules/current-version.spec.ts` | — | `matchCurrentVersion` negated regex `!/^0/` | `crates/renovate-core/src/repo_config.rs` | `match_current_version_negated_regex` | ported |
| `lib/util/package-rules/current-version.spec.ts` | — | `matchCurrentVersion` pep440 / ruby / same-major versioning | — | (not fully ported — versioning schemes not in scope) | pending |
| `lib/util/package-rules/current-version.spec.ts` | — | regex matched against `lockedVersion` when set | `crates/renovate-core/src/package_rule.rs` | `match_current_version_regex_prefers_locked_version`, `match_current_version_via_dep_context_with_locked_version` | ported |
| `lib/util/package-rules/current-version.spec.ts` | — | regex returns false when lockedVersion absent + currentValue is non-version | `crates/renovate-core/src/repo_config.rs` | `match_current_version_regex_false_without_locked_version` | ported |
| `lib/util/package-rules/files.spec.ts` | — | `matchFileNames` undefined packageFile → false | `crates/renovate-core/src/repo_config.rs` | `match_file_names_blocks_matching_path` | ported |
| `lib/util/package-rules/package-names.spec.ts` | — | `matchPackageNames` negation | `crates/renovate-core/src/repo_config.rs` | `match_package_names_negation`, `match_package_names_glob_negation` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `matchDatasources` list | `crates/renovate-core/src/repo_config.rs` | `match_datasources_method_matches_listed_datasource`, `match_datasources_empty_matches_all` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `matchUpdateTypes` major/minor/patch | `crates/renovate-core/src/repo_config.rs` | `is_update_blocked_for_major_but_not_minor` | ported |
| `lib/util/package-rules/index.spec.ts` | 42 | `matchUpdateTypes: ['bump']` with `isBump: true` → rule applies | `crates/renovate-core/src/repo_config.rs` | `is_bump_matches_bump_update_type_rule`, `match_update_types_bump_parses` | ported |
| `lib/util/package-rules/update-types.ts` | — | `isBump: true` adds virtual `'bump'` update type | `crates/renovate-core/src/package_rule.rs` | `update_type_matches(ut, is_bump)` logic | ported |
| `lib/util/package-rules/index.spec.ts` | — | `matchFileNames` glob | `crates/renovate-core/src/repo_config.rs` | `is_update_blocked_for_file_with_filename_rule` | ported |
| `lib/util/package-rules/managers.spec.ts` | — | `matchManagers` exact match | `crates/renovate-core/src/repo_config.rs` | `package_rules_match_managers_respected` | ported |
| `lib/util/package-rules/managers.spec.ts` | — | `matchManagers` null for undefined matchManagers | `crates/renovate-core/src/repo_config.rs` | `package_rules_match_managers_respected` | ported |
| `lib/util/package-rules/managers.spec.ts` | — | `matchManagers` custom manager `custom.regex` | `crates/renovate-core/src/repo_config.rs` | `match_managers_custom_prefix` | ported |
| `lib/util/package-rules/managers.spec.ts` | — | `matchManagers` glob pattern | `crates/renovate-core/src/repo_config.rs` | `match_managers_glob_pattern` | ported |
| `lib/util/package-rules/managers.spec.ts` | — | `matchManagers` regex pattern | `crates/renovate-core/src/repo_config.rs` | `match_managers_regex_pattern` | ported |
| `lib/util/package-rules/managers.spec.ts` | — | `matchManagers` negation `!pattern` | `crates/renovate-core/src/repo_config.rs` | `match_managers_negation` | ported |
| `lib/util/package-rules/managers.spec.ts` | — | undefined manager + `matchManagers` set → rule doesn't fire | `crates/renovate-core/src/repo_config.rs` | `match_managers_no_manager_no_rule_fire` | ported |
| `lib/util/package-rules/managers.spec.ts` | — | `matchManagers` absent → fires for any manager | `crates/renovate-core/src/repo_config.rs` | `match_managers_undefined_rule_fires_for_any_manager` | ported |
| `lib/util/package-rules/managers.spec.ts` | — | legacy `manager:'regex'` matches `matchManagers:['custom.regex']` | `crates/renovate-core/src/repo_config.rs` | `match_managers_legacy_regex_matches_custom_regex_rule` | ported |
| `lib/util/string-match.ts` | — | `matchRegexOrGlob` exact | `crates/renovate-core/src/string_match.rs` | `exact_match` | ported |
| `lib/util/string-match.ts` | — | `matchRegexOrGlob` regex `/pattern/` | `crates/renovate-core/src/string_match.rs` | `regex_pattern_match` | ported |
| `lib/util/string-match.ts` | — | `matchRegexOrGlob` regex `/pattern/flags` | `crates/renovate-core/src/string_match.rs` | `regex_pattern_with_flags_match` | ported |
| `lib/util/string-match.ts` | — | `matchRegexOrGlob` glob `npm*` | `crates/renovate-core/src/string_match.rs` | `glob_star_prefix` | ported |
| `lib/util/string-match.ts` | — | `matchRegexOrGlobList` empty → false | `crates/renovate-core/src/string_match.rs` | `empty_list_returns_false` | ported |
| `lib/util/string-match.ts` | — | `matchRegexOrGlobList` positive list | `crates/renovate-core/src/string_match.rs` | `positive_list_matches` | ported |
| `lib/util/string-match.ts` | — | `matchRegexOrGlobList` negation `!pattern` | `crates/renovate-core/src/string_match.rs` | `negation_excludes_input`, `all_negative_patterns_allow_non_matching` | ported |
| `lib/util/string-match.spec.ts` | — | `matchRegexOrGlobList` case-insensitive glob `'TEST'` vs `'t*'` → true | `crates/renovate-core/src/string_match.rs` | `glob_is_case_insensitive_matching_renovate_nocase` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `matchSourceUrls` case-insensitive URL matching (bare string) | `crates/renovate-core/src/string_match.rs` | `exact_match_is_case_insensitive` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `matchSourceUrls` with glob patterns (case-insensitive) | `crates/renovate-core/src/repo_config.rs` | `source_url_tests::match_source_urls_glob` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `matchSourceUrls` missing sourceUrl → rule doesn't fire | `crates/renovate-core/src/repo_config.rs` | `source_url_tests::match_source_urls_empty_matches_all` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `matchFileNames: ['yarn.lock']` matches `lockFiles: ['yarn.lock']` | `crates/renovate-core/src/repo_config.rs` | `match_file_names_matches_lock_files`, `match_file_names_lock_file_pattern_with_glob` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `force.enabled: true` overrides `enabled: false` | `crates/renovate-core/src/repo_config.rs` | `force_enabled_true_overrides_enabled_false`, `force_enabled_true_also_overrides_config_level_disabled` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `overridePackageName` / `overrideDepName` / `overrideDatasource` | — | (not implemented — override fields not supported) | pending |
| `lib/util/package-rules/index.spec.ts` | — | Handlebars template in `overrideDepName` (`{{replace}}`) | — | (not implemented — template engine not integrated) | pending |
| `lib/util/package-rules/index.spec.ts` | — | `matchBaseBranches` filters by base branch | `crates/renovate-core/src/repo_config.rs` | `categories_base_branch_tests::match_base_branches_exact_hit` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `matchCategories` filters by category | `crates/renovate-core/src/repo_config.rs` | `categories_base_branch_tests::match_categories_exact_hit` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `matchCategories` undefined categories → rule doesn't fire | `crates/renovate-core/src/repo_config.rs` | `categories_base_branch_tests::match_categories_empty_matches_all`, `needs_categories_to_match_rule_does_not_fire_without_it` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `matchBaseBranches` absent baseBranch → rule doesn't fire | `crates/renovate-core/src/repo_config.rs` | `needs_base_branch_to_match_rule_does_not_fire_without_it` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `matchManagers` absent manager → rule doesn't fire | `crates/renovate-core/src/repo_config.rs` | `needs_manager_to_match_rule_does_not_fire_without_it` | ported |
| `lib/util/package-rules/index.spec.ts` | 370 | `matchDepTypes` multiple types in list | `crates/renovate-core/src/repo_config.rs` | `match_dep_types_multiple_types_in_list` | ported |
| `lib/util/package-rules/index.spec.ts` | 414 | no `depType` + `matchDepTypes` set → rule doesn't fire | `crates/renovate-core/src/repo_config.rs` | `match_dep_types_no_dep_type_rule_does_not_fire` | ported |
| `lib/util/package-rules/index.spec.ts` | — | rule with no matchers → fires for all deps | `crates/renovate-core/src/repo_config.rs` | `package_rules_enabled_true_does_not_ignore` | ported |
| `lib/util/package-rules/index.spec.ts` | — | only-negative matchPackageNames → others pass | `crates/renovate-core/src/repo_config.rs` | `match_package_names_negation` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `matchCurrentVersion` regex `/^4/` match | `crates/renovate-core/src/repo_config.rs` | `match_current_version_regex_against_current_value` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `matchCurrentVersion` negated regex `!/^4/` | `crates/renovate-core/src/repo_config.rs` | (implicit via negation logic) | ported |
| `lib/util/string-match.spec.ts` | — | `matchRegexOrGlobList` two negative regex patterns that fail | `crates/renovate-core/src/string_match.rs` | `all_negative_patterns_both_must_not_match` | ported |
| `lib/util/string-match.spec.ts` | — | `matchRegexOrGlobList` two negative glob patterns that fail | `crates/renovate-core/src/string_match.rs` | `all_negative_patterns_both_must_not_match_glob` | ported |
| `lib/util/string-match.spec.ts` | — | `matchRegexOrGlobList` positive + negative regex → true | `crates/renovate-core/src/string_match.rs` | `negative_regex_positive_pattern_returns_true` | ported |
| `lib/util/string-match.spec.ts` | — | `matchRegexOrGlobList` positive + two negative globs → true | `crates/renovate-core/src/string_match.rs` | `negative_glob_positive_pattern_returns_true` | ported |
| `lib/util/string-match.spec.ts` | — | `anyMatchRegexOrGlobList` empty patterns → false | `crates/renovate-core/src/string_match.rs` | `any_match_empty_patterns_returns_false` | ported |
| `lib/util/string-match.spec.ts` | — | `anyMatchRegexOrGlobList` empty inputs → false | `crates/renovate-core/src/string_match.rs` | `any_match_empty_inputs_returns_false` | ported |
| `lib/util/string-match.spec.ts` | — | `anyMatchRegexOrGlobList` positive pattern matches any | `crates/renovate-core/src/string_match.rs` | `any_match_positive_list_matches` | ported |
| `lib/util/string-match.spec.ts` | — | `anyMatchRegexOrGlobList` negative pattern matches any-non-excluded | `crates/renovate-core/src/string_match.rs` | `any_match_negative_list_matches_non_excluded` | ported |
| `lib/util/package-rules/dep-names.spec.ts` | — | `matchDepNames` undefined depName → false | `crates/renovate-core/src/repo_config.rs` | `match_dep_names_undefined_dep_name_does_not_fire` | ported |
| `lib/util/package-rules/dep-names.spec.ts` | — | `@opentelemetry**` without `/` does NOT match `@opentelemetry/http` | `crates/renovate-core/src/string_match.rs` | `dep_names_no_slash_double_star_does_not_cross_slash` | ported |
| `lib/util/package-rules/package-names.spec.ts` | — | `matchPackageNames` undefined packageName → false | `crates/renovate-core/src/repo_config.rs` | `match_package_names_uses_package_name_when_set` | ported |
| `lib/util/package-rules/package-names.spec.ts` | — | `matchPackageNames` uses packageName, not depName | `crates/renovate-core/src/repo_config.rs` | `match_package_names_with_dep_name_and_package_name` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `matchDepTypes` | `crates/renovate-core/src/repo_config.rs` | `dep_type_rule_blocks_dependencies`, `dep_type_rule_does_not_block_dev_dep` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `allowedVersions` semver range | `crates/renovate-core/src/repo_config.rs` | `allowed_versions_blocks_out_of_range_update` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `ignoreVersions` in packageRules | `crates/renovate-core/src/repo_config.rs` | `package_rule_ignore_versions_scoped_to_matched_package` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `matchPackagePrefixes` (deprecated) | `crates/renovate-core/src/repo_config.rs` | `match_package_prefixes_converted_to_glob` | ported |
| `lib/util/package-rules/registryurls.ts` | — | `matchRegistryUrls` exact | `crates/renovate-core/src/repo_config.rs` | `registry_url_repository_tests::match_registry_urls_exact_hit` | ported |
| `lib/util/package-rules/registryurls.ts` | — | `matchRegistryUrls` any-of-dep-urls | `crates/renovate-core/src/repo_config.rs` | `registry_url_repository_tests::match_registry_urls_any_of_dep_urls` | ported |
| `lib/util/package-rules/registryurls.ts` | — | `matchRegistryUrls` glob | `crates/renovate-core/src/repo_config.rs` | `registry_url_repository_tests::match_registry_urls_glob` | ported |
| `lib/util/package-rules/registryurls.ts` | — | empty `matchRegistryUrls` matches all | `crates/renovate-core/src/repo_config.rs` | `registry_url_repository_tests::match_registry_urls_empty_matches_all` | ported |
| `lib/util/package-rules/repositories.spec.ts` | — | `matchRepositories` exact | `crates/renovate-core/src/repo_config.rs` | `registry_url_repository_tests::match_repositories_exact_hit` | ported |
| `lib/util/package-rules/repositories.spec.ts` | — | `matchRepositories` glob | `crates/renovate-core/src/repo_config.rs` | `registry_url_repository_tests::match_repositories_glob` | ported |
| `lib/util/package-rules/repositories.spec.ts` | — | `matchRepositories` regex | `crates/renovate-core/src/repo_config.rs` | `registry_url_repository_tests::match_repositories_regex` | ported |
| `lib/util/package-rules/repositories.spec.ts` | — | empty `matchRepositories` matches all | `crates/renovate-core/src/repo_config.rs` | `registry_url_repository_tests::match_repositories_empty_matches_all` | ported |
| `lib/util/package-rules/repositories.spec.ts` | — | undefined repository + `matchRepositories` set → false | `crates/renovate-core/src/repo_config.rs` | `match_repositories_fires_only_for_matching_repo` | ported |
| `lib/util/package-rules/repositories.spec.ts` | — | invalid regex `/[/` → false | `crates/renovate-core/src/repo_config.rs` | `match_repositories_invalid_regex_returns_false` | ported |
| `lib/util/package-rules/repositories.spec.ts` | — | invalid negated regex `!/[/` → true | `crates/renovate-core/src/repo_config.rs` | `match_repositories_invalid_negated_regex_returns_true` | ported |
| `lib/util/package-rules/repositories.spec.ts` | — | any-of multiple patterns (regex OR glob) | `crates/renovate-core/src/repo_config.rs` | `match_repositories_any_of_patterns` | ported |
| `lib/util/package-rules/current-value.spec.ts` | — | undefined currentValue → false | `crates/renovate-core/src/repo_config.rs` | `match_current_value_undefined_returns_false` | ported |
| `lib/util/package-rules/current-value.spec.ts` | — | glob `1.2.*` match | `crates/renovate-core/src/repo_config.rs` | `match_current_value_glob_match` | ported |
| `lib/util/package-rules/new-value.spec.ts` | — | undefined newValue → false | `crates/renovate-core/src/repo_config.rs` | `match_new_value_undefined_returns_false` | ported |
| `lib/util/package-rules/new-value.spec.ts` | — | glob `1.2.*` match | `crates/renovate-core/src/repo_config.rs` | `match_new_value_glob_match` | ported |
| `lib/util/package-rules/files.spec.ts` | — | undefined packageFile + matchFileNames set → false | `crates/renovate-core/src/repo_config.rs` | `match_file_names_undefined_returns_false` | ported |
| `lib/util/package-rules/current-age.ts` | — | `matchCurrentAge` set without timestamp → false | `crates/renovate-core/src/repo_config.rs` | `dep_context_tests::match_current_age_set_without_timestamp_returns_false` | ported |
| `lib/util/package-rules/current-age.ts` | — | `matchCurrentAge` old dep matches `"> 3 days"` | `crates/renovate-core/src/repo_config.rs` | `dep_context_tests::match_current_age_old_dep_matches_gt_constraint` | ported |
| `lib/util/package-rules/current-age.ts` | — | `matchCurrentAge` new dep does not match `"> 3 days"` | `crates/renovate-core/src/repo_config.rs` | `dep_context_tests::match_current_age_new_dep_does_not_match_gt_constraint` | ported |
| `lib/util/package-rules/current-age.ts` | — | `matchCurrentAge` disables dep via DepContext | `crates/renovate-core/src/repo_config.rs` | `dep_context_tests::match_current_age_via_dep_context_disables_dep` | ported |
| `lib/util/package-rules/current-age.ts` | — | `matchCurrentAge` no constraint matches all | `crates/renovate-core/src/repo_config.rs` | `dep_context_tests::match_current_age_none_unset_matches_all` | ported |
| `lib/util/package-rules/current-age.spec.ts` | — | "returns false if release is older" (`< 1 year` with 2020 ts) | `crates/renovate-core/src/repo_config.rs` | `current_age_returns_false_if_release_older_than_constraint_bound` | ported |
| `lib/util/package-rules/current-age.spec.ts` | — | "returns false if release is younger" (`> 10 years` with 2020 ts) | `crates/renovate-core/src/repo_config.rs` | `current_age_returns_false_if_release_younger_than_constraint_bound` | ported |
| `lib/util/package-rules/current-age.spec.ts` | — | "returns null if release invalid" (invalid ts → null/pass) | `crates/renovate-core/src/repo_config.rs` | `current_age_returns_false_for_invalid_timestamp` | partial (Renovate: null→pass, our impl: false→block) |
| `lib/util/package-rules/current-age.spec.ts` | — | "returns false if release undefined" | `crates/renovate-core/src/repo_config.rs` | `match_current_age_set_without_timestamp_returns_false` | ported |
| `lib/util/package-rules/current-age.spec.ts` | — | "returns true if age matches" (`> 3 years` with 2020 ts) | `crates/renovate-core/src/repo_config.rs` | `match_current_age_old_dep_matches_gt_constraint` | ported |
| `lib/util/pretty-time.ts` | 50 | `satisfiesDateRange(old, "> 3 days")` → true | `crates/renovate-core/src/schedule.rs` | `date_range_gt_old_timestamp_is_true` | ported |
| `lib/util/pretty-time.ts` | 50 | `satisfiesDateRange(future, "> 3 days")` → false | `crates/renovate-core/src/schedule.rs` | `date_range_gt_future_timestamp_is_false` | ported |
| `lib/util/pretty-time.ts` | 50 | `satisfiesDateRange(recent, "< 3 days")` → true | `crates/renovate-core/src/schedule.rs` | `date_range_lt_recent_timestamp_is_true` | ported |
| `lib/util/pretty-time.ts` | 50 | `satisfiesDateRange(old, "< 3 days")` → false | `crates/renovate-core/src/schedule.rs` | `date_range_lt_old_timestamp_is_false` | ported |
| `lib/util/pretty-time.ts` | 50 | `satisfiesDateRange(old, ">= 1 week")` → true | `crates/renovate-core/src/schedule.rs` | `date_range_gte_old_is_true` | ported |
| `lib/util/pretty-time.ts` | 50 | `satisfiesDateRange(recent, "<= 1 week")` → true | `crates/renovate-core/src/schedule.rs` | `date_range_lte_future_is_true` | ported |
| `lib/util/pretty-time.ts` | 50 | invalid operator → false | `crates/renovate-core/src/schedule.rs` | `date_range_invalid_operator_returns_false` | ported |
| `lib/util/pretty-time.ts` | 50 | invalid timestamp → false | `crates/renovate-core/src/schedule.rs` | `date_range_invalid_timestamp_returns_false` | ported |
| `lib/workers/repository/update/branch/schedule.spec.ts` | 184 | `before 4:00pm` at 10am → true | `crates/renovate-core/src/schedule.rs` | `spec_supports_before_hours_true` | ported |
| `lib/workers/repository/update/branch/schedule.spec.ts` | 190 | `before 4:00am` at 10am → false | `crates/renovate-core/src/schedule.rs` | `spec_supports_before_hours_false` | ported |
| `lib/workers/repository/update/branch/schedule.spec.ts` | 196 | `after 4:00pm` at 10am → false | `crates/renovate-core/src/schedule.rs` | `spec_supports_outside_hours` | ported |
| `lib/workers/repository/update/branch/schedule.spec.ts` | 203 | cron `* 10 * * *` at hour=10 → true | `crates/renovate-core/src/schedule.rs` | `spec_cron_with_hours_match` | ported |
| `lib/workers/repository/update/branch/schedule.spec.ts` | 215 | cron `* * 30 * *` at dom=30 → true | `crates/renovate-core/src/schedule.rs` | `spec_cron_with_days_match` | ported |
| `lib/workers/repository/update/branch/schedule.spec.ts` | 225 | cron `* * * 6 *` at month=6 → true | `crates/renovate-core/src/schedule.rs` | `spec_cron_with_months_match` | ported |
| `lib/workers/repository/update/branch/schedule.spec.ts` | 235 | cron `* * * * 5` at weekday=5 (Friday) → true | `crates/renovate-core/src/schedule.rs` | `spec_cron_with_weekdays_match` | ported |
| `lib/workers/repository/update/branch/schedule.spec.ts` | 249 | cron `* * * * 0` on Sunday → true | `crates/renovate-core/src/schedule.rs` | `spec_cron_on_sunday_weekday_0` | ported |
| `lib/workers/repository/update/branch/schedule.spec.ts` | 154 | no schedule → true | `crates/renovate-core/src/schedule.rs` | `spec_returns_true_if_no_schedule` | ported |
| `lib/workers/repository/update/branch/schedule.spec.ts` | 159 | "at any time" → true | `crates/renovate-core/src/schedule.rs` | `spec_returns_true_for_at_any_time` | ported |
| `lib/workers/repository/update/branch/schedule.spec.ts` | 348 | multiple schedules: any one true → true | `crates/renovate-core/src/schedule.rs` | `spec_supports_multiple_schedules` | ported |
| `lib/workers/repository/update/branch/schedule.spec.ts` | 355 | "on friday and saturday" on Friday → true | `crates/renovate-core/src/schedule.rs` | `spec_supports_day_match_friday` | ported |
| `lib/workers/repository/update/branch/schedule.spec.ts` | 361 | "on monday and tuesday" on Friday → false | `crates/renovate-core/src/schedule.rs` | `spec_supports_day_mismatch` | ported |
| `lib/workers/repository/update/branch/schedule.spec.ts` | 367 | "every weekday" on Friday → true | `crates/renovate-core/src/schedule.rs` | `spec_every_weekday_matches_friday` | ported |
| `lib/workers/repository/update/branch/schedule.spec.ts` | 373 | "every weekend" on Friday → false | `crates/renovate-core/src/schedule.rs` | `spec_every_weekend_rejects_friday` | ported |
| `lib/workers/repository/update/branch/schedule.spec.ts` | 379 | "before 11:00am every weekday" at 10am → true | `crates/renovate-core/src/schedule.rs` | `spec_before_11am_every_weekday_matches` | ported |
| `lib/workers/repository/update/branch/schedule.spec.ts` | 344 | cron dom mismatch → false | `crates/renovate-core/src/schedule.rs` | `spec_cron_dom_mismatch_false` | ported |
| `lib/workers/repository/update/branch/schedule.spec.ts` | 349 | cron month mismatch → false | `crates/renovate-core/src/schedule.rs` | `spec_cron_month_mismatch_false` | ported |
| `lib/workers/repository/update/branch/schedule.spec.ts` | 396 | first day of month rejects June 30 | `crates/renovate-core/src/schedule.rs` | `spec_first_day_of_month_rejects_non_first` | ported |
| `lib/workers/repository/update/branch/schedule.spec.ts` | 402 | first day of month approves October 1 | `crates/renovate-core/src/schedule.rs` | `spec_first_day_of_month_approves_first` | ported |
| `lib/modules/platform/local/index.ts` | — | `--platform=local` scans CWD | `crates/renovate-core/src/platform/local.rs` | (integration, verified via `renovate --platform=local --dry-run=full`) | ported |
| `lib/util/ignore.spec.ts` | — | `isSkipComment('renovate:ignore')` → true | `crates/renovate-core/src/string_match.rs` | `skip_comment_renovate_ignore_returns_true` | ported |
| `lib/util/ignore.spec.ts` | — | `isSkipComment('other:ignore')` → false | `crates/renovate-core/src/string_match.rs` | `skip_comment_other_prefix_returns_false` | ported |
| `lib/util/ignore.spec.ts` | — | `isSkipComment('renovate:update')` → false | `crates/renovate-core/src/string_match.rs` | `skip_comment_renovate_non_ignore_returns_false` | ported |
| `lib/util/ignore.spec.ts` | — | `isSkipComment(undefined)` → false | `crates/renovate-core/src/string_match.rs` | `skip_comment_empty_returns_false` | ported |
| `lib/modules/manager/asdf/extract.spec.ts` | 1096 | `renovate:ignore` comment skips dep | `crates/renovate-core/src/extractors/asdf.rs` | `renovate_ignore_comment_skips_dep` | ported |
| `lib/modules/manager/asdf/extract.spec.ts` | 19 | provides skipReason for unsupported tooling | `crates/renovate-core/src/extractors/asdf.rs` | `provides_skip_reason_for_unsupported_tooling` | ported |
| `lib/modules/manager/asdf/extract.spec.ts` | 31 | only captures first version (multiple versions on line) | `crates/renovate-core/src/extractors/asdf.rs` | `only_captures_first_version` | ported |
| `lib/modules/manager/pip_requirements/index.spec.ts` | — | pip_requirements file patterns match all expected paths | `crates/renovate-core/src/managers.rs` | `pip_requirements_file_patterns_match_spec` | ported |
| `lib/modules/manager/mise/index.spec.ts` | — | mise file patterns match all expected paths | `crates/renovate-core/src/managers.rs` | `mise_file_patterns_match_spec` | ported |
| `lib/modules/manager/circleci/index.spec.ts` | — | circleci file patterns match all expected paths | `crates/renovate-core/src/managers.rs` | `circleci_file_patterns_match_spec` | ported |
| `lib/util/package-rules/categories.ts` | — | `matchCategories` exact hit | `crates/renovate-core/src/repo_config.rs` | `categories_base_branch_tests::match_categories_exact_hit` | ported |
| `lib/util/package-rules/categories.ts` | — | `matchCategories` any-of-many | `crates/renovate-core/src/repo_config.rs` | `categories_base_branch_tests::match_categories_any_of_many` | ported |
| `lib/util/package-rules/categories.ts` | — | empty `matchCategories` matches all | `crates/renovate-core/src/repo_config.rs` | `categories_base_branch_tests::match_categories_empty_matches_all` | ported |
| `lib/util/package-rules/base-branch.ts` | — | `matchBaseBranches` exact hit | `crates/renovate-core/src/repo_config.rs` | `categories_base_branch_tests::match_base_branches_exact_hit` | ported |
| `lib/util/package-rules/base-branch.ts` | — | `matchBaseBranches` glob `release/*` | `crates/renovate-core/src/repo_config.rs` | `categories_base_branch_tests::match_base_branches_glob` | ported |
| `lib/util/package-rules/base-branch.ts` | — | empty `matchBaseBranches` matches all | `crates/renovate-core/src/repo_config.rs` | `categories_base_branch_tests::match_base_branches_empty_matches_all` | ported |
| `lib/util/package-rules/base-branch.ts` | — | `matchBaseBranches` multiple entries | `crates/renovate-core/src/repo_config.rs` | `categories_base_branch_tests::match_base_branches_multiple_entries` | ported |
| `lib/util/package-rules/index.spec.ts` | 468 | `matchCategories` dep-provided categories override manager-derived | `crates/renovate-core/src/repo_config.rs` | `categories_base_branch_tests::match_categories_dep_provided_categories_override_manager_derived` | ported |
| `lib/util/package-rules/index.spec.ts` | 489 | `matchCategories` non-matching dep-provided categories → rule doesn't fire | `crates/renovate-core/src/repo_config.rs` | `categories_base_branch_tests::match_categories_dep_provided_categories_non_matching` | ported |
| `lib/util/package-rules/index.spec.ts` | 510 | `matchCategories` undefined/absent categories → rule doesn't fire | `crates/renovate-core/src/repo_config.rs` | `categories_base_branch_tests::needs_categories_to_match_rule_does_not_fire_without_it` | ported |
| `lib/util/package-rules/index.spec.ts` | 1101 | `matchCurrentVersion` regex `/^4/` with currentVersion field | `crates/renovate-core/src/repo_config.rs` | `match_current_version_index_spec_regex_matches` | ported |
| `lib/util/package-rules/index.spec.ts` | 1132 | `matchCurrentVersion` negated regex `!/^4/` with currentVersion field | `crates/renovate-core/src/repo_config.rs` | `match_current_version_index_spec_negated_regex` | ported |
| `lib/util/package-rules/index.spec.ts` | 1079 | `matchCurrentVersion` static version value matches currentValue range | `crates/renovate-core/src/repo_config.rs` | `match_current_version_index_spec_static_value` | ported |
| `lib/util/package-rules/index.spec.ts` | 1049 | `matchCurrentVersion` version pattern matches/mismatches currentValue range | `crates/renovate-core/src/repo_config.rs` | `match_current_version_index_spec_version_matches_range` | ported |
| `lib/util/package-rules/index.spec.ts` | 987 | `matchCurrentVersion` range overlap requires currentVersion field | `crates/renovate-core/src/repo_config.rs` | `match_current_version_range_requires_current_version`, `match_current_version_range_uses_current_version_field` | ported |
| `lib/util/package-rules/index.spec.ts` | 389 | `depTypes` (plural array) — any element matches → rule fires | `crates/renovate-core/src/repo_config.rs` | `match_dep_types_plural_array_any_matches`, `match_dep_types_plural_array_no_match` | ported |
| `lib/util/package-rules/index.spec.ts` | 950 | `filters naked depType` — singular depType matches | `crates/renovate-core/src/repo_config.rs` | `match_dep_types_naked_dep_type_matches` | ported |
| `lib/util/package-rules/index.spec.ts` | 968 | `filters out unrequested depType` — non-matching dep type → rule doesn't fire | `crates/renovate-core/src/repo_config.rs` | `match_dep_types_out_of_requested_does_not_match` | ported |
| `lib/util/package-rules/index.spec.ts` | 38 | `applies` — comprehensive integration: isBump+matchCurrentVersion+matchPackageNames negation | `crates/renovate-core/src/repo_config.rs` | `applies_comprehensive_integration` | ported |
| `lib/util/package-rules/index.spec.ts` | 1242 | `creates groupSlug if necessary` — auto-generate groupSlug from groupName when prior slug exists | `crates/renovate-core/src/repo_config.rs` | `rule_effects_tests::group_slug_auto_generated_from_group_name_when_prior_slug_exists` | ported |
| `lib/util/package-rules/index.spec.ts` | 202 | `force.enabled:true` overrides `enabled:false` (vulnerability alert) | `crates/renovate-core/src/repo_config.rs` | `force_enabled_true_overrides_enabled_false` | ported |
| `lib/util/package-rules/index.spec.ts` | 223 | `force.enabled:true` overrides config-level `enabled:false` | `crates/renovate-core/src/repo_config.rs` | `force_enabled_true_also_overrides_config_level_disabled` | ported |
| `lib/util/package-rules/index.spec.ts` | 292 | `force.enabled:false` overrides `enabled:true` | `crates/renovate-core/src/repo_config.rs` | `force_enabled_false_overrides_enabled_true` | ported |
| `lib/config/migration.spec.ts` | 835 | `baseBranches: ['main', 'dev']` → `baseBranchPatterns` support | `crates/renovate-core/src/repo_config.rs` | `base_branch_patterns_parsed`, `base_branch_patterns_merged_with_base_branches` | ported |
| `lib/config/migration.spec.ts` | 762 | `azureAutoComplete: true/false` → `platformAutomerge` | `crates/renovate-core/src/repo_config.rs` | `azure_auto_complete_migrated_to_platform_automerge` | ported |
| `lib/config/migration.spec.ts` | 791 | `gitLabAutomerge: true/false` → `platformAutomerge` | `crates/renovate-core/src/repo_config.rs` | `git_lab_automerge_migrated_to_platform_automerge` | ported |
| `lib/config/migration.spec.ts` | 184 | compound schedule splitting: "after 10pm and before 7am" → ["after 10pm", "before 7am"] | `crates/renovate-core/src/repo_config.rs` | `schedule_compound_after_before_splits_at_midnight_boundary`, `schedule_compound_split_with_day_suffix`, `schedule_compound_non_straddling_not_split` | ported |
| `lib/config/migrations/custom/path-rules-migration.spec.ts` | — | `pathRules: [...]` → merged into `packageRules` | `crates/renovate-core/src/repo_config.rs` | `deprecated_path_rules_field_merged_into_package_rules`, `path_rules_concat_with_existing_package_rules` | ported |
| `lib/config/migrations/custom/match-managers-migration.spec.ts` | — | `matchManagers: ["renovate-config-presets"]` → `["renovate-config"]` | `crates/renovate-core/src/repo_config.rs` | `match_managers_renovate_config_presets_migrated_to_renovate_config` | ported |
| `lib/config/migrations/custom/extends-migration.spec.ts` | — | `github>whitesource/merge-confidence:beta` → `mergeConfidence:all-badges` | `crates/renovate-core/src/repo_config.rs` | `extend_whitesource_merge_confidence_preset_normalized` | ported |

---

## `extends` preset resolution

| Renovate test file | Line | Renovate test | Rust location | Rust test | Status |
|--------------------|------|---------------|---------------|-----------|--------|
| `lib/config/presets/internal/default.preset.ts` | 303 | `:ignoreModulesAndTests` ignore paths | `crates/renovate-core/src/repo_config.rs` | `ignore_modules_and_tests_preset_direct` | ported |
| `lib/config/presets/internal/config.preset.ts` | — | `config:recommended` includes `:ignoreModulesAndTests` | `crates/renovate-core/src/repo_config.rs` | `config_recommended_adds_ignore_modules_and_tests_paths` | ported |
| `lib/config/presets/internal/config.preset.ts` | — | `config:base` has same ignore paths | `crates/renovate-core/src/repo_config.rs` | `config_base_adds_ignore_paths` | ported |
| `lib/config/presets/internal/default.preset.ts` | — | `:semanticCommits` sets semanticCommits=enabled | `crates/renovate-core/src/repo_config.rs` | `semantic_commits_preset_sets_field` | ported |
| `lib/config/presets/internal/default.preset.ts` | — | `:semanticCommitsDisabled` sets semanticCommits=disabled | `crates/renovate-core/src/repo_config.rs` | `semantic_commits_disabled_preset` | ported |
| `lib/config/presets/internal/default.preset.ts` | — | Explicit field overrides preset value | `crates/renovate-core/src/repo_config.rs` | `explicit_semantic_commits_overrides_preset` | ported |
| `lib/config/presets/index.ts` | — | Unknown preset does not break parsing | `crates/renovate-core/src/repo_config.rs` | `unknown_preset_ignored` | ported |
| `lib/config/presets/index.ts` | — | Preset paths are prepended (user paths last) | `crates/renovate-core/src/repo_config.rs` | `user_ignore_paths_appended_after_preset_paths` | ported |
| `lib/config/presets/index.ts` | — | Duplicate presets deduplicated | `crates/renovate-core/src/repo_config.rs` | `duplicate_preset_deduplicated` | ported |

---

## Deprecated field migrations (`lib/config/migration.spec.ts`)

| Renovate test file | Line | Renovate test | Rust location | Rust test | Status |
|--------------------|------|---------------|---------------|-----------|--------|
| `lib/config/migration.spec.ts` | 441 | `unpublishSafe: true` → injects `security:minimumReleaseAgeNpm` | `crates/renovate-core/src/repo_config.rs` | `unpublish_safe_true_injects_minimum_release_age_preset` | ported |
| `lib/config/migration.spec.ts` | 441 | `unpublishSafe: true` + existing extends → appends preset | `crates/renovate-core/src/repo_config.rs` | `unpublish_safe_true_with_existing_extends_appends_preset` | ported |
| `lib/config/migration.spec.ts` | 441 | `unpublishSafe: false` → no injection | `crates/renovate-core/src/repo_config.rs` | `unpublish_safe_false_does_not_inject` | ported |
| `lib/config/migration.spec.ts` | 441 | `unpublishSafe: true` + `:unpublishSafe` in extends → no duplication | `crates/renovate-core/src/repo_config.rs` | `unpublish_safe_with_unpublish_safe_preset_already_in_extends_does_not_duplicate` | ported |
| `lib/config/migration.spec.ts` | 532 | `npm:unpublishSafe` in extends → `security:minimumReleaseAgeNpm` | `crates/renovate-core/src/repo_config.rs` | `npm_unPublish_safe_resolves_to_minimum_release_age` | ported |
| `lib/config/migration.spec.ts` | 17 | `automerge: 'none'` → `automerge: false` | `crates/renovate-core/src/repo_config.rs` | `automerge_none_migrated_to_false` | ported |
| `lib/config/migration.spec.ts` | 17 | `separateMajorReleases: true` → `separateMajorMinor: true` | `crates/renovate-core/src/repo_config.rs` | `separate_major_releases_migrated` | ported |
| `lib/config/migration.spec.ts` | 17 | `upgradeInRange: true` → `rangeStrategy: 'bump'` | `crates/renovate-core/src/repo_config.rs` | `upgrade_in_range_true_sets_range_strategy_bump` | ported |
| `lib/config/migration.spec.ts` | 17 | `baseBranch: 'next'` → `baseBranches: ['next']` | `crates/renovate-core/src/repo_config.rs` | `base_branch_singular_migrated_to_base_branches` | ported |
| `lib/config/migration.spec.ts` | 17 | `rebaseStalePrs: true` → `rebaseWhen: 'behind-base-branch'` | `crates/renovate-core/src/repo_config.rs` | `rebase_stale_prs_true_sets_rebase_when` | ported |
| `lib/config/migration.spec.ts` | 205 | `"every friday"` → `"on friday"` schedule migration | `crates/renovate-core/src/repo_config.rs` | `schedule_every_friday_migrated_to_on_friday`, `schedule_every_monday_migrated` | ported |
| `lib/config/migration.spec.ts` | 226 | `"every weekday"` → no migration (handled natively) | `crates/renovate-core/src/repo_config.rs` | `schedule_every_weekday_not_migrated` | ported |
| `lib/config/migration.spec.ts` | — | `schedule: "string"` → `schedule: ["string"]` (string coerced to array) | `crates/renovate-core/src/repo_config.rs` | `schedule_every_friday_migrated_to_on_friday` | ported |
| `lib/config/migration.spec.ts` | 419 | `extends: 'foo'` → `extends: ['foo']` (string to array) | `crates/renovate-core/src/repo_config.rs` | `extends_string_coerced_to_array` | ported |
| `lib/config/migration.spec.ts` | 419 | `extends: ':js-app'` → `config:js-app` (removedPresets) | `crates/renovate-core/src/repo_config.rs` | `extends_js_app_shorthand_normalized` | ported |
| `lib/config/migration.spec.ts` | 419 | `extends: ':base'` → `config:recommended` | `crates/renovate-core/src/repo_config.rs` | `extends_base_shorthand_normalized` | ported |
| `lib/config/migration.spec.ts` | — | `extends: ':masterIssue'` → `':dependencyDashboard'` | `crates/renovate-core/src/repo_config.rs` | `extends_master_issue_normalized` | ported |
| `lib/config/migration.spec.ts` | 532 | `extends: ['npm:unpublishSafe']` → `security:minimumReleaseAgeNpm` | `crates/renovate-core/src/repo_config.rs` | `extends_npm_unpublish_safe_normalized` | ported |
| `lib/config/migration.spec.ts` | — | `regexManagers:*` → `customManagers:*` (all 10 entries) | `crates/renovate-core/src/repo_config.rs` | (via normalize_preset) | ported |
| `lib/config/migration.spec.ts` | 17 | `rebaseConflictedPrs: false` → `rebaseWhen: 'never'` | `crates/renovate-core/src/repo_config.rs` | `rebase_conflicted_prs_false_sets_rebase_when_never` | ported |
| `lib/config/migration.spec.ts` | 17 | `ignoreNodeModules: true` → adds `node_modules/` to `ignorePaths` | `crates/renovate-core/src/repo_config.rs` | `ignore_node_modules_true_adds_to_ignore_paths` | ported |
| `lib/config/migration.spec.ts` | 17 | `enabledManagers: ['yarn']` → `['npm']` | `crates/renovate-core/src/repo_config.rs` | `enabled_managers_yarn_migrated_to_npm` | ported |
| `lib/config/migration.spec.ts` | 17 | `semanticCommits: true` → `semanticCommits: 'enabled'` | `crates/renovate-core/src/repo_config.rs` | `semantic_commits_bool_true_migrated` | ported |
| `lib/config/migration.spec.ts` | — | `stabilityDays: N` → `minimumReleaseAge: 'N days'` | `crates/renovate-core/src/repo_config.rs` | `stability_days_migrated_to_minimum_release_age` | ported |
| `lib/config/migration.spec.ts` | 551 | `paths` → `matchFileNames` | `crates/renovate-core/src/repo_config.rs` | `migrates_package_rules_all_deprecated_fields` | ported |
| `lib/config/migration.spec.ts` | 551 | `languages` → `matchCategories` | `crates/renovate-core/src/repo_config.rs` | `migrates_package_rules_all_deprecated_fields` | ported |
| `lib/config/migration.spec.ts` | 551 | `baseBranchList` → `matchBaseBranches` | `crates/renovate-core/src/repo_config.rs` | `migrates_package_rules_all_deprecated_fields` | ported |
| `lib/config/migration.spec.ts` | 551 | `managers` → `matchManagers` | `crates/renovate-core/src/repo_config.rs` | `migrates_package_rules_all_deprecated_fields` | ported |
| `lib/config/migration.spec.ts` | 551 | `datasources` → `matchDatasources` | `crates/renovate-core/src/repo_config.rs` | `migrates_package_rules_all_deprecated_fields` | ported |
| `lib/config/migration.spec.ts` | 551 | `depTypeList` → `matchDepTypes` | `crates/renovate-core/src/repo_config.rs` | `migrates_package_rules_all_deprecated_fields` | ported |
| `lib/config/migration.spec.ts` | 551 | `packageNames` → `matchPackageNames` | `crates/renovate-core/src/repo_config.rs` | `migrates_package_rules_all_deprecated_fields`, `deprecated_package_names_merged_with_match_package_names` | ported |
| `lib/config/migration.spec.ts` | 551 | `packagePatterns` → `/pattern/` in `matchPackageNames` | `crates/renovate-core/src/repo_config.rs` | `migrates_package_rules_all_deprecated_fields` | ported |
| `lib/config/migration.spec.ts` | 551 | `excludePackageNames` → `!name` in `matchPackageNames` | `crates/renovate-core/src/repo_config.rs` | `migrates_package_rules_all_deprecated_fields` | ported |
| `lib/config/migration.spec.ts` | 551 | `excludePackagePatterns` → `!/pattern/` in `matchPackageNames` | `crates/renovate-core/src/repo_config.rs` | `migrates_package_rules_all_deprecated_fields` | ported |
| `lib/config/migration.spec.ts` | 551 | `excludeRepositories` → `!repo` in `matchRepositories` | `crates/renovate-core/src/repo_config.rs` | `deprecated_exclude_repositories_negation` | ported |
| `lib/config/migration.spec.ts` | 551 | `sourceUrlPrefixes` → `url{/,}**` in `matchSourceUrls` | `crates/renovate-core/src/repo_config.rs` | `deprecated_source_url_prefixes_become_glob` | ported |
| `lib/config/migration.spec.ts` | 551 | `updateTypes` → `matchUpdateTypes` | `crates/renovate-core/src/repo_config.rs` | `migrates_package_rules_all_deprecated_fields` | ported |
| `lib/config/migration.spec.ts` | 257 | `packages: [{...}]` → `packageRules` (old field name) | `crates/renovate-core/src/repo_config.rs` | `deprecated_packages_field_merged_into_package_rules` | ported |
| `lib/config/migration.spec.ts` | 257 | `groupName: ["name"]` array → `groupName: "name"` string | `crates/renovate-core/src/repo_config.rs` | `group_name_array_first_element_used` | ported |

---

## Manager detection

| Renovate test file | Line | Renovate test | Rust location | Rust test | Status |
|--------------------|------|---------------|---------------|-----------|--------|
| `lib/modules/manager/cargo/index.ts` | — | Cargo.toml pattern | `crates/renovate-core/src/managers.rs` | `detects_cargo` | ported |
| `lib/modules/manager/npm/index.ts` | — | package.json pattern | `crates/renovate-core/src/managers.rs` | `detects_npm_package_json` | ported |
| `lib/modules/manager/pip_requirements/index.ts` | — | requirements.txt pattern | `crates/renovate-core/src/managers.rs` | `detects_pip_requirements` | ported |
| `lib/modules/manager/github-actions/index.ts` | — | workflow YAML pattern | `crates/renovate-core/src/managers.rs` | `detects_github_actions_workflow` | ported |
| `lib/modules/manager/dockerfile/index.ts` | — | Dockerfile pattern | `crates/renovate-core/src/managers.rs` | `detects_dockerfile` | ported |
| `lib/modules/manager/docker-compose/index.ts` | — | docker-compose.yml pattern | `crates/renovate-core/src/managers.rs` | `detects_docker_compose` | ported |
| `lib/modules/manager/maven/index.ts` | — | pom.xml pattern | `crates/renovate-core/src/managers.rs` | `detects_maven_pom` | ported |
| `lib/modules/manager/git-submodules/index.ts` | — | `defaultConfig.enabled: false` | `crates/renovate-core/src/managers.rs` | `DISABLED_BY_DEFAULT` array | ported |
| `lib/modules/manager/html/index.ts` | — | `defaultConfig.enabled: false` | `crates/renovate-core/src/managers.rs` | `DISABLED_BY_DEFAULT` array | ported |
| `lib/modules/manager/nix/index.ts` | — | `defaultConfig.enabled: false` | `crates/renovate-core/src/managers.rs` | `DISABLED_BY_DEFAULT` array | ported |
| `lib/modules/manager/pre-commit/index.ts` | — | `defaultConfig.enabled: false` | `crates/renovate-core/src/managers.rs` | `DISABLED_BY_DEFAULT` array | ported |
| `lib/modules/manager/travis/index.ts` | — | `defaultConfig.enabled: false` | `crates/renovate-core/src/managers.rs` | `DISABLED_BY_DEFAULT` array | ported |
| `lib/modules/manager/azure-pipelines/index.ts` | — | `defaultConfig.enabled: false` | `crates/renovate-core/src/managers.rs` | `DISABLED_BY_DEFAULT` array | ported |

---

## Cargo.toml extractor (`lib/modules/manager/cargo/extract.spec.ts`)

| Renovate test file | Line | Renovate test | Rust location | Rust test | Status |
|--------------------|------|---------------|---------------|-----------|--------|
| `lib/modules/manager/cargo/extract.spec.ts` | 46 | returns null for invalid toml | `crates/renovate-core/src/extractors/cargo.rs` | `empty_manifest_returns_empty_list` | ported |
| `lib/modules/manager/cargo/extract.spec.ts` | 52 | returns null for empty dependencies | `crates/renovate-core/src/extractors/cargo.rs` | `empty_manifest_returns_empty_list` | ported |
| `lib/modules/manager/cargo/extract.spec.ts` | 73 | extracts multiple dependencies simple | `crates/renovate-core/src/extractors/cargo.rs` | `extracts_simple_string_deps` | ported |
| `lib/modules/manager/cargo/extract.spec.ts` | 79 | extracts multiple dependencies advanced | `crates/renovate-core/src/extractors/cargo.rs` | `extracts_table_deps_with_version` | ported |
| `lib/modules/manager/cargo/extract.spec.ts` | 85 | handles inline tables | `crates/renovate-core/src/extractors/cargo.rs` | `extracts_table_deps_with_version` | ported |
| `lib/modules/manager/cargo/extract.spec.ts` | 91 | handles standard tables | `crates/renovate-core/src/extractors/cargo.rs` | `version_constraint_forms_are_preserved` | ported |
| `lib/modules/manager/cargo/extract.spec.ts` | — | platform-specific deps extracted | `crates/renovate-core/src/extractors/cargo.rs` | `target_cfg_dependencies_extracted` | ported |
| `lib/modules/manager/cargo/extract.spec.ts` | — | workspace deps extracted | `crates/renovate-core/src/extractors/cargo.rs` | `workspace_dependencies_extracted`, `workspace_and_member_deps_both_extracted` | ported |
| `lib/modules/manager/cargo/extract.spec.ts` | — | path dep is skipped | `crates/renovate-core/src/extractors/cargo.rs` | `path_dep_is_skipped` | ported |
| `lib/modules/manager/cargo/extract.spec.ts` | — | git dep is skipped | `crates/renovate-core/src/extractors/cargo.rs` | `git_dep_is_skipped` | ported |
| `lib/modules/manager/cargo/extract.spec.ts` | — | dev/build deps have correct type | `crates/renovate-core/src/extractors/cargo.rs` | `dev_and_build_deps_have_correct_type` | ported |
| `lib/modules/manager/cargo/extract.spec.ts` | — | `package.name` field overrides crate name | `crates/renovate-core/src/extractors/cargo.rs` | `package_field_overrides_name` | ported |
| `lib/modules/manager/cargo/extract.spec.ts` | 103 | extracts registry URLs from `.cargo/config.toml` | `crates/renovate-core/src/extractors/cargo.rs` | (not tested — no config.toml support yet) | pending |

---

## `enabledManagers` filtering

| Renovate test file | Line | Renovate test | Rust location | Rust test | Status |
|--------------------|------|---------------|---------------|-----------|--------|
| `lib/config/options/index.ts` | — | Empty `enabledManagers` → all non-disabled active | `crates/renovate-core/src/repo_config.rs` | `enabled_managers_empty_means_all_active` | ported |
| `lib/config/options/index.ts` | — | Non-empty `enabledManagers` → whitelist | `crates/renovate-core/src/repo_config.rs` | `enabled_managers_parsed` | ported |
| `lib/modules/manager/git-submodules/index.ts` | — | Disabled-by-default manager skipped without explicit list | `crates/renovate-core/src/repo_config.rs` | `disabled_by_default_manager_skipped_without_explicit_list` | ported |
| `lib/modules/manager/git-submodules/index.ts` | — | Disabled-by-default manager enabled when listed | `crates/renovate-core/src/repo_config.rs` | `disabled_by_default_manager_enabled_when_explicitly_listed` | ported |

---

## Branch name generation

| Renovate test file | Line | Renovate test | Rust location | Rust test | Status |
|--------------------|------|---------------|---------------|-----------|--------|
| `lib/workers/repository/updates/flatten.spec.ts` | 20 | `sanitizeDepName` sanitizes URLs | `crates/renovate-core/src/branch.rs` | `sanitize_url_style_dep` | ported |
| `lib/workers/repository/updates/flatten.spec.ts` | — | Scoped npm package `@org/pkg` → `org-pkg` | `crates/renovate-core/src/branch.rs` | `sanitize_scoped_npm_package` | ported |
| `lib/workers/repository/updates/flatten.spec.ts` | — | `@types/pkg` → strips `@types/` prefix | `crates/renovate-core/src/branch.rs` | `sanitize_types_prefix_stripped` | ported |
| `lib/config/options/index.ts` | 2368 | Default `branchTopic` = `{dep}-{major}.x` | `crates/renovate-core/src/branch.rs` | `branch_topic_default_no_minor_component` | ported |
| `lib/config/options/index.ts` | 2368 | `separateMinorPatch=true` + patch → `{dep}-{major}.{minor}.x` | `crates/renovate-core/src/branch.rs` | `branch_topic_separate_minor_patch_for_patch_update` | ported |
| `lib/config/options/index.ts` | 2348 | Default `branchName` = `{prefix}{topic}` | `crates/renovate-core/src/branch.rs` | `branch_name_default_prefix`, `branch_name_roundtrip` | ported |
| `lib/workers/repository/updates/branch-name.spec.ts` | — | `cleanBranchName` strips leading/trailing dots | `crates/renovate-core/src/branch.rs` | `branch_name_roundtrip` | partial |
| `lib/workers/repository/updates/branch-name.spec.ts` | 269 | realistic defaults: `renovate/jest-42.x` | `crates/renovate-core/src/branch.rs` | `branch_name_roundtrip` | partial (no Handlebars template) |
| `lib/workers/repository/updates/branch-name.spec.ts` | 316 | `hashedBranchLength` produces truncated hash | `crates/renovate-core/src/branch.rs` | `hashed_branch_length_produces_exact_length` | ported |
| `lib/workers/repository/updates/branch-name.spec.ts` | 350 | `hashedBranchLength` too short → minimum applied | `crates/renovate-core/src/branch.rs` | `hashed_branch_length_too_small_uses_min` | ported |
| `lib/workers/repository/updates/branch-name.spec.ts` | 405 | enforces valid git branch name (no reserved chars) | `crates/renovate-core/src/branch.rs` | `clean_branch_name_*` tests | ported |
| `lib/workers/repository/updates/branch-name.spec.ts` | — | group branch names with template compilation | — | (pending — requires Handlebars template engine) | pending |

---

## Git submodules extractor

| Renovate test file | Line | Renovate test | Rust location | Rust test | Status |
|--------------------|------|---------------|---------------|-----------|--------|
| `lib/modules/manager/git-submodules/extract.spec.ts` | 48 | Empty `.gitmodules` returns null/empty | `crates/renovate-core/src/extractors/git_submodules.rs` | `empty_content_returns_no_deps` | ported |
| `lib/modules/manager/git-submodules/extract.spec.ts` | 52 | `currentValue` unset when no branch | `crates/renovate-core/src/extractors/git_submodules.rs` | `single_submodule_no_branch` | ported |
| `lib/modules/manager/git-submodules/extract.spec.ts` | 58 | Branch is used when specified | `crates/renovate-core/src/extractors/git_submodules.rs` | `single_submodule_with_branch` | ported |
| `lib/modules/manager/git-submodules/extract.spec.ts` | 72–88 | `sourceUrl` from SSH URL → HTTPS | `crates/renovate-core/src/extractors/git_submodules.rs` | `single_submodule_no_branch` (SSH→HTTPS) | ported |
| `lib/modules/manager/git-submodules/extract.spec.ts` | 89 | `branch = .` → "current branch" → `currentValue` unset | `crates/renovate-core/src/extractors/git_submodules.rs` | `branch_dot_normalized_to_none` | ported |
| `lib/modules/manager/git-submodules/extract.spec.ts` | 64 | Relative URL constructed from relative path | `crates/renovate-core/src/extractors/git_submodules.rs` | `relative_url_passthrough` | partial (pass-through; relative URL resolution needs origin URL) |
| `lib/modules/manager/git-submodules/extract.spec.ts` | 127 | Semver branch extracted as `currentValue` | `crates/renovate-core/src/extractors/git_submodules.rs` | `semver_and_non_semver_branches` | ported |

---

## Custom managers (`customManagers` + `custom-managers:*` preset)

| Renovate test file | Line | Renovate test | Rust location | Rust test | Status |
|--------------------|------|---------------|---------------|-----------|--------|
| `lib/modules/manager/custom/regex/index.spec.ts` | 23 | extracts multiple dependencies | `crates/renovate-core/src/repo_config.rs` | `custom_manager_extracts_deps_from_content` | ported |
| `lib/modules/manager/custom/regex/index.spec.ts` | 50 | returns null if no dependencies found | `crates/renovate-core/src/repo_config.rs` | `custom_manager_combination_incomplete_match_returns_empty` | ported |
| `lib/modules/manager/custom/regex/index.spec.ts` | 81 | extracts `extractVersion` from capture | `crates/renovate-core/src/repo_config.rs` | `custom_manager_extracts_deps_from_content` | partial |
| `lib/modules/manager/custom/regex/index.spec.ts` | 299 | combination strategy merges captures | `crates/renovate-core/src/repo_config.rs` | `custom_manager_combination_strategy_merges_captures` | ported |
| `lib/modules/manager/custom/regex/index.spec.ts` | — | `customType: "regex"` is default | `crates/renovate-core/src/repo_config.rs` | (handled in migration at parse time) | ported |
| `lib/modules/manager/custom/regex/index.spec.ts` | — | `fileMatch`/`managerFilePatterns` gating | `crates/renovate-core/src/repo_config.rs` | `custom_manager_matches_file_by_pattern`, `custom_manager_file_match_legacy_field_parsed` | ported |
| `lib/modules/manager/custom/regex/index.spec.ts` | — | `datasourceTemplate` used when group missing | `crates/renovate-core/src/repo_config.rs` | `custom_manager_uses_datasource_template_when_group_missing` | ported |
| `lib/config/presets/internal/custom-managers.ts` | — | `dockerfileVersions` preset regression | `crates/renovate-core/src/repo_config.rs` | `dockerfile_versions_extracts_env_with_double_quotes`, `…_single_quotes`, `…_without_quotes`, `…_arg_directive`, `…_versioning_and_extract_version`, `…_file_pattern_matches` | ported |
| `lib/config/presets/internal/custom-managers.ts` | — | `makefileVersions` preset: `=`, ` = `, `:=`, `?=` | `crates/renovate-core/src/repo_config.rs` | `makefile_versions_extracts_simple_assignment`, `…_space_assignment`, `…_colon_equal`, `…_question_equal`, `…_file_pattern_matches` | ported |
| `lib/config/presets/internal/custom-managers.ts` | — | `helmChartYamlAppVersions` preset extracts appVersion | `crates/renovate-core/src/repo_config.rs` | `helm_chart_yaml_extracts_app_version`, `…_file_pattern_matches` | ported |
| `lib/config/presets/internal/custom-managers.ts` | — | `azurePipelinesVersions` file pattern | `crates/renovate-core/src/repo_config.rs` | `azure_pipelines_file_pattern_matches` | ported |
| `lib/config/presets/internal/custom-managers.ts` | — | `mavenPropertyVersions` preset extracts pom.xml | `crates/renovate-core/src/repo_config.rs` | `maven_property_versions_extracts_from_pom_xml`, `…_file_pattern_matches` | ported |
| `lib/config/presets/internal/custom-managers.ts` | — | `recursive` strategy | — | (not implemented — recursive extraction strategy not supported) | pending |
| `lib/modules/manager/custom/regex/index.spec.ts` | 221 | `autoReplaceStringTemplate` extraction | — | (not implemented) | pending |

---

## Versioning

| Renovate test file | Line | Renovate test | Rust location | Rust test | Status |
|--------------------|------|---------------|---------------|-----------|--------|
| `lib/modules/versioning/semver/index.spec.ts` | — | `isValid` accepts semver and ranges | `crates/renovate-core/src/versioning/semver_generic.rs` | `parse_padded_*` | partial |
| `lib/modules/versioning/semver/index.spec.ts` | — | Major/minor/patch classification | `crates/renovate-core/src/versioning/semver_generic.rs` | `classify_semver_update` tests | ported |
| `lib/modules/versioning/cargo/index.spec.ts` | 101 | `getPinnedValue` returns `=1.2.3` | `crates/renovate-core/src/versioning/cargo.rs` | `update_summary_tests` | partial |
| `lib/modules/versioning/cargo/index.spec.ts` | — | Build metadata does not create false update | `crates/renovate-core/src/versioning/cargo.rs` | `build_metadata_same_precedence_is_not_update` | ported |
| `lib/modules/versioning/cargo/index.spec.ts` | — | Genuine newer version alongside build-metadata variant is update | `crates/renovate-core/src/versioning/cargo.rs` | `build_metadata_with_actual_newer_version_is_update` | ported |

---

## Datasource release timestamps

| Renovate test file | Line | Renovate test | Rust location | Rust test | Status |
|--------------------|------|---------------|---------------|-----------|--------|
| `lib/modules/datasource/crate/index.spec.ts` | — | `releaseTimestamp` populated from API | `crates/renovate-core/src/datasources/crates_io.rs` | `fetch_version_timestamps_parses_created_at` | ported |
| `lib/modules/datasource/crate/index.spec.ts` | — | 404 returns error | `crates/renovate-core/src/datasources/crates_io.rs` | `fetch_version_timestamps_404_returns_error` | ported |
| `lib/modules/datasource/crate/index.spec.ts` | — | Batch timestamp fetch collects results | `crates/renovate-core/src/datasources/crates_io.rs` | `fetch_timestamps_batch_collects_results` | ported |

---

## `ignorePaths` / `ignoreDeps`

| Renovate test file | Line | Renovate test | Rust location | Rust test | Status |
|--------------------|------|---------------|---------------|-----------|--------|
| `lib/config/options/index.ts` | — | `ignorePaths` glob pattern match | `crates/renovate-core/src/repo_config.rs` | `ignore_paths_glob_excludes`, `ignore_paths_plain_prefix` | ported |
| `lib/config/options/index.ts` | — | `ignoreDeps` exact match | `crates/renovate-core/src/repo_config.rs` | `ignore_deps_parsed` | ported |
| `lib/config/options/index.ts` | — | `ignoreVersions` global list | `crates/renovate-core/src/repo_config.rs` | `ignore_versions_regex_glob`, `ignore_versions_semver_range` | ported |

---

## Pending / not yet ported

The following Renovate test areas have no current Rust coverage and are planned
for future slices.  **Do not list ported items here** — move them to the
relevant section above.

| Renovate spec file | Topic | Priority |
|--------------------|-------|----------|
| `lib/config/presets/index.spec.ts` | Remote preset resolution (`github>`) | high |
| `lib/modules/manager/npm/extract.spec.ts` | npm dep extraction edge cases | high |
| `lib/modules/manager/cargo/extract.spec.ts` | Cargo workspace dep extraction | `crates/renovate-core/src/extractors/cargo.rs` | `workspace_dependencies_extracted`, `workspace_and_member_deps_both_extracted` | ported |
| `lib/workers/repository/updates/branch-name.spec.ts` | Group branch naming | medium |
| `lib/modules/datasource/npm/index.spec.ts` | npm registry lookup | high |
| `lib/modules/datasource/docker/index.spec.ts` | Docker Hub lookup | medium |
| `lib/modules/datasource/github-releases/index.spec.ts` | GitHub Releases lookup | medium |
| `lib/modules/versioning/pep440/index.spec.ts` | Full PEP 440 versioning suite | medium |
| `lib/modules/versioning/docker/index.spec.ts` | Docker versioning (shorter-is-bigger) | medium |
