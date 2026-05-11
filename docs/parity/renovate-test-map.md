# Renovate Test Map

**Overall progress (per-test sections only):** 1009 / 1217 actionable tests ported (83%) — updated 2026-05-11

Legacy summary tables below cover the remaining 47 spec files not yet converted to per-test format (35 pending, 11 partial, 1 not-applicable). They are dominated by non-extract specs — index, parser, integration, lockfile, properties, update — that need a different test-port strategy than the per-test extract sections above.

Status key: `ported` · `pending` · `not-applicable`

> Note: Files below the per-test sections are in the legacy summary format and
> will be converted to per-test format in future iterations.

---

## `lib/modules/manager/ansible-galaxy/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/ansible-galaxy/extract.spec.ts
**Total tests:** 14 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 15 | ported | `ansible_galaxy.rs` | `empty_content_returns_no_deps` | — |
| extracts multiple dependencies from requirements.yml | 19 | ported | `ansible_galaxy.rs` | `requirements01_extracts_thirteen_deps` | — |
| extracts dependencies from a not beautified requirements file | 25 | ported | `ansible_galaxy.rs` | `non_beautified_requirements_extracts_two_deps` | — |
| extracts dependencies from requirements.yml with a space at the end of line | 31 | ported | `ansible_galaxy.rs` | `collections_with_git_url_name_and_version` | — |
| extracts git@ dependencies | 41 | ported | `ansible_galaxy.rs` | `collections_with_source_field_and_git_at_url` | — |
| check if an empty file returns null | 56 | ported | `ansible_galaxy.rs` | `blank_file_returns_no_deps` | — |
| check if a requirements file of other systems returns null | 61 | ported | `ansible_galaxy.rs` | `non_ansible_content_returns_empty` | — |
| check collection style requirements file | 66 | ported | `ansible_galaxy.rs` | `collections1_extracts_fourteen_deps_all_galaxy_hosted` | — |
| check collection style requirements file in reverse order and missing empty line | 73 | ported | `ansible_galaxy.rs` | `collections_before_roles_extracts_all_four` | — |
| check galaxy definition file | 79 | not-applicable | — | — | galaxy.yml uses a metadata format parsed by a separate TS function; Rust extract() produces 0 deps for it |

### `getSliceEndNumber()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| negative start number returns -1 | 87 | not-applicable | — | — | TypeScript internal parsing utility; Rust uses a state machine with no equivalent function |
| a start number bigger then number of lines return -1 | 92 | not-applicable | — | — | TypeScript internal parsing utility; Rust uses a state machine with no equivalent function |
| choose first block | 97 | not-applicable | — | — | TypeScript internal parsing utility; Rust uses a state machine with no equivalent function |
| choose second block | 102 | not-applicable | — | — | TypeScript internal parsing utility; Rust uses a state machine with no equivalent function |

---

## `lib/modules/manager/ansible/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/ansible/extract.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 6 | ported | `ansible.rs` | `empty_returns_empty` | — |
| extracts multiple image lines from docker_container | 10 | ported | `ansible.rs` | `extracts_images` | — |
| extracts multiple image lines from docker_service | 16 | ported | `ansible.rs` | `extracts_docker_service_images` | — |
| extracts image and replaces registry | 22 | ported | `ansible.rs` | `extracts_image_and_replaces_registry` | — |
| extracts image but no replacement | 52 | ported | `ansible.rs` | `extracts_image_without_registry_replacement` | — |
| extracts image and no double replacement | 82 | ported | `ansible.rs` | `extracts_image_without_double_registry_replacement` | — |

---

## `lib/modules/manager/asdf/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/asdf/extract.spec.ts
**Total tests:** 13 | **Ported:** 13 | **Actionable:** 13 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns a result | 6 | ported | `asdf.rs` | `nodejs_maps_to_node_version_datasource` | — |
| provides skipReason for lines with unsupported tooling | 19 | ported | `asdf.rs` | `unknown_tool_gets_skip_reason` | — |
| only captures the first version | 31 | ported | `asdf.rs` | `only_captures_first_version` | — |
| can handle multiple tools in one file | 44 | ported | `asdf.rs` | `extracts_github_releases_tool` (+ 6 others) | — |
| can handle multiple tools with indented versions in one file | 890 | ported | `asdf.rs` | `indented_spacing_still_parses` | — |
| can handle flutter version channel | 923 | ported | `asdf.rs` | `flutter_strips_channel_suffix` | — |
| can handle java jre / jdk | 946 | ported | `asdf.rs` | `java_adoptopenjdk_jdk` (+ 4 others) | — |
| can handle scala v 2 & 3 | 1004 | ported | `asdf.rs` | `scala_v2_uses_scala_scala` (+ 2 others) | — |

### `extractPackageFile() › comment handling › ignores proper comments at the end of lines`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| entry: '${data.entry}' | 1054 | ported | `asdf.rs` | `strips_inline_comments` | — |

### `extractPackageFile() › comment handling`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| invalid comment placements fail to parse | 1069 | ported | `asdf.rs` | `invalid_comment_no_space_fails_parse` | — |
| ignores lines that are just comments | 1076 | ported | `asdf.rs` | `comment_lines_skipped` | — |
| ignores comments across multiple lines | 1081 | ported | `asdf.rs` | `ignores_comments_across_multiple_lines` | — |
| ignores supported tooling with a renovate:ignore comment | 1096 | ported | `asdf.rs` | `renovate_ignore_comment_skips_dep` | — |

---

## `lib/modules/manager/argocd/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/argocd/extract.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 8 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 11 | ported | `argocd.rs` | `empty_content_returns_empty` | — |
| returns null for invalid | 15 | ported | `argocd.rs` | `invalid_yaml_with_trailing_content_returns_empty` | — |
| return null for kubernetes manifest | 21 | ported | `argocd.rs` | `skips_non_argocd_file` | — |
| return null if deps array would be empty | 26 | ported | `argocd.rs` | `malformed_applications_return_empty` | — |
| return result for double quoted argoproj.io apiVersion reference | 34 | ported | `argocd.rs` | `double_quoted_apiversion_accepted` | — |
| return result for single quoted argoproj.io apiVersion reference | 61 | ported | `argocd.rs` | `single_quoted_apiversion_accepted` | — |
| full test | 88 | ported | `argocd.rs` | `full_test_helm_source, full_test_git_source_dep_name_is_full_url, full_test_docker_source_no_protocol, full_test_oci_helm_chart, full_test_kustomize_images` | — |
| supports applicationsets | 203 | ported | `argocd.rs` | `supports_applicationsets` | — |

---

## `lib/modules/manager/batect-wrapper/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/batect-wrapper/extract.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty wrapper file | 9 | ported | `batect_wrapper.rs` | `empty_returns_none` | — |
| returns null for file without version information | 13 | ported | `batect_wrapper.rs` | `no_version_line_returns_none` | — |
| extracts the current version from a valid wrapper script | 17 | ported | `batect_wrapper.rs` | `extracts_version` | — |
| returns the first version from a wrapper script with multiple versions | 31 | ported | `batect_wrapper.rs` | `multiple_version_lines_uses_first` | — |

---

## `lib/modules/manager/batect/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/batect/extract.spec.ts
**Total tests:** 4 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty array for empty configuration file | 41 | ported | `batect.rs` | `empty_returns_empty` | — |
| returns empty array for non-object configuration file | 49 | not-applicable | — | — | Requires async mock filesystem; no Rust equivalent |
| returns an a package file with no dependencies for configuration file without containers or includes | 57 | ported | `batect.rs` | `no_containers_block_returns_empty` | — |
| extracts all available images and bundles from a valid Batect configuration file, including dependencies in included files | 70 | not-applicable | — | — | Requires async mock filesystem; no Rust equivalent |

---

## `lib/modules/manager/buildpacks/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/buildpacks/extract.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid files | 7 | ported | `buildpacks.rs` | `invalid_toml_returns_empty` | — |
| returns null for empty package.toml | 11 | ported | `buildpacks.rs` | `no_io_buildpacks_returns_empty` | — |
| extracts builder and buildpack images | 20 | ported | `buildpacks.rs` | `extracts_registry_deps` (+ `skips_docker_refs`, `no_version_skipped`) | — |

---

## `lib/modules/manager/cargo/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/cargo/extract.spec.ts
**Total tests:** 32 | **Ported:** 14 | **Actionable:** 14 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid toml | 46 | ported | `cargo.rs` | `invalid_toml_returns_error` | — |
| returns null for empty dependencies | 52 | ported | `cargo.rs` | `empty_dependencies_section_returns_empty` | — |
| returns null for empty dev-dependencies | 59 | ported | `cargo.rs` | `empty_dev_dependencies_returns_empty` | — |
| returns null for empty custom target | 66 | ported | `cargo.rs` | `empty_custom_target_returns_empty` | — |
| extracts multiple dependencies simple | 73 | ported | `cargo.rs` | `extracts_simple_string_deps` | — |
| extracts multiple dependencies advanced | 79 | ported | `cargo.rs` | `version_constraint_forms_are_preserved` | — |
| handles inline tables | 85 | ported | `cargo.rs` | `handles_inline_tables` | — |
| handles standard tables | 91 | ported | `cargo.rs` | `extracts_table_deps_with_version` | — |
| extracts platform specific dependencies | 97 | ported | `cargo.rs` | `target_cfg_dependencies_extracted` | — |
| extracts registry urls from .cargo/config.toml | 103 | not-applicable | — | — | Requires async mock filesystem for .cargo/config.toml |
| extracts registry urls from .cargo/config (legacy path) | 112 | not-applicable | — | — | Requires async mock filesystem |
| extracts overridden registry indexes from .cargo/config.toml | 121 | not-applicable | — | — | Requires async mock filesystem |
| extracts overridden source registry indexes from .cargo/config.toml | 180 | not-applicable | — | — | Requires async mock filesystem |
| extracts registries overridden to the default | 205 | not-applicable | — | — | Requires async mock filesystem |
| extracts registries with an empty config.toml | 249 | not-applicable | — | — | Requires async mock filesystem |
| extracts registry urls from environment | 299 | not-applicable | — | — | Requires environment variable injection |
| extracts workspace dependencies | 345 | ported | `cargo.rs` | `workspace_dependencies_extracted` | — |
| skips workspace dependency | 390 | ported | `cargo.rs` | `workspace_dep_is_skipped` | — |
| skips unknown registries | 407 | not-applicable | — | — | Requires async mock filesystem |
| fails to parse cargo config with invalid TOML | 415 | not-applicable | — | — | Requires async mock filesystem |
| ignore cargo config registries with missing index | 424 | not-applicable | — | — | Requires async mock filesystem |
| ignore cargo config source replaced registries with missing index | 433 | not-applicable | — | — | Requires async mock filesystem |
| ignore cargo config with circular registry source replacements | 481 | not-applicable | — | — | Requires async mock filesystem |
| extracts original package name of renamed dependencies | 539 | ported | `cargo.rs` | `renamed_dep_extracts_original_package_name` | — |
| extracts locked versions | 549 | not-applicable | — | — | Requires async mock filesystem for Cargo.lock |
| does not extract locked versions for git dependencies | 567 | not-applicable | — | — | Requires async mock filesystem for Cargo.lock |
| extracts locked versions for renamed packages | 585 | not-applicable | — | — | Requires async mock filesystem for Cargo.lock |
| handles missing locked versions | 601 | not-applicable | — | — | Requires async mock filesystem for Cargo.lock |
| handles invalid versions in the toml file | 617 | not-applicable | — | — | Requires async mock filesystem for Cargo.lock |
| handles invalid lock file | 635 | not-applicable | — | — | Requires async mock filesystem for Cargo.lock |
| should extract project version | 650 | ported | `cargo.rs` | `extracts_project_version` | — |
| should extract project version from workspace | 664 | ported | `cargo.rs` | `extracts_project_version_from_workspace` | — |

---

## `lib/modules/manager/cloudbuild/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/cloudbuild/extract.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 6 | ported | `cloudbuild.rs` | `empty_returns_empty` | — |
| extracts multiple image lines | 10 | ported | `cloudbuild.rs` | `extracts_three_step_images` | — |

---

## `lib/modules/manager/cocoapods/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/cocoapods/extract.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts from simple file | 13 | ported | `cocoapods.rs` | `simple_podfile_fixture` | — |
| extracts from complex file | 42 | ported | `cocoapods.rs` | `complex_podfile_fixture` | — |

---

## `lib/modules/manager/deps-edn/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deps-edn/extract.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `extractPackageFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid file | 6 | ported | `deps_edn.rs` | `invalid_edn_returns_empty` | — |
| extractPackageFile | 10 | ported | `deps_edn.rs` | `extracts_deps` (+ `skips_git_deps`, `skips_local_deps`, `extracts_alias_deps`) | — |

---

## `lib/modules/manager/droneci/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/droneci/extract.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 8 | ported | `droneci.rs` | `empty_returns_empty` | — |
| extracts multiple image lines | 12 | ported | `droneci.rs` | `extracts_drone_fixture_six_deps` | — |

### `modules/manager/droneci/extract`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts image and replaces registry | 19 | ported | `droneci.rs` | `extracts_image_and_replaces_registry` | — |
| extracts image but no replacement | 42 | ported | `droneci.rs` | `extracts_image_without_registry_replacement` | — |
| extracts image and no double replacement | 65 | ported | `droneci.rs` | `extracts_image_without_double_registry_replacement` | — |

---

## `lib/modules/manager/fvm/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/fvm/extract.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid json | 7 | ported | `fvm.rs` | `invalid_json_returns_none` | — |
| returns null for empty flutter sdk version | 13 | ported | `fvm.rs` | `missing_version_returns_none` | — |
| returns null for non string flutter sdk version | 17 | ported | `fvm.rs` | `non_string_flutter_sdk_version_returns_none` | — |
| returns a result for .fvm/fvm_config.json | 26 | ported | `fvm.rs` | `extracts_flutter_sdk_version_key` | — |
| returns a result for .fvmrc | 41 | ported | `fvm.rs` | `extracts_flutter_key` | — |
| supports non range for .fvm/fvm_config.json | 53 | ported | `fvm.rs` | `flutter_sdk_version_channel_extracted` | — |
| supports non range for .fvmrc | 68 | ported | `fvm.rs` | `flutter_channel_extracted` | — |

---

## `lib/modules/manager/glasskube/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/glasskube/extract.spec.ts
**Total tests:** 5 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should extract version and registryUrl | 43 | ported | `glasskube.rs` | `extracts_cluster_package` (+ `extracts_multiple_packages`) | — |

### `extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null for empty packageFiles | 62 | not-applicable | — | — | Requires async mock filesystem access; no Rust equivalent |
| should skip package with non-existing repo | 67 | ported | `glasskube.rs` | `skips_non_glasskube_files` | — |
| should extract registryUrl from repo in other file | 85 | not-applicable | — | — | Requires async mock filesystem access; no Rust equivalent |
| should extract registryUrl from default repo in other file | 107 | not-applicable | — | — | Requires async mock filesystem access; no Rust equivalent |

---

## `lib/modules/manager/gleam/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gleam/extract.spec.ts
**Total tests:** 9 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `modules/manager/gleam/extract`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should extract dev and prod dependencies | 8 | ported | `gleam.rs` | `both_sections` | — |
| should extract dev only dependencies | 41 | ported | `gleam.rs` | `extracts_dev_dependencies` | — |
| should return null when no dependencies are found | 65 | ported | `gleam.rs` | `no_deps_section_returns_empty` | — |
| should return null when gleam.toml is invalid | 82 | ported | `gleam.rs` | `invalid_toml_returns_empty` | — |
| should return locked versions | 91 | not-applicable | — | — | Requires async mock filesystem access; no Rust equivalent |
| should fail to extract locked version | 119 | not-applicable | — | — | Requires async mock filesystem access; no Rust equivalent |
| should fail to find locked version in range | 138 | not-applicable | — | — | Requires async mock filesystem access; no Rust equivalent |
| should handle invalid versions in lock file | 166 | not-applicable | — | — | Requires async mock filesystem access; no Rust equivalent |
| should handle lock file parsing and extracting errors | 193 | not-applicable | — | — | Requires async mock filesystem access; no Rust equivalent |

---

## `lib/modules/manager/git-submodules/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/git-submodules/extract.spec.ts
**Total tests:** 8 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| empty submodule returns null | 48 | ported | `git_submodules.rs` | `empty_content_returns_no_deps` | — |
| currentValue is unset when no branch is specified | 52 | ported | `git_submodules.rs` | `single_submodule_no_branch` | — |
| given branch is used when branch is specified | 58 | ported | `git_submodules.rs` | `single_submodule_with_branch` | — |
| submodule packageName is constructed from relative path | 64 | ported | `git_submodules.rs` | `https_url_strips_git_suffix` (+ 4 others) | — |
| fallback to current branch if special value is detected | 89 | ported | `git_submodules.rs` | `branch_dot_normalized_to_none` | — |
| given semver version is extracted from branch and versioning is set to semver | 127 | ported | `git_submodules.rs` | `semver_and_non_semver_branches` | — |

### `extractPackageFile() › submodule sourceUrl is determined from packageName`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| when using SSH clone URL | 73 | ported | `git_submodules.rs` | `ssh_clone_url_converted_to_https_for_source_url` | — |
| when using a relative path | 80 | not-applicable | — | — | Relative URL resolution requires knowledge of git origin remote; not available in static extractor |

---

## `lib/modules/manager/github-actions/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/github-actions/extract.spec.ts
**Total tests:** 27 | **Ported:** 20 | **Actionable:** 20 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 42 | ported | `github_actions.rs` | `empty_content_returns_empty` | — |
| returns null for invalid yaml | 48 | ported | `github_actions.rs` | `invalid_yaml_returns_empty` | — |
| extracts multiple docker image lines from yaml configuration file | 54 | ported | `github_actions.rs` | `docker_container_inline` (+ 5 others) | — |
| extracts multiple action tag lines from yaml configuration file | 65 | ported | `github_actions.rs` | `extracts_simple_action` | — |
| use github.com as registry when no settings provided | 79 | not-applicable | — | — | registryUrls not produced by Rust extractor |
| use github.enterprise.com first and then github.com as registry running against github.enterprise.com | 87 | not-applicable | — | — | registryUrls not produced by Rust extractor |
| use github.enterprise.com first and then github.com as registry running against github.enterprise.com/api/v3 | 102 | not-applicable | — | — | registryUrls not produced by Rust extractor |
| use github.com only as registry when running against non-GitHub | 117 | not-applicable | — | — | registryUrls not produced by Rust extractor |
| use github.com only as registry when running against github.com | 129 | not-applicable | — | — | registryUrls not produced by Rust extractor |
| use github.com only as registry when running against api.github.com | 141 | not-applicable | — | — | registryUrls not produced by Rust extractor |
| extracts multiple action tag lines with double quotes and comments | 153 | ported | `github_actions.rs` | `quoted_action_is_parsed` | — |
| maintains quotes | 217 | ported | `github_actions.rs` | `single_and_double_quoted_uses_parsed` | — |
| maintains spaces between hash and comment | 299 | ported | `github_actions.rs` | `inline_comment_stripped` | — |
| extracts tags in different formats | 352 | ported | `github_actions.rs` | `comment_version_formats` | — |
| extracts non-semver ref automatically | 484 | ported | `github_actions.rs` | `non_semver_ref_extracted` | — |
| extracts pinned non-semver ref with digest | 504 | ported | `github_actions.rs` | `pinned_non_semver_ref_with_digest` | — |
| disables naked SHA pins without version comment | 527 | ported | `github_actions.rs` | `full_sha_pin_skipped` | — |
| disables naked short SHA pins without version comment | 546 | ported | `github_actions.rs` | `short_sha_pin_skipped` | — |
| does not disable SHA pins with version comment | 565 | ported | `github_actions.rs` | `full_sha_with_version_comment_not_skipped` | — |
| does not disable short SHA pins with version comment | 590 | ported | `github_actions.rs` | `short_sha_with_version_comment_not_skipped` | — |
| extracts actions with fqdn | 614 | ported | `github_actions.rs` | `extracts_actions_with_fqdn` | — |
| extracts multiple action runners from yaml configuration file | 673 | ported | `github_actions.rs` | `runner_simple_ubuntu` (+ 4 others) | — |
| extracts x-version from actions/setup-x | 741 | ported | `github_actions.rs` | `setup_x_extracts_versioned_deps` | — |
| handles actions/setup-x without x-version field | 873 | ported | `github_actions.rs` | `setup_x_without_version_returns_only_action_dep`, `setup_x_missing_version_key_emits_unspecified` | — |
| extracts x-version from actions/setup-x in composite action | 891 | ported | `github_actions.rs` | `setup_x_composite_action` | — |
| logs unknown schema | 1023 | not-applicable | — | — | Tests log output; no Rust equivalent |
| extract from $step.uses | 1033 | ported | `github_actions.rs` | `community_trivy_*`, `community_pnpm_*`, `community_bun_*`, `community_ruby_*`, `community_pyright_*`, `community_jaxxstorm_*`, `community_pixi_*`, `community_zizmor_*`, `community_docker_*`, `community_setup_uv_*` (14 tests) | — |

---

## `lib/modules/manager/gitlabci/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gitlabci/extract.spec.ts
**Total tests:** 14 | **Ported:** 10 | **Actionable:** 10 | **Status:** ported

### `extractAllPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts from empty file | 22 | ported | `gitlabci.rs` | `empty_content_returns_no_deps` | — |

### `extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 28 | not-applicable | — | — | Uses extractAllPackageFiles with mock filesystem; no Rust equivalent |
| extracts from multidoc yaml | 36 | ported | `gitlabci.rs` | `multidoc_yaml_extracts_from_all_docs` | — |
| extracts multiple included image lines | 46 | not-applicable | — | — | Uses extractAllPackageFiles with mock filesystem; no Rust equivalent |
| extracts named services | 57 | ported | `gitlabci.rs` | `extracts_services` | — |
| extracts multiple named services | 66 | not-applicable | — | — | Uses extractAllPackageFiles with mock filesystem; no Rust equivalent |
| extracts multiple image lines | 75 | ported | `gitlabci.rs` | `extracts_top_level_image` | — |
| extracts multiple image lines with comments | 94 | ported | `gitlabci.rs` | `extracts_images_with_comment_lines` | — |
| catches errors | 110 | not-applicable | — | — | Uses extractAllPackageFiles with mock filesystem; no Rust equivalent |
| skips images with variables | 118 | ported | `gitlabci.rs` | `variable_image_has_skip_reason` | — |
| extract images from dependency proxy | 172 | ported | `gitlabci.rs` | `dependency_proxy_prefix_stripped` | — |
| extract images via registry aliases | 229 | ported | `gitlabci.rs` | `extract_images_via_registry_aliases` | — |
| extracts component references via registry aliases | 299 | ported | `gitlabci.rs` | `extracts_component_references_via_registry_aliases` | — |
| extracts component references | 377 | ported | `gitlabci.rs` | `extracts_component_references` | — |

