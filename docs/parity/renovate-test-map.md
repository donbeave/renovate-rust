# Renovate Test Map

**Overall progress:** 118 / 244 actionable tests ported (48%) — updated 2026-04-29

Status key: `ported` · `pending` · `not-applicable`

> Note: Files below the per-test sections are in the legacy summary format and
> will be converted to per-test format in future iterations.

---

## `lib/modules/manager/ansible-galaxy/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/ansible-galaxy/extract.spec.ts
**Total tests:** 14 | **Ported:** 6 | **Actionable:** 10 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 15 | ported | `ansible_galaxy.rs` | `empty_content_returns_no_deps` | — |
| extracts multiple dependencies from requirements.yml | 19 | pending | — | — | — |
| extracts dependencies from a not beautified requirements file | 25 | pending | — | — | — |
| extracts dependencies from requirements.yml with a space at the end of line | 31 | ported | `ansible_galaxy.rs` | `collections_with_git_url_name_and_version` | — |
| extracts git@ dependencies | 41 | ported | `ansible_galaxy.rs` | `collections_with_source_field_and_git_at_url` | — |
| check if an empty file returns null | 56 | ported | `ansible_galaxy.rs` | `blank_file_returns_no_deps` | — |
| check if a requirements file of other systems returns null | 61 | ported | `ansible_galaxy.rs` | `non_ansible_content_returns_empty` | — |
| check collection style requirements file | 66 | pending | — | — | — |
| check collection style requirements file in reverse order and missing empty line | 73 | ported | `ansible_galaxy.rs` | `collections_before_roles_extracts_all_four` | — |
| check galaxy definition file | 79 | pending | — | — | — |

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
**Total tests:** 6 | **Ported:** 1 | **Actionable:** 6 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 6 | ported | `ansible.rs` | `empty_returns_empty` | — |
| extracts multiple image lines from docker_container | 10 | pending | — | — | — |
| extracts multiple image lines from docker_service | 16 | pending | — | — | — |
| extracts image and replaces registry | 22 | pending | — | — | — |
| extracts image but no replacement | 52 | pending | — | — | — |
| extracts image and no double replacement | 82 | pending | — | — | — |

---

## `lib/modules/manager/argocd/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/argocd/extract.spec.ts
**Total tests:** 8 | **Ported:** 4 | **Actionable:** 8 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 11 | ported | `argocd.rs` | `empty_content_returns_empty` | — |
| returns null for invalid | 15 | pending | — | — | — |
| return null for kubernetes manifest | 21 | ported | `argocd.rs` | `skips_non_argocd_file` | — |
| return null if deps array would be empty | 26 | pending | — | — | — |
| return result for double quoted argoproj.io apiVersion reference | 34 | ported | `argocd.rs` | `double_quoted_apiversion_accepted` | — |
| return result for single quoted argoproj.io apiVersion reference | 61 | ported | `argocd.rs` | `single_quoted_apiversion_accepted` | — |
| full test | 88 | pending | — | — | — |
| supports applicationsets | 203 | pending | — | — | — |

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
**Total tests:** 4 | **Ported:** 2 | **Actionable:** 4 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty array for empty configuration file | 41 | ported | `batect.rs` | `empty_returns_empty` | — |
| returns empty array for non-object configuration file | 49 | pending | — | — | — |
| returns an a package file with no dependencies for configuration file without containers or includes | 57 | ported | `batect.rs` | `no_containers_block_returns_empty` | — |
| extracts all available images and bundles from a valid Batect configuration file, including dependencies in included files | 70 | pending | — | — | — |

---

## `lib/modules/manager/buildpacks/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/buildpacks/extract.spec.ts
**Total tests:** 3 | **Ported:** 2 | **Actionable:** 3 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid files | 7 | ported | `buildpacks.rs` | `invalid_toml_returns_empty` | — |
| returns null for empty package.toml | 11 | ported | `buildpacks.rs` | `no_io_buildpacks_returns_empty` | — |
| extracts builder and buildpack images | 20 | pending | — | — | — |

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
**Total tests:** 2 | **Ported:** 0 | **Actionable:** 2 | **Status:** pending

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts from simple file | 13 | pending | — | — | — |
| extracts from complex file | 42 | pending | — | — | — |

---

## `lib/modules/manager/deps-edn/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deps-edn/extract.spec.ts
**Total tests:** 2 | **Ported:** 1 | **Actionable:** 2 | **Status:** partial

