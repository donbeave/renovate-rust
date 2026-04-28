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

---

## `packageRules` matchers

| Renovate test file | Line | Renovate test | Rust location | Rust test | Status |
|--------------------|------|---------------|---------------|-----------|--------|
| `lib/util/package-rules/index.spec.ts` | — | `matchPackageNames` exact | `crates/renovate-core/src/repo_config.rs` | `enabled_managers_parsed` | partial |
| `lib/util/package-rules/index.spec.ts` | — | `matchPackageNames` regex `/pattern/` | `crates/renovate-core/src/repo_config.rs` | `match_dep_names_regex_disables_dep` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `matchPackageNames` glob `@aws-sdk/**` | `crates/renovate-core/src/repo_config.rs` | `match_dep_names_glob_disables_dep` | ported |
| `lib/util/package-rules/dep-names.ts` | — | `matchDepNames` matches `depName` | `crates/renovate-core/src/repo_config.rs` | `match_dep_names_exact_disables_dep`, `match_dep_names_regex_disables_dep`, `match_dep_names_glob_disables_dep` | ported |
| `lib/util/package-rules/sourceurls.ts` | — | `matchSourceUrls` exact | `crates/renovate-core/src/repo_config.rs` | `source_url_tests::match_source_urls_exact_disables_dep` | ported |
| `lib/util/package-rules/sourceurls.ts` | — | `matchSourceUrls` glob | `crates/renovate-core/src/repo_config.rs` | `source_url_tests::match_source_urls_glob` | ported |
| `lib/util/package-rules/sourceurls.ts` | — | `matchSourceUrls` regex | `crates/renovate-core/src/repo_config.rs` | `source_url_tests::match_source_urls_regex` | ported |
| `lib/util/package-rules/sourceurls.ts` | — | empty `matchSourceUrls` matches all | `crates/renovate-core/src/repo_config.rs` | `source_url_tests::match_source_urls_empty_matches_all` | ported |
| `lib/util/package-rules/current-value.ts` | — | `matchCurrentValue` regex match | `crates/renovate-core/src/repo_config.rs` | `source_url_tests::match_current_value_regex` | ported |
| `lib/util/package-rules/current-value.ts` | — | `matchCurrentValue` exact match | `crates/renovate-core/src/repo_config.rs` | `source_url_tests::match_current_value_exact` | ported |
| `lib/util/package-rules/new-value.ts` | — | `matchNewValue` glob match | `crates/renovate-core/src/repo_config.rs` | `source_url_tests::match_new_value_glob` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `matchDatasources` list | `crates/renovate-core/src/repo_config.rs` | `match_datasources_method_matches_listed_datasource`, `match_datasources_empty_matches_all` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `matchUpdateTypes` major/minor/patch | `crates/renovate-core/src/repo_config.rs` | `is_update_blocked_for_major_but_not_minor` | ported |
| `lib/util/package-rules/index.spec.ts` | — | `matchFileNames` glob | `crates/renovate-core/src/repo_config.rs` | `is_update_blocked_for_file_with_filename_rule` | ported |
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
| `lib/util/package-rules/current-age.ts` | — | `matchCurrentAge` set without timestamp → false | `crates/renovate-core/src/repo_config.rs` | `dep_context_tests::match_current_age_set_without_timestamp_returns_false` | ported |
| `lib/util/package-rules/current-age.ts` | — | `matchCurrentAge` old dep matches `"> 3 days"` | `crates/renovate-core/src/repo_config.rs` | `dep_context_tests::match_current_age_old_dep_matches_gt_constraint` | ported |
| `lib/util/package-rules/current-age.ts` | — | `matchCurrentAge` new dep does not match `"> 3 days"` | `crates/renovate-core/src/repo_config.rs` | `dep_context_tests::match_current_age_new_dep_does_not_match_gt_constraint` | ported |
| `lib/util/package-rules/current-age.ts` | — | `matchCurrentAge` disables dep via DepContext | `crates/renovate-core/src/repo_config.rs` | `dep_context_tests::match_current_age_via_dep_context_disables_dep` | ported |
| `lib/util/package-rules/current-age.ts` | — | `matchCurrentAge` no constraint matches all | `crates/renovate-core/src/repo_config.rs` | `dep_context_tests::match_current_age_none_unset_matches_all` | ported |
| `lib/util/pretty-time.ts` | 50 | `satisfiesDateRange(old, "> 3 days")` → true | `crates/renovate-core/src/schedule.rs` | `date_range_gt_old_timestamp_is_true` | ported |
| `lib/util/pretty-time.ts` | 50 | `satisfiesDateRange(future, "> 3 days")` → false | `crates/renovate-core/src/schedule.rs` | `date_range_gt_future_timestamp_is_false` | ported |
| `lib/util/pretty-time.ts` | 50 | `satisfiesDateRange(recent, "< 3 days")` → true | `crates/renovate-core/src/schedule.rs` | `date_range_lt_recent_timestamp_is_true` | ported |
| `lib/util/pretty-time.ts` | 50 | `satisfiesDateRange(old, "< 3 days")` → false | `crates/renovate-core/src/schedule.rs` | `date_range_lt_old_timestamp_is_false` | ported |
| `lib/util/pretty-time.ts` | 50 | `satisfiesDateRange(old, ">= 1 week")` → true | `crates/renovate-core/src/schedule.rs` | `date_range_gte_old_is_true` | ported |
| `lib/util/pretty-time.ts` | 50 | `satisfiesDateRange(recent, "<= 1 week")` → true | `crates/renovate-core/src/schedule.rs` | `date_range_lte_future_is_true` | ported |
| `lib/util/pretty-time.ts` | 50 | invalid operator → false | `crates/renovate-core/src/schedule.rs` | `date_range_invalid_operator_returns_false` | ported |
| `lib/util/pretty-time.ts` | 50 | invalid timestamp → false | `crates/renovate-core/src/schedule.rs` | `date_range_invalid_timestamp_returns_false` | ported |
| `lib/modules/platform/local/index.ts` | — | `--platform=local` scans CWD | `crates/renovate-core/src/platform/local.rs` | (integration, verified via `renovate --platform=local --dry-run=full`) | ported |
| `lib/util/package-rules/categories.ts` | — | `matchCategories` exact hit | `crates/renovate-core/src/repo_config.rs` | `categories_base_branch_tests::match_categories_exact_hit` | ported |
| `lib/util/package-rules/categories.ts` | — | `matchCategories` any-of-many | `crates/renovate-core/src/repo_config.rs` | `categories_base_branch_tests::match_categories_any_of_many` | ported |
| `lib/util/package-rules/categories.ts` | — | empty `matchCategories` matches all | `crates/renovate-core/src/repo_config.rs` | `categories_base_branch_tests::match_categories_empty_matches_all` | ported |
| `lib/util/package-rules/base-branch.ts` | — | `matchBaseBranches` exact hit | `crates/renovate-core/src/repo_config.rs` | `categories_base_branch_tests::match_base_branches_exact_hit` | ported |
| `lib/util/package-rules/base-branch.ts` | — | `matchBaseBranches` glob `release/*` | `crates/renovate-core/src/repo_config.rs` | `categories_base_branch_tests::match_base_branches_glob` | ported |
| `lib/util/package-rules/base-branch.ts` | — | empty `matchBaseBranches` matches all | `crates/renovate-core/src/repo_config.rs` | `categories_base_branch_tests::match_base_branches_empty_matches_all` | ported |
| `lib/util/package-rules/base-branch.ts` | — | `matchBaseBranches` multiple entries | `crates/renovate-core/src/repo_config.rs` | `categories_base_branch_tests::match_base_branches_multiple_entries` | ported |

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