---

## `lib/modules/manager/gomod/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gomod/extract.spec.ts
**Total tests:** 21 | **Ported:** 17 | **Actionable:** 17 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 12 | ported | `gomod.rs` | `empty_content_returns_empty` | — |
| extracts single-line requires | 16 | ported | `gomod.rs` | `single_line_require` | — |
| extracts multi-line requires | 26 | ported | `gomod.rs` | `require_block` | — |
| ignores empty spaces in multi-line requires | 34 | ported | `gomod.rs` | `empty_lines_inside_require_block` | — |
| extracts replace directives from multi-line and single line | 48 | ported | `gomod.rs` | `replace_directives_multi_line_and_single_line` | — |
| extracts replace directives from non-public module path | 136 | ported | `gomod.rs` | `replace_directive_non_public_module_path` | — |
| ignores exclude directives from multi-line and single line | 193 | ported | `gomod.rs` | `exclude_block_ignored` | — |
| extracts the toolchain directive | 212 | ported | `gomod.rs` | `toolchain_directive_extracted` | — |
| extracts single-line tool directives | 263 | ported | `gomod.rs` | `tool_directive_single_line_ignored` | — |
| extracts multi-line tool directives | 282 | ported | `gomod.rs` | `tool_directive_multi_line_ignored` | — |
| extracts tool directives with required modules | 304 | ported | `gomod.rs` | `tool_directive_with_required_module_not_indirect` | — |
| extracts tool directives of sub-modules | 323 | ported | `gomod.rs` | `tool_directive_sub_modules_disable_non_matching_indirects` | — |
| extracts tool directives with exact match | 370 | ported | `gomod.rs` | `tool_directive_exact_match_keeps_indirect_enabled` | — |
| extracts tool directives with no matching dependencies | 389 | ported | `gomod.rs` | `tool_directive_alone_produces_no_deps` | — |
| ignores directives unrelated to dependencies | 402 | ported | `gomod.rs` | `unrelated_directives_ignored` | — |
| marks placeholder pseudo versions with skipReason invalid-version | 426 | ported | `gomod.rs` | `placeholder_pseudo_versions_have_skip_reason` | — |
| parses go $version directive | 528 | ported | `gomod.rs` | `go_directive_extracted` | — |
| the extracted version can be used as a SemVer constraint | 582 | not-applicable | — | — | Tests versioning API (isValid/matches), not the extractor |
| matches version 1.19, even though it is not valid SemVer | 586 | not-applicable | — | — | Tests versioning API (isValid/matches), not the extractor |
| matches the current SemVer minor | 590 | not-applicable | — | — | Tests versioning API (isValid/matches), not the extractor |
| does not match the next SemVer minor | 595 | not-applicable | — | — | Tests versioning API (isValid/matches), not the extractor |

---

## `lib/modules/manager/gradle/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gradle/extract.spec.ts
**Total tests:** 30 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null | 37 | ported | `gradle.rs` | `empty_returns_empty` | — |
| logs a warning in case parseGradle throws an exception | 52 | not-applicable | — | — | Tests warning log output; no Rust equivalent |
| skips versions composed from multiple variables | 71 | ported | `gradle.rs` | `skips_variable_references` | — |
| extracts from cross-referenced files | 97 | not-applicable | — | — | Requires cross-file extraction; single-file extractor only |
| resolves versions in build.gradle.kts | 125 | not-applicable | — | — | Requires cross-file Kotlin object resolution |
| resolves cross-file Kotlin objects | 191 | not-applicable | — | — | Requires cross-file analysis |
| inherits gradle variables | 311 | not-applicable | — | — | Requires multi-file variable inheritance |
| filters duplicate dependency findings | 341 | ported | `gradle.rs` | `deduplicates_same_dep` | — |
| ensures depType is assigned | 385 | not-applicable | — | — | Requires multi-file extraction with filesystem mock |

### `extractPackageFile() › registry URLs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deduplicates registry urls | 414 | not-applicable | — | — | Requires multi-file repository registry resolution |
| interpolates registry URLs | 451 | not-applicable | — | — | Requires multi-file repository registry resolution |
| supports separate registry URLs for plugins | 507 | not-applicable | — | — | Requires multi-file repository registry resolution |

### `extractPackageFile() › registry URLs › content descriptors › simple descriptor matches`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input \| $output | 568 | not-applicable | — | — | Tests TypeScript-internal matchesContentDescriptor utility |

### `extractPackageFile() › registry URLs › content descriptors › multiple descriptors`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| if both includes and excludes exist, dep must match include and not match exclude | 609 | not-applicable | — | — | Tests configuration filtering logic |
| if only includes exist, dep must match at least one include | 635 | not-applicable | — | — | Tests configuration filtering logic |
| if only excludes exist, dep must match not match any exclude | 653 | not-applicable | — | — | Tests configuration filtering logic |

### `extractPackageFile() › registry URLs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts content descriptors | 672 | not-applicable | — | — | Requires multi-file repository registry resolution |
| exclusiveContent | 775 | not-applicable | — | — | Requires multi-file repository registry resolution |
| exclusiveContent with repeated repository definition | 823 | not-applicable | — | — | Requires multi-file repository registry resolution |

### `extractPackageFile() › version catalogs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works with dependency catalogs | 889 | not-applicable | — | — | Requires cross-file version catalog resolution |
| provides versions from external version catalogs to gradle files | 1006 | not-applicable | — | — | Requires cross-file version catalog resolution |
| provides versions to gradle files with changed default catalog name | 1061 | not-applicable | — | — | Requires cross-file version catalog resolution |
| ignores version catalog accessor with non-get provider method | 1106 | not-applicable | — | — | Requires multi-file extractAllPackageFiles with cross-file version catalog resolution |
| aligns sharedVariableName if version reference has multiple aliases | 1127 | not-applicable | — | — | Requires multi-file extractAllPackageFiles with cross-file version catalog resolution |

### `extractPackageFile() › apply from`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| loads further scripts using apply from statements | 1175 | not-applicable | — | — | Requires multi-file extractAllPackageFiles; apply from resolution crosses file boundaries |
| works with files in sub-directories | 1269 | not-applicable | — | — | Requires multi-file extractAllPackageFiles; subdirectory traversal |
| prevents recursive apply from calls | 1304 | not-applicable | — | — | Requires multi-file extractAllPackageFiles; circular dependency detection |
| prevents inclusion of non-Gradle files | 1319 | not-applicable | — | — | Requires multi-file extractAllPackageFiles |

### `extractPackageFile() › gradle-consistent-versions plugin`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses versions files | 1335 | not-applicable | — | — | Requires multi-file: versions.props + versions.lock |
| plugin not used due to lockfile not a GCV lockfile | 1385 | not-applicable | — | — | Requires multi-file lockfile detection |
| plugin not used due to lockfile missing | 1401 | not-applicable | — | — | Requires multi-file lockfile detection |

---

## `lib/modules/manager/helm-values/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/helm-values/extract.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid yaml file content | 26 | ported | `helm_values.rs` | `invalid_yaml_returns_empty` | — |
| returns null for empty yaml file content | 31 | ported | `helm_values.rs` | `empty_returns_empty` | — |
| extracts from values.yaml correctly with same structure as "helm create" | 36 | ported | `helm_values.rs` | `helm_create_default_values` | — |
| extracts from complex values file correctly | 52 | ported | `helm_values.rs` | `inline_string_form` (+ `multiple_images`) | — |
| extract data from file with multiple documents | 62 | ported | `helm_values.rs` | `multidoc_yaml_extracts_nested_images` | — |
| extract data from file with registry aliases | 85 | ported | `helm_values.rs` | `registry_prefix_combined` | — |

---

## `lib/modules/manager/helmsman/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/helmsman/extract.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if empty | 9 | ported | `helmsman.rs` | `empty_file_returns_empty` | — |
| returns null if extracting non helmsman yaml file | 16 | ported | `helmsman.rs` | `app_with_no_chart_or_version_has_skip_reason` | — |
| returns null if apps not defined | 23 | ported | `helmsman.rs` | `invalid_yaml_returns_empty` | — |
| extract deps | 29 | ported | `helmsman.rs` | `extract_deps_validhelmsfile` (+ `extracts_helm_deps`, `skips_missing_version`, `skips_unknown_repo`) | — |

---

## `lib/modules/manager/hermit/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/hermit/extract.spec.ts
**Total tests:** 2 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `extractPackageFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should list packages on command success | 11 | ported | `hermit.rs` | `extracts_versioned_packages` | — |
| should throw error on execution failure | 75 | not-applicable | — | — | Requires mock readdir failure; no Rust equivalent |

---

## `lib/modules/manager/bitbucket-pipelines/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bitbucket-pipelines/extract.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 6 | ported | `bitbucket_pipelines.rs` | `empty_returns_empty` | — |
| returns null for malformed | 12 | ported | `bitbucket_pipelines.rs` | `malformed_image_object_without_name_returns_empty` | — |
| extracts dependencies | 22 | ported | `bitbucket_pipelines.rs` | `extracts_full_fixture_seven_deps` | — |
| extracts dependencies with registryAlias | 82 | ported | `bitbucket_pipelines.rs` | `extracts_dependencies_with_registry_alias` | — |

---

## `lib/modules/manager/bitrise/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bitrise/extract.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null on an empty file | 7 | ported | `bitrise.rs` | `empty_file_returns_empty` | — |
| returns a valid file | 11 | ported | `bitrise.rs` | `extracts_plain_step` | — |
| returns a valid file with custom default_step_lib_source | 34 | ported | `bitrise.rs` | `extracts_custom_default_registry` | — |
| extracts git and path prefixes | 75 | ported | `bitrise.rs` | `extracts_git_step` | — |
| handles workflows without steps | 114 | ported | `bitrise.rs` | `workflow_without_steps_ignored` | — |
| extracts Bitrise library reference | 142 | ported | `bitrise.rs` | `custom_steplib_reference` | — |

---

## `lib/modules/manager/gradle-wrapper/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gradle-wrapper/extract.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 8 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for property file without distributionUrl | 24 | ported | `gradle_wrapper.rs` | `no_distribution_url_returns_none` | — |
| returns null for property file with unsupported distributionUrl format | 28 | ported | `gradle_wrapper.rs` | `unsupported_url_format_returns_none` | — |
| extracts version for property file with distribution type "bin" in distributionUrl | 33 | ported | `gradle_wrapper.rs` | `extracts_bin_version` | — |
| extracts version for property file with distribution type "all" in distributionUrl | 47 | ported | `gradle_wrapper.rs` | `extracts_all_version` | — |
| extracts version for property file with prerelease version in distributionUrl | 61 | ported | `gradle_wrapper.rs` | `prerelease_version_extracted` | — |
| extracts version for property file with unnecessary whitespace in distributionUrl | 75 | ported | `gradle_wrapper.rs` | `whitespace_around_value_handled` | — |
| extracts version for property file with custom distribution of type "bin" in distributionUrl | 89 | ported | `gradle_wrapper.rs` | `custom_distribution_bin_extracted` | — |
| extracts version for property file with custom distribution of type "all" in distributionUrl | 103 | ported | `gradle_wrapper.rs` | `custom_distribution_all_extracted` | — |

---

## `lib/modules/manager/buildkite/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/buildkite/extract.spec.ts
**Total tests:** 11 | **Ported:** 11 | **Actionable:** 11 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 7 | ported | `buildkite.rs` | `empty_content_returns_no_deps` | — |
| extracts simple single plugin | 11 | ported | `buildkite.rs` | `two_part_plugin` | — |
| extracts multiple plugins in same file | 22 | ported | `buildkite.rs` | `one_part_plugin` | — |
| adds skipReason | 47 | ported | `buildkite.rs` | `non_semver_version_skipped` | — |
| extracts arrays of plugins | 70 | ported | `buildkite.rs` | `array_plugins_extracted` | — |
| extracts git-based plugins | 92 | ported | `buildkite.rs` | `github_url_plugin` | — |
| extracts git-based plugin with .git at the end of its name | 105 | ported | `buildkite.rs` | `github_enterprise_ssh_url_with_git_suffix` | — |
| extracts plugins outside plugins sections | 121 | ported | `buildkite.rs` | `ssh_url_plugin_outside_plugins_section` | — |
| extracts plugin with preceding ? | 140 | ported | `buildkite.rs` | `yaml_question_mark_prefix` | — |
| extracts plugin tags from bitbucket | 155 | ported | `buildkite.rs` | `bitbucket_plugin_extracted` | — |
| extracts plugin tags with quotes | 178 | ported | `buildkite.rs` | `single_quoted_plugin` | — |

---

## `lib/modules/manager/circleci/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/circleci/extract.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 12 | ported | `circleci.rs` | `empty_content_returns_no_deps` | — |
| handles registry alias | 16 | ported | `circleci.rs` | `handles_registry_alias` | — |
| extracts multiple image and resolves yaml anchors | 48 | ported | `circleci.rs` | `fixture_config_resolves_yaml_anchor_images` | — |
| extracts orbs too | 93 | ported | `circleci.rs` | `extracts_orbs` | — |
| extracts image without leading dash | 200 | ported | `circleci.rs` | `anchor_image_without_leading_dash_is_resolved` | — |
| extracts and exclude android images | 226 | ported | `circleci.rs` | `machine_image_not_extracted` | — |
| extracts orbs without jobs | 237 | ported | `circleci.rs` | `extracts_orbs_without_jobs` | — |
| extracts executors | 251 | ported | `circleci.rs` | `executor_docker_image_extracted` | — |
| extracts orb definitions | 273 | ported | `circleci.rs` | `extracts_orb_definitions` | — |

---

## `lib/modules/manager/composer/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/composer/extract.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 10 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid json | 24 | ported | `composer.rs` | `invalid_json_returns_error` | — |
| returns null for empty deps | 28 | ported | `composer.rs` | `empty_content_ok` | — |
| extracts dependencies with no lock file | 32 | ported | `composer.rs` | `extracts_regular_deps` (+ extracts_dev_deps, composer1_fixture, composer1_fixture_has_33_deps, php_constraint_skipped, ext_skipped, lib_skipped, dev_master_skipped, x_dev_skipped) | — |
| extracts registryUrls | 38 | ported | `composer.rs` | `extracts_registry_urls` | — |
| extracts object registryUrls | 81 | ported | `composer.rs` | `extracts_object_registry_urls` | — |
| extracts repositories and registryUrls | 186 | ported | `composer.rs` | `extracts_repositories_and_registry_urls` | — |
| extracts bitbucket repositories and registryUrls | 219 | ported | `composer.rs` | `extracts_bitbucket_repositories` | — |
| extracts object repositories and registryUrls with lock file | 248 | ported | `composer.rs` | `extracts_object_repositories_and_registry_urls_with_lock_file` | — |
| skips path dependencies | 284 | ported | `composer.rs` | `path_dependency_skipped` | — |
| extracts dependencies with lock file | 313 | ported | `composer.rs` | `extracts_dependencies_with_empty_lock_file` | — |

---

## `lib/modules/manager/conan/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/conan/extract.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `extractPackageFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 10 | ported | `conan.rs` | `empty_returns_empty` | — |
| extracts multiple image lines from conanfile.txt | 14 | ported | `conan.rs` | `extracts_full_conanfile_txt_fixture` | — |
| extracts multiple 0 lines from conanfile.txt | 129 | ported | `conan.rs` | `conanfile_without_requires_section_returns_empty` | — |
| extracts multiple image lines from conanfile.py | 134 | ported | `conan.rs` | `extracts_py_requires` | — |

---

## `lib/modules/manager/copier/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/copier/extract.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts repository and version from .copier-answers.yml | 6 | ported | `copier.rs` | `extracts_github_url` | — |
| extracts repository and version from .copier-answers.yml with ssh URL | 25 | ported | `copier.rs` | `ssh_url_src_path_extracted` | — |
| extracts repository and version from .copier-answers.yml with ssh URL and non-bare Repo | 44 | ported | `copier.rs` | `non_github_ssh_url_extracted` | — |
| extracts repository and version from .copier-answers.yml with ssh URL and a username different from git | 63 | ported | `copier.rs` | `non_git_username_ssh_url_extracted` | — |
| extracts and strips git+ prefix from $srcPath | 84 | ported | `copier.rs` | `strips_git_plus_prefix` | — |
| returns null for invalid .copier-answers.yml | 119 | ported | `copier.rs` | `invalid_yaml_returns_none` | — |
| returns null for invalid _src_path | 128 | ported | `copier.rs` | `non_url_src_path_extracted_without_github_repo` | — |
| returns null for missing _commit field | 137 | ported | `copier.rs` | `missing_commit_returns_none` | — |
| returns null for missing _src_path field | 145 | ported | `copier.rs` | `missing_src_path_returns_none` | — |

---

## `lib/modules/manager/crossplane/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/crossplane/extract.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 12 | ported | `crossplane.rs` | `empty_content_returns_empty` | — |
| strips invalid templates | 16 | ported | `crossplane.rs` | `invalid_template_returns_empty` | — |
| return null for kubernetes manifest | 20 | ported | `crossplane.rs` | `skips_non_crossplane_files` | — |
| return invalid-value if deps are not valid images and ignore if missing | 25 | ported | `crossplane.rs` | `malformed_packages_produce_invalid_value_dep` | — |
| return result for double quoted pkg.crossplane.io apiVersion reference | 37 | ported | `crossplane.rs` | `double_quoted_api_version_extracted` | — |
| return result for single quoted pkg.crossplane.io apiVersion reference | 58 | ported | `crossplane.rs` | `single_quoted_api_version_extracted` | — |
| return no results for invalid resource | 79 | ported | `crossplane.rs` | `reports_missing_package` | — |
| full test | 94 | ported | `crossplane.rs` | `extracts_valid_packages_full_test` | — |
| should work even if there are other resources in the file | 137 | ported | `crossplane.rs` | `handles_multi_document` | — |

---

## `lib/modules/manager/crow/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/crow/extract.spec.ts
**Total tests:** 15 | **Ported:** 15 | **Actionable:** 15 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 6 | ported | `crow.rs` | `empty_returns_empty` | — |
| returns null for non-object YAML | 10 | ported | `crow.rs` | `no_image_keys_returns_empty` | — |
| returns null for malformed YAML | 15 | ported | `crow.rs` | `malformed_yaml_returns_empty` | — |
| extracts multiple image lines | 19 | ported | `crow.rs` | `extracts_pipeline_images` | — |
| extracts image and replaces registry | 164 | ported | `crow.rs` | `extracts_image_and_replaces_registry` | — |
| extracts image but no replacement | 194 | ported | `crow.rs` | `extracts_image_without_registry_replacement` | — |
| extracts image and no double replacement | 224 | ported | `crow.rs` | `extracts_image_without_double_registry_replacement` | — |
| extracts the 1.0.0 version | 255 | ported | `crow.rs` | `extracts_semver_version_from_steps` | — |
| should parse multiple sources of dependencies together | 281 | ported | `crow.rs` | `extracts_from_clone_and_steps_sections` | — |
| return dependency when a plugin-git is cloned | 321 | ported | `crow.rs` | `clone_section` | — |
| return null when no dependencies are provided | 348 | ported | `crow.rs` | `no_dependencies_returns_empty` | — |
| handles empty pipeline section gracefully | 362 | ported | `crow.rs` | `empty_pipeline_object_is_skipped` | — |
| returns null when pipeline keys exist but contain no valid images | 390 | ported | `crow.rs` | `pipeline_without_valid_images_returns_empty` | — |
| extracts images from array-based steps format | 408 | ported | `crow.rs` | `steps_as_array` | — |
| extracts images from mixed array and object formats | 447 | ported | `crow.rs` | `extracts_images_from_mixed_array_and_object_formats` | — |

---

## `lib/modules/manager/devbox/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/devbox/extract.spec.ts
**Total tests:** 13 | **Ported:** 13 | **Actionable:** 13 | **Status:** ported

### `extractPackageFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when the devbox JSON file is empty | 6 | ported | `devbox.rs` | `empty_returns_empty` | — |
| returns null when the devbox JSON file is malformed | 11 | ported | `devbox.rs` | `invalid_json_returns_empty` | — |
| returns null when the devbox JSON file has no packages | 16 | ported | `devbox.rs` | `no_packages_key_returns_empty` | — |
| returns a package dependency when the devbox JSON file has a single package | 21 | ported | `devbox.rs` | `array_form` | — |
| returns a package dependency when the devbox JSON file has a single package with a version object | 42 | ported | `devbox.rs` | `object_with_version_field` | — |
| returns invalid-version when the devbox JSON file has a single package with an invalid version | 65 | ported | `devbox.rs` | `invalid_semver_range_flagged` | — |
| returns a package dependency when the devbox JSON file has multiple packages | 89 | ported | `devbox.rs` | `valid_versions_have_no_skip_reason` | — |
| returns a package dependency when the devbox JSON file has multiple packages with in a packages object | 115 | ported | `devbox.rs` | `object_form` | — |
| returns a package dependency when the devbox JSON file has multiple packages with package objects | 144 | ported | `devbox.rs` | `object_form_with_nested_version` | — |
| returns invalid dependencies | 177 | ported | `devbox.rs` | `mixed_valid_and_invalid_versions` | — |
| returns invalid dependencies with package objects | 213 | ported | `devbox.rs` | `object_form_mixed_valid_invalid` | — |
| returns invalid dependencies from the packages array | 251 | ported | `devbox.rs` | `array_form_with_invalid_and_no_version` | — |
| returns null if there are no dependencies | 288 | ported | `devbox.rs` | `empty_packages_array_returns_empty` | — |

---