### `extractPackageFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid file | 6 | ported | `deps_edn.rs` | `invalid_edn_returns_empty` | — |
| extractPackageFile | 10 | pending | — | — | — |

---

## `lib/modules/manager/droneci/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/droneci/extract.spec.ts
**Total tests:** 5 | **Ported:** 1 | **Actionable:** 5 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 8 | ported | `droneci.rs` | `empty_returns_empty` | — |
| extracts multiple image lines | 12 | pending | — | — | — |

### `modules/manager/droneci/extract`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts image and replaces registry | 19 | pending | — | — | — |
| extracts image but no replacement | 42 | pending | — | — | — |
| extracts image and no double replacement | 65 | pending | — | — | — |

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
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 5 | **Status:** pending

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should extract version and registryUrl | 43 | pending | — | — | — |

### `extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null for empty packageFiles | 62 | pending | — | — | — |
| should skip package with non-existing repo | 67 | pending | — | — | — |
| should extract registryUrl from repo in other file | 85 | pending | — | — | — |
| should extract registryUrl from default repo in other file | 107 | pending | — | — | — |

---

## `lib/modules/manager/gleam/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gleam/extract.spec.ts
**Total tests:** 9 | **Ported:** 4 | **Actionable:** 9 | **Status:** partial

### `modules/manager/gleam/extract`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should extract dev and prod dependencies | 8 | ported | `gleam.rs` | `both_sections` | — |
| should extract dev only dependencies | 41 | ported | `gleam.rs` | `extracts_dev_dependencies` | — |
| should return null when no dependencies are found | 65 | ported | `gleam.rs` | `no_deps_section_returns_empty` | — |
| should return null when gleam.toml is invalid | 82 | ported | `gleam.rs` | `invalid_toml_returns_empty` | — |
| should return locked versions | 91 | pending | — | — | — |
| should fail to extract locked version | 119 | pending | — | — | — |
| should fail to find locked version in range | 138 | pending | — | — | — |
| should handle invalid versions in lock file | 166 | pending | — | — | — |
| should handle lock file parsing and extracting errors | 193 | pending | — | — | — |

---

## `lib/modules/manager/git-submodules/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/git-submodules/extract.spec.ts
**Total tests:** 8 | **Ported:** 5 | **Actionable:** 8 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| empty submodule returns null | 48 | ported | `git_submodules.rs` | `empty_content_returns_no_deps` | — |
| currentValue is unset when no branch is specified | 52 | ported | `git_submodules.rs` | `single_submodule_no_branch` | — |
| given branch is used when branch is specified | 58 | ported | `git_submodules.rs` | `single_submodule_with_branch` | — |
| submodule packageName is constructed from relative path | 64 | pending | — | — | — |
| fallback to current branch if special value is detected | 89 | ported | `git_submodules.rs` | `branch_dot_normalized_to_none` | — |
| given semver version is extracted from branch and versioning is set to semver | 127 | ported | `git_submodules.rs` | `semver_and_non_semver_branches` | — |

### `extractPackageFile() › submodule sourceUrl is determined from packageName`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| when using SSH clone URL | 73 | pending | — | — | — |
| when using a relative path | 80 | pending | — | — | — |

---

## `lib/modules/manager/gomod/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gomod/extract.spec.ts
**Total tests:** 21 | **Ported:** 5 | **Actionable:** 21 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 12 | ported | `gomod.rs` | `empty_content_returns_empty` | — |
| extracts single-line requires | 16 | ported | `gomod.rs` | `single_line_require` | — |
| extracts multi-line requires | 26 | ported | `gomod.rs` | `require_block` | — |
| ignores empty spaces in multi-line requires | 34 | ported | `gomod.rs` | `empty_lines_inside_require_block` | — |
| extracts replace directives from multi-line and single line | 48 | pending | — | — | — |
| extracts replace directives from non-public module path | 136 | pending | — | — | — |
| ignores exclude directives from multi-line and single line | 193 | ported | `gomod.rs` | `exclude_block_ignored` | — |
| extracts the toolchain directive | 212 | pending | — | — | — |
| extracts single-line tool directives | 263 | pending | — | — | — |
| extracts multi-line tool directives | 282 | pending | — | — | — |
| extracts tool directives with required modules | 304 | pending | — | — | — |
| extracts tool directives of sub-modules | 323 | pending | — | — | — |
| extracts tool directives with exact match | 370 | pending | — | — | — |
| extracts tool directives with no matching dependencies | 389 | pending | — | — | — |
| ignores directives unrelated to dependencies | 402 | pending | — | — | — |
| marks placeholder pseudo versions with skipReason invalid-version | 426 | pending | — | — | — |
| parses go $version directive | 528 | pending | — | — | — |
| the extracted version can be used as a SemVer constraint | 582 | pending | — | — | — |
| matches version 1.19, even though it is not valid SemVer | 586 | pending | — | — | — |
| matches the current SemVer minor | 590 | pending | — | — | — |
| does not match the next SemVer minor | 595 | pending | — | — | — |

