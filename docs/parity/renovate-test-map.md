# Renovate Test Map

**Overall progress:** 24 / 46 actionable tests ported (52%) — updated 2026-04-29

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
| `lib/modules/manager/bitbucket-pipelines/extract.spec.ts` | 4 | `crates/renovate-core/src/extractors/bitbucket_pipelines.rs` | 8 | partial |
| `lib/modules/manager/bitrise/extract.spec.ts` | 6 | `crates/renovate-core/src/extractors/bitrise.rs` | 10 | partial |
| `lib/modules/manager/buildkite/extract.spec.ts` | 11 | `crates/renovate-core/src/extractors/buildkite.rs` | 7 | partial |
| `lib/modules/manager/cargo/extract.spec.ts` | 32 | `crates/renovate-core/src/extractors/cargo.rs` | 16 | partial |
| `lib/modules/manager/circleci/extract.spec.ts` | 9 | `crates/renovate-core/src/extractors/circleci.rs` | 10 | partial |
| `lib/modules/manager/composer/extract.spec.ts` | 10 | `crates/renovate-core/src/extractors/composer.rs` | 9 | partial |
| `lib/modules/manager/conan/extract.spec.ts` | 4 | `crates/renovate-core/src/extractors/conan.rs` | 5 | partial |
| `lib/modules/manager/copier/extract.spec.ts` | 8 | `crates/renovate-core/src/extractors/copier.rs` | 8 | partial |
| `lib/modules/manager/cpanfile/extract.spec.ts` | 4 | `crates/renovate-core/src/extractors/cpanfile.rs` | 8 | partial |
| `lib/modules/manager/crossplane/extract.spec.ts` | 9 | `crates/renovate-core/src/extractors/crossplane.rs` | 7 | partial |
| `lib/modules/manager/crow/extract.spec.ts` | 15 | `crates/renovate-core/src/extractors/crow.rs` | 10 | partial |
| `lib/modules/manager/devbox/extract.spec.ts` | 13 | `crates/renovate-core/src/extractors/devbox.rs` | 9 | partial |
| `lib/modules/manager/devcontainer/extract.spec.ts` | 15 | `crates/renovate-core/src/extractors/devcontainer.rs` | 10 | partial |
| `lib/modules/manager/docker-compose/extract.spec.ts` | 13 | `crates/renovate-core/src/extractors/docker_compose.rs` | 10 | partial |
| `lib/modules/manager/dockerfile/extract.spec.ts` | 75 | `crates/renovate-core/src/extractors/dockerfile.rs` | 16 | partial |
| `lib/modules/manager/fleet/extract.spec.ts` | 10 | `crates/renovate-core/src/extractors/fleet.rs` | 10 | partial |
| `lib/modules/manager/flux/extract.spec.ts` | 58 | `crates/renovate-core/src/extractors/flux.rs` | 5 | partial |
| `lib/modules/manager/fvm/extract.spec.ts` | 7 | `crates/renovate-core/src/extractors/fvm.rs` | 5 | partial |
| `lib/modules/manager/git-submodules/extract.spec.ts` | 8 | `crates/renovate-core/src/extractors/git_submodules.rs` | 11 | partial |
| `lib/modules/manager/github-actions/extract.spec.ts` | 26 | `crates/renovate-core/src/extractors/github_actions.rs` | 28 | partial |
| `lib/modules/manager/gitlabci/extract.spec.ts` | 14 | `crates/renovate-core/src/extractors/gitlabci.rs` | 8 | partial |
| `lib/modules/manager/glasskube/extract.spec.ts` | 5 | `crates/renovate-core/src/extractors/glasskube.rs` | 3 | partial |
| `lib/modules/manager/gleam/extract.spec.ts` | 9 | `crates/renovate-core/src/extractors/gleam.rs` | 6 | partial |
| `lib/modules/manager/gomod/extract.spec.ts` | 21 | `crates/renovate-core/src/extractors/gomod.rs` | 9 | partial |
| `lib/modules/manager/gradle-wrapper/extract.spec.ts` | 8 | `crates/renovate-core/src/extractors/gradle_wrapper.rs` | 8 | partial |
| `lib/modules/manager/gradle/extract.spec.ts` | 30 | `crates/renovate-core/src/extractors/gradle.rs` | 20 | partial |
| `lib/modules/manager/helm-values/extract.spec.ts` | 6 | `crates/renovate-core/src/extractors/helm_values.rs` | 7 | partial |
| `lib/modules/manager/helm-requirements/extract.spec.ts` | 11 | `crates/renovate-core/src/extractors/helm.rs` | 10 | partial |
| `lib/modules/manager/helmfile/extract.spec.ts` | 19 | `crates/renovate-core/src/extractors/helmfile.rs` | 10 | partial |
| `lib/modules/manager/helmsman/extract.spec.ts` | 4 | `crates/renovate-core/src/extractors/helmsman.rs` | 4 | partial |
| `lib/modules/manager/hermit/extract.spec.ts` | 2 | `crates/renovate-core/src/extractors/hermit.rs` | 5 | partial |
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