## `lib/modules/manager/devcontainer/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/devcontainer/extract.spec.ts
**Total tests:** 15 | **Ported:** 15 | **Actionable:** 15 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when the dev container JSON file is empty | 10 | ported | `devcontainer.rs` | `empty_content_returns_empty` | — |
| returns null when the dev container JSON file contents are malformed | 22 | ported | `devcontainer.rs` | `invalid_json_returns_empty` | — |
| tests if JSONC can be parsed | 34 | ported | `devcontainer.rs` | `jsonc_with_comments_and_trailing_commas` | — |
| returns feature image deps when only the features property is defined in dev container JSON file | 72 | ported | `devcontainer.rs` | `extracts_node_feature_and_version` | — |
| returns image and feature image deps when both image and features properties are defined in dev container JSON file | 124 | ported | `devcontainer.rs` | `image_and_feature_combined` | — |
| returns image dep when only the image property is defined in dev container JSON file | 174 | ported | `devcontainer.rs` | `extracts_image` | — |
| returns null when the only feature property is malformed and no image property is defined in dev container JSON file | 207 | ported | `devcontainer.rs` | `malformed_feature_key_returns_empty` | — |
| returns null when the features property is malformed and no image property is defined in dev container JSON file | 227 | ported | `devcontainer.rs` | `features_as_string_returns_empty` | — |
| returns null when the image property is malformed and no features are defined in dev container JSON file | 245 | ported | `devcontainer.rs` | `typo_in_image_key_returns_empty` | — |
| returns null when no image or features properties are defined in dev container JSON file | 263 | ported | `devcontainer.rs` | `empty_object_returns_empty` | — |
| returns null when the features property is null and no image property is defined in dev container JSON file | 278 | ported | `devcontainer.rs` | `null_features_value_returns_empty` | — |
| returns null when the features property is not defined and the image property is null in dev container JSON file | 296 | ported | `devcontainer.rs` | `no_image_returns_empty` | — |
| returns null when both the image and features properties are null | 314 | ported | `devcontainer.rs` | `both_null_returns_empty` | — |
| returns only docker dependencies when non-docker feature types are defined beneath the features property in dev container JSON file | 333 | ported | `devcontainer.rs` | `feature_without_version_skipped_from_version_deps` (+ `local_feature_path_excluded_from_version_deps`) | — |
| parses known tool versions | 372 | ported | `devcontainer.rs` | `extracts_go_feature_and_version` | — |

---

## `lib/modules/manager/docker-compose/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/docker-compose/extract.spec.ts
**Total tests:** 13 | **Ported:** 13 | **Actionable:** 13 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 12 | ported | `docker_compose.rs` | `empty_content_returns_empty` | — |
| returns null for non-object YAML | 16 | ported | `docker_compose.rs` | `non_object_yaml_returns_empty` | — |
| returns null for malformed YAML | 20 | ported | `docker_compose.rs` | `malformed_yaml_returns_empty` | — |
| extracts multiple image lines for version 1 | 24 | ported | `docker_compose.rs` | `renovate_fixture_1_v1_format` | — |
| extracts multiple image lines for version 3 | 30 | ported | `docker_compose.rs` | `extracts_images_from_compose_v3` | — |
| extracts multiple image lines for version 3 without set version key | 36 | ported | `docker_compose.rs` | `no_version_key_extracts_eight_deps` | — |
| extracts default variable values for version 3 | 42 | ported | `docker_compose.rs` | `variable_interpolation_is_skipped` | — |
| extracts can parse yaml tags for version 3 | 59 | ported | `docker_compose.rs` | `yaml_tags_do_not_break_extraction` | — |
| extracts image and replaces registry | 87 | ported | `docker_compose.rs` | `extracts_image_and_replaces_registry` | — |
| extracts image but no replacement | 115 | ported | `docker_compose.rs` | `extracts_image_without_registry_replacement` | — |
| extracts image and no double replacement | 143 | ported | `docker_compose.rs` | `extracts_image_without_double_registry_replacement` | — |
| extracts image of templated compose file | 172 | ported | `docker_compose.rs` | `extracts_image_from_templated_compose_file` | — |
| extract images from fragments | 198 | ported | `docker_compose.rs` | `extracts_image_from_yaml_anchor_fragment` | — |

---

## `lib/modules/manager/dockerfile/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/dockerfile/extract.spec.ts
**Total tests:** 75 | **Ported:** 66 | **Actionable:** 66 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles no FROM | 14 | ported | `dockerfile.rs` | `no_from_returns_empty` | — |
| handles naked dep | 19 | ported | `dockerfile.rs` | `extracts_image_without_tag` | — |
| handles run --mount=from | 36 | ported | `dockerfile.rs` | `run_mount_from_extracts_external_images` | — |
| is case insensitive | 72 | ported | `dockerfile.rs` | `from_is_case_insensitive` | — |
| handles tag | 89 | ported | `dockerfile.rs` | `extracts_image_and_tag` | — |
| handles digest | 106 | ported | `dockerfile.rs` | `extracts_image_with_digest_only` | — |
| handles tag and digest | 129 | ported | `dockerfile.rs` | `extracts_image_with_digest` | — |
| handles from as | 152 | ported | `dockerfile.rs` | `as_alias_does_not_become_dep` | — |
| handles comments | 173 | ported | `dockerfile.rs` | `commented_from_ignored` | — |
| handles custom hosts | 194 | ported | `dockerfile.rs` | `extracts_image_with_custom_host` | — |
| handles custom hosts and suffix | 215 | ported | `dockerfile.rs` | `custom_host_with_suffix_in_tag` | — |
| handles custom hosts with port | 236 | ported | `dockerfile.rs` | `registry_port_not_confused_with_tag` | — |
| handles custom hosts with port without tag | 257 | ported | `dockerfile.rs` | `custom_host_with_port_no_tag` | — |
| handles quay hosts with port | 278 | ported | `dockerfile.rs` | `quay_host_with_port_no_tag` | — |
| handles namespaced images | 295 | ported | `dockerfile.rs` | `extracts_namespaced_image` | — |
| handles custom hosts with namespace | 312 | ported | `dockerfile.rs` | `extracts_scoped_image` | — |
| handles abnormal spacing | 333 | ported | `dockerfile.rs` | `abnormal_spacing_after_from` | — |
| extracts multiple FROM tags | 354 | ported | `dockerfile.rs` | `only_from_instructions_extracted` | — |
| extracts tags from Dockerfile which begins with a BOM marker | 386 | ported | `dockerfile.rs` | `bom_marker_stripped` | — |
| skips scratches | 407 | ported | `dockerfile.rs` | `scratch_is_skipped` | — |
| skips named multistage FROM tags | 412 | ported | `dockerfile.rs` | `stage_reference_is_skipped` | — |
| handles COPY --from | 433 | ported | `dockerfile.rs` | `copy_from_extracts_external_image` | — |
| handles COPY --from with digest | 454 | ported | `dockerfile.rs` | `copy_from_with_digest` | — |
| handles COPY --link --from | 481 | ported | `dockerfile.rs` | `copy_link_from_extracts_image` | — |
| skips named multistage COPY --from tags | 507 | ported | `dockerfile.rs` | `copy_from_stage_name_is_skipped` | — |
| skips index reference COPY --from tags | 528 | ported | `dockerfile.rs` | `copy_from_index_is_skipped` | — |
| detects ["stage"] and ["final"] deps of docker multi-stage build. | 549 | ported | `dockerfile.rs` | `multistage_build_with_copy_from_stage` | — |
| extracts images on adjacent lines | 598 | ported | `dockerfile.rs` | `renovate_fixture_1` | — |
| extracts images from all sorts of (maybe multiline) FROM and COPY --from statements | 628 | ported | `dockerfile.rs` | `renovate_fixture_2_multiline` | — |
| handles calico/node | 733 | ported | `dockerfile.rs` | `namespaced_image_without_tag` | — |
| handles ubuntu | 750 | ported | `dockerfile.rs` | `ubuntu_with_version_tag` | — |
| handles debian with codename | 768 | ported | `dockerfile.rs` | `debian_with_codename_tag` | — |
| handles debian with regular tag | 786 | ported | `dockerfile.rs` | `debian_with_version_tag` | — |
| handles debian with prefixes | 803 | ported | `dockerfile.rs` | `debian_with_platform_prefix` | — |
| handles debian with prefixes and registries | 821 | ported | `dockerfile.rs` | `debian_with_registry_prefix` | — |
| handles prefixes | 843 | ported | `dockerfile.rs` | `ubuntu_with_platform_prefix` | — |
| handles prefixes with registries | 861 | ported | `dockerfile.rs` | `registry_with_namespace_prefix` | — |
| handles implausible line continuation | 883 | ported | `dockerfile.rs` | `implausible_continuation_does_not_affect_from` | — |
| handles multi-line FROM with space after escape character | 904 | ported | `dockerfile.rs` | `multiline_from_with_space_after_escape` | — |
| handles FROM without ARG default value | 921 | ported | `dockerfile.rs` | `from_with_arg_variable_is_skipped` | — |
| handles FROM with empty ARG default value | 939 | ported | `dockerfile.rs` | `from_with_empty_arg_defaults_extracts_literal_image` | — |
| handles FROM with version in ARG value | 960 | ported | `dockerfile.rs` | `from_with_version_in_arg_value` | — |
| handles FROM with version in ARG default value | 981 | ported | `dockerfile.rs` | `from_with_version_in_arg_default_value` | — |
| handles FROM with digest in ARG default value | 1002 | ported | `dockerfile.rs` | `from_with_digest_in_arg_value` | — |
| handles FROM with overwritten ARG value | 1026 | ported | `dockerfile.rs` | `from_with_overwritten_arg_value` | — |
| handles FROM with multiple ARG values | 1058 | ported | `dockerfile.rs` | `from_with_multiple_arg_values` | — |
| skips scratch if provided in ARG value | 1079 | ported | `dockerfile.rs` | `scratch_from_arg_value_is_skipped` | — |
| extracts images from multi-line ARG statements | 1088 | ported | `dockerfile.rs` | `extracts_images_from_multiline_arg_statements` | — |
| ignores parser directives in wrong order | 1131 | ported | `dockerfile.rs` | `parser_directives_in_wrong_order_ignored` | — |
| handles an alternative escape character | 1152 | ported | `dockerfile.rs` | `alternative_escape_character` | — |
| handles FROM with version in ARG default value and quotes | 1227 | ported | `dockerfile.rs` | `from_with_quoted_arg_default_value` | — |
| handles version in ARG and digest in FROM with CRLF linefeed | 1249 | ported | `dockerfile.rs` | `from_with_arg_tag_and_digest_with_crlf` | — |
| handles updates of multiple ARG values | 1272 | ported | `dockerfile.rs` | `from_with_multiple_arg_components` | — |
| handles same argument multiple times | 1308 | ported | `dockerfile.rs` | `same_arg_used_multiple_times` | — |
| handles empty optional parameters | 1329 | ported | `dockerfile.rs` | `handles_empty_optional_parameters` | — |
| handles registry alias | 1352 | ported | `dockerfile.rs` | `handles_registry_alias` | — |
| replaces registry alias from start only | 1380 | ported | `dockerfile.rs` | `registry_alias_matches_start_only` | — |
| handles empty registry | 1407 | ported | `dockerfile.rs` | `namespaced_image_without_registry_extracted_normally` | — |
| handles # syntax statements | 1435 | ported | `dockerfile.rs` | `syntax_directive_extracted` | — |
| ignores # syntax statements after first line | 1469 | ported | `dockerfile.rs` | `syntax_directive_after_from_ignored` | — |

### `getDep()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| rejects null | 1493 | not-applicable | — | — | TypeScript-only null guard; Rust Option handles this at type level |
| rejects empty or whitespace | 1497 | not-applicable | — | — | TypeScript-only guard; no Rust equivalent needed |
| handles default environment variable values | 1501 | ported | `dockerfile.rs` | `default_variable_value_extracted` | — |
| skips tag containing a variable | 1563 | ported | `dockerfile.rs` | `tag_with_variable_is_skipped` | — |
| skips depName containing a non default variable at start | 1574 | ported | `dockerfile.rs` | `arg_variable_is_skipped` | — |
| skips depName containing a non default variable with brackets at start | 1585 | ported | `dockerfile.rs` | `arg_braces_variable_is_skipped` | — |
| skips depName containing a non default variable | 1596 | ported | `dockerfile.rs` | `variable_in_image_path_is_skipped` | — |
| skips depName containing a non default variable with brackets | 1607 | ported | `dockerfile.rs` | `braced_variable_in_image_path_is_skipped` | — |

### `extractVariables()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles no variable | 1651 | not-applicable | — | — | TypeScript utility with no Rust equivalent; variable expansion is done inline |
| handles simple variable | 1655 | not-applicable | — | — | TypeScript utility with no Rust equivalent |
| handles escaped variable | 1661 | not-applicable | — | — | TypeScript utility with no Rust equivalent |
| handles complex variable | 1667 | not-applicable | — | — | TypeScript utility with no Rust equivalent |
| handles complex variable with static default value | 1673 | not-applicable | — | — | TypeScript utility with no Rust equivalent |
| handles complex variable with other variable as default value | 1679 | not-applicable | — | — | TypeScript utility with no Rust equivalent |
| handles multiple variables | 1685 | not-applicable | — | — | TypeScript utility with no Rust equivalent |

---

## `lib/modules/manager/fleet/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/fleet/extract.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 10 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null if empty content | 24 | ported | `fleet.rs` | `empty_content_returns_empty` | — |
| should return null if a unknown manifest is supplied | 30 | ported | `fleet.rs` | `unknown_manifest_returns_empty` | — |
| should return null if content is a malformed YAML (fleet.yaml) | 37 | ported | `fleet.rs` | `malformed_fleet_yaml_returns_empty` | — |
| should parse valid configuration (fleet.yaml) | 49 | ported | `fleet.rs` | `extracts_helm_dep_from_fleet_yaml` | — |
| should support registryAlias configuration | 88 | ported | `fleet.rs` | `supports_registry_alias_configuration` | — |
| should parse valid configuration with target customization | 132 | ported | `fleet.rs` | `extracts_target_customizations` | — |
| should parse parse invalid configurations | 208 | ported | `fleet.rs` | `missing_chart_sets_skip_reason` / `no_version_sets_skip_reason` | — |
| should return null if content is a malformed YAML (GitRepo) | 242 | ported | `fleet.rs` | `malformed_gitrepo_yaml_returns_empty` | — |
| should parse valid configuration (GitRepo) | 254 | ported | `fleet.rs` | `extracts_gitrepo_dep` | — |
| should parse invalid configuration (GitRepo) | 276 | ported | `fleet.rs` | `gitrepo_missing_revision_sets_skip_reason` / `non_gitrepo_yaml_returns_empty` | — |

---

## `lib/modules/manager/nvm/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/nvm/extract.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns a result | 5 | ported | `version_file.rs` | `nvmrc_plain_version` | — |
| supports ranges | 16 | ported | `version_file.rs` | `nvmrc_partial_version_range` | — |
| skips non ranges | 27 | ported | `version_file.rs` | `nvmrc_passes_through_latest_literal` | — |
| supports code comments | 38 | ported | `version_file.rs` | `nvmrc_skips_full_line_comments_and_inline_comment` | — |

---

## `lib/modules/manager/ruby-version/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/ruby-version/extract.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns a result | 5 | ported | `version_file.rs` | `ruby_version_file` | — |
| supports ranges | 16 | ported | `version_file.rs` | `ruby_version_partial_range` | — |
| skips non ranges | 27 | ported | `version_file.rs` | `ruby_version_passes_through_non_alias_literal` | — |

---

## `lib/modules/manager/terraform-version/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/terraform-version/extract.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns a result | 5 | ported | `version_file.rs` | `terraform_version_plain` | — |
| skips non ranges | 18 | ported | `version_file.rs` | `terraform_version_passes_through_non_alias_literal` | — |

---

## `lib/modules/manager/gitlabci-include/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gitlabci-include/extract.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 8 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 13 | ported | `gitlabci_include.rs` | `empty_returns_empty` | — |
| returns null for include block without any actual includes | 17 | ported | `gitlabci_include.rs` | `empty_include_block_returns_no_deps` | — |
| extracts single include block | 22 | ported | `gitlabci_include.rs` | `extracts_include_with_ref` | — |
| extracts multiple include blocks | 28 | ported | `gitlabci_include.rs` | `multiple_includes` | — |
| extracts multiple embedded include blocks | 34 | ported | `gitlabci_include.rs` | `extracts_multiple_embedded_include_blocks` | — |
| ignores includes without project and file keys | 51 | ported | `gitlabci_include.rs` | `ignores_includes_without_project_and_file_keys` | — |
| normalizes configured endpoints | 60 | ported | `gitlabci_include.rs` | `normalizes_configured_endpoints` | — |
| supports multi-document files | 73 | ported | `gitlabci_include.rs` | `supports_multi_document_files` | — |

---

## `lib/modules/manager/bazel/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazel/extract.spec.ts
**Total tests:** 12 | **Ported:** 12 | **Actionable:** 12 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty if fails to parse | 10 | ported | `bazel.rs` | `empty_file_returns_empty` (+ invalid_content_returns_empty, git_repository_without_url_returns_empty) | — |
| returns empty if cannot parse dependency | 15 | ported | `bazel.rs` | `invalid_content_returns_empty` | — |
| returns empty for incomplete dependency | 20 | ported | `bazel.rs` | `http_archive_with_no_url_returns_dep_with_skip_reason` | — |
| extracts multiple types of dependencies | 25 | ported | `bazel.rs` | `workspace1_multiple_dependency_types` | — |
| extracts github tags | 31 | ported | `bazel.rs` | `extracts_github_archive_dep` (+ extracts_github_release_dep, extracts_multiple_archives) | — |
| handle comments and strings | 42 | ported | `bazel.rs` | `workspace3_comments_and_strings` | — |
| extracts dependencies from *.bzl files | 47 | ported | `bazel.rs` | `extracts_dependencies_from_bzl_files` | — |
| extracts dependencies for container_pull deptype | 65 | ported | `bazel.rs` | `container_pull_extracted` | — |
| extracts dependencies for oci_pull deptype | 90 | ported | `bazel.rs` | `oci_pull_extracted` | — |
| check remote option in go_repository | 113 | ported | `bazel.rs` | `go_repository_remote_option` | — |
| sequential http_archive | 166 | ported | `bazel.rs` | `singular_url_form_extracted` | — |
| http_archive with GitLab url | 190 | ported | `bazel.rs` | `gitlab_archive_with_version_extracted` (+ gitlab_archive_with_commit_digest_extracted) | — |

---

## `lib/modules/manager/bicep/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bicep/extract.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should extract a normal resource | 5 | ported | `bicep.rs` | `extracts_resource_declaration` (+ extracts_multiple_resources, preview_version_captured) | — |
| should not extract a commented out resource | 37 | ported | `bicep.rs` | `comment_lines_skipped` (+ no_resources_returns_empty) | — |
| should extract a conditional resource | 58 | ported | `bicep.rs` | `extracts_conditional_resource` | — |
| should extract a existing resource | 90 | ported | `bicep.rs` | `extracts_existing_resource` | — |
| should extract a conditional loop resource | 117 | ported | `bicep.rs` | `extracts_conditional_loop_resource` | — |
| should extract a loop resource | 149 | ported | `bicep.rs` | `extracts_loop_resource` | — |
| should not extract a nested unversioned resource | 181 | ported | `bicep.rs` | `nested_unversioned_resource_skipped` | — |
| should not extract a nested versioned resource | 217 | ported | `bicep.rs` | `nested_versioned_resource_skipped` | — |
| should extract a sub resource | 253 | ported | `bicep.rs` | `extracts_sub_resource_with_multiple_slashes` | — |

---

## `lib/modules/manager/html/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/html/extract.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `modules/manager/html/extract`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extractPackageFile | 8 | ported | `html.rs` | `extracts_from_sample_html_fixture` | — |
| returns null | 21 | ported | `html.rs` | `nothing_html_returns_empty` | — |

---

## `lib/modules/manager/meteor/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/meteor/extract.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty if fails to parse | 8 | ported | `meteor.rs` | `no_npm_depends_returns_empty` (+ empty_returns_empty) | — |
| returns results | 13 | ported | `meteor.rs` | `extracts_deps` | — |

---

## `lib/modules/manager/jsonnet-bundler/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/jsonnet-bundler/extract.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid jsonnetfile | 24 | ported | `jsonnet_bundler.rs` | `invalid_json_returns_empty` | — |
| returns null for jsonnetfile with no dependencies | 30 | ported | `jsonnet_bundler.rs` | `empty_returns_empty` | — |
| returns null for local dependencies | 36 | ported | `jsonnet_bundler.rs` | `local_deps_returns_empty` | — |
| returns null for vendored dependencies | 42 | ported | `jsonnet_bundler.rs` | `vendored_dependencies_return_empty` | — |
| returns null for dependencies with empty Git source | 48 | ported | `jsonnet_bundler.rs` | `empty_git_source_returns_empty` | — |
| extracts dependency | 57 | ported | `jsonnet_bundler.rs` | `extracts_github_deps` (+ extracts_main_fixture_two_deps) | — |
| extracts dependency with custom name | 79 | ported | `jsonnet_bundler.rs` | `extracts_dep_with_optional_name_field_uses_path_name` | — |

---

## `lib/modules/manager/scalafmt/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/scalafmt/extract.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts version correctly | 6 | ported | `scalafmt.rs` | `extracts_version` | — |
| extracts version correctly if enclosed in quotes | 25 | ported | `scalafmt.rs` | `version_without_quotes` | — |
| ignore file if no version specified | 44 | ported | `scalafmt.rs` | `no_version_returns_none` | — |
| should return empty packagefiles is no content is provided | 52 | ported | `scalafmt.rs` | `empty_returns_none` | — |

---

## `lib/modules/manager/runtime-version/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/runtime-version/extract.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns a result - python | 5 | ported | `runtime_version.rs` | `extracts_python_version` (+ extracts_with_trailing_newline) | — |
| returns no result | 16 | ported | `runtime_version.rs` | `ignores_partial_version` (+ returns_none_for_empty) | — |

---

## `lib/modules/manager/pipenv/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pipenv/extract.spec.ts
**Total tests:** 16 | **Ported:** 16 | **Actionable:** 16 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 37 | ported | `pipfile.rs` | `empty_content_returns_no_deps` | — |
| returns null for invalid toml file | 41 | ported | `pipfile.rs` | `invalid_toml_returns_empty` | — |
| extracts dependencies | 45 | ported | `pipfile.rs` | `extracts_string_form` (+ extracts_multi_constraint, extracts_table_form, dev_packages_flagged) | — |
| marks packages with "extras" as skipReason === unspecified-version | 136 | ported | `pipfile.rs` | `packages_with_only_extras_are_skipped` | — |
| extracts multiple dependencies | 142 | ported | `pipfile.rs` | `dev_packages_flagged` (+ extracts_string_form combined coverage) | — |
| ignores git dependencies | 192 | ported | `pipfile.rs` | `git_dependency_in_mixed_list_skipped` (+ git_dep_skipped) | — |
| ignores invalid package names | 202 | ported | `pipfile.rs` | `invalid_package_name_starting_with_underscore_skipped` | — |
| ignores relative path dependencies | 213 | ported | `pipfile.rs` | `relative_path_in_mixed_list_skipped` (+ local_dep_skipped) | — |
| ignores invalid versions | 223 | ported | `pipfile.rs` | `version_with_spaces_skipped` (+ wildcard_skipped, dev_wildcard_skipped) | — |
| extracts all sources | 234 | ported | `pipfile.rs` | `extracts_all_sources` | — |
| extracts example pipfile | 247 | ported | `pipfile.rs` | `extracts_example_pipfile` | — |
| supports custom index | 313 | ported | `pipfile.rs` | `supports_custom_index` | — |
| gets python constraint from python_version | 338 | ported | `pipfile.rs` | `gets_python_constraint_from_python_version` | — |
| gets python constraint from python_full_version | 350 | ported | `pipfile.rs` | `gets_python_constraint_from_python_full_version` | — |
| gets pipenv constraint from packages | 362 | ported | `pipfile.rs` | `gets_pipenv_constraint_from_packages` | — |
| gets pipenv constraint from dev-packages | 372 | ported | `pipfile.rs` | `gets_pipenv_constraint_from_dev_packages` | — |