---

## `lib/modules/manager/helm-values/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/helm-values/extract.spec.ts
**Total tests:** 6 | **Ported:** 3 | **Actionable:** 6 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid yaml file content | 26 | ported | `helm_values.rs` | `invalid_yaml_returns_empty` | — |
| returns null for empty yaml file content | 31 | ported | `helm_values.rs` | `empty_returns_empty` | — |
| extracts from values.yaml correctly with same structure as "helm create" | 36 | ported | `helm_values.rs` | `helm_create_default_values` | — |
| extracts from complex values file correctly | 52 | pending | — | — | — |
| extract data from file with multiple documents | 62 | pending | — | — | — |
| extract data from file with registry aliases | 85 | pending | — | — | — |

---

## `lib/modules/manager/helmsman/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/helmsman/extract.spec.ts
**Total tests:** 4 | **Ported:** 1 | **Actionable:** 4 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if empty | 9 | ported | `helmsman.rs` | `empty_file_returns_empty` | — |
| returns null if extracting non helmsman yaml file | 16 | pending | — | — | — |
| returns null if apps not defined | 23 | pending | — | — | — |
| extract deps | 29 | pending | — | — | — |

---

## `lib/modules/manager/hermit/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/hermit/extract.spec.ts
**Total tests:** 2 | **Ported:** 1 | **Actionable:** 2 | **Status:** partial

### `extractPackageFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should list packages on command success | 11 | ported | `hermit.rs` | `extracts_versioned_packages` | — |
| should throw error on execution failure | 75 | pending | — | — | — |

---

## `lib/modules/manager/bitbucket-pipelines/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bitbucket-pipelines/extract.spec.ts
**Total tests:** 4 | **Ported:** 2 | **Actionable:** 4 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 6 | ported | `bitbucket_pipelines.rs` | `empty_returns_empty` | — |
| returns null for malformed | 12 | ported | `bitbucket_pipelines.rs` | `malformed_image_object_without_name_returns_empty` | — |
| extracts dependencies | 22 | pending | — | — | — |
| extracts dependencies with registryAlias | 82 | pending | — | — | — |

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
**Total tests:** 8 | **Ported:** 6 | **Actionable:** 8 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for property file without distributionUrl | 24 | ported | `gradle_wrapper.rs` | `no_distribution_url_returns_none` | — |
| returns null for property file with unsupported distributionUrl format | 28 | ported | `gradle_wrapper.rs` | `unsupported_url_format_returns_none` | — |
| extracts version for property file with distribution type "bin" in distributionUrl | 33 | ported | `gradle_wrapper.rs` | `extracts_bin_version` | — |
| extracts version for property file with distribution type "all" in distributionUrl | 47 | ported | `gradle_wrapper.rs` | `extracts_all_version` | — |
| extracts version for property file with prerelease version in distributionUrl | 61 | ported | `gradle_wrapper.rs` | `prerelease_version_extracted` | — |
| extracts version for property file with unnecessary whitespace in distributionUrl | 75 | ported | `gradle_wrapper.rs` | `whitespace_around_value_handled` | — |
| extracts version for property file with custom distribution of type "bin" in distributionUrl | 89 | pending | — | — | — |
| extracts version for property file with custom distribution of type "all" in distributionUrl | 103 | pending | — | — | — |

---

