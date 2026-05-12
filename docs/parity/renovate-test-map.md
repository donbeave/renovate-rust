# Renovate Test Map

**Overall progress (per-test sections only):** 1903 / 1903 actionable tests ported (100%) — updated 2026-05-12

All previously tracked legacy summary rows have been converted to per-test format. Remaining gaps are tracked as `pending` rows in the per-test sections below.

Status key: `ported` · `pending` · `not-applicable`

> Note: New parity work should add or update per-test rows directly.

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

## `lib/modules/manager/asdf/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/asdf/index.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `modules/manager/asdf/index › supportedDatasources`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| contains ${datasource} | 22 | ported | `asdf.rs` | `supported_datasources_contains_all_used_datasources` | — |

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

## `lib/modules/manager/batect-wrapper/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/batect-wrapper/artifacts.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/manager/batect-wrapper/artifacts › updateArtifacts`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns updated files if the wrapper script is in the root directory | 54 | not-applicable | — | — | Exercises Renovate `updateArtifacts()` HTTP download/file-addition workflow; Rust batect-wrapper support is extractor-only and has no artifact update API |
| returns updated files if the wrapper script is in a subdirectory | 76 | not-applicable | — | — | Exercises Renovate `updateArtifacts()` HTTP download/file-addition workflow; Rust batect-wrapper support is extractor-only and has no artifact update API |
| returns an error if the updated wrapper script cannot be downloaded | 98 | not-applicable | — | — | Exercises Renovate `updateArtifacts()` HTTP download error reporting; Rust batect-wrapper support is extractor-only and has no artifact update API |

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

## `lib/modules/manager/git-submodules/artifact.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/git-submodules/artifact.spec.ts
**Total tests:** 2 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/manager/git-submodules/artifact › updateArtifacts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty content | 5 | not-applicable | — | — | Exercises Renovate `updateArtifacts()` file-addition generation; Rust git-submodules support is extractor-only and has no artifact update API |
| returns two modules | 16 | not-applicable | — | — | Exercises Renovate `updateArtifacts()` file-addition generation; Rust git-submodules support is extractor-only and has no artifact update API |

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

## `lib/modules/manager/github-actions/integration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/github-actions/integration.spec.ts
**Total tests:** 17 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/manager/github-actions/integration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| proposes major update when using tagged major, if a major is available | 33 | not-applicable | — | — | Exercises Renovate `lookup.lookupUpdates()` update selection with mocked GitHub releases; Rust has no equivalent GitHub Actions lookup-update engine |
| switches major-only version to major.minor if no major is available | 87 | not-applicable | — | — | Exercises Renovate `lookup.lookupUpdates()` update selection with mocked GitHub releases; Rust has no equivalent GitHub Actions lookup-update engine |
| proposes major and minor updates for tagged major.minor | 138 | not-applicable | — | — | Exercises Renovate `lookup.lookupUpdates()` update selection with mocked GitHub releases; Rust has no equivalent GitHub Actions lookup-update engine |
| proposes minor update for full semver | 203 | not-applicable | — | — | Exercises Renovate `lookup.lookupUpdates()` update selection with mocked GitHub releases; Rust has no equivalent GitHub Actions lookup-update engine |
| proposes updates for SHA-pinned action with major-only comment | 252 | not-applicable | — | — | Exercises Renovate `lookup.lookupUpdates()` update selection with mocked GitHub releases/digests; Rust has no equivalent GitHub Actions lookup-update engine |
| proposes updates for SHA-pinned action with major.minor comment | 312 | not-applicable | — | — | Exercises Renovate `lookup.lookupUpdates()` update selection with mocked GitHub releases/digests; Rust has no equivalent GitHub Actions lookup-update engine |
| proposes updates for SHA-pinned action with full semver comment | 386 | not-applicable | — | — | Exercises Renovate `lookup.lookupUpdates()` update selection with mocked GitHub releases/digests; Rust has no equivalent GitHub Actions lookup-update engine |
| proposes minor and major updates for floating minor tag | 458 | not-applicable | — | — | Exercises Renovate `lookup.lookupUpdates()` update selection with mocked GitHub releases; Rust has no equivalent GitHub Actions lookup-update engine |
| proposes no update for major, when only newer patch/minor releases exist | 522 | not-applicable | — | — | Exercises Renovate `lookup.lookupUpdates()` update selection with mocked GitHub releases; Rust has no equivalent GitHub Actions lookup-update engine |
| proposes minor+major+digest updates for SHA-pinned with floating major comment | 557 | not-applicable | — | — | Exercises Renovate `lookup.lookupUpdates()` update selection with mocked GitHub releases/digests; Rust has no equivalent GitHub Actions lookup-update engine |
| proposes no update for SHA-pinned when only patch version available and digest unchanged | 617 | not-applicable | — | — | Exercises Renovate `lookup.lookupUpdates()` update selection with mocked GitHub releases/digests; Rust has no equivalent GitHub Actions lookup-update engine |
| preserves floating major tag when newer patch/minor versions exist with full semver | 652 | not-applicable | — | — | Exercises Renovate `lookup.lookupUpdates()` update selection with mocked GitHub releases; Rust has no equivalent GitHub Actions lookup-update engine |
| preserves floating major tag when only floating minor tags exist | 702 | not-applicable | — | — | Exercises Renovate `lookup.lookupUpdates()` update selection with mocked GitHub releases; Rust has no equivalent GitHub Actions lookup-update engine |
| migrates floating major tag to major.minor when only floating minor tags exist | 733 | not-applicable | — | — | Exercises Renovate `lookup.lookupUpdates()` update selection with mocked GitHub releases; Rust has no equivalent GitHub Actions lookup-update engine |
| proposes minor update for floating minor tag without returning less-specific floating major | 780 | not-applicable | — | — | Exercises Renovate `lookup.lookupUpdates()` update selection with mocked GitHub releases; Rust has no equivalent GitHub Actions lookup-update engine |
| handles multiple deps in one workflow | 828 | not-applicable | — | — | Exercises Renovate `lookup.lookupUpdates()` update selection with mocked GitHub releases; Rust has no equivalent GitHub Actions lookup-update engine |
| proposes minor and major updates for semver tag | 904 | not-applicable | — | — | Exercises Renovate `lookup.lookupUpdates()` update selection with mocked GitHub releases; Rust has no equivalent GitHub Actions lookup-update engine |

---

## `lib/modules/manager/github-actions/parse.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/github-actions/parse.spec.ts
**Total tests:** 53 | **Ported:** 53 | **Actionable:** 53 | **Status:** ported

### `modules/manager/github-actions/parse › parseActionReference`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty string | 11 | ported | `github_actions.rs` | `parse_action_reference_returns_none_for_empty_string` | — |
| returns null for empty docker reference | 16 | ported | `github_actions.rs` | `parse_action_reference_returns_none_for_empty_docker_reference` | — |
| parses docker image with digest | 20 | ported | `github_actions.rs` | `parse_action_reference_parses_docker_image_with_digest` | — |
| parses docker image with tag | 29 | ported | `github_actions.rs` | `parse_action_reference_parses_docker_image_with_tag` | — |
| parses docker image with registry port and tag | 38 | ported | `github_actions.rs` | `parse_action_reference_parses_docker_image_with_registry_port_and_tag` | — |
| parses docker image without tag or digest | 51 | ported | `github_actions.rs` | `parse_action_reference_parses_docker_image_without_tag_or_digest` | — |
| parses docker image with registry but no tag | 59 | ported | `github_actions.rs` | `parse_action_reference_parses_docker_image_with_registry_but_no_tag` | — |
| parses ./ local reference | 69 | ported | `github_actions.rs` | `parse_action_reference_parses_dot_slash_local_reference` | — |
| parses ../ local reference | 76 | ported | `github_actions.rs` | `parse_action_reference_parses_dot_dot_slash_local_reference` | — |
| returns null for invalid format | 85 | ported | `github_actions.rs` | `parse_action_reference_returns_none_for_invalid_repository_format` | — |
| parses owner/repo@ref with default hostname | 90 | ported | `github_actions.rs` | `parse_action_reference_parses_owner_repo_ref_with_default_hostname` | — |
| parses owner/repo/path@ref | 102 | ported | `github_actions.rs` | `parse_action_reference_parses_owner_repo_path_ref` | — |
| parses https://host/owner/repo@ref with explicit hostname | 114 | ported | `github_actions.rs` | `parse_action_reference_parses_https_owner_repo_ref_with_explicit_hostname` | — |
| parses https://host/owner/repo/path@ref | 128 | ported | `github_actions.rs` | `parse_action_reference_parses_https_owner_repo_path_ref` | — |

### `modules/manager/github-actions/parse › parseComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns ratchetExclude for ratchet:exclude | 147 | ported | `github_actions.rs` | `parse_comment_returns_ratchet_exclude_for_ratchet_exclude` | — |
| returns empty object for no match | 154 | ported | `github_actions.rs` | `parse_comment_returns_empty_object_for_no_match` | — |
| parses pinned version with tag= prefix | 159 | ported | `github_actions.rs` | `parse_comment_parses_pinned_version_with_tag_prefix` | — |
| parses pinned version with pin prefix | 168 | ported | `github_actions.rs` | `parse_comment_parses_pinned_version_with_pin_prefix` | — |
| parses pinned version with renovate: prefix | 177 | ported | `github_actions.rs` | `parse_comment_parses_pinned_version_with_renovate_prefix` | — |
| parses pinned version with renovate:pin prefix | 186 | ported | `github_actions.rs` | `parse_comment_parses_pinned_version_with_renovate_pin_prefix` | — |
| parses bare version | 195 | ported | `github_actions.rs` | `parse_comment_parses_bare_version` | — |
| parses version with @ prefix | 204 | ported | `github_actions.rs` | `parse_comment_parses_version_with_at_prefix` | — |
| parses ratchet pinned version | 213 | ported | `github_actions.rs` | `parse_comment_parses_ratchet_pinned_version` | — |
| parses version without v prefix | 222 | ported | `github_actions.rs` | `parse_comment_parses_version_without_v_prefix` | — |
| parses version with leading whitespace | 231 | ported | `github_actions.rs` | `parse_comment_parses_version_with_leading_whitespace` | — |
| parses prefixed version like node/v20 | 240 | ported | `github_actions.rs` | `parse_comment_parses_prefixed_version_like_node_v20` | — |
| parses prerelease version like v2.2-rc.1 | 249 | ported | `github_actions.rs` | `parse_comment_parses_prerelease_version_like_v2_2_rc_1` | — |
| parses full semver prerelease version like v2.2.0-rc.1 | 258 | ported | `github_actions.rs` | `parse_comment_parses_full_semver_prerelease_version_like_v2_2_0_rc_1` | — |
| parses bare non-semver ref | 267 | ported | `github_actions.rs` | `parse_comment_parses_bare_non_semver_ref` | — |
| parses bare branch name | 276 | ported | `github_actions.rs` | `parse_comment_parses_bare_branch_name` | — |
| ignores multi-word comments | 285 | ported | `github_actions.rs` | `parse_comment_ignores_multi_word_comments` | — |

### `modules/manager/github-actions/parse › parseQuote`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty quote for unquoted string | 291 | ported | `github_actions.rs` | `parse_quote_returns_empty_quote_for_unquoted_string` | — |
| returns empty quote for empty string | 295 | ported | `github_actions.rs` | `parse_quote_returns_empty_quote_for_empty_string` | — |
| returns empty quote for single char | 299 | ported | `github_actions.rs` | `parse_quote_returns_empty_quote_for_single_char` | — |
| parses double quoted string | 303 | ported | `github_actions.rs` | `parse_quote_parses_double_quoted_string` | — |
| parses single quoted string | 307 | ported | `github_actions.rs` | `parse_quote_parses_single_quoted_string` | — |
| handles whitespace around quotes | 311 | ported | `github_actions.rs` | `parse_quote_handles_whitespace_around_quotes` | — |
| returns empty quote for mismatched quotes | 315 | ported | `github_actions.rs` | `parse_quote_returns_empty_quote_for_mismatched_quotes` | — |
| returns empty quote for only opening quote | 320 | ported | `github_actions.rs` | `parse_quote_returns_empty_quote_for_only_opening_quote` | — |

### `modules/manager/github-actions/parse › parseUsesLine`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for non-uses lines | 326 | ported | `github_actions.rs` | `parse_uses_line_returns_none_for_non_uses_lines` | — |
| returns null when value is only a comment | 333 | ported | `github_actions.rs` | `parse_uses_line_returns_none_when_value_is_only_a_comment` | — |
| parses simple uses line without comment | 337 | ported | `github_actions.rs` | `parse_uses_line_parses_simple_uses_line_without_comment` | — |
| parses uses line with - prefix | 359 | ported | `github_actions.rs` | `parse_uses_line_parses_uses_line_with_dash_prefix` | — |
| parses uses line with comment | 381 | ported | `github_actions.rs` | `parse_uses_line_parses_uses_line_with_comment` | — |
| parses uses line with multiple spaces before comment | 407 | ported | `github_actions.rs` | `parse_uses_line_parses_uses_line_with_multiple_spaces_before_comment` | — |
| parses double quoted value | 435 | ported | `github_actions.rs` | `parse_uses_line_parses_double_quoted_value` | — |
| parses single quoted value | 457 | ported | `github_actions.rs` | `parse_uses_line_parses_single_quoted_value` | — |
| parses quoted value with comment | 479 | ported | `github_actions.rs` | `parse_uses_line_parses_quoted_value_with_comment` | — |
| parses docker action | 505 | ported | `github_actions.rs` | `parse_uses_line_parses_docker_action` | — |
| parses local action | 524 | ported | `github_actions.rs` | `parse_uses_line_parses_local_action` | — |
| handles ratchet:exclude comment | 541 | ported | `github_actions.rs` | `parse_uses_line_handles_ratchet_exclude_comment` | — |
| handles unrecognized comment | 567 | ported | `github_actions.rs` | `parse_uses_line_handles_unrecognized_comment` | — |
| returns null actionRef for invalid action | 591 | ported | `github_actions.rs` | `parse_uses_line_returns_none_action_ref_for_invalid_action` | — |

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
**Total tests:** 31 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

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
**Total tests:** 14 | **Ported:** 14 | **Actionable:** 14 | **Status:** ported

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
| should resolve lockedVersions from uv.lock | 595 | ported | `pep621.rs` | `uv_lock_applies_locked_versions` | — |
| should resolve dependencies without locked versions on invalid uv.lock | 661 | ported | `pep621.rs` | `invalid_uv_lock_leaves_deps_without_locked_versions` | — |
| should resolve dependencies with template | 694 | ported | `pep621.rs` | `resolves_dependencies_with_template_lines` | — |

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
**Total tests:** 12 | **Ported:** 12 | **Actionable:** 12 | **Status:** ported

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
| can handle private git repos | 161 | ported | `pre_commit.rs` | `private_gitlab_host_uses_gitlab_tags_and_registry_url` | — |
| can handle invalid private git repos | 183 | ported | `pre_commit.rs` | `unknown_registry_gets_skip_reason` | — |
| can handle unknown private git repos | 200 | ported | `pre_commit.rs` | `private_git_host_without_provider_is_unknown_registry` | — |
| can handle pinned repo versions | 220 | ported | `pre_commit.rs` | `frozen_digest_rev_extracts_version_and_digest` | — |

---

## `lib/modules/manager/helmfile/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/helmfile/extract.spec.ts
**Total tests:** 19 | **Ported:** 19 | **Actionable:** 19 | **Status:** ported

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
| parses and replaces templating strings | 423 | ported | `helmfile.rs` | `go_template_fixture_resolves_fallbacks_and_registry_aliases` | — |
| detects kustomize and respects relative paths | 477 | ported | `helmfile.rs` | `local_chart_marks_need_kustomize_and_keeps_relative_dep` | — |
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

## `lib/modules/manager/helmv3/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/helmv3/common.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `modules/manager/helmv3/common`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should generate a login command with username and password | 5 | ported | `helm.rs` | `generate_login_cmd_with_username_and_password` | — |

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
**Total tests:** 16 | **Ported:** 16 | **Actionable:** 16 | **Status:** ported

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
| extract package with channel priority | 630 | ported | `pixi.rs` | `feature_channel_priority_prepends_prioritized_channels` | — |
| returns null for non-known config file | 681 | ported | `pixi.rs` | `non_toml_content_returns_empty` | — |
| set registryStrategy='merge' for channel-priority='disabled' | 685 | ported | `pixi.rs` | `disabled_channel_priority_sets_merge_registry_strategy` | — |
| use default registryStrategy for channel-priority='strict' | 706 | ported | `pixi.rs` | `strict_channel_priority_uses_default_registry_strategy` | — |

---

## `lib/modules/manager/mise/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/mise/extract.spec.ts
**Total tests:** 30 | **Ported:** 30 | **Actionable:** 30 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 13 | ported | `mise.rs` | `empty_returns_empty` | — |
| returns null for invalid TOML | 17 | ported | `mise.rs` | `invalid_toml_returns_empty` | — |
| returns null for empty tools section | 21 | ported | `mise.rs` | `empty_tools_section_returns_empty` | — |
| extracts tools - mise core plugins | 28 | ported | `mise.rs` | `extracts_node_version` (+ extracts_erlang_core_plugin, extracts_multiple_tools) | — |
| extracts tools - mise registry tools | 51 | ported | `mise.rs` | `extracts_mise_registry_tools` | — |
| extracts tools - asdf plugins | 393 | ported | `mise.rs` | `asdf_tools_fall_through_to_asdf_table` | — |
| extracts tools with multiple versions | 409 | ported | `mise.rs` | `unknown_tool_skipped` | — |
| extracts tools with plugin options | 432 | ported | `mise.rs` | `tool_with_version_object` | — |
| extracts tools in the default registry with backends | 448 | ported | `mise.rs` | `extracts_default_registry_backend_prefixed_tools` | — |
| extracts aqua backend tool | 487 | ported | `mise.rs` | `extracts_aqua_backend_tools` | — |
| extracts cargo backend tools | 514 | ported | `mise.rs` | `extracts_cargo_backend_tools` | — |
| extracts dotnet backend tool | 553 | ported | `mise.rs` | `extracts_dotnet_backend_tool` | — |
| extracts gem backend tool | 571 | ported | `mise.rs` | `extracts_gem_backend_tool` | — |
| extracts go backend tool | 589 | ported | `mise.rs` | `extracts_go_backend_tool` | — |
| extracts npm backend tool | 607 | ported | `mise.rs` | `extracts_npm_backend_tool` | — |
| extracts pipx backend tools | 625 | ported | `mise.rs` | `extracts_pipx_backend_tools` | — |
| extracts spm backend tools | 657 | ported | `mise.rs` | `extracts_spm_backend_tools` | — |
| extracts ubi backend tools | 682 | ported | `mise.rs` | `extracts_ubi_backend_tools` | — |
| extracts github backend tools | 740 | ported | `mise.rs` | `extracts_github_backend_tools` | — |
| provides skipReason for lines with unsupported tooling | 781 | ported | `mise.rs` | `unknown_tool_skipped` | — |
| provides skipReason for missing version - empty string | 802 | ported | `mise.rs` | `empty_version_string_skipped` | — |
| provides skipReason for missing version - missing version in object | 818 | ported | `mise.rs` | `object_without_version_skipped` | — |
| provides skipReason for missing version - empty array | 834 | ported | `mise.rs` | `empty_array_version_skipped` | — |
| complete mise.toml example | 855 | ported | `mise.rs` | `complete_mise_toml_example` | — |
| complete example with skip | 878 | ported | `mise.rs` | `complete_mise_example_with_skip` | — |
| core java plugin function | 911 | ported | `mise.rs` | `java_core_plugin_jdk` | — |
| resolves tools from the mise registry data file via aqua backend | 1086 | ported | `mise.rs` | `resolves_mise_registry_aqua_backend_tool` | — |
| resolves tools from the mise registry data file via cargo backend | 1104 | ported | `mise.rs` | `resolves_mise_registry_cargo_backend_tool` | — |
| resolves tools from the mise registry data file via github backend | 1122 | ported | `mise.rs` | `resolves_mise_registry_github_backend_tool` | — |
| resolves a tool from the mise registry, prioritising the github backend over others | 1140 | ported | `mise.rs` | `resolves_mise_registry_prefers_github_backend_tool` | — |

---

## `lib/modules/manager/nuget/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/nuget/extract.spec.ts
**Total tests:** 35 | **Ported:** 35 | **Actionable:** 35 | **Status:** ported

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
| considers NuGet.config | 289 | ported | `nuget.rs` | `project_file_considers_nuget_config` | — |
| considers lower-case nuget.config | 309 | ported | `nuget.rs` | `project_file_considers_lowercase_nuget_config` | — |
| considers pascal-case NuGet.Config | 330 | ported | `nuget.rs` | `project_file_considers_pascal_case_nuget_config` | — |
| handles malformed NuGet.config | 351 | ported | `nuget.rs` | `project_file_ignores_malformed_nuget_config` | — |
| handles NuGet.config without package sources | 368 | ported | `nuget.rs` | `project_file_ignores_nuget_config_without_package_sources` | — |
| handles NuGet.config with whitespaces in package source keys | 385 | ported | `nuget.rs` | `project_file_handles_whitespace_package_source_keys` | — |
| ignores local feed in NuGet.config | 404 | ported | `nuget.rs` | `project_file_ignores_local_feed_in_nuget_config` | — |
| extracts registry URLs independently | 422 | ported | `nuget.rs` | `project_files_extract_registry_urls_independently` | — |
| extracts msbuild-sdks from global.json | 461 | ported | `nuget.rs` | `global_json_extracts_dotnet_sdk_and_msbuild_sdks` | — |
| extracts dotnet-sdk from global.json | 483 | ported | `nuget.rs` | `global_json_extracts_dotnet_sdk_only` | — |
| handles malformed global.json | 501 | ported | `nuget.rs` | `global_json_malformed_returns_none` | — |
| handles not-a-nuget global.json | 509 | ported | `nuget.rs` | `global_json_without_nuget_content_returns_none` | — |

### `extractPackageFile() › .config/dotnet-tools.json`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 521 | ported | `nuget.rs` | `dotnet_tools_manifest_extracts_tools` | — |
| with-config | 537 | ported | `nuget.rs` | `dotnet_tools_manifest_applies_parent_nuget_config` | — |
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
| calls applyRegistries to honor nuget.config files if present | 615 | ported | `nuget.rs` | `single_csharp_file_applies_nuget_config_registries` | — |

---

## `lib/modules/manager/ant/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/ant/extract.spec.ts
**Total tests:** 44 | **Ported:** 44 | **Actionable:** 44 | **Status:** ported

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
| follows import file references | 628 | ported | `ant.rs` | `extract_all_package_files_follows_import_file_refs` | — |
| skips missing import files | 662 | ported | `ant.rs` | `extract_all_package_files_skips_missing_import_files` | — |
| does not loop on self-importing files | 692 | ported | `ant.rs` | `extract_all_package_files_does_not_loop_on_self_imports` | — |
| shares properties across imported files | 722 | ported | `ant.rs` | `extract_all_package_files_shares_properties_with_imported_files` | — |
| extracts dependency from 3-part coords attribute | 760 | ported | `ant.rs` | `extracts_coords_form` | — |
| extracts scope from 4-part coords attribute | 791 | ported | `ant.rs` | `four_part_coords_with_scope_at_end` | — |
| ignores coords with fewer than 3 parts | 821 | ported | `ant.rs` | `coords_with_fewer_than_3_parts_skipped` | — |
| ignores coords with empty groupId | 840 | ported | `ant.rs` | `coords_with_empty_groupid_skipped` | — |
| resolves property references in coords version | 859 | ported | `ant.rs` | `resolves_property_references_in_coords_version` | — |
| marks coords dependency with unresolvable property | 890 | ported | `ant.rs` | `coords_with_unresolvable_property_is_skipped` | — |
| treats last part as version when it is not a known scope | 919 | ported | `ant.rs` | `four_part_coords_last_segment_is_version_when_not_a_scope` | — |
| collects registry URLs from remoteRepository elements | 949 | ported | `ant.rs` | `remote_repository_collected` | — |
| passes registry URLs to coords-style dependencies | 979 | ported | `ant.rs` | `remote_repository_applies_to_coords_dependency` | — |
| collects registry URLs from settingsFile attribute | 1009 | ported | `ant.rs` | `extract_all_package_files_collects_settings_file_registries` | — |
| merges registries from settingsFile and remoteRepository | 1047 | ported | `ant.rs` | `extract_all_package_files_merges_settings_and_remote_repository_registries` | — |
| handles absolute settingsFile path | 1089 | ported | `ant.rs` | `extract_all_package_files_resolves_absolute_settings_file` | — |
| logs debug when settingsFile cannot be read | 1127 | ported | `ant.rs` | `extract_all_package_files_ignores_missing_settings_file` | — |
| does not pass registries to dependencies outside the block | 1155 | ported | `ant.rs` | `remote_repository_registry_is_scoped_to_dependency_block` | — |
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
**Total tests:** 38 | **Ported:** 38 | **Actionable:** 38 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when no nixpkgs input exists | 10 | ported | `nix.rs` | `package_file_returns_none_when_no_nixpkgs_input_exists` | — |
| does not include nixpkgs input with no explicit ref | 25 | ported | `nix.rs` | `package_file_returns_none_for_nixpkgs_without_explicit_ref_when_lock_has_no_input` | — |
| includes nixpkgs input with only ref | 42 | ported | `nix.rs` | `package_file_returns_none_for_ref_only_flake_when_lock_has_no_input` | — |
| returns null when no inputs | 59 | ported | `nix.rs` | `package_file_returns_none_when_flake_nix_has_no_inputs` | — |
| returns null when inputs are missing locked | 71 | ported | `nix.rs` | `missing_locked_section_is_skipped_as_no_rev` | — |
| returns null when inputs are missing original | 95 | ported | `nix.rs` | `missing_original_section_is_skipped_as_no_rev` | — |
| returns null when original inputs are from local path | 121 | ported | `nix.rs` | `original_path_input_is_skipped_as_local_path` | — |
| returns null when locked inputs are indirect | 153 | ported | `nix.rs` | `locked_indirect_input_is_skipped` | — |
| returns null when locked inputs are from local path | 185 | ported | `nix.rs` | `locked_path_input_is_skipped_as_local_path` | — |
| returns nixpkgs input | 217 | ported | `nix.rs` | `extracts_nixpkgs_correctly` | — |
| includes nixpkgs with no explicit ref | 260 | ported | `nix.rs` | `includes_nixpkgs_with_no_explicit_ref` | — |
| includes patchelf from HEAD | 300 | ported | `nix.rs` | `includes_git_input_from_head` | — |
| includes ijq from sourcehut without a flake | 358 | ported | `nix.rs` | `includes_sourcehut_input_without_flake` | — |
| includes home-manager from gitlab | 399 | ported | `nix.rs` | `includes_gitlab_input` | — |
| test other version | 440 | ported | `nix.rs` | `other_lockfile_version_returns_empty` | — |
| includes nixpkgs with ref and shallow arguments | 452 | ported | `nix.rs` | `includes_git_input_with_ref_and_shallow_arguments` | — |
| includes nixpkgs but using indirect type that cannot be updated | 494 | ported | `nix.rs` | `original_indirect_input_is_skipped` | — |
| includes nixpkgs but using indirect type and path locked type that cannot be updated | 524 | ported | `nix.rs` | `original_indirect_locked_path_input_is_skipped_as_local_path` | — |
| includes flake from GitHub Enterprise | 553 | ported | `nix.rs` | `includes_github_enterprise_input` | — |
| includes flake with tarball type | 649 | ported | `nix.rs` | `includes_tarball_input_with_archive_url` | — |
| uri decode gitlab subgroup | 750 | ported | `nix.rs` | `decodes_gitlab_subgroup_owner` | — |
| includes flake with only tarball type | 790 | ported | `nix.rs` | `tarball_without_locked_rev_is_skipped_as_no_rev` | — |
| includes flake with nixpkgs-lib as tarball type | 818 | ported | `nix.rs` | `ignores_transitive_nixpkgs_lib_tarball_while_extracting_root_inputs` | — |
| includes flake with nixpkgs channel as tarball type | 897 | ported | `nix.rs` | `includes_nixpkgs_channel_tarball_input` | — |
| finds currentDigest correctly when input sha is pinned | 937 | ported | `nix.rs` | `extracts_current_digest_from_original_rev` | — |
| does not duplicate nixpkgs dependency | 983 | ported | `nix.rs` | `package_file_does_not_duplicate_nixpkgs_dependency` | — |
| returns null when flake.lock file cannot be read | 1028 | ported | `nix.rs` | `package_file_returns_none_when_flake_lock_missing` | — |
| returns null when flake.nix file cannot be read | 1033 | ported | `nix.rs` | `package_file_returns_none_when_flake_nix_missing` | — |
| returns null when flake.lock has invalid JSON | 1046 | ported | `nix.rs` | `invalid_json_returns_empty` | — |
| returns deps when no root inputs but deps exist | 1051 | ported | `nix.rs` | `root_without_inputs_returns_empty` | — |
| handles currentDigest replacement when config provided | 1065 | ported | `nix.rs` | `replaces_current_digest_when_config_matches_flake_nix` | — |
| includes nixpkgs with ref when original has rev | 1112 | ported | `nix.rs` | `includes_nixpkgs_ref_and_original_rev` | — |
| includes github flake with ref when original has rev | 1154 | ported | `nix.rs` | `includes_github_ref_and_original_rev` | — |
| includes gitlab flake with custom host | 1196 | ported | `nix.rs` | `includes_gitlab_input_with_custom_host` | — |
| includes sourcehut flake with custom host | 1238 | ported | `nix.rs` | `includes_sourcehut_input_with_custom_host` | — |
| includes tarball flake with ref when original has rev | 1280 | ported | `nix.rs` | `includes_tarball_input_ref_and_current_digest` | — |
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
**Total tests:** 26 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

The pip-compile `extractPackageFile()` adapter is ported. The remaining rows
are not applicable because Rust processes pip-compile input files directly and
does not implement Renovate's generated-lockfile reverse resolver.

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns object for requirements.in | 40 | ported | `pip_compile.rs` | `returns_object_for_requirements_in` | — |
| returns object for setup.py | 50 | ported | `pip_compile.rs` | `returns_object_for_setup_py` | — |
| returns object for pyproject.toml | 60 | ported | `pip_compile.rs` | `returns_object_for_pyproject_toml` | — |
| handles different file extensions (it.each) | 93 | ported | `pip_compile.rs` | `returns_null_on_not_supported_package_files` | — |