---

## `lib/modules/manager/pip_requirements/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pip_requirements/extract.spec.ts
**Total tests:** 22 | **Ported:** 22 | **Actionable:** 22 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 39 | ported | `pip.rs` | `invalid_line_returns_empty` | — |
| extracts dependencies | 43 | ported | `pip.rs` | `extracts_exact_pin` (+ extracts_unconstrained_package, requirements1_fixture, blank_lines_ignored) | — |
| extracts dependencies with --index-url short code | 50 | ported | `pip.rs` | `index_url_short_code_skipped_package_extracted` | — |
| extracts --requirement short code option | 68 | ported | `pip.rs` | `sub_requirement_is_skipped` | — |
| extracts --constraints short code option | 79 | ported | `pip.rs` | `constraints_file_is_skipped` | — |
| extracts multiple dependencies | 90 | ported | `pip.rs` | `handles_multiple_packages` (+ requirements2_fixture) | — |
| handles comments and commands | 96 | ported | `pip.rs` | `comment_only_lines_ignored` (+ blank_lines_ignored, index_url_directive_ignored) | — |
| handles extras and complex index url | 102 | ported | `pip.rs` | `extracts_range_constraint` (+ handles_extras_and_complex_index_url_registry) | — |
| handles extra index url | 111 | ported | `pip.rs` | `handles_extra_index_url` | — |
| handles extra index url and defaults without index to config | 123 | ported | `pip.rs` | `handles_extra_index_url_without_index_for_config_default` | — |
| handles extra index url and defaults without index to pypi | 132 | ported | `pip.rs` | `handles_extra_index_url_without_index_for_pypi_default` | — |
| handles extra spaces around pinned dependency equal signs | 141 | ported | `pip.rs` | `extra_spaces_around_equal_signs` | — |
| should not replace env vars in low trust mode | 155 | ported | `pip.rs` | `does_not_replace_env_vars_in_low_trust_mode` | — |
| should replace env vars in high trust mode | 166 | ported | `pip.rs` | `replaces_env_vars_in_high_trust_mode` | — |
| should handle hashes | 178 | ported | `pip.rs` | `hash_continuation_lines_handled` | — |
| should handle package with extras and no version specifiers | 184 | ported | `pip.rs` | `extracts_unconstrained_package` | — |
| should handle dependency and ignore env markers | 198 | ported | `pip.rs` | `extracts_range_constraint` | — |
| should handle git packages | 213 | ported | `pip.rs` | `git_source_is_skipped` | — |
| extracts a file with only --index-url flags | 258 | ported | `pip.rs` | `url_install_is_skipped` | — |
| extracts a file with only --extra-index-url flags | 266 | ported | `pip.rs` | `extra_index_url_only_file_returns_no_deps` | — |
| extracts a file with only -r flags | 276 | ported | `pip.rs` | `r_flag_only_file_has_no_actionable_deps` | — |
| extracts a file with only -c flags | 286 | ported | `pip.rs` | `c_flag_only_file_has_no_actionable_deps` | — |

---

## `lib/modules/manager/pep621/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pep621/extract.spec.ts
**Total tests:** 14 | **Ported:** 11 | **Actionable:** 14 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null for empty content | 16 | ported | `pep621.rs` | `empty_content_returns_empty` | — |
| should return null for invalid toml | 21 | ported | `pep621.rs` | `invalid_toml_returns_error` | — |
| should return dependencies for valid content | 32 | ported | `pep621.rs` | `extracts_project_dependencies` (+ extracts_optional_dependencies, extracts_dependency_groups_skips_include_tables) | — |
| should return dependencies with overwritten pypi registryUrl | 233 | ported | `pep621.rs` | `pdm_sources_apply_registry_urls_to_project_optional_and_dev_dependencies` | — |
| should return dependencies with original pypi registryUrl | 309 | ported | `pep621.rs` | `pdm_sources_apply_registry_urls_to_project_dependencies` | — |
| should skip dependencies with unsupported uv sources | 340 | ported | `pep621.rs` | `uv_sources_classify_git_path_url_and_workspace_sources` | — |
| should handle SSH git URLs correctly for GitHub sources | 412 | ported | `pep621.rs` | `uv_sources_handle_ssh_github_tag_and_rev` | — |
| should extract dependencies from hatch environments | 446 | ported | `pep621.rs` | `hatch_env_dependencies_and_extra_dependencies_are_extracted` | — |
| should extract project version | 498 | ported | `pep621.rs` | `project_version_field_is_parseable` | — |
| should extract dependencies from build-system.requires | 510 | ported | `pep621.rs` | `build_system_requires_extracted_with_project_deps` | — |
| should resolve lockedVersions from pdm.lock | 551 | ported | `pep621.rs` | `pdm_fixture` | — |
| should resolve lockedVersions from uv.lock | 595 | pending | — | — | Requires uv.lock support |
| should resolve dependencies without locked versions on invalid uv.lock | 661 | pending | — | — | Requires uv.lock support |
| should resolve dependencies with template | 694 | pending | — | — | Requires templating support in pep621 extractor |

---

## `lib/modules/manager/osgi/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/osgi/extract.spec.ts
**Total tests:** 14 | **Ported:** 14 | **Actionable:** 14 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty file | 143 | ported | `osgi.rs` | `empty_returns_empty` | — |
| returns null for invalid file | 147 | ported | `osgi.rs` | `invalid_json_returns_empty` | — |
| returns null for unsupported version of feature model definition | 151 | ported | `osgi.rs` | `unsupported_version_skipped` | — |
| returns null for an invalid version of feature model definition | 157 | ported | `osgi.rs` | `invalid_feature_version_returns_empty` | — |
| returns null for a null string passed in as a feature model definition | 163 | ported | `osgi.rs` | `null_string_returns_empty` | — |
| returns null for a valid file with no artifact definitions | 167 | ported | `osgi.rs` | `no_bundles_returns_empty` | — |
| extracts the bundles from a file with object bundles definitions | 171 | ported | `osgi.rs` | `extracts_object_bundle` | — |
| extracts the bundles from a file with string bundles defintions | 193 | ported | `osgi.rs` | `extracts_string_bundle` (+ slash_separator_normalized) | — |
| extracts the bundles from a file with comments | 215 | ported | `osgi.rs` | `json_with_comments` | — |
| extracts the artifacts from an extension section | 228 | ported | `osgi.rs` | `extracts_from_extension_section` | — |
| extracts the artifacts a file with a double slash | 241 | ported | `osgi.rs` | `double_slash_in_value_not_treated_as_comment` | — |
| extracts the artifacts from the framework artifact section | 263 | ported | `osgi.rs` | `extracts_from_framework_artifact_section` | — |
| skips depedencies with with malformed definitions | 276 | ported | `osgi.rs` | `malformed_definitions_skipped_with_valid_kept` | — |
| skips artifacts with variables in version | 297 | ported | `osgi.rs` | `variable_version_skipped` | — |

---

## `lib/modules/manager/woodpecker/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/woodpecker/extract.spec.ts
**Total tests:** 11 | **Ported:** 11 | **Actionable:** 11 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 8 | ported | `woodpecker.rs` | `empty_returns_empty` | — |
| returns null for non-object YAML | 12 | ported | `woodpecker.rs` | `non_object_yaml_returns_empty` | — |
| returns null for malformed YAML | 17 | ported | `woodpecker.rs` | `malformed_yaml_returns_empty` | — |
| extracts multiple image lines | 21 | ported | `woodpecker.rs` | `extracts_step_image` (+ extracts_service_image, multiple_steps_and_services, steps_section_extracts_image) | — |
| extracts image and replaces registry | 129 | ported | `woodpecker.rs` | `extracts_image_and_replaces_registry` | — |
| extracts image but no replacement | 159 | ported | `woodpecker.rs` | `extracts_image_without_registry_replacement` | — |
| extracts image and no double replacement | 189 | ported | `woodpecker.rs` | `extracts_image_without_double_registry_replacement` | — |
| extracts the v.1.0.x version | 220 | ported | `woodpecker.rs` | `steps_section_extracts_image` | — |
| should parse multiple sources of dependencies together | 246 | ported | `woodpecker.rs` | `clone_and_steps_both_extracted` | — |
| return dependency when an plugin-git is cloned | 286 | ported | `woodpecker.rs` | `clone_section_extracted` | — |
| return null when no dependencies are provided | 313 | ported | `woodpecker.rs` | `no_steps_or_services_returns_empty` | — |

---

## `lib/modules/manager/travis/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/travis/extract.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 8 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty if fails to parse | 13 | ported | `travis.rs` | `empty_returns_empty` (+ no_node_js_key_returns_empty, invalid_content_returns_empty) | — |
| returns results | 18 | ported | `travis.rs` | `extracts_node_js_versions` (+ lts_alias_skipped, stable_skipped) | — |
| should handle invalid YAML | 24 | ported | `travis.rs` | `invalid_yaml_no_node_js_returns_empty` | — |
| handles matrix node_js syntax with node_js string | 29 | ported | `travis.rs` | `matrix_jobs_include_node_js_string` | — |
| handles matrix node_js syntax with node_js array | 42 | ported | `travis.rs` | `matrix_jobs_node_js_inline_array` | — |
| handles matrix node_js syntax with node_js array 2 | 60 | ported | `travis.rs` | `matrix_jobs_include_node_js_multiline_list` | — |
| handles matrix node_js syntax with alias | 78 | ported | `travis.rs` | `matrix_alias_node_js_string` | — |
| handles invalid matrix node_js syntax | 91 | ported | `travis.rs` | `matrix_without_node_js_returns_empty` | — |

---

## `lib/modules/manager/typst/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/typst/extract.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty deps for empty content | 5 | ported | `typst.rs` | `empty_content_returns_empty` | — |
| returns empty deps when no imports found | 10 | ported | `typst.rs` | `no_imports_returns_empty` | — |
| extracts single import | 21 | ported | `typst.rs` | `extracts_preview_import` (+ extracts_import_with_trailing_colon_import) | — |
| extracts multiple imports | 36 | ported | `typst.rs` | `multiple_imports` | — |
| handles imports with different version formats | 67 | ported | `typst.rs` | `prerelease_version_formats_extracted` | — |
| strips JSON comments before parsing | 98 | ported | `typst.rs` | `comment_line_skipped` | — |
| handles multiple imports on same line | 125 | ported | `typst.rs` | `multiple_imports_on_same_line` | — |
| ignores invalid import formats | 147 | ported | `typst.rs` | `ignores_invalid_import_formats` | — |
| adds skipReason for non-preview namespaces | 167 | ported | `typst.rs` | `local_namespace_skipped` (+ unknown_namespace_skipped, non_preview_namespaces_get_skip_reasons) | — |

---

## `lib/modules/manager/terragrunt/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/terragrunt/extract.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 6 | ported | `terragrunt.rs` | `no_terraform_block_returns_empty` | — |
| extracts terragrunt sources using tfr protocol | 10 | ported | `terragrunt.rs` | `extracts_tfr_protocol_sources` | — |
| extracts terragrunt sources | 51 | ported | `terragrunt.rs` | `extracts_github_ref_source` (+ extracts_git_prefix_github, multiple_terraform_blocks, local_path_skipped) | — |
| extracts terragrunt sources with depth specified after the branch | 269 | ported | `terragrunt.rs` | `extracts_sources_with_depth_after_ref` | — |
| extracts terragrunt sources with depth specified before the branch | 487 | ported | `terragrunt.rs` | `extracts_sources_with_depth_before_ref` | — |
| returns null if only local terragrunt deps | 698 | ported | `terragrunt.rs` | `local_only_deps_returns_empty` | — |
| returns empty deps if only local terragrunt includes | 707 | ported | `terragrunt.rs` | `include_block_only_returns_empty` | — |

---

## `lib/modules/manager/tflint-plugin/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/tflint-plugin/extract.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 22 | ported | `tflint_plugin.rs` | `no_plugins_returns_empty` | — |
| returns null when there are no version | 28 | ported | `tflint_plugin.rs` | `missing_version_sets_skip_reason` (+ plugin_without_source_or_version_gets_missing_source) | — |
| extracts plugins | 38 | ported | `tflint_plugin.rs` | `extracts_github_plugin` (+ extracts_multiple_plugins, extracts_plugins_with_org_paths) | — |
| extracts from full configuration | 71 | ported | `tflint_plugin.rs` | `extracts_plugin_from_full_config` | — |
| extracts no source | 112 | ported | `tflint_plugin.rs` | `plugins_without_source_get_missing_source_skip` | — |
| extracts nothing if not from github | 138 | ported | `tflint_plugin.rs` | `non_github_source_skipped` (+ gitlab_source_gets_unsupported_datasource) | — |

---

## `lib/modules/manager/kotlin-script/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/kotlin-script/extract.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts dependencies in a generic case | 12 | ported | `kotlin_script.rs` | `extracts_generic_case_fixture_three_deps` (+ extracts_single_dep, extracts_multiple_deps) | — |
| detects custom repository definitions | 43 | ported | `kotlin_script.rs` | `extracts_custom_repositories` | — |
| no dependencies | 71 | ported | `kotlin_script.rs` | `no_annotations_returns_empty` | — |
| skips dependencies with missing parts | 81 | ported | `kotlin_script.rs` | `skips_missing_parts` | — |

---

## `lib/modules/manager/maven-wrapper/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/maven-wrapper/extract.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts version for property file with distribution type "bin" in distributionUrl | 14 | ported | `maven_wrapper.rs` | `extracts_wrapper_and_maven_properties` | — |
| extracts version for property file with only a wrapper url | 37 | ported | `maven_wrapper.rs` | `extracts_only_wrapper_url` | — |
| extracts version for property file with only a wrapper version | 51 | ported | `maven_wrapper.rs` | `extracts_only_wrapper_version_key` | — |
| extracts wrapper information from wrapperUrl in precedence to wrapperVersion | 64 | ported | `maven_wrapper.rs` | `wrapper_url_takes_precedence_over_wrapper_version` | — |
| extracts maven warapper version from mvnw file | 80 | ported | `maven_wrapper.rs` | `extracts_version_from_mvnw_unix` | — |
| extracts maven warapper version from mvnw file - Windows | 93 | ported | `maven_wrapper.rs` | `extracts_version_from_mvnw_windows` | — |
| returns null for invalid wrapper version string in from mvnw file | 106 | ported | `maven_wrapper.rs` | `invalid_mvnw_prefix_returns_empty` | — |
| extracts version for property file with only a maven url | 111 | ported | `maven_wrapper.rs` | `extracts_maven_version` | — |
| should return null when there is no string matching the maven properties regex | 125 | ported | `maven_wrapper.rs` | `no_matching_key_returns_empty` | — |

---

## `lib/modules/manager/pre-commit/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pre-commit/extract.spec.ts
**Total tests:** 12 | **Ported:** 10 | **Actionable:** 12 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid yaml file content | 52 | ported | `pre_commit.rs` | `invalid_yaml_returns_empty` | — |
| returns null for empty yaml file content | 57 | ported | `pre_commit.rs` | `empty_content_returns_no_deps` | — |
| returns null for no file content | 62 | ported | `pre_commit.rs` | `null_content_returns_empty` | — |
| returns null for no repos | 68 | ported | `pre_commit.rs` | `no_repos_section_returns_no_deps` | — |
| returns null for empty repos | 73 | ported | `pre_commit.rs` | `empty_repos_list_returns_empty` | — |
| returns null for invalid repo | 78 | ported | `pre_commit.rs` | `repo_entry_without_repo_key_returns_empty` | — |
| extracts from values.yaml correctly with same structure as "pre-commit sample-config" | 83 | ported | `pre_commit.rs` | `git_suffix_stripped` | — |
| extracts from complex config file correctly | 105 | ported | `pre_commit.rs` | `extracts_github_hooks` (+ extracts_gitlab_hooks, skips_local_hooks, skips_meta_hooks, total_dep_count) | — |
| can handle private git repos | 161 | pending | — | — | Requires hostRules / token-based platform detection |
| can handle invalid private git repos | 183 | ported | `pre_commit.rs` | `unknown_registry_gets_skip_reason` | — |
| can handle unknown private git repos | 200 | pending | — | — | Requires hostRules / token-based platform detection |
| can handle pinned repo versions | 220 | ported | `pre_commit.rs` | `frozen_digest_rev_extracts_version_and_digest` | — |

---

## `lib/modules/manager/helmfile/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/helmfile/extract.spec.ts
**Total tests:** 19 | **Ported:** 17 | **Actionable:** 19 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skip null YAML document | 18 | ported | `helmfile.rs` | `null_yaml_document_returns_empty` | — |
| returns null if no releases | 31 | ported | `helmfile.rs` | `no_releases_section_returns_empty` | — |
| do not crash on invalid helmfile.yaml | 46 | ported | `helmfile.rs` | `invalid_yaml_does_not_crash` | — |
| skip if repository details are not specified | 63 | ported | `helmfile.rs` | `unknown_repo_alias_has_skip_reason` | — |
| skip templetized release with invalid characters | 84 | ported | `helmfile.rs` | `invalid_chart_name_chars_skipped` | — |
| skip local charts | 118 | ported | `helmfile.rs` | `local_path_chart_gets_skip_reason` | — |
| skip chart with unknown repository | 139 | ported | `helmfile.rs` | `chart_with_no_matching_repo_skipped` | — |
| skip chart with special character in the name | 160 | ported | `helmfile.rs` | `chart_with_special_chars_skipped` | — |
| skip chart that does not have specified version | 184 | ported | `helmfile.rs` | `release_without_version_has_invalid_version_skip` | — |
| parses multidoc yaml | 204 | ported | `helmfile.rs` | `parses_multidoc_yaml` | — |
| parses a chart with a go templating | 242 | ported | `helmfile.rs` | `go_template_chart_skipped_real_chart_kept` | — |
| parses a chart with empty strings for template values | 280 | ported | `helmfile.rs` | `template_version_gets_invalid_version_skip` | — |
| parses a chart with an oci repository and non-oci one | 316 | ported | `helmfile.rs` | `oci_backed_repo_uses_docker_datasource` | — |
| allows OCI chart names containing forward slashes | 366 | ported | `helmfile.rs` | `oci_nested_path_chart_uses_docker_datasource` | — |
| parses a chart with an oci repository with --- | 392 | ported | `helmfile.rs` | `oci_repo_with_yaml_document_separator` | — |
| parses and replaces templating strings | 423 | pending | — | — | Requires helmfile go-template substitution |
| detects kustomize and respects relative paths | 477 | pending | — | — | Requires kustomize detection inside helmfile releases |
| makes sure url joiner works correctly | 513 | ported | `helmfile.rs` | `oci_url_with_port_in_chart_ref` | — |
| skips helm-git repos | 539 | ported | `helmfile.rs` | `helm_git_repo_releases_get_unknown_registry` | — |

---

## `lib/modules/manager/helm-requirements/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/helm-requirements/extract.spec.ts
**Total tests:** 11 | **Ported:** 11 | **Actionable:** 11 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ensure that currentValue is string | 8 | ported | `helm.rs` | `at_alias_skipped` | — |
| skips invalid registry urls | 34 | ported | `helm.rs` | `oci_registry_skipped` | — |
| parses simple requirements.yaml correctly | 64 | ported | `helm.rs` | `simple_chart_yaml` (+ requirements_yaml_format) | — |
| parses simple requirements.yaml but skips if necessary fields missing | 96 | ported | `helm.rs` | `no_dependencies_returns_empty` | — |
| resolves aliased registry urls | 112 | ported | `helm.rs` | `stable_alias_resolved` | — |
| skips local dependencies | 141 | ported | `helm.rs` | `local_file_dependency_skipped` | — |
| returns null if no dependencies | 172 | ported | `helm.rs` | `no_dependencies_returns_empty` | — |
| returns null if requirements.yaml is invalid | 192 | ported | `helm.rs` | `invalid_yaml_returns_empty` | — |
| returns null if Chart.yaml is empty | 214 | ported | `helm.rs` | `empty_content_returns_empty` | — |
| validates ${fieldName} is required | 279 | ported | `helm.rs` | `no_repository_skipped` (+ missing_version_dep_skipped, dep_without_name_is_silently_skipped) | — |
| skips only invalid dependences | 293 | ported | `helm.rs` | `skips_only_invalid_deps_keeps_valid_ones` | — |

---

## `lib/modules/manager/homebrew/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/homebrew/extract.spec.ts
**Total tests:** 17 | **Ported:** 17 | **Actionable:** 17 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips sourceforge dependency 1 | 10 | ported | `homebrew.rs` | `unsupported_url_skipped` | — |
| skips sourceforge dependency 2 | 32 | ported | `homebrew.rs` | `skips_sourceforge_dependency_2` | — |
| skips github dependency with wrong format | 54 | ported | `homebrew.rs` | `skips_github_dependency_wrong_format` | — |
| extracts "releases" github dependency | 77 | ported | `homebrew.rs` | `extracts_github_release` | — |
| extracts "archive" github dependency | 99 | ported | `homebrew.rs` | `extracts_github_archive_refs_tags` | — |
| handles old "archive" github url format | 121 | ported | `homebrew.rs` | `extracts_github_archive_old_form` | — |
| handles no space before class header | 152 | ported | `homebrew.rs` | `handles_no_space_before_class_header` | — |
| returns null for invalid class header 1 | 183 | ported | `homebrew.rs` | `no_class_header_returns_none` | — |
| returns null for invalid class header 2 | 198 | ported | `homebrew.rs` | `invalid_class_header_not_formula_returns_none` | — |
| skips if there is no url field | 213 | ported | `homebrew.rs` | `missing_url_skipped` | — |
| skips if invalid url protocol | 235 | ported | `homebrew.rs` | `skips_invalid_url_protocol` | — |
| skips if invalid url | 257 | ported | `homebrew.rs` | `skips_invalid_url` | — |
| skips if there is no sha256 field | 279 | ported | `homebrew.rs` | `skips_no_sha256_field` | — |
| skips if sha256 field is invalid | 301 | ported | `homebrew.rs` | `invalid_sha256_skipped` | — |
| extracts npm scoped package dependency | 323 | ported | `homebrew.rs` | `extracts_npm_scoped_package` | — |
| extracts npm unscoped package dependency | 354 | ported | `homebrew.rs` | `extracts_npm_unscoped_package` | — |
| skips npm package from custom registry | 385 | ported | `homebrew.rs` | `skips_npm_custom_registry` | — |