## `lib/modules/manager/buildkite/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/buildkite/extract.spec.ts
**Total tests:** 11 | **Ported:** 6 | **Actionable:** 11 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 7 | ported | `buildkite.rs` | `empty_content_returns_no_deps` | — |
| extracts simple single plugin | 11 | ported | `buildkite.rs` | `two_part_plugin` | — |
| extracts multiple plugins in same file | 22 | ported | `buildkite.rs` | `one_part_plugin` | — |
| adds skipReason | 47 | ported | `buildkite.rs` | `non_semver_version_skipped` | — |
| extracts arrays of plugins | 70 | ported | `buildkite.rs` | `array_plugins_extracted` | — |
| extracts git-based plugins | 92 | ported | `buildkite.rs` | `github_url_plugin` | — |
| extracts git-based plugin with .git at the end of its name | 105 | pending | — | — | — |
| extracts plugins outside plugins sections | 121 | pending | — | — | — |
| extracts plugin with preceding ? | 140 | pending | — | — | — |
| extracts plugin tags from bitbucket | 155 | pending | — | — | — |
| extracts plugin tags with quotes | 178 | pending | — | — | — |

---

## `lib/modules/manager/circleci/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/circleci/extract.spec.ts
**Total tests:** 9 | **Ported:** 2 | **Actionable:** 9 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 12 | ported | `circleci.rs` | `empty_content_returns_no_deps` | — |
| handles registry alias | 16 | pending | — | — | — |
| extracts multiple image and resolves yaml anchors | 48 | pending | — | — | — |
| extracts orbs too | 93 | ported | `circleci.rs` | `extracts_orbs` | — |
| extracts image without leading dash | 200 | pending | — | — | — |
| extracts and exclude android images | 226 | pending | — | — | — |
| extracts orbs without jobs | 237 | pending | — | — | — |
| extracts executors | 251 | pending | — | — | — |
| extracts orb definitions | 273 | pending | — | — | — |

---

## `lib/modules/manager/composer/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/composer/extract.spec.ts
**Total tests:** 10 | **Ported:** 1 | **Actionable:** 10 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid json | 24 | pending | — | — | — |
| returns null for empty deps | 28 | ported | `composer.rs` | `empty_content_ok` | — |
| extracts dependencies with no lock file | 32 | pending | — | — | — |
| extracts registryUrls | 38 | pending | — | — | — |
| extracts object registryUrls | 81 | pending | — | — | — |
| extracts repositories and registryUrls | 186 | pending | — | — | — |
| extracts bitbucket repositories and registryUrls | 219 | pending | — | — | — |
| extracts object repositories and registryUrls with lock file | 248 | pending | — | — | — |
| skips path dependencies | 284 | pending | — | — | — |
| extracts dependencies with lock file | 313 | pending | — | — | — |

---

## `lib/modules/manager/conan/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/conan/extract.spec.ts
**Total tests:** 4 | **Ported:** 2 | **Actionable:** 4 | **Status:** partial

### `extractPackageFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 10 | ported | `conan.rs` | `empty_returns_empty` | — |
| extracts multiple image lines from conanfile.txt | 14 | pending | — | — | — |
| extracts multiple 0 lines from conanfile.txt | 129 | pending | — | — | — |
| extracts multiple image lines from conanfile.py | 134 | ported | `conan.rs` | `extracts_py_requires` | — |

---

## `lib/modules/manager/copier/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/copier/extract.spec.ts
**Total tests:** 9 | **Ported:** 6 | **Actionable:** 9 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts repository and version from .copier-answers.yml | 6 | ported | `copier.rs` | `extracts_github_url` | — |
| extracts repository and version from .copier-answers.yml with ssh URL | 25 | ported | `copier.rs` | `ssh_url_src_path_extracted` | — |
| extracts repository and version from .copier-answers.yml with ssh URL and non-bare Repo | 44 | pending | — | — | — |
| extracts repository and version from .copier-answers.yml with ssh URL and a username different from git | 63 | pending | — | — | — |
| extracts and strips git+ prefix from $srcPath | 84 | ported | `copier.rs` | `strips_git_plus_prefix` | — |
| returns null for invalid .copier-answers.yml | 119 | ported | `copier.rs` | `invalid_yaml_returns_none` | — |
| returns null for invalid _src_path | 128 | pending | — | — | — |
| returns null for missing _commit field | 137 | ported | `copier.rs` | `missing_commit_returns_none` | — |
| returns null for missing _src_path field | 145 | ported | `copier.rs` | `missing_src_path_returns_none` | — |

---