## Versioning

| Renovate test file | Line | Renovate test | Rust location | Rust test | Status |
|--------------------|------|---------------|---------------|-----------|--------|
| `lib/modules/versioning/semver/index.spec.ts` | — | `isValid` accepts semver and ranges | `crates/renovate-core/src/versioning/semver_generic.rs` | `parse_padded_*` | partial |
| `lib/modules/versioning/semver/index.spec.ts` | — | Major/minor/patch classification | `crates/renovate-core/src/versioning/semver_generic.rs` | `classify_semver_update` tests | ported |
| `lib/modules/versioning/cargo/index.spec.ts` | 101 | `getPinnedValue` returns `=1.2.3` | `crates/renovate-core/src/versioning/cargo.rs` | `update_summary_tests` | partial |
| `lib/modules/versioning/cargo/index.spec.ts` | — | Build metadata does not create false update | `crates/renovate-core/src/versioning/cargo.rs` | `build_metadata_same_precedence_is_not_update` | ported |
| `lib/modules/versioning/cargo/index.spec.ts` | — | Genuine newer version alongside build-metadata variant is update | `crates/renovate-core/src/versioning/cargo.rs` | `build_metadata_with_actual_newer_version_is_update` | ported |

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
for future slices:

| Renovate spec file | Topic | Priority |
|--------------------|-------|----------|
| `lib/util/package-rules/repositories.ts` | `matchRepositories` matcher | ported |
| `lib/config/presets/index.spec.ts` | Remote preset resolution (`github>`) | high |
| `lib/modules/manager/npm/extract.spec.ts` | npm dep extraction edge cases | high |
| `lib/modules/manager/cargo/extract.spec.ts` | Cargo workspace dep extraction | high |
| `lib/workers/repository/updates/branch-name.spec.ts` | Group branch naming | medium |
| `lib/modules/datasource/npm/index.spec.ts` | npm registry lookup | high |
| `lib/modules/datasource/docker/index.spec.ts` | Docker Hub lookup | medium |
| `lib/modules/datasource/github-releases/index.spec.ts` | GitHub Releases lookup | medium |
| `lib/modules/versioning/*/index.spec.ts` | Full versioning suites | medium |