---

## `lib/modules/manager/xcodegen/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/xcodegen/extract.spec.ts
**Total tests:** 24 | **Ported:** 24 | **Actionable:** 24 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty content | 7 | ported | `xcodegen.rs` | `empty_content_returns_empty` | — |
| returns null for invalid YAML | 11 | ported | `xcodegen.rs` | `invalid_yaml_returns_empty` | — |
| returns null for YAML without packages | 22 | ported | `xcodegen.rs` | `no_packages_returns_empty` | — |
| returns null for empty packages | 36 | ported | `xcodegen.rs` | `empty_packages_section_returns_empty` | — |
| extracts packages from a realistic project.yml | 44 | ported | `xcodegen.rs` | `multiple_packages` | — |
| extracts remote package with url and from | 71 | ported | `xcodegen.rs` | `extracts_github_url_with_from` (+ gitlab_url_detected) | — |
| extracts remote package with github shorthand | 92 | ported | `xcodegen.rs` | `extracts_github_shorthand` | — |
| extracts remote package with majorVersion | 113 | ported | `xcodegen.rs` | `extracts_major_version` | — |
| extracts remote package with minorVersion | 134 | ported | `xcodegen.rs` | `extracts_minor_version` | — |
| extracts remote package with exactVersion | 155 | ported | `xcodegen.rs` | `extracts_exact_version` | — |
| extracts remote package with version | 176 | ported | `xcodegen.rs` | `extracts_version_field` | — |
| skips local packages with path | 197 | ported | `xcodegen.rs` | `local_path_skipped` | — |
| skips packages with branch reference | 214 | ported | `xcodegen.rs` | `branch_only_skipped` | — |
| skips packages with revision reference | 233 | ported | `xcodegen.rs` | `revision_reference_skipped` | — |
| skips packages with minVersion/maxVersion range | 252 | ported | `xcodegen.rs` | `min_max_version_range_skipped` | — |
| uses gitlab-tags datasource for GitLab URLs | 272 | ported | `xcodegen.rs` | `gitlab_url_produces_gitlab_source` | — |
| uses github-tags datasource with registryUrls for self-hosted GHES | 293 | ported | `xcodegen.rs` | `self_hosted_ghes_registry_url` | — |
| uses gitlab-tags datasource with registryUrls for self-hosted GitLab | 314 | ported | `xcodegen.rs` | `self_hosted_gitlab_registry_url` | — |
| uses git-tags datasource for non-GitHub/GitLab URLs | 335 | ported | `xcodegen.rs` | `generic_url_produces_git_source` | — |
| skips packages without url or github | 356 | ported | `xcodegen.rs` | `package_without_url_or_github_skipped` | — |
| skips packages without version specifier | 373 | ported | `xcodegen.rs` | `no_version_specifier_skipped` | — |
| extracts multiple packages correctly | 390 | ported | `xcodegen.rs` | `extracts_multiple_packages_correctly` | — |
| handles github URL with .git suffix | 427 | ported | `xcodegen.rs` | `github_url_with_git_suffix` | — |
| handles numeric version values from YAML parsing | 448 | ported | `xcodegen.rs` | `numeric_version_from_yaml` | — |

---

## `lib/modules/manager/puppet/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/puppet/extract.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty Puppetfile | 10 | ported | `puppet.rs` | `empty_returns_empty` | — |
| extracts multiple modules from Puppetfile without a forge | 14 | ported | `puppet.rs` | `extracts_forge_module_with_version` (+ multiple_modules) | — |
| extracts multiple modules from Puppetfile with multiple forges/registries | 47 | ported | `puppet.rs` | `extracts_custom_forge` | — |
| extracts multiple git tag modules from Puppetfile | 100 | ported | `puppet.rs` | `extracts_github_git_module` | — |
| Use GithubTagsDatasource only if host is exactly github.com | 125 | ported | `puppet.rs` | `non_github_host_uses_git_tags_datasource` | — |
| Github url without https is skipped | 146 | ported | `puppet.rs` | `http_github_url_marked_invalid_url` | — |
| Git module without a tag should result in a skip reason | 162 | ported | `puppet.rs` | `git_no_tag_skipped` | — |
| Skip reason should be overwritten by parser | 181 | ported | `puppet.rs` | `malformed_mod_with_three_positional_args_is_invalid_config` | — |
| GitTagsDatasource | 200 | ported | `puppet.rs` | `git_tags_fixture_extracts_four_valid_and_one_invalid` | — |

---

## `lib/modules/manager/tekton/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/tekton/extract.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts deps from a file | 6 | ported | `tekton.rs` | `extracts_step_images` (+ extracts_gcr_images_without_skip) | — |
| extracts deps from a file in annotations | 15 | ported | `tekton.rs` | `extracts_annotation_task_and_pipeline_refs` | — |
| ignores file without any deps | 96 | ported | `tekton.rs` | `ignores_file_without_deps` | — |
| ignores invalid YAML | 100 | ported | `tekton.rs` | `ignores_invalid_yaml_with_stray_bundle_key` | — |
| ignores empty file | 112 | ported | `tekton.rs` | `ignores_empty_file` | — |

---

## `lib/modules/manager/vendir/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/vendir/extract.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid yaml file content | 10 | ported | `vendir.rs` | `invalid_yaml_returns_empty` | — |
| returns null for empty yaml file content | 15 | ported | `vendir.rs` | `empty_returns_empty` | — |
| returns null for empty directories key | 20 | ported | `vendir.rs` | `no_helm_charts_returns_empty` | — |
| returns null for nonHelmChart key | 30 | ported | `vendir.rs` | `non_helm_chart_contents_key_returns_empty` | — |
| multiple charts - extracts helm-chart from vendir.yml correctly | 35 | ported | `vendir.rs` | `extracts_helm_charts` (+ extracts_second_chart) | — |

---

## `lib/modules/manager/velaci/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/velaci/extract.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should handle invalid YAML | 6 | ported | `velaci.rs` | `invalid_yaml_returns_empty` (+ empty_returns_empty) | — |
| should handle YAML without pipeline/images | 11 | ported | `velaci.rs` | `yaml_without_pipeline_returns_empty` | — |
| extracts multiple step pipeline image lines | 16 | ported | `velaci.rs` | `extracts_step_image` | — |
| extracts multiple services pipeline image lines | 30 | ported | `velaci.rs` | `extracts_service_image` | — |
| extracts multiple stages pipeline image lines | 48 | ported | `velaci.rs` | `extracts_stages_pipeline_images` | — |
| extracts multiple secrets pipeline image lines | 62 | ported | `velaci.rs` | `extracts_secrets_pipeline_images` | — |

---

## `lib/modules/manager/sveltos/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/sveltos/extract.spec.ts
**Total tests:** 13 | **Ported:** 12 | **Actionable:** 12 | **Status:** ported

### `extractDefinition()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns an empty array when parsing fails | 234 | not-applicable | — | — | Tests TypeScript-internal `extractDefinition` helper directly; no Rust equivalent (Rust extracts via the public `extract()` only) |
| returns null if extractDefinition returns an empty array | 240 | ported | `sveltos.rs` | `clusterprofile_with_no_helm_charts_returns_empty` | — |

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 254 | ported | `sveltos.rs` | `empty_content_returns_empty` | — |
| returns null for invalid | 258 | ported | `sveltos.rs` | `malformed_profiles_all_empty_charts_returns_empty` | — |
| return null for Kubernetes manifest | 264 | ported | `sveltos.rs` | `skips_non_sveltos_files` | — |
| return null if deps array would be empty | 269 | ported | `sveltos.rs` | `malformed_no_charts_returns_empty` | — |
| return null if YAML is invalid | 274 | ported | `sveltos.rs` | `invalid_yaml_with_no_valid_helm_charts_returns_empty` | — |
| return result for double quoted projectsveltos.io apiVersion reference | 288 | ported | `sveltos.rs` | `double_quoted_api_version_extracted` | — |
| return result for single quoted projectsveltos.io apiVersion reference | 320 | ported | `sveltos.rs` | `single_quoted_api_version_extracted` | — |
| supports profiles | 352 | ported | `sveltos.rs` | `profile_kind_extracted` | — |
| supports clusterprofiles | 400 | ported | `sveltos.rs` | `extracts_helm_chart` (+ extracts_multiple_charts) | — |
| considers registryAliases | 451 | ported | `sveltos.rs` | `considers_registry_aliases_for_oci_charts` | — |
| supports eventtriggers | 474 | ported | `sveltos.rs` | `eventtrigger_kind_extracted` | — |

---

## `lib/modules/manager/kubernetes/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/kubernetes/extract.spec.ts
**Total tests:** 15 | **Ported:** 15 | **Actionable:** 15 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 14 | ported | `kubernetes.rs` | `returns_empty_for_empty_input` (+ returns_empty_for_non_k8s) | — |
| does not return unknown kind | 18 | ported | `kubernetes.rs` | `configmap_with_no_images_returns_empty` | — |
| extracts multiple Kubernetes configurations | 23 | ported | `kubernetes.rs` | `extracts_docker_hub_images` (+ extracts_non_docker_hub_registries) | — |
| extracts image line in a YAML array | 71 | ported | `kubernetes.rs` | `extracts_docker_hub_images` | — |
| extracts image tag when it contains underscores | 98 | ported | `kubernetes.rs` | `extracts_image_with_underscore_in_tag` | — |
| ignores non-Kubernetes YAML files | 121 | ported | `kubernetes.rs` | `ignores_non_kubernetes_yaml` | — |
| handles invalid YAML files | 125 | ported | `kubernetes.rs` | `handles_invalid_yaml_with_no_images` | — |
| extracts images and replaces registries | 133 | ported | `kubernetes.rs` | `extracts_images_and_replaces_registries` | — |
| extracts images but does no replacement | 155 | ported | `kubernetes.rs` | `extracts_images_without_registry_replacement` | — |
| extracts images and does no double replacements | 177 | ported | `kubernetes.rs` | `extracts_images_without_double_registry_replacement` | — |
| extracts from complex templates | 200 | ported | `kubernetes.rs` | `extracts_from_complex_templates` | — |
| extracts image volumes from $kind | 223 | ported | `kubernetes.rs` | `extracts_image_volumes_from_workload_kinds` | — |
| extracts image volumes from Pod and CronJob | 265 | ported | `kubernetes.rs` | `extracts_image_volumes_from_pod_and_cronjob` | — |
| does not extract image volumes for unsupported kind | 326 | ported | `kubernetes.rs` | `does_not_extract_image_volumes_for_unsupported_kind` | — |
| skips malformed volume entries and extracts valid ones | 349 | ported | `kubernetes.rs` | `skips_malformed_image_volume_entries_and_extracts_valid_ones` | — |

---

## `lib/modules/manager/azure-pipelines/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/azure-pipelines/extract.spec.ts
**Total tests:** 29 | **Ported:** 28 | **Actionable:** 28 | **Status:** ported

### `extractRepository / extractContainer / extractAzurePipelinesTaskDependency` helpers

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should parse a valid azure-pipelines file | 25 | not-applicable | — | — | Tests TS-internal YAML parser helper directly; Rust extractor goes through full pipeline only |
| return null on an invalid file | 30 | ported | `azure_pipelines.rs` | `invalid_yaml_returns_empty` | — |
| should extract repository information | 36 | ported | `azure_pipelines.rs` | `extracts_github_repository_information` | — |
| should return null when repository type is not github | 52 | ported | `azure_pipelines.rs` | `non_github_repository_type_returns_none` | — |
| should return null when reference is not defined specified | 65 | ported | `azure_pipelines.rs` | `repository_without_ref_returns_none` | — |
| should return null when reference is invalid tag format | 77 | ported | `azure_pipelines.rs` | `repository_with_invalid_ref_returns_none` | — |
| should extract Azure repository information if project in name | 90 | ported | `azure_pipelines.rs` | `extracts_azure_repository_when_project_in_name` | — |
| should extract Azure repository information if project is not in name but is in the config repository | 111 | ported | `azure_pipelines.rs` | `extracts_azure_repository_project_from_current_repository` | — |
| should return null if repository type is git and project not in name nor in config repository name | 132 | ported | `azure_pipelines.rs` | `azure_repository_without_project_returns_none` | — |
| should return null if repository type is git and currentRepository is undefined | 150 | ported | `azure_pipelines.rs` | `azure_repository_without_current_repository_returns_none` | — |
| should return null for git repo type if platform not Azure | 168 | ported | `azure_pipelines.rs` | `git_repository_non_azure_platform_returns_none` | — |
| should extract container information | 187 | ported | `azure_pipelines.rs` | `extracts_container_image` (+ extracts_multiple_containers) | — |
| should extract azure-pipelines task information | 201 | ported | `azure_pipelines.rs` | `extracts_tasks` (+ tasks_in_nested_jobs_stages) | — |
| should return null for invalid task format | 209 | ported | `azure_pipelines.rs` | `task_without_at_ignored` | — |

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid azure pipelines files | 215 | ported | `azure_pipelines.rs` | `invalid_yaml_returns_empty` | — |
| extracts dependencies | 221 | ported | `azure_pipelines.rs` | `extracts_container_image` (+ extracts_tasks, extracts_multiple_containers) | — |
| should return null when there is no dependency found | 245 | ported | `azure_pipelines.rs` | `no_tasks_or_containers_returns_empty` (+ empty_returns_empty, non_containers_resources_not_extracted) | — |
| should extract deployment jobs runonce | 253 | ported | `azure_pipelines.rs` | `extracts_task_from_deployment_job_runonce` | — |
| should extract deployment jobs on failure | 277 | ported | `azure_pipelines.rs` | `extracts_task_from_deployment_job_on_failure` | — |
| should extract deployment jobs on success | 302 | ported | `azure_pipelines.rs` | `extracts_task_from_deployment_job_on_success` | — |
| should extract deployment jobs postroute | 327 | ported | `azure_pipelines.rs` | `extracts_task_from_deployment_postroute` | — |
| should extract deployment jobs predeploy | 351 | ported | `azure_pipelines.rs` | `extracts_task_from_deployment_predeploy` | — |
| should extract deployment jobs route | 375 | ported | `azure_pipelines.rs` | `extracts_task_from_deployment_route_traffic` | — |
| should extract deployment jobs rolling | 399 | ported | `azure_pipelines.rs` | `extracts_task_from_deployment_rolling` | — |
| should extract deployment jobs canary | 423 | ported | `azure_pipelines.rs` | `extracts_task_from_deployment_canary` | — |
| should extract stages | 447 | ported | `azure_pipelines.rs` | `extracts_task_from_nested_stages` | — |
| should extract jobs | 470 | ported | `azure_pipelines.rs` | `extracts_task_from_nested_jobs` | — |
| should extract steps | 491 | ported | `azure_pipelines.rs` | `extracts_task_from_top_level_steps` | — |
| should return null when task alias used | 510 | ported | `azure_pipelines.rs` | `task_alias_bash_not_extracted` | — |

---

## `lib/modules/manager/pixi/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pixi/extract.spec.ts
**Total tests:** 16 | **Ported:** 13 | **Actionable:** 16 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty pyproject.toml | 145 | ported | `pixi.rs` | `empty_pyproject_returns_empty` | — |
| returns null for empty pixi.toml | 151 | ported | `pixi.rs` | `empty_file_returns_empty` | — |
| returns null for parsed file without pixi section | 155 | ported | `pixi.rs` | `file_without_pixi_section_returns_empty` | — |
| returns parse pixi.toml | 161 | ported | `pixi.rs` | `extracts_pypi_deps` (+ extracts_conda_deps_as_actionable) | — |
| returns parse pixi section from pyproject.toml | 297 | ported | `pixi.rs` | `extract_from_pyproject_tool_pixi` | — |
| returns package of pyproject.toml tool.pixi section | 316 | ported | `pixi.rs` | `extract_tool_pixi_section_without_lockfile` | — |
| returns parse pixi.toml with features | 335 | ported | `pixi.rs` | `extracts_feature_pypi_deps` | — |
| returns parse non-known config file as pyproject.toml | 481 | ported | `pixi.rs` | `non_known_file_with_project_section` | — |
| returns parse non-known config file as pixi.toml | 509 | ported | `pixi.rs` | `non_known_file_with_tool_pixi_section` | — |
| extract feature with channels | 538 | ported | `pixi.rs` | `extract_feature_with_url_channel` | — |
| skip package without channels | 571 | ported | `pixi.rs` | `skip_package_without_channels` | — |
| extract package from with workspace | 601 | ported | `pixi.rs` | `extract_from_workspace_section` | — |
| extract package with channel priority | 630 | pending | — | — | Requires channel-priority resolution semantics |
| returns null for non-known config file | 681 | ported | `pixi.rs` | `non_toml_content_returns_empty` | — |
| set registryStrategy='merge' for channel-priority='disabled' | 685 | pending | — | — | Requires registryStrategy plumbing |
| use default registryStrategy for channel-priority='strict' | 706 | pending | — | — | Requires registryStrategy plumbing |

---

## `lib/modules/manager/mise/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/mise/extract.spec.ts
**Total tests:** 30 | **Ported:** 12 | **Actionable:** 30 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 13 | ported | `mise.rs` | `empty_returns_empty` | — |
| returns null for invalid TOML | 17 | ported | `mise.rs` | `invalid_toml_returns_empty` | — |
| returns null for empty tools section | 21 | ported | `mise.rs` | `empty_tools_section_returns_empty` | — |
| extracts tools - mise core plugins | 28 | ported | `mise.rs` | `extracts_node_version` (+ extracts_erlang_core_plugin, extracts_multiple_tools) | — |
| extracts tools - mise registry tools | 51 | pending | — | — | Requires mise registry resolution data file |
| extracts tools - asdf plugins | 393 | ported | `mise.rs` | `asdf_tools_fall_through_to_asdf_table` | — |
| extracts tools with multiple versions | 409 | ported | `mise.rs` | `unknown_tool_skipped` | — |
| extracts tools with plugin options | 432 | ported | `mise.rs` | `tool_with_version_object` | — |
| extracts tools in the default registry with backends | 448 | pending | — | — | Requires backend prefix syntax (e.g. `aqua:`, `cargo:`) parsing |
| extracts aqua backend tool | 487 | pending | — | — | Requires aqua backend support |
| extracts cargo backend tools | 514 | pending | — | — | Requires cargo backend support |
| extracts dotnet backend tool | 553 | pending | — | — | Requires dotnet backend support |
| extracts gem backend tool | 571 | pending | — | — | Requires gem backend support |
| extracts go backend tool | 589 | pending | — | — | Requires go backend support |
| extracts npm backend tool | 607 | pending | — | — | Requires npm backend support |
| extracts pipx backend tools | 625 | pending | — | — | Requires pipx backend support |
| extracts spm backend tools | 657 | pending | — | — | Requires spm backend support |
| extracts ubi backend tools | 682 | pending | — | — | Requires ubi backend support |
| extracts github backend tools | 740 | pending | — | — | Requires github backend support |
| provides skipReason for lines with unsupported tooling | 781 | ported | `mise.rs` | `unknown_tool_skipped` | — |
| provides skipReason for missing version - empty string | 802 | ported | `mise.rs` | `empty_version_string_skipped` | — |
| provides skipReason for missing version - missing version in object | 818 | ported | `mise.rs` | `object_without_version_skipped` | — |
| provides skipReason for missing version - empty array | 834 | ported | `mise.rs` | `empty_array_version_skipped` | — |
| complete mise.toml example | 855 | pending | — | — | Requires fixture round-trip with full mise.toml |
| complete example with skip | 878 | pending | — | — | Requires fixture round-trip with skip-reason mix |
| core java plugin function | 911 | ported | `mise.rs` | `java_core_plugin_jdk` | — |
| resolves tools from the mise registry data file via aqua backend | 1086 | pending | — | — | Requires mise registry data file resolver |
| resolves tools from the mise registry data file via cargo backend | 1104 | pending | — | — | Requires mise registry data file resolver |
| resolves tools from the mise registry data file via github backend | 1122 | pending | — | — | Requires mise registry data file resolver |
| resolves a tool from the mise registry, prioritising the github backend over others | 1140 | pending | — | — | Requires mise registry data file resolver |

---

## `lib/modules/manager/nuget/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/nuget/extract.spec.ts
**Total tests:** 35 | **Ported:** 25 | **Actionable:** 35 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid csproj | 28 | ported | `nuget.rs` | `invalid_xml_returns_error_or_empty` | — |
| returns null if not xml | 43 | ported | `nuget.rs` | `non_xml_content_returns_empty_or_error` | — |
| extracts package version dependency | 61 | ported | `nuget.rs` | `package_version_dependency_extracted` | — |
| extracts package file version | 70 | ported | `nuget.rs` | `package_file_version_and_lock_file_extracted` | — |
| does not fail on package file without version | 79 | ported | `nuget.rs` | `no_version_skipped` | — |
| extracts all dependencies | 86 | ported | `nuget.rs` | `simple_package_reference` (+ update_attribute_extracted, version_override_attribute_wins, version_child_element, exact_nuget_range_normalized, minimum_only_range_normalized) | — |
| extracts msbuild sdk from the Sdk attr of Project element | 94 | ported | `nuget.rs` | `msbuild_sdk_from_project_attr` | — |
| does not extract msbuild sdk from the Sdk attr of Project element if version is missing | 117 | ported | `nuget.rs` | `msbuild_sdk_missing_version_from_project_attr` | — |
| extracts msbuild sdk from the Sdk element | 132 | ported | `nuget.rs` | `msbuild_sdk_from_sdk_element` | — |
| does not extract msbuild sdk from the Sdk element if version is missing | 156 | ported | `nuget.rs` | `msbuild_sdk_element_without_version_is_skipped` | — |
| extracts msbuild sdk from the Import element | 172 | ported | `nuget.rs` | `msbuild_sdk_from_import_element` | — |
| does not extract msbuild sdk from the Import element if version is missing | 196 | ported | `nuget.rs` | `msbuild_import_element_without_version_is_skipped` | — |
| extracts dependency with lower-case Version attribute | 212 | ported | `nuget.rs` | `lowercase_version_attribute_extracted` | — |
| extracts all dependencies from global packages file | 226 | ported | `nuget.rs` | `global_and_cli_tool_references` | — |
| extracts ContainerBaseImage | 234 | ported | `nuget.rs` | `extracts_container_base_image` | — |
| extracts ContainerBaseImage with pinned digest | 260 | ported | `nuget.rs` | `extracts_container_base_image_with_digest` | — |
| considers NuGet.config | 289 | pending | — | — | Requires NuGet.config sibling-file resolution |
| considers lower-case nuget.config | 309 | pending | — | — | Requires NuGet.config sibling-file resolution |
| considers pascal-case NuGet.Config | 330 | pending | — | — | Requires NuGet.config sibling-file resolution |
| handles malformed NuGet.config | 351 | pending | — | — | Requires NuGet.config tolerant parsing |
| handles NuGet.config without package sources | 368 | pending | — | — | Requires NuGet.config tolerant parsing |
| handles NuGet.config with whitespaces in package source keys | 385 | pending | — | — | Requires NuGet.config key normalization |
| ignores local feed in NuGet.config | 404 | pending | — | — | Requires NuGet.config local-feed filtering |
| extracts registry URLs independently | 422 | pending | — | — | Requires registryUrls plumbing |
| extracts msbuild-sdks from global.json | 461 | ported | `nuget.rs` | `global_json_extracts_dotnet_sdk_and_msbuild_sdks` | — |
| extracts dotnet-sdk from global.json | 483 | ported | `nuget.rs` | `global_json_extracts_dotnet_sdk_only` | — |
| handles malformed global.json | 501 | ported | `nuget.rs` | `global_json_malformed_returns_none` | — |
| handles not-a-nuget global.json | 509 | ported | `nuget.rs` | `global_json_without_nuget_content_returns_none` | — |