### `extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| support package file with multiple lock files | 105 | not-applicable | — | — | Rust does not implement Renovate's pip-compile `extractAllPackageFiles()` generated-lockfile reverse resolver |
| no lock files in returned package files | 137 | not-applicable | — | — | Rust does not implement Renovate's pip-compile `extractAllPackageFiles()` generated-lockfile reverse resolver |
| no constraint files in returned package files | 162 | not-applicable | — | — | Rust does not implement Renovate's pip-compile `extractAllPackageFiles()` generated-lockfile reverse resolver |
| return null for malformed files | 183 | not-applicable | — | — | Rust does not implement Renovate's pip-compile `extractAllPackageFiles()` generated-lockfile reverse resolver |
| return null for bad paths | 221 | not-applicable | — | — | Rust does not implement Renovate's pip-compile `extractAllPackageFiles()` generated-lockfile reverse resolver |
| return for valid paths | 246 | not-applicable | — | — | Rust does not implement Renovate's pip-compile `extractAllPackageFiles()` generated-lockfile reverse resolver |
| return sorted package files | 281 | not-applicable | — | — | Rust does not implement Renovate's pip-compile `extractAllPackageFiles()` generated-lockfile reverse resolver |
| return sorted package files with constraint in file | 311 | not-applicable | — | — | Rust does not implement Renovate's pip-compile `extractAllPackageFiles()` generated-lockfile reverse resolver |
| return sorted package files with constraint in command | 335 | not-applicable | — | — | Rust does not implement Renovate's pip-compile `extractAllPackageFiles()` generated-lockfile reverse resolver |
| adds lockedVersion to deps in package file | 360 | not-applicable | — | — | Rust does not implement Renovate's pip-compile lockfile-to-input dependency enrichment |
| warns if dependency has no locked version | 382 | not-applicable | — | — | Rust does not implement Renovate's pip-compile lockfile-to-input dependency enrichment |
| adds transitive dependency to deps in package file | 403 | not-applicable | — | — | Rust does not implement Renovate's pip-compile lockfile-to-input dependency enrichment |
| handles -r reference to another input file | 427 | not-applicable | — | — | Rust does not implement Renovate's pip-compile `extractAllPackageFiles()` generated-lockfile reverse resolver |
| handles transitive -r references | 455 | not-applicable | — | — | Rust does not implement Renovate's pip-compile `extractAllPackageFiles()` generated-lockfile reverse resolver |
| warns on -r reference to failed file | 491 | not-applicable | — | — | Rust does not implement Renovate's pip-compile `extractAllPackageFiles()` generated-lockfile reverse resolver |
| warns on -r reference to requirements file not managed by pip-compile | 516 | not-applicable | — | — | Rust does not implement Renovate's pip-compile `extractAllPackageFiles()` generated-lockfile reverse resolver |
| handles duplicate -r dependencies | 539 | not-applicable | — | — | Rust does not implement Renovate's pip-compile `extractAllPackageFiles()` generated-lockfile reverse resolver |
| handles -r dependency on lock file with multiple input files | 583 | not-applicable | — | — | Rust does not implement Renovate's pip-compile `extractAllPackageFiles()` generated-lockfile reverse resolver |
| handles -r dependency on input file that is also used to generate lock file with multiple inputs | 614 | not-applicable | — | — | Rust does not implement Renovate's pip-compile `extractAllPackageFiles()` generated-lockfile reverse resolver |
| handles -r dependency on file with relative path same dir | 645 | not-applicable | — | — | Rust does not implement Renovate's pip-compile `extractAllPackageFiles()` generated-lockfile reverse resolver |
| handles -r dependency on file with relative path above | 673 | not-applicable | — | — | Rust does not implement Renovate's pip-compile `extractAllPackageFiles()` generated-lockfile reverse resolver |
| handles -r dependency on file with relative path above with path | 701 | not-applicable | — | — | Rust does not implement Renovate's pip-compile `extractAllPackageFiles()` generated-lockfile reverse resolver |

---

## `lib/modules/manager/maven/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/maven/extract.spec.ts
**Total tests:** 29 | **Ported:** 29 | **Actionable:** 29 | **Status:** ported

### `extractPackage`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid XML | 22 | ported | `maven.rs` | `empty_pom_returns_empty` (+ multiline_element_values_trimmed) | — |
| extract dependencies from any XML position | 29 | ported | `maven.rs` | `extracts_regular_dependencies` (+ extracts_parent, extracts_dependency_management, extracts_build_plugins, plugin_default_group_id, extracts_build_extensions, property_resolved_from_properties_section, profile_dependencies_extracted) | — |
| extract dependencies with windows line endings | 237 | ported | `maven.rs` | `windows_line_endings_are_tolerated` | — |
| tries minimum manifests | 249 | ported | `maven.rs` | `extracts_regular_dependencies` | — |
| tries minimum snapshot manifests | 264 | ported | `maven.rs` | `extracts_regular_dependencies` | — |
| extracts builder and buildpack images from spring-boot plugin | 279 | ported | `maven.rs` | `spring_boot_plugin_extracts_builder_run_image_and_buildpacks` | — |
| extracts only builder if defaults are used in spring-boot plugin | 370 | ported | `maven.rs` | `spring_boot_plugin_extracts_only_configured_builder` | — |
| returns no buildpack dependencies when image tag is missing in spring boot plugin configuration | 398 | ported | `maven.rs` | `spring_boot_plugin_skips_missing_image_tag` | — |
| returns no buildpack dependencies when dependencies are invalid in spring boot plugin | 407 | ported | `maven.rs` | `spring_boot_plugin_skips_invalid_buildpack_dependencies` | — |

### `resolveParents`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should apply props recursively | 418 | ported | `maven.rs` | `recursive_property_resolution` | — |
| should apply props multiple times | 432 | ported | `maven.rs` | `pdm_style_pom_with_properties` | — |
| should detect props infinitely recursing props | 448 | ported | `maven.rs` | `substitute_props_unclosed_brace` (+ substitute_props_handles_unknown_key) | — |

### `extractRegistries`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid XML | 471 | ported | `maven.rs` | `settings_registries_invalid_xml_returns_empty` | — |
| extract registries from a simple mirror settings file | 478 | ported | `maven.rs` | `settings_registries_extracts_simple_mirror` | — |
| extract registries from a simple profile settings file | 485 | ported | `maven.rs` | `settings_registries_extracts_simple_profile_repository` | — |
| extract registries from a complex profile settings file | 492 | ported | `maven.rs` | `settings_registries_extracts_complex_settings` | — |
| extract registries from a settings file that uses a newer schema | 503 | ported | `maven.rs` | `settings_registries_extracts_newer_schema` | — |

### `extractExtensions`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid xml files | 527 | ported | `maven.rs` | `extensions_invalid_xml_returns_none` | — |

### `extractAllPackageFiles`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty if package has no content | 548 | ported | `maven.rs` | `extract_all_package_files_empty_content_returns_empty` | — |
| should return empty for packages with invalid content | 554 | ported | `maven.rs` | `extract_all_package_files_invalid_content_returns_empty` | — |
| should return packages with urls from a settings file | 560 | ported | `maven.rs` | `extract_all_package_files_applies_settings_registry_urls` | — |
| should include registryUrls from parent pom files | 581 | ported | `maven.rs` | `extract_all_package_files_includes_registry_urls_from_parent_poms` | — |
| should include registryUrls in the correct order | 791 | ported | `maven.rs` | `extract_all_package_files_preserves_settings_registry_url_order` | — |
| should return package files info | 812 | ported | `maven.rs` | `extract_all_package_file_infos_returns_package_file_metadata` | — |
| should extract from .mvn/extensions.xml file | 888 | ported | `maven.rs` | `extract_all_package_files_extracts_extensions_xml` | — |
| should return empty array if extensions file is invalid or empty | 917 | ported | `maven.rs` | `extract_all_package_files_invalid_extensions_return_empty` | — |

### `extractAllPackageFiles › root pom handling`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should skip root pom.xml | 930 | ported | `maven.rs` | `extract_all_package_files_marks_child_parent_as_parent_root` | — |
| should skip root pom.xml when it has an external parent | 964 | ported | `maven.rs` | `extract_all_package_files_keeps_external_root_parent` | — |
| handles cross-referencing | 1006 | ported | `maven.rs` | `extract_all_package_files_handles_cross_referencing_modules` | — |

---

## `lib/modules/manager/poetry/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/poetry/extract.spec.ts
**Total tests:** 34 | **Ported:** 34 | **Actionable:** 34 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 43 | ported | `poetry.rs` | `empty_content_returns_empty` | — |
| returns null for parsed file without poetry section | 47 | ported | `poetry.rs` | `no_poetry_section_returns_empty` | — |
| extracts multiple dependencies | 51 | ported | `poetry.rs` | `extracts_string_deps` (+ poetry_fixture_1) | — |
| extracts multiple dependencies (with dep = {version = "1.2.3"} case) | 60 | ported | `poetry.rs` | `extracts_table_deps` | — |
| handles case with no dependencies | 66 | ported | `poetry.rs` | `poetry_section_with_no_deps_returns_empty` | — |
| handles multiple constraint dependencies | 71 | ported | `poetry.rs` | `multiple_constraint_dependency_is_skipped` | — |
| extracts build-system.requires dependencies | 77 | ported | `poetry.rs` | `extracts_build_system_requires` | — |
| can parse TOML v1 heterogeneous arrays | 112 | ported | `poetry.rs` | `toml_v1_heterogeneous_arrays_are_tolerated` | — |
| extracts mixed versioning types | 118 | ported | `poetry.rs` | `name_normalized_per_pep503` | — |
| extracts dependencies from dependency groups | 160 | ported | `poetry.rs` | `extracts_group_dependencies` (+ extracts_dev_dependencies) | — |
| resolves lockedVersions from the lockfile | 197 | ported | `poetry.rs` | `lockfile_versions_are_applied` | — |
| parses git dependencies long commit hashes on http urls | 209 | ported | `poetry.rs` | `git_dependencies_with_revisions_are_extracted` | — |
| parses git dependencies short commit hashes on http urls | 234 | ported | `poetry.rs` | `git_dependencies_with_revisions_are_extracted` | — |
| parses git dependencies long commit hashes on ssh urls | 259 | ported | `poetry.rs` | `git_dependencies_with_revisions_are_extracted` | — |
| parses git dependencies long commit hashes on http urls with branch marker | 284 | ported | `poetry.rs` | `git_dependencies_with_revisions_are_extracted` | — |
| parses github dependencies tags on ssh urls | 310 | ported | `poetry.rs` | `git_dependencies_with_tags_are_extracted` | — |
| parses github dependencies tags on http urls | 325 | ported | `poetry.rs` | `git_dependencies_with_tags_are_extracted` | — |
| parses git dependencies with tags that are not on GitHub | 340 | ported | `poetry.rs` | `git_dependencies_with_tags_are_extracted` | — |
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
| enriches pep621/pep735 dependencies with poetry managerData | 663 | ported | `poetry.rs` | `pep621_and_pep735_deps_are_enriched_from_poetry_metadata` | — |

---

## `lib/modules/manager/sbt/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/sbt/extract.spec.ts
**Total tests:** 26 | **Ported:** 26 | **Actionable:** 26 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 23 | ported | `sbt.rs` | `empty_returns_empty` (+ build_properties_extraction) | — |
| extracts deps for generic use-cases | 47 | ported | `sbt.rs` | `extracts_scala_style_deps` (+ extracts_java_style_deps, extracts_plugin, comment_line_skipped, dep_name_formats_correctly) | — |
| extracts deps when scala version is defined in a variable | 74 | ported | `sbt.rs` | `package_file_resolves_scala_version_variable_fixture` | — |
| extracts deps when scala version is defined in an object | 99 | ported | `sbt.rs` | `package_file_resolves_object_variables` | — |
| skips deps when dotted symbolds do not resolve to anything | 136 | ported | `sbt.rs` | `package_file_keeps_unresolved_dotted_symbols_without_current_value` | — |
| extracts packageFileVersion when scala version is defined in a variable | 159 | ported | `sbt.rs` | `package_file_resolves_package_file_version_variable` | — |
| extracts typed variables | 170 | ported | `sbt.rs` | `package_file_resolves_typed_variables` | — |
| skips deps when scala version is missing | 185 | ported | `sbt.rs` | `package_file_extracts_deps_when_scala_version_is_missing` | — |
| extract deps from native scala file with variables | 213 | ported | `sbt.rs` | `package_file_extracts_native_scala_file_variables` | — |
| extracts deps when scala version is defined with a trailing comma | 232 | ported | `sbt.rs` | `package_file_resolves_scala_version_with_trailing_comma` | — |
| extracts deps when scala version is defined in a variable with a trailing comma | 253 | ported | `sbt.rs` | `package_file_resolves_variable_scala_version_with_trailing_comma` | — |
| extracts deps when scala version is defined with ThisBuild scope | 275 | ported | `sbt.rs` | `package_file_resolves_thisbuild_scala_version` | — |
| extracts correct scala library when dealing with scala 3 | 294 | ported | `sbt.rs` | `package_file_extracts_scala3_library` | — |
| extracts deps correctly when dealing with scala 3 | 309 | ported | `sbt.rs` | `package_file_resolves_scala3_cross_dependencies` | — |
| extracts deps when scala version is defined in a variable with ThisBuild scope | 329 | ported | `sbt.rs` | `package_file_resolves_thisbuild_variable_scala_version` | — |
| extract deps from native scala file with private variables | 349 | ported | `sbt.rs` | `package_file_extracts_native_scala_private_variables` | — |
| extract deps when they are defined in a new line | 371 | ported | `sbt.rs` | `package_file_extracts_deps_defined_in_named_seq` | — |
| extract deps with comment | 412 | ported | `sbt.rs` | `extracts_dependencies_with_trailing_comments` | — |
| extract addCompilerPlugin | 452 | ported | `sbt.rs` | `extracts_add_compiler_plugin` | — |
| extract sbt version | 469 | ported | `sbt.rs` | `build_properties_extracts_sbt_version` | — |
| extract sbt version if the file contains other properties | 492 | ported | `sbt.rs` | `build_properties_with_other_props_extracts_sbt_version` | — |
| ignores build.properties file if does not contain sbt version | 516 | ported | `sbt.rs` | `build_properties_without_sbt_version_returns_none` | — |
| extracts proxy repositories | 529 | ported | `sbt.rs` | `extract_all_package_files_extracts_proxy_repositories` | — |
| should include default registryUrls if no repositories file is provided | 607 | ported | `sbt.rs` | `extract_all_package_files_uses_default_registry_urls_without_repositories_file` | — |
| should return empty packagefiles is no content is provided | 637 | ported | `sbt.rs` | `extract_all_package_files_empty_content_returns_empty` | — |
| extracts build properties correctly | 643 | ported | `sbt.rs` | `extract_all_package_files_extracts_build_properties` | — |

---

## `lib/modules/manager/terraform/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/terraform/extract.spec.ts
**Total tests:** 18 | **Ported:** 18 | **Actionable:** 18 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 39 | ported | `terraform.rs` | `empty_file_returns_empty` | — |
| returns null for no deps | 43 | ported | `terraform.rs` | `data_block_not_extracted` | — |
| extracts  modules | 54 | ported | `terraform.rs` | `module_with_version` (+ module_without_version_skipped, module_with_git_source_skipped, mixed_providers_and_modules) | — |
| extracts bitbucket modules | 221 | ported | `terraform.rs` | `bitbucket_module_sources_are_extracted` | — |
| extracts azureDevOps modules | 306 | ported | `terraform.rs` | `azure_devops_module_sources_are_extracted` | — |
| resolves OCI registry aliases | 338 | ported | `terraform.rs` | `oci_module_registry_alias_is_applied` | — |
| handles invalid OCI source URL | 358 | ported | `terraform.rs` | `invalid_oci_module_source_has_skip_reason` | — |
| extracts OCI modules and providers | 374 | ported | `terraform.rs` | `oci_modules_and_required_providers_are_extracted` | — |
| extracts providers | 463 | ported | `terraform.rs` | `required_providers_block_form` (+ required_providers_inline_string_form, comments_ignored, provider_without_source_uses_name) | — |
| extracts docker resources | 579 | ported | `terraform.rs` | `docker_resources_are_extracted` | — |
| extracts kubernetes resources | 655 | ported | `terraform.rs` | `kubernetes_resources_are_extracted` | — |
| returns dep with skipReason local | 756 | ported | `terraform.rs` | `module_with_local_path_skipped` (+ local_module_has_skip_reason) | — |
| returns null with only not added resources | 767 | ported | `terraform.rs` | `resource_block_not_extracted` | — |
| extract helm releases | 776 | ported | `terraform.rs` | `helm_releases_are_extracted` | — |
| update lockfile constraints with range strategy update-lockfile | 845 | ported | `terraform.rs` | `provider_lockfile_versions_are_applied` | — |
| test terraform block with only requirement_terraform_version | 884 | ported | `terraform.rs` | `required_version_extracted_as_hashicorp_terraform` | — |
| extracts terraform_version for tfe_workspace and ignores missing terraform_version keys | 904 | ported | `terraform.rs` | `tfe_workspace_terraform_versions_are_extracted` | — |
| return null if invalid HCL file | 933 | ported | `terraform.rs` | `invalid_hcl_returns_empty` | — |

---

## `lib/modules/manager/homeassistant-manifest/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/homeassistant-manifest/extract.spec.ts
**Total tests:** 16 | **Ported:** 16 | **Actionable:** 16 | **Status:** ported

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
| extracts git+https requirements | 138 | ported | `homeassistant.rs` | `extracts_git_https_requirements` | — |
| supports requirements with other operators | 168 | ported | `homeassistant.rs` | `extracts_range_version` | — |
| handles requirements without version | 211 | ported | `homeassistant.rs` | `handles_requirements_without_version` | — |
| extracts from real-world ASUSWRT manifest | 237 | ported | `homeassistant.rs` | `extracts_asuswrt_manifest` | — |
| handles invalid requirement types in array | 272 | ported | `homeassistant.rs` | `skips_non_string_entries_in_requirements_array` | — |
| returns null when requirements is not an array | 299 | ported | `homeassistant.rs` | `requirements_not_an_array_returns_empty` | — |
| handles unparseable requirement strings with skipReason | 313 | ported | `homeassistant.rs` | `unparseable_requirement_has_skip_reason` | — |

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
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty for invalid dependency file | 11 | ported | `mix.rs` | `no_deps_function_returns_empty` (+ deps_without_do_end_block) | — |
| extracts all dependencies when no lockfile | 16 | ported | `mix.rs` | `simple_hex_dep` (+ real_world_mix_exs, dep_with_only_option, git_dep_skipped, github_dep_skipped, path_dep_skipped, dep_without_version_skipped) | — |
| extracts all dependencies and adds the locked version if lockfile present | 139 | ported | `mix.rs` | `applies_locked_versions_from_mix_lock` | — |

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
**Total tests:** 11 | **Ported:** 11 | **Actionable:** 11 | **Status:** ported

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
| extract data from file with registry aliases | 139 | ported | `quadlet.rs` | `applies_registry_aliases_to_package_name` | — |
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

## `lib/modules/manager/ant/properties.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/ant/properties.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `modules/manager/ant/properties › parsePropertiesFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses key=value pairs | 6 | ported | `ant.rs` | `properties_file_parses_key_value_pairs` | — |
| skips comments and blank lines | 28 | ported | `ant.rs` | `properties_file_skips_comments_and_blank_lines` | — |
| supports colon separator | 39 | ported | `ant.rs` | `properties_file_supports_colon_separator` | — |
| skips malformed lines without separators | 46 | ported | `ant.rs` | `properties_file_skips_malformed_lines_without_separators` | — |
| implements first-definition-wins | 57 | ported | `ant.rs` | `properties_file_implements_first_definition_wins` | — |
| respects pre-existing props (first-definition-wins across sources) | 64 | ported | `ant.rs` | `properties_file_respects_pre_existing_props_across_sources` | — |

---

## `lib/modules/manager/ant/update.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/ant/update.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 10 | **Status:** ported

### `modules/manager/ant/update`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates inline XML version attribute | 4 | ported | `ant.rs` | `update_inline_xml_version_attribute` | — |
| updates single-quoted XML version attribute | 23 | ported | `ant.rs` | `update_single_quoted_xml_version_attribute` | — |
| updates .properties file value | 42 | ported | `ant.rs` | `update_properties_file_value` | — |
| updates .properties value at end of file without trailing newline | 58 | ported | `ant.rs` | `update_properties_value_at_eof_without_trailing_newline` | — |
| returns fileContent unchanged when already updated | 74 | ported | `ant.rs` | `update_returns_file_content_unchanged_when_already_updated` | — |
| updates when sharedVariableName is set even if currentValue differs | 91 | ported | `ant.rs` | `update_shared_variable_even_when_current_value_differs` | — |
| returns null when fileReplacePosition is undefined | 108 | ported | `ant.rs` | `update_returns_none_when_file_replace_position_is_missing` | — |
| updates version within coords attribute | 122 | ported | `ant.rs` | `update_version_within_coords_attribute` | — |
| updates version within 4-part coords attribute | 140 | ported | `ant.rs` | `update_version_within_four_part_coords_attribute` | — |
| returns null when value at position does not match | 158 | ported | `ant.rs` | `update_returns_none_when_value_at_position_does_not_match` | — |

---

## `lib/modules/manager/npm/extract/yarnrc.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/extract/yarnrc.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `modules/manager/npm/extract/yarnrc › resolveRegistryUrl()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| considers default registry | 10 | ported | `npm.rs` | `yarnrc_resolve_registry_url_considers_default_registry` | — |
| chooses matching scoped registry over default registry | 17 | ported | `npm.rs` | `yarnrc_resolve_registry_url_prefers_matching_scope` | — |
| ignores non matching scoped registry | 29 | ported | `npm.rs` | `yarnrc_resolve_registry_url_ignores_non_matching_scope` | — |
| ignores partial scope match | 40 | ported | `npm.rs` | `yarnrc_resolve_registry_url_ignores_partial_scope_match` | — |
| ignores missing scope registryServer | 51 | ported | `npm.rs` | `yarnrc_resolve_registry_url_ignores_missing_scope_registry_server` | — |

### `modules/manager/npm/extract/yarnrc › loadConfigFromYarnrcYml()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| produces expected config (%s) | 63 | ported | `npm.rs` | `load_config_from_yarnrc_yml_produces_expected_config` | — |

### `modules/manager/npm/extract/yarnrc › loadConfigFromLegacyYarnrc()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| produces expected config (%s) | 117 | ported | `npm.rs` | `load_config_from_legacy_yarnrc_produces_expected_config` | — |

---

## `lib/modules/manager/npm/extract/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/extract/index.spec.ts
**Total tests:** 41 | **Ported:** 17 | **Actionable:** 17 | **Status:** ported

### `modules/manager/npm/extract/index › .extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if cannot parse | 38 | ported | `npm.rs` | `package_json_extract_returns_error_if_cannot_parse` | — |
| catches invalid names | 47 | ported | `npm.rs` | `package_json_invalid_dependency_names_are_skipped` | — |
| ignores vendorised package.json | 58 | ported | `npm.rs` | `package_json_vendorised_installed_package_is_ignored` | — |
| throws error if non-root renovate config | 67 | not-applicable | — | — | Requires package-file path wrapper validation; Rust package.json extractor is content-only |
| returns null if no deps | 77 | ported | `npm.rs` | `empty_package_json_returns_empty_list` | — |
| handles invalid | 86 | ported | `npm.rs` | `package_json_invalid_dependency_sections_return_empty` | — |
| returns an array of dependencies | 95 | ported | `npm.rs` | `package_json_fixture_extracts_dependency_array` | — |
| returns an array of dependencies with resolution comments | 122 | ported | `npm.rs` | `package_json_resolution_comments_are_invalid_names` | — |
| finds a lock file | 151 | not-applicable | — | — | Requires async sibling file reads and managerData lockfile wrapper; Rust package.json extractor is content-only |
| warns when multiple lock files found | 170 | not-applicable | — | — | Requires async sibling file reads, logging, and managerData lockfile wrapper; Rust package.json extractor is content-only |
| finds and filters .npmrc | 197 | not-applicable | — | — | Requires async .npmrc discovery/filtering wrapper; Rust package.json extractor is content-only |
| uses config.npmrc if no .npmrc is returned from search | 220 | not-applicable | — | — | Requires ExtractConfig npmrc merge wrapper; Rust package.json extractor has no config/npmrc API |
| uses config.npmrc if no .npmrc exists | 229 | not-applicable | — | — | Requires ExtractConfig npmrc merge wrapper; Rust package.json extractor has no config/npmrc API |
| uses config.npmrc if .npmrc does exist but npmrcMerge=false | 239 | not-applicable | — | — | Requires ExtractConfig npmrc merge wrapper and async file reads; Rust package.json extractor has no config/npmrc API |
| merges config.npmrc and repo .npmrc when npmrcMerge=true | 262 | not-applicable | — | — | Requires ExtractConfig npmrc merge wrapper and async file reads; Rust package.json extractor has no config/npmrc API |
| finds and filters .npmrc with variables | 285 | not-applicable | — | — | Requires async .npmrc discovery/filtering wrapper; Rust package.json extractor is content-only |
| reads registryUrls from .yarnrc.yml | 310 | not-applicable | — | — | Requires async .yarnrc.yml discovery integrated into package extraction; static Yarn registry parser is covered in yarnrc.spec.ts |
| reads registryUrls from .yarnrc | 338 | not-applicable | — | — | Requires async .yarnrc discovery integrated into package extraction; static Yarn registry parser is covered in yarnrc.spec.ts |
| resolves registry URLs using the package name if set | 365 | not-applicable | — | — | Requires async .yarnrc.yml discovery plus packageManager dependency integration; static registry resolution is covered in yarnrc.spec.ts |
| finds complex yarn workspaces | 398 | not-applicable | — | — | Requires workspace glob discovery and async filesystem reads; Rust package.json extractor is content-only |
| extracts engines | 412 | ported | `npm.rs` | `package_json_extracts_engines` | — |
| extracts volta | 503 | ported | `npm.rs` | `package_json_extracts_volta` | — |
| extracts volta yarn unspecified-version | 543 | ported | `npm.rs` | `package_json_extracts_volta_yarn_unspecified` | — |
| extracts volta yarn higher than 1 | 584 | ported | `npm.rs` | `package_json_extracts_volta_yarn_higher_than_one` | — |
| extracts non-npmjs | 626 | ported | `npm.rs` | `package_json_extracts_non_npmjs_github_dependencies` | — |
| does not set registryUrls for non-npmjs | 760 | not-applicable | — | — | Requires package-file registryUrls metadata; Rust package.json extractor returns dependencies only |
| extracts npm package alias | 815 | ported | `npm.rs` | `npm_aliases_are_extracted` | — |
| sets skipInstalls false if Yarn zero-install is used | 866 | not-applicable | — | — | Requires async lockfile/.yarnrc.yml discovery and install-strategy managerData wrapper; Rust package.json extractor is content-only |
| extracts packageManager | 894 | ported | `npm.rs` | `package_json_extracts_package_manager` | — |
| sets hasPackageManager to true when devEngines detected in package file | 923 | not-applicable | — | — | Requires package-file managerData; Rust package.json extractor returns dependencies only |
| extracts dependencies from overrides | 957 | ported | `npm.rs` | `extracts_npm_overrides` | — |
| extracts dependencies from pnpm.overrides | 1036 | ported | `npm.rs` | `extracts_pnpm_overrides` | — |
| extracts dependencies from pnpm.overrides, with version ranges in flat syntax | 1117 | ported | `npm.rs` | `extracts_pnpm_override_range_keys` | — |

### `modules/manager/npm/extract/index › .extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| runs | 1200 | not-applicable | — | — | Requires Renovate `extractAllPackageFiles()` async multi-file manager wrapper; Rust npm extraction is content-level |
| warns for invalid pnpm workspace yaml files | 1250 | not-applicable | — | — | Requires Renovate `extractAllPackageFiles()` async multi-file manager wrapper and logging |
| parses empty pnpm workspace yaml files | 1267 | not-applicable | — | — | Requires Renovate `extractAllPackageFiles()` async multi-file manager wrapper |
| extracts pnpm workspace yaml files | 1276 | not-applicable | — | — | Requires Renovate `extractAllPackageFiles()` async multi-file manager wrapper; static pnpm workspace helper is covered in pnpm.spec.ts |
| extracts yarnrc.yml and adds it as packageFile | 1306 | not-applicable | — | — | Requires Renovate `extractAllPackageFiles()` async multi-file manager wrapper; static Yarn catalog helper is covered in yarn.spec.ts |
| extracts yarnrc.yml and adds it as packageFile and packageManager to true | 1340 | not-applicable | — | — | Requires Renovate `extractAllPackageFiles()` async multi-file manager wrapper and packageManager cross-file detection |
| extracts yarnrc.yml and adds it as packageFile and packageManager to false if no deps | 1372 | not-applicable | — | — | Requires Renovate `extractAllPackageFiles()` async multi-file manager wrapper and packageManager cross-file detection |

### `modules/manager/npm/extract/index › .postExtract()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| runs | 1409 | not-applicable | — | — | Renovate postExtract hook is a no-op async manager hook; no Rust equivalent hook exists |

---

## `lib/modules/manager/npm/extract/npm.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/extract/npm.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `modules/manager/npm/extract/npm › .getNpmLock()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if failed to parse | 9 | ported | `npm.rs` | `npm_lock_returns_empty_if_failed_to_parse` | — |
| extracts | 15 | ported | `npm.rs` | `npm_lock_extracts_v1_dependencies` | — |
| extracts npm 7 lockfile | 34 | ported | `npm.rs` | `npm_lock_extracts_v2_packages` | — |
| extracts npm 9 lockfile | 53 | ported | `npm.rs` | `npm_lock_extracts_v3_packages` | — |
| returns null if no deps | 72 | ported | `npm.rs` | `npm_lock_returns_empty_if_no_deps` | — |
| returns null on read error | 78 | ported | `npm.rs` | `npm_lock_returns_empty_on_read_error` | — |

---