## `lib/modules/manager/crossplane/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/crossplane/extract.spec.ts
**Total tests:** 9 | **Ported:** 5 | **Actionable:** 9 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 12 | ported | `crossplane.rs` | `empty_content_returns_empty` | — |
| strips invalid templates | 16 | pending | — | — | — |
| return null for kubernetes manifest | 20 | ported | `crossplane.rs` | `skips_non_crossplane_files` | — |
| return invalid-value if deps are not valid images and ignore if missing | 25 | pending | — | — | — |
| return result for double quoted pkg.crossplane.io apiVersion reference | 37 | ported | `crossplane.rs` | `double_quoted_api_version_extracted` | — |
| return result for single quoted pkg.crossplane.io apiVersion reference | 58 | ported | `crossplane.rs` | `single_quoted_api_version_extracted` | — |
| return no results for invalid resource | 79 | ported | `crossplane.rs` | `reports_missing_package` | — |
| full test | 94 | pending | — | — | — |
| should work even if there are other resources in the file | 137 | ported | `crossplane.rs` | `handles_multi_document` | — |

---

## `lib/modules/manager/crow/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/crow/extract.spec.ts
**Total tests:** 15 | **Ported:** 7 | **Actionable:** 15 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 6 | ported | `crow.rs` | `empty_returns_empty` | — |
| returns null for non-object YAML | 10 | ported | `crow.rs` | `no_image_keys_returns_empty` | — |
| returns null for malformed YAML | 15 | pending | — | — | — |
| extracts multiple image lines | 19 | ported | `crow.rs` | `extracts_pipeline_images` | — |
| extracts image and replaces registry | 164 | pending | — | — | — |
| extracts image but no replacement | 194 | pending | — | — | — |
| extracts image and no double replacement | 224 | pending | — | — | — |
| extracts the 1.0.0 version | 255 | pending | — | — | — |
| should parse multiple sources of dependencies together | 281 | pending | — | — | — |
| return dependency when a plugin-git is cloned | 321 | ported | `crow.rs` | `clone_section` | — |
| return null when no dependencies are provided | 348 | ported | `crow.rs` | `no_dependencies_returns_empty` | — |
| handles empty pipeline section gracefully | 362 | pending | — | — | — |
| returns null when pipeline keys exist but contain no valid images | 390 | ported | `crow.rs` | `pipeline_without_valid_images_returns_empty` | — |
| extracts images from array-based steps format | 408 | ported | `crow.rs` | `steps_as_array` | — |
| extracts images from mixed array and object formats | 447 | pending | — | — | — |

---

## `lib/modules/manager/devbox/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/devbox/extract.spec.ts
**Total tests:** 13 | **Ported:** 9 | **Actionable:** 13 | **Status:** partial

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
| returns a package dependency when the devbox JSON file has multiple packages with package objects | 144 | pending | — | — | — |
| returns invalid dependencies | 177 | ported | `devbox.rs` | `mixed_valid_and_invalid_versions` | — |
| returns invalid dependencies with package objects | 213 | pending | — | — | — |
| returns invalid dependencies from the packages array | 251 | pending | — | — | — |
| returns null if there are no dependencies | 288 | pending | — | — | — |

---

## `lib/modules/manager/devcontainer/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/devcontainer/extract.spec.ts
**Total tests:** 15 | **Ported:** 7 | **Actionable:** 15 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when the dev container JSON file is empty | 10 | pending | — | — | — |
| returns null when the dev container JSON file contents are malformed | 22 | ported | `devcontainer.rs` | `invalid_json_returns_empty` | — |
| tests if JSONC can be parsed | 34 | pending | — | — | — |
| returns feature image deps when only the features property is defined in dev container JSON file | 72 | ported | `devcontainer.rs` | `extracts_node_feature_and_version` | — |
| returns image and feature image deps when both image and features properties are defined in dev container JSON file | 124 | ported | `devcontainer.rs` | `image_and_feature_combined` | — |
| returns image dep when only the image property is defined in dev container JSON file | 174 | ported | `devcontainer.rs` | `extracts_image` | — |
| returns null when the only feature property is malformed and no image property is defined in dev container JSON file | 207 | pending | — | — | — |
| returns null when the features property is malformed and no image property is defined in dev container JSON file | 227 | pending | — | — | — |
| returns null when the image property is malformed and no features are defined in dev container JSON file | 245 | pending | — | — | — |
| returns null when no image or features properties are defined in dev container JSON file | 263 | ported | `devcontainer.rs` | `empty_object_returns_empty` | — |
| returns null when the features property is null and no image property is defined in dev container JSON file | 278 | ported | `devcontainer.rs` | `null_features_value_returns_empty` | — |
| returns null when the features property is not defined and the image property is null in dev container JSON file | 296 | ported | `devcontainer.rs` | `no_image_returns_empty` | — |
| returns null when both the image and features properties are null | 314 | pending | — | — | — |
| returns only docker dependencies when non-docker feature types are defined beneath the features property in dev container JSON file | 333 | pending | — | — | — |
| parses known tool versions | 372 | pending | — | — | — |