### `extractPackageFile() › .config/dotnet-tools.json`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 521 | ported | `nuget.rs` | `dotnet_tools_manifest_extracts_tools` | — |
| with-config | 537 | pending | — | — | Requires .config/dotnet-tools.json + NuGet.config |
| wrong version | 561 | ported | `nuget.rs` | `dotnet_tools_manifest_wrong_version_returns_empty` | — |
| returns null for no deps | 571 | ported | `nuget.rs` | `dotnet_tools_manifest_without_tools_returns_empty` | — |
| does not throw | 577 | ported | `nuget.rs` | `dotnet_tools_manifest_malformed_returns_empty` | — |

### `extractPackageFile() › single-csharp-file`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reads sdk and package directives | 583 | ported | `nuget.rs` | `single_csharp_file_reads_sdk_and_package_directives` | — |

### `extractPackageFile() › single-csharp-file-nuget`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| calls applyRegistries to honor nuget.config files if present | 615 | pending | — | — | Requires single-csharp-file + NuGet.config integration |

---

## `lib/modules/manager/ant/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/ant/extract.spec.ts
**Total tests:** 44 | **Ported:** 35 | **Actionable:** 44 | **Status:** partial

### `extractPackageFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts inline version dependencies from build.xml | 9 | ported | `ant.rs` | `extracts_inline_dependency` | — |
| extracts multiple dependencies | 33 | ported | `ant.rs` | `multiple_deps_extracted` | — |
| defaults depType to compile when no scope is set | 68 | ported | `ant.rs` | `defaults_dep_type_to_compile_without_scope` | — |
| returns null for invalid XML | 90 | ported | `ant.rs` | `invalid_xml_returns_empty` | — |
| returns null for build.xml with no dependencies | 94 | ported | `ant.rs` | `project_without_artifact_dependencies_returns_empty` | — |
| ignores dependency nodes without version | 104 | ported | `ant.rs` | `dependency_without_version_returns_empty` | — |
| extracts dependencies with single-quoted attributes | 119 | ported | `ant.rs` | `single_quoted_attributes_extracted` | — |
| returns null for unreadable build.xml | 135 | ported | `ant.rs` | `extract_all_package_files_ignores_unreadable_build_xml` | — |
| does not revisit the same file | 143 | ported | `ant.rs` | `extract_all_package_files_deduplicates_paths` | — |

### `property resolution`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| resolves inline property references | 167 | ported | `ant.rs` | `resolves_inline_property_references` | — |
| resolves properties from external .properties files | 193 | ported | `ant.rs` | `extract_all_package_files_resolves_external_properties_file` | — |
| implements first-definition-wins for inline properties | 228 | ported | `ant.rs` | `first_inline_property_definition_wins` | — |
| inline properties take precedence over file properties | 254 | ported | `ant.rs` | `extract_all_package_files_inline_properties_override_file_properties` | — |
| skips dependencies with unresolvable property references | 288 | ported | `ant.rs` | `property_ref_skipped` | — |
| detects circular property references | 312 | ported | `ant.rs` | `circular_property_reference_is_skipped` | — |
| resolves chained property references | 338 | ported | `ant.rs` | `resolves_chained_property_references` | — |
| groups multiple dependencies sharing the same property | 368 | ported | `ant.rs` | `resolves_shared_property_for_multiple_dependencies` | — |
| handles properties file in subdirectory | 400 | ported | `ant.rs` | `extract_all_package_files_resolves_subdirectory_properties_file` | — |
| handles unreadable properties file gracefully | 434 | ported | `ant.rs` | `extract_all_package_files_handles_unreadable_properties_file` | — |
| returns deps with mixed inline and property versions | 464 | ported | `ant.rs` | `returns_mixed_inline_and_property_versions` | — |
| ignores dependency without version during property resolution | 495 | ported | `ant.rs` | `ignores_dependency_without_version_during_property_resolution` | — |
| skips partial placeholder in version string | 522 | ported | `ant.rs` | `partial_placeholder_version_is_skipped` | — |

### `edge cases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles unparseable XML returned by readLocalFile | 549 | ported | `ant.rs` | `unparseable_xml_returns_empty` | — |
| handles absolute path in property file reference | 557 | ported | `ant.rs` | `extract_all_package_files_resolves_absolute_properties_file` | — |
| skips duplicate property file references | 591 | ported | `ant.rs` | `extract_all_package_files_deduplicates_properties_file_refs` | — |
| follows import file references | 628 | pending | — | — | Requires `<import>` element resolution |
| skips missing import files | 662 | pending | — | — | Requires missing-import tolerance |
| does not loop on self-importing files | 692 | pending | — | — | Requires self-import detection |
| shares properties across imported files | 722 | pending | — | — | Requires multi-file property sharing |
| extracts dependency from 3-part coords attribute | 760 | ported | `ant.rs` | `extracts_coords_form` | — |
| extracts scope from 4-part coords attribute | 791 | ported | `ant.rs` | `four_part_coords_with_scope_at_end` | — |
| ignores coords with fewer than 3 parts | 821 | ported | `ant.rs` | `coords_with_fewer_than_3_parts_skipped` | — |
| ignores coords with empty groupId | 840 | ported | `ant.rs` | `coords_with_empty_groupid_skipped` | — |
| resolves property references in coords version | 859 | ported | `ant.rs` | `resolves_property_references_in_coords_version` | — |
| marks coords dependency with unresolvable property | 890 | ported | `ant.rs` | `coords_with_unresolvable_property_is_skipped` | — |
| treats last part as version when it is not a known scope | 919 | ported | `ant.rs` | `four_part_coords_last_segment_is_version_when_not_a_scope` | — |
| collects registry URLs from remoteRepository elements | 949 | ported | `ant.rs` | `remote_repository_collected` | — |
| passes registry URLs to coords-style dependencies | 979 | ported | `ant.rs` | `remote_repository_applies_to_coords_dependency` | — |
| collects registry URLs from settingsFile attribute | 1009 | pending | — | — | Requires settingsFile attribute parsing |
| merges registries from settingsFile and remoteRepository | 1047 | pending | — | — | Requires registries merging |
| handles absolute settingsFile path | 1089 | pending | — | — | Requires absolute settingsFile path |
| logs debug when settingsFile cannot be read | 1127 | pending | — | — | Requires settingsFile read-failure tolerance |
| does not pass registries to dependencies outside the block | 1155 | pending | — | — | Requires registry-block scoping |
| handles chain referencing undefined property | 1191 | ported | `ant.rs` | `chain_referencing_undefined_property_is_skipped` | — |

---

## `lib/modules/manager/kustomize/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/kustomize/extract.spec.ts
**Total tests:** 44 | **Ported:** 39 | **Actionable:** 39 | **Status:** ported

### `parseKustomize` (top-level)

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should successfully parse a valid kustomize file | 16 | not-applicable | — | — | Tests TS-internal parseKustomize helper directly; Rust extractor has no equivalent public entry point |
| return null on an invalid file | 33 | ported | `kustomize.rs` | `empty_content_returns_empty` | — |
| should return null when header has invalid resource kind | 38 | ported | `kustomize.rs` | `invalid_resource_kind_returns_none` | — |
| should fall back to default resource kind when header is missing | 47 | ported | `kustomize.rs` | `missing_kind_defaults_to_kustomization` | — |
| should extract chartHome | 56 | ported | `kustomize.rs` | `extracts_chart_home` | — |

### `extractBase`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null for a local base | 66 | ported | `kustomize.rs` | `local_base_returns_none` | — |
| should return null for an http base without ref/version | 71 | ported | `kustomize.rs` | `http_base_without_ref_returns_none` | — |
| should extract out the version of an http base | 77 | ported | `kustomize.rs` | `extracts_http_base_ref` | — |
| should extract the version of a non http base | 90 | ported | `kustomize.rs` | `extracts_non_http_ssh_base_ref` | — |
| should extract the depName if the URL includes a port number | 102 | ported | `kustomize.rs` | `extracts_ssh_base_with_port` | — |
| should extract the version of a non http base with subdir | 114 | ported | `kustomize.rs` | `extracts_ssh_base_with_subdir` | — |
| should extract out the version of an github base | 126 | ported | `kustomize.rs` | `extracts_github_shorthand_base_ref` | — |
| should extract out the version of a git base | 139 | ported | `kustomize.rs` | `extracts_git_at_github_base_ref` | — |
| should extract out the version of a git base with subdir | 152 | ported | `kustomize.rs` | `extracts_git_at_github_base_with_subdir` | — |
| should extract out the version of an http base with additional params | 165 | ported | `kustomize.rs` | `extracts_http_base_ref_with_additional_params` | — |
| should extract out the version of an http base from first version param | 180 | ported | `kustomize.rs` | `extracts_http_base_first_version_param` | — |
| should extract out the version of an http base from first ref param | 193 | ported | `kustomize.rs` | `extracts_http_base_first_ref_param` | — |

### `extractHelmChart`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null on a null input | 208 | not-applicable | — | — | Tests TS-internal extractHelmChart helper directly with null input |
| should correctly extract a chart | 217 | ported | `kustomize.rs` | `extracts_helm_charts` | — |
| should correctly extract an OCI chart | 233 | ported | `kustomize.rs` | `extracts_oci_helm_chart` | — |
| should correctly extract an OCI chart with registryAliases | 249 | ported | `kustomize.rs` | `extracts_oci_helm_chart_with_registry_aliases` | — |

### `image extraction`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null on a null input | 270 | not-applicable | — | — | Tests TS-internal image-extraction helper directly with null input |
| should return null on invalid input | 278 | not-applicable | — | — | Tests TS-internal image-extraction helper directly with invalid input |
| should correctly extract a default image | 287 | ported | `kustomize.rs` | `extracts_images` | — |
| should correctly extract an image in a repo | 305 | ported | `kustomize.rs` | `extracts_image_in_repo` | — |
| should correctly extract from a different registry | 323 | ported | `kustomize.rs` | `extracts_image_from_different_registry` | — |
| should correctly extract from a different port | 341 | ported | `kustomize.rs` | `extracts_image_from_registry_with_port` | — |
| should correctly extract from a multi-depth registry | 359 | ported | `kustomize.rs` | `extracts_image_from_multi_depth_registry` | — |
| should correctly extract with registryAliases | 377 | ported | `kustomize.rs` | `extracts_image_with_registry_aliases` | — |

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for non kustomize kubernetes files | 400 | ported | `kustomize.rs` | `non_kustomize_kubernetes_file_returns_empty` | — |
| extracts multiple image lines | 416 | ported | `kustomize.rs` | `extracts_multiple_base_lines` | — |
| extracts ssh dependency | 444 | ported | `kustomize.rs` | `package_file_extracts_ssh_dependency` | — |
| extracts ssh dependency with a subdir | 462 | ported | `kustomize.rs` | `package_file_extracts_ssh_dependency_with_subdir` | — |
| extracts http dependency | 481 | ported | `kustomize.rs` | `package_file_extracts_http_dependencies` | — |
| should extract out image versions | 506 | ported | `kustomize.rs` | `package_file_extracts_image_versions` | — |
| ignores non-Kubernetes empty files | 586 | ported | `kustomize.rs` | `ignores_non_kubernetes_empty_files` | — |
| does nothing with kustomize empty kustomize files | 590 | ported | `kustomize.rs` | `empty_kustomization_returns_empty` | — |
| should extract bases resources and components from their respective blocks | 598 | ported | `kustomize.rs` | `extracts_bases_resources_and_components_blocks` | — |
| should extract dependencies when kind is Component | 632 | ported | `kustomize.rs` | `extracts_dependencies_when_kind_is_component` | — |

### `extractResource`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts from newTag | 675 | ported | `kustomize.rs` | `extracts_images` | — |
| extracts from digest | 710 | ported | `kustomize.rs` | `extracts_images_from_digest` | — |
| extracts newName | 757 | ported | `kustomize.rs` | `extracts_new_name_override` | — |
| parses helmChart field | 799 | ported | `kustomize.rs` | `mixed_images_and_helm` | — |
| extracts from various URL forms (it.each) | 1104 | not-applicable | — | — | Tests TS-internal `extractResource` helper directly across many URL forms; Rust extractor has no equivalent public-API entry point |

---

## `lib/modules/manager/nix/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/nix/extract.spec.ts
**Total tests:** 38 | **Ported:** 4 | **Actionable:** 38 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when no nixpkgs input exists | 10 | pending | — | — | Requires `extractPackageFile(flake.nix, ...)` integration with flake.lock read |
| does not include nixpkgs input with no explicit ref | 25 | pending | — | — | Requires flake.nix + flake.lock integration |
| includes nixpkgs input with only ref | 42 | pending | — | — | Requires flake.nix + flake.lock integration |
| returns null when no inputs | 59 | pending | — | — | Requires flake.nix + flake.lock integration |
| returns null when inputs are missing locked | 71 | pending | — | — | Requires flake.lock locked-section validation |
| returns null when inputs are missing original | 95 | pending | — | — | Requires flake.lock original-section validation |
| returns null when original inputs are from local path | 121 | pending | — | — | Requires path-input filtering |
| returns null when locked inputs are indirect | 153 | pending | — | — | Requires indirect-input filtering |
| returns null when locked inputs are from local path | 185 | pending | — | — | Requires path-input filtering |
| returns nixpkgs input | 217 | ported | `nix.rs` | `extracts_nixpkgs_correctly` | — |
| includes nixpkgs with no explicit ref | 260 | pending | — | — | Requires explicit-ref handling |
| includes patchelf from HEAD | 300 | pending | — | — | Requires HEAD ref handling |
| includes ijq from sourcehut without a flake | 358 | pending | — | — | Requires sourcehut detection |
| includes home-manager from gitlab | 399 | pending | — | — | Requires gitlab detection |
| test other version | 440 | pending | — | — | Requires older flake.lock version handling |
| includes nixpkgs with ref and shallow arguments | 452 | pending | — | — | Requires shallow-arg handling |
| includes nixpkgs but using indirect type that cannot be updated | 494 | pending | — | — | Requires indirect-type skip-reason |
| includes nixpkgs but using indirect type and path locked type that cannot be updated | 524 | pending | — | — | Requires indirect+path-type skip-reason |
| includes flake from GitHub Enterprise | 553 | pending | — | — | Requires GitHub Enterprise detection |
| includes flake with tarball type | 649 | pending | — | — | Requires tarball-type handling |
| uri decode gitlab subgroup | 750 | pending | — | — | Requires URI-decode for gitlab subgroup |
| includes flake with only tarball type | 790 | pending | — | — | Requires tarball-only handling |
| includes flake with nixpkgs-lib as tarball type | 818 | pending | — | — | Requires nixpkgs-lib tarball detection |
| includes flake with nixpkgs channel as tarball type | 897 | pending | — | — | Requires nixpkgs-channel tarball detection |
| finds currentDigest correctly when input sha is pinned | 937 | pending | — | — | Requires currentDigest extraction |
| does not duplicate nixpkgs dependency | 983 | pending | — | — | Requires dedup logic |
| returns null when flake.lock file cannot be read | 1028 | pending | — | — | Requires flake.lock read-failure path |
| returns null when flake.nix file cannot be read | 1033 | pending | — | — | Requires flake.nix read-failure path |
| returns null when flake.lock has invalid JSON | 1046 | ported | `nix.rs` | `invalid_json_returns_empty` | — |
| returns deps when no root inputs but deps exist | 1051 | pending | — | — | Requires non-root deps fallback |
| handles currentDigest replacement when config provided | 1065 | pending | — | — | Requires currentDigest replacement plumbing |
| includes nixpkgs with ref when original has rev | 1112 | pending | — | — | Requires ref+rev priority handling |
| includes github flake with ref when original has rev | 1154 | pending | — | — | Requires github flake ref+rev handling |
| includes gitlab flake with custom host | 1196 | pending | — | — | Requires gitlab custom-host handling |
| includes sourcehut flake with custom host | 1238 | pending | — | — | Requires sourcehut custom-host handling |
| includes tarball flake with ref when original has rev | 1280 | pending | — | — | Requires tarball ref+rev handling |
| handles unknown flake lock type | 1321 | ported | `nix.rs` | `unknown_flake_lock_type_returns_empty` | — |
| ignores unsupported file type and still extracts other inputs | 1348 | ported | `nix.rs` | `unsupported_file_type_is_ignored_while_other_inputs_extract` | — |

---

## `lib/modules/manager/flux/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/flux/extract.spec.ts
**Total tests:** 59 | **Ported:** 59 | **Actionable:** 59 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts multiple resources | 27 | ported | `flux.rs` | `extracts_multiple_resources` | — |
| extracts version and components from system manifest at $filepath | 72 | ported | `flux.rs` | `extracts_version_with_components` | — |
| considers components optional in system manifests | 102 | ported | `flux.rs` | `extracts_version_without_components` | — |
| ignores system manifests without a version | 111 | ported | `flux.rs` | `no_header_returns_none` | — |
| extracts releases without repositories | 119 | ported | `flux.rs` | `extracts_helm_release_without_repository` | — |
| falls back to unknown-registry when registryAliases has no matching HelmRelease sourceRef name | 136 | ported | `flux.rs` | `helm_release_registry_alias_without_source_match_is_unknown` | — |
| uses registryAliases to resolve HelmRelease sourceRef name when repository is missing | 158 | ported | `flux.rs` | `helm_release_registry_alias_resolves_source_name` | — |
| uses registryAliases with an OCI URL for HelmRelease sourceRef name | 180 | ported | `flux.rs` | `helm_release_registry_alias_oci_url_uses_docker` | — |
| ignores HelmRelease resources without an apiVersion | 202 | ported | `flux.rs` | `ignores_helm_release_without_api_version` | — |
| ignores HelmRepository resources without an apiVersion | 207 | ported | `flux.rs` | `ignores_helm_repository_without_api_version` | — |
| ignores HelmRepository resources without metadata | 212 | ported | `flux.rs` | `ignores_helm_repository_without_metadata` | — |
| ignores HelmRelease resources without any chart reference | 234 | ported | `flux.rs` | `ignores_helm_release_without_chart_reference` | — |
| ignores HelmRelease resources without a chart name | 250 | ported | `flux.rs` | `ignores_helm_release_without_chart_name` | — |
| skip HelmRelease with local chart | 271 | ported | `flux.rs` | `skips_helm_release_with_local_chart` | — |
| does not match HelmRelease resources without a namespace to HelmRepository resources without a namespace | 299 | ported | `flux.rs` | `does_not_match_release_without_namespace_to_repository_without_namespace` | — |
| does not match HelmRelease resources without a sourceRef | 325 | ported | `flux.rs` | `release_without_source_ref_is_unknown_registry` | — |
| does not match HelmRelease resources without a namespace | 355 | ported | `flux.rs` | `does_not_match_release_without_namespace` | — |
| ignores HelmRepository resources without a namespace | 376 | ported | `flux.rs` | `ignores_helm_repository_without_namespace` | — |
| ignores HelmRepository resources without a URL | 400 | ported | `flux.rs` | `ignores_helm_repository_without_url` | — |
| ignores HelmRelease resources using an invalid chartRef | 425 | ported | `flux.rs` | `ignores_helm_release_with_invalid_chart_ref` | — |
| ignores HelmRelease resources using a chartRef targetting a HelmChart | 433 | ported | `flux.rs` | `ignores_release_chart_ref_and_extracts_helm_chart` | — |
| ignores HelmRelease resources using a chartRef targetting an OCIRepository | 457 | ported | `flux.rs` | `ignores_release_chart_ref_and_extracts_oci_repository` | — |
| extracts HelmChart version | 492 | ported | `flux.rs` | `extracts_helm_chart_version` | — |
| does not match HelmChart resources without a namespace | 513 | ported | `flux.rs` | `helm_chart_without_namespace_is_unknown_registry` | — |
| falls back to unknown-registry when registryAliases has no matching HelmChart sourceRef name | 544 | ported | `flux.rs` | `helm_chart_registry_alias_without_source_match_is_unknown` | — |
| uses registryAliases to resolve HelmChart sourceRef name when repository is missing | 566 | ported | `flux.rs` | `helm_chart_registry_alias_resolves_source_name` | — |
| ignores HelmChart resources using git sources | 588 | ported | `flux.rs` | `ignores_helm_chart_using_git_source` | — |
| ignores HelmChart resources using bucket sources | 608 | ported | `flux.rs` | `helm_chart_using_bucket_source_is_unsupported` | — |
| ignores GitRepository without a tag nor a commit | 645 | ported | `flux.rs` | `ignores_git_repository_without_tag_or_commit` | — |
| extracts GitRepository with a commit | 665 | ported | `flux.rs` | `extracts_git_repository_with_commit` | — |
| extracts GitRepository with a tag from github with ssh | 694 | ported | `flux.rs` | `extracts_git_repository_tag_from_github_ssh` | — |
| extracts GitRepository with a tag from github | 722 | ported | `flux.rs` | `extracts_git_repository_tag_from_github` | — |
| extracts GitRepository with a tag from gitlab | 750 | ported | `flux.rs` | `extracts_git_repository_tag_from_gitlab` | — |
| extracts GitRepository with a tag from bitbucket | 778 | ported | `flux.rs` | `extracts_git_repository_tag_from_bitbucket` | — |
| extracts GitRepository with a tag from an unkown domain | 806 | ported | `flux.rs` | `extracts_git_repository_tag_from_unknown_domain` | — |
| ignores OCIRepository with no tag and no digest | 834 | ported | `flux.rs` | `oci_repository_without_tag_or_digest_is_unversioned` | — |
| extracts OCIRepository with a tag | 861 | ported | `flux.rs` | `extracts_oci_repository_with_tag` | — |
| extracts OCIRepository with a digest | 897 | ported | `flux.rs` | `extracts_oci_repository_with_digest` | — |
| extracts OCIRepository with a tag that contains a digest | 925 | ported | `flux.rs` | `extracts_oci_repository_with_tag_containing_digest` | — |
| extracts OCIRepository with a digest and tag | 958 | ported | `flux.rs` | `extracts_oci_repository_with_digest_and_tag` | — |
| extracts OCIRepository with quoted digest and tag | 994 | ported | `flux.rs` | `extracts_oci_repository_with_quoted_digest_and_tag` | — |
| extracts OCIRepository with quoted keys | 1030 | ported | `flux.rs` | `extracts_oci_repository_with_quoted_keys` | — |
| extracts OCIRepository when ref key is quoted | 1063 | ported | `flux.rs` | `extracts_oci_repository_with_quoted_ref_key` | — |
| skips OCIRepository when tag value is a YAML alias | 1098 | ported | `flux.rs` | `skips_oci_repository_when_tag_value_is_yaml_alias` | — |
| extracts OCIRepository with tag and digest preceded by other document types | 1129 | ported | `flux.rs` | `extracts_oci_repository_after_other_document_types` | — |
| extracts OCIRepository with tag and digest when preceded by same-named resource with scalar ref | 1195 | ported | `flux.rs` | `extracts_oci_repository_after_same_name_scalar_ref` | — |
| extracts OCIRepository with tag and digest when preceded by same-named resource with scalar spec | 1241 | ported | `flux.rs` | `extracts_oci_repository_after_same_name_scalar_spec` | — |
| extracts OCIRepository with tag and digest when ref contains a non-scalar key | 1285 | ported | `flux.rs` | `extracts_oci_repository_when_ref_contains_non_scalar_key` | — |
| extracts Kustomization | 1323 | ported | `flux.rs` | `extracts_kustomization_images` | — |
| ignores resources of an unknown kind | 1389 | ported | `flux.rs` | `ignores_resources_of_unknown_kind` | — |
| ignores resources without a kind | 1400 | ported | `flux.rs` | `ignores_resources_without_kind` | — |
| ignores bad manifests | 1408 | ported | `flux.rs` | `ignores_bad_manifests` | — |
| ignores null resources | 1413 | ported | `flux.rs` | `ignores_null_resources` | — |