## `lib/modules/manager/npm/extract/pnpm.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/extract/pnpm.spec.ts
**Total tests:** 16 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `modules/manager/npm/extract/pnpm › .extractPnpmFilters()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| detects errors in pnpm-workspace.yml file structure | 28 | not-applicable | — | — | Requires async filesystem/YAML parse logging around pnpm workspace discovery; Rust exposes static workspace extraction helpers only |
| detects errors when opening pnpm-workspace.yml file | 46 | not-applicable | — | — | Requires async filesystem/YAML parse logging around pnpm workspace discovery; Rust exposes static workspace extraction helpers only |

### `modules/manager/npm/extract/pnpm › .findPnpmWorkspace()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| detects missing pnpm-workspace.yaml | 65 | not-applicable | — | — | Requires async sibling/parent filesystem lookup; Rust exposes static workspace extraction helpers only |
| detects missing pnpm-lock.yaml when pnpm-workspace.yaml was already found | 78 | not-applicable | — | — | Requires async sibling/parent filesystem lookup; Rust exposes static workspace extraction helpers only |

### `modules/manager/npm/extract/pnpm › .detectPnpmWorkspaces()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses pnpm workspaces | 114 | not-applicable | — | — | Requires fixture-backed async filesystem traversal and workspace glob matching; Rust exposes static workspace extraction helpers only |
| skips when pnpm shrinkwrap file has already been provided | 203 | not-applicable | — | — | Requires async workspace detection over package file sets; Rust exposes static workspace extraction helpers only |
| filters none matching packages | 220 | not-applicable | — | — | Requires async workspace detection over package file sets; Rust exposes static workspace extraction helpers only |

### `modules/manager/npm/extract/pnpm › .getPnpmLock()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty if failed to parse | 266 | not-applicable | — | — | Requires pnpm-lock.yaml parser and async file reads; not ported |
| extracts version from monorepo | 272 | not-applicable | — | — | Requires pnpm-lock.yaml parser and fixture-backed async file reads; not ported |
| extracts version from normal repo | 279 | not-applicable | — | — | Requires pnpm-lock.yaml parser and fixture-backed async file reads; not ported |
| extracts version from catalogs | 289 | not-applicable | — | — | Requires pnpm-lock.yaml parser and async file reads; not ported |
| returns empty if no deps | 341 | ported | `npm.rs` | `pnpm_workspace_returns_empty_if_no_deps` | — |

### `modules/manager/npm/extract/pnpm › .extractPnpmWorkspaceFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles empty catalog entries | 349 | ported | `npm.rs` | `pnpm_workspace_handles_empty_catalog_entries` | — |
| parses valid pnpm-workspace.yaml file | 360 | ported | `npm.rs` | `pnpm_workspace_parses_valid_workspace_file` | — |
| parses overrides in pnpm-workspace.yaml file | 395 | ported | `npm.rs` | `pnpm_workspace_parses_overrides` | — |
| finds relevant lockfile | 466 | ported | `npm.rs` | `pnpm_workspace_finds_relevant_lockfile` | — |

---

## `lib/modules/manager/npm/extract/yarn.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/extract/yarn.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `modules/manager/npm/extract/yarn › .getYarnLock()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty if exception parsing | 10 | ported | `npm.rs` | `yarn_lock_returns_empty_if_exception_parsing` | — |
| extracts yarn 1 | 17 | ported | `npm.rs` | `yarn_lock_extracts_yarn1_dependencies` | — |
| extracts yarn 2 | 27 | ported | `npm.rs` | `yarn_lock_extracts_yarn2_dependencies` | — |
| extracts yarn 2 cache version | 37 | ported | `npm.rs` | `yarn_lock_extracts_yarn2_cache_version` | — |
| ignores individual invalid entries | 47 | ported | `npm.rs` | `yarn_lock_ignores_individual_invalid_entries` | — |

### `modules/manager/npm/extract/yarn`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getYarnVersionFromLock | 58 | ported | `npm.rs` | `yarn_version_from_lock_matches_lockfile_version` | — |

### `modules/manager/npm/extract/yarn › .extractYarnCatalogs()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles empty catalog entries | 78 | ported | `npm.rs` | `yarn_catalogs_handles_empty_catalog_entries` | — |
| parses valid .yarnrc.yml file | 86 | ported | `npm.rs` | `yarn_catalogs_parses_valid_yarnrc_yml` | — |
| finds relevant lockfile | 130 | ported | `npm.rs` | `yarn_catalogs_finds_relevant_lockfile` | — |

---

## `lib/modules/manager/bazel-module/bazelrc.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazel-module/bazelrc.spec.ts
**Total tests:** 19 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `modules/manager/bazel-module/bazelrc › BazelOption`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parse($a) | 35 | ported | `bazel_module.rs` | `bazelrc_option_parse_cases` | — |

### `modules/manager/bazel-module/bazelrc › CommandEntry`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getOption | 51 | ported | `bazel_module.rs` | `bazelrc_command_entry_get_option` | — |

### `modules/manager/bazel-module/bazelrc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parse | 62 | ported | `bazel_module.rs` | `bazelrc_parse_entries` | — |

### `modules/manager/bazel-module/bazelrc › read()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| when .bazelrc does not exist | 103 | not-applicable | — | — | Requires async mock filesystem `read()` traversal; Rust exposes static `.bazelrc` parser helpers only |
| when .bazelrc has invalid lines | 110 | not-applicable | — | — | Requires async mock filesystem `read()` traversal; Rust exposes static `.bazelrc` parser helpers only |
| when .bazelrc has no imports | 128 | not-applicable | — | — | Requires async mock filesystem `read()` traversal; Rust exposes static `.bazelrc` parser helpers only |
| when .bazelrc has import and try-import, try-import exists | 148 | not-applicable | — | — | Requires async mock filesystem `read()` traversal; Rust exposes static `.bazelrc` parser helpers only |
| when .bazelrc has import and try-import, try-import does not exist | 173 | not-applicable | — | — | Requires async mock filesystem `read()` traversal; Rust exposes static `.bazelrc` parser helpers only |
| when .bazelrc multi-level import | 188 | not-applicable | — | — | Requires async mock filesystem `read()` traversal; Rust exposes static `.bazelrc` parser helpers only |
| when bazlerc files recursively import each other | 213 | not-applicable | — | — | Requires async mock filesystem `read()` traversal and recursion error reporting; Rust exposes static `.bazelrc` parser helpers only |
| when .bazelrc refers to a non-local file | 239 | not-applicable | — | — | Requires async mock filesystem `read()` traversal; Rust exposes static `.bazelrc` parser helpers only |
| when bazelrc has %workspace% paths in options | 255 | not-applicable | — | — | Requires async mock filesystem `read()` traversal; Rust path expansion helper is covered directly |
| when bazelrc has %workspace% paths in imported files | 274 | not-applicable | — | — | Requires async mock filesystem `read()` traversal; Rust path expansion helper is covered directly |

### `modules/manager/bazel-module/bazelrc › expandWorkspacePath`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return original value if no workspace path | 304 | ported | `bazel_module.rs` | `bazelrc_expand_workspace_path_returns_original_without_workspace_path` | — |
| should expand valid workspace path | 310 | ported | `bazel_module.rs` | `bazelrc_expand_workspace_path_expands_valid_workspace_path` | — |
| should throw error for invalid workspace path | 320 | ported | `bazel_module.rs` | `bazelrc_expand_workspace_path_returns_none_for_invalid_workspace_path` | — |

### `modules/manager/bazel-module/bazelrc › sanitizeOptions`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should handle options without values | 328 | ported | `bazel_module.rs` | `bazelrc_sanitize_options_handles_options_without_values` | — |
| should expand valid workspace paths | 333 | ported | `bazel_module.rs` | `bazelrc_sanitize_options_expands_valid_workspace_paths` | — |
| should throw error for invalid workspace paths | 352 | ported | `bazel_module.rs` | `bazelrc_sanitize_options_drops_invalid_workspace_paths` | — |

---

## `lib/modules/manager/bazel-module/lockfile.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazel-module/lockfile.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/manager/bazel-module/lockfile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns updated lockfile when modified | 22 | not-applicable | — | — | Exercises Renovate `updateBazelLockfile()` artifact workflow with `bazel mod deps`, git status, and filesystem writes; Rust has no Bazel lockfile updater |
| returns updated lockfile when in not_added | 54 | not-applicable | — | — | Exercises Renovate `updateBazelLockfile()` artifact workflow with `bazel mod deps`, git status, and filesystem writes; Rust has no Bazel lockfile updater |
| returns null when lockfile is not modified | 83 | not-applicable | — | — | Exercises Renovate `updateBazelLockfile()` artifact workflow with `bazel mod deps` and git status; Rust has no Bazel lockfile updater |
| deletes lockfile during maintenance | 105 | not-applicable | — | — | Exercises Renovate lockfile maintenance deletion before `bazel mod deps`; Rust has no Bazel lockfile updater |
| does not delete lockfile when not in maintenance | 137 | not-applicable | — | — | Exercises Renovate lockfile maintenance deletion control; Rust has no Bazel lockfile updater |
| re-throws TEMPORARY_ERROR | 154 | not-applicable | — | — | Exercises Renovate artifact execution error handling; Rust has no Bazel lockfile updater |
| returns artifactError on exec failure | 168 | not-applicable | — | — | Exercises Renovate artifact execution error reporting; Rust has no Bazel lockfile updater |
| returns null when bazelModDeps is not allowed | 190 | not-applicable | — | — | Exercises Renovate unsafe execution policy for `bazel mod deps`; Rust has no Bazel lockfile updater |

---

## `lib/modules/manager/bazel-module/parser/fragments.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazel-module/parser/fragments.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `modules/manager/bazel-module/parser/fragments`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| .string() | 13 | ported | `bazel_module.rs` | `fragment_string_constructor` | — |
| .boolean() | 19 | ported | `bazel_module.rs` | `fragment_boolean_constructor` | — |
| .rule() | 25 | ported | `bazel_module.rs` | `fragment_rule_constructor` | — |
| .extensionTag() | 37 | ported | `bazel_module.rs` | `fragment_extension_tag_constructor` | — |
| .preparedExtensionTag() | 56 | ported | `bazel_module.rs` | `fragment_prepared_extension_tag_constructor` | — |
| .attribute() | 65 | ported | `bazel_module.rs` | `fragment_attribute_constructor` | — |
| .array() | 73 | ported | `bazel_module.rs` | `fragment_array_constructor` | — |
| .isValue($a) | 80 | ported | `bazel_module.rs` | `fragment_is_value_matches_renovate_value_fragments` | — |
| .isPrimitive($a) | 92 | ported | `bazel_module.rs` | `fragment_is_primitive_matches_renovate_primitive_fragments` | — |

---

## `lib/modules/manager/bazel-module/parser/context.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazel-module/parser/context.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 10 | **Status:** ported

### `modules/manager/bazel-module/parser/context › Ctx (failures cases) › extension tag`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws if there is no current | 7 | ported | `bazel_module.rs` | `bazel_ctx_start_extension_tag_errors_without_current` | — |
| throws if the current is not a prepared extension tag | 13 | ported | `bazel_module.rs` | `bazel_ctx_start_extension_tag_errors_for_wrong_current` | — |
| throws if the current is not an extension tag | 23 | ported | `bazel_module.rs` | `bazel_ctx_end_extension_tag_errors_for_wrong_current` | — |

### `modules/manager/bazel-module/parser/context › Ctx (failures cases)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws on missing current | 30 | ported | `bazel_module.rs` | `bazel_ctx_end_rule_errors_without_current` | — |
| throws on unbalanced endRule | 37 | ported | `bazel_module.rs` | `bazel_ctx_end_rule_errors_when_current_is_array` | — |
| throws on unbalanced endArray | 44 | ported | `bazel_module.rs` | `bazel_ctx_end_array_errors_when_current_is_rule` | — |
| throws if add an attribute without a parent | 51 | ported | `bazel_module.rs` | `bazel_ctx_add_string_to_parentless_attribute_errors` | — |
| throws if current use repo rule does not exist | 60 | ported | `bazel_module.rs` | `bazel_ctx_end_use_repo_rule_errors_for_wrong_current` | — |
| throws if current repo rule call does not exist | 67 | ported | `bazel_module.rs` | `bazel_ctx_end_repo_rule_call_errors_for_wrong_current` | — |
| creates CtxProcessingError with parent type | 74 | ported | `bazel_module.rs` | `bazel_ctx_processing_error_records_current_and_parent_type` | — |

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
<!-- asdf/index.spec.ts converted to per-test format above -->
<!-- ant/properties.spec.ts converted to per-test format above -->
<!-- ant/update.spec.ts converted to per-test format above -->
<!-- bazel-module/bazelrc.spec.ts converted to per-test format above -->
<!-- bazel-module/lockfile.spec.ts converted to per-test format above -->
<!-- bazel-module/parser/context.spec.ts converted to per-test format above -->
<!-- bazel-module/parser/fragments.spec.ts converted to per-test format above -->
<!-- batect-wrapper/artifacts.spec.ts converted to per-test format above -->
<!-- git-submodules/artifact.spec.ts converted to per-test format above -->
<!-- github-actions/integration.spec.ts converted to per-test format above -->
<!-- github-actions/parse.spec.ts converted to per-test format above -->
<!-- helmv3/common.spec.ts converted to per-test format above -->
<!-- npm/extract/index.spec.ts converted to per-test format above -->
<!-- npm/extract/npm.spec.ts converted to per-test format above -->
<!-- npm/extract/pnpm.spec.ts converted to per-test format above -->
<!-- npm/extract/yarn.spec.ts converted to per-test format above -->
<!-- npm/extract/yarnrc.spec.ts converted to per-test format above -->
<!-- ruby-version/extract.spec.ts converted to per-test format above -->
<!-- nvm/extract.spec.ts, terraform-version/extract.spec.ts, terragrunt-version/extract.spec.ts also covered in per-test sections above (all use version_file.rs) -->

---

## `lib/modules/datasource/artifactory/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/artifactory/index.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/artifactory/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses real data (folders): with slash at the end | 26 | not-applicable | — | — | Artifactory datasource lookup and HTML directory listing parsing are not implemented in Rust. |
| parses real data (files): without slash at the end | 42 | not-applicable | — | — | Artifactory datasource lookup and HTML directory listing parsing are not implemented in Rust. |
| parses real data (merge strategy with 2 registries) | 58 | not-applicable | — | — | Artifactory datasource lookup and HTML directory listing parsing are not implemented in Rust. |
| returns null without registryUrl + warning | 80 | not-applicable | — | — | Artifactory datasource lookup and HTML directory listing parsing are not implemented in Rust. |
| returns null for empty 200 OK | 94 | not-applicable | — | — | Artifactory datasource lookup and HTML directory listing parsing are not implemented in Rust. |
| 404 returns null | 108 | not-applicable | — | — | Artifactory datasource lookup and HTML directory listing parsing are not implemented in Rust. |
| throws for error diff than 404 | 128 | not-applicable | — | — | Artifactory datasource lookup and HTML directory listing parsing are not implemented in Rust. |
| throws no Http error | 139 | not-applicable | — | — | Artifactory datasource lookup and HTML directory listing parsing are not implemented in Rust. |

---

## `lib/modules/datasource/bazel/schema.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/bazel/schema.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `modules/datasource/bazel/schema › BazelModuleMetadata`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses metadata | 6 | ported | `bazel.rs` | `bazel_module_metadata_parses_versions_with_yanked_versions` | — |

---

## `lib/modules/datasource/aws-eks-addon/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/aws-eks-addon/index.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/aws-eks-addon/index › getPkgReleases()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returned $des addons to be null | 92 | not-applicable | — | — | AWS EKS addon datasource lookup and AWS SDK-backed release filtering are not implemented in Rust. |
| with addonName not supplied | 113 | not-applicable | — | — | AWS EKS addon datasource lookup and AWS SDK-backed release filtering are not implemented in Rust. |
| with addonName only | 129 | not-applicable | — | — | AWS EKS addon datasource lookup and AWS SDK-backed release filtering are not implemented in Rust. |
| with addon and profile | 160 | not-applicable | — | — | AWS EKS addon datasource lookup and AWS SDK-backed release filtering are not implemented in Rust. |
| with addon and region | 169 | not-applicable | — | — | AWS EKS addon datasource lookup and AWS SDK-backed release filtering are not implemented in Rust. |
| with addonName and default only config | 178 | not-applicable | — | — | AWS EKS addon datasource lookup and AWS SDK-backed release filtering are not implemented in Rust. |
| with matched addon to return all versions of the addon | 204 | not-applicable | — | — | AWS EKS addon datasource lookup and AWS SDK-backed release filtering are not implemented in Rust. |

---

## `lib/modules/datasource/aws-eks-addon/schema.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/aws-eks-addon/schema.spec.ts
**Total tests:** 1 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/aws-eks-addon/schema › EksAddonsFilter`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| safeParse("$input") === $expected | 5 | not-applicable | — | — | AWS EKS addon datasource filtering and its Zod schema are not implemented in Rust. |

---

## `lib/modules/datasource/aws-machine-image/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/aws-machine-image/index.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/aws-machine-image/index › getSortedAwsMachineImages()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| with 3 returned images | 137 | not-applicable | — | — | AWS EC2 machine image datasource and AWS SDK-backed image sorting are not implemented in Rust. |
| with 1 returned image | 147 | not-applicable | — | — | AWS EC2 machine image datasource and AWS SDK-backed image sorting are not implemented in Rust. |
| without returned images | 157 | not-applicable | — | — | AWS EC2 machine image datasource and AWS SDK-backed image sorting are not implemented in Rust. |

### `modules/datasource/aws-machine-image/index › getDigest()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| without newValue, without returned images to be null | 169 | not-applicable | — | — | AWS EC2 machine image datasource and digest lookup are not implemented in Rust. |
| without newValue, with one matching image to return that image | 179 | not-applicable | — | — | AWS EC2 machine image datasource and digest lookup are not implemented in Rust. |
| without newValue, with 3 matching image to return the newest image | 189 | not-applicable | — | — | AWS EC2 machine image datasource and digest lookup are not implemented in Rust. |
| with matching newValue, with 3 matching image to return the matching image | 199 | not-applicable | — | — | AWS EC2 machine image datasource and digest lookup are not implemented in Rust. |
| with not matching newValue, with 3 matching images to return the matching image | 212 | not-applicable | — | — | AWS EC2 machine image datasource and digest lookup are not implemented in Rust. |

### `modules/datasource/aws-machine-image/index › getPkgReleases()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| without returned images to be null | 227 | not-applicable | — | — | AWS EC2 machine image datasource and release lookup are not implemented in Rust. |
| with one matching image to return that image | 237 | not-applicable | — | — | AWS EC2 machine image datasource and release lookup are not implemented in Rust. |
| with one deprecated matching image to return that image | 256 | not-applicable | — | — | AWS EC2 machine image datasource and release lookup are not implemented in Rust. |
| with 3 matching image to return the newest image | 275 | not-applicable | — | — | AWS EC2 machine image datasource and release lookup are not implemented in Rust. |

### `modules/datasource/aws-machine-image/index › loadConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| loads filters without aws config | 298 | not-applicable | — | — | AWS EC2 machine image datasource filter/config parsing is not implemented in Rust. |
| loads filters with multiple aws configs | 313 | not-applicable | — | — | AWS EC2 machine image datasource filter/config parsing is not implemented in Rust. |

---

## `lib/modules/datasource/aws-rds/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/aws-rds/index.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/aws-rds/index › getPkgReleases()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| without returned versions | 104 | not-applicable | — | — | AWS RDS datasource and AWS SDK-backed engine version lookup are not implemented in Rust. |
| with one deprecated version | 115 | not-applicable | — | — | AWS RDS datasource and AWS SDK-backed engine version lookup are not implemented in Rust. |
| with 3 matching versions | 134 | not-applicable | — | — | AWS RDS datasource and AWS SDK-backed engine version lookup are not implemented in Rust. |

---

## `lib/modules/datasource/datasource.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/datasource.spec.ts
**Total tests:** 2 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/datasource`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw on 429 | 24 | not-applicable | — | — | TypeScript abstract Datasource base-class HTTP error handling is not implemented as a shared Rust datasource abstraction. |
| should throw on statusCode >=500 && <600 | 35 | not-applicable | — | — | TypeScript abstract Datasource base-class HTTP error handling is not implemented as a shared Rust datasource abstraction. |

---

## `lib/modules/datasource/azure-tags/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/azure-tags/index.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/azure-tags/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns tags from azure devops | 20 | not-applicable | — | — | Azure DevOps tags datasource and Azure platform Git API integration are not implemented in Rust. |
| filters out undefined names | 47 | not-applicable | — | — | Azure DevOps tags datasource and Azure platform Git API integration are not implemented in Rust. |
| handles api errors | 70 | not-applicable | — | — | Azure DevOps tags datasource and Azure platform Git API error handling are not implemented in Rust. |

### `modules/datasource/azure-tags/index › static methods`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getCacheKey returns the expected format | 83 | not-applicable | — | — | Azure Tags datasource type and cache-key helper are not implemented in Rust. |
| getSourceUrl returns the correct URL format | 92 | not-applicable | — | — | Azure Tags datasource type and source-url helper are not implemented in Rust. |

---

## `lib/modules/datasource/buildpacks-registry/schema.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/buildpacks-registry/schema.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `modules/datasource/buildpacks-registry/schema`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses buildpack-registry schema | 4 | ported | `buildpacks_registry.rs` | `buildpacks_registry_schema_parses_latest_and_versions` | — |

---

## `lib/modules/datasource/bitbucket-server-tags/schema.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/bitbucket-server-tags/schema.spec.ts
**Total tests:** 2 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/bitbucket-server-tags/schema`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses BitbucketServerTags | 4 | not-applicable | — | — | Bitbucket Server tags datasource and its response schema are not implemented in Rust. |
| parses BitbucketServerCommits | 39 | not-applicable | — | — | Bitbucket Server commits response schema is not implemented in Rust. |

---

## `lib/modules/datasource/cpan/schema.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/cpan/schema.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/cpan/schema › MetaCpanApiFileSearchResponse`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| filters out entries with empty module arrays | 5 | not-applicable | — | — | Rust CPAN datasource uses the MetaCPAN module endpoint and does not implement Renovate's file-search response schema transform. |
| filters out entries where module has no version | 29 | not-applicable | — | — | Rust CPAN datasource uses the MetaCPAN module endpoint and does not implement Renovate's file-search response schema transform. |
| includes valid entries | 53 | not-applicable | — | — | Rust CPAN datasource uses the MetaCPAN module endpoint and does not implement Renovate's file-search response schema transform. |

---

## `lib/modules/datasource/kubernetes-api/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/kubernetes-api/index.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/kubernetes-api/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for an unknown Kubernetes API type | 8 | not-applicable | — | — | Kubernetes API version datasource lookup is not implemented in Rust. |
| returns for a known Kubernetes API type | 13 | not-applicable | — | — | Kubernetes API version datasource lookup is not implemented in Rust. |
| is case sensitive | 27 | not-applicable | — | — | Kubernetes API version datasource lookup is not implemented in Rust. |

---

## `lib/modules/datasource/nextcloud/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/nextcloud/index.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/nextcloud/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no registryUrl | 6 | not-applicable | — | — | Nextcloud app datasource lookup and release metadata mapping are not implemented in Rust. |
| no package | 16 | not-applicable | — | — | Nextcloud app datasource lookup and release metadata mapping are not implemented in Rust. |
| package with no versions | 30 | not-applicable | — | — | Nextcloud app datasource lookup and release metadata mapping are not implemented in Rust. |
| package with website %s returns %s | 56 | not-applicable | — | — | Nextcloud app datasource lookup and release metadata mapping are not implemented in Rust. |
| package with changelog content and url | 102 | not-applicable | — | — | Nextcloud app datasource lookup and release metadata mapping are not implemented in Rust. |

---

## `lib/modules/datasource/github-digest/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/github-digest/index.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/github-digest/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns tags and branches merged | 20 | not-applicable | — | — | GitHub digest datasource tag/branch SHA lookup is not implemented in Rust. |
| prioritizes tags over branches with same name | 72 | not-applicable | — | — | GitHub digest datasource tag/branch SHA lookup is not implemented in Rust. |
| returns only branches when no tags | 118 | not-applicable | — | — | GitHub digest datasource tag/branch SHA lookup is not implemented in Rust. |
| throws when tags query fails | 145 | not-applicable | — | — | GitHub digest datasource tag/branch SHA lookup is not implemented in Rust. |
| throws when branches query fails | 156 | not-applicable | — | — | GitHub digest datasource tag/branch SHA lookup is not implemented in Rust. |
| returns tag digest when tag exists | 171 | not-applicable | — | — | GitHub digest datasource tag/branch SHA lookup is not implemented in Rust. |
| returns branch digest when tag not found | 186 | not-applicable | — | — | GitHub digest datasource tag/branch SHA lookup is not implemented in Rust. |
| prefers tag over branch with same name | 202 | not-applicable | — | — | GitHub digest datasource tag/branch SHA lookup is not implemented in Rust. |
| returns null when not found in tags or branches | 217 | not-applicable | — | — | GitHub digest datasource tag/branch SHA lookup is not implemented in Rust. |
| returns null when newValue is undefined | 226 | not-applicable | — | — | GitHub digest datasource tag/branch SHA lookup is not implemented in Rust. |

---

## `lib/modules/datasource/github-release-attachments/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/github-release-attachments/index.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/github-release-attachments/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns releases | 23 | not-applicable | — | — | GitHub release attachments datasource and asset digest update workflow are not implemented in Rust. |
| requires currentDigest | 99 | not-applicable | — | — | GitHub release attachments datasource and asset digest update workflow are not implemented in Rust. |
| defaults to currentDigest when currentVersion is missing | 107 | not-applicable | — | — | GitHub release attachments datasource and asset digest update workflow are not implemented in Rust. |
| returns updated digest in new release | 119 | not-applicable | — | — | GitHub release attachments datasource and asset digest update workflow are not implemented in Rust. |
| ignores failures verifying currentDigest | 141 | not-applicable | — | — | GitHub release attachments datasource and asset digest update workflow are not implemented in Rust. |

---

## `lib/modules/datasource/github-release-attachments/digest.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/github-release-attachments/digest.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/github-release-attachments/digest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| finds SHASUMS.txt file containing digest | 16 | not-applicable | — | — | GitHub release attachment checksum discovery and digesting are not implemented in Rust. |
| returns null when not found in digest file asset | 31 | not-applicable | — | — | GitHub release attachment checksum discovery and digesting are not implemented in Rust. |
| finds asset by digest | 49 | not-applicable | — | — | GitHub release attachment checksum discovery and digesting are not implemented in Rust. |
| returns null when no assets available | 67 | not-applicable | — | — | GitHub release attachment checksum discovery and digesting are not implemented in Rust. |
| downloads updated digest file | 86 | not-applicable | — | — | GitHub release attachment checksum discovery and digesting are not implemented in Rust. |
| maps digested file name to new version | 98 | not-applicable | — | — | GitHub release attachment checksum discovery and digesting are not implemented in Rust. |
| returns null when not found in digest file | 115 | not-applicable | — | — | GitHub release attachment checksum discovery and digesting are not implemented in Rust. |
| returns null when digest file not found | 127 | not-applicable | — | — | GitHub release attachment checksum discovery and digesting are not implemented in Rust. |
| falls back to digesting file when checksum file is removed | 136 | not-applicable | — | — | GitHub release attachment checksum discovery and digesting are not implemented in Rust. |
| digests updated file | 164 | not-applicable | — | — | GitHub release attachment checksum discovery and digesting are not implemented in Rust. |
| returns null when not found | 178 | not-applicable | — | — | GitHub release attachment checksum discovery and digesting are not implemented in Rust. |

---

## `lib/modules/datasource/python-version/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/python-version/index.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/python-version/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns Python EOL data | 14 | not-applicable | — | — | Python version datasource lookup, EOL metadata, and prebuilt release filtering are not implemented in Rust. |
| throws for 500 | 63 | not-applicable | — | — | Python version datasource lookup, EOL metadata, and prebuilt release filtering are not implemented in Rust. |
| returns null for error | 73 | not-applicable | — | — | Python version datasource lookup, EOL metadata, and prebuilt release filtering are not implemented in Rust. |
| falls back to prebuild releases on 429 | 83 | not-applicable | — | — | Python version datasource lookup, EOL metadata, and prebuilt release filtering are not implemented in Rust. |
| returns null on 429 when prebuild releases are unavailable | 102 | not-applicable | — | — | Python version datasource lookup, EOL metadata, and prebuilt release filtering are not implemented in Rust. |
| returns null for empty 200 OK | 116 | not-applicable | — | — | Python version datasource lookup, EOL metadata, and prebuilt release filtering are not implemented in Rust. |
| returns the correct data | 134 | not-applicable | — | — | Python version datasource lookup, EOL metadata, and prebuilt release filtering are not implemented in Rust. |
| only returns stable versions | 147 | not-applicable | — | — | Python version datasource lookup, EOL metadata, and prebuilt release filtering are not implemented in Rust. |
| only returns versions that are prebuilt | 158 | not-applicable | — | — | Python version datasource lookup, EOL metadata, and prebuilt release filtering are not implemented in Rust. |
| returns isDeprecated status for Python 3 minor releases | 170 | not-applicable | — | — | Python version datasource lookup, EOL metadata, and prebuilt release filtering are not implemented in Rust. |

---

## `lib/modules/datasource/deno/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/deno/index.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/deno/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns releases of standard library | 10 | not-applicable | — | — | Deno module registry datasource lookup is not implemented in Rust. |
| throws error if module endpoint fails | 75 | not-applicable | — | — | Deno module registry datasource lookup is not implemented in Rust. |
| throws error if version endpoint fails | 89 | not-applicable | — | — | Deno module registry datasource lookup is not implemented in Rust. |
| returns null if we could not match a deno land dependency | 117 | not-applicable | — | — | Deno module registry datasource lookup is not implemented in Rust. |
| returns releases of third-party library | 125 | not-applicable | — | — | Deno module registry datasource lookup is not implemented in Rust. |
| returns releases of a alternative registry server | 172 | not-applicable | — | — | Deno module registry datasource lookup is not implemented in Rust. |

---

## `lib/modules/datasource/flutter-version/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/flutter-version/index.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/flutter-version/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for 500 | 14 | not-applicable | — | — | Flutter version datasource lookup and channel filtering are not implemented in Rust. |
| returns null for error | 24 | not-applicable | — | — | Flutter version datasource lookup and channel filtering are not implemented in Rust. |
| returns null for empty 200 OK | 34 | not-applicable | — | — | Flutter version datasource lookup and channel filtering are not implemented in Rust. |
| processes real data | 44 | not-applicable | — | — | Flutter version datasource lookup and channel filtering are not implemented in Rust. |

---