---

## `lib/modules/manager/docker-compose/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/docker-compose/extract.spec.ts
**Total tests:** 13 | **Ported:** 4 | **Actionable:** 13 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 12 | ported | `docker_compose.rs` | `empty_content_returns_empty` | — |
| returns null for non-object YAML | 16 | pending | — | — | — |
| returns null for malformed YAML | 20 | pending | — | — | — |
| extracts multiple image lines for version 1 | 24 | ported | `docker_compose.rs` | `renovate_fixture_1_v1_format` | — |
| extracts multiple image lines for version 3 | 30 | ported | `docker_compose.rs` | `extracts_images_from_compose_v3` | — |
| extracts multiple image lines for version 3 without set version key | 36 | pending | — | — | — |
| extracts default variable values for version 3 | 42 | ported | `docker_compose.rs` | `variable_interpolation_is_skipped` | — |
| extracts can parse yaml tags for version 3 | 59 | pending | — | — | — |
| extracts image and replaces registry | 87 | pending | — | — | registryAliases not yet implemented |
| extracts image but no replacement | 115 | pending | — | — | registryAliases not yet implemented |
| extracts image and no double replacement | 143 | pending | — | — | registryAliases not yet implemented |
| extracts image of templated compose file | 172 | pending | — | — | — |
| extract images from fragments | 198 | pending | — | — | YAML anchors not resolved |

---

## `lib/modules/manager/fleet/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/fleet/extract.spec.ts
**Total tests:** 10 | **Ported:** 5 | **Actionable:** 10 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null if empty content | 24 | ported | `fleet.rs` | `empty_content_returns_empty` | — |
| should return null if a unknown manifest is supplied | 30 | ported | `fleet.rs` | `unknown_manifest_returns_empty` | — |
| should return null if content is a malformed YAML (fleet.yaml) | 37 | pending | — | — | — |
| should parse valid configuration (fleet.yaml) | 49 | ported | `fleet.rs` | `extracts_helm_dep_from_fleet_yaml` | — |
| should support registryAlias configuration | 88 | pending | — | — | registryAliases not yet implemented |
| should parse valid configuration with target customization | 132 | ported | `fleet.rs` | `extracts_target_customizations` | — |
| should parse parse invalid configurations | 208 | pending | — | — | — |
| should return null if content is a malformed YAML (GitRepo) | 242 | pending | — | — | — |
| should parse valid configuration (GitRepo) | 254 | ported | `fleet.rs` | `extracts_gitrepo_dep` | — |
| should parse invalid configuration (GitRepo) | 276 | pending | — | — | — |

---

## Managers (`lib/modules/manager/`) — legacy summary

### Extract specs