### `extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts multiple files | 1420 | ported | `flux.rs` | `extract_all_package_files_extracts_multiple_files` | — |
| should handle HelmRepository with type OCI | 1486 | ported | `flux.rs` | `extract_all_package_files_handles_helm_repository_type_oci` | — |
| should handle HelmRepository w/o type oci and url starts with oci | 1514 | ported | `flux.rs` | `extract_all_package_files_handles_helm_repository_oci_url_without_type` | — |
| ignores files that do not exist | 1535 | ported | `flux.rs` | `extract_all_package_files_ignores_missing_files` | — |
| ignores system manifest files without valid Flux version header | 1542 | ported | `flux.rs` | `extract_all_package_files_ignores_invalid_system_manifest` | — |
| should pick correct package file when using HelmRepository with chartRef | 1549 | ported | `flux.rs` | `extract_all_package_files_picks_helm_chart_package_file_for_chart_ref` | — |

---

## `lib/modules/manager/bazel-module/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazel-module/extract.spec.ts
**Total tests:** 35 | **Ported:** 34 | **Actionable:** 34 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if fails to parse | 25 | ported | `bazel_module.rs` | `malformed_content_returns_empty` | — |
| returns null if something throws an error | 33 | not-applicable | — | — | TypeScript mock-injected parser exception; Rust extractor has no parser mock hook and malformed input tolerance is covered by `malformed_content_returns_empty` |
| returns null if file is empty | 41 | ported | `bazel_module.rs` | `empty_content_returns_empty` | — |
| returns null if file has unrecognized declarations | 46 | ported | `bazel_module.rs` | `empty_file_returns_empty` (+ comment_lines_stripped) | — |
| returns bazel_dep and git_override dependencies | 54 | ported | `bazel_module.rs` | `extracts_bazel_dep` (+ extracts_dev_dependency, extracts_multiline_dep, multiple_deps) | — |
| returns bazel_dep with no version and git_override | 95 | ported | `bazel_module.rs` | `dep_without_version_skipped` | — |
| returns dependencies and custom registry URLs when specified in a bazelrc | 125 | ported | `bazel_module.rs` | `extracts_bazelrc_registry_urls_for_module` | — |
| returns bazel_dep and archive_override dependencies | 148 | ported | `bazel_module.rs` | `extracts_archive_override_with_bazel_dep_version` | — |
| returns bazel_dep with no version and archive_override dependencies | 179 | ported | `bazel_module.rs` | `extracts_archive_override_with_unversioned_bazel_dep` | — |
| returns bazel_dep and local_path_override dependencies | 209 | ported | `bazel_module.rs` | `extracts_local_path_override_with_bazel_dep_version` | — |
| returns bazel_dep with no version and local_path_override dependencies | 238 | ported | `bazel_module.rs` | `extracts_local_path_override_with_unversioned_bazel_dep` | — |
| returns bazel_dep and single_version_override dependencies if a version is specified | 266 | ported | `bazel_module.rs` | `extracts_single_version_override_with_bazel_dep_version` | — |
| returns bazel_dep with no version and single_version_override dependencies if a version is specified | 299 | ported | `bazel_module.rs` | `extracts_single_version_override_with_unversioned_bazel_dep` | — |
| returns bazel_dep dependency if single_version_override does not have a version | 331 | ported | `bazel_module.rs` | `single_version_override_without_version_only_adds_registry_to_versioned_bazel_dep` | — |
| returns bazel_dep with no version dependency if single_version_override does not have a version | 355 | ported | `bazel_module.rs` | `single_version_override_without_version_keeps_unversioned_bazel_dep_skipped` | — |
| returns crate.spec dependencies | 377 | ported | `bazel_module.rs` | `extracts_crate_spec_dependencies` | — |
| returns maven.install and maven.artifact dependencies | 453 | ported | `bazel_module.rs` | `extracts_maven_install_and_artifact_dependencies` | — |
| returns oci.pull dependencies | 507 | ported | `bazel_module.rs` | `extracts_oci_pull_dependency` | — |
| returns oci.pull dependencies without tags | 544 | ported | `bazel_module.rs` | `extracts_oci_pull_dependency_without_tag` | — |
| returns oci.pull dependencies with tag only (no digest) | 578 | ported | `bazel_module.rs` | `extracts_oci_pull_dependency_with_tag_only` | — |
| returns oci.pull dependencies without tag or digest | 611 | ported | `bazel_module.rs` | `extracts_oci_pull_dependency_without_tag_or_digest` | — |
| returns oci.pull dependencies with registryAliases | 641 | ported | `bazel_module.rs` | `extracts_oci_pull_dependency_with_registry_alias` | — |
| returns oci.pull dependencies with registryAliases with multiple segments | 682 | ported | `bazel_module.rs` | `extracts_oci_pull_dependency_with_multisegment_registry_alias` | — |
| returns maven.install and bazel_dep dependencies together | 723 | ported | `bazel_module.rs` | `extracts_maven_install_and_bazel_dep_together` | — |
| returns git_repository dependencies with digest | 772 | ported | `bazel_module.rs` | `extracts_git_repository_dependency_with_digest` | — |
| returns git_repository dependencies with tag | 796 | ported | `bazel_module.rs` | `extracts_git_repository_dependency_with_tag` | — |
| returns new_git_repository dependencies | 820 | ported | `bazel_module.rs` | `extracts_new_git_repository_dependency` | — |
| handles a real-world MODULE.bazel file (rules_sh) | 846 | ported | `bazel_module.rs` | `extracts_rules_sh_real_world_module_bazel` | — |
| handles every method available in MODULE.bazel files | 887 | ported | `bazel_module.rs` | `extracts_every_supported_module_bazel_method` | — |
| returns rules_img pull dependencies | 1005 | ported | `bazel_module.rs` | `extracts_rules_img_pull_dependency` | — |
| returns rules_img pull dependencies with custom registry | 1051 | ported | `bazel_module.rs` | `extracts_rules_img_pull_dependency_with_custom_registry` | — |
| returns rules_img pull dependencies with multiple pulls | 1086 | ported | `bazel_module.rs` | `extracts_multiple_rules_img_pull_dependencies` | — |
| ignores rules_img pull without required fields | 1141 | ported | `bazel_module.rs` | `ignores_rules_img_pull_without_required_fields` | — |
| handles rules_img with renamed variable | 1161 | ported | `bazel_module.rs` | `extracts_rules_img_pull_dependency_with_renamed_variable` | — |
| ignores non-rules_img repo rules | 1193 | ported | `bazel_module.rs` | `ignores_non_rules_img_repo_rules` | — |

---

## `lib/modules/manager/cpanfile/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/cpanfile/extract.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 10 | **Status:** ported

### `extractPackageFile() › parse perl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 6 | ported | `cpanfile.rs` | `empty_input_returns_no_deps` | — |
| parse modules with requires | 39 | ported | `cpanfile.rs` | `extracts_basic_requires` (+ extracts_fat_arrow_form) | — |
| parse modules with recommends | 113 | ported | `cpanfile.rs` | `parse_modules_with_recommends` | — |
| parse modules with suggests | 138 | ported | `cpanfile.rs` | `parse_modules_with_suggests` | — |

### `extractPackageFile() › parse modules with phases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| configure phase | 164 | ported | `cpanfile.rs` | `parse_phase_configure` | — |
| build phase | 186 | ported | `cpanfile.rs` | `parse_phase_build_bareword` | — |
| phase | 208 | ported | `cpanfile.rs` | `extracts_test_phase_block` | — |
| runtime phase | 237 | ported | `cpanfile.rs` | `parse_phase_runtime_bareword_suggests` | — |
| develop phase | 266 | ported | `cpanfile.rs` | `parse_phase_develop` | — |

### `extractPackageFile() › parse modules with phase shortcuts`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $shortcut (configure_requires/build_requires/test_requires/author_requires) | 296 | ported | `cpanfile.rs` | `extracts_phase_shortcut_keywords` | — |

---

## `lib/modules/manager/pip-compile/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pip-compile/extract.spec.ts
**Total tests:** 26 | **Ported:** 0 | **Actionable:** 26 | **Status:** pending

The pip-compile manager has no Rust extractor yet — all rows pending
the implementation of `extractAllPackageFiles()` (multi-file path
resolver) and the inner `extractPackageFile()` adapter.

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns object for requirements.in | 40 | pending | — | — | pip-compile manager not implemented |
| returns object for setup.py | 50 | pending | — | — | pip-compile manager not implemented |
| returns object for pyproject.toml | 60 | pending | — | — | pip-compile manager not implemented |
| handles different file extensions (it.each) | 93 | pending | — | — | pip-compile manager not implemented |

### `extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| support package file with multiple lock files | 105 | pending | — | — | pip-compile manager not implemented |
| no lock files in returned package files | 137 | pending | — | — | pip-compile manager not implemented |
| no constraint files in returned package files | 162 | pending | — | — | pip-compile manager not implemented |
| return null for malformed files | 183 | pending | — | — | pip-compile manager not implemented |
| return null for bad paths | 221 | pending | — | — | pip-compile manager not implemented |
| return for valid paths | 246 | pending | — | — | pip-compile manager not implemented |
| return sorted package files | 281 | pending | — | — | pip-compile manager not implemented |
| return sorted package files with constraint in file | 311 | pending | — | — | pip-compile manager not implemented |
| return sorted package files with constraint in command | 335 | pending | — | — | pip-compile manager not implemented |
| adds lockedVersion to deps in package file | 360 | pending | — | — | pip-compile manager not implemented |
| warns if dependency has no locked version | 382 | pending | — | — | pip-compile manager not implemented |
| adds transitive dependency to deps in package file | 403 | pending | — | — | pip-compile manager not implemented |
| handles -r reference to another input file | 427 | pending | — | — | pip-compile manager not implemented |
| handles transitive -r references | 455 | pending | — | — | pip-compile manager not implemented |
| warns on -r reference to failed file | 491 | pending | — | — | pip-compile manager not implemented |
| warns on -r reference to requirements file not managed by pip-compile | 516 | pending | — | — | pip-compile manager not implemented |
| handles duplicate -r dependencies | 539 | pending | — | — | pip-compile manager not implemented |
| handles -r dependency on lock file with multiple input files | 583 | pending | — | — | pip-compile manager not implemented |
| handles -r dependency on input file that is also used to generate lock file with multiple inputs | 614 | pending | — | — | pip-compile manager not implemented |
| handles -r dependency on file with relative path same dir | 645 | pending | — | — | pip-compile manager not implemented |
| handles -r dependency on file with relative path above | 673 | pending | — | — | pip-compile manager not implemented |
| handles -r dependency on file with relative path above with path | 701 | pending | — | — | pip-compile manager not implemented |

---

## `lib/modules/manager/maven/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/maven/extract.spec.ts
**Total tests:** 29 | **Ported:** 8 | **Actionable:** 29 | **Status:** partial

### `extractPackage`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid XML | 22 | ported | `maven.rs` | `empty_pom_returns_empty` (+ multiline_element_values_trimmed) | — |
| extract dependencies from any XML position | 29 | ported | `maven.rs` | `extracts_regular_dependencies` (+ extracts_parent, extracts_dependency_management, extracts_build_plugins, plugin_default_group_id, extracts_build_extensions, property_resolved_from_properties_section, profile_dependencies_extracted) | — |
| extract dependencies with windows line endings | 237 | ported | `maven.rs` | `windows_line_endings_are_tolerated` | — |
| tries minimum manifests | 249 | ported | `maven.rs` | `extracts_regular_dependencies` | — |
| tries minimum snapshot manifests | 264 | ported | `maven.rs` | `extracts_regular_dependencies` | — |
| extracts builder and buildpack images from spring-boot plugin | 279 | pending | — | — | Requires spring-boot plugin builder/buildpack extraction |
| extracts only builder if defaults are used in spring-boot plugin | 370 | pending | — | — | Requires spring-boot plugin builder default handling |
| returns no buildpack dependencies when image tag is missing in spring boot plugin configuration | 398 | pending | — | — | Requires spring-boot plugin missing-tag handling |
| returns no buildpack dependencies when dependencies are invalid in spring boot plugin | 407 | pending | — | — | Requires spring-boot plugin tolerant parsing |

### `resolveParents`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should apply props recursively | 418 | ported | `maven.rs` | `recursive_property_resolution` | — |
| should apply props multiple times | 432 | ported | `maven.rs` | `pdm_style_pom_with_properties` | — |
| should detect props infinitely recursing props | 448 | ported | `maven.rs` | `substitute_props_unclosed_brace` (+ substitute_props_handles_unknown_key) | — |

### `extractRegistries`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid XML | 471 | pending | — | — | Requires settings.xml registry-extraction parser |
| extract registries from a simple mirror settings file | 478 | pending | — | — | Requires settings.xml mirror parser |
| extract registries from a simple profile settings file | 485 | pending | — | — | Requires settings.xml profile parser |
| extract registries from a complex profile settings file | 492 | pending | — | — | Requires settings.xml complex profile parser |
| extract registries from a settings file that uses a newer schema | 503 | pending | — | — | Requires settings.xml newer-schema tolerance |

### `extractExtensions`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid xml files | 527 | pending | — | — | Requires .mvn/extensions.xml extractor |

### `extractAllPackageFiles`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty if package has no content | 548 | pending | — | — | Requires extractAllPackageFiles wrapper |
| should return empty for packages with invalid content | 554 | pending | — | — | Requires extractAllPackageFiles wrapper |
| should return packages with urls from a settings file | 560 | pending | — | — | Requires settings.xml integration |
| should include registryUrls from parent pom files | 581 | pending | — | — | Requires multi-file parent resolution |
| should include registryUrls in the correct order | 791 | pending | — | — | Requires registryUrls ordering |
| should return package files info | 812 | pending | — | — | Requires package files info aggregation |
| should extract from .mvn/extensions.xml file | 888 | pending | — | — | Requires .mvn/extensions.xml integration |
| should return empty array if extensions file is invalid or empty | 917 | pending | — | — | Requires .mvn/extensions.xml tolerant parsing |

### `extractAllPackageFiles › root pom handling`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should skip root pom.xml | 930 | pending | — | — | Requires root-pom detection |
| should skip root pom.xml when it has an external parent | 964 | pending | — | — | Requires root-pom detection with external parent |
| handles cross-referencing | 1006 | pending | — | — | Requires multi-file pom cross-reference resolution |

---

## `lib/modules/manager/poetry/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/poetry/extract.spec.ts
**Total tests:** 34 | **Ported:** 23 | **Actionable:** 34 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 43 | ported | `poetry.rs` | `empty_content_returns_empty` | — |
| returns null for parsed file without poetry section | 47 | ported | `poetry.rs` | `no_poetry_section_returns_empty` | — |
| extracts multiple dependencies | 51 | ported | `poetry.rs` | `extracts_string_deps` (+ poetry_fixture_1) | — |
| extracts multiple dependencies (with dep = {version = "1.2.3"} case) | 60 | ported | `poetry.rs` | `extracts_table_deps` | — |
| handles case with no dependencies | 66 | ported | `poetry.rs` | `poetry_section_with_no_deps_returns_empty` | — |
| handles multiple constraint dependencies | 71 | pending | — | — | Requires multi-constraint dep table-of-tables parsing |
| extracts build-system.requires dependencies | 77 | ported | `poetry.rs` | `extracts_build_system_requires` | — |
| can parse TOML v1 heterogeneous arrays | 112 | pending | — | — | Requires TOML v1 heterogeneous-array tolerance |
| extracts mixed versioning types | 118 | ported | `poetry.rs` | `name_normalized_per_pep503` | — |
| extracts dependencies from dependency groups | 160 | ported | `poetry.rs` | `extracts_group_dependencies` (+ extracts_dev_dependencies) | — |
| resolves lockedVersions from the lockfile | 197 | pending | — | — | Requires poetry.lock parsing |
| parses git dependencies long commit hashes on http urls | 209 | pending | — | — | Requires git dep + commit hash extraction |
| parses git dependencies short commit hashes on http urls | 234 | pending | — | — | Requires git dep + short commit hash extraction |
| parses git dependencies long commit hashes on ssh urls | 259 | pending | — | — | Requires git dep + ssh URL extraction |
| parses git dependencies long commit hashes on http urls with branch marker | 284 | pending | — | — | Requires git dep + branch marker handling |
| parses github dependencies tags on ssh urls | 310 | pending | — | — | Requires git dep + GitHub-tags datasource selection on ssh |
| parses github dependencies tags on http urls | 325 | pending | — | — | Requires git dep + GitHub-tags datasource selection |
| parses git dependencies with tags that are not on GitHub | 340 | pending | — | — | Requires generic git-tags datasource selection |
| skips git dependencies | 363 | ported | `poetry.rs` | `git_source_skipped` | — |
| skips git dependencies with version | 375 | ported | `poetry.rs` | `git_dep_with_version_shows_version` | — |
| skips path dependencies | 388 | ported | `poetry.rs` | `path_source_skipped` | — |
| skips path dependencies with version | 400 | ported | `poetry.rs` | `path_dep_with_version_shows_version` | — |
| does not include registry url for dependency python | 413 | ported | `poetry.rs` | `python_dependency_has_no_registry_urls` | — |
| can parse empty registries | 436 | ported | `poetry.rs` | `empty_registry_list_returns_no_registry_urls` | — |
| can parse missing registries | 441 | ported | `poetry.rs` | `missing_registry_list_returns_no_registry_urls` | — |
| extracts registries | 446 | ported | `poetry.rs` | `extracts_registry_urls` | — |
| dedupes registries | 455 | ported | `poetry.rs` | `dedupes_registry_urls` | — |
| source with priority="default" and implicit PyPI priority="primary" | 463 | ported | `poetry.rs` | `source_default_with_implicit_pypi_primary` | — |
| source with implicit priority and PyPI with priority="explicit" | 483 | ported | `poetry.rs` | `source_with_explicit_pypi_suppresses_implicit_pypi_url` | — |
| supports dependencies with explicit source | 500 | ported | `poetry.rs` | `dependencies_with_explicit_source_get_registry_urls` | — |
| parses package file with template | 535 | ported | `poetry.rs` | `parses_package_file_with_template_lines` | — |
| extract dependencies from the project section | 555 | ported | `poetry.rs` | `extracts_poetry_v2_project_section_dependencies` | — |
| extracts dependencies from pep735 dependency-groups | 616 | ported | `poetry.rs` | `extracts_pep735_dependency_groups` | — |
| enriches pep621/pep735 dependencies with poetry managerData | 663 | pending | — | — | Requires PEP 621/735 enrichment with poetry managerData |

---