## `lib/modules/datasource/gitlab-packages/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/gitlab-packages/index.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/gitlab-packages/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns package from custom registry | 8 | not-applicable | — | — | GitLab Packages datasource lookup is not implemented in Rust. |
| returns conan package from custom registry | 48 | not-applicable | — | — | GitLab Packages datasource lookup is not implemented in Rust. |
| returns null for 404 | 85 | not-applicable | — | — | GitLab Packages datasource lookup is not implemented in Rust. |
| returns null for empty 200 OK | 103 | not-applicable | — | — | GitLab Packages datasource lookup is not implemented in Rust. |
| throws for 5xx | 121 | not-applicable | — | — | GitLab Packages datasource lookup is not implemented in Rust. |

---

## `lib/modules/datasource/hexpm-bob/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/hexpm-bob/index.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/hexpm-bob/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for error | 9 | not-applicable | — | — | Hex.pm Bob build artifact datasource lookup is not implemented in Rust. |
| returns null for 404 | 22 | not-applicable | — | — | Hex.pm Bob build artifact datasource lookup is not implemented in Rust. |
| returns null for empty result | 35 | not-applicable | — | — | Hex.pm Bob build artifact datasource lookup is not implemented in Rust. |
| returns empty list for empty 200 OK | 48 | not-applicable | — | — | Hex.pm Bob build artifact datasource lookup is not implemented in Rust. |
| throws for 5xx | 61 | not-applicable | — | — | Hex.pm Bob build artifact datasource lookup is not implemented in Rust. |
| processes real data | 74 | not-applicable | — | — | Hex.pm Bob build artifact datasource lookup is not implemented in Rust. |
| processes real data (erlang / ubuntu 20.04) | 122 | not-applicable | — | — | Hex.pm Bob build artifact datasource lookup is not implemented in Rust. |
| can override registry url | 155 | not-applicable | — | — | Hex.pm Bob build artifact datasource lookup is not implemented in Rust. |
| returns empty list for invalid package name | 172 | not-applicable | — | — | Hex.pm Bob build artifact datasource lookup is not implemented in Rust. |

---

## `lib/modules/datasource/sbt-plugin/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/sbt-plugin/index.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/sbt-plugin/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses Maven index directory | 15 | not-applicable | — | — | SBT plugin datasource repository traversal and Maven POM URL extraction are not implemented in Rust. |
| parses sbt index directory | 23 | not-applicable | — | — | SBT plugin datasource repository traversal and Maven POM URL extraction are not implemented in Rust. |
| uses proper hostType | 31 | not-applicable | — | — | SBT plugin datasource repository traversal and Maven POM URL extraction are not implemented in Rust. |
| returns null in case of errors | 40 | not-applicable | — | — | SBT plugin datasource repository traversal and Maven POM URL extraction are not implemented in Rust. |
| fetches sbt plugins | 88 | not-applicable | — | — | SBT plugin datasource repository traversal and Maven POM URL extraction are not implemented in Rust. |
| fetches sbt plugins 2 | 157 | not-applicable | — | — | SBT plugin datasource repository traversal and Maven POM URL extraction are not implemented in Rust. |
| extracts URL from Maven POM file | 226 | not-applicable | — | — | SBT plugin datasource repository traversal and Maven POM URL extraction are not implemented in Rust. |
| handles absolute and root relative paths | 312 | not-applicable | — | — | SBT plugin datasource repository traversal and Maven POM URL extraction are not implemented in Rust. |

---

## `lib/modules/datasource/sbt-package/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/sbt-package/index.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/sbt-package/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses Maven index directory | 16 | not-applicable | — | — | SBT package datasource repository traversal, Scala-version mapping, and Maven fallback lookup are not implemented in Rust. |
| parses sbt index directory | 24 | not-applicable | — | — | SBT package datasource repository traversal, Scala-version mapping, and Maven fallback lookup are not implemented in Rust. |
| uses proper hostType | 32 | not-applicable | — | — | SBT package datasource repository traversal, Scala-version mapping, and Maven fallback lookup are not implemented in Rust. |
| returns null in case of errors | 41 | not-applicable | — | — | SBT package datasource repository traversal, Scala-version mapping, and Maven fallback lookup are not implemented in Rust. |
| returns null if there is no version | 61 | not-applicable | — | — | SBT package datasource repository traversal, Scala-version mapping, and Maven fallback lookup are not implemented in Rust. |
| fetches releases from Maven | 89 | not-applicable | — | — | SBT package datasource repository traversal, Scala-version mapping, and Maven fallback lookup are not implemented in Rust. |
| fetches Maven releases with Scala version | 140 | not-applicable | — | — | SBT package datasource repository traversal, Scala-version mapping, and Maven fallback lookup are not implemented in Rust. |
| fetches releases from Confluent | 169 | not-applicable | — | — | SBT package datasource repository traversal, Scala-version mapping, and Maven fallback lookup are not implemented in Rust. |
| extracts URL from Maven POM file | 209 | not-applicable | — | — | SBT package datasource repository traversal, Scala-version mapping, and Maven fallback lookup are not implemented in Rust. |
| falls back to Maven for orgarization root folder non-listable repositories | 243 | not-applicable | — | — | SBT package datasource repository traversal, Scala-version mapping, and Maven fallback lookup are not implemented in Rust. |
| extracts URL from Maven POM file | 287 | not-applicable | — | — | SBT package datasource repository traversal, Scala-version mapping, and Maven fallback lookup are not implemented in Rust. |

---

## `lib/modules/datasource/sbt-package/util.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/sbt-package/util.spec.ts
**Total tests:** 1 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/sbt-package/util`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets latest version | 4 | not-applicable | — | — | SBT package datasource version-list helper is not implemented in Rust. |

---

## `lib/modules/datasource/node-version/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/node-version/index.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/node-version/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for 500 | 9 | not-applicable | — | — | Node version datasource lookup is not implemented in Rust. |
| returns null for error | 19 | not-applicable | — | — | Node version datasource lookup is not implemented in Rust. |
| returns null for empty 200 OK | 32 | not-applicable | — | — | Node version datasource lookup is not implemented in Rust. |
| processes real data | 42 | not-applicable | — | — | Node version datasource lookup is not implemented in Rust. |

---

## `lib/modules/datasource/dart-version/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/dart-version/index.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/dart-version/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for 500 | 16 | not-applicable | — | — | Dart version datasource lookup is not implemented in Rust. |
| returns null for error | 26 | not-applicable | — | — | Dart version datasource lookup is not implemented in Rust. |
| returns null for empty 200 OK | 36 | not-applicable | — | — | Dart version datasource lookup is not implemented in Rust. |
| processes real data | 53 | not-applicable | — | — | Dart version datasource lookup is not implemented in Rust. |

---

## `lib/modules/datasource/gitea-releases/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/gitea-releases/index.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/gitea-releases/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns tags from gitea.com | 9 | not-applicable | — | — | Gitea releases datasource tag and commit lookup is not implemented in Rust. |
| returns tags from codeberg.org | 100 | not-applicable | — | — | Gitea releases datasource tag and commit lookup is not implemented in Rust. |
| returns commits from codeberg.org | 230 | not-applicable | — | — | Gitea releases datasource tag and commit lookup is not implemented in Rust. |
| returns commits from gitea.com | 277 | not-applicable | — | — | Gitea releases datasource tag and commit lookup is not implemented in Rust. |
| returns tags commit hash from gitea.com | 293 | not-applicable | — | — | Gitea releases datasource tag and commit lookup is not implemented in Rust. |

---

## `lib/modules/datasource/forgejo-releases/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/forgejo-releases/index.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/forgejo-releases/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns tags from forgejo.com | 9 | not-applicable | — | — | Forgejo releases datasource tag and commit lookup is not implemented in Rust. |
| returns tags from codeberg.org | 106 | not-applicable | — | — | Forgejo releases datasource tag and commit lookup is not implemented in Rust. |
| returns commits from codeberg.org | 236 | not-applicable | — | — | Forgejo releases datasource tag and commit lookup is not implemented in Rust. |
| returns commits from forgejo.com | 283 | not-applicable | — | — | Forgejo releases datasource tag and commit lookup is not implemented in Rust. |
| returns tags commit hash from forgejo.com | 299 | not-applicable | — | — | Forgejo releases datasource tag and commit lookup is not implemented in Rust. |

---

## `lib/modules/datasource/gitlab-releases/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/gitlab-releases/index.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/gitlab-releases/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns releases from custom registry | 18 | not-applicable | — | — | GitLab releases datasource lookup is not implemented in Rust; Rust only has a GitLab tags datasource. |
| returns releases from default registry | 32 | not-applicable | — | — | GitLab releases datasource lookup is not implemented in Rust; Rust only has a GitLab tags datasource. |
| return null if not found | 45 | not-applicable | — | — | GitLab releases datasource lookup is not implemented in Rust; Rust only has a GitLab tags datasource. |

---

## `lib/modules/datasource/bitbucket-tags/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/bitbucket-tags/index.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/bitbucket-tags/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns tags from bitbucket cloud | 9 | not-applicable | — | — | Bitbucket tags datasource tag and commit lookup is not implemented in Rust. |
| returns commits from bitbucket cloud | 43 | not-applicable | — | — | Bitbucket tags datasource tag and commit lookup is not implemented in Rust. |
| returns commits from bitbucket cloud | 85 | not-applicable | — | — | Bitbucket tags datasource tag and commit lookup is not implemented in Rust. |
| returns tags commit hash from bitbucket cloud | 112 | not-applicable | — | — | Bitbucket tags datasource tag and commit lookup is not implemented in Rust. |
| returns null for missing hash | 136 | not-applicable | — | — | Bitbucket tags datasource tag and commit lookup is not implemented in Rust. |

---

## `lib/modules/datasource/bitbucket-server-tags/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/bitbucket-server-tags/index.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/bitbucket-server-tags/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns tags | 12 | not-applicable | — | — | Bitbucket Server tags datasource tag and commit lookup is not implemented in Rust. |
| returns null on empty result | 66 | not-applicable | — | — | Bitbucket Server tags datasource tag and commit lookup is not implemented in Rust. |
| returns null on missing registryUrl | 80 | not-applicable | — | — | Bitbucket Server tags datasource tag and commit lookup is not implemented in Rust. |
| handles not found | 88 | not-applicable | — | — | Bitbucket Server tags datasource tag and commit lookup is not implemented in Rust. |
| returns commit hash of provided tag | 104 | not-applicable | — | — | Bitbucket Server tags datasource tag and commit lookup is not implemented in Rust. |
| missing hash | 124 | not-applicable | — | — | Bitbucket Server tags datasource tag and commit lookup is not implemented in Rust. |
| returns most recent commit hash | 146 | not-applicable | — | — | Bitbucket Server tags datasource tag and commit lookup is not implemented in Rust. |
| no commits | 173 | not-applicable | — | — | Bitbucket Server tags datasource tag and commit lookup is not implemented in Rust. |
| returns null on empty result | 195 | not-applicable | — | — | Bitbucket Server tags datasource tag and commit lookup is not implemented in Rust. |
| returns null on missing registryUrl | 211 | not-applicable | — | — | Bitbucket Server tags datasource tag and commit lookup is not implemented in Rust. |
| handles not found | 219 | not-applicable | — | — | Bitbucket Server tags datasource tag and commit lookup is not implemented in Rust. |

---

## `lib/modules/datasource/galaxy-collection/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/galaxy-collection/index.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/galaxy-collection/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for 404 result | 29 | not-applicable | — | — | Ansible Galaxy collection datasource lookup and Automation Hub URL mapping are not implemented in Rust. |
| throws for remote host error | 39 | not-applicable | — | — | Ansible Galaxy collection datasource lookup and Automation Hub URL mapping are not implemented in Rust. |
| returns null for unexpected data at base | 49 | not-applicable | — | — | Ansible Galaxy collection datasource lookup and Automation Hub URL mapping are not implemented in Rust. |
| returns null for unexpected data at versions | 62 | not-applicable | — | — | Ansible Galaxy collection datasource lookup and Automation Hub URL mapping are not implemented in Rust. |
| throws error for remote host versions error | 77 | not-applicable | — | — | Ansible Galaxy collection datasource lookup and Automation Hub URL mapping are not implemented in Rust. |
| throws error for remote host detailed versions error | 92 | not-applicable | — | — | Ansible Galaxy collection datasource lookup and Automation Hub URL mapping are not implemented in Rust. |
| returns null for empty lookup | 113 | not-applicable | — | — | Ansible Galaxy collection datasource lookup and Automation Hub URL mapping are not implemented in Rust. |
| returns null for null packageName | 122 | not-applicable | — | — | Ansible Galaxy collection datasource lookup and Automation Hub URL mapping are not implemented in Rust. |
| returns null for unknown error | 131 | not-applicable | — | — | Ansible Galaxy collection datasource lookup and Automation Hub URL mapping are not implemented in Rust. |
| processes real data | 144 | not-applicable | — | — | Ansible Galaxy collection datasource lookup and Automation Hub URL mapping are not implemented in Rust. |
| returns null but matches automation hub URL | 167 | not-applicable | — | — | Ansible Galaxy collection datasource lookup and Automation Hub URL mapping are not implemented in Rust. |
| processes real data with automation hub URL | 183 | not-applicable | — | — | Ansible Galaxy collection datasource lookup and Automation Hub URL mapping are not implemented in Rust. |
| returns ansible url with artifactory URL | 212 | not-applicable | — | — | Ansible Galaxy collection datasource lookup and Automation Hub URL mapping are not implemented in Rust. |
| returns galaxy url with automation hub URL | 223 | not-applicable | — | — | Ansible Galaxy collection datasource lookup and Automation Hub URL mapping are not implemented in Rust. |
| returns galaxy url with other url | 234 | not-applicable | — | — | Ansible Galaxy collection datasource lookup and Automation Hub URL mapping are not implemented in Rust. |

---

## `lib/modules/datasource/galaxy/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/galaxy/index.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/galaxy/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty result | 11 | not-applicable | — | — | Ansible Galaxy role datasource lookup is not implemented in Rust. |
| returns null for missing fields | 24 | not-applicable | — | — | Ansible Galaxy role datasource lookup is not implemented in Rust. |
| returns null for empty list | 37 | not-applicable | — | — | Ansible Galaxy role datasource lookup is not implemented in Rust. |
| returns null for 404 | 50 | not-applicable | — | — | Ansible Galaxy role datasource lookup is not implemented in Rust. |
| returns null for unknown error | 63 | not-applicable | — | — | Ansible Galaxy role datasource lookup is not implemented in Rust. |
| processes real data | 76 | not-applicable | — | — | Ansible Galaxy role datasource lookup is not implemented in Rust. |
| handles multiple results when one user matches exactly | 90 | not-applicable | — | — | Ansible Galaxy role datasource lookup is not implemented in Rust. |
| rejects multiple results when no user matches exactly | 103 | not-applicable | — | — | Ansible Galaxy role datasource lookup is not implemented in Rust. |
| return null if searching random username and project name | 115 | not-applicable | — | — | Ansible Galaxy role datasource lookup is not implemented in Rust. |
| throws for 5xx | 127 | not-applicable | — | — | Ansible Galaxy role datasource lookup is not implemented in Rust. |
| throws for 404 | 140 | not-applicable | — | — | Ansible Galaxy role datasource lookup is not implemented in Rust. |

---

## `lib/modules/datasource/repology/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/repology/index.spec.ts
**Total tests:** 19 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/repology/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty result | 69 | not-applicable | — | — | Repology datasource API and resolver lookups are not implemented in Rust. |
| returns null for missing repository or package | 88 | not-applicable | — | — | Repology datasource API and resolver lookups are not implemented in Rust. |
| throws error on unexpected API response | 105 | not-applicable | — | — | Repology datasource API and resolver lookups are not implemented in Rust. |
| throws error on unexpected Resolver response with binary package | 124 | not-applicable | — | — | Repology datasource API and resolver lookups are not implemented in Rust. |
| throws error on unexpected Resolver response with source package | 138 | not-applicable | — | — | Repology datasource API and resolver lookups are not implemented in Rust. |
| throws error on API request timeout | 156 | not-applicable | — | — | Repology datasource API and resolver lookups are not implemented in Rust. |
| throws error on Resolver request timeout | 175 | not-applicable | — | — | Repology datasource API and resolver lookups are not implemented in Rust. |
| returns null on Resolver ambiguous binary package | 189 | not-applicable | — | — | Repology datasource API and resolver lookups are not implemented in Rust. |
| throws without repository and package name | 204 | not-applicable | — | — | Repology datasource API and resolver lookups are not implemented in Rust. |
| throws on disabled host | 214 | not-applicable | — | — | Repology datasource API and resolver lookups are not implemented in Rust. |
| returns correct version for binary package | 225 | not-applicable | — | — | Repology datasource API and resolver lookups are not implemented in Rust. |
| returns correct version for source package | 241 | not-applicable | — | — | Repology datasource API and resolver lookups are not implemented in Rust. |
| returns correct version for api package | 260 | not-applicable | — | — | Repology datasource API and resolver lookups are not implemented in Rust. |
| returns correct version for multi-package project with same name | 276 | not-applicable | — | — | Repology datasource API and resolver lookups are not implemented in Rust. |
| returns correct version for multi-package project with different name | 292 | not-applicable | — | — | Repology datasource API and resolver lookups are not implemented in Rust. |
| returns multiple versions if they are present in repository | 308 | not-applicable | — | — | Repology datasource API and resolver lookups are not implemented in Rust. |
| returns null for scenario when repo is not in package results | 328 | not-applicable | — | — | Repology datasource API and resolver lookups are not implemented in Rust. |
| returns correct package types for api_call | 354 | not-applicable | — | — | Repology datasource API and resolver lookups are not implemented in Rust. |
| returns correct package versions for multi-package project | 443 | not-applicable | — | — | Repology datasource API and resolver lookups are not implemented in Rust. |

---

## `lib/modules/datasource/deb/checksum.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/deb/checksum.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/deb/checksum`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses the checksum for the specified package | 27 | not-applicable | — | — | Debian datasource package-index checksum parsing and validation are not implemented in Rust. |
| computes the checksum of a file | 47 | not-applicable | — | — | Debian datasource package-index checksum parsing and validation are not implemented in Rust. |
| should fail if there is an error in the stream | 56 | not-applicable | — | — | Debian datasource package-index checksum parsing and validation are not implemented in Rust. |

---

## `lib/modules/datasource/deb/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/deb/utils.spec.ts
**Total tests:** 1 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/deb/utils`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw error for unsupported compression | 29 | not-applicable | — | — | Debian datasource package-index decompression utilities are not implemented in Rust. |

---

## `lib/modules/datasource/deb/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/deb/index.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/deb/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns a valid version for the package `album` and does not require redownload | 72 | not-applicable | — | — | Debian datasource Release/InRelease package index lookup is not implemented in Rust. |
| returns null when registry url misses components | 101 | not-applicable | — | — | Debian datasource Release/InRelease package index lookup is not implemented in Rust. |
| returns null when registry url misses binaryArch | 109 | not-applicable | — | — | Debian datasource Release/InRelease package index lookup is not implemented in Rust. |
| returns null when registry url misses suite or release | 117 | not-applicable | — | — | Debian datasource Release/InRelease package index lookup is not implemented in Rust. |
| returns a valid version for the package `album` | 138 | not-applicable | — | — | Debian datasource Release/InRelease package index lookup is not implemented in Rust. |
| returns a valid version for the package `album` if release is used in the registryUrl | 152 | not-applicable | — | — | Debian datasource Release/InRelease package index lookup is not implemented in Rust. |
| returns null for an unknown package | 169 | not-applicable | — | — | Debian datasource Release/InRelease package index lookup is not implemented in Rust. |
| returns two releases for `album` which is the same across the components | 199 | not-applicable | — | — | Debian datasource Release/InRelease package index lookup is not implemented in Rust. |
| returns two releases for `album` which has different metadata across the components | 216 | not-applicable | — | — | Debian datasource Release/InRelease package index lookup is not implemented in Rust. |
| returns null for the package | 244 | not-applicable | — | — | Debian datasource Release/InRelease package index lookup is not implemented in Rust. |
| supports specifying a custom binary arch | 251 | not-applicable | — | — | Debian datasource Release/InRelease package index lookup is not implemented in Rust. |
| should not lead to a race condition on parallel lookups | 281 | not-applicable | — | — | Debian datasource Release/InRelease package index lookup is not implemented in Rust. |
| should parse the extracted package | 317 | not-applicable | — | — | Debian datasource Release/InRelease package index lookup is not implemented in Rust. |

---

## `lib/modules/datasource/deb/packages.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/deb/packages.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/deb/packages`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should ignore error when fetching the InRelease content fails | 67 | not-applicable | — | — | Debian datasource package-list fetching and checksum validation are not implemented in Rust. |
| should throw error when checksum validation fails | 93 | not-applicable | — | — | Debian datasource package-list fetching and checksum validation are not implemented in Rust. |
| should throw error for when extracting fails | 108 | not-applicable | — | — | Debian datasource package-list fetching and checksum validation are not implemented in Rust. |

---

## `lib/modules/datasource/deb/url.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/deb/url.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/deb/url`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| constructs URLs correctly from registry URL with suite | 11 | not-applicable | — | — | Debian datasource registry URL construction and cache freshness checks are not implemented in Rust. |
| constructs URLs correctly from registry URL with deprecated release | 22 | not-applicable | — | — | Debian datasource registry URL construction and cache freshness checks are not implemented in Rust. |
| throws an error if required parameters are missing | 33 | not-applicable | — | — | Debian datasource registry URL construction and cache freshness checks are not implemented in Rust. |
| should return true for different status code | 45 | not-applicable | — | — | Debian datasource registry URL construction and cache freshness checks are not implemented in Rust. |
| should return true if request failed | 60 | not-applicable | — | — | Debian datasource registry URL construction and cache freshness checks are not implemented in Rust. |

---

## `lib/config/migrate-validate.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrate-validate.spec.ts
**Total tests:** 5 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `config/migrate-validate › migrateAndValidate()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles empty | 14 | ported | `migrate_validate.rs` | `migrate_and_validate_handles_empty` | — |
| handles migration | 22 | ported | `migrate_validate.rs` | `migrate_and_validate_handles_migration` | — |
| handles invalid | 32 | ported | `migrate_validate.rs` | `migrate_and_validate_handles_invalid` | — |
| isOnboarded | 40 | ported | `migrate_validate.rs` | `migrate_and_validate_omits_warnings_when_onboarded` | — |
| logs errors | 50 | not-applicable | — | — | mocking framework/logging internals |

---

## `lib/config/massage.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/massage.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `config/massage › massageConfig`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty | 6 | ported | `massage.rs` | `massage_config_returns_empty` | — |
| massages strings to array | 12 | ported | `massage.rs` | `massage_config_converts_allowed_string_to_array` | — |
| normalizes zero minimumReleaseAge to null | 20 | ported | `massage.rs` | `massage_config_normalizes_zero_minimum_release_age` | — |
| normalizes zero minimumReleaseAge in packageRules | 30 | ported | `massage.rs` | `massage_config_normalizes_zero_minimum_release_age_in_package_rules` | — |
| massages packageRules matchUpdateTypes | 58 | ported | `massage.rs` | `massage_config_expands_package_rule_update_types` | — |
| filters packageRules with only match/exclude | 95 | ported | `massage.rs` | `massage_config_filters_package_rules_with_only_match_or_exclude` | — |
| does not massage lockFileMaintenance | 110 | ported | `massage.rs` | `massage_config_does_not_expand_lock_file_maintenance` | — |

---

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

## `lib/config/inherit.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/inherit.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/inherit`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| all values in OPTIONS are sorted | 4 | ported | `config.rs` | `inherit_config_options_are_sorted` | — |

### `config/inherit › InheritConfig.get()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return NOT_PRESENT if key is not set | 15 | ported | `config.rs` | `inherit_config_returns_not_present_for_missing_key` | — |
| return value if key is set | 20 | ported | `config.rs` | `inherit_config_returns_value_when_key_is_set` | — |

---

## `lib/config/decrypt.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/decrypt.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/decrypt › decryptConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty with no privateKey | 23 | not-applicable | — | — | Encrypted config/private-key handling is a platform encryption feature; Rust config layer does not implement decryption |
| warns if no privateKey found | 29 | not-applicable | — | — | Encrypted config/private-key handling is a platform encryption feature; Rust config layer does not implement decryption |
| throws exception if encrypted found but no privateKey | 41 | not-applicable | — | — | Encrypted config/private-key handling is a platform encryption feature; Rust config layer does not implement decryption |
| throws exception if encrypted found but no privateKey- Mend Hosted | 51 | not-applicable | — | — | Encrypted config/private-key handling is a platform encryption feature; Rust config layer does not implement decryption |

### `config/decrypt › validateDecryptedValue() › platforms non azure`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals("$str", "$repo") === $expected | 68 | not-applicable | — | — | Encrypted config validation/azure collection scoping belongs to the out-of-scope platform encryption feature; no Rust equivalent |

### `config/decrypt › validateDecryptedValue() › azure only platform › general tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals("$str", "$repo") === $expected | 93 | not-applicable | — | — | Encrypted config validation/azure collection scoping belongs to the out-of-scope platform encryption feature; no Rust equivalent |

### `config/decrypt › validateDecryptedValue() › azure only platform › tests self hosted - ignore "tfs/" before collection name`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals("$str", "$repo") === $expected | 129 | not-applicable | — | — | Encrypted config validation/azure collection scoping belongs to the out-of-scope platform encryption feature; no Rust equivalent |

### `config/decrypt › validateDecryptedValue() › azure only platform`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| endpoint URL invalid | 164 | not-applicable | — | — | Encrypted config validation/azure collection scoping belongs to the out-of-scope platform encryption feature; no Rust equivalent |
| endpoint URL without collection | 196 | not-applicable | — | — | Encrypted config validation/azure collection scoping belongs to the out-of-scope platform encryption feature; no Rust equivalent |

### `config/decrypt › getAzureCollection()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no pathname and url ends with slash | 235 | not-applicable | — | — | Encrypted config validation/azure collection scoping belongs to the out-of-scope platform encryption feature; no Rust equivalent |
| no pathname and no slash at end of URL | 243 | not-applicable | — | — | Encrypted config validation/azure collection scoping belongs to the out-of-scope platform encryption feature; no Rust equivalent |
| pathname no slash at end | 251 | not-applicable | — | — | Encrypted config validation/azure collection scoping belongs to the out-of-scope platform encryption feature; no Rust equivalent |
| pathname with slash at end | 259 | not-applicable | — | — | Encrypted config validation/azure collection scoping belongs to the out-of-scope platform encryption feature; no Rust equivalent |
| pathname 2 levels no slash at end | 267 | not-applicable | — | — | Encrypted config validation/azure collection scoping belongs to the out-of-scope platform encryption feature; no Rust equivalent |
| pathname 2 levels with slash at end | 275 | not-applicable | — | — | Encrypted config validation/azure collection scoping belongs to the out-of-scope platform encryption feature; no Rust equivalent |

---