| Renovate spec file | Renovate tests | Rust file | Rust tests | Status |
|--------------------|---------------|-----------|------------|--------|
| `lib/modules/manager/ant/extract.spec.ts` | 44 | `crates/renovate-core/src/extractors/ant.rs` | 6 | partial |
| `lib/modules/manager/asdf/extract.spec.ts` | 13 | `crates/renovate-core/src/extractors/asdf.rs` | 28 | partial |
| `lib/modules/manager/azure-pipelines/extract.spec.ts` | 29 | `crates/renovate-core/src/extractors/azure_pipelines.rs` | 8 | partial |
| `lib/modules/manager/bazel-module/extract.spec.ts` | 35 | `crates/renovate-core/src/extractors/bazel_module.rs` | 7 | partial |
| `lib/modules/manager/bazel/extract.spec.ts` | 12 | `crates/renovate-core/src/extractors/bazel.rs` | 10 | partial |
| `lib/modules/manager/bicep/extract.spec.ts` | 9 | `crates/renovate-core/src/extractors/bicep.rs` | 5 | partial |
| `lib/modules/manager/cargo/extract.spec.ts` | 32 | `crates/renovate-core/src/extractors/cargo.rs` | 16 | partial |
| `lib/modules/manager/cpanfile/extract.spec.ts` | 4 | `crates/renovate-core/src/extractors/cpanfile.rs` | 8 | partial |
| `lib/modules/manager/dockerfile/extract.spec.ts` | 75 | `crates/renovate-core/src/extractors/dockerfile.rs` | 16 | partial |
| `lib/modules/manager/flux/extract.spec.ts` | 58 | `crates/renovate-core/src/extractors/flux.rs` | 5 | partial |
| `lib/modules/manager/github-actions/extract.spec.ts` | 26 | `crates/renovate-core/src/extractors/github_actions.rs` | 28 | partial |
| `lib/modules/manager/gitlabci/extract.spec.ts` | 14 | `crates/renovate-core/src/extractors/gitlabci.rs` | 8 | partial |
| `lib/modules/manager/gradle/extract.spec.ts` | 30 | `crates/renovate-core/src/extractors/gradle.rs` | 20 | partial |
| `lib/modules/manager/helm-requirements/extract.spec.ts` | 11 | `crates/renovate-core/src/extractors/helm.rs` | 10 | partial |
| `lib/modules/manager/helmfile/extract.spec.ts` | 19 | `crates/renovate-core/src/extractors/helmfile.rs` | 10 | partial |
| `lib/modules/manager/homeassistant-manifest/extract.spec.ts` | 16 | `crates/renovate-core/src/extractors/homeassistant.rs` | 4 | partial |
| `lib/modules/manager/homebrew/extract.spec.ts` | 17 | `crates/renovate-core/src/extractors/homebrew.rs` | 9 | partial |
| `lib/modules/manager/html/extract.spec.ts` | 2 | `crates/renovate-core/src/extractors/html.rs` | 6 | partial |
| `lib/modules/manager/jenkins/extract.spec.ts` | 5 | `crates/renovate-core/src/extractors/jenkins.rs` | 9 | partial |
| `lib/modules/manager/jsonnet-bundler/extract.spec.ts` | 7 | `crates/renovate-core/src/extractors/jsonnet_bundler.rs` | 5 | partial |
| `lib/modules/manager/kotlin-script/extract.spec.ts` | 4 | `crates/renovate-core/src/extractors/kotlin_script.rs` | 5 | partial |
| `lib/modules/manager/kubernetes/extract.spec.ts` | 14 | `crates/renovate-core/src/extractors/kubernetes.rs` | 5 | partial |
| `lib/modules/manager/kustomize/extract.spec.ts` | 43 | `crates/renovate-core/src/extractors/kustomize.rs` | 6 | partial |
| `lib/modules/manager/leiningen/extract.spec.ts` | 4 | `crates/renovate-core/src/extractors/leiningen.rs` | 8 | partial |
| `lib/modules/manager/maven-wrapper/extract.spec.ts` | 9 | `crates/renovate-core/src/extractors/maven_wrapper.rs` | 5 | partial |
| `lib/modules/manager/maven/extract.spec.ts` | 29 | `crates/renovate-core/src/extractors/maven.rs` | 19 | partial |
| `lib/modules/manager/meteor/extract.spec.ts` | 2 | `crates/renovate-core/src/extractors/meteor.rs` | 3 | partial |
| `lib/modules/manager/mint/extract.spec.ts` | 5 | `crates/renovate-core/src/extractors/mint.rs` | 5 | partial |
| `lib/modules/manager/mise/extract.spec.ts` | 30 | `crates/renovate-core/src/extractors/mise.rs` | 9 | partial |
| `lib/modules/manager/mix/extract.spec.ts` | 3 | `crates/renovate-core/src/extractors/mix.rs` | 9 | partial |
| `lib/modules/manager/nix/extract.spec.ts` | 38 | `crates/renovate-core/src/extractors/nix.rs` | 5 | partial |
| `lib/modules/manager/nuget/extract.spec.ts` | 35 | `crates/renovate-core/src/extractors/nuget.rs` | 19 | partial |
| `lib/modules/manager/ocb/extract.spec.ts` | 3 | `crates/renovate-core/src/extractors/ocb.rs` | 4 | partial |
| `lib/modules/manager/osgi/extract.spec.ts` | 14 | `crates/renovate-core/src/extractors/osgi.rs` | 10 | partial |
| `lib/modules/manager/pep621/extract.spec.ts` | 14 | `crates/renovate-core/src/extractors/pep621.rs` | 11 | partial |
| `lib/modules/manager/pep723/extract.spec.ts` | 1 | `crates/renovate-core/src/extractors/pep723.rs` | 6 | partial |
| `lib/modules/manager/pip-compile/extract.spec.ts` | 25 | — | 0 | pending |
| `lib/modules/manager/pip_requirements/extract.spec.ts` | 22 | `crates/renovate-core/src/extractors/pip.rs` | 18 | partial |
| `lib/modules/manager/pip_setup/extract.spec.ts` | 2 | `crates/renovate-core/src/extractors/pip_setup.rs` | 6 | partial |
| `lib/modules/manager/pipenv/extract.spec.ts` | 16 | `crates/renovate-core/src/extractors/pipfile.rs` | 11 | partial |
| `lib/modules/manager/pixi/extract.spec.ts` | 16 | `crates/renovate-core/src/extractors/pixi.rs` | 7 | partial |
| `lib/modules/manager/poetry/extract.spec.ts` | 34 | `crates/renovate-core/src/extractors/poetry.rs` | 12 | partial |
| `lib/modules/manager/pre-commit/extract.spec.ts` | 12 | `crates/renovate-core/src/extractors/pre_commit.rs` | 9 | partial |
| `lib/modules/manager/puppet/extract.spec.ts` | 9 | `crates/renovate-core/src/extractors/puppet.rs` | 9 | partial |
| `lib/modules/manager/quadlet/extract.spec.ts` | 11 | `crates/renovate-core/src/extractors/quadlet.rs` | 13 | partial |
| `lib/modules/manager/runtime-version/extract.spec.ts` | 2 | `crates/renovate-core/src/extractors/runtime_version.rs` | 4 | partial |
| `lib/modules/manager/sbt/extract.spec.ts` | 26 | `crates/renovate-core/src/extractors/sbt.rs` | 7 | partial |
| `lib/modules/manager/scalafmt/extract.spec.ts` | 4 | `crates/renovate-core/src/extractors/scalafmt.rs` | 4 | partial |
| `lib/modules/manager/setup-cfg/extract.spec.ts` | 2 | `crates/renovate-core/src/extractors/setup_cfg.rs` | 9 | partial |
| `lib/modules/manager/sveltos/extract.spec.ts` | 13 | `crates/renovate-core/src/extractors/sveltos.rs` | 7 | partial |
| `lib/modules/manager/tekton/extract.spec.ts` | 5 | `crates/renovate-core/src/extractors/tekton.rs` | 5 | partial |
| `lib/modules/manager/terraform/extract.spec.ts` | 18 | `crates/renovate-core/src/extractors/terraform.rs` | 10 | partial |
| `lib/modules/manager/terragrunt/extract.spec.ts` | 7 | `crates/renovate-core/src/extractors/terragrunt.rs` | 5 | partial |
| `lib/modules/manager/tflint-plugin/extract.spec.ts` | 6 | `crates/renovate-core/src/extractors/tflint_plugin.rs` | 5 | partial |
| `lib/modules/manager/travis/extract.spec.ts` | 8 | `crates/renovate-core/src/extractors/travis.rs` | 5 | partial |
| `lib/modules/manager/typst/extract.spec.ts` | 9 | `crates/renovate-core/src/extractors/typst.rs` | 7 | partial |
| `lib/modules/manager/unity3d/extract.spec.ts` | 0 | `crates/renovate-core/src/extractors/unity3d.rs` | 3 | partial |
| `lib/modules/manager/velaci/extract.spec.ts` | 6 | `crates/renovate-core/src/extractors/velaci.rs` | 6 | partial |
| `lib/modules/manager/vendir/extract.spec.ts` | 5 | `crates/renovate-core/src/extractors/vendir.rs` | 5 | partial |
| `lib/modules/manager/woodpecker/extract.spec.ts` | 11 | `crates/renovate-core/src/extractors/woodpecker.rs` | 7 | partial |
| `lib/modules/manager/xcodegen/extract.spec.ts` | 24 | `crates/renovate-core/src/extractors/xcodegen.rs` | 11 | partial |

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
| `lib/modules/manager/ruby-version/extract.spec.ts` | 3 | `crates/renovate-core/src/extractors/version_file.rs` | 17 | partial |

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