## `lib/modules/manager/sbt/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/sbt/extract.spec.ts
**Total tests:** 26 | **Ported:** 6 | **Actionable:** 26 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 23 | ported | `sbt.rs` | `empty_returns_empty` (+ build_properties_extraction) | — |
| extracts deps for generic use-cases | 47 | ported | `sbt.rs` | `extracts_scala_style_deps` (+ extracts_java_style_deps, extracts_plugin, comment_line_skipped, dep_name_formats_correctly) | — |
| extracts deps when scala version is defined in a variable | 74 | pending | — | — | Requires scalaVersion := Var resolution |
| extracts deps when scala version is defined in an object | 99 | pending | — | — | Requires scalaVersion := Obj.field resolution |
| skips deps when dotted symbolds do not resolve to anything | 136 | pending | — | — | Requires unresolved-symbol skip path |
| extracts packageFileVersion when scala version is defined in a variable | 159 | pending | — | — | Requires packageFileVersion plumbing |
| extracts typed variables | 170 | pending | — | — | Requires typed `val x: T = "..."` parsing |
| skips deps when scala version is missing | 185 | pending | — | — | Requires scala-version-missing skip path |
| extract deps from native scala file with variables | 213 | pending | — | — | Requires .scala build file variable resolution |
| extracts deps when scala version is defined with a trailing comma | 232 | pending | — | — | Requires trailing-comma tolerance |
| extracts deps when scala version is defined in a variable with a trailing comma | 253 | pending | — | — | Requires trailing-comma tolerance |
| extracts deps when scala version is defined with ThisBuild scope | 275 | pending | — | — | Requires ThisBuild scope handling |
| extracts correct scala library when dealing with scala 3 | 294 | pending | — | — | Requires Scala 3 library naming |
| extracts deps correctly when dealing with scala 3 | 309 | pending | — | — | Requires Scala 3 dep resolution |
| extracts deps when scala version is defined in a variable with ThisBuild scope | 329 | pending | — | — | Requires ThisBuild + variable resolution |
| extract deps from native scala file with private variables | 349 | pending | — | — | Requires private variable resolution in .scala files |
| extract deps when they are defined in a new line | 371 | pending | — | — | Requires multi-line dep continuation parsing |
| extract deps with comment | 412 | pending | — | — | Requires inline-comment-stripping in dep position |
| extract addCompilerPlugin | 452 | ported | `sbt.rs` | `extracts_add_compiler_plugin` | — |
| extract sbt version | 469 | ported | `sbt.rs` | `build_properties_extracts_sbt_version` | — |
| extract sbt version if the file contains other properties | 492 | ported | `sbt.rs` | `build_properties_with_other_props_extracts_sbt_version` | — |
| ignores build.properties file if does not contain sbt version | 516 | ported | `sbt.rs` | `build_properties_without_sbt_version_returns_none` | — |
| extracts proxy repositories | 529 | pending | — | — | Requires repositories file parsing |
| should include default registryUrls if no repositories file is provided | 607 | pending | — | — | Requires registryUrls default plumbing |
| should return empty packagefiles is no content is provided | 637 | pending | — | — | Already partly covered by empty_returns_empty; TS uses extractAllPackageFiles wrapper not ported |
| extracts build properties correctly | 643 | pending | — | — | Requires extractAllPackageFiles wrapper port |

---

## `lib/modules/manager/terraform/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/terraform/extract.spec.ts
**Total tests:** 18 | **Ported:** 8 | **Actionable:** 18 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 39 | ported | `terraform.rs` | `empty_file_returns_empty` | — |
| returns null for no deps | 43 | ported | `terraform.rs` | `data_block_not_extracted` | — |
| extracts  modules | 54 | ported | `terraform.rs` | `module_with_version` (+ module_without_version_skipped, module_with_git_source_skipped, mixed_providers_and_modules) | — |
| extracts bitbucket modules | 221 | pending | — | — | Requires Bitbucket source URL detection |
| extracts azureDevOps modules | 306 | pending | — | — | Requires Azure DevOps source URL detection |
| resolves OCI registry aliases | 338 | pending | — | — | registryAliases not yet implemented |
| handles invalid OCI source URL | 358 | pending | — | — | Requires OCI source validation |
| extracts OCI modules and providers | 374 | pending | — | — | Requires OCI module / provider source parsing |
| extracts providers | 463 | ported | `terraform.rs` | `required_providers_block_form` (+ required_providers_inline_string_form, comments_ignored, provider_without_source_uses_name) | — |
| extracts docker resources | 579 | pending | — | — | Requires docker_image / docker_registry_image resource extraction |
| extracts kubernetes resources | 655 | pending | — | — | Requires kubernetes_manifest / kubernetes_pod resource extraction |
| returns dep with skipReason local | 756 | ported | `terraform.rs` | `module_with_local_path_skipped` (+ local_module_has_skip_reason) | — |
| returns null with only not added resources | 767 | ported | `terraform.rs` | `resource_block_not_extracted` | — |
| extract helm releases | 776 | pending | — | — | Requires helm_release resource extraction |
| update lockfile constraints with range strategy update-lockfile | 845 | pending | — | — | Requires .terraform.lock.hcl parsing |
| test terraform block with only requirement_terraform_version | 884 | ported | `terraform.rs` | `required_version_extracted_as_hashicorp_terraform` | — |
| extracts terraform_version for tfe_workspace and ignores missing terraform_version keys | 904 | pending | — | — | Requires tfe_workspace resource extraction |
| return null if invalid HCL file | 933 | ported | `terraform.rs` | `invalid_hcl_returns_empty` | — |

---

## `lib/modules/manager/homeassistant-manifest/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/homeassistant-manifest/extract.spec.ts
**Total tests:** 16 | **Ported:** 14 | **Actionable:** 16 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid JSON | 9 | ported | `homeassistant.rs` | `invalid_json_returns_empty` | — |
| returns null for non-Home Assistant manifest (missing domain) | 14 | ported | `homeassistant.rs` | `missing_domain_returns_empty` | — |
| returns null for non-Home Assistant manifest (missing name) | 24 | ported | `homeassistant.rs` | `missing_name_returns_empty` | — |
| returns null for chrome extension manifest | 34 | ported | `homeassistant.rs` | `chrome_extension_manifest_returns_empty` | — |
| returns null for empty requirements | 45 | ported | `homeassistant.rs` | `empty_requirements_returns_empty` | — |
| returns null when no requirements field | 55 | ported | `homeassistant.rs` | `no_requirements_field_returns_empty` | — |
| extracts single requirement with exact version | 64 | ported | `homeassistant.rs` | `extracts_single_requirement_exact_version` | — |
| extracts multiple requirements | 84 | ported | `homeassistant.rs` | `extracts_multiple_requirements` (+ extracts_requirements) | — |
| handles requirements with extras | 118 | ported | `homeassistant.rs` | `handles_requirements_with_extras` | — |
| extracts git+https requirements | 138 | pending | — | — | Requires git+https VCS source detection in homeassistant requirements |
| supports requirements with other operators | 168 | ported | `homeassistant.rs` | `extracts_range_version` | — |
| handles requirements without version | 211 | ported | `homeassistant.rs` | `handles_requirements_without_version` | — |
| extracts from real-world ASUSWRT manifest | 237 | ported | `homeassistant.rs` | `extracts_asuswrt_manifest` | — |
| handles invalid requirement types in array | 272 | ported | `homeassistant.rs` | `skips_non_string_entries_in_requirements_array` | — |
| returns null when requirements is not an array | 299 | ported | `homeassistant.rs` | `requirements_not_an_array_returns_empty` | — |
| handles unparseable requirement strings with skipReason | 313 | pending | — | — | Requires explicit skipReason path for unparseable PEP 508 strings |

---

## `lib/modules/manager/setup-cfg/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/setup-cfg/extract.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 6 | ported | `setup_cfg.rs` | `empty_content_returns_no_deps` | — |
| extracts dependencies | 10 | ported | `setup_cfg.rs` | `extracts_install_requires` (+ extracts_setup_requires, extracts_tests_require, extracts_extras_require, skips_git_source, normalizes_package_name, strips_env_markers, ignores_unrelated_sections) | — |

---

## `lib/modules/manager/mix/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/mix/extract.spec.ts
**Total tests:** 3 | **Ported:** 2 | **Actionable:** 3 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty for invalid dependency file | 11 | ported | `mix.rs` | `no_deps_function_returns_empty` (+ deps_without_do_end_block) | — |
| extracts all dependencies when no lockfile | 16 | ported | `mix.rs` | `simple_hex_dep` (+ real_world_mix_exs, dep_with_only_option, git_dep_skipped, github_dep_skipped, path_dep_skipped, dep_without_version_skipped) | — |
| extracts all dependencies and adds the locked version if lockfile present | 139 | pending | — | — | Requires mix.lock parsing + lockedVersion plumbing |

---

## `lib/modules/manager/leiningen/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/leiningen/extract.spec.ts
**Total tests:** 4 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `modules/manager/leiningen/extract`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| trimAtKey | 10 | not-applicable | — | — | TypeScript-internal helper; Rust extractor uses different parser structure |
| extractFromVectors | 22 | not-applicable | — | — | TypeScript-internal helper; Rust extractor uses different parser structure |
| extractPackageFile | 74 | ported | `leiningen.rs` | `extracts_dependencies` (+ extracts_managed_dependencies, extracts_plugins, dev_profile_dependencies_also_extracted) | — |
| extractVariables | 239 | not-applicable | — | — | TypeScript-internal helper; Rust handles variable expansion inline in extract() |

---

## `lib/modules/manager/pep723/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pep723/extract.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should extract dependencies | 10 | ported | `pep723.rs` | `extracts_script_block_with_version` (+ extracts_pinned_version, handles_direct_reference, normalizes_package_name) | — |

---

## `lib/modules/manager/unity3d/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/unity3d/extract.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles no version | 5 | ported | `unity3d.rs` | `returns_none_for_empty` | — |
| handles $packageName | 14 | ported | `unity3d.rs` | `extracts_plain_version` (+ extracts_with_revision_version) | — |
| handles $type version | 39 | ported | `unity3d.rs` | `extracts_alpha_beta_and_stable_versions_with_revisions` | — |

---

## `lib/modules/manager/quadlet/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/quadlet/extract.spec.ts
**Total tests:** 11 | **Ported:** 10 | **Actionable:** 11 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid quadlet file content | 19 | ported | `quadlet.rs` | `ignores_non_container_sections` | — |
| returns null for empty yaml file content | 24 | ported | `quadlet.rs` | `empty_returns_empty` | — |
| extracts from quadlet container unit | 29 | ported | `quadlet.rs` | `extracts_container_image` (+ skips_local_transport, skips_comment_lines, variable_ref_skipped) | — |
| extracts from quadlet image unit | 47 | ported | `quadlet.rs` | `image_section_extracted` | — |
| extracts from quadlet volume unit | 65 | ported | `quadlet.rs` | `volume_section_extracted` | — |
| handles docker prefix | 83 | ported | `quadlet.rs` | `strips_docker_transport_prefix` | — |
| handles docker-daemon prefix | 101 | ported | `quadlet.rs` | `docker_daemon_prefix_stripped` | — |
| does not extract an image file reference | 119 | ported | `quadlet.rs` | `image_file_reference_skipped` | — |
| does not extract an build file reference | 129 | ported | `quadlet.rs` | `build_file_reference_skipped` | — |
| extract data from file with registry aliases | 139 | pending | — | — | registryAliases not yet implemented |
| handles an unsuccessful parse | 158 | ported | `quadlet.rs` | `container_section_without_image_returns_empty` | — |

---

## `lib/modules/manager/jenkins/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/jenkins/extract.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty list for an empty text file | 15 | ported | `jenkins.rs` | `txt_empty_file_returns_empty` | — |
| returns empty list for an empty yaml file | 21 | ported | `jenkins.rs` | `yml_empty_returns_empty` | — |
| returns empty list for an invalid yaml file | 27 | ported | `jenkins.rs` | `yml_invalid_yaml_returns_empty` | — |
| extracts multiple image lines in text format | 33 | ported | `jenkins.rs` | `txt_plugins_fixture_six_deps` | — |
| extracts multiple image lines in yaml format | 40 | ported | `jenkins.rs` | `yml_plugins_fixture_eight_deps` | — |

---

## `lib/modules/manager/mint/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/mint/extract.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 6 | ported | `mint.rs` | `empty_returns_empty` | — |
| Mintfile With Version Description | 10 | ported | `mint.rs` | `extracts_deps_with_version` | — |
| Mintfile Without Version Description | 41 | ported | `mint.rs` | `extracts_deps_without_version_as_skipped` | — |
| Complex Mintfile | 61 | ported | `mint.rs` | `complex_mintfile_mixed` | — |
| Mintfile Includes Commented Out | 86 | ported | `mint.rs` | `comment_lines_skipped` | — |

---

## `lib/modules/manager/ocb/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/ocb/extract.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `extractPackageFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| run successfully with full example | 6 | ported | `ocb.rs` | `extracts_full_example` | — |
| return null for unknown content | 81 | ported | `ocb.rs` | `skips_unknown_content` | — |
| return null for content which is not YAML | 85 | ported | `ocb.rs` | `skips_arbitrary_yaml` | — |

---

## `lib/modules/manager/pip_setup/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pip_setup/extract.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns found deps | 12 | ported | `pip_setup.rs` | `extracts_install_requires` | — |
| returns nothing | 41 | ported | `pip_setup.rs` | `no_requires_returns_empty` | — |

---

## `lib/modules/manager/terragrunt-version/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/terragrunt-version/extract.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns a result | 5 | ported | `version_file.rs` | `terragrunt_version_file` | — |

---

## Managers (`lib/modules/manager/`) — legacy summary

### Extract specs

| Renovate spec file | Renovate tests | Rust file | Rust tests | Status |
|--------------------|---------------|-----------|------------|--------|
<!-- ant/extract.spec.ts converted to per-test format above -->
<!-- asdf/extract.spec.ts converted to per-test format above -->
<!-- azure-pipelines/extract.spec.ts converted to per-test format above -->
<!-- bazel-module/extract.spec.ts converted to per-test format above -->
<!-- bazel/extract.spec.ts converted to per-test format above -->
<!-- bicep/extract.spec.ts converted to per-test format above -->
<!-- cargo/extract.spec.ts converted to per-test format above -->
<!-- cpanfile/extract.spec.ts converted to per-test format above -->
<!-- flux/extract.spec.ts converted to per-test format above -->
<!-- github-actions/extract.spec.ts converted to per-test format above -->
<!-- gitlabci converted to per-test format above -->
<!-- gradle/extract.spec.ts converted to per-test format above -->
<!-- helm-requirements/extract.spec.ts converted to per-test format above -->
<!-- helmfile/extract.spec.ts converted to per-test format above -->
<!-- homeassistant-manifest/extract.spec.ts converted to per-test format above -->
<!-- homebrew/extract.spec.ts converted to per-test format above -->
<!-- html/extract.spec.ts converted to per-test format above -->
<!-- jenkins/extract.spec.ts converted to per-test format above -->
<!-- jsonnet-bundler/extract.spec.ts converted to per-test format above -->
<!-- kotlin-script/extract.spec.ts converted to per-test format above -->
<!-- kubernetes/extract.spec.ts converted to per-test format above -->
<!-- kustomize/extract.spec.ts converted to per-test format above -->
<!-- leiningen/extract.spec.ts converted to per-test format above -->
<!-- maven-wrapper/extract.spec.ts converted to per-test format above -->
<!-- maven/extract.spec.ts converted to per-test format above -->
<!-- meteor/extract.spec.ts converted to per-test format above -->
<!-- mint/extract.spec.ts converted to per-test format above -->
<!-- mise/extract.spec.ts converted to per-test format above -->
<!-- mix/extract.spec.ts converted to per-test format above -->
<!-- nix/extract.spec.ts converted to per-test format above -->
<!-- nuget/extract.spec.ts converted to per-test format above -->
<!-- ocb/extract.spec.ts converted to per-test format above -->
<!-- osgi/extract.spec.ts converted to per-test format above -->
<!-- pep621/extract.spec.ts converted to per-test format above -->
<!-- pep723/extract.spec.ts converted to per-test format above -->
<!-- pip-compile/extract.spec.ts converted to per-test format above -->
<!-- pip_requirements/extract.spec.ts converted to per-test format above -->
<!-- pip_setup/extract.spec.ts converted to per-test format above -->
<!-- pipenv/extract.spec.ts converted to per-test format above -->
<!-- pixi/extract.spec.ts converted to per-test format above -->
<!-- poetry/extract.spec.ts converted to per-test format above -->
<!-- pre-commit/extract.spec.ts converted to per-test format above -->
<!-- puppet/extract.spec.ts converted to per-test format above -->
<!-- quadlet/extract.spec.ts converted to per-test format above -->
<!-- runtime-version/extract.spec.ts converted to per-test format above -->
<!-- sbt/extract.spec.ts converted to per-test format above -->
<!-- scalafmt/extract.spec.ts converted to per-test format above -->
<!-- setup-cfg/extract.spec.ts converted to per-test format above -->
<!-- sveltos/extract.spec.ts converted to per-test format above -->
<!-- tekton/extract.spec.ts converted to per-test format above -->
<!-- terraform/extract.spec.ts converted to per-test format above -->
<!-- terragrunt/extract.spec.ts converted to per-test format above -->
<!-- tflint-plugin/extract.spec.ts converted to per-test format above -->
<!-- travis/extract.spec.ts converted to per-test format above -->
<!-- typst/extract.spec.ts converted to per-test format above -->
<!-- unity3d/extract.spec.ts converted to per-test format above -->
<!-- velaci/extract.spec.ts converted to per-test format above -->
<!-- vendir/extract.spec.ts converted to per-test format above -->
<!-- woodpecker/extract.spec.ts converted to per-test format above -->
<!-- xcodegen/extract.spec.ts converted to per-test format above -->

### Other manager specs (non-extract)

| Renovate spec file | Renovate tests | Rust file | Rust tests | Status |
|--------------------|---------------|-----------|------------|--------|
| `lib/modules/manager/asdf/index.spec.ts` | — | `crates/renovate-core/src/extractors/asdf.rs` | — | partial |
| `lib/modules/manager/ant/properties.spec.ts` | — | `crates/renovate-core/src/extractors/ant.rs` | — | partial |
| `lib/modules/manager/ant/update.spec.ts` | — | — | — | pending |
| `lib/modules/manager/bazel-module/bazelrc.spec.ts` | — | — | — | pending |
| `lib/modules/manager/bazel-module/lockfile.spec.ts` | — | — | — | pending |
| `lib/modules/manager/bazel-module/parser/context.spec.ts` | — | `crates/renovate-core/src/extractors/bazel_module.rs` | — | partial |
| `lib/modules/manager/bazel-module/parser/fragments.spec.ts` | — | `crates/renovate-core/src/extractors/bazel_module.rs` | — | partial |
| `lib/modules/manager/batect-wrapper/artifacts.spec.ts` | — | — | — | pending |
| `lib/modules/manager/git-submodules/artifact.spec.ts` | — | — | — | pending |
| `lib/modules/manager/github-actions/integration.spec.ts` | — | `crates/renovate-core/src/extractors/github_actions.rs` | — | partial |
| `lib/modules/manager/github-actions/parse.spec.ts` | — | `crates/renovate-core/src/extractors/github_actions.rs` | — | partial |
| `lib/modules/manager/helmv3/common.spec.ts` | — | — | — | pending |
| `lib/modules/manager/npm/extract/index.spec.ts` | — | `crates/renovate-core/src/extractors/npm.rs` | — | partial |
| `lib/modules/manager/npm/extract/npm.spec.ts` | — | `crates/renovate-core/src/extractors/npm.rs` | — | partial |
| `lib/modules/manager/npm/extract/pnpm.spec.ts` | — | `crates/renovate-core/src/extractors/npm.rs` | — | partial |
| `lib/modules/manager/npm/extract/yarn.spec.ts` | — | `crates/renovate-core/src/extractors/npm.rs` | — | partial |
| `lib/modules/manager/npm/extract/yarnrc.spec.ts` | — | — | — | pending |
<!-- ruby-version/extract.spec.ts converted to per-test format above -->
<!-- nvm/extract.spec.ts, terraform-version/extract.spec.ts, terragrunt-version/extract.spec.ts also covered in per-test sections above (all use version_file.rs) -->

---

## Config specs (`lib/config/`)

| Renovate spec file | Renovate tests | Rust file | Rust tests | Status |
|--------------------|---------------|-----------|------------|--------|
| `lib/config/defaults.spec.ts` | 2 | `crates/renovate-core/src/config.rs` | 0 | pending |
| `lib/config/app-strings.spec.ts` | 3 | `crates/renovate-core/src/config.rs` | 0 | pending |
| `lib/config/parse.spec.ts` | 4 | `crates/renovate-cli/src/config_builder.rs` | 0 | pending |
| `lib/config/global.spec.ts` | 1 | `crates/renovate-core/src/config/run.rs` | 0 | pending |
| `lib/config/validation.spec.ts` | 124 | — | 0 | pending |
| `lib/config/migration.spec.ts` | 30 | — | 0 | pending |
| `lib/config/migrate-validate.spec.ts` | 5 | — | 0 | pending |
| `lib/config/massage.spec.ts` | 7 | — | 0 | pending |
| `lib/config/secrets.spec.ts` | 11 | — | 0 | pending |
| `lib/config/inherit.spec.ts` | 3 | — | 0 | pending |
| `lib/config/index.spec.ts` | 12 | — | 0 | pending |
| `lib/config/decrypt.spec.ts` | 12 | — | 0 | not-applicable |

---

## Workers specs

| Renovate spec file | Renovate tests | Rust file | Rust tests | Status |
|--------------------|---------------|-----------|------------|--------|
| `lib/workers/global/config/parse/cli.spec.ts` | 29 | `crates/renovate-cli/src/cli.rs` | 0 | pending |
| `lib/workers/global/config/parse/env.spec.ts` | — | `crates/renovate-cli/src/config_builder.rs` | 0 | pending |
| `lib/workers/global/config/parse/file.spec.ts` | — | `crates/renovate-cli/src/config_builder.rs` | 0 | pending |
| `lib/workers/repository/init/merge.spec.ts` | 37 | `crates/renovate-core/src/repo_config.rs` | 0 | pending |
| `lib/workers/repository/init/apis.spec.ts` | — | — | 0 | pending |
| `lib/workers/repository/init/cache.spec.ts` | — | — | 0 | pending |

---

## Utility specs (`lib/util/`)

| Renovate spec file | Renovate tests | Rust file | Rust tests | Status |
|--------------------|---------------|-----------|------------|--------|
| `lib/util/string-match.spec.ts` | 25 | `crates/renovate-core/src/string_match.rs` | 38 | partial |
| `lib/util/package-rules/index.spec.ts` | 73 | `crates/renovate-core/src/package_rule.rs` | 0 | pending |
| `lib/util/package-rules/managers.spec.ts` | 5 | `crates/renovate-core/src/package_rule.rs` | 0 | pending |
| `lib/util/package-rules/dep-names.spec.ts` | 4 | `crates/renovate-core/src/package_rule.rs` | 0 | pending |
| `lib/util/package-rules/current-age.spec.ts` | — | `crates/renovate-core/src/package_rule.rs` | 0 | pending |
| `lib/util/package-rules/current-value.spec.ts` | — | `crates/renovate-core/src/package_rule.rs` | 0 | pending |
| `lib/util/package-rules/current-version.spec.ts` | — | `crates/renovate-core/src/package_rule.rs` | 0 | pending |
| `lib/util/package-rules/files.spec.ts` | — | `crates/renovate-core/src/package_rule.rs` | 0 | pending |
| `lib/util/package-rules/new-value.spec.ts` | — | `crates/renovate-core/src/package_rule.rs` | 0 | pending |
| `lib/util/package-rules/package-names.spec.ts` | — | `crates/renovate-core/src/package_rule.rs` | 0 | pending |
| `lib/util/package-rules/repositories.spec.ts` | — | `crates/renovate-core/src/package_rule.rs` | 0 | pending |
| `lib/util/package-rules/jsonata.spec.ts` | — | — | 0 | pending |