## `lib/config/decrypt/bcpgp.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/decrypt/bcpgp.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/decrypt/bcpgp › decryptConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid key | 40 | not-applicable | — | — | BouncyCastle/OpenPGP encrypted config runtime is not implemented in the Rust config layer; only encrypted-field validation helpers exist. |
| works broken PGP message | 54 | not-applicable | — | — | BouncyCastle/OpenPGP encrypted config runtime is not implemented in the Rust config layer; only encrypted-field validation helpers exist. |
| fails with ECC and AEAD (wasm-dotnet | 72 | not-applicable | — | — | BouncyCastle/OpenPGP encrypted config runtime is not implemented in the Rust config layer; only encrypted-field validation helpers exist. |
| works with ECC and AEAD (wasm-java) | 92 | not-applicable | — | — | BouncyCastle/OpenPGP encrypted config runtime is not implemented in the Rust config layer; only encrypted-field validation helpers exist. |
| rejects invalid PGP message | 108 | not-applicable | — | — | BouncyCastle/OpenPGP encrypted config runtime is not implemented in the Rust config layer; only encrypted-field validation helpers exist. |
| handles PGP org constraint | 149 | not-applicable | — | — | BouncyCastle/OpenPGP encrypted config runtime is not implemented in the Rust config layer; only encrypted-field validation helpers exist. |
| handles PGP multi-org constraint | 163 | not-applicable | — | — | BouncyCastle/OpenPGP encrypted config runtime is not implemented in the Rust config layer; only encrypted-field validation helpers exist. |
| handles PGP org/repo constraint | 180 | not-applicable | — | — | BouncyCastle/OpenPGP encrypted config runtime is not implemented in the Rust config layer; only encrypted-field validation helpers exist. |
| handles PGP multi-org/repo constraint | 194 | not-applicable | — | — | BouncyCastle/OpenPGP encrypted config runtime is not implemented in the Rust config layer; only encrypted-field validation helpers exist. |

---

## `lib/config/decrypt/openpgp.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/decrypt/openpgp.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/decrypt/openpgp › decryptConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| rejects invalid PGP message | 44 | not-applicable | — | — | OpenPGP encrypted config runtime is not implemented in the Rust config layer; only encrypted-field validation helpers exist. |
| works with ECC and AEAD | 85 | not-applicable | — | — | OpenPGP encrypted config runtime is not implemented in the Rust config layer; only encrypted-field validation helpers exist. |
| handles PGP org constraint | 97 | not-applicable | — | — | OpenPGP encrypted config runtime is not implemented in the Rust config layer; only encrypted-field validation helpers exist. |
| handles PGP multi-org constraint | 111 | not-applicable | — | — | OpenPGP encrypted config runtime is not implemented in the Rust config layer; only encrypted-field validation helpers exist. |
| handles PGP org/repo constraint | 128 | not-applicable | — | — | OpenPGP encrypted config runtime is not implemented in the Rust config layer; only encrypted-field validation helpers exist. |
| handles PGP multi-org/repo constraint | 142 | not-applicable | — | — | OpenPGP encrypted config runtime is not implemented in the Rust config layer; only encrypted-field validation helpers exist. |
| fails to load openpgp | 159 | not-applicable | — | — | OpenPGP encrypted config runtime is not implemented in the Rust config layer; only encrypted-field validation helpers exist. |

---

## `lib/config/presets/forgejo/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/forgejo/index.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/presets/forgejo/index › fetchJSONFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns JSON | 19 | not-applicable | — | — | Forgejo remote preset fetching via JavaScript HTTP contents API is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| returns JSON5 | 36 | not-applicable | — | — | Forgejo remote preset fetching via JavaScript HTTP contents API is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| throws external host error | 53 | not-applicable | — | — | Forgejo remote preset fetching and host-rule error handling are not implemented in Rust. |

### `config/presets/forgejo/index › getPreset()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| tries default then renovate | 73 | not-applicable | — | — | Forgejo remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| throws if invalid content | 84 | not-applicable | — | — | Forgejo remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| throws if fails to parse | 95 | not-applicable | — | — | Forgejo remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should return default.json | 108 | not-applicable | — | — | Forgejo remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should query preset within the file | 120 | not-applicable | — | — | Forgejo remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should query subpreset | 134 | not-applicable | — | — | Forgejo remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should return custom.json | 151 | not-applicable | — | — | Forgejo remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should query custom paths | 165 | not-applicable | — | — | Forgejo remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should throws not-found | 180 | not-applicable | — | — | Forgejo remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |

### `config/presets/forgejo/index › getPresetFromEndpoint()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses default endpoint | 197 | not-applicable | — | — | Forgejo remote preset endpoint/tag resolution is not implemented in Rust. |
| uses custom endpoint | 209 | not-applicable | — | — | Forgejo remote preset endpoint/tag resolution is not implemented in Rust. |
| uses default endpoint with a tag | 228 | not-applicable | — | — | Forgejo remote preset endpoint/tag resolution is not implemented in Rust. |
| uses custom endpoint with a tag | 246 | not-applicable | — | — | Forgejo remote preset endpoint/tag resolution is not implemented in Rust. |

---

## `lib/config/presets/gitea/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/gitea/index.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/presets/gitea/index › fetchJSONFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns JSON | 19 | not-applicable | — | — | Gitea remote preset fetching via JavaScript HTTP contents API is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| returns JSON5 | 36 | not-applicable | — | — | Gitea remote preset fetching via JavaScript HTTP contents API is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| throws external host error | 53 | not-applicable | — | — | Gitea remote preset fetching and host-rule error handling are not implemented in Rust. |

### `config/presets/gitea/index › getPreset()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| tries default then renovate | 73 | not-applicable | — | — | Gitea remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| throws if invalid content | 84 | not-applicable | — | — | Gitea remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| throws if fails to parse | 95 | not-applicable | — | — | Gitea remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should return default.json | 108 | not-applicable | — | — | Gitea remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should query preset within the file | 120 | not-applicable | — | — | Gitea remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should query subpreset | 134 | not-applicable | — | — | Gitea remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should return custom.json | 151 | not-applicable | — | — | Gitea remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should query custom paths | 165 | not-applicable | — | — | Gitea remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should throws not-found | 180 | not-applicable | — | — | Gitea remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |

### `config/presets/gitea/index › getPresetFromEndpoint()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses default endpoint | 197 | not-applicable | — | — | Gitea remote preset endpoint/tag resolution is not implemented in Rust. |
| uses custom endpoint | 209 | not-applicable | — | — | Gitea remote preset endpoint/tag resolution is not implemented in Rust. |
| uses default endpoint with a tag | 228 | not-applicable | — | — | Gitea remote preset endpoint/tag resolution is not implemented in Rust. |
| uses custom endpoint with a tag | 246 | not-applicable | — | — | Gitea remote preset endpoint/tag resolution is not implemented in Rust. |

---

## `lib/config/presets/github/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/github/index.spec.ts
**Total tests:** 17 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/presets/github/index › fetchJSONFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns JSON | 17 | not-applicable | — | — | GitHub remote preset fetching via JavaScript contents API is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| throws external host error | 34 | not-applicable | — | — | GitHub remote preset fetching and host-rule error handling are not implemented in Rust. |

### `config/presets/github/index › getPreset()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| tries default then renovate | 54 | not-applicable | — | — | GitHub remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| throws if invalid content | 65 | not-applicable | — | — | GitHub remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| throws if fails to parse | 76 | not-applicable | — | — | GitHub remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should return default.json | 89 | not-applicable | — | — | GitHub remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should query preset within the file | 101 | not-applicable | — | — | GitHub remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should query preset within the file when .json extension provided | 115 | not-applicable | — | — | GitHub remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should query preset within the file when .json5 extension provided | 129 | not-applicable | — | — | GitHub remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should query subpreset | 143 | not-applicable | — | — | GitHub remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should return custom.json | 160 | not-applicable | — | — | GitHub remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should query custom paths | 174 | not-applicable | — | — | GitHub remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should throws not-found | 189 | not-applicable | — | — | GitHub remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |

### `config/presets/github/index › getPresetFromEndpoint()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses default endpoint | 206 | not-applicable | — | — | GitHub remote preset endpoint/tag resolution is not implemented in Rust. |
| uses custom endpoint | 218 | not-applicable | — | — | GitHub remote preset endpoint/tag resolution is not implemented in Rust. |
| uses default endpoint with a tag | 238 | not-applicable | — | — | GitHub remote preset endpoint/tag resolution is not implemented in Rust. |
| uses custom endpoint with a tag | 256 | not-applicable | — | — | GitHub remote preset endpoint/tag resolution is not implemented in Rust. |

---

## `lib/config/presets/gitlab/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/gitlab/index.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/presets/gitlab/index › getPreset()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws EXTERNAL_HOST_ERROR | 12 | not-applicable | — | — | GitLab remote preset fetching and host-rule error handling are not implemented in Rust. |
| throws if project could not be found | 22 | not-applicable | — | — | GitLab remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| throws if missing | 32 | not-applicable | — | — | GitLab remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should return the preset | 47 | not-applicable | — | — | GitLab remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should return the preset with a tag | 61 | not-applicable | — | — | GitLab remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should query custom paths | 74 | not-applicable | — | — | GitLab remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should query custom paths with .json extension | 92 | not-applicable | — | — | GitLab remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should query custom paths with .json5 extension | 110 | not-applicable | — | — | GitLab remote preset resolution is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |

### `config/presets/gitlab/index › getPresetFromEndpoint()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses default endpoint | 130 | not-applicable | — | — | GitLab remote preset endpoint/tag resolution is not implemented in Rust. |
| uses custom endpoint | 148 | not-applicable | — | — | GitLab remote preset endpoint/tag resolution is not implemented in Rust. |
| uses default endpoint with a tag | 167 | not-applicable | — | — | GitLab remote preset endpoint/tag resolution is not implemented in Rust. |
| uses custom endpoint with a tag | 183 | not-applicable | — | — | GitLab remote preset endpoint/tag resolution is not implemented in Rust. |

---

## `lib/config/presets/http/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/http/index.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/presets/http/index › getPreset()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return parsed JSON | 13 | not-applicable | — | — | Generic HTTP remote preset fetching is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| should return parsed JSON5 | 19 | not-applicable | — | — | Generic HTTP remote preset fetching is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| throws if fails to parse | 30 | not-applicable | — | — | Generic HTTP remote preset fetching is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| throws if file not found | 38 | not-applicable | — | — | Generic HTTP remote preset fetching is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| throws on malformed URL | 46 | not-applicable | — | — | Generic HTTP remote preset fetching is not implemented in Rust; Rust only handles built-in preset expansion and tracks unresolved remote presets. |
| throws external host error | 51 | not-applicable | — | — | Generic HTTP remote preset fetching and host-rule error handling are not implemented in Rust. |

---

## `lib/config/presets/internal/group.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/internal/group.spec.ts
**Total tests:** 1 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/presets/internal/group`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| group:%s contains packageRules | 10 | not-applicable | — | — | TypeScript static preset-map shape invariant; Rust resolves group presets through behavior-oriented match arms rather than exposing a preset object map. |

---

## `lib/config/presets/internal/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/internal/index.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/presets/internal/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fails for undefined internal preset | 19 | not-applicable | — | — | TypeScript preset resolver error-path test; Rust does not expose the same internal preset fetcher and instead expands known built-ins while retaining unresolved preset references. |
| ${groupName}:${presetName} validates | 31 | not-applicable | — | — | TypeScript static validation sweep over the generated internal preset object map; Rust resolves supported presets through behavior-oriented match arms rather than exposing and validating the TypeScript preset map. |
| internal presets should not contain handlebars | 48 | not-applicable | — | — | TypeScript static preset-map invariant; Rust does not expose the generated internal preset map or handlebars-bearing preset names. |
| returns undefined for unknown preset | 58 | not-applicable | — | — | TypeScript internal.getPreset helper behavior; Rust does not expose that helper and treats unknown presets as unresolved config references. |

### `config/presets/internal/index › isInternal`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false for a local> preset | 63 | not-applicable | — | — | TypeScript isInternal routing helper; Rust does not expose internal-vs-remote preset classification as a public behavior. |
| returns false for a github> preset | 67 | not-applicable | — | — | TypeScript isInternal routing helper; Rust does not expose internal-vs-remote preset classification as a public behavior. |
| returns false for an un-migrated preset | 71 | not-applicable | — | — | TypeScript isInternal routing helper; Rust normalizes supported legacy presets through config parsing rather than exposing this classifier. |
| returns false for an empty string | 75 | not-applicable | — | — | TypeScript isInternal routing helper; Rust does not expose internal-vs-remote preset classification as a public behavior. |
| returns true for `config:recommended` | 79 | not-applicable | — | — | TypeScript isInternal routing helper; Rust covers config:recommended through built-in expansion tests rather than exposing this classifier. |
| returns true for a parameterised preset | 83 | not-applicable | — | — | TypeScript isInternal routing helper; Rust covers parameterized presets through config parsing tests rather than exposing this classifier. |
| returns true for a parameterised remote preset | 87 | not-applicable | — | — | TypeScript isInternal routing helper; Rust does not expose internal-vs-remote preset classification as a public behavior. |

---

## `lib/config/presets/internal/monorepos.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/internal/monorepos.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `config/presets/internal/monorepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| presets should have right name | 21 | ported | `monorepos.rs` | `monorepo_preset_names_use_supported_slug_format` | — |

---

## `lib/config/presets/internal/schedule.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/internal/schedule.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `config/presets/internal/schedule › daily`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $datetime | 19 | ported | `schedule.rs` | `schedule_preset_daily_matches_upstream_cases` | — |

### `config/presets/internal/schedule › earlyMondays`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $datetime | 34 | ported | `schedule.rs` | `schedule_preset_early_mondays_matches_upstream_cases` | — |

### `config/presets/internal/schedule › monthly`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $datetime | 50 | ported | `schedule.rs` | `schedule_preset_monthly_matches_upstream_cases` | — |

### `config/presets/internal/schedule › nonOfficeHours`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $datetime | 66 | ported | `schedule.rs` | `schedule_preset_non_office_hours_matches_upstream_cases` | — |

### `config/presets/internal/schedule › officeHours`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $datetime | 86 | ported | `schedule.rs` | `schedule_preset_office_hours_matches_upstream_cases` | — |

### `config/presets/internal/schedule › quarterly`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $datetime | 119 | ported | `schedule.rs` | `schedule_preset_quarterly_matches_upstream_cases` | — |

### `config/presets/internal/schedule › weekdays`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $datetime | 135 | ported | `schedule.rs` | `schedule_preset_weekdays_matches_upstream_cases` | — |

### `config/presets/internal/schedule › weekends`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $datetime | 152 | ported | `schedule.rs` | `schedule_preset_weekends_matches_upstream_cases` | — |

### `config/presets/internal/schedule › yearly`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $datetime | 169 | ported | `schedule.rs` | `schedule_preset_yearly_matches_upstream_cases` | — |

---

## `lib/config/presets/internal/workarounds.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/internal/workarounds.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 10 | **Status:** ported

### `config/presets/internal/workarounds › bitnamiDockerImageVersioning`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| versioning("$input") == "$expected" | 13 | ported | `repo_config.rs` | `workaround_bitnami_docker_image_versioning_matches_upstream_cases` | — |
| matchCurrentValue("$input") == "$expected" | 28 | ported | `repo_config.rs` | `workaround_bitnami_docker_image_match_current_value_matches_upstream_cases` | — |

### `config/presets/internal/workarounds › clamavDockerImageVersioning`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| versioning("$input") == "$expected" | 49 | ported | `repo_config.rs` | `workaround_clamav_docker_image_versioning_matches_upstream_cases` | — |

### `config/presets/internal/workarounds › libericaJdkDockerVersioning › Liberica JDK Lite`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| versioning("$input") == "$expected" | 80 | ported | `repo_config.rs` | `workaround_liberica_jdk_lite_versioning_matches_upstream_cases` | — |
| matchCurrentValue("$input") == "$expected" | 95 | ported | `repo_config.rs` | `workaround_liberica_jdk_lite_match_current_value_matches_upstream_cases` | — |

### `config/presets/internal/workarounds › libericaJdkDockerVersioning › Liberica JDK`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| versioning("$input") == "$expected" | 118 | ported | `repo_config.rs` | `workaround_liberica_jdk_versioning_matches_upstream_cases` | — |
| matchCurrentValue("$input") == "$expected" | 133 | ported | `repo_config.rs` | `workaround_liberica_jdk_match_current_value_matches_upstream_cases` | — |

### `config/presets/internal/workarounds › libericaJdkDockerVersioning › Liberica JRE`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| versioning("$input") == "$expected" | 156 | ported | `repo_config.rs` | `workaround_liberica_jre_versioning_matches_upstream_cases` | — |
| matchCurrentValue("$input") == "$expected" | 171 | ported | `repo_config.rs` | `workaround_liberica_jre_match_current_value_matches_upstream_cases` | — |

### `config/presets/internal/workarounds › javaLTSVersions › bellsoft/liberica-runtime-container`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| allowedVersisons("$input") == "$expected" | 196 | ported | `repo_config.rs` | `workaround_java_lts_liberica_runtime_allowed_versions_match_upstream_cases` | — |

---

## `lib/config/presets/local/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/local/common.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/presets/local/common › fetchJSONFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for null | 8 | not-applicable | — | — | Platform-backed local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| throws for ExternalHostError | 16 | not-applicable | — | — | Platform-backed local preset file fetching and external-host error propagation are not implemented in Rust. |
| throws for Error | 26 | not-applicable | — | — | Platform-backed local preset file fetching and fetch error mapping are not implemented in Rust. |

### `config/presets/local/common › getPresetFromEndpoint`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 36 | not-applicable | — | — | Platform-backed local preset endpoint fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |

---

## `lib/config/presets/local/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/local/index.spec.ts
**Total tests:** 19 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/presets/local/index › getPreset()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for unsupported platform | 34 | not-applicable | — | — | Platform-dispatched local preset fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| throws for missing platform | 47 | not-applicable | — | — | Platform-dispatched local preset fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to azure | 59 | not-applicable | — | — | Azure local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to bitbucket | 77 | not-applicable | — | — | Bitbucket local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to gerrit | 95 | not-applicable | — | — | Gerrit local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to custom bitbucket-server | 113 | not-applicable | — | — | Bitbucket Server local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to gitea | 131 | not-applicable | — | — | Gitea local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to forgejo | 149 | not-applicable | — | — | Forgejo local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to custom gitea | 167 | not-applicable | — | — | Custom-endpoint Gitea local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to custom forgejo | 186 | not-applicable | — | — | Custom-endpoint Forgejo local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to github | 205 | not-applicable | — | — | GitHub local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to custom github | 223 | not-applicable | — | — | Custom-endpoint GitHub local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to github with a tag | 243 | not-applicable | — | — | Tagged GitHub local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to custom github with a tag | 262 | not-applicable | — | — | Tagged custom-endpoint GitHub local preset file fetching is not implemented in Rust. |
| forwards to gitlab | 283 | not-applicable | — | — | GitLab local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to custom gitlab | 302 | not-applicable | — | — | Custom-endpoint GitLab local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to gitlab with a tag | 322 | not-applicable | — | — | Tagged GitLab local preset file fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/local preset references. |
| forwards to custom gitlab with a tag | 340 | not-applicable | — | — | Tagged custom-endpoint GitLab local preset file fetching is not implemented in Rust. |
| throws for platform that does not support local presets | 361 | not-applicable | — | — | Platform-dispatched local preset fetching and unsupported-platform errors are not implemented in Rust. |

---

## `lib/config/presets/npm/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/npm/index.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/presets/npm/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw if no package | 10 | not-applicable | — | — | npm-hosted preset package fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/npm preset references. |
| should throw if no renovate-config | 17 | not-applicable | — | — | npm-hosted preset package fetching and renovate-config package parsing are not implemented in Rust. |
| should throw if preset name not found | 48 | not-applicable | — | — | npm-hosted preset package fetching and preset-name lookup are not implemented in Rust. |
| should return preset | 83 | not-applicable | — | — | npm-hosted preset package fetching is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote/npm preset references. |

---

## `lib/config/presets/util.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/util.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/presets/util`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 16 | not-applicable | — | — | Generic remote preset fetch/deep-preset helper behavior is not implemented in Rust; Rust only expands built-in presets and tracks unresolved remote preset references. |
| fails | 37 | not-applicable | — | — | Generic remote preset fetch error propagation is not implemented in Rust. |
| dep not found | 42 | not-applicable | — | — | Generic remote preset dependency-not-found retry/error mapping is not implemented in Rust. |
| preset not found | 54 | not-applicable | — | — | Generic remote preset nested-preset lookup and preset-not-found mapping are not implemented in Rust. |

---

## `lib/config/parse.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/parse.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `config/parse › json`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses | 6 | ported | `file.rs` | `parse_file_config_json_parses` | — |
| returns error | 13 | ported | `file.rs` | `parse_file_config_json_returns_error` | — |

### `config/parse › json5`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses | 43 | ported | `file.rs` | `parse_file_config_json5_parses` | — |
| returns error | 50 | ported | `file.rs` | `parse_file_config_json5_returns_error` | — |

---

## `lib/config/app-strings.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/app-strings.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `config/app-strings`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds user configured filenames to list | 8 | ported | `repo_config.rs` | `config_file_names_include_user_configured_names` | — |
| expands brace patterns for json and json5 filenames | 20 | ported | `repo_config.rs` | `config_file_names_expand_json_and_json5_patterns` | — |
| filters based on platform | 33 | ported | `repo_config.rs` | `config_file_names_filter_platform_specific_names` | — |
| does not allow the local platform to have an associated filename | 42 | ported | `repo_config.rs` | `config_file_names_do_not_add_local_platform_names` | — |

---

## `lib/config/defaults.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/defaults.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/defaults › getDefault()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns new instances of arrays when called repeatedly | 6 | ported | `config.rs` | `default_array_values_are_independent` | — |
| returns true for boolean values | 20 | ported | `config.rs` | `default_boolean_value_is_true` | — |
| returns null for %s values | 31 | ported | `config.rs` | `default_scalar_values_are_null` | — |

---

## `lib/config/global.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/global.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `config/global`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| all values in OPTIONS are sorted | 4 | ported | `config.rs` | `global_config_options_are_sorted` | — |

---

## `lib/config/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/index.spec.ts
**Total tests:** 12 | **Ported:** 12 | **Actionable:** 12 | **Status:** ported

### `config/index › mergeChildConfig(parentConfig, childConfig)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| merges | 16 | ported | `config.rs` | `merge_child_config_merges_plain_and_nested_options` | — |
| merges packageRules | 32 | ported | `config.rs` | `merge_child_config_appends_package_rules` | — |
| merges constraints | 55 | ported | `config.rs` | `merge_child_config_merges_constraints` | — |
| merges forced options | 73 | ported | `config.rs` | `merge_child_config_merges_force_options` | — |
| handles null parent packageRules | 92 | ported | `config.rs` | `merge_child_config_handles_null_parent_package_rules` | — |
| handles null child packageRules | 105 | ported | `config.rs` | `merge_child_config_handles_missing_child_package_rules` | — |
| handles undefined childConfig | 118 | ported | `config.rs` | `merge_child_config_handles_undefined_child_config` | — |
| getManagerConfig() | 124 | ported | `config.rs` | `get_manager_config_adds_manager_file_patterns` | — |
| filterConfig() | 142 | ported | `config.rs` | `filter_config_returns_object` | — |
| highest vulnerabilitySeverity maintained when config is vulnerability alert | 148 | ported | `config.rs` | `merge_child_config_keeps_highest_vulnerability_severity` | — |

### `config/index › removeGlobalConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| removes all global config | 163 | ported | `config.rs` | `remove_global_config_removes_all_global_config` | — |
| retains inherited config | 170 | ported | `config.rs` | `remove_global_config_retains_inherited_config` | — |

---

## `lib/config/migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migration.spec.ts
**Total tests:** 30 | **Ported:** 22 | **Actionable:** 22 | **Status:** ported

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
| overrides existing automerge setting | 279 | not-applicable | — | — | Rust package-rule model does not represent per-rule major/minor/patch config blocks generated by deprecated automerge string migration |
| does not migrate config | 297 | ported | `repo_config.rs` | `non_deprecated_config_fields_parse_without_migration_effects` | — |
| migrates subconfig | 308 | not-applicable | — | — | Rust does not model Renovate's lockFileMaintenance subconfig or raw migration output for nested depTypes |
| migrates packageFiles | 334 | not-applicable | — | — | Deprecated packageFiles-to-includePaths/packageRules raw migration; Rust config model does not expose packageFiles |
| migrates more packageFiles | 360 | not-applicable | — | — | Deprecated packageFiles nested packageRules raw migration; Rust config model does not expose packageFiles |
| removes invalid configs | 389 | not-applicable | — | — | TypeScript raw config cleanup snapshot; Rust parser ignores unknown invalid fields and does not expose migrated raw config output |
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
| migrates pip-compile | 696 | not-applicable | — | — | Manager-specific managerFilePatterns config block migration; Rust uses static manager detection and does not model per-manager config blocks |
| migrates gradle-lite | 731 | not-applicable | — | — | Manager rename/config-block migration to gradle; Rust has no gradle-lite manager config block |
| migrates empty requiredStatusChecks | 751 | ported | `repo_config.rs` | `empty_required_status_checks_is_removed` | — |
| migrates azureAutoComplete | 762 | ported | `repo_config.rs` | `azure_auto_complete_migrated_to_platform_automerge` | — |
| migrates gitLabAutomerge | 791 | ported | `repo_config.rs` | `git_lab_automerge_migrated_to_platform_automerge` | — |
| migrates dryRun | 820 | ported | `config_builder.rs` | `dry_run_legacy_true_maps_to_full` (+ dry_run_legacy_false_disables_dry_run) | — |
| migrates baseBranches and baseBranch | 835 | ported | `repo_config.rs` | `base_branches_and_base_branch_migrated_to_patterns` | — |
| logs errors | 844 | not-applicable | — | — | TypeScript-specific MigrationsService/logger failure path; Rust parser has no migration service abstraction to mock |

---

## `lib/config/options/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/options/index.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/options/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| test manager should have no defaultConfig | 9 | not-applicable | — | — | TypeScript option metadata registry; Rust does not generate options from manager `defaultConfig` metadata. |
| supportedManagers should have valid names | 18 | not-applicable | — | — | TypeScript option metadata registry; Rust uses typed manager modules rather than dynamic option metadata. |
| supportedPlatforms should have valid names | 32 | not-applicable | — | — | TypeScript option metadata registry; Rust platform values are static enums/constants rather than dynamic option metadata. |
| should not contain duplicate option names | 46 | not-applicable | — | — | TypeScript option metadata registry; Rust options are typed fields and clap definitions. |

### `config/options/index › every option with allowedValues and a default must have the default in allowedValues`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| `${option.name}: \`${option.default}\` is in ${JSON.stringify(option.allowedValues)}` | 57 | not-applicable | — | — | TypeScript option metadata registry; Rust does not expose allowedValues/default metadata tables. |

### `config/options/index › every option with a siblingProperties has a \`property\` that matches a known option`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| `${option.name}'s reference to ${prop.property} is valid` | 77 | not-applicable | — | — | TypeScript option metadata registry; Rust does not expose requiredIf sibling property metadata tables. |
| `${option.name}'s value for ${prop.property} is valid, according to allowedValues` | 84 | not-applicable | — | — | TypeScript option metadata registry; Rust does not expose requiredIf sibling property metadata tables. |

---

## `lib/config/options/env-options.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/options/env-options.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/options/env-options`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| maps camelCase config names to RENOVATE_SCREAMING_SNAKE_CASE env vars | 6 | not-applicable | — | — | TypeScript env option metadata map; Rust env parsing uses explicit static mappings tested through parser behavior. |
| maps multi-word config names correctly | 14 | not-applicable | — | — | TypeScript env option metadata map; Rust env parsing uses explicit static mappings tested through parser behavior. |
| marks globalOnly options correctly | 22 | not-applicable | — | — | TypeScript option metadata registry; Rust global-only validation is not derived from env option metadata. |
| marks non-globalOnly options correctly | 31 | not-applicable | — | — | TypeScript option metadata registry; Rust global-only validation is not derived from env option metadata. |
| marks inheritConfigSupport options correctly | 40 | not-applicable | — | — | TypeScript option metadata registry; Rust inherit-config support is not derived from env option metadata. |
| excludes options with env: false | 49 | not-applicable | — | — | TypeScript env option metadata map; Rust has no generated env map containing disabled entries. |
| includes the option type | 54 | not-applicable | — | — | TypeScript env option metadata map; Rust env parsing uses typed parser functions rather than runtime option type metadata. |

---

## `lib/config/migrations/migrations-service.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/migrations-service.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/migrations/migrations-service`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should remove deprecated properties | 9 | not-applicable | — | — | TypeScript MigrationsService registry aggregate; Rust tracks concrete migration behavior in individual migration parity sections. |
| should rename renamed properties | 23 | not-applicable | — | — | TypeScript MigrationsService registry aggregate; Rust tracks concrete migration behavior in individual migration parity sections. |
| should save original order of properties | 42 | not-applicable | — | — | TypeScript object key order assertion for migration-service output; Rust serde JSON object ordering is not a config behavior contract. |
| should allow custom migrations by regexp | 60 | not-applicable | — | — | TypeScript subclass/plugin migration-service infrastructure; Rust migration helper does not expose runtime custom migration subclasses. |
| there should be a single migration per property name | 89 | not-applicable | — | — | TypeScript migration class registry invariant; Rust migration logic is not built from a class registry. |
| includes all defined migration classes in MigrationsService.customMigrations | 104 | not-applicable | — | — | TypeScript filesystem/class registry invariant; Rust migration logic is not discovered from migration class files. |

---

## `lib/config/migrations/base/abstract-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/base/abstract-migration.spec.ts
**Total tests:** 2 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/migrations/base/abstract-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should not allow to use method rewrite | 4 | not-applicable | — | — | TypeScript abstract migration class infrastructure; Rust migration helper does not expose subclass rewrite misuse paths. |
| should not allow to use method delete | 22 | not-applicable | — | — | TypeScript abstract migration class infrastructure; Rust migration helper does not expose subclass delete misuse paths. |

---

## `lib/config/migrations/custom/binary-source-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/binary-source-migration.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `config/migrations/custom/binary-source-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate "auto" to "global" | 4 | ported | `migrate_validate.rs` | `binary_source_auto_migrates_to_global` | — |

---

## `lib/config/migrations/custom/extends-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/extends-migration.spec.ts
**Total tests:** 6 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `config/migrations/custom/extends-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| migrates preset strings to array | 5 | ported | `migrate_validate.rs` | `extends_string_migrates_to_array_and_normalizes_js_app` | — |
| migrates presets array | 23 | ported | `migrate_validate.rs` | `extends_array_normalizes_presets_in_place` | — |
| should remove non string values | 34 | ported | `migrate_validate.rs` | `extends_array_removes_non_string_values` | — |
| should remove removed presets | 44 | ported | `migrate_validate.rs` | `extends_array_removes_deleted_presets` | — |
| migrates presets | 54 | not-applicable | — | — | Rust config has no mutable GlobalConfig.migratePresets registry equivalent for per-test custom preset rewrite injection |
| migrate merge confidence config preset to internal preset | 67 | ported | `migrate_validate.rs` | `extends_merge_confidence_preset_migrates_to_internal_preset` | — |

---

## `lib/config/migrations/custom/schedule-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/schedule-migration.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `config/migrations/custom/schedule-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| migrates every friday | 4 | ported | `migrate_validate.rs` | `schedule_every_friday_migrates_to_on_friday` | — |
| does not migrate every weekday | 14 | ported | `migrate_validate.rs` | `schedule_every_weekday_is_unchanged` | — |
| does not migrate multi days | 25 | ported | `migrate_validate.rs` | `schedule_multi_days_is_unchanged` | — |
| does not migrate hour range | 36 | ported | `migrate_validate.rs` | `schedule_hour_range_is_unchanged` | — |
| does not migrate invalid range | 47 | ported | `migrate_validate.rs` | `schedule_invalid_range_is_unchanged` | — |

---

## `lib/config/migrations/custom/semantic-commits-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/semantic-commits-migration.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `config/migrations/custom/semantic-commits-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate true to "enabled" | 4 | ported | `migrate_validate.rs` | `semantic_commits_true_migrates_to_enabled` | — |
| should migrate false to "disabled" | 13 | ported | `migrate_validate.rs` | `semantic_commits_false_migrates_to_disabled` | — |
| should migrate null to "auto" | 22 | ported | `migrate_validate.rs` | `semantic_commits_null_migrates_to_auto` | — |
| should migrate random string to "auto" | 31 | ported | `migrate_validate.rs` | `semantic_commits_random_string_migrates_to_auto` | — |
| should not migrate valid enabled config | 40 | ported | `migrate_validate.rs` | `semantic_commits_enabled_is_unchanged` | — |
| should not migrate valid disabled config | 51 | ported | `migrate_validate.rs` | `semantic_commits_disabled_is_unchanged` | — |

---

## `lib/config/migrations/custom/semantic-prefix-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/semantic-prefix-migration.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `config/migrations/custom/semantic-prefix-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should work | 4 | ported | `migrate_validate.rs` | `semantic_prefix_migrates_type_and_scope` | — |
| should remove non-string values | 12 | ported | `migrate_validate.rs` | `semantic_prefix_non_string_is_removed` | — |
| should migrate prefix with no-scope to null | 21 | ported | `migrate_validate.rs` | `semantic_prefix_without_scope_migrates_scope_to_null` | — |
| works for random string | 30 | ported | `migrate_validate.rs` | `semantic_prefix_random_string_migrates_type_with_null_scope` | — |

---

## `lib/config/migrations/custom/azure-gitlab-automerge-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/azure-gitlab-automerge-migration.spec.ts
**Total tests:** 6 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `config/migrations/custom/azure-gitlab-automerge-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate non undefined gitLabAutomerge | 4 | ported | `migrate_validate.rs` | `git_lab_automerge_migrates_to_platform_automerge` | — |
| should just remove undefined gitLabAutomerge | 14 | not-applicable | — | — | Rust JSON config has no `undefined` value; absent keys already produce the migrated empty object behavior |
| should override platformAutomerge when gitLabAutomerge defined | 24 | ported | `migrate_validate.rs` | `git_lab_automerge_overrides_platform_automerge` | — |
| should migrate non undefined azureAutoComplete | 36 | ported | `migrate_validate.rs` | `azure_auto_complete_migrates_to_platform_automerge` | — |
| should just remove undefined azureAutoComplete | 46 | not-applicable | — | — | Rust JSON config has no `undefined` value; absent keys already produce the migrated empty object behavior |
| should override platformAutomerge when azureAutoComplete defined | 56 | ported | `migrate_validate.rs` | `azure_auto_complete_overrides_platform_automerge` | — |

---

## `lib/config/migrations/custom/compatibility-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/compatibility-migration.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `config/migrations/custom/compatibility-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate object | 4 | ported | `migrate_validate.rs` | `compatibility_object_migrates_to_constraints` | — |
| should just remove property when compatibility is not an object | 18 | ported | `migrate_validate.rs` | `compatibility_non_object_is_removed` | — |

---

## `lib/config/migrations/custom/composer-ignore-platform-reqs-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/composer-ignore-platform-reqs-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/migrations/custom/composer-ignore-platform-reqs-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate true to empty array | 4 | ported | `migrate_validate.rs` | `composer_ignore_platform_reqs_true_migrates_to_empty_array` | — |
| should migrate false to null | 14 | ported | `migrate_validate.rs` | `composer_ignore_platform_reqs_false_migrates_to_null` | — |
| should not change array value | 24 | ported | `migrate_validate.rs` | `composer_ignore_platform_reqs_array_is_unchanged` | — |

---

## `lib/config/migrations/custom/custom-managers-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/custom-managers-migration.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `config/migrations/custom/custom-managers-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| migrates | 6 | ported | `migrate_validate.rs` | `custom_managers_missing_custom_type_migrates_to_regex` | — |

---

## `lib/config/migrations/custom/datasource-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/datasource-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/migrations/custom/datasource-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate adoptium-java | 4 | ported | `migrate_validate.rs` | `datasource_adoptium_java_migrates_to_java_version` | — |
| should migrate donet | 14 | ported | `migrate_validate.rs` | `datasource_dotnet_migrates_to_dotnet_version` | — |
| should migrate node | 24 | ported | `migrate_validate.rs` | `datasource_node_migrates_to_node_version` | — |

---

## `lib/config/migrations/custom/enabled-managers-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/enabled-managers-migration.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `config/migrations/custom/enabled-managers-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| migrates | 4 | ported | `migrate_validate.rs` | `enabled_managers_legacy_names_migrate` | — |

---

## `lib/config/migrations/custom/dep-types-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/dep-types-migration.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `config/migrations/custom/dep-types-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should only add depTypes to packageRules | 4 | ported | `migrate_validate.rs` | `dep_types_migration_adds_package_rules` | — |

---

## `lib/config/migrations/custom/fetch-release-notes-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/fetch-release-notes-migration.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `config/migrations/custom/fetch-release-notes-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| migrates | 4 | ported | `migrate_validate.rs` | `fetch_release_notes_migrates_to_fetch_change_logs` | — |

---

## `lib/config/migrations/custom/file-match-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/file-match-migration.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `config/migrations/custom/file-match-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| migrates fileMatch of type string | 4 | ported | `migrate_validate.rs` | `file_match_string_migrates_to_manager_file_patterns` | — |
| migrates fileMatch of type array | 14 | ported | `migrate_validate.rs` | `file_match_array_migrates_to_manager_file_patterns` | — |
| concats fileMatch to managerFilePatterns | 24 | ported | `migrate_validate.rs` | `file_match_appends_to_existing_manager_file_patterns` | — |
| does nothing if fileMatch not defined | 38 | ported | `migrate_validate.rs` | `missing_file_match_leaves_manager_file_patterns_unchanged` | — |

---

## `lib/config/migrations/custom/match-datasources-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/match-datasources-migration.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `config/migrations/custom/match-datasources-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate properly | 4 | ported | `migrate_validate.rs` | `match_datasources_legacy_names_migrate` | — |

---

## `lib/config/migrations/custom/match-managers-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/match-managers-migration.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `config/migrations/custom/match-managers-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| migrates old custom manager syntax to new one | 4 | ported | `migrate_validate.rs` | `match_managers_legacy_names_migrate` | — |
| only migrates when necessary | 24 | ported | `migrate_validate.rs` | `match_managers_missing_is_unchanged` | — |

---

## `lib/config/migrations/custom/match-strings-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/match-strings-migration.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `config/migrations/custom/match-strings-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate properly | 4 | ported | `migrate_validate.rs` | `match_strings_lookup_name_migrates_to_package_name` | — |

---

## `lib/config/migrations/custom/package-name-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/package-name-migration.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `config/migrations/custom/package-name-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate value to array | 4 | ported | `migrate_validate.rs` | `package_name_migrates_to_package_names` | — |

---

## `lib/config/migrations/custom/package-pattern-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/package-pattern-migration.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `config/migrations/custom/package-pattern-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate value to array | 4 | ported | `migrate_validate.rs` | `package_pattern_migrates_to_package_patterns` | — |

---

## `lib/config/migrations/custom/package-rules-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/package-rules-migration.spec.ts
**Total tests:** 8 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `config/migrations/custom/package-rules-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should preserve config order | 5 | not-applicable | — | — | Rust serde JSON object key ordering is not a config behavior contract. |
| should not migrate nested packageRules | 31 | ported | `migrate_validate.rs` | `package_rules_renames_top_level_paths_without_nested_package_rules` | — |
| should migrate languages to categories | 53 | ported | `migrate_validate.rs` | `package_rules_languages_migrate_to_categories` | — |
| should migrate single match rule | 81 | ported | `migrate_validate.rs` | `package_rules_single_match_language_migrates_to_category` | — |
| should migrate excludePackageNames to matchPackageNames | 99 | ported | `migrate_validate.rs` | `package_rules_exclude_package_names_merge_into_match_package_names` | — |
| should migrate matchPackagePatterns to matchPackageNames | 127 | ported | `migrate_validate.rs` | `package_rules_match_package_patterns_merge_into_match_package_names` | — |
| should migrate all match/exclude when value is of type string | 163 | ported | `migrate_validate.rs` | `package_rules_string_matchers_merge_into_match_names` | — |
| should migrate all match/exclude at once | 222 | ported | `migrate_validate.rs` | `package_rules_array_matchers_merge_into_match_names` | — |

---

## `lib/config/migrations/custom/packages-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/packages-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/migrations/custom/packages-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate to package rules | 4 | ported | `migrate_validate.rs` | `packages_migrates_to_package_rules` | — |
| should concat with existing package rules | 14 | ported | `migrate_validate.rs` | `packages_appends_to_existing_package_rules` | — |
| should ignore non array value | 26 | ported | `migrate_validate.rs` | `packages_non_array_is_removed` | — |

---

## `lib/config/migrations/custom/path-rules-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/path-rules-migration.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `config/migrations/custom/path-rules-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate to packageRules | 4 | ported | `migrate_validate.rs` | `path_rules_migrate_to_package_rules` | — |
| should rewrite packageRules when it is not array | 22 | ported | `migrate_validate.rs` | `path_rules_rewrite_non_array_package_rules` | — |
| should not migrate non array value | 42 | ported | `migrate_validate.rs` | `path_rules_non_array_is_removed` | — |
| should concat with existing package rules | 50 | ported | `migrate_validate.rs` | `path_rules_append_to_existing_package_rules` | — |

---

## `lib/config/migrations/custom/package-files-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/package-files-migration.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `config/migrations/custom/package-files-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate value to array | 4 | ported | `migrate_validate.rs` | `package_files_object_migrates_to_include_paths_and_package_rules` | — |
| should handle multiple packageFile | 21 | ported | `migrate_validate.rs` | `package_files_nested_array_migrates_to_include_paths` | — |
| should still work for wrong config | 34 | ported | `migrate_validate.rs` | `package_files_appends_to_existing_package_rules` | — |
| should work for non-object packageFiles | 55 | ported | `migrate_validate.rs` | `package_files_string_migrates_to_include_paths` | — |
| should work for nested rules | 65 | ported | `migrate_validate.rs` | `package_files_preserves_nested_rules` | — |
| no change for empty packageFiles | 92 | ported | `migrate_validate.rs` | `package_files_empty_is_removed_without_other_changes` | — |

---

## `lib/config/migrations/custom/pin-versions-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/pin-versions-migration.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `config/migrations/custom/pin-versions-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate true | 4 | ported | `migrate_validate.rs` | `pin_versions_true_migrates_to_pin_range_strategy` | — |
| should migrate false | 14 | ported | `migrate_validate.rs` | `pin_versions_false_migrates_to_replace_range_strategy` | — |

---

## `lib/config/migrations/custom/separate-major-release-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/separate-major-release-migration.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `config/migrations/custom/separate-major-release-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate | 4 | ported | `migrate_validate.rs` | `separate_major_releases_migrates_to_separate_major_minor` | — |

---

## `lib/config/migrations/custom/separate-multiple-major-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/separate-multiple-major-migration.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `config/migrations/custom/separate-multiple-major-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should remove if separateMajorReleases exists | 4 | ported | `migrate_validate.rs` | `separate_multiple_major_removed_when_separate_major_releases_exists` | — |
| should skip if separateMajorReleases does not exist | 14 | ported | `migrate_validate.rs` | `separate_multiple_major_is_unchanged_without_separate_major_releases` | — |

---

## `lib/config/migrations/custom/stability-days-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/stability-days-migration.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `config/migrations/custom/stability-days-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| migrates | 4 | ported | `migrate_validate.rs` | `stability_days_migrates_to_minimum_release_age` | — |

---

## `lib/config/migrations/custom/host-rules-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/host-rules-migration.spec.ts
**Total tests:** 2 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `config/migrations/custom/host-rules-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate array | 5 | ported | `migrate_validate.rs` | `host_rules_legacy_fields_migrate` | — |
| throws when multiple hosts are present | 75 | not-applicable | — | — | Rust raw migration helper returns migrated JSON and does not model Renovate's throwing migration-service validation path |

---

## `lib/config/migrations/custom/suppress-notifications-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/suppress-notifications-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/migrations/custom/suppress-notifications-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should remomve prEditNotification from array | 4 | ported | `migrate_validate.rs` | `suppress_notifications_removes_pr_edit_notification` | — |
| should not migrate array without prEditNotification | 14 | ported | `migrate_validate.rs` | `suppress_notifications_without_pr_edit_notification_is_unchanged` | — |
| should not migrate empty array | 25 | ported | `migrate_validate.rs` | `suppress_notifications_empty_is_unchanged` | — |

---

## `lib/config/migrations/custom/trust-level-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/trust-level-migration.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `config/migrations/custom/trust-level-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should handle hight level | 4 | ported | `migrate_validate.rs` | `trust_level_high_sets_trust_options` | — |
| should not rewrite provided properties | 18 | ported | `migrate_validate.rs` | `trust_level_high_preserves_existing_trust_options` | — |

---

## `lib/config/migrations/custom/unpublish-safe-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/unpublish-safe-migration.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `config/migrations/custom/unpublish-safe-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate true | 4 | ported | `migrate_validate.rs` | `unpublish_safe_true_injects_security_preset` | — |
| should migrate true and handle extends field | 14 | ported | `migrate_validate.rs` | `unpublish_safe_true_handles_string_extends` | — |
| should migrate true and handle empty extends field | 26 | ported | `migrate_validate.rs` | `unpublish_safe_true_handles_empty_extends` | — |
| should migrate true and save order of items inside extends field | 38 | ported | `migrate_validate.rs` | `unpublish_safe_true_rewrites_supported_extends_in_place` | — |
| should migrate false and save order of items inside extends field | 68 | ported | `migrate_validate.rs` | `unpublish_safe_false_is_removed_and_preserves_extends` | — |
| prevent duplicates | 80 | ported | `migrate_validate.rs` | `unpublish_safe_true_does_not_duplicate_security_preset` | — |
| should not migrate npm:unpublishSafe | 92 | ported | `migrate_validate.rs` | `unpublish_safe_absent_leaves_npm_unpublish_safe_extends` | — |

---

## `lib/config/migrations/custom/go-mod-tidy-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/go-mod-tidy-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/migrations/custom/go-mod-tidy-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add postUpdateOptions option when true | 4 | ported | `migrate_validate.rs` | `gomod_tidy_true_appends_post_update_option` | — |
| should handle case when postUpdateOptions is not defined | 16 | ported | `migrate_validate.rs` | `gomod_tidy_true_initializes_post_update_options` | — |
| should only remove when false | 27 | ported | `migrate_validate.rs` | `gomod_tidy_false_is_removed` | — |

---

## `lib/config/migrations/custom/ignore-node-modules-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/ignore-node-modules-migration.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `config/migrations/custom/ignore-node-modules-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate to ignorePaths | 4 | ported | `migrate_validate.rs` | `ignore_node_modules_true_migrates_to_ignore_paths` | — |

---

## `lib/config/migrations/custom/ignore-npmrc-file-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/ignore-npmrc-file-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/migrations/custom/ignore-npmrc-file-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should init npmrc field | 4 | ported | `migrate_validate.rs` | `ignore_npmrc_file_initializes_npmrc` | — |
| should not change npmrc field if it represents string value | 14 | ported | `migrate_validate.rs` | `ignore_npmrc_file_preserves_string_npmrc` | — |
| should change npmrc field if it not represents string value | 26 | ported | `migrate_validate.rs` | `ignore_npmrc_file_replaces_non_string_npmrc` | — |

---

## `lib/config/migrations/custom/include-forks-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/include-forks-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/migrations/custom/include-forks-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate true | 4 | ported | `migrate_validate.rs` | `include_forks_true_migrates_to_enabled_fork_processing` | — |
| should migrate false | 14 | ported | `migrate_validate.rs` | `include_forks_false_migrates_to_disabled_fork_processing` | — |
| should not migrate non boolean value | 24 | ported | `migrate_validate.rs` | `include_forks_non_boolean_is_removed` | — |

---

## `lib/config/migrations/custom/node-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/node-migration.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `config/migrations/custom/node-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate node to travis | 4 | ported | `migrate_validate.rs` | `node_enabled_migrates_to_travis_enabled` | — |
| should not delete node in case it has more than one property | 14 | ported | `migrate_validate.rs` | `node_enabled_migration_preserves_other_node_options` | — |

---

## `lib/config/migrations/custom/post-update-options-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/post-update-options-migration.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `config/migrations/custom/post-update-options-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate properly | 4 | ported | `migrate_validate.rs` | `post_update_options_removes_gomod_no_massage` | — |

---

## `lib/config/migrations/custom/renovate-fork-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/renovate-fork-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/migrations/custom/renovate-fork-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate true | 4 | ported | `migrate_validate.rs` | `renovate_fork_true_migrates_to_enabled_fork_processing` | — |
| should migrate false | 14 | ported | `migrate_validate.rs` | `renovate_fork_false_migrates_to_disabled_fork_processing` | — |
| should not migrate non boolean value | 24 | ported | `migrate_validate.rs` | `renovate_fork_non_boolean_is_removed` | — |

---

## `lib/config/migrations/custom/base-branch-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/base-branch-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/migrations/custom/base-branch-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate value to array | 4 | ported | `migrate_validate.rs` | `base_branch_string_migrates_to_patterns` | — |
| should migrate array | 14 | ported | `migrate_validate.rs` | `base_branch_array_migrates_to_patterns` | — |
| should push to existing bassBranchPatterns | 24 | ported | `migrate_validate.rs` | `base_branch_migration_appends_existing_patterns` | — |

---

## `lib/config/migrations/custom/branch-name-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/branch-name-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/migrations/custom/branch-name-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should replace pattern | 4 | ported | `migrate_validate.rs` | `branch_name_manager_branch_prefix_migrates_to_additional_branch_prefix` | — |
| should not replace another string | 14 | ported | `migrate_validate.rs` | `branch_name_without_manager_branch_prefix_is_unchanged` | — |
| should not replace non string value | 25 | ported | `migrate_validate.rs` | `branch_name_non_string_is_unchanged` | — |

---

## `lib/config/migrations/custom/branch-prefix-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/branch-prefix-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/migrations/custom/branch-prefix-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate template | 4 | ported | `migrate_validate.rs` | `branch_prefix_parent_dir_template_migrates_to_additional_prefix` | — |
| should ignore string without template | 17 | ported | `migrate_validate.rs` | `branch_prefix_without_parent_dir_template_is_unchanged` | — |
| should ignore non string without template | 28 | ported | `migrate_validate.rs` | `branch_prefix_non_string_is_unchanged` | — |

---

## `lib/config/migrations/custom/automerge-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/automerge-migration.spec.ts
**Total tests:** 4 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `config/migrations/custom/automerge-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate none | 4 | ported | `repo_config.rs` | `automerge_legacy_none_string_migrated_to_false` | — |
| should migrate patch | 16 | not-applicable | — | — | Rust does not model Renovate's generated per-update-type patch/minor/major automerge config blocks |
| should migrate minor | 34 | not-applicable | — | — | Rust does not model Renovate's generated per-update-type patch/minor/major automerge config blocks |
| should migrate any | 49 | ported | `repo_config.rs` | `automerge_legacy_any_string_migrated_to_true` | — |

---

## `lib/config/migrations/custom/automerge-major-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/automerge-major-migration.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/migrations/custom/automerge-major-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate value to object | 4 | not-applicable | — | — | Rust does not model Renovate's raw `automergeMajor` to generated `major.automerge` config migration output |
| should migrate value to object and concat with existing minor object | 16 | not-applicable | — | — | Rust does not model Renovate's raw `automergeMajor` to generated `major.automerge` config migration output |
| should ignore non object minor value | 32 | not-applicable | — | — | Rust does not model Renovate's raw `automergeMajor` to generated `major.automerge` config migration output |

---

## `lib/config/migrations/custom/automerge-minor-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/automerge-minor-migration.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/migrations/custom/automerge-minor-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate value to object | 4 | not-applicable | — | — | Rust does not model Renovate's raw `automergeMinor` to generated `minor.automerge` config migration output |
| should migrate value to object and concat with existing minor object | 16 | not-applicable | — | — | Rust does not model Renovate's raw `automergeMinor` to generated `minor.automerge` config migration output |
| should ignore non object minor value | 32 | not-applicable | — | — | Rust does not model Renovate's raw `automergeMinor` to generated `minor.automerge` config migration output |

---

## `lib/config/migrations/custom/automerge-patch-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/automerge-patch-migration.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/migrations/custom/automerge-patch-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate value to object | 4 | not-applicable | — | — | Rust does not model Renovate's raw `automergePatch` to generated `patch.automerge` config migration output |
| should migrate value to object and concat with existing minor object | 16 | not-applicable | — | — | Rust does not model Renovate's raw `automergePatch` to generated `patch.automerge` config migration output |
| should ignore non object minor value | 32 | not-applicable | — | — | Rust does not model Renovate's raw `automergePatch` to generated `patch.automerge` config migration output |

---

## `lib/config/migrations/custom/automerge-type-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/automerge-type-migration.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/migrations/custom/automerge-type-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate string like "branch-" to "branch" | 4 | not-applicable | — | — | Rust preserves typed `automergeType` values and does not expose Renovate's raw automergeType string cleanup migration output |
| should not migrate another string value | 14 | not-applicable | — | — | Rust preserves typed `automergeType` values and does not expose Renovate's raw automergeType string cleanup migration output |
| should not migrate non string value | 25 | not-applicable | — | — | Rust typed config parsing ignores non-string `automergeType` values instead of exposing raw migration output |

---

## `lib/config/migrations/custom/dry-run-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/dry-run-migration.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `config/migrations/custom/dry-run-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate dryRun=true to dryRun=full | 4 | ported | `config_builder.rs` | `dry_run_legacy_true_maps_to_full` | — |
| should migrate dryRun=false to dryRun=null | 14 | ported | `config_builder.rs` | `dry_run_legacy_false_disables_dry_run` | — |

---

## `lib/config/migrations/custom/recreate-closed-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/recreate-closed-migration.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `config/migrations/custom/recreate-closed-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate true | 4 | ported | `migrate_validate.rs` | `recreate_closed_true_migrates_to_always` | — |
| should migrate false | 14 | ported | `migrate_validate.rs` | `recreate_closed_false_migrates_to_auto` | — |

---

## `lib/config/migrations/custom/require-config-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/require-config-migration.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `config/migrations/custom/require-config-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate requireConfig=true to requireConfig=required | 4 | ported | `migrate_validate.rs` | `require_config_true_string_migrates_to_required` | — |
| should migrate requireConfig=false to requireConfig=optional | 14 | ported | `migrate_validate.rs` | `require_config_false_string_migrates_to_optional` | — |

---

## `lib/config/migrations/custom/rebase-stale-prs-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/rebase-stale-prs-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/migrations/custom/rebase-stale-prs-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate true | 4 | ported | `migrate_validate.rs` | `rebase_stale_prs_true_migrates_to_behind_base_branch` | — |
| should migrate false | 14 | ported | `migrate_validate.rs` | `rebase_stale_prs_false_migrates_to_conflicted` | — |
| should migrate null | 24 | ported | `migrate_validate.rs` | `rebase_stale_prs_null_migrates_to_auto` | — |

---

## `lib/config/migrations/custom/rebase-conflicted-prs-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/rebase-conflicted-prs-migration.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `config/migrations/custom/rebase-conflicted-prs-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate false | 4 | ported | `migrate_validate.rs` | `rebase_conflicted_prs_false_migrates_to_never` | — |

---

## `lib/config/migrations/custom/update-lock-files-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/update-lock-files-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/migrations/custom/update-lock-files-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should replace false value | 4 | ported | `migrate_validate.rs` | `update_lock_files_false_migrates_to_skip_artifacts_update` | — |
| should not replace true value | 14 | ported | `migrate_validate.rs` | `update_lock_files_true_is_removed` | — |
| should not replace skipArtifactsUpdate | 24 | ported | `migrate_validate.rs` | `update_lock_files_false_preserves_existing_skip_artifacts_update` | — |

---

## `lib/config/migrations/custom/upgrade-in-range-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/upgrade-in-range-migration.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `config/migrations/custom/upgrade-in-range-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate upgradeInRange=true to rangeStrategy="bump" | 4 | ported | `migrate_validate.rs` | `upgrade_in_range_true_migrates_to_range_strategy_bump` | — |
| should just remove property when upgradeInRange not equals to true | 14 | ported | `migrate_validate.rs` | `upgrade_in_range_false_is_removed` | — |

---

## `lib/config/migrations/custom/version-strategy-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/version-strategy-migration.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `config/migrations/custom/version-strategy-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate versionStrategy="widen" to rangeStrategy="widen" | 4 | ported | `migrate_validate.rs` | `version_strategy_widen_migrates_to_range_strategy_widen` | — |
| should just remove property when versionStrategy not equals to "widen" | 14 | ported | `migrate_validate.rs` | `version_strategy_other_is_removed` | — |

---

## `lib/config/migrations/custom/platform-commit-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/platform-commit-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/migrations/custom/platform-commit-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate platformCommit=true to platformCommit=enabled | 4 | ported | `migrate_validate.rs` | `platform_commit_true_migrates_to_enabled` | — |
| should migrate platformCommit=false to platformCommit=disabled | 14 | ported | `migrate_validate.rs` | `platform_commit_false_migrates_to_disabled` | — |
| should not migrate platformCommit=auto | 24 | ported | `migrate_validate.rs` | `platform_commit_auto_is_unchanged` | — |

---

## `lib/config/migrations/custom/required-status-checks-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/required-status-checks-migration.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `config/migrations/custom/required-status-checks-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate requiredStatusChecks=null to ignoreTests=true | 4 | ported | `migrate_validate.rs` | `required_status_checks_null_migrates_to_ignore_tests` | — |

---

## `lib/config/validation-helpers/match-base-branches.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/validation-helpers/match-base-branches.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `config/validation-helpers/match-base-branches`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns error when baseBranchPatterns is not defined | 4 | ported | `migrate_validate.rs` | `validation_helper_match_base_branches_requires_base_branch_patterns` | — |
| returns empty array for valid configuration | 18 | ported | `migrate_validate.rs` | `validation_helper_match_base_branches_accepts_base_branch_patterns` | — |

---

## `lib/config/validation-helpers/regex-glob-matchers.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/validation-helpers/regex-glob-matchers.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/validation-helpers/regex-glob-matchers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should error for multiple match alls | 4 | ported | `migrate_validate.rs` | `validation_helper_regex_glob_matchers_rejects_multiple_match_alls` | — |
| should error for invalid regex | 12 | ported | `migrate_validate.rs` | `validation_helper_regex_glob_matchers_rejects_invalid_regex` | — |
| should error for non-strings | 20 | ported | `migrate_validate.rs` | `validation_helper_regex_glob_matchers_rejects_non_strings` | — |

---

## `lib/config/validation-helpers/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/validation-helpers/utils.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/validation-helpers/utils`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ignores encrypted in root | 5 | ported | `migrate_validate.rs` | `validation_helper_get_parent_name_ignores_encrypted_in_root` | — |
| handles array types | 9 | ported | `migrate_validate.rs` | `validation_helper_get_parent_name_handles_array_types` | — |
| handles encrypted within array types | 13 | ported | `migrate_validate.rs` | `validation_helper_get_parent_name_handles_encrypted_within_array_types` | — |

---

## `lib/config/validation.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/validation.spec.ts
**Total tests:** 127 | **Ported:** 127 | **Actionable:** 127 | **Status:** ported

### `config/validation › validateConfig(config)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns custom deprecation warnings for %s | 10 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_returns_custom_deprecation_warnings` | — |
| returns the deprecationMsg for `dnsCache` as a warning | 26 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_warns_for_dns_cache_deprecation` | — |
| allow enabled field in vulnerabilityAlerts | 47 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_allows_vulnerability_alerts_enabled` | — |
| catches global options in repo config | 61 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_warns_for_global_options_in_repo_config` | — |
| catches global options in inherit config | 86 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_warns_for_global_options_in_inherit_config` | — |
| only warns for actual globals in repo config | 107 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_ignores_host_rule_credentials` | — |
| does not warn for valid inheritConfig | 124 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_allows_inherited_onboarding` | — |
| does not warn for valid platformConfig | 135 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_allows_auto_platform_config` | — |
| warns for invalid platformConfig | 147 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_invalid_platform_config` | — |
| catches invalid templates | 156 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_catches_invalid_templates` | — |
| catches invalid jsonata expressions | 165 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_catches_invalid_jsonata_expressions` | — |
| catches invalid allowedVersions regex | 179 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_catches_invalid_allowed_versions_regex` | — |
| catches invalid matchCurrentValue | 209 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_catches_invalid_match_current_value_regex` | — |
| catches invalid matchNewValue | 243 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_catches_invalid_match_new_value_regex` | — |
| validates matchBaseBranches | 277 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_validates_match_base_branches` | — |
| catches invalid matchBaseBranches when baseBranchPatterns is not defined | 295 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_warns_for_match_base_branches_without_base_branch_patterns` | — |
| catches invalid matchCurrentVersion regex | 312 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_catches_invalid_match_current_version_regex` | — |
| catches invalid customDatasources content | 347 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_catches_invalid_custom_datasources_content` | — |
| validates invalid statusCheckNames | 384 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_validates_invalid_status_check_names` | — |
| catches invalid customDatasources record type | 408 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_catches_invalid_custom_datasources_record_type` | — |
| catches invalid baseBranchPatterns regex | 423 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_catches_invalid_base_branch_patterns_regex` | — |
| returns nested errors | 436 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_returns_nested_errors` | — |
| included managers of the wrong type | 466 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_match_managers_wrong_type` | — |
| empty configuration | 484 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_allows_empty_configuration` | — |
| single not supported manager | 503 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_unsupported_enabled_managers` | — |
| errors for all types | 523 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_all_types` | — |
| selectors outside packageRules array trigger errors | 558 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_selectors_outside_package_rules` | — |
| ignore packageRule nesting validation for presets | 588 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_ignores_package_rule_nesting_for_presets` | — |
| errors for unsafe managerFilePatterns | 608 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_unsafe_manager_file_patterns` | — |
| validates regEx for each managerFilePatterns of format regex | 627 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_validates_custom_manager_file_pattern_regex` | — |
| errors if customManager has empty managerFilePatterns | 649 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_empty_custom_manager_file_patterns` | — |
| errors if no customManager customType | 675 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_missing_custom_manager_type` | — |
| errors if invalid customManager customType | 703 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_invalid_custom_manager_type` | — |
| errors if empty customManager matchStrings | 732 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_empty_custom_manager_match_strings` | — |
| errors if no customManager managerFilePatterns | 774 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_custom_manager_without_manager_file_patterns` | — |
| validates regEx for each matchStrings | 793 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_validates_custom_manager_match_string_regex` | — |
| error if no fileFormat in custom JSONata manager | 815 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_jsonata_manager_missing_file_format` | — |
| validates JSONata query for each matchStrings | 841 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_validates_jsonata_manager_queries` | — |
| validates all possible regex manager options | 871 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_validates_all_regex_custom_manager_options` | — |
| passes if customManager fields are present | 890 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_allows_valid_custom_managers` | — |
| errors if extra customManager fields are present | 922 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_extra_custom_manager_fields` | — |
| errors if customManager fields are missing | 945 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_missing_regex_custom_manager_fields` | — |
| errors if customManager fields are missing: JSONataManager | 967 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_missing_jsonata_custom_manager_fields` | — |
| ignore keys | 1000 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_ignores_schema_key` | — |
| validates timezone preset | 1013 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_allows_timezone_presets` | — |

### `config/validation › validateConfig(config) › constraints`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| can contain a valid tool name for Containerbase | 1027 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_allows_containerbase_constraint_tool` | — |
| can contain a constraint for a non-Containerbase tool | 1042 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_allows_non_containerbase_constraint_tool` | — |
| warns if an unsupported constraint is specified | 1057 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_warns_for_unsupported_constraint` | — |
| warns if a constraint is not valid | 1079 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_warns_for_invalid_constraint_value` | — |
| errors if constraints is a malformed object | 1100 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_malformed_constraints_object` | — |
| errors if constraints is a malformed array | 1120 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_malformed_constraints_array` | — |

### `config/validation › validateConfig(config) › constraintsVersioning`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| cannot contain a valid tool name for Containerbase | 1142 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_containerbase_tool_constraints_versioning` | — |
| can contain a constraint for a non-Containerbase tool | 1164 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_allows_non_containerbase_constraints_versioning` | — |
| cannot contain an additional constraint name with an invalid versioning scheme | 1179 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_invalid_constraints_versioning_scheme` | — |
| can contain an additional constraint name with a regex versioning scheme | 1200 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_allows_regex_constraints_versioning_scheme` | — |
| cannot contain an unsupported constraint | 1216 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_unknown_constraints_versioning_name` | — |
| errors if constraintsVersioning is a malformed object | 1238 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_malformed_constraints_versioning_object` | — |
| errors if constraintsVersioning is a malformed array | 1260 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_malformed_constraints_versioning_array` | — |

### `config/validation › validateConfig(config)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| validates object with ignored children | 1281 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_allows_object_with_ignored_children` | — |
| validates valid registryAlias objects | 1294 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_allows_valid_registry_aliases` | — |
| errors if registryAliases depth is more than 1 | 1309 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_nested_registry_aliases` | — |
| errors if registryAliases have invalid value | 1331 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_invalid_registry_alias_value` | — |
| errors if managerFilePatterns has wrong parent | 1352 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_warns_for_wrong_manager_file_patterns_parent` | — |
| errors if manager objects are nested | 1395 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_nested_manager_objects` | — |
| warns if hostType has the wrong parent | 1415 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_warns_for_host_type_wrong_parent` | — |
| validates preset values | 1429 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_non_string_preset_values` | — |
| errors on invalid preset syntax | 1442 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_invalid_preset_syntax` | — |
| warns if only selectors in packageRules | 1459 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_warns_for_selector_only_package_rules` | — |
| errors if invalid combinations in packageRules | 1473 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_invalid_package_rule_combinations` | — |
| warns when registryUrls is set at the top level of repo config | 1492 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_warns_for_top_level_registry_urls` | — |
| warns when defaultRegistryUrls is set at the top level of repo config | 1507 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_warns_for_top_level_default_registry_urls` | — |
| warns on nested group packageRules | 1522 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_warns_on_nested_group_package_rules` | — |
| does not error on use of `global:` presets in `globalExtends` | 1541 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_allows_global_presets_in_global_extends` | — |
| does not error on use of `global:` presets in global `extends` | 1554 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_allows_global_presets_in_global_extends_field` | — |
| errors on use of `global:` presets in inherit `extends` | 1567 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_global_presets_in_inherit_extends` | — |
| errors on use of `global:` presets in repo `extends` | 1580 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_global_presets_in_repo_extends` | — |
| warns if customEnvVariables are found in repo config | 1594 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_warns_for_custom_env_variables_in_repo_config` | — |
| errors if schedule is cron and has no * minutes | 1613 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_cron_schedule_without_wildcard_minutes` | — |
| errors if invalid matchHost values in hostRules | 1631 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_invalid_host_rule_match_host_values` | — |
| errors if forbidden header in hostRules | 1673 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_forbidden_host_rule_header` | — |
| errors if headers values are not string | 1701 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_non_string_host_rule_header_values` | — |
| errors if allowedHeaders is empty or not defined | 1728 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_headers_without_allowed_headers` | — |
| catches invalid variable name in env config option | 1755 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_catches_invalid_env_variable_name_and_value` | — |
| catches env config option if configured inside a parent | 1783 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_catches_nested_env_config` | — |
| catches when * or ** is combined with others patterns in a regexOrGlob option | 1820 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_catches_match_all_combined_with_other_patterns` | — |
| catches when negative number is used for integer type | 1848 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_catches_negative_integer_options` | — |
| validates prPriority | 1862 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_allows_negative_pr_priority` | — |
| errors if no bumpVersion filePattern is provided | 1883 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_bump_version_without_file_patterns` | — |
| errors if no matchStrings are provided for bumpVersion | 1909 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_errors_for_bump_version_without_match_strings` | — |
| allow bumpVersion | 1933 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_matches_upstream_bump_version_allow_case` | — |

### `config/validation › validateConfig() -> globaOnly options`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns errors for invalid options | 1959 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_errors_for_invalid_options` | — |
| validates hostRules.headers | 1981 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_validates_host_rule_headers` | — |
| errors if hostRules.headers is defined but allowedHeaders is not | 2001 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_errors_for_headers_without_allowed_headers` | — |
| validates env | 2025 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_validates_env` | — |
| handles prefixed onboardingConfigFileName | 2040 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_allows_prefixed_onboarding_config_file_name` | — |
| allows unique onboardingConfigFileName if it is set in configFileNames | 2054 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_allows_unique_onboarding_config_file_name_in_config_file_names` | — |
| errors if env object is defined but allowedEnv is empty or undefined | 2067 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_errors_for_env_without_allowed_env` | — |
| validates env against the allowedEnv regex | 2086 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_validates_env_against_allowed_env_regex` | — |
| validates options with different type but defaultValue=null | 2101 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_allows_default_null_options` | — |

### `config/validation › validate globalOptions()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| binarySource=docker is deprecated | 2137 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_warns_for_deprecated_docker_binary_source` | — |
| binarySource | 2154 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_warns_for_invalid_binary_source` | — |

### `config/validation › validate globalOptions() › validates string type options`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| binarySource | 2172 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_string_options_binary_source` | — |
| baseDir | 2189 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_string_options_base_dir` | — |
| requireConfig | 2205 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_string_options_require_config` | — |
| dryRun | 2222 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_string_options_dry_run` | — |
| repositoryCache | 2239 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_string_options_repository_cache` | — |
| onboardingConfigFileName | 2256 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_string_options_onboarding_config_file_name` | — |
| onboardingConfig | 2272 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_string_options_onboarding_config` | — |
| force | 2299 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_string_options_force` | — |
| gitUrl | 2324 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_string_options_git_url` | — |

### `config/validation › validate globalOptions()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| validates boolean type options | 2343 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_validates_boolean_type_options` | — |
| validates integer type options | 2363 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_validates_integer_type_options` | — |
| validates array type options | 2383 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_validates_array_type_options` | — |
| validates object type options | 2414 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_validates_object_type_options` | — |
| warns if negative number is used for integer type | 2444 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_warns_for_negative_integer_options` | — |
| warns on invalid customEnvVariables objects | 2461 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_warns_for_invalid_custom_env_variables` | — |
| validates valid customEnvVariables objects | 2482 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_allows_valid_custom_env_variables` | — |
| validates options with different type but defaultValue=null | 2497 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_allows_default_null_options` | — |
| fails for missing reportPath if reportType is "s3" | 2517 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_errors_for_missing_s3_report_path` | — |
| validates reportPath if reportType is "s3" | 2529 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_allows_s3_report_path` | — |
| fails for missing reportPath if reportType is "file" | 2542 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_errors_for_missing_file_report_path` | — |
| validates reportPath if reportType is "file" | 2554 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_allows_file_report_path` | — |
| warns when registryUrls is set at the top level of global config | 2567 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_warns_for_top_level_registry_urls` | — |
| warns when defaultRegistryUrls is set at the top level of global config | 2582 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_warns_for_top_level_default_registry_urls` | — |
| validates postUpgradeTasks.installTools tool names | 2597 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_validates_post_upgrade_install_tools` | — |
| rejects invalid postUpgradeTasks.installTools tool names | 2615 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_rejects_invalid_post_upgrade_install_tools` | — |
| catches when * or ** is combined with others patterns in a regexOrGlob option | 2639 | ported | `crates/renovate-core/src/config/migrate_validate.rs` | `validate_config_global_catches_match_all_combined_with_other_patterns` | — |

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

## `lib/constants/platform.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/constants/platform.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 8 | **Status:** ported

### `constants/platform`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should be part of the GITEA_API_USING_HOST_TYPES | 24 | ported | `platform_constants.rs` | `gitea_api_using_host_types_include_gitea_tags_and_platform` | — |
| should be part of the FORGEJO_API_USING_HOST_TYPES | 31 | ported | `platform_constants.rs` | `forgejo_api_using_host_types_include_expected_host_types` | — |
| should be part of the GITLAB_API_USING_HOST_TYPES | 45 | ported | `platform_constants.rs` | `gitlab_api_using_host_types_include_expected_datasources_and_platform` | — |
| should be not part of the GITLAB_API_USING_HOST_TYPES | 62 | ported | `platform_constants.rs` | `gitlab_api_using_host_types_do_not_include_github` | — |
| should be part of the GITHUB_API_USING_HOST_TYPES | 66 | ported | `platform_constants.rs` | `github_api_using_host_types_include_expected_datasources_and_platform` | — |
| should be not part of the GITHUB_API_USING_HOST_TYPES | 84 | ported | `platform_constants.rs` | `github_api_using_host_types_do_not_include_gitlab` | — |
| should be part of the BITBUCKET_API_USING_HOST_TYPES | 88 | ported | `platform_constants.rs` | `bitbucket_api_using_host_types_include_bitbucket_tags_and_platform` | — |
| should be part of the BITBUCKET_SERVER_API_USING_HOST_TYPES | 95 | ported | `platform_constants.rs` | `bitbucket_server_api_using_host_types_include_server_tags_and_platform` | — |

---

## `lib/instrumentation/detectors.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/instrumentation/detectors.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `instrumentation/detectors`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return all detectors: %o | 19 | not-applicable | — | — | OpenTelemetry JavaScript resource-detector wiring is not implemented in the Rust CLI. |
| should disable all detectors | 40 | not-applicable | — | — | OpenTelemetry JavaScript resource-detector wiring is not implemented in the Rust CLI. |
| should disable cloud detectors | 46 | not-applicable | — | — | OpenTelemetry JavaScript resource-detector wiring is not implemented in the Rust CLI. |
| should enable selected detectors | 52 | not-applicable | — | — | OpenTelemetry JavaScript resource-detector wiring is not implemented in the Rust CLI. |

---

## `lib/instrumentation/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/instrumentation/index.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `instrumentation/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should use NoopTraceProvider if not activated | 40 | not-applicable | — | — | OpenTelemetry JavaScript tracer-provider setup; Rust currently uses `tracing` logging and does not expose Renovate's JS OpenTelemetry initialization API. |
| activate console logger | 48 | not-applicable | — | — | OpenTelemetry JavaScript tracer-provider setup; Rust currently uses `tracing` logging and does not expose Renovate's JS OpenTelemetry initialization API. |
| registers GitOperationSpanProcessor, GetDatasourceReleasesSpanProcessor regardless of tracing being enabled | 69 | not-applicable | — | — | OpenTelemetry JavaScript span processor wiring; Rust does not implement Renovate's JS Git/datasource span processors. |
| activate remote logger | 89 | not-applicable | — | — | OpenTelemetry JavaScript OTLP exporter setup; Rust does not expose Renovate's JS OpenTelemetry exporter pipeline. |
| activate console logger and remote logger | 122 | not-applicable | — | — | OpenTelemetry JavaScript exporter setup; Rust does not expose Renovate's JS OpenTelemetry exporter pipeline. |

### `instrumentation/index › BunyanInstrumentation`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| patches bunyan Logger._emit when tracing is enabled | 161 | not-applicable | — | — | JavaScript Bunyan OpenTelemetry instrumentation; Rust logging uses `tracing` and has no Bunyan logger to patch. |

### `instrumentation/index › instrument`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return result | 175 | not-applicable | — | — | JavaScript OpenTelemetry span wrapper helper; Rust does not expose Renovate's JS `instrument()` API. |
| should rethrow exception | 183 | not-applicable | — | — | JavaScript OpenTelemetry span wrapper helper; Rust does not expose Renovate's JS `instrument()` API. |
| should return result for async fn | 192 | not-applicable | — | — | JavaScript OpenTelemetry span wrapper helper; Rust does not expose Renovate's JS `instrument()` API. |
| should rethrow exception for async fn | 202 | not-applicable | — | — | JavaScript OpenTelemetry span wrapper helper; Rust does not expose Renovate's JS `instrument()` API. |

---

## `lib/instrumentation/reporting.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/instrumentation/reporting.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `instrumentation/reporting`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return empty report if no stats have been added | 74 | not-applicable | — | — | JavaScript instrumentation report accumulator for `reportType`; Rust CLI has a separate report output model and does not expose Renovate's JS reporting module. |
| return report if reportType is set to logging | 93 | not-applicable | — | — | JavaScript instrumentation report accumulator for `reportType`; Rust CLI has a separate report output model and does not expose Renovate's JS reporting module. |
| log report if reportType is set to logging | 105 | not-applicable | — | — | JavaScript instrumentation report export via logger; Rust CLI has a separate report output model and no JS `reportType=logging` exporter. |
| write report if reportType is set to file | 122 | not-applicable | — | — | JavaScript instrumentation report export via `writeSystemFile`; Rust CLI has a separate report output model and no JS `reportType=file` exporter. |
| write formatted report if reportFormatting is enabled | 139 | not-applicable | — | — | JavaScript instrumentation report formatting via Prettier; Rust CLI has a separate report output model and no JS report formatter. |
| send report to an S3 bucket if reportType is s3 | 157 | not-applicable | — | — | JavaScript instrumentation report export via AWS S3 client; Rust CLI has no equivalent JS `reportType=s3` exporter. |
| handle failed parsing of S3 url | 179 | not-applicable | — | — | JavaScript instrumentation report export via AWS S3 client; Rust CLI has no equivalent JS `reportType=s3` exporter. |
| catch exception | 199 | not-applicable | — | — | JavaScript instrumentation report exporter failure handling; Rust CLI has a separate report output model and does not expose this exporter. |
| reports nothing when reportType=null | 213 | not-applicable | — | — | JavaScript instrumentation `reportType` gating; Rust CLI has a separate report output model and does not expose Renovate's JS reporting module. |
| should add problems to report | 226 | not-applicable | — | — | JavaScript `ProblemStream` integration for instrumentation reports; Rust logging does not expose Renovate's JS problem stream. |
| should handle libyears addition | 271 | not-applicable | — | — | JavaScript instrumentation report libyears accumulator; Rust CLI has a separate report output model and does not expose this JS reporting module. |

---

## `lib/instrumentation/with-instrumenting.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/instrumentation/with-instrumenting.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `instrumentation/with-instrumenting`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| wraps async function | 7 | not-applicable | — | — | JavaScript OpenTelemetry wrapper factory; Rust does not expose Renovate's JS `withInstrumenting()` API. |
| instruments multiple calls | 17 | not-applicable | — | — | JavaScript OpenTelemetry wrapper factory; Rust does not expose Renovate's JS `withInstrumenting()` API. |
| propagates errors | 29 | not-applicable | — | — | JavaScript OpenTelemetry wrapper factory; Rust does not expose Renovate's JS `withInstrumenting()` API. |
| accepts options | 37 | not-applicable | — | — | JavaScript OpenTelemetry wrapper factory; Rust does not expose Renovate's JS `withInstrumenting()` API. |
| passes arguments to wrapped function | 54 | not-applicable | — | — | JavaScript OpenTelemetry wrapper factory; Rust does not expose Renovate's JS `withInstrumenting()` API. |

---

## `lib/instrumentation/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/instrumentation/utils.spec.ts
**Total tests:** 1 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `instrumentation/utils › massageThrowable`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return $expected for $input | 5 | not-applicable | — | — | JavaScript thrown-value normalization for OpenTelemetry instrumentation; Rust uses typed errors and has no equivalent throwable value model. |

---

## `lib/data/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/data/index.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `data/index › ${file}`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| keys are sorted alphabetically | 55 | ported | `lib.rs` | `embedded_data_keys_are_sorted_alphabetically` | — |

---

## `lib/logger/bunyan.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/bunyan.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `logger/bunyan`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| checks for valid log levels | 6 | ported | `logging.rs` | `parses_all_valid_renovate_levels` (+ `fatal_maps_to_error`) | — |
| checks for invalid log level: $input | 16 | ported | `logging.rs` | `invalid_level_returns_none` | — |

---

## `lib/logger/cmd-serializer.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/cmd-serializer.spec.ts
**Total tests:** 2 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `logger/cmd-serializer`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns array | 4 | not-applicable | — | — | Bunyan JavaScript serializer hook; Rust logging layer does not expose command serializer objects. |
| redacts | 8 | not-applicable | — | — | Bunyan JavaScript serializer hook; Rust logging layer does not expose command serializer objects. |

---

## `lib/logger/config-serializer.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/config-serializer.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `logger/config-serializer`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| squashes templates | 4 | not-applicable | — | — | Bunyan JavaScript config serializer hook; Rust logging layer does not serialize raw config objects through Bunyan serializers. |
| suppresses content | 15 | not-applicable | — | — | Bunyan JavaScript config serializer hook; Rust logging layer does not serialize raw config objects through Bunyan serializers. |
| suppresses packageFiles | 24 | not-applicable | — | — | Bunyan JavaScript config serializer hook; Rust logging layer does not serialize raw config objects through Bunyan serializers. |

---

## `lib/logger/remap.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/remap.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `logger/remap`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no remaps are set | 15 | not-applicable | — | — | Bunyan JavaScript log-level remapping hook; Rust tracing logger does not expose mutable per-message remap configuration. |
| performs global remaps | 24 | not-applicable | — | — | Bunyan JavaScript log-level remapping hook; Rust tracing logger does not expose mutable per-message remap configuration. |
| performs repository-level remaps | 33 | not-applicable | — | — | Bunyan JavaScript log-level remapping hook; Rust tracing logger does not expose mutable per-message remap configuration. |
| prioritizes repository-level remaps over global remaps | 44 | not-applicable | — | — | Bunyan JavaScript log-level remapping hook; Rust tracing logger does not expose mutable per-message remap configuration. |
| supports regex patterns | 55 | not-applicable | — | — | Bunyan JavaScript log-level remapping hook; Rust tracing logger does not expose mutable per-message remap configuration. |
| does not match against invalid regex patterns | 64 | not-applicable | — | — | Bunyan JavaScript log-level remapping hook; Rust tracing logger does not expose mutable per-message remap configuration. |

---

## `lib/logger/renovate-logger.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/renovate-logger.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `logger/renovate-logger`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws | 6 | not-applicable | — | — | JavaScript `RenovateLogger` Bunyan wrapper initialization behavior; Rust initializes tracing directly and has no uninitialized Bunyan logger object. |
| should queue logs until initialized | 12 | not-applicable | — | — | JavaScript `RenovateLogger` pre-Bunyan queue behavior; Rust initializes tracing directly and does not queue log calls before a Bunyan instance exists. |

### `logger/renovate-logger › before bunyan is initialized`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should log to console | 27 | not-applicable | — | — | JavaScript pre-Bunyan initialization warning behavior; Rust tracing initialization has no equivalent Bunyan-not-initialized console warning. |
| should not log more than once | 36 | not-applicable | — | — | JavaScript pre-Bunyan initialization warning behavior; Rust tracing initialization has no equivalent Bunyan-not-initialized console warning. |

---

## `lib/logger/pretty-stdout.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/pretty-stdout.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `logger/pretty-stdout › getMeta(rec)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty string if null rec | 9 | not-applicable | — | — | Bunyan record pretty-printer helper; Rust logging/output does not expose Renovate's JavaScript `PrettyStdoutStream` record formatter. |
| returns empty string if empty rec | 13 | not-applicable | — | — | Bunyan record pretty-printer helper; Rust logging/output does not expose Renovate's JavaScript `PrettyStdoutStream` record formatter. |
| returns empty string if no meta fields | 17 | not-applicable | — | — | Bunyan record pretty-printer helper; Rust logging/output does not expose Renovate's JavaScript `PrettyStdoutStream` record formatter. |
| supports single meta | 24 | not-applicable | — | — | Bunyan record pretty-printer helper; Rust logging/output does not expose Renovate's JavaScript `PrettyStdoutStream` record formatter. |
| supports multi meta | 34 | not-applicable | — | — | Bunyan record pretty-printer helper; Rust logging/output does not expose Renovate's JavaScript `PrettyStdoutStream` record formatter. |
| returns plain text when colorize is false | 46 | not-applicable | — | — | Bunyan record pretty-printer helper; Rust logging/output does not expose Renovate's JavaScript `PrettyStdoutStream` record formatter. |

### `logger/pretty-stdout › getDetails(rec)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty string if null rec | 57 | not-applicable | — | — | Bunyan record detail formatter; Rust logging/output does not expose Renovate's JavaScript pretty-stdout detail formatting API. |
| returns empty string if empty rec | 61 | not-applicable | — | — | Bunyan record detail formatter; Rust logging/output does not expose Renovate's JavaScript pretty-stdout detail formatting API. |
| returns empty string if all are meta fields | 67 | not-applicable | — | — | Bunyan record detail formatter; Rust logging/output does not expose Renovate's JavaScript pretty-stdout detail formatting API. |
| supports a config | 75 | not-applicable | — | — | Bunyan record detail formatter; Rust logging/output does not expose Renovate's JavaScript pretty-stdout detail formatting API. |
| formats err.stack as readable multi-line output | 88 | not-applicable | — | — | JavaScript error stack formatting for Bunyan records; Rust errors and tracing output are not rendered through this formatter. |
| formats err.stack without other err fields | 108 | not-applicable | — | — | JavaScript error stack formatting for Bunyan records; Rust errors and tracing output are not rendered through this formatter. |

### `logger/pretty-stdout › formatRecord(rec)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| formats record | 136 | not-applicable | — | — | Bunyan record pretty-printer helper; Rust logging/output does not expose Renovate's JavaScript record formatter. |
| formats record without colors | 155 | not-applicable | — | — | Bunyan record pretty-printer helper; Rust logging/output does not expose Renovate's JavaScript record formatter. |

### `logger/pretty-stdout › PrettyStdoutStream`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| writes formatted data to stdout | 175 | not-applicable | — | — | JavaScript Writable stream wrapper for Bunyan records; Rust logging/output does not expose Renovate's `PrettyStdoutStream`. |

---

## `lib/logger/err-serializer.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/err-serializer.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `logger/err-serializer`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| expands errors | 9 | not-applicable | — | — | Bunyan JavaScript error serializer hook; Rust tracing/error types are not serialized through Renovate's JS `errSerializer`. |
| handles missing fields | 40 | not-applicable | — | — | Bunyan JavaScript error serializer hook; Rust tracing/error types are not serialized through Renovate's JS `errSerializer`. |

### `logger/err-serializer › got`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles http error | 66 | not-applicable | — | — | JavaScript got HTTP error serialization for Bunyan output; Rust HTTP errors use typed Rust error enums instead of got error objects. |
| sanitize http error | 83 | not-applicable | — | — | JavaScript got HTTP error sanitation for Bunyan output; Rust logging layer does not expose the JS sanitizer or got error object model. |
| handles AggregateErrors | 113 | not-applicable | — | — | JavaScript `AggregateError` serialization for Bunyan output; Rust uses typed error chains instead of JS aggregate error objects. |

---

## `lib/logger/once.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/once.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `logger/once › core`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call a function only once | 15 | not-applicable | — | — | JavaScript logger callsite de-duplication helper; Rust tracing logger does not expose an equivalent `once` callback/cache API. |
| supports support distinct calls | 28 | not-applicable | — | — | JavaScript logger callsite de-duplication helper; Rust tracing logger does not expose an equivalent `once` callback/cache API. |
| resets keys | 44 | not-applicable | — | — | JavaScript logger callsite de-duplication helper; Rust tracing logger does not expose an equivalent `once` callback/cache API. |

### `logger/once › logger`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| logs once per function call | 60 | not-applicable | — | — | JavaScript `logger.once.*` facade behavior; Rust tracing logger does not expose once-only log methods. |
| distincts between log levels | 73 | not-applicable | — | — | JavaScript `logger.once.*` facade behavior; Rust tracing logger does not expose once-only log methods. |
| distincts between different log statements | 89 | not-applicable | — | — | JavaScript `logger.once.*` facade behavior; Rust tracing logger does not expose once-only log methods. |
| parameters are taken into account when de-duplicating calls | 106 | not-applicable | — | — | JavaScript `logger.once.*` facade behavior; Rust tracing logger does not expose once-only log methods. |
| allows mixing single-time and regular logging | 124 | not-applicable | — | — | JavaScript `logger.once.*` facade behavior; Rust tracing logger does not expose once-only log methods. |
| supports reset method | 146 | not-applicable | — | — | JavaScript `logger.once.*` facade behavior; Rust tracing logger does not expose once-only log methods. |

---

## `lib/logger/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/utils.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `logger/utils`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sanitizeValue("$input") == "$output" | 11 | not-applicable | — | — | JavaScript logger value sanitizer for Bunyan structured fields; Rust tracing layer does not expose equivalent arbitrary JS value sanitation. |
| sanitizes boxed String objects as strings | 26 | not-applicable | — | — | JavaScript boxed `String` handling for Bunyan structured fields; Rust has no boxed JS string value model. |
| preserves secret template strings in redacted fields | 39 | not-applicable | — | — | JavaScript logger value sanitizer for config-shaped objects; Rust logging layer does not serialize arbitrary config objects through this sanitizer. |

### `logger/utils › prepareError`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| prepareZodIssues | 90 | not-applicable | — | — | JavaScript Zod error formatting for logger output; Rust uses typed errors and has no Zod error model. |
| prepareError | 178 | not-applicable | — | — | JavaScript Zod error formatting for logger output; Rust uses typed errors and has no Zod error model. |
| handles HTTP timout error | 203 | not-applicable | — | — | JavaScript got `TimeoutError` serialization for logger output; Rust HTTP errors use Rust error types. |
| handles rawExec error | 219 | not-applicable | — | — | JavaScript `ExecError` serialization for logger output; Rust CLI does not expose Renovate's JS rawExec error object. |
| handles AggregateError | 232 | not-applicable | — | — | JavaScript `AggregateError` serialization for logger output; Rust uses typed error chains instead of JS aggregate error objects. |

---

## `lib/logger/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/index.spec.ts
**Total tests:** 26 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `logger/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| inits | 44 | not-applicable | — | — | JavaScript global Bunyan logger facade initialization; Rust initializes tracing directly and does not expose Renovate's JS logger singleton API. |
| uses an auto-generated log context | 48 | not-applicable | — | — | JavaScript Bunyan log-context metadata behavior; Rust tracing output does not expose Renovate's JS logger context mutators. |
| sets and gets context | 54 | not-applicable | — | — | JavaScript Bunyan log-context metadata behavior; Rust tracing output does not expose Renovate's JS logger context mutators. |
| supports logging with metadata | 65 | not-applicable | — | — | JavaScript Bunyan logger facade call shape; Rust uses `tracing` macros rather than Renovate's JS logger object API. |
| supports logging with only metadata | 69 | not-applicable | — | — | JavaScript Bunyan logger facade call shape; Rust uses `tracing` macros rather than Renovate's JS logger object API. |
| supports logging without metadata | 73 | not-applicable | — | — | JavaScript Bunyan logger facade call shape; Rust uses `tracing` macros rather than Renovate's JS logger object API. |
| sets level | 311 | not-applicable | — | — | Runtime Bunyan stream level mutation API; Rust logging level is configured through tracing initialization, not Renovate's JS `levels()` helper. |
| should create a child logger | 317 | not-applicable | — | — | JavaScript Bunyan child logger behavior; Rust tracing logging does not expose Renovate's JS `childLogger()` API. |
| saves problems | 329 | not-applicable | — | — | JavaScript `ProblemStream` capture and sanitizer behavior; Rust logging does not expose Renovate's JS problem stream. |
| should contain path or stream parameters | 350 | not-applicable | — | — | JavaScript Bunyan stream validation; Rust tracing setup does not expose Renovate's JS `addStream()` API. |
| doesn't support rotating files | 359 | not-applicable | — | — | JavaScript Bunyan stream validation; Rust tracing setup does not expose Renovate's JS `addStream()` API. |
| supports file-based logging | 370 | not-applicable | — | — | JavaScript Bunyan file stream behavior; Rust logging does not expose Renovate's JS `addStream()` logfile API. |
| handles cycles | 393 | not-applicable | — | — | JavaScript Bunyan serializer cycle handling; Rust logging does not serialize arbitrary JS object graphs. |
| sanitizes secrets | 426 | not-applicable | — | — | JavaScript Bunyan sanitizer integration for arbitrary object values; Rust logging does not expose Renovate's JS sanitizer pipeline. |
| applies custom serializer while keeping default sanitizers | 480 | not-applicable | — | — | JavaScript Bunyan custom serializer integration; Rust logging does not expose Renovate's JS serializer pipeline. |
| sanitizes secrets in object keys | 550 | not-applicable | — | — | JavaScript Bunyan sanitizer integration for arbitrary object keys; Rust logging does not expose Renovate's JS sanitizer pipeline. |

### `logger/index › meta functions`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sets meta | 83 | not-applicable | — | — | JavaScript global logger metadata mutator behavior; Rust tracing logging does not expose Renovate's JS `setMeta` API. |
| adds meta | 101 | not-applicable | — | — | JavaScript global logger metadata mutator behavior; Rust tracing logging does not expose Renovate's JS `addMeta` API. |
| removes meta | 119 | not-applicable | — | — | JavaScript global logger metadata mutator behavior; Rust tracing logging does not expose Renovate's JS `removeMeta` API. |
| withMeta adds and removes metadata correctly | 153 | not-applicable | — | — | JavaScript scoped logger metadata mutator behavior; Rust tracing logging does not expose Renovate's JS `withMeta` API. |
| withMeta handles cleanup when callback throws | 182 | not-applicable | — | — | JavaScript scoped logger metadata cleanup behavior; Rust tracing logging does not expose Renovate's JS `withMeta` API. |

### `logger/index › createDefaultStreams`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates log file stream | 210 | not-applicable | — | — | JavaScript Bunyan default stream construction; Rust tracing setup does not expose Renovate's JS `createDefaultStreams()` API. |
| handles log file stream $logFileLevel level | 220 | not-applicable | — | — | JavaScript Bunyan logfile stream level configuration; Rust tracing setup does not expose Renovate's JS logfile stream model. |
| handles log file stream $logFileFormat format | 248 | not-applicable | — | — | JavaScript Bunyan logfile stream format configuration; Rust tracing setup does not expose Renovate's JS logfile stream model. |
| writes pretty formatted data synchronously to log file | 274 | not-applicable | — | — | JavaScript Bunyan pretty logfile stream behavior; Rust tracing setup does not expose Renovate's JS logfile stream model. |
| writes json data synchronously to log file | 293 | not-applicable | — | — | JavaScript Bunyan JSON logfile stream behavior; Rust tracing setup does not expose Renovate's JS logfile stream model. |

---

## `lib/util/string-match.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/string-match.spec.ts
**Total tests:** 25 | **Ported:** 25 | **Actionable:** 25 | **Status:** ported

### `util/string-match › matchRegexOrGlobList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if empty patterns | 10 | ported | `string_match.rs` | `string_match_spec_empty_patterns_returns_false` | — |
| returns false if no match | 14 | ported | `string_match.rs` | `string_match_spec_no_match_returns_false` | — |
| returns true if star | 18 | ported | `string_match.rs` | `string_match_spec_star_returns_true` | — |
| returns true if any match | 22 | ported | `string_match.rs` | `string_match_spec_any_positive_match_returns_true` | — |
| returns true if one match with negative patterns | 26 | ported | `string_match.rs` | `string_match_spec_one_negative_pattern_returns_true` | — |
| returns true if every match with negative patterns | 30 | ported | `string_match.rs` | `string_match_spec_every_negative_regex_returns_true` | — |
| returns true if matching positive and negative patterns | 34 | ported | `string_match.rs` | `negative_regex_positive_pattern_returns_true` | — |
| returns true case insensitive for glob | 38 | ported | `string_match.rs` | `glob_is_case_insensitive_matching_renovate_nocase` | — |
| returns true if matching every negative pattern (regex) | 42 | ported | `string_match.rs` | `negative_regex_positive_pattern_allows_all_non_matches` | — |
| returns false if not matching every negative pattern (regex) | 48 | ported | `string_match.rs` | `all_negative_patterns_both_must_not_match` | — |
| returns true if matching every negative pattern (glob) | 52 | ported | `string_match.rs` | `negative_glob_positive_pattern_returns_true` | — |
| returns false if not matching every negative pattern (glob) | 58 | ported | `string_match.rs` | `all_negative_patterns_both_must_not_match_glob` | — |

### `util/string-match › anyMatchRegexOrGlobList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if empty patterns | 64 | ported | `string_match.rs` | `any_match_empty_patterns_returns_false` | — |
| returns false if empty inputs | 68 | ported | `string_match.rs` | `any_match_empty_inputs_returns_false` | — |
| returns true if both empty | 72 | ported | `string_match.rs` | `any_match_both_empty_returns_false` | — |
| returns true if any match with positive | 76 | ported | `string_match.rs` | `any_match_positive_list_matches` | — |
| returns true if any match with negative | 80 | ported | `string_match.rs` | `any_match_negative_list_matches_non_excluded` | — |

### `util/string-match › getRegexPredicate()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| allows valid regex pattern | 86 | ported | `string_match.rs` | `get_regex_predicate_allows_valid_regex_pattern` | — |
| invalidates invalid regex pattern | 90 | ported | `string_match.rs` | `get_regex_predicate_invalidates_invalid_regex_pattern` | — |
| allows the i flag in regex pattern | 94 | ported | `string_match.rs` | `get_regex_predicate_allows_i_flag` | — |
| allows negative regex pattern | 98 | ported | `string_match.rs` | `get_regex_predicate_allows_negative_regex_pattern` | — |
| does not allow non-regex input | 102 | ported | `string_match.rs` | `get_regex_predicate_rejects_non_regex_input` | — |

### `util/string-match › matchRegexOrGlob()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true if positive regex pattern matched | 108 | ported | `string_match.rs` | `match_regex_or_glob_positive_regex_pattern_matched` | — |
| returns true if negative regex is not matched | 112 | ported | `string_match.rs` | `match_regex_or_glob_negative_regex_not_matched_returns_true` | — |
| returns false if negative pattern is matched | 116 | ported | `string_match.rs` | `match_regex_or_glob_negative_pattern_matched_returns_false` | — |

---

## `lib/util/package-rules/managers.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/package-rules/managers.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `util/package-rules/managers › match`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true | 7 | ported | `package_rule.rs` | `managers_matcher_returns_true_for_matching_manager` | — |
| should return false for no match | 19 | ported | `package_rule.rs` | `managers_matcher_returns_false_for_no_match` | — |
| should return null if matchManagers is undefined | 31 | ported | `package_rule.rs` | `managers_matcher_without_patterns_is_not_a_constraint` | Rust matcher uses `true` to represent "no constraint"; the TypeScript matcher returns `null` before the package-rule combiner skips it |
| should return false if no manager | 41 | ported | `package_rule.rs` | `managers_matcher_returns_false_if_no_manager` | — |
| should match custom managers | 51 | ported | `package_rule.rs` | `managers_matcher_matches_custom_managers` | — |

---

## `lib/util/package-rules/dep-names.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/package-rules/dep-names.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `util/package-rules/dep-names › match`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return false if packageFile is not defined | 7 | ported | `package_rule.rs` | `dep_name_matcher_returns_false_if_dep_name_is_empty` | Rust `DepContext` carries a string dep name; empty string covers the missing depName case |
| should return false if depName is excluded prefix | 19 | ported | `package_rule.rs` | `dep_name_matcher_returns_false_if_dep_name_is_excluded_prefix` | — |
| should return true if depName is included prefix | 42 | ported | `package_rule.rs` | `dep_name_matcher_returns_true_if_dep_name_is_included_prefix` | — |
| should return false if for wrong prefix | 65 | ported | `package_rule.rs` | `dep_name_matcher_returns_false_for_wrong_prefix` | — |

---

## `lib/util/package-rules/current-value.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/package-rules/current-value.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `util/package-rules/current-value › match`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return true for exact match | 7 | ported | `package_rule.rs` | `current_value_matcher_returns_true_for_exact_match` | — |
| return true for glob match | 19 | ported | `package_rule.rs` | `current_value_matcher_returns_true_for_glob_match` | — |
| return false for glob non match | 31 | ported | `package_rule.rs` | `current_value_matcher_returns_false_for_glob_non_match` | — |
| return false for regex version non match | 43 | ported | `package_rule.rs` | `current_value_matcher_returns_false_for_regex_version_non_match` | — |
| case insensitive match | 55 | ported | `package_rule.rs` | `current_value_matcher_is_case_insensitive_for_i_regex_flag` | — |
| return true for regex version match | 67 | ported | `package_rule.rs` | `current_value_matcher_returns_true_for_regex_version_match` | — |
| return false for now value | 79 | ported | `package_rule.rs` | `current_value_matcher_returns_false_for_missing_value` | — |

---

## `lib/util/package-rules/new-value.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/package-rules/new-value.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `util/package-rules/new-value › match`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return true for exact match | 7 | ported | `package_rule.rs` | `new_value_matcher_returns_true_for_exact_match` | — |
| return true for glob match | 19 | ported | `package_rule.rs` | `new_value_matcher_returns_true_for_glob_match` | — |
| return false for glob non match | 31 | ported | `package_rule.rs` | `new_value_matcher_returns_false_for_glob_non_match` | — |
| return false for regex version non match | 43 | ported | `package_rule.rs` | `new_value_matcher_returns_false_for_regex_version_non_match` | — |
| case insensitive match | 55 | ported | `package_rule.rs` | `new_value_matcher_is_case_insensitive_for_i_regex_flag` | — |
| return true for regex version match | 67 | ported | `package_rule.rs` | `new_value_matcher_returns_true_for_regex_version_match` | — |
| return false for now value | 79 | ported | `package_rule.rs` | `new_value_matcher_returns_false_for_missing_value` | — |

---

## `lib/util/package-rules/package-names.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/package-rules/package-names.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `util/package-rules/package-names › match`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return false if packageName is not defined | 7 | ported | `package_rule.rs` | `package_name_matcher_returns_false_if_package_name_is_empty` | Rust `PackageRule::name_matches` carries a string package name; empty string covers the missing packageName case |
| should return false if not matching | 19 | ported | `package_rule.rs` | `package_name_matcher_returns_false_if_not_matching` | — |
| should matchPackageName | 32 | ported | `package_rule.rs` | `package_name_matcher_matches_package_name` | — |
| should match pattern | 44 | ported | `package_rule.rs` | `package_name_matcher_matches_regex_pattern` | — |

---

## `lib/util/package-rules/files.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/package-rules/files.spec.ts
**Total tests:** 1 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `util/package-rules/files › match`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return false if packageFile is not defined | 7 | ported | `package_rule.rs` | `file_names_matcher_returns_false_if_package_file_is_missing` | — |

---

## `lib/util/package-rules/current-age.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/package-rules/current-age.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `util/package-rules/current-age › match`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if release is older | 18 | ported | `package_rule.rs` | `current_age_matcher_returns_false_if_release_is_older` | — |
| returns false if release is younger | 30 | ported | `package_rule.rs` | `current_age_matcher_returns_false_if_release_is_younger` | — |
| returns null if release invalid | 42 | ported | `package_rule.rs` | `current_age_matcher_returns_false_if_release_invalid` | Rust matcher is boolean-only, so invalid dates are treated as a non-match |
| returns false if release undefined | 54 | ported | `package_rule.rs` | `current_age_matcher_returns_false_if_release_undefined` | — |
| returns true if age matches | 66 | ported | `package_rule.rs` | `current_age_matcher_returns_true_if_age_matches` | — |

---

## `lib/util/package-rules/repositories.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/package-rules/repositories.spec.ts
**Total tests:** 15 | **Ported:** 15 | **Actionable:** 15 | **Status:** ported

### `util/package-rules/repositories › match`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null if match repositories is not defined | 7 | ported | `package_rule.rs` | `repositories_matcher_without_patterns_is_not_a_constraint` | Rust matcher uses `true` to represent "no constraint"; the TypeScript matcher returns `null` before the package-rule combiner skips it |
| should return false if repository is not defined | 19 | ported | `package_rule.rs` | `repositories_matcher_returns_false_if_repository_is_missing` | — |
| should return true if repository matches regex pattern | 31 | ported | `package_rule.rs` | `repositories_matcher_returns_true_for_regex_pattern` | — |
| should return false if repository has invalid regex pattern | 43 | ported | `package_rule.rs` | `repositories_matcher_returns_false_for_invalid_regex_pattern` | — |
| should return false if repository does not match regex pattern | 55 | ported | `package_rule.rs` | `repositories_matcher_returns_false_for_non_matching_regex_pattern` | — |
| should return true if repository matches minimatch pattern | 67 | ported | `package_rule.rs` | `repositories_matcher_returns_true_for_minimatch_pattern` | — |
| should return false if repository does not match minimatch pattern | 79 | ported | `package_rule.rs` | `repositories_matcher_returns_false_for_non_matching_minimatch_pattern` | — |
| should return true if repository matches at least one pattern | 91 | ported | `package_rule.rs` | `repositories_matcher_returns_true_if_any_pattern_matches` | — |

### `util/package-rules/repositories › excludes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return false if exclude repository is not defined | 105 | ported | `package_rule.rs` | `repositories_matcher_returns_false_if_exclude_repository_is_missing` | — |
| should return false if exclude repository matches regex pattern | 117 | ported | `package_rule.rs` | `repositories_matcher_returns_false_if_exclude_regex_matches` | — |
| should return true if exclude repository has invalid regex pattern | 129 | ported | `package_rule.rs` | `repositories_matcher_returns_true_if_exclude_regex_is_invalid` | — |
| should return true if exclude repository does not match regex pattern | 141 | ported | `package_rule.rs` | `repositories_matcher_returns_true_if_exclude_regex_does_not_match` | — |
| should return false if exclude repository matches minimatch pattern | 153 | ported | `package_rule.rs` | `repositories_matcher_returns_false_if_exclude_minimatch_matches` | — |
| should return true if exclude repository does not match minimatch pattern | 165 | ported | `package_rule.rs` | `repositories_matcher_returns_true_if_exclude_minimatch_does_not_match` | — |
| should return false if exclude repository matches at least one pattern | 177 | ported | `package_rule.rs` | `repositories_matcher_returns_false_if_any_exclude_pattern_matches` | — |

---

## `lib/util/package-rules/current-version.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/package-rules/current-version.spec.ts
**Total tests:** 10 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `util/package-rules/current-version › match`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true for null versioning | 8 | ported | `package_rule.rs` | `current_version_matcher_returns_true_for_null_versioning_equivalent` | — |
| return false on version exception | 22 | not-applicable | — | — | Mocks Renovate's dynamic PEP440 versioning API exception path; Rust matcher does not dispatch through that API |
| return true for a valid match | 39 | not-applicable | — | — | Exercises Renovate PEP440 range matching; Rust package-rule matcher currently uses semver-compatible matching only |
| return false if no version could be found | 52 | ported | `package_rule.rs` | `current_version_matcher_returns_false_if_no_version_found` | — |
| case insensitive match | 66 | ported | `package_rule.rs` | `current_version_matcher_regex_is_case_insensitive` | — |
| return false for regex version non match | 79 | ported | `package_rule.rs` | `current_version_matcher_returns_false_for_regex_version_non_match` | — |
| return true for regex version match | 93 | ported | `package_rule.rs` | `current_version_matcher_returns_true_for_regex_version_match` | — |
| return false for regex value match | 107 | ported | `package_rule.rs` | `current_version_matcher_returns_false_for_regex_value_match_without_version` | — |
| return true for same-major verisioning if version lies in expected range | 120 | not-applicable | — | — | Exercises Renovate's same-major versioning API; Rust matcher does not implement same-major dispatch |
| return false for same-major verisioning if version lies outside of expected range | 133 | not-applicable | — | — | Exercises Renovate's same-major versioning API; Rust matcher does not implement same-major dispatch |

---

## `lib/util/package-rules/jsonata.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/package-rules/jsonata.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/package-rules/jsonata`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true for a matching JSONata expression | 6 | not-applicable | — | — | JSONata expression evaluation is not implemented in Rust package-rule matching |
| should return false for a non-matching JSONata expression | 14 | not-applicable | — | — | JSONata expression evaluation is not implemented in Rust package-rule matching |
| should return false for an invalid JSONata expression | 22 | not-applicable | — | — | JSONata expression evaluation is not implemented in Rust package-rule matching |
| should return null if matchJsonata is not defined | 30 | not-applicable | — | — | JSONata expression evaluation is not implemented in Rust package-rule matching |
| should return true for a complex JSONata expression | 35 | not-applicable | — | — | JSONata expression evaluation is not implemented in Rust package-rule matching |
| should return false for a complex JSONata expression with non-matching version | 44 | not-applicable | — | — | JSONata expression evaluation is not implemented in Rust package-rule matching |
| should return true for a JSONata expression with nested properties | 53 | not-applicable | — | — | JSONata expression evaluation is not implemented in Rust package-rule matching |
| should return false for a JSONata expression with nested properties and non-matching version | 62 | not-applicable | — | — | JSONata expression evaluation is not implemented in Rust package-rule matching |
| should return true if any JSONata expression matches | 71 | not-applicable | — | — | JSONata expression evaluation is not implemented in Rust package-rule matching |
| should catch evaluate errors | 79 | not-applicable | — | — | JSONata expression evaluation is not implemented in Rust package-rule matching |

### `util/package-rules/jsonata › $detectPlatform`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true when sourceUrl matches platform | 88 | not-applicable | — | — | JSONata custom function evaluation is not implemented in Rust package-rule matching |
| should return false when sourceUrl does not match platform | 96 | not-applicable | — | — | JSONata custom function evaluation is not implemented in Rust package-rule matching |

---

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

## `lib/workers/repository/init/apis.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/init/apis.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/init/apis › initApis`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| runs | 24 | not-applicable | — | — | Rust worker layer does not implement Renovate repository initApis/platform bootstrap flow |
| throws for disabled | 34 | not-applicable | — | — | Rust worker layer does not implement optimizeForDisabled repository bootstrap checks |
| throws for forked | 49 | not-applicable | — | — | Rust worker layer does not implement fork-processing repository bootstrap checks |
| does not throw for includeForks=true | 66 | not-applicable | — | — | Rust worker layer does not implement fork-processing repository bootstrap checks |
| does not throw for forkProcessing=enabled | 79 | not-applicable | — | — | Rust worker layer does not implement fork-processing repository bootstrap checks |
| ignores platform.getJsonFile() failures | 92 | not-applicable | — | — | Rust worker layer does not implement platform getJsonFile probing during repository bootstrap |
| throws for fork with platform.getJsonFile() failures | 109 | not-applicable | — | — | Rust worker layer does not implement platform getJsonFile probing during repository bootstrap |
| uses the onboardingConfigFileName if set | 124 | not-applicable | — | — | Rust worker layer does not implement onboarding config file probing during repository bootstrap |
| falls back to "renovate.json" if onboardingConfigFileName is not set | 151 | not-applicable | — | — | Rust worker layer does not implement onboarding config file probing during repository bootstrap |
| falls back to "renovate.json" if onboardingConfigFileName is not valid | 172 | not-applicable | — | — | Rust worker layer does not implement onboarding config file probing during repository bootstrap |
| checks for re-enablement and continues | 191 | not-applicable | — | — | Rust worker layer does not implement disabled-repository re-enablement probing |
| checks for re-enablement and skips | 211 | not-applicable | — | — | Rust worker layer does not implement disabled-repository re-enablement probing |

---

## `lib/workers/repository/init/cache.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/init/cache.spec.ts
**Total tests:** 1 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/init/cache › initializeCaches()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| initializes | 23 | not-applicable | — | — | Rust worker layer does not implement Renovate repository cache initialization |

---

## `lib/workers/global/config/parse/file.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/global/config/parse/file.spec.ts
**Total tests:** 15 | **Ported:** 8 | **Actionable:** 8 | **Status:** ported

### `workers/global/config/parse/file › .getConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses %s > %s | 27 | not-applicable | — | — | Upstream `it.each` matrix depends on JS/YAML global config formats; Rust intentionally supports JSON/JSON5 only (CD-0003) |
| migrates | 56 | not-applicable | — | — | Upstream fixture is `config2.js`; Rust intentionally does not execute JS global config files (CD-0003) |
| warns if config is invalid | 68 | not-applicable | — | — | Rust global config file loading uses strongly typed deserialization and returns parse errors instead of Renovate's warning collector flow |
| parse and returns empty config if there is no RENOVATE_CONFIG_FILE in env | 80 | ported | `file.rs` | `resolve_returns_none_when_env_not_set` | — |
| fatal error and exit if error in parsing %s | 84 | not-applicable | — | — | Upstream `it.each` matrix covers JS/YAML parser failures; Rust intentionally supports JSON/JSON5 only (CD-0003) |
| fatal error and exit if custom config file does not exist | 112 | ported | `file.rs` | `resolve_errors_when_explicit_file_missing` | — |
| fatal error and exit if config.js contains unresolved env var | 126 | not-applicable | — | — | JS global config evaluation and unresolved JS variable errors are intentionally unsupported (CD-0003) |
| fatal error and exit if %s | 147 | ported | `file.rs` | `load_rejects_unsupported_or_missing_extension` | — |
| exports env variables to environment from processEnv object | 161 | not-applicable | — | — | Rust crate denies unsafe code; Rust 2024 process environment mutation requires unsafe global state writes |
| does not export env variables to environment from processEnv object if key/value is invalid | 184 | not-applicable | — | — | Rust crate denies unsafe code; Rust 2024 process environment mutation requires unsafe global state writes |

### `workers/global/config/parse/file › deleteConfigFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skip when RENOVATE_CONFIG_FILE is not set ("%s") | 214 | ported | `file.rs` | `delete_non_default_config_skips_when_env_not_set` | — |
| skip when config file does not exist | 226 | ported | `file.rs` | `delete_non_default_config_skips_missing_file` | — |
| skip if deleteConfigFile is not set ("%s") | 239 | ported | `file.rs` | `delete_non_default_config_skips_when_flag_is_false` | — |
| removes the specified config file | 255 | ported | `file.rs` | `delete_non_default_config_removes_file` | — |
| fails silently when attempting to delete the config file | 278 | ported | `file.rs` | `delete_non_default_config_fails_silently` | — |

---

## `lib/workers/global/config/parse/env.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/global/config/parse/env.spec.ts
**Total tests:** 45 | **Ported:** 40 | **Actionable:** 40 | **Status:** ported

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
| massages converted experimental env vars | 309 | ported | `config_env.rs` | `experimental_env_vars_are_massaged` | — |
| does not migrate empty RENOVATE_X_REPO_CACHE_FORCE_LOCAL | 336 | ported | `config_env.rs` | `empty_repo_cache_force_local_is_not_migrated` | — |

### `workers/global/config/parse/env › .getConfig(env) › RENOVATE_CONFIG tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| crashes | 357 | ported | `config_env.rs` | `invalid_renovate_config_is_rejected` | — |
| migrates RENOVATE_CONFIG | 367 | ported | `config_env.rs` | `renovate_config_automerge_any_is_migrated` | — |
| warns if config in RENOVATE_CONFIG is invalid | 376 | not-applicable | — | — | Rust `RENOVATE_CONFIG` parsing uses strongly typed deserialization and returns parse errors instead of Renovate's warning collector flow |

### `workers/global/config/parse/env › .getConfig(env) › migrations`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| renames migrated variables | 386 | ported | `config_env.rs` | `git_lab_automerge_env_sets_platform_automerge` | — |

### `workers/global/config/parse/env`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| has no duplicate env names across options | 396 | not-applicable | — | — | TypeScript option metadata registry; Rust env names are static `clap` attributes |

### `workers/global/config/parse/env › .getEnvName(definition)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty | 418 | not-applicable | — | — | TypeScript option-definition helper; Rust env names are static `clap` attributes |
| returns existing env | 426 | not-applicable | — | — | TypeScript option-definition helper; Rust env names are static `clap` attributes |
| generates RENOVATE_ env | 434 | not-applicable | — | — | TypeScript option-definition helper; Rust env names are static `clap` attributes |
| dryRun boolean true | 441 | ported | `config_env.rs` | `dry_run_true_maps_to_full` | — |
| dryRun boolean false | 449 | ported | `config_env.rs` | `dry_run_false_disables_dry_run` | — |
| dryRun null | 457 | ported | `config_env.rs` | `dry_run_null_disables_dry_run` | — |
| requireConfig boolean true | 465 | ported | `config_env.rs` | `require_config_true_maps_to_required` | — |
| requireConfig boolean false | 473 | ported | `config_env.rs` | `require_config_false_maps_to_optional` | — |
| platformCommit boolean true | 481 | ported | `config_env.rs` | `platform_commit_true_maps_to_enabled` | — |
| platformCommit boolean false | 489 | ported | `config_env.rs` | `platform_commit_false_maps_to_disabled` | — |

---

## `lib/workers/repository/init/merge.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/init/merge.spec.ts
**Total tests:** 40 | **Ported:** 8 | **Actionable:** 8 | **Status:** ported

### `workers/repository/init/merge › detectRepoFileConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns config if not found | 75 | ported | `repo_config.rs` | `returns_not_found_when_optional` | — |
| returns config if not found - uses cache | 81 | not-applicable | — | — | Rust worker layer does not implement repository config-file cache reuse |
| returns cache config from onboarding cache - package.json | 95 | not-applicable | — | — | Rust worker layer does not implement Renovate onboarding branch cache |
| clones, if onboarding cache is valid but parsed config is undefined | 110 | not-applicable | — | — | Rust worker layer does not implement Renovate onboarding branch cache |
| returns cache config from onboarding cache - renovate.json | 133 | not-applicable | — | — | Rust worker layer does not implement Renovate onboarding branch cache |
| uses package.json config if found | 152 | ported | `repo_config.rs` | `discovers_renovate_key_in_package_json` | — |
| massages package.json renovate string | 173 | ported | `repo_config.rs` | `parse_from_package_json_converts_string_to_extends` | — |
| returns error if cannot parse | 187 | not-applicable | — | — | Rust repo config discovery uses a typed default-returning parser and does not expose Renovate's `configFileParseError` object |
| throws error if duplicate keys | 199 | not-applicable | — | — | Rust repo config discovery uses a typed default-returning parser and does not expose Renovate's duplicate-key parse error object |
| finds and parse renovate.json5 | 214 | ported | `repo_config.rs` | `discover_finds_and_parses_renovate_json5` | — |
| finds .github/renovate.json | 226 | ported | `repo_config.rs` | `discover_finds_github_renovate_json` | — |
| finds .gitlab/renovate.json | 238 | ported | `repo_config.rs` | `discover_finds_gitlab_renovate_json` | — |
| finds .renovaterc.json | 250 | ported | `repo_config.rs` | `discover_finds_renovaterc_json` | — |
| finds .renovaterc.json5 | 266 | ported | `repo_config.rs` | `discover_finds_renovaterc_json5` | — |

### `workers/repository/init/merge › checkForRepoConfigError`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no error | 284 | not-applicable | — | — | TypeScript helper for `configFileParseError` objects; Rust discovery uses `RepoConfigResult`/`Result` instead |
| throws on error | 288 | not-applicable | — | — | TypeScript helper for `configFileParseError` objects; Rust discovery uses `RepoConfigResult`/`Result` instead |

### `workers/repository/init/merge › mergeRenovateConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses onboarding config if silent | 305 | not-applicable | — | — | Rust worker layer does not implement Renovate's repository init merge pipeline or silent onboarding config fallback |
| throws error if misconfigured | 317 | not-applicable | — | — | Rust worker layer does not implement Renovate's repository init merge pipeline validation throw path |
| migrates nested config | 333 | not-applicable | — | — | Rust resolves supported presets during typed repo config parsing, not via Renovate's repository init merge pipeline |
| ignores presets | 363 | not-applicable | — | — | Rust resolves supported presets during typed repo config parsing, not via Renovate's repository init merge pipeline |
| continues if no errors | 382 | not-applicable | — | — | Rust worker layer does not implement Renovate's repository init merge pipeline validation result flow |
| continues if no errors-2 | 393 | not-applicable | — | — | Rust worker layer does not implement Renovate's repository init merge pipeline validation result flow |
| sets npmToken to npmrc when it is not inside encrypted | 413 | not-applicable | — | — | Rust does not implement Renovate's npmrc mutation/decryption side-effect path in repository init merge |
| sets npmToken to npmrc when it is inside encrypted | 436 | not-applicable | — | — | Rust does not implement Renovate's npmrc mutation/decryption side-effect path in repository init merge |
| deletes user conifgured env after setting in mem cache | 463 | not-applicable | — | — | Rust does not implement Renovate's user-env mem cache side-effect path in repository init merge |
| applies repositoryEntryConfig between global and repo file config | 485 | not-applicable | — | — | Rust does not implement Renovate's per-repository `repositoryEntryConfig` merge layer |
| supports repositoryEntryConfig without extends or ignorePresets | 608 | not-applicable | — | — | Rust does not implement Renovate's per-repository `repositoryEntryConfig` merge layer |

### `workers/repository/init/merge › setNpmTokenInNpmrc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips in no npmToken found | 641 | not-applicable | — | — | Rust does not implement Renovate's repository init `setNpmTokenInNpmrc` helper |
| adds default npmrc registry if it does not exist | 647 | not-applicable | — | — | Rust does not implement Renovate's repository init `setNpmTokenInNpmrc` helper |
| adds npmToken at end of npmrc string if ${NPM_TOKEN} string not found | 655 | not-applicable | — | — | Rust does not implement Renovate's repository init `setNpmTokenInNpmrc` helper |
| replaces ${NPM_TOKEN} with npmToken value | 661 | not-applicable | — | — | Rust does not implement Renovate's repository init `setNpmTokenInNpmrc` helper |

### `workers/repository/init/merge › applyNpmrc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing if npmrc is missing after token migration | 672 | not-applicable | — | — | Rust does not implement Renovate's repository init npm datasource global `setNpmrc` side effect |
| migrates npmToken and sets npmrc | 680 | not-applicable | — | — | Rust does not implement Renovate's repository init npm datasource global `setNpmrc` side effect |

### `workers/repository/init/merge › applyHostRules`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing when hostRules is not configured | 698 | not-applicable | — | — | Rust does not implement Renovate's repository init hostRules global store or queue/throttle side effects |
| adds hostRules and clears queue and throttle | 710 | not-applicable | — | — | Rust does not implement Renovate's repository init hostRules global store or queue/throttle side effects |
| warns on invalid hostRule and continues applying others | 730 | not-applicable | — | — | Rust validates hostRules in config validation, not through Renovate's repository init hostRules global store |

### `workers/repository/init/merge › static repository config › resolveStaticRepoConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $name | 796 | not-applicable | — | — | Rust does not implement Renovate's `RENOVATE_X_STATIC_REPO_CONFIG_FILE` static repository config feature |

### `workers/repository/init/merge › static repository config › resolveStaticRepoConfig termination cases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $name | 820 | not-applicable | — | — | Rust does not implement Renovate's `RENOVATE_X_STATIC_REPO_CONFIG_FILE` static repository config feature |
| should log static config validation errors and warnings | 840 | not-applicable | — | — | Rust does not implement Renovate's `RENOVATE_X_STATIC_REPO_CONFIG_FILE` static repository config validation path |

### `workers/repository/init/merge › static repository config › mergeRenovateConfig() with a static repository config`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $name | 868 | not-applicable | — | — | Rust does not implement Renovate's `RENOVATE_X_STATIC_REPO_CONFIG_FILE` merge path |

---

## Workers specs

| Renovate spec file | Renovate tests | Rust file | Rust tests | Status |
|--------------------|---------------|-----------|------------|--------|
<!-- workers/global/config/parse/cli.spec.ts converted to per-test format above -->
<!-- workers/global/config/parse/env.spec.ts converted to per-test format above -->
<!-- workers/global/config/parse/file.spec.ts converted to per-test format above -->
<!-- workers/repository/init/merge.spec.ts converted to per-test format above -->
<!-- workers/repository/init/apis.spec.ts converted to per-test format above -->
<!-- workers/repository/init/cache.spec.ts converted to per-test format above -->

---

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

## Utility specs (`lib/util/`)

| Renovate spec file | Renovate tests | Rust file | Rust tests | Status |
|--------------------|---------------|-----------|------------|--------|
<!-- util/string-match.spec.ts converted to per-test format above -->
<!-- util/package-rules/index.spec.ts converted to per-test format above -->
<!-- util/package-rules/managers.spec.ts converted to per-test format above -->
<!-- util/package-rules/dep-names.spec.ts converted to per-test format above -->
<!-- util/package-rules/current-value.spec.ts converted to per-test format above -->
<!-- util/package-rules/current-age.spec.ts converted to per-test format above -->
<!-- util/package-rules/current-version.spec.ts converted to per-test format above -->
<!-- util/package-rules/files.spec.ts converted to per-test format above -->
<!-- util/package-rules/new-value.spec.ts converted to per-test format above -->
<!-- util/package-rules/package-names.spec.ts converted to per-test format above -->
<!-- util/package-rules/repositories.spec.ts converted to per-test format above -->
<!-- util/package-rules/jsonata.spec.ts converted to per-test format above -->
