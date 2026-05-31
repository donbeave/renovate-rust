# Renovate Source Map

Maps Renovate TypeScript **source** files to their Rust counterparts.
This file tracks source-level port coverage. Only `.ts` source files appear here (never `.spec.ts`).

**Status:** `full` · `partial` · `not-started` · `out-of-scope`

---

## Managers (`lib/modules/manager/`)

### ansible

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/ansible/extract.ts` | `extractors/ansible.rs` | full | `extract()` extracts Docker image refs from Ansible task YAML |
| `lib/modules/manager/ansible/index.ts` | `managers.rs` | partial | fileMatch pattern present; `defaultConfig` managerFilePatterns covered; `supportedDatasources`, `categories`, `url` not stored |

### ansible-galaxy

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/ansible-galaxy/extract.ts` | `extractors/ansible_galaxy.rs` | full | `extract()` handles roles and collections from YAML |
| `lib/modules/manager/ansible-galaxy/index.ts` | `managers.rs` | partial | fileMatch pattern present; full config not stored |
| `lib/modules/manager/ansible-galaxy/collections-metadata.ts` | `extractors/ansible_galaxy.rs` | partial | Collection metadata extraction is inline |
| `lib/modules/manager/ansible-galaxy/collections.ts` | `extractors/ansible_galaxy.rs` | partial | Collection parsing is inline |
| `lib/modules/manager/ansible-galaxy/dep-types.ts` | `extractors/ansible_galaxy.rs` | full | Dep types are embedded as enum fields |
| `lib/modules/manager/ansible-galaxy/roles.ts` | `extractors/ansible_galaxy.rs` | partial | Roles parsing is inline |
| `lib/modules/manager/ansible-galaxy/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/ansible-galaxy/util.ts` | `extractors/ansible_galaxy.rs` | partial | Utility functions inline |

### ant

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/ant/extract.ts` | `extractors/ant.rs` | full | `extract()` + `extract_all_package_files()` present; handles Maven deps from XML |
| `lib/modules/manager/ant/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/ant/properties.ts` | `extractors/ant.rs` | full | Property resolution is inline |
| `lib/modules/manager/ant/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/ant/update.ts` | `extractors/ant.rs` | full | `update_dependency()` present |

### argocd

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/argocd/extract.ts` | `extractors/argocd.rs` | full | `extract()` handles ArgoCD Application sources |
| `lib/modules/manager/argocd/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/argocd/schema.ts` | N/A | out-of-scope | Zod schema (TS-specific) |
| `lib/modules/manager/argocd/util.ts` | `extractors/argocd.rs` | partial | Utility functions inline |

### asdf

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/asdf/extract.ts` | `extractors/asdf.rs` | full | `extract()` parses `.tool-versions` format |
| `lib/modules/manager/asdf/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/asdf/upgradeable-tooling.ts` | `extractors/asdf.rs` + `extractors/mise.rs` | full | Full tooling table with datasource mappings, all 30+ tools present |

### azure-pipelines

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/azure-pipelines/extract.ts` | `extractors/azure_pipelines.rs` | full | `extract()` handles Docker images from Azure Pipelines YAML |
| `lib/modules/manager/azure-pipelines/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/azure-pipelines/dep-types.ts` | `extractors/azure_pipelines.rs` | full | Dep types embedded in extractor |
| `lib/modules/manager/azure-pipelines/schema.ts` | N/A | out-of-scope | Zod schema |

### batect

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/batect/extract.ts` | `extractors/batect.rs` | full | `extract()` handles batect YAML config |
| `lib/modules/manager/batect/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/batect/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/batect/types.ts` | N/A | out-of-scope | Type-only file |

### batect-wrapper

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/batect-wrapper/extract.ts` | `extractors/batect_wrapper.rs` | full | `extract()` handles wrapper config |
| `lib/modules/manager/batect-wrapper/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/batect-wrapper/artifacts.ts` | N/A | not-started | Artifact execution not implemented |

### bazel

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/bazel/extract.ts` | `extractors/bazel.rs` | full | `extract()` handles WORKSPACE/bzl files |
| `lib/modules/manager/bazel/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/bazel/common.ts` | `extractors/bazel.rs` | full | Common utilities inline |
| `lib/modules/manager/bazel/dep-types.ts` | `extractors/bazel.rs` | full | Dep types embedded as enum |
| `lib/modules/manager/bazel/parser.ts` | `extractors/bazel_parser.rs` | full | Full Starlark parser with `parse()` and `update_code()` |
| `lib/modules/manager/bazel/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/bazel/rules/docker.ts` | `extractors/bazel.rs` | full | `container_pull` / `oci_pull` parsing present |
| `lib/modules/manager/bazel/rules/git.ts` | `extractors/bazel.rs` | full | Git archive URL parsing present |
| `lib/modules/manager/bazel/rules/go.ts` | `extractors/bazel.rs` | full | `go_repository()` parsing present |
| `lib/modules/manager/bazel/rules/http.ts` | `extractors/bazel.rs` | full | `http_archive()` parsing present |
| `lib/modules/manager/bazel/rules/index.ts` | `extractors/bazel.rs` | full | Rule dispatch inline |
| `lib/modules/manager/bazel/rules/maven.ts` | `extractors/bazel.rs` | full | `maven_install()` parsing present |
| `lib/modules/manager/bazel/rules/oci.ts` | `extractors/bazel.rs` | full | `oci_pull()` parsing present |
| `lib/modules/manager/bazel/types.ts` | N/A | out-of-scope | Type-only file |

### bazel-module

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/bazel-module/extract.ts` | `extractors/bazel_module.rs` | full | Full MODULE.bazel extraction |
| `lib/modules/manager/bazel-module/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/bazel-module/dep-types.ts` | `extractors/bazel_module.rs` | full | Dep types embedded |
| `lib/modules/manager/bazel-module/rules.ts` | `extractors/bazel_module.rs` | full | Rule extraction inline |
| `lib/modules/manager/bazel-module/rules-img.ts` | `extractors/bazel_module.rs` | full | Image pull rules inline |
| `lib/modules/manager/bazel-module/lockfile.ts` | `extractors/bazel_module.rs` | partial | Some lockfile handling |
| `lib/modules/manager/bazel-module/bazelrc.ts` | `extractors/bazel_module.rs` | full | `.bazelrc` parsing present |
| `lib/modules/manager/bazel-module/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/bazel-module/parser/common.ts` | `extractors/bazel_module.rs` | full | Parser common utilities inline |
| `lib/modules/manager/bazel-module/parser/context.ts` | `extractors/bazel_module.rs` | full | `BazelCtx` context handling present |
| `lib/modules/manager/bazel-module/parser/crate.ts` | `extractors/bazel_module.rs` | full | Crate spec parsing present |
| `lib/modules/manager/bazel-module/parser/extension-tags.ts` | `extractors/bazel_module.rs` | full | Extension tag parsing present |
| `lib/modules/manager/bazel-module/parser/fragments.ts` | `extractors/bazel_parser.rs` | full | Parser fragments |
| `lib/modules/manager/bazel-module/parser/index.ts` | `extractors/bazel_module.rs` | full | Parser dispatch inline |
| `lib/modules/manager/bazel-module/parser/maven.ts` | `extractors/bazel_module.rs` | full | Maven parsing present |
| `lib/modules/manager/bazel-module/parser/oci.ts` | `extractors/bazel_module.rs` | full | OCI parsing present |
| `lib/modules/manager/bazel-module/parser/repo-rules.ts` | `extractors/bazel_module.rs` | full | Repo rule parsing present |
| `lib/modules/manager/bazel-module/parser/rules.ts` | `extractors/bazel_module.rs` | full | Rule parsing present |
| `lib/modules/manager/bazel-module/parser/starlark.ts` | `extractors/bazel_parser.rs` | full | Starlark parsing present |

### bazelisk

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/bazelisk/extract.ts` | `extractors/bazelisk.rs` | full | `extract_bazelisk()` present |
| `lib/modules/manager/bazelisk/index.ts` | `managers.rs` | partial | fileMatch pattern present; `supportsLockFileMaintenance`, `lockFileNames` not stored |
| `lib/modules/manager/bazelisk/artifacts.ts` | N/A | not-started | Artifact execution not implemented |

### bicep

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/bicep/extract.ts` | `extractors/bicep.rs` | full | `extract()` handles Bicep resource references |
| `lib/modules/manager/bicep/index.ts` | `managers.rs` | partial | fileMatch pattern present |

### bitbucket-pipelines

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/bitbucket-pipelines/extract.ts` | `extractors/bitbucket_pipelines.rs` | full | `extract()` handles Docker images from Bitbucket pipelines |
| `lib/modules/manager/bitbucket-pipelines/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/bitbucket-pipelines/dep-types.ts` | `extractors/bitbucket_pipelines.rs` | full | Dep types embedded |
| `lib/modules/manager/bitbucket-pipelines/util.ts` | `extractors/bitbucket_pipelines.rs` | full | Utility functions inline |

### bitrise

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/bitrise/extract.ts` | `extractors/bitrise.rs` | full | `extract()` handles Bitrise step references |
| `lib/modules/manager/bitrise/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/bitrise/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/bitrise/utils.ts` | `extractors/bitrise.rs` | partial | Utility functions inline |

### buildkite

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/buildkite/extract.ts` | `extractors/buildkite.rs` | full | `extract()` handles plugin and Docker image refs |
| `lib/modules/manager/buildkite/index.ts` | `managers.rs` | partial | fileMatch pattern present |

### buildpacks

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/buildpacks/extract.ts` | `extractors/buildpacks.rs` | full | `extract()` handles buildpack refs from project.toml |
| `lib/modules/manager/buildpacks/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/buildpacks/schema.ts` | N/A | out-of-scope | Zod schema |

### bun

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/bun/extract.ts` | `extractors/npm.rs` (partial) | partial | Bun reuses npm extract for package.json; `extractAllPackageFiles` + workspace matching present in npm.rs |
| `lib/modules/manager/bun/index.ts` | `managers.rs` | partial | fileMatch pattern present; `supersedesManagers`, `supportsLockFileMaintenance`, `lockFileNames` partially handled |
| `lib/modules/manager/bun/artifacts.ts` | N/A | not-started | Bun lockfile artifact execution not implemented |
| `lib/modules/manager/bun/utils.ts` | `extractors/npm.rs` | partial | `fileMatchesWorkspaces` and `filesMatchingWorkspaces` present in npm.rs |

### bun-version

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/bun-version/index.ts` | `extractors/version_file.rs` + `extractors/bun_version.rs` | partial | TS uses NpmDatasource; Rust `bun_version.rs` uses GithubReleases. Different datasource mapping. |

### bundler

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/bundler/extract.ts` | `extractors/bundler.rs` | full | `extract()` + `extract_with_lock()` handles Gemfile parsing with lock file support |
| `lib/modules/manager/bundler/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/bundler/common.ts` | `extractors/bundler.rs` | full | Common utilities inline |
| `lib/modules/manager/bundler/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/bundler/host-rules.ts` | `extractors/bundler.rs` | full | `find_all_authenticatable()` and `get_authentication_header_value()` present |
| `lib/modules/manager/bundler/locked-version.ts` | `extractors/bundler.rs` | full | `extract_lock_file_entries()` present |
| `lib/modules/manager/bundler/update-locked.ts` | `extractors/bundler.rs` | full | `update_locked_bundler_dependency()` present |

### cake

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/cake/index.ts` | `extractors/cake.rs` | partial | TS uses `moo` lexer for parsing; Rust uses regex-based parsing. Both extract NuGet deps from Cake files. `getConfiguredRegistries` not in Rust. |

### cargo

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/cargo/extract.ts` | `extractors/cargo.rs` | full | `extract()` + `extract_with_context()` handle workspace, features, registries |
| `lib/modules/manager/cargo/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/cargo/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/cargo/locked-version.ts` | `extractors/cargo.rs` | full | `build_lock_map()` + `find_locked_version()` present |
| `lib/modules/manager/cargo/range.ts` | `extractors/cargo.rs` | full | `get_range_strategy()` present |
| `lib/modules/manager/cargo/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/cargo/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/cargo/update-locked.ts` | `extractors/cargo.rs` | full | `update_locked_dependency()` present |
| `lib/modules/manager/cargo/update.ts` | `extractors/cargo.rs` | full | `bump_package_version()` present |
| `lib/modules/manager/cargo/utils.ts` | `extractors/cargo.rs` | partial | Some utility functions inline |

### cdnurl

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/cdnurl/extract.ts` | `extractors/cdnurl.rs` | full | `extract_package_file()` present with cloudflare URL regex |
| `lib/modules/manager/cdnurl/index.ts` | `managers.rs` | partial | Empty fileMatch patterns present |

### circleci

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/circleci/extract.ts` | `extractors/circleci.rs` | full | `extract()` handles Docker images + orbs |
| `lib/modules/manager/circleci/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/circleci/dep-types.ts` | `extractors/circleci.rs` | full | Dep types embedded |
| `lib/modules/manager/circleci/range.ts` | `extractors/circleci.rs` | full | `get_range_strategy()` present |
| `lib/modules/manager/circleci/schema.ts` | N/A | out-of-scope | Zod schema |

### cloudbuild

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/cloudbuild/extract.ts` | `extractors/cloudbuild.rs` | full | `extract()` handles Docker images from Cloud Build YAML |
| `lib/modules/manager/cloudbuild/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/cloudbuild/schema.ts` | N/A | out-of-scope | Zod schema |

### cocoapods

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/cocoapods/extract.ts` | `extractors/cocoapods.rs` | full | `extract()` handles Podfile deps |
| `lib/modules/manager/cocoapods/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/cocoapods/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/cocoapods/types.ts` | N/A | out-of-scope | Type-only file |

### composer

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/composer/extract.ts` | `extractors/composer.rs` | full | `extract()` + `extract_resolved()` handle packages, repos, path repos |
| `lib/modules/manager/composer/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/composer/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/composer/dep-types.ts` | `extractors/composer.rs` | full | Dep types embedded |
| `lib/modules/manager/composer/range.ts` | `extractors/composer.rs` | full | `get_composer_range_strategy()` present |
| `lib/modules/manager/composer/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/composer/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/composer/update-locked.ts` | `extractors/composer.rs` | full | `update_locked_composer_dependency()` present |
| `lib/modules/manager/composer/utils.ts` | `extractors/composer.rs` | partial | Some utilities inline (normalize_composer_repo_url, parse_repos) |

### conan

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/conan/extract.ts` | `extractors/conan.rs` | full | `extract_txt()` + `extract_py()` handle both conanfile formats |
| `lib/modules/manager/conan/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/conan/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/conan/common.ts` | `extractors/conan.rs` | partial | Common utilities inline |
| `lib/modules/manager/conan/dep-types.ts` | `extractors/conan.rs` | full | Dep types embedded |
| `lib/modules/manager/conan/range.ts` | `extractors/conan.rs` | full | `get_range_strategy()` present |

### copier

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/copier/extract.ts` | `extractors/copier.rs` | full | `extract()` handles copier answer files |
| `lib/modules/manager/copier/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/copier/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/copier/dep-types.ts` | `extractors/copier.rs` | full | Dep types embedded |
| `lib/modules/manager/copier/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/copier/update.ts` | `extractors/copier.rs` | full | `update_copier_dependency()` present with `#copier updated` marker |
| `lib/modules/manager/copier/utils.ts` | `extractors/copier.rs` | partial | Utility functions inline |

### cpanfile

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/cpanfile/extract.ts` | `extractors/cpanfile.rs` | full | `extract()` handles cpanfile deps with phase handling |
| `lib/modules/manager/cpanfile/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/cpanfile/dep-types.ts` | `extractors/cpanfile.rs` | full | Dep types embedded (CpanDepPhase) |
| `lib/modules/manager/cpanfile/language.ts` | `extractors/cpanfile.rs` | partial | Language detection inline |
| `lib/modules/manager/cpanfile/parser.ts` | `extractors/cpanfile.rs` | partial | Parsing inline |

### crossplane

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/crossplane/extract.ts` | `extractors/crossplane.rs` | full | `extract()` handles Crossplane package refs |
| `lib/modules/manager/crossplane/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/crossplane/dep-types.ts` | `extractors/crossplane.rs` | full | Dep types embedded |
| `lib/modules/manager/crossplane/schema.ts` | N/A | out-of-scope | Zod schema |

### crow

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/crow/extract.ts` | `extractors/crow.rs` | full | `extract()` handles Crow Docker image refs |
| `lib/modules/manager/crow/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/crow/schema.ts` | N/A | out-of-scope | Zod schema |

### custom

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/custom/index.ts` | `managers.rs` | partial | `is_custom_manager()`, `CUSTOM_MANAGER_LIST` present; full custom manager dispatch not implemented |
| `lib/modules/manager/custom/api.ts` | N/A | not-started | Custom manager API (regex/jsonata registration) not implemented |
| `lib/modules/manager/custom/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/custom/utils.ts` | `managers.rs` | partial | `regex_match_all()` present |
| `lib/modules/manager/custom/regex/index.ts` | N/A | not-started | Regex custom manager extraction not implemented |
| `lib/modules/manager/custom/regex/strategies.ts` | N/A | not-started | Regex strategies not implemented |
| `lib/modules/manager/custom/regex/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/custom/regex/utils.ts` | `managers.rs` | partial | `regex_match_all()` present in managers.rs |
| `lib/modules/manager/custom/jsonata/index.ts` | N/A | not-started | JSONata custom manager not implemented |
| `lib/modules/manager/custom/jsonata/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/custom/jsonata/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/custom/jsonata/utils.ts` | N/A | not-started | JSONata utils not implemented |

### deno

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/deno/extract.ts` | `extractors/npm.rs` (partial) | partial | Deno reuses npm extract for package.json; `extractAllPackageFiles` pattern not fully separate |
| `lib/modules/manager/deno/index.ts` | `managers.rs` | partial | fileMatch pattern present; `supersedesManagers`, `supportsLockFileMaintenance` partially handled |
| `lib/modules/manager/deno/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/deno/compat.ts` | `extractors/deno.rs` | partial | Compat layer for npm re-export |
| `lib/modules/manager/deno/post.ts` | `extractors/deno.rs` | partial | Post-processing: parse_deno_lock, get_locked_version, normalize_workspace, apply_locked_versions present. Missing: collectPackageJsonAsWorkspaceMember, postExtract error handling, workspace member pattern matching |
| `lib/modules/manager/deno/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/deno/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/deno/update.ts` | `extractors/deno.rs` | full | `deno_update_dependency()` present |
| `lib/modules/manager/deno/utils.ts` | `extractors/npm.rs` | partial | Some utility functions reused |

### deps-edn

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/deps-edn/extract.ts` | `extractors/deps_edn.rs` | full | `extract()` + `parse_deps_edn_file()` handle EDN parsing |
| `lib/modules/manager/deps-edn/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/deps-edn/parser.ts` | `extractors/deps_edn.rs` | partial | EDN tokenizer inline |
| `lib/modules/manager/deps-edn/types.ts` | N/A | out-of-scope | Type-only file |

### devbox

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/devbox/extract.ts` | `extractors/devbox.rs` | full | `extract()` handles devbox.json packages |
| `lib/modules/manager/devbox/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/devbox/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/devbox/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/devbox/tool-versioning.ts` | `extractors/devbox.rs` | partial | Tool versioning logic inline |

### devcontainer

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/devcontainer/extract.ts` | `extractors/devcontainer.rs` | full | `extract()` handles devcontainer.json features and image refs |
| `lib/modules/manager/devcontainer/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/devcontainer/dep-types.ts` | `extractors/devcontainer.rs` | full | Dep types embedded |
| `lib/modules/manager/devcontainer/schema.ts` | N/A | out-of-scope | Zod schema |

### docker-compose

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/docker-compose/extract.ts` | `extractors/docker_compose.rs` | full | `extract()` handles Docker image refs from compose files |
| `lib/modules/manager/docker-compose/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/docker-compose/schema.ts` | N/A | out-of-scope | Zod schema |

### dockerfile

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/dockerfile/extract.ts` | `extractors/dockerfile.rs` | full | `extract()` handles FROM, COPY --from, RUN --mount with variables, escape char, parser directives |
| `lib/modules/manager/dockerfile/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/dockerfile/dep-types.ts` | `extractors/dockerfile.rs` | full | Dep types embedded |

### droneci

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/droneci/extract.ts` | `extractors/droneci.rs` | full | `extract()` handles Drone CI Docker image refs |
| `lib/modules/manager/droneci/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/droneci/dep-types.ts` | `extractors/droneci.rs` | full | Dep types embedded |

### fleet

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/fleet/extract.ts` | `extractors/fleet.rs` | full | `extract()` handles Fleet YAML helm and git deps |
| `lib/modules/manager/fleet/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/fleet/dep-types.ts` | `extractors/fleet.rs` | full | Dep types embedded |
| `lib/modules/manager/fleet/schema.ts` | N/A | out-of-scope | Zod schema |

### flux

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/flux/extract.ts` | `extractors/flux.rs` | full | `extract()` + `extract_all_package_files()` handle HelmRelease, GitRepository, OCI, Kustomization |
| `lib/modules/manager/flux/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/flux/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/flux/common.ts` | `extractors/flux.rs` | full | Constants and shared utilities inline |
| `lib/modules/manager/flux/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/flux/types.ts` | N/A | out-of-scope | Type-only file |

### fvm

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/fvm/extract.ts` | `extractors/fvm.rs` | full | `extract()` handles FVM config |
| `lib/modules/manager/fvm/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/fvm/schema.ts` | N/A | out-of-scope | Zod schema |

### git-submodules

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/git-submodules/extract.ts` | `extractors/git_submodules.rs` | full | `extract()` + `extract_with_remote()` handle .gitmodules parsing |
| `lib/modules/manager/git-submodules/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/git-submodules/artifacts.ts` | `extractors/git_submodules.rs` | partial | `update_artifacts()` stub present (returns empty) |
| `lib/modules/manager/git-submodules/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/git-submodules/update.ts` | `extractors/git_submodules.rs` | full | update_dependency with branch field update/add |

### github-actions

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/github-actions/extract.ts` | `extractors/github_actions.rs` | full | `extract()` + `extract_with_context()` handle `uses:`, `docker://`, `runs-on:` |
| `lib/modules/manager/github-actions/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/github-actions/community.ts` | `extractors/github_actions.rs` | partial | Community action index not implemented |
| `lib/modules/manager/github-actions/dep-types.ts` | `extractors/github_actions.rs` | full | Dep types embedded |
| `lib/modules/manager/github-actions/parse.ts` | `extractors/github_actions.rs` | full | `parse_action_reference()`, `parse_uses_line()`, `parse_comment()` present |
| `lib/modules/manager/github-actions/schema.ts` | N/A | out-of-scope | Zod schema |

### gitlabci

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/gitlabci/extract.ts` | `extractors/gitlabci.rs` | full | `extract()` + `extract_docker_with_registry_aliases()` + `extract_components()` present |
| `lib/modules/manager/gitlabci/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/gitlabci/dep-types.ts` | `extractors/gitlabci.rs` | full | Dep types embedded |
| `lib/modules/manager/gitlabci/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/gitlabci/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/gitlabci/utils.ts` | `extractors/gitlabci.rs` | partial | `get_gitlab_dep()`, `strip_dependency_proxy_prefix()` present |

### gitlabci-include

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/gitlabci-include/extract.ts` | `extractors/gitlabci_include.rs` | full | `extract()` + `extract_with_endpoint()` handle include directives |
| `lib/modules/manager/gitlabci-include/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/gitlabci-include/dep-types.ts` | `extractors/gitlabci_include.rs` | full | Dep types embedded |
| `lib/modules/manager/gitlabci-include/schema.ts` | N/A | out-of-scope | Zod schema |

### glasskube

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/glasskube/extract.ts` | `extractors/glasskube.rs` | full | `extract()` handles Glasskube package refs |
| `lib/modules/manager/glasskube/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/glasskube/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/glasskube/types.ts` | N/A | out-of-scope | Type-only file |

### gleam

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/gleam/extract.ts` | `extractors/gleam.rs` | full | `extract()` + `extract_with_lock()` handle gleam.toml and lock file |
| `lib/modules/manager/gleam/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/gleam/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/gleam/dep-types.ts` | `extractors/gleam.rs` | full | Dep types embedded |
| `lib/modules/manager/gleam/locked-version.ts` | `extractors/gleam.rs` | full | `parse_gleam_lock_file()` + `hex_version_satisfies()` present |
| `lib/modules/manager/gleam/range.ts` | `extractors/gleam.rs` | full | `get_range_strategy()` present |
| `lib/modules/manager/gleam/schema.ts` | N/A | out-of-scope | Zod schema |

### gomod

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/gomod/extract.ts` | `extractors/gomod.rs` | full | `extract()` handles require, replace, tool directives |
| `lib/modules/manager/gomod/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/gomod/artifacts.ts` | `extractors/gomod_artifact_runner.rs` | partial | `GomodArtifactRunner` implements lockfile regeneration via `go mod tidy`; wired into CLI pipeline; missing: `go mod vendor`, import path upgrades, marwan-at-work/mod |
| `lib/modules/manager/gomod/artifacts-extra.ts` | `extractors/gomod.rs` | full | `get_extra_deps()` + `extra_deps_table()` + `get_extra_deps_notice()` present |
| `lib/modules/manager/gomod/dep-types.ts` | `extractors/gomod.rs` | full | Dep types embedded |
| `lib/modules/manager/gomod/line-parser.ts` | `extractors/gomod.rs` | full | `parse_line()` present |
| `lib/modules/manager/gomod/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/gomod/update.ts` | `extractors/gomod.rs` | full | `gomod_update_dependency()` present |

### gradle

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/gradle/extract.ts` | `extractors/gradle.rs` | full | `extract_build_file()` handles Gradle dependency declarations |
| `lib/modules/manager/gradle/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/gradle/extract/catalog.ts` | `extractors/gradle.rs` | full | `extract_version_catalog()` handles TOML catalogs |
| `lib/modules/manager/gradle/extract/consistent-versions-plugin.ts` | `extractors/gradle.rs` | full | `uses_gcv()` + `parse_gcv()` handle Palantir GCV |
| `lib/modules/manager/gradle/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/gradle/dep-types.ts` | `extractors/gradle.rs` | full | Dep types embedded |
| `lib/modules/manager/gradle/parser.ts` | `extractors/gradle.rs` | partial | Gradle Groovy parser partially implemented |
| `lib/modules/manager/gradle/parser/apply-from.ts` | `extractors/gradle.rs` | partial | Apply-from handling inline |
| `lib/modules/manager/gradle/parser/assignments.ts` | `extractors/gradle.rs` | partial | Assignment handling inline |
| `lib/modules/manager/gradle/parser/common.ts` | `extractors/gradle.rs` | partial | Common parser utilities inline |
| `lib/modules/manager/gradle/parser/dependencies.ts` | `extractors/gradle.rs` | partial | Dependency parsing inline |
| `lib/modules/manager/gradle/parser/handlers.ts` | `extractors/gradle.rs` | partial | Handler functions inline |
| `lib/modules/manager/gradle/parser/language-version.ts` | `extractors/gradle.rs` | partial | Language version handling inline |
| `lib/modules/manager/gradle/parser/objects.ts` | `extractors/gradle.rs` | partial | Object parsing inline |
| `lib/modules/manager/gradle/parser/plugins.ts` | `extractors/gradle.rs` | partial | Plugin parsing inline |
| `lib/modules/manager/gradle/parser/registry-urls.ts` | `extractors/gradle.rs` | partial | Registry URL handling inline |
| `lib/modules/manager/gradle/parser/version-catalogs.ts` | `extractors/gradle.rs` | full | Version catalog parsing inline |
| `lib/modules/manager/gradle/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/gradle/update.ts` | `extractors/gradle.rs` | full | `update_dependency()` present |
| `lib/modules/manager/gradle/utils.ts` | `extractors/gradle.rs` | partial | Some utility functions inline |

### gradle-wrapper

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/gradle-wrapper/extract.ts` | `extractors/gradle_wrapper.rs` | full | `extract()` + `extract_gradle_version()` present |
| `lib/modules/manager/gradle-wrapper/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/gradle-wrapper/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/gradle-wrapper/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/gradle-wrapper/utils.ts` | `extractors/gradle_wrapper.rs` | full | `java_constraint_from_gradle_version()`, `parse_jvm_toolchain_version()`, etc. present |

### haskell-cabal

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/haskell-cabal/extract.ts` | `extractors/cabal.rs` | full | `extract()` + `extract_names_and_ranges()` + `find_depends()` present |
| `lib/modules/manager/haskell-cabal/index.ts` | `managers.rs` + `extractors/cabal.rs` | partial | `getRangeStrategy()` maps to `get_range_strategy()` in cabal.rs; `extractPackageFile` inline in TS index.ts maps to cabal.rs |

### helm-requirements

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/helm-requirements/extract.ts` | `extractors/helm.rs` | full | Handled by `extract()` in helm.rs (requirements.yaml is Chart.yaml v2) |
| `lib/modules/manager/helm-requirements/index.ts` | `managers.rs` | partial | fileMatch pattern present as `helm-requirements` |

### helm-values

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/helm-values/extract.ts` | `extractors/helm_values.rs` | full | `extract()` handles Docker image refs from Helm values YAML |
| `lib/modules/manager/helm-values/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/helm-values/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/helm-values/util.ts` | `extractors/helm_values.rs` | partial | Utility functions inline |

### helmfile

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/helmfile/extract.ts` | `extractors/helmfile.rs` | full | `extract()` + `extract_package_file()` handle releases, repos, templates |
| `lib/modules/manager/helmfile/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/helmfile/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/helmfile/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/helmfile/utils.ts` | `extractors/helmfile.rs` | partial | Template handling and utility functions inline |

### helmsman

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/helmsman/extract.ts` | `extractors/helmsman.rs` | full | `extract()` handles Helmsman YAML apps |
| `lib/modules/manager/helmsman/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/helmsman/types.ts` | N/A | out-of-scope | Type-only file |

### helmv3

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/helmv3/extract.ts` | `extractors/helm.rs` | full | `extract()` handles Chart.yaml dependencies |
| `lib/modules/manager/helmv3/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/helmv3/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/helmv3/common.ts` | `extractors/helm.rs` | full | `is_alias()`, `resolve_alias()`, `is_oci_registry()` present |
| `lib/modules/manager/helmv3/oci.ts` | `extractors/helm.rs` | partial | OCI login cmd generation present (`generate_login_cmd`) |
| `lib/modules/manager/helmv3/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/helmv3/update.ts` | `extractors/helm.rs` | full | `bump_package_version()` present |
| `lib/modules/manager/helmv3/utils.ts` | `extractors/helm.rs` | partial | Utility functions inline |

### hermit

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/hermit/extract.ts` | `extractors/hermit.rs` | full | `extract_from_file_list()` handles hermit package files |
| `lib/modules/manager/hermit/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/hermit/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/hermit/default-config.ts` | `extractors/hermit.rs` | partial | File patterns and exclude paths inline |
| `lib/modules/manager/hermit/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/hermit/update.ts` | `extractors/hermit.rs` | full | `update_hermit_dependency()` present |

### homeassistant-manifest

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/homeassistant-manifest/extract.ts` | `extractors/homeassistant.rs` | full | `extract()` handles Python requirement deps |
| `lib/modules/manager/homeassistant-manifest/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/homeassistant-manifest/schema.ts` | N/A | out-of-scope | Zod schema |

### homebrew

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/homebrew/extract.ts` | `extractors/homebrew.rs` | full | `extract()` handles Homebrew formula URLs |
| `lib/modules/manager/homebrew/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/homebrew/handlers/base.ts` | `extractors/homebrew.rs` | full | `find_handler()` + `find_handler_by_type()` present |
| `lib/modules/manager/homebrew/handlers/github.ts` | `extractors/homebrew.rs` | full | `github_parse_url()` + `github_create_dependency()` + `github_build_archive_urls()` present |
| `lib/modules/manager/homebrew/handlers/npm.ts` | `extractors/homebrew.rs` | full | `npm_parse_url()` + `npm_create_dependency()` + `npm_build_archive_urls()` present |
| `lib/modules/manager/homebrew/handlers/index.ts` | `extractors/homebrew.rs` | full | Handler dispatch inline |
| `lib/modules/manager/homebrew/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/homebrew/update.ts` | N/A | not-started | `updateDependency` not implemented in Rust |
| `lib/modules/manager/homebrew/utils.ts` | `extractors/homebrew.rs` | partial | URL parsing utilities inline |

### html

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/html/extract.ts` | `extractors/html.rs` | full | `extract()` handles CDN script/link tags |
| `lib/modules/manager/html/index.ts` | `managers.rs` | partial | fileMatch pattern present |

### jenkins

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/jenkins/extract.ts` | `extractors/jenkins.rs` | full | `extract_txt()` + `extract_yml()` handle both plugin file formats |
| `lib/modules/manager/jenkins/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/jenkins/types.ts` | N/A | out-of-scope | Type-only file |

### jsonnet-bundler

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/jsonnet-bundler/extract.ts` | `extractors/jsonnet_bundler.rs` | full | `extract()` + `extract_with_path()` handle jsonnetfile.json |
| `lib/modules/manager/jsonnet-bundler/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/jsonnet-bundler/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/jsonnet-bundler/types.ts` | N/A | out-of-scope | Type-only file |

### kotlin-script

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/kotlin-script/extract.ts` | `extractors/kotlin_script.rs` | full | `extract()` handles Maven deps from .main.kts files |
| `lib/modules/manager/kotlin-script/index.ts` | `managers.rs` | partial | fileMatch pattern present |

### kubernetes

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/kubernetes/extract.ts` | `extractors/kubernetes.rs` | full | `extract()` handles K8s manifest image refs |
| `lib/modules/manager/kubernetes/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/kubernetes/schema.ts` | N/A | out-of-scope | Zod schema |

### kustomize

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/kustomize/extract.ts` | `extractors/kustomize.rs` | full | `extract()` handles images, helmChart, resources |
| `lib/modules/manager/kustomize/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/kustomize/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/kustomize/common.ts` | `extractors/kustomize.rs` | partial | Common utilities inline |
| `lib/modules/manager/kustomize/dep-types.ts` | `extractors/kustomize.rs` | full | Dep types embedded |
| `lib/modules/manager/kustomize/types.ts` | N/A | out-of-scope | Type-only file |

### leiningen

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/leiningen/extract.ts` | `extractors/leiningen.rs` | full | `extract()` + `extract_from_vectors()` handle project.clj parsing |
| `lib/modules/manager/leiningen/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/leiningen/dep-types.ts` | `extractors/leiningen.rs` | full | Dep types embedded |
| `lib/modules/manager/leiningen/types.ts` | N/A | out-of-scope | Type-only file |

### maven

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/maven/extract.ts` | `extractors/maven.rs` | full | `extract()` + `extract_all_package_files()` handle POM parsing with properties |
| `lib/modules/manager/maven/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/maven/dep-types.ts` | `extractors/maven.rs` | full | Dep types embedded |
| `lib/modules/manager/maven/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/maven/update.ts` | `extractors/maven.rs` | full | `maven_update_dependency()` + `maven_update_at_position()` + `maven_bump_package_version()` present |

### maven-wrapper

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/maven-wrapper/extract.ts` | `extractors/maven_wrapper.rs` | full | `extract()` handles maven-wrapper.properties |
| `lib/modules/manager/maven-wrapper/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/maven-wrapper/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/maven-wrapper/types.ts` | N/A | out-of-scope | Type-only file |

### meteor

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/meteor/extract.ts` | `extractors/meteor.rs` | full | `extract()` handles Meteor package.js deps |
| `lib/modules/manager/meteor/index.ts` | `managers.rs` | partial | fileMatch pattern present |

### mint

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/mint/extract.ts` | `extractors/mint.rs` | full | `extract()` handles Mintfile deps |
| `lib/modules/manager/mint/index.ts` | `managers.rs` | partial | fileMatch pattern present |

### mise

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/mise/extract.ts` | `extractors/mise.rs` | full | `extract()` handles mise.toml tool definitions |
| `lib/modules/manager/mise/index.ts` | `managers.rs` | partial | fileMatch pattern present (all 7 patterns) |
| `lib/modules/manager/mise/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/mise/backends.ts` | `extractors/mise.rs` | full | All backend configs (aqua, cargo, dotnet, gem, github, go, npm, pipx, spm, ubi) present |
| `lib/modules/manager/mise/lockfile.ts` | `extractors/mise.rs` | partial | `MiseConfigType` + `get_config_type()` present |
| `lib/modules/manager/mise/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/mise/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/mise/update-locked.ts` | `extractors/mise.rs` | full | `update_locked_dependency()` present |
| `lib/modules/manager/mise/upgradeable-tooling.ts` | `extractors/mise.rs` | full | Full mise tooling table with all backends |
| `lib/modules/manager/mise/utils.ts` | `extractors/mise.rs` | partial | Utility functions inline |

### mix

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/mix/extract.ts` | `extractors/mix.rs` | full | `extract()` + `extract_with_lock()` handle mix.exs and lock file |
| `lib/modules/manager/mix/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/mix/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/mix/dep-types.ts` | `extractors/mix.rs` | full | Dep types embedded |
| `lib/modules/manager/mix/range.ts` | `extractors/mix.rs` | full | `get_range_strategy()` present |

### nix

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/nix/extract.ts` | `extractors/nix.rs` | full | `extract()` + `extract_package_file()` + `extract_with_config()` handle flake.nix and flake.lock |
| `lib/modules/manager/nix/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/nix/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/nix/range.ts` | `extractors/nix.rs` | full | `get_range_strategy()` present |
| `lib/modules/manager/nix/schema.ts` | N/A | out-of-scope | Zod schema |

### nodenv

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/nodenv/extract.ts` | `extractors/version_file.rs` | full | `extract_nodenv()` present |
| `lib/modules/manager/nodenv/index.ts` | `managers.rs` | partial | fileMatch pattern present |

### npm

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/npm/extract/index.ts` | `extractors/npm.rs` | partial | `extract()` handles core package.json parsing |
| `lib/modules/manager/npm/extract/common/catalogs.ts` | `extractors/npm.rs` | full | `extract_catalog_deps()` + `extract_yarn_catalogs()` present |
| `lib/modules/manager/npm/extract/common/dependency.ts` | `extractors/npm.rs` | partial | Dependency extraction inline |
| `lib/modules/manager/npm/extract/common/node.ts` | `extractors/npm.rs` | partial | Node engine/version extraction inline |
| `lib/modules/manager/npm/extract/common/overrides.ts` | `extractors/npm.rs` | full | `collect_overrides()` present |
| `lib/modules/manager/npm/extract/common/package-file.ts` | `extractors/npm.rs` | partial | Package file extraction inline |
| `lib/modules/manager/npm/extract/npm.ts` | `extractors/npm.rs` | partial | npm-specific extraction inline |
| `lib/modules/manager/npm/extract/pnpm.ts` | `extractors/npm.rs` | full | `extract_pnpm_workspace_file()` present |
| `lib/modules/manager/npm/extract/post/index.ts` | `extractors/npm.rs` | partial | Post-processing partially inline |
| `lib/modules/manager/npm/extract/post/locked-versions.ts` | `extractors/npm.rs` | partial | Lock file parsing (`parse_npm_lock()`, `parse_yarn_lock()`) present. Missing: pnpm-lock locked version support, workspace lockfile path resolution, lockfile v2 constraint augmentation, engines/packageManager/volta exclusion |
| `lib/modules/manager/npm/extract/post/monorepo.ts` | `extractors/npm.rs` | partial | Monorepo detection partially handled |
| `lib/modules/manager/npm/extract/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/npm/extract/utils.ts` | `extractors/npm.rs` | partial | Utility functions inline |
| `lib/modules/manager/npm/extract/yarn.ts` | `extractors/npm.rs` | partial | Yarn-specific extraction inline |
| `lib/modules/manager/npm/extract/yarnrc.ts` | `extractors/npm.rs` | full | `load_config_from_yarnrc_yml()` + `load_config_from_legacy_yarnrc()` present |
| `lib/modules/manager/npm/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/npm/artifacts.ts` | `extractors/npm_post_update/artifact_runner.rs` | partial | `NpmArtifactRunner` implements lockfile regeneration for npm/yarn/pnpm; wired into CLI pipeline; missing: full constraint env, global npmrc, yarnrc, pnpmfile hooks |
| `lib/modules/manager/npm/constants.ts` | `extractors/npm.rs` | partial | Some constants inline |
| `lib/modules/manager/npm/dep-types.ts` | `extractors/npm.rs` | full | All dep types (NpmDepType) embedded |
| `lib/modules/manager/npm/detect.ts` | `extractors/npm.rs` | full | `detect_global_config()` + `detect_global_config_from()` present |
| `lib/modules/manager/npm/npmrc.ts` | `extractors/npm.rs` | partial | npmrc processing inline |
| `lib/modules/manager/npm/post-update/index.ts` | N/A | not-started | Post-update artifact execution not implemented |
| `lib/modules/manager/npm/post-update/node-version.ts` | N/A | not-started | Node version post-update not implemented |
| `lib/modules/manager/npm/post-update/npm.ts` | N/A | not-started | npm post-update not implemented |
| `lib/modules/manager/npm/post-update/pnpm.ts` | N/A | not-started | pnpm post-update not implemented |
| `lib/modules/manager/npm/post-update/rules.ts` | `extractors/npm.rs` | full | `process_host_rules()` present |
| `lib/modules/manager/npm/post-update/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/npm/post-update/utils.ts` | N/A | not-started | Post-update utils not implemented |
| `lib/modules/manager/npm/post-update/yarn.ts` | N/A | not-started | Yarn post-update not implemented |
| `lib/modules/manager/npm/range.ts` | `extractors/npm.rs` | partial | Range strategy handling partially present |
| `lib/modules/manager/npm/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/npm/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/npm/update/dependency/common.ts` | `extractors/npm.rs` | partial | `npm_update_dependency()` present |
| `lib/modules/manager/npm/update/dependency/index.ts` | `extractors/npm.rs` | partial | Update dispatch partially handled |
| `lib/modules/manager/npm/update/dependency/pnpm.ts` | `extractors/npm.rs` | partial | pnpm update partially handled |
| `lib/modules/manager/npm/update/dependency/yarn.ts` | `extractors/npm.rs` | partial | Yarn update partially handled |
| `lib/modules/manager/npm/update/index.ts` | `extractors/npm.rs` | partial | Update index partially handled |
| `lib/modules/manager/npm/update/locked-dependency/index.ts` | `extractors/npm.rs` | partial | `yarn_update_locked_dependency()` + `package_lock_get_locked_dependencies()` present |
| `lib/modules/manager/npm/update/locked-dependency/common/parent-version.ts` | `extractors/npm.rs` | partial | Parent version resolution inline |
| `lib/modules/manager/npm/update/locked-dependency/package-lock/dep-constraints.ts` | `extractors/npm.rs` | partial | Dep constraints inline |
| `lib/modules/manager/npm/update/locked-dependency/package-lock/get-locked.ts` | `extractors/npm.rs` | partial | Get locked inline |
| `lib/modules/manager/npm/update/locked-dependency/package-lock/index.ts` | `extractors/npm.rs` | partial | Package lock update partially handled |
| `lib/modules/manager/npm/update/locked-dependency/package-lock/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/npm/update/locked-dependency/yarn-lock/get-locked.ts` | `extractors/npm.rs` | partial | `get_yarn_locked_dependencies()` present |
| `lib/modules/manager/npm/update/locked-dependency/yarn-lock/index.ts` | `extractors/npm.rs` | partial | Yarn lock update partially handled |
| `lib/modules/manager/npm/update/locked-dependency/yarn-lock/replace.ts` | `extractors/npm.rs` | partial | Yarn lock replace inline |
| `lib/modules/manager/npm/update/locked-dependency/yarn-lock/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/npm/update/package-version/index.ts` | N/A | not-started | Package version bump not implemented |
| `lib/modules/manager/npm/utils.ts` | `extractors/npm.rs` | partial | Some utility functions inline |

### nuget

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/nuget/extract.ts` | `extractors/nuget.rs` | full | `extract()` + `extract_project_file()` handle csproj/fsproj/vbproj/props/targets |
| `lib/modules/manager/nuget/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/nuget/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/nuget/config-formatter.ts` | `extractors/nuget.rs` | partial | NuGet config handling inline |
| `lib/modules/manager/nuget/dep-types.ts` | `extractors/nuget.rs` | full | Dep types embedded |
| `lib/modules/manager/nuget/extract/global-manifest.ts` | `extractors/nuget.rs` | full | `extract_global_json()` present |
| `lib/modules/manager/nuget/extract/single-csharp-file.ts` | `extractors/nuget.rs` | full | `extract_single_csharp_file()` present |
| `lib/modules/manager/nuget/package-tree.ts` | `extractors/nuget.rs` | partial | Package tree logic partially inline |
| `lib/modules/manager/nuget/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/nuget/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/nuget/update.ts` | `extractors/nuget.rs` | full | `bump_package_version()` present |
| `lib/modules/manager/nuget/util.ts` | `extractors/nuget.rs` | partial | NuGet config registry URL parsing inline |

### nvm

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/nvm/extract.ts` | `extractors/version_file.rs` | full | `extract()` with `"nvmrc"` manager handles nvm aliases, comments, v-prefix |
| `lib/modules/manager/nvm/index.ts` | `managers.rs` | partial | fileMatch pattern present |

### ocb

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/ocb/extract.ts` | `extractors/ocb.rs` | full | `extract()` handles OCB builder YAML |
| `lib/modules/manager/ocb/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/ocb/dep-types.ts` | `extractors/ocb.rs` | full | Dep types embedded |
| `lib/modules/manager/ocb/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/ocb/update.ts` | `extractors/ocb.rs` | full | `bump_package_version()` present |

### osgi

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/osgi/extract.ts` | `extractors/osgi.rs` | full | `extract()` handles OSGi feature JSON |
| `lib/modules/manager/osgi/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/osgi/types.ts` | N/A | out-of-scope | Type-only file |

### pep621

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/pep621/extract.ts` | `extractors/pep621.rs` | full | `extract()` + `extract_package_file()` handle all dep types including PDM, UV, Hatch |
| `lib/modules/manager/pep621/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/pep621/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/pep621/dep-types.ts` | `extractors/pep621.rs` | full | All dep types (Pep621DepType) embedded including UvSources, PdmDev, HatchEnv |
| `lib/modules/manager/pep621/processors/abstract.ts` | `extractors/pep621.rs` | partial | Abstract processor logic inline |
| `lib/modules/manager/pep621/processors/hatch.ts` | `extractors/pep621.rs` | partial | Hatch env handling inline |
| `lib/modules/manager/pep621/processors/index.ts` | `extractors/pep621.rs` | partial | Processor dispatch inline |
| `lib/modules/manager/pep621/processors/pdm.ts` | `extractors/pep621.rs` | partial | PDM source handling inline |
| `lib/modules/manager/pep621/processors/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/pep621/processors/uv.ts` | `extractors/pep621.rs` | partial | UV source handling inline |
| `lib/modules/manager/pep621/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/pep621/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/pep621/update.ts` | `extractors/pep621.rs` | full | `bump_package_version()` present |
| `lib/modules/manager/pep621/utils.ts` | `extractors/pep621.rs` | partial | PEP508 parsing utility inline |

### pep723

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/pep723/extract.ts` | `extractors/pep723.rs` | full | `extract()` + `extract_pep723()` handle inline script metadata |
| `lib/modules/manager/pep723/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/pep723/dep-types.ts` | `extractors/pep723.rs` | full | Dep types embedded |
| `lib/modules/manager/pep723/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/pep723/utils.ts` | `extractors/pep723.rs` | partial | Utility functions inline |

### pip-compile

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/pip-compile/extract.ts` | `extractors/pip_compile.rs` | partial | `extract_package_file()` handles `.in` files; multi-file delegation limited |
| `lib/modules/manager/pip-compile/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/pip-compile/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/pip-compile/common.ts` | `extractors/pip_compile.rs` | partial | Common utilities inline |
| `lib/modules/manager/pip-compile/dep-types.ts` | `extractors/pip_compile.rs` | full | Dep types embedded |
| `lib/modules/manager/pip-compile/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/pip-compile/utils.ts` | `extractors/pip_compile.rs` | partial | Utility functions inline |

### pip_requirements

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/pip_requirements/extract.ts` | `extractors/pip.rs` | full | `extract()` + `extract_package_file()` handle requirements.txt parsing with env var interpolation |
| `lib/modules/manager/pip_requirements/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/pip_requirements/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/pip_requirements/common.ts` | `extractors/pip.rs` | partial | Common parsing utilities inline |
| `lib/modules/manager/pip_requirements/types.ts` | N/A | out-of-scope | Type-only file |

### pip_setup

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/pip_setup/extract.ts` | `extractors/pip_setup.rs` | full | `extract()` handles setup.py install_requires |
| `lib/modules/manager/pip_setup/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/pip_setup/types.ts` | N/A | out-of-scope | Type-only file |

### pipenv

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/pipenv/extract.ts` | `extractors/pipfile.rs` | full | `extract()` + `extract_package_file()` handle Pipfile sources and deps |
| `lib/modules/manager/pipenv/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/pipenv/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/pipenv/dep-types.ts` | `extractors/pipfile.rs` | full | Dep types embedded |
| `lib/modules/manager/pipenv/types.ts` | N/A | out-of-scope | Type-only file |

### pixi

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/pixi/extract.ts` | `extractors/pixi.rs` | full | `extract()` + `extract_from_pyproject()` handle conda and pypi deps |
| `lib/modules/manager/pixi/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/pixi/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/pixi/dep-types.ts` | `extractors/pixi.rs` | full | Dep types embedded |
| `lib/modules/manager/pixi/schema.ts` | N/A | out-of-scope | Zod schema |

### poetry

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/poetry/extract.ts` | `extractors/poetry.rs` | full | `extract()` + `extract_package_file()` + `extract_with_lockfile()` handle all sections |
| `lib/modules/manager/poetry/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/poetry/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/poetry/dep-types.ts` | `extractors/poetry.rs` | full | Dep types embedded |
| `lib/modules/manager/poetry/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/poetry/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/poetry/update-locked.ts` | `extractors/poetry.rs` | full | `update_locked_poetry_dependency()` present |

### pre-commit

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/pre-commit/extract.ts` | `extractors/pre_commit.rs` | full | `extract()` + `extract_with_private_hosts()` handle pre-commit hooks |
| `lib/modules/manager/pre-commit/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/pre-commit/dep-types.ts` | `extractors/pre_commit.rs` | full | Dep types embedded (GitHost) |
| `lib/modules/manager/pre-commit/parsing.ts` | `extractors/pre_commit.rs` | partial | Parsing utilities inline |
| `lib/modules/manager/pre-commit/types.ts` | N/A | out-of-scope | Type-only file |

### proto

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/proto/extract.ts` | `extractors/proto.rs` | full | `extract_package_file()` handles .prototools files |
| `lib/modules/manager/proto/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/proto/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/proto/upgradeable-tooling.ts` | `extractors/proto.rs` | partial | Tooling config inline |

### pub

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/pub/extract.ts` | `extractors/pubspec.rs` | full | `extract()` + `extract_package_file()` handle pubspec.yaml |
| `lib/modules/manager/pub/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/pub/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/pub/dep-types.ts` | `extractors/pubspec.rs` | full | Dep types embedded (dart, git-refs) |
| `lib/modules/manager/pub/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/pub/utils.ts` | `extractors/pubspec.rs` | full | `parse_pubspec()` + `parse_pubspec_lock()` present |

### puppet

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/puppet/extract.ts` | `extractors/puppet.rs` | full | `extract()` handles Puppetfile modules |
| `lib/modules/manager/puppet/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/puppet/common.ts` | `extractors/puppet.rs` | partial | Common utilities inline |
| `lib/modules/manager/puppet/puppetfile-parser.ts` | `extractors/puppet.rs` | partial | Parser inline |
| `lib/modules/manager/puppet/types.ts` | N/A | out-of-scope | Type-only file |

### pyenv

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/pyenv/extract.ts` | `extractors/pyenv.rs` | full | `extract()` handles .python-version files |
| `lib/modules/manager/pyenv/index.ts` | `managers.rs` | partial | fileMatch pattern present |

### quadlet

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/quadlet/extract.ts` | `extractors/quadlet.rs` | full | `extract()` handles quadlet container/image/volume files |
| `lib/modules/manager/quadlet/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/quadlet/dep-types.ts` | `extractors/quadlet.rs` | full | Dep types embedded |
| `lib/modules/manager/quadlet/schema.ts` | N/A | out-of-scope | Zod schema |

### renovate-config

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/renovate-config/extract.ts` | `extractors/renovate_config_presets.rs` | full | `extract()` + `extract_package_file()` + `parse_preset()` handle renovate config presets |
| `lib/modules/manager/renovate-config/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/renovate-config/schema.ts` | N/A | out-of-scope | Zod schema |

### ruby-version

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/ruby-version/extract.ts` | `extractors/version_file.rs` | full | `extract()` with `"ruby-version"` manager handles ruby version files |
| `lib/modules/manager/ruby-version/index.ts` | `managers.rs` | partial | fileMatch pattern present |

### runtime-version

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/runtime-version/extract.ts` | `extractors/runtime_version.rs` | full | `extract()` handles runtime.txt |
| `lib/modules/manager/runtime-version/index.ts` | `managers.rs` | partial | fileMatch pattern present |

### rust-toolchain

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/rust-toolchain/extract.ts` | `extractors/rust_toolchain.rs` | full | `extract()` handles rust-toolchain.toml and legacy files |
| `lib/modules/manager/rust-toolchain/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/rust-toolchain/dep-types.ts` | `extractors/rust_toolchain.rs` | full | Dep types embedded |
| `lib/modules/manager/rust-toolchain/schema.ts` | N/A | out-of-scope | Zod schema |

### sbt

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/sbt/extract.ts` | `extractors/sbt.rs` | full | `extract()` + `extract_package_file()` + `extract_all_package_files()` handle SBT deps |
| `lib/modules/manager/sbt/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/sbt/dep-types.ts` | `extractors/sbt.rs` | full | Dep types embedded |
| `lib/modules/manager/sbt/update.ts` | `extractors/sbt.rs` | full | `bump_package_version()` present |
| `lib/modules/manager/sbt/util.ts` | `extractors/sbt.rs` | partial | `sort_package_files()`, `normalize_scala_version()` present |

### scalafmt

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/scalafmt/extract.ts` | `extractors/scalafmt.rs` | full | `extract()` handles .scalafmt.conf version |
| `lib/modules/manager/scalafmt/index.ts` | `managers.rs` | partial | fileMatch pattern present |

### setup-cfg

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/setup-cfg/extract.ts` | `extractors/setup_cfg.rs` | full | `extract()` handles setup.cfg dependencies |
| `lib/modules/manager/setup-cfg/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/setup-cfg/dep-types.ts` | `extractors/setup_cfg.rs` | full | Dep types embedded |

### sveltos

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/sveltos/extract.ts` | `extractors/sveltos.rs` | full | `extract()` handles Sveltos ClusterProfile Helm chart refs |
| `lib/modules/manager/sveltos/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/sveltos/dep-types.ts` | `extractors/sveltos.rs` | full | Dep types embedded |
| `lib/modules/manager/sveltos/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/sveltos/util.ts` | `extractors/sveltos.rs` | partial | Utility functions inline |

### swift

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/swift/extract.ts` | `extractors/spm.rs` | full | `extract()` + `extract_package_file()` handle Package.swift deps |
| `lib/modules/manager/swift/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/swift/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/swift/range.ts` | `extractors/spm.rs` | full | `get_range_strategy()` present |
| `lib/modules/manager/swift/schema.ts` | N/A | out-of-scope | Zod schema |
| `lib/modules/manager/swift/types.ts` | N/A | out-of-scope | Type-only file |

### tekton

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/tekton/extract.ts` | `extractors/tekton.rs` | full | `extract()` + `extract_annotation_deps()` handle Tekton task/pipeline images |
| `lib/modules/manager/tekton/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/tekton/dep-types.ts` | `extractors/tekton.rs` | full | Uses Kubernetes dep types |
| `lib/modules/manager/tekton/types.ts` | N/A | out-of-scope | Type-only file |

### terraform

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/terraform/extract.ts` | `extractors/terraform.rs` | full | `extract()` handles required_providers, modules, terraform version |
| `lib/modules/manager/terraform/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/terraform/base.ts` | `extractors/terraform.rs` | partial | Base utilities inline |
| `lib/modules/manager/terraform/dep-types.ts` | `extractors/terraform.rs` | full | All dep types (Provider, Module, TfeWorkspace, DockerImage, HelmRelease) present |
| `lib/modules/manager/terraform/extractors.ts` | `extractors/terraform.rs` | partial | Extractor dispatch inline |
| `lib/modules/manager/terraform/extractors/others/modules.ts` | `extractors/terraform.rs` | full | Module source classification and parsing present (regexes for GitHub, GitLab, Bitbucket, Azure, GCS, S3, generic git) |
| `lib/modules/manager/terraform/extractors/others/providers.ts` | `extractors/terraform.rs` | full | Provider source parsing present |
| `lib/modules/manager/terraform/extractors/resources/generic-docker-image-ref.ts` | `extractors/terraform.rs` | partial | Docker image ref extraction inline |
| `lib/modules/manager/terraform/extractors/resources/helm-release.ts` | `extractors/terraform.rs` | partial | Helm release extraction inline |
| `lib/modules/manager/terraform/extractors/resources/terraform-workspace.ts` | `extractors/terraform.rs` | partial | TFE workspace extraction inline |
| `lib/modules/manager/terraform/extractors/resources/utils.ts` | `extractors/terraform.rs` | partial | Resource utilities inline |
| `lib/modules/manager/terraform/extractors/terraform-block/required-provider.ts` | `extractors/terraform.rs` | full | Required provider extraction present |
| `lib/modules/manager/terraform/extractors/terraform-block/terraform-version.ts` | `extractors/terraform.rs` | full | Terraform version extraction present |
| `lib/modules/manager/terraform/hcl/index.ts` | N/A | not-started | Full HCL parser not implemented (uses regex-based extraction) |
| `lib/modules/manager/terraform/hcl/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/terraform/lockfile/hash.ts` | `extractors/terraform.rs` | partial | Lockfile hash handling inline |
| `lib/modules/manager/terraform/lockfile/index.ts` | `extractors/terraform.rs` | partial | `extract_terraform_locks()` + `extract_with_lockfile()` present. Missing: getNewConstraint, full lockfile maintenance update, hash generation, subfolder handling |
| `lib/modules/manager/terraform/lockfile/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/terraform/lockfile/update-locked.ts` | `extractors/terraform.rs` | full | `update_locked_terraform_dependency()` present |
| `lib/modules/manager/terraform/lockfile/util.ts` | `extractors/terraform.rs` | partial | Lockfile utilities inline |
| `lib/modules/manager/terraform/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/terraform/util.ts` | `extractors/terraform.rs` | partial | Utility functions inline |

### terraform-version

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/terraform-version/extract.ts` | `extractors/version_file.rs` | full | `extract()` with `"terraform-version"` manager |
| `lib/modules/manager/terraform-version/index.ts` | `managers.rs` | partial | fileMatch pattern present |

### terragrunt

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/terragrunt/extract.ts` | `extractors/terragrunt.rs` | full | `extract()` handles terragrunt.hcl module sources |
| `lib/modules/manager/terragrunt/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/terragrunt/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/terragrunt/common.ts` | `extractors/terragrunt.rs` | partial | Common utilities inline |
| `lib/modules/manager/terragrunt/dep-types.ts` | `extractors/terragrunt.rs` | full | Dep types (TerragruntSource) embedded |
| `lib/modules/manager/terragrunt/modules.ts` | `extractors/terragrunt.rs` | partial | Module source parsing inline |
| `lib/modules/manager/terragrunt/providers.ts` | `extractors/terragrunt.rs` | partial | Provider handling inline |
| `lib/modules/manager/terragrunt/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/terragrunt/util.ts` | `extractors/terragrunt.rs` | partial | `get_terragrunt_dependency_type()` present |

### terragrunt-version

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/terragrunt-version/extract.ts` | `extractors/version_file.rs` | full | `extract()` with `"terragrunt-version"` manager |
| `lib/modules/manager/terragrunt-version/index.ts` | `managers.rs` | partial | fileMatch pattern present |

### tflint-plugin

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/tflint-plugin/extract.ts` | `extractors/tflint_plugin.rs` | full | `extract()` handles .tflint.hcl plugin blocks |
| `lib/modules/manager/tflint-plugin/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/tflint-plugin/dep-types.ts` | `extractors/tflint_plugin.rs` | full | Dep types embedded |
| `lib/modules/manager/tflint-plugin/plugins.ts` | `extractors/tflint_plugin.rs` | partial | Plugin parsing inline |
| `lib/modules/manager/tflint-plugin/types.ts` | N/A | out-of-scope | Type-only file |
| `lib/modules/manager/tflint-plugin/util.ts` | `extractors/tflint_plugin.rs` | partial | Utility functions inline |

### travis

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/travis/extract.ts` | `extractors/travis.rs` | full | `extract()` handles .travis.yml node versions |
| `lib/modules/manager/travis/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/travis/types.ts` | N/A | out-of-scope | Type-only file |

### typst

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/typst/extract.ts` | `extractors/typst.rs` | full | `extract()` handles Typst package imports |
| `lib/modules/manager/typst/index.ts` | `managers.rs` | partial | fileMatch pattern present |

### unity3d

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/unity3d/extract.ts` | `extractors/unity3d.rs` | full | `extract()` handles ProjectVersion.txt |
| `lib/modules/manager/unity3d/index.ts` | `managers.rs` | partial | fileMatch pattern present |

### velaci

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/velaci/extract.ts` | `extractors/velaci.rs` | full | `extract()` handles Vela CI Docker images |
| `lib/modules/manager/velaci/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/velaci/types.ts` | N/A | out-of-scope | Type-only file |

### vendir

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/vendir/extract.ts` | `extractors/vendir.rs` | full | `extract()` handles vendir.yml Helm chart refs |
| `lib/modules/manager/vendir/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/vendir/artifacts.ts` | N/A | not-started | Artifact execution not implemented |
| `lib/modules/manager/vendir/dep-types.ts` | `extractors/vendir.rs` | full | Dep types embedded |
| `lib/modules/manager/vendir/schema.ts` | N/A | out-of-scope | Zod schema |

### woodpecker

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/woodpecker/extract.ts` | `extractors/woodpecker.rs` | full | `extract()` handles Woodpecker CI Docker images |
| `lib/modules/manager/woodpecker/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/woodpecker/types.ts` | N/A | out-of-scope | Type-only file |

### xcodegen

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lib/modules/manager/xcodegen/extract.ts` | `extractors/xcodegen.rs` | full | `extract()` handles XcodeGen project.yml package deps |
| `lib/modules/manager/xcodegen/index.ts` | `managers.rs` | partial | fileMatch pattern present |
| `lib/modules/manager/xcodegen/schema.ts` | N/A | out-of-scope | Zod schema |

---

## Datasources (`lib/modules/datasource/`)

### Infrastructure (root-level files)

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `datasource/api.ts` | `datasources.rs` (KNOWN_DATASOURCES, get_datasource_for) | full | Static registry of all datasource IDs and default versioning |
| `datasource/common.ts` | `datasources.rs` (get_datasource_for, get_datasource_default_versioning, is_get_pkg_releases_config, apply_constraints_filtering) | full | All exported functions ported |
| `datasource/datasource.ts` | `datasources.rs` (Datasource trait infrastructure) | partial | Base Datasource class; Rust uses standalone functions instead of OOP |
| `datasource/index.ts` | `datasources.rs` (get_datasource_list) | full | Registry listing |
| `datasource/types.ts` | `datasources.rs` (ReleaseResult, Release, ReleaseMeta structs) | full | Core types ported |
| `datasource/schema.ts` | — | not-started | Zod schema definitions; Rust uses serde derives inline |
| `datasource/metadata.ts` | `datasources.rs` (add_metadata, massage_timestamps, normalize_timestamp) | full | Manual changelog/source URL logic, timestamp normalization |
| `datasource/metadata-manual.ts` | `datasources.rs` (MANUAL_CHANGELOG_URLS, MANUAL_SOURCE_URLS constants) | full | Hardcoded URL maps inlined |
| `datasource/postprocess-release.ts` | `datasources.rs` (postprocess_release) | partial | Framework present; no datasource overrides wired yet |
| `datasource/util.ts` | — | not-started | Utility functions |
| `datasource/span-processor.ts` | — | out-of-scope | OpenTelemetry tracing; not in Rust scope |

### artifactory

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `artifactory/common.ts` | `datasources/artifactory.rs` | full | Common helpers merged into single file |
| `artifactory/index.ts` | `datasources/artifactory.rs` | full | getReleases ported (396 lines) |

### aws-eks-addon

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `aws-eks-addon/index.ts` | — | not-started | No Rust datasource file (versioning exists) |
| `aws-eks-addon/schema.ts` | — | not-started | |

### aws-machine-image

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `aws-machine-image/index.ts` | — | not-started | No Rust datasource file (versioning exists) |
| `aws-machine-image/types.ts` | — | not-started | |

### aws-rds

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `aws-rds/index.ts` | — | not-started | |

### azure-bicep-resource

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `azure-bicep-resource/index.ts` | `datasources/azure_bicep.rs` | full | (289 lines) |
| `azure-bicep-resource/schema.ts` | `datasources/azure_bicep.rs` | full | Schema types via serde |

### azure-pipelines-tasks

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `azure-pipelines-tasks/index.ts` | `datasources/azure_pipelines_tasks.rs` | full | (533 lines) |
| `azure-pipelines-tasks/schema.ts` | `datasources/azure_pipelines_tasks.rs` | full | Schema types via serde |

### azure-tags

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `azure-tags/index.ts` | — | not-started | |

### bazel

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `bazel/index.ts` | `datasources/bazel.rs` | full | (393 lines) |
| `bazel/schema.ts` | `datasources/bazel.rs` | full | Schema types via serde |

### bitbucket-server-tags

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `bitbucket-server-tags/index.ts` | `datasources/bitbucket_server_tags.rs` | full | (476 lines) |
| `bitbucket-server-tags/schema.ts` | `datasources/bitbucket_server_tags.rs` | full | Schema types via serde |

### bitbucket-tags

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `bitbucket-tags/index.ts` | `datasources/bitbucket_tags.rs` | full | (341 lines) |
| `bitbucket-tags/types.ts` | `datasources/bitbucket_tags.rs` | full | Types inlined |

### bitrise

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `bitrise/index.ts` | `datasources/bitrise.rs` | full | (494 lines) |
| `bitrise/schema.ts` | `datasources/bitrise.rs` | full | Schema types via serde |

### buildpacks-registry

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `buildpacks-registry/index.ts` | `datasources/buildpacks_registry.rs` | full | (216 lines) |
| `buildpacks-registry/schema.ts` | `datasources/buildpacks_registry.rs` | full | Schema types via serde |

### cdnjs

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `cdnjs/index.ts` | `datasources/cdnjs.rs` | full | (403 lines) |
| `cdnjs/schema.ts` | `datasources/cdnjs.rs` | full | Schema types via serde |

### clojure

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `clojure/common.ts` | `datasources/clojure.rs` | full | Common logic inlined |
| `clojure/index.ts` | `datasources/clojure.rs` | full | (416 lines) |

### conan

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `conan/common.ts` | `datasources/conan.rs` | full | Common logic inlined |
| `conan/index.ts` | `datasources/conan.rs` | partial | Conan Center only (130 lines); V2 not ported |
| `conan/schema.ts` | `datasources/conan.rs` | full | Schema types via serde |
| `conan/types.ts` | `datasources/conan.rs` | full | Types inlined |

### conda

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `conda/common.ts` | `datasources/conda.rs` | full | Common logic inlined |
| `conda/index.ts` | `datasources/conda.rs` | full | (583 lines) |
| `conda/prefix-dev.ts` | `datasources/conda.rs` | full | Prefix.dev API included |
| `conda/schema/prefix-dev.ts` | `datasources/conda.rs` | full | Schema types via serde |
| `conda/types.ts` | `datasources/conda.rs` | full | Types inlined |

### cpan

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `cpan/index.ts` | `datasources/cpan.rs` | full | (403 lines) |
| `cpan/schema.ts` | `datasources/cpan.rs` | full | Schema types via serde |
| `cpan/types.ts` | `datasources/cpan.rs` | full | Types inlined |

### crate

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `crate/index.ts` | `datasources/crates_io.rs` | full | (1151 lines) |
| `crate/schema.ts` | `datasources/crates_io.rs` | full | Schema types via serde |
| `crate/types.ts` | `datasources/crates_io.rs` | full | Types inlined |

### custom

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `custom/index.ts` | — | not-started | Custom datasource framework |
| `custom/schema.ts` | — | not-started | |
| `custom/utils.ts` | — | not-started | |
| `custom/formats/html.ts` | — | not-started | |
| `custom/formats/index.ts` | — | not-started | |
| `custom/formats/json.ts` | — | not-started | |
| `custom/formats/plain.ts` | — | not-started | |
| `custom/formats/toml.ts` | — | not-started | |
| `custom/formats/types.ts` | — | not-started | |
| `custom/formats/yaml.ts` | — | not-started | |

### dart

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `dart/index.ts` | `datasources/pub_dev.rs` | full | (369 lines); DATASOURCE_ID = "dart" |
| `dart/types.ts` | `datasources/pub_dev.rs` | full | Types inlined |

### dart-version

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `dart-version/index.ts` | `datasources/dart_version.rs` | full | (160 lines) |
| `dart-version/types.ts` | `datasources/dart_version.rs` | full | Types inlined |

### deb

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `deb/checksum.ts` | `datasources/deb.rs` | partial | Utilities only; no full getReleases |
| `deb/common.ts` | `datasources/deb.rs` | partial | |
| `deb/index.ts` | — | not-started | Main datasource entry not ported |
| `deb/packages.ts` | — | not-started | |
| `deb/release.ts` | — | not-started | |
| `deb/types.ts` | `datasources/deb.rs` | partial | Some types inlined |
| `deb/url.ts` | `datasources/deb.rs` | full | URL construction ported |
| `deb/utils.ts` | `datasources/deb.rs` | full | Utils ported |

### deno

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `deno/index.ts` | `datasources/deno.rs` | full | (429 lines) |
| `deno/schema.ts` | `datasources/deno.rs` | full | Schema types via serde |

### devbox

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `devbox/common.ts` | `datasources/devbox.rs` | full | Common logic inlined |
| `devbox/index.ts` | `datasources/devbox.rs` | full | (349 lines) |
| `devbox/schema.ts` | `datasources/devbox.rs` | full | Schema types via serde |

### docker

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `docker/common.ts` | `datasources/docker_hub.rs` | partial | Common logic partially covered |
| `docker/dockerhub-cache.ts` | `datasources/docker_hub.rs` | partial | Caching simplified |
| `docker/ecr.ts` | — | not-started | ECR registry support |
| `docker/google.ts` | — | not-started | Google Container Registry |
| `docker/index.ts` | `datasources/docker_hub.rs` | partial | Docker Hub tags only (626 lines); no ECR/GCR |
| `docker/schema.ts` | `datasources/docker_hub.rs` | partial | Schema types partially covered |
| `docker/types.ts` | `datasources/docker_hub.rs` | partial | Some types inlined |

### dotnet-version

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `dotnet-version/index.ts` | `datasources/dotnet_version.rs` | full | (318 lines) |
| `dotnet-version/schema.ts` | `datasources/dotnet_version.rs` | full | Schema types via serde |

### elm-package

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `elm-package/index.ts` | `datasources/elm_package.rs` | full | (187 lines) |
| `elm-package/schema.ts` | `datasources/elm_package.rs` | full | Schema types via serde |

### endoflife-date

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `endoflife-date/common.ts` | `datasources/endoflife.rs` | full | Common logic inlined |
| `endoflife-date/index.ts` | `datasources/endoflife.rs` | full | (381 lines) |
| `endoflife-date/schema.ts` | `datasources/endoflife.rs` | full | Schema types via serde |

### flutter-version

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `flutter-version/index.ts` | `datasources/flutter_version.rs` | full | (163 lines) |
| `flutter-version/types.ts` | `datasources/flutter_version.rs` | full | Types inlined |

### forgejo-releases

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `forgejo-releases/index.ts` | `datasources/forgejo_releases.rs` | full | (192 lines) |
| `forgejo-releases/schema.ts` | `datasources/forgejo_releases.rs` | full | Schema types via serde |

### forgejo-tags

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `forgejo-tags/index.ts` | `datasources/forgejo_tags.rs` | full | (173 lines) |
| `forgejo-tags/schema.ts` | `datasources/forgejo_tags.rs` | full | Schema types via serde |

### galaxy

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `galaxy/index.ts` | `datasources/galaxy.rs` | full | (345 lines) |
| `galaxy/schema.ts` | `datasources/galaxy.rs` | full | Schema types via serde |

### galaxy-collection

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `galaxy-collection/index.ts` | `datasources/galaxy_collection.rs` | full | (622 lines) |
| `galaxy-collection/schema.ts` | `datasources/galaxy_collection.rs` | full | Schema types via serde |

### git-refs

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `git-refs/base.ts` | `datasources/git_refs.rs` | full | Base logic inlined |
| `git-refs/index.ts` | `datasources/git_refs.rs` | full | (296 lines) |
| `git-refs/types.ts` | `datasources/git_refs.rs` | full | Types inlined |

### git-tags

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `git-tags/index.ts` | `datasources/git_tags.rs` | full | (160 lines) |

### gitea-releases

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `gitea-releases/index.ts` | `datasources/gitea_releases.rs` | full | (330 lines) |
| `gitea-releases/schema.ts` | `datasources/gitea_releases.rs` | full | Schema types via serde |

### gitea-tags

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `gitea-tags/index.ts` | `datasources/gitea_tags.rs` | full | (305 lines) |
| `gitea-tags/schema.ts` | `datasources/gitea_tags.rs` | full | Schema types via serde |

### github-digest

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `github-digest/index.ts` | — | not-started | |

### github-release-attachments

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `github-release-attachments/index.ts` | — | not-started | |
| `github-release-attachments/test/index.ts` | — | not-started | |

### github-releases

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `github-releases/index.ts` | `datasources/github_releases.rs` | full | (501 lines) |

### github-runners

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `github-runners/index.ts` | `datasources/github_runners.rs` | full | (432 lines) |

### github-tags

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `github-tags/index.ts` | `datasources/github_tags.rs` | full | (773 lines) |

### gitlab-packages

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `gitlab-packages/common.ts` | `datasources/gitlab_packages.rs` | full | Common logic inlined |
| `gitlab-packages/index.ts` | `datasources/gitlab_packages.rs` | full | (215 lines) |
| `gitlab-packages/types.ts` | `datasources/gitlab_packages.rs` | full | Types inlined |

### gitlab-releases

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `gitlab-releases/index.ts` | `datasources/gitlab_releases.rs` | full | (152 lines) |
| `gitlab-releases/types.ts` | `datasources/gitlab_releases.rs` | full | Types inlined |

### gitlab-tags

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `gitlab-tags/index.ts` | `datasources/gitlab_tags.rs` | full | (555 lines) |
| `gitlab-tags/types.ts` | `datasources/gitlab_tags.rs` | full | Types inlined |
| `gitlab-tags/util.ts` | `datasources/gitlab_tags.rs` | full | Utils inlined |

### glasskube-packages

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `glasskube-packages/index.ts` | `datasources/glasskube_packages.rs` | full | (365 lines) |
| `glasskube-packages/schema.ts` | `datasources/glasskube_packages.rs` | full | Schema types via serde |

### go

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `go/base.ts` | `datasources/gomod.rs` | partial | Base logic partially covered |
| `go/common.ts` | `datasources/gomod.rs` | partial | |
| `go/goproxy-parser.ts` | — | not-started | Goproxy parsing not ported |
| `go/index.ts` | `datasources/gomod.rs` | partial | (369 lines); proxy.golang.org only |
| `go/releases-direct.ts` | — | not-started | Direct VCS fetching |
| `go/releases-goproxy.ts` | `datasources/gomod.rs` | partial | Simplified proxy support |
| `go/types.ts` | `datasources/gomod.rs` | partial | Some types inlined |

### golang-version

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `golang-version/index.ts` | `datasources/golang_version.rs` | full | (319 lines) |

### gradle-version

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `gradle-version/index.ts` | `datasources/gradle_version.rs` | full | (314 lines) |
| `gradle-version/types.ts` | `datasources/gradle_version.rs` | full | Types inlined |

### hackage

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `hackage/index.ts` | `datasources/hackage.rs` | full | (250 lines) |
| `hackage/schema.ts` | `datasources/hackage.rs` | full | Schema types via serde |

### helm

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `helm/index.ts` | `datasources/helm.rs` | full | (837 lines) |
| `helm/schema.ts` | `datasources/helm.rs` | full | Schema types via serde |

### hermit

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `hermit/index.ts` | `datasources/hermit.rs` | full | (452 lines) |
| `hermit/types.ts` | `datasources/hermit.rs` | full | Types inlined |

### hex

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `hex/index.ts` | `datasources/hex.rs` | full | (244 lines) |
| `hex/schema.ts` | `datasources/hex.rs` | full | Schema types via serde |
| `hex/v2/package.ts` | — | not-started | Hex v2 API package endpoint |
| `hex/v2/signed.ts` | — | not-started | Hex v2 signed artifacts |

### hexpm-bob

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `hexpm-bob/common.ts` | `datasources/hexpm_bob.rs` | full | Common logic inlined |
| `hexpm-bob/index.ts` | `datasources/hexpm_bob.rs` | full | (396 lines) |
| `hexpm-bob/types.ts` | `datasources/hexpm_bob.rs` | full | Types inlined |

### java-version

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `java-version/common.ts` | `datasources/java_version.rs` | full | Common logic inlined |
| `java-version/index.ts` | `datasources/java_version.rs` | full | (426 lines) |
| `java-version/types.ts` | `datasources/java_version.rs` | full | Types inlined |

### jenkins-plugins

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `jenkins-plugins/index.ts` | `datasources/jenkins_plugins.rs` | full | (351 lines) |
| `jenkins-plugins/types.ts` | `datasources/jenkins_plugins.rs` | full | Types inlined |

### jsr

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `jsr/common.ts` | `datasources/jsr.rs` | full | Common logic inlined |
| `jsr/index.ts` | `datasources/jsr.rs` | full | (401 lines) |
| `jsr/schema.ts` | `datasources/jsr.rs` | full | Schema types via serde |
| `jsr/util.ts` | `datasources/jsr.rs` | full | Utils inlined |

### kubernetes-api

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `kubernetes-api/index.ts` | `datasources/kubernetes_api.rs` | full | (77 lines); inline API version data |

### maven

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `maven/common.ts` | `datasources/maven.rs` | full | Common logic inlined |
| `maven/index.ts` | `datasources/maven.rs` | full | (706 lines) |
| `maven/schema.ts` | `datasources/maven.rs` | full | Schema types via serde |
| `maven/types.ts` | `datasources/maven.rs` | full | Types inlined |
| `maven/util.ts` | `datasources/maven.rs` | full | Utils inlined |

### nextcloud

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `nextcloud/index.ts` | `datasources/nextcloud.rs` | full | (327 lines) |
| `nextcloud/schema.ts` | `datasources/nextcloud.rs` | full | Schema types via serde |

### node-version

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `node-version/common.ts` | `datasources/node_version.rs` | full | Common logic inlined |
| `node-version/index.ts` | `datasources/node_version.rs` | full | (161 lines) |
| `node-version/types.ts` | `datasources/node_version.rs` | full | Types inlined |

### npm

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `npm/common.ts` | `datasources/npm.rs` | full | Common logic inlined |
| `npm/get.ts` | `datasources/npm.rs` | full | |
| `npm/index.ts` | `datasources/npm.rs` | full | (511 lines) |
| `npm/npmrc.ts` | — | not-started | .npmrc parsing not ported |
| `npm/schema.ts` | `datasources/npm.rs` | full | Schema types via serde |
| `npm/types.ts` | `datasources/npm.rs` | full | Types inlined |

### nuget

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `nuget/common.ts` | `datasources/nuget.rs` | full | Common logic inlined |
| `nuget/index.ts` | `datasources/nuget.rs` | full | (329 lines) |
| `nuget/types.ts` | `datasources/nuget.rs` | full | Types inlined |
| `nuget/v2.ts` | `datasources/nuget.rs` | full | v2 API covered |
| `nuget/v3.ts` | `datasources/nuget.rs` | full | v3 API covered |

### orb

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `orb/index.ts` | `datasources/orb.rs` | full | (429 lines) |
| `orb/types.ts` | `datasources/orb.rs` | full | Types inlined |

### packagist

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `packagist/index.ts` | `datasources/packagist.rs` | full | (270 lines) |
| `packagist/schema.ts` | `datasources/packagist.rs` | full | Schema types via serde |

### pod

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `pod/index.ts` | `datasources/cocoapods.rs` | full | (253 lines); CocoaPods trunk API |

### puppet-forge

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `puppet-forge/common.ts` | `datasources/puppet_forge.rs` | full | Common logic inlined |
| `puppet-forge/index.ts` | `datasources/puppet_forge.rs` | full | (369 lines) |
| `puppet-forge/types.ts` | `datasources/puppet_forge.rs` | full | Types inlined |

### pypi

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `pypi/common.ts` | `datasources/pypi.rs` | full | Common logic inlined |
| `pypi/index.ts` | `datasources/pypi.rs` | full | (406 lines) |
| `pypi/types.ts` | `datasources/pypi.rs` | full | Types inlined |

### python-version

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `python-version/common.ts` | `datasources/python_version.rs` | full | Common logic inlined |
| `python-version/index.ts` | `datasources/python_version.rs` | full | (479 lines) |
| `python-version/schema.ts` | `datasources/python_version.rs` | full | Schema types via serde |

### repology

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `repology/index.ts` | `datasources/repology.rs` | full | (634 lines) |
| `repology/types.ts` | `datasources/repology.rs` | full | Types inlined |

### rpm

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `rpm/common.ts` | — | not-started | |
| `rpm/index.ts` | — | not-started | |
| `rpm/providers/common.ts` | — | not-started | |
| `rpm/providers/xml.ts` | — | not-started | |
| `rpm/repomd.ts` | — | not-started | |

### ruby-version

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `ruby-version/index.ts` | `datasources/ruby_version.rs` | full | (242 lines) |

### rubygems

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `rubygems/common.ts` | `datasources/rubygems.rs` | full | Common logic inlined |
| `rubygems/index.ts` | `datasources/rubygems.rs` | full | (472 lines) |
| `rubygems/metadata-cache.ts` | `datasources/rubygems.rs` | full | Metadata caching inlined |
| `rubygems/schema.ts` | `datasources/rubygems.rs` | full | Schema types via serde |
| `rubygems/versions-endpoint-cache.ts` | `datasources/rubygems.rs` | full | Versions cache inlined |

### rust-version

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `rust-version/index.ts` | `datasources/rust_version.rs` | full | (424 lines) |
| `rust-version/parse.ts` | `datasources/rust_version.rs` | full | Parsing inlined |

### sbt-package

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `sbt-package/index.ts` | `datasources/sbt_package.rs` | full | (1410 lines) |
| `sbt-package/util.ts` | `datasources/sbt_package.rs` | full | Utils inlined |

### sbt-plugin

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `sbt-plugin/index.ts` | `datasources/sbt_plugin.rs` | full | (718 lines) |

### terraform-module

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `terraform-module/base.ts` | `datasources/terraform.rs` | full | Combined terraform-module + terraform-provider (568 lines) |
| `terraform-module/index.ts` | `datasources/terraform.rs` | full | |
| `terraform-module/schema.ts` | `datasources/terraform.rs` | full | Schema types via serde |
| `terraform-module/types.ts` | `datasources/terraform.rs` | full | Types inlined |
| `terraform-module/utils.ts` | `datasources/terraform.rs` | full | Utils inlined |

### terraform-provider

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `terraform-provider/index.ts` | `datasources/terraform.rs` | full | Shares terraform.rs with terraform-module |
| `terraform-provider/schema.ts` | `datasources/terraform.rs` | full | Schema types via serde |
| `terraform-provider/types.ts` | `datasources/terraform.rs` | full | Types inlined |

### typst

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `typst/index.ts` | `datasources/typst.rs` | full | (308 lines) |
| `typst/schema.ts` | `datasources/typst.rs` | full | Schema types via serde |

### unity3d

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `unity3d/index.ts` | `datasources/unity3d.rs` | full | (495 lines) |
| `unity3d/schema.ts` | `datasources/unity3d.rs` | full | Schema types via serde |

### unity3d-packages

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `unity3d-packages/index.ts` | `datasources/unity3d_packages.rs` | full | (435 lines) |
| `unity3d-packages/schema.ts` | `datasources/unity3d_packages.rs` | full | Schema types via serde |

### Datasources without any TS directory (Rust-only)

| Rust file | Notes |
|---|---|
| `datasources/cocoapods.rs` | Maps to TS `pod/` |
| `datasources/crates_io.rs` | Maps to TS `crate/` |
| `datasources/docker_hub.rs` | Maps to TS `docker/` (subset) |
| `datasources/gomod.rs` | Maps to TS `go/` |
| `datasources/pub_dev.rs` | Maps to TS `dart/` |
| `datasources/terraform.rs` | Maps to TS `terraform-module/` + `terraform-provider/` |

---

## Versioning (`lib/modules/versioning/`)

### Infrastructure (root-level files)

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `versioning/api.ts` | `versioning.rs` (ALL_VERSIONING_IDS) | full | All IDs registered |
| `versioning/common.ts` | `versioning.rs` | full | Common types and helpers |
| `versioning/generic.ts` | `versioning.rs` | partial | Generic versioning interface |
| `versioning/index.ts` | `versioning.rs` (get_versioning_list, get_versioning_id) | full | Registry and lookup |
| `versioning/types.ts` | `versioning.rs` | full | Core types |
| `versioning/schema.ts` | — | not-started | Zod schemas; Rust uses serde |
| `versioning/distro.ts` | — | not-started | Distro versioning base |

### apk

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `apk/index.ts` | `versioning/apk.rs` | full | (952 lines) |

### aws-eks-addon

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `aws-eks-addon/index.ts` | `versioning/aws_eks_addon.rs` | full | (363 lines) |

### aws-machine-image

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `aws-machine-image/index.ts` | `versioning/aws_machine_image.rs` | full | (106 lines) |

### azure-rest-api

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `azure-rest-api/index.ts` | `versioning/azure_rest_api.rs` | full | (242 lines) |

### bazel-module

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `bazel-module/bzlmod-version.ts` | `versioning/bazel_module/bzlmod_version.rs` | full | (536 lines) |
| `bazel-module/index.ts` | `versioning/bazel_module.rs` | full | (214 lines) |

### cargo

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `cargo/index.ts` | `versioning/cargo.rs` | full | (1601 lines) |

### composer

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `composer/index.ts` | `versioning/composer.rs` | full | (957 lines) |

### conan

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `conan/common.ts` | `versioning/conan.rs` | full | Common logic inlined |
| `conan/index.ts` | `versioning/conan.rs` | full | (1984 lines) |
| `conan/range.ts` | `versioning/conan.rs` | full | Range logic inlined |

### conda

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `conda/index.ts` | `versioning/conda.rs` | full | (623 lines) |

### deb

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `deb/index.ts` | `versioning/deb.rs` | full | (598 lines) |

### debian

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `debian/common.ts` | `versioning/debian.rs` | full | Common logic inlined |
| `debian/index.ts` | `versioning/debian.rs` | full | (1006 lines) |

### deno

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `deno/index.ts` | `versioning/deno.rs` | full | (226 lines) |

### devbox

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `devbox/index.ts` | `versioning/devbox.rs` | full | (202 lines) |

### docker

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `docker/index.ts` | `versioning/docker.rs` | full | (406 lines) |

### elm

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `elm/index.ts` | `versioning/elm.rs` | full | (626 lines) |

### exact

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `exact/index.ts` | `versioning/exact.rs` | full | (208 lines) |

### git

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `git/index.ts` | `versioning/git.rs` | full | (77 lines) |

### github-actions

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `github-actions/index.ts` | `versioning/github_actions.rs` | full | (881 lines) |

### glasskube

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `glasskube/index.ts` | `versioning/glasskube.rs` | full | (157 lines) |

### go-mod-directive

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `go-mod-directive/index.ts` | `versioning/go_mod_directive.rs` | full | (194 lines) |

### gradle

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `gradle/compare.ts` | `versioning/gradle.rs` | full | Compare logic inlined |
| `gradle/index.ts` | `versioning/gradle.rs` | full | (1156 lines) |

### hashicorp

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `hashicorp/convertor.ts` | `versioning/hashicorp.rs` | full | Converter inlined |
| `hashicorp/index.ts` | `versioning/hashicorp.rs` | full | (975 lines) |

### helm

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `helm/index.ts` | `versioning/helm.rs` | full | (290 lines) |

### hermit

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `hermit/index.ts` | `versioning/hermit.rs` | full | (500 lines) |

### hex

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `hex/index.ts` | `versioning/hex.rs` | full | (708 lines) |

### ivy

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `ivy/index.ts` | `versioning/ivy.rs` | full | (379 lines) |
| `ivy/parse.ts` | `versioning/ivy.rs` | full | Parsing inlined |

### kubernetes-api

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `kubernetes-api/index.ts` | `versioning/kubernetes_api.rs` | full | (271 lines) |

### lambda-node

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `lambda-node/index.ts` | `versioning/lambda_node.rs` | full | (201 lines) |
| `lambda-node/schedule.ts` | `versioning/lambda_node.rs` | full | Schedule inlined |

### loose

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `loose/index.ts` | `versioning/loose.rs` | full | (271 lines) |

### maven

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `maven/compare.ts` | `versioning/maven.rs` | full | Compare logic inlined |
| `maven/index.ts` | `versioning/maven.rs` | full | (1636 lines) |

### nixpkgs

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `nixpkgs/index.ts` | `versioning/nixpkgs.rs` | full | (223 lines) |

### node

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `node/index.ts` | `versioning/node.rs` | full | (337 lines) |
| `node/schedule.ts` | `versioning/node.rs` | full | Schedule inlined |

### npm

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `npm/index.ts` | `versioning/npm.rs` | full | (970 lines) |
| `npm/range.ts` | `versioning/npm.rs` | full | Range logic inlined |

### nuget

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `nuget/index.ts` | `versioning/nuget.rs` | full | (2358 lines) |
| `nuget/parser.ts` | `versioning/nuget.rs` | full | Parser inlined |
| `nuget/range.ts` | `versioning/nuget.rs` | full | Range logic inlined |
| `nuget/types.ts` | `versioning/nuget.rs` | full | Types inlined |
| `nuget/version.ts` | `versioning/nuget.rs` | full | Version logic inlined |

### pep440

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `pep440/index.ts` | `versioning/pep440.rs` | full | (1295 lines) |
| `pep440/range.ts` | `versioning/pep440.rs` | full | Range logic inlined |

### perl

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `perl/index.ts` | `versioning/perl.rs` | full | (269 lines) |

### poetry

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `poetry/index.ts` | `versioning/poetry.rs` | full | (1224 lines) |
| `poetry/patterns.ts` | `versioning/poetry.rs` | full | Patterns inlined |
| `poetry/transform.ts` | `versioning/poetry.rs` | full | Transform inlined |

### pvp

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `pvp/index.ts` | `versioning/pvp.rs` | full | (619 lines) |
| `pvp/range.ts` | `versioning/pvp.rs` | full | Range logic inlined |
| `pvp/types.ts` | `versioning/pvp.rs` | full | Types inlined |
| `pvp/util.ts` | `versioning/pvp.rs` | full | Utils inlined |

### python

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `python/index.ts` | `versioning/python.rs` | full | (258 lines) |

### redhat

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `redhat/index.ts` | `versioning/redhat.rs` | full | (127 lines) |

### regex

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `regex/index.ts` | `versioning/regex_versioning.rs` | full | (847 lines) |

### rez

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `rez/index.ts` | `versioning/rez.rs` | full | (1576 lines) |
| `rez/pattern.ts` | `versioning/rez.rs` | full | Pattern logic inlined |
| `rez/transform.ts` | `versioning/rez.rs` | full | Transform inlined |

### rpm

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `rpm/index.ts` | `versioning/rpm.rs` | full | (453 lines) |

### ruby

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `ruby/index.ts` | `versioning/ruby.rs` | full | (1294 lines) |
| `ruby/operator.ts` | `versioning/ruby.rs` | full | Operator logic inlined |
| `ruby/range.ts` | `versioning/ruby.rs` | full | Range logic inlined |
| `ruby/strategies/bump.ts` | `versioning/ruby.rs` | full | Bump strategy inlined |
| `ruby/strategies/index.ts` | `versioning/ruby.rs` | full | Strategy dispatch inlined |
| `ruby/strategies/replace.ts` | `versioning/ruby.rs` | full | Replace strategy inlined |
| `ruby/strategies/widen.ts` | `versioning/ruby.rs` | full | Widen strategy inlined |
| `ruby/version.ts` | `versioning/ruby.rs` | full | Version logic inlined |

### rust-release-channel

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `rust-release-channel/index.ts` | `versioning/rust_release_channel.rs` | full | (1043 lines) |
| `rust-release-channel/parse.ts` | `versioning/rust_release_channel.rs` | full | Parsing inlined |
| `rust-release-channel/types.ts` | `versioning/rust_release_channel.rs` | full | Types inlined |
| `rust-release-channel/util.ts` | `versioning/rust_release_channel.rs` | full | Utils inlined |

### same-major

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `same-major/index.ts` | `versioning/same_major.rs` | full | (119 lines) |

### semver

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `semver/common.ts` | `versioning/semver_generic.rs` | full | (338 lines) |
| `semver/index.ts` | `versioning/semver_node.rs` | full | (146 lines); splits into semver_node + semver_generic |

### semver-coerced

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `semver-coerced/index.ts` | `versioning/semver_coerced.rs` | full | (615 lines) |

### semver-partial

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `semver-partial/index.ts` | `versioning/semver_partial.rs` | full | (791 lines) |

### swift

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `swift/index.ts` | `versioning/swift.rs` | full | (458 lines) |
| `swift/range.ts` | `versioning/swift.rs` | full | Range logic inlined |

### ubuntu

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `ubuntu/common.ts` | `versioning/ubuntu.rs` | full | Common logic inlined |
| `ubuntu/index.ts` | `versioning/ubuntu.rs` | full | (956 lines) |

### unity3d

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `unity3d/index.ts` | `versioning/unity3d.rs` | full | (247 lines) |

### unity3d-packages

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `unity3d-packages/index.ts` | `versioning/unity3d_packages.rs` | full | (202 lines) |

---

## Platform (`lib/modules/platform/`)

### Infrastructure (root-level files)

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `platform/api.ts` | `platform.rs` (PlatformClient trait, AnyPlatformClient) | full | Trait + enum dispatch |
| `platform/types.ts` | `platform.rs` (CurrentUser, RawFile, PlatformError) | full | Core types ported |
| `platform/index.ts` | `platform.rs` (AnyPlatformClient::create) | full | Platform factory |
| `platform/comment.ts` | — | not-started | PR comment formatting |
| `platform/pr-body.ts` | `platform/pr_body.rs` | full | PR body construction |
| `platform/scm.ts` | — | not-started | SCM interface |
| `platform/default-scm.ts` | — | not-started | Default SCM implementation |
| `platform/util.ts` | `platform/util.rs` | full | (69 lines) |

### azure

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `azure/azure-got-wrapper.ts` | `platform/azure_utils.rs` | partial | HTTP helpers only (431 lines); no full client |
| `azure/azure-helper.ts` | `platform/azure_utils.rs` | partial | Some helpers ported |
| `azure/index.ts` | — | not-started | Main platform client |
| `azure/schema.ts` | — | not-started | |
| `azure/types.ts` | — | not-started | |
| `azure/util.ts` | — | not-started | |

### bitbucket

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `bitbucket/comments.ts` | — | not-started | |
| `bitbucket/index.ts` | — | not-started | |
| `bitbucket/pr-cache.ts` | — | not-started | |
| `bitbucket/schema.ts` | — | not-started | |
| `bitbucket/types.ts` | — | not-started | |
| `bitbucket/utils.ts` | — | not-started | |

### bitbucket-server

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `bitbucket-server/index.ts` | — | not-started | |
| `bitbucket-server/pr-cache.ts` | — | not-started | |
| `bitbucket-server/schema.ts` | — | not-started | |
| `bitbucket-server/types.ts` | — | not-started | |
| `bitbucket-server/utils.ts` | — | not-started | |

### codecommit

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `codecommit/codecommit-client.ts` | — | not-started | |
| `codecommit/index.ts` | — | not-started | |

### forgejo

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `forgejo/forgejo-helper.ts` | `platform/gitea_forgejo_utils.rs` | partial | Shared Gitea/Forgejo helpers (201 lines) |
| `forgejo/index.ts` | — | not-started | Main platform client |
| `forgejo/pr-cache.ts` | — | not-started | |
| `forgejo/schema.ts` | — | not-started | |
| `forgejo/types.ts` | — | not-started | |
| `forgejo/utils.ts` | — | not-started | |

### gerrit

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `gerrit/client.ts` | — | not-started | |
| `gerrit/index.ts` | — | not-started | |
| `gerrit/scm.ts` | — | not-started | |
| `gerrit/types.ts` | — | not-started | |
| `gerrit/utils.ts` | — | not-started | |

### gitea

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `gitea/gitea-helper.ts` | `platform/gitea_forgejo_utils.rs` | partial | Shared Gitea/Forgejo helpers (201 lines) |
| `gitea/index.ts` | — | not-started | Main platform client |
| `gitea/pr-cache.ts` | — | not-started | |
| `gitea/schema.ts` | — | not-started | |
| `gitea/types.ts` | — | not-started | |
| `gitea/utils.ts` | — | not-started | |

### github

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `github/api-cache.ts` | `platform/github_api_cache.rs` | full | (569 lines) |
| `github/branch.ts` | `platform/github.rs` | partial | Branch ops partially covered |
| `github/common.ts` | `platform/github.rs` | partial | Common helpers |
| `github/graphql.ts` | `platform/github.rs` | partial | Some GraphQL queries ported |
| `github/index.ts` | `platform/github.rs` | full | (1056 lines); main client |
| `github/issue.ts` | `platform/github.rs` | partial | Issue operations |
| `github/massage-markdown-links.ts` | `platform/github.rs` | partial | Markdown link massaging |
| `github/pr.ts` | `platform/github.rs` | partial | PR operations |
| `github/schema.ts` | `platform/github.rs` | partial | Schema types via serde |
| `github/scm.ts` | — | not-started | GitHub SCM |
| `github/types.ts` | `platform/github.rs` | full | Types inlined |
| `github/user.ts` | `platform/github.rs` | partial | User operations |
| `github/utils.ts` | `platform/github.rs` | partial | Utility functions |

### gitlab

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `gitlab/code-owners.ts` | — | not-started | CODEOWNERS parsing |
| `gitlab/http.ts` | `platform/gitlab.rs` | full | HTTP client inlined |
| `gitlab/index.ts` | `platform/gitlab.rs` | full | (737 lines); main client |
| `gitlab/merge-request.ts` | `platform/gitlab.rs` | partial | MR operations |
| `gitlab/pr-cache.ts` | `platform/gitlab.rs` | partial | PR caching |
| `gitlab/schema.ts` | `platform/gitlab.rs` | partial | Schema types via serde |
| `gitlab/types.ts` | `platform/gitlab.rs` | full | Types inlined |
| `gitlab/utils.ts` | `platform/gitlab.rs` | partial | Utility functions |

### local

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `local/index.ts` | `platform/local.rs` | full | (266 lines) |
| `local/scm.ts` | `platform/local.rs` | full | SCM ops inlined |

### scm-manager

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `scm-manager/index.ts` | `platform/scm_manager.rs` | partial | (262 lines); basic operations |
| `scm-manager/mapper.ts` | `platform/scm_manager.rs` | partial | Mapping logic |
| `scm-manager/schema.ts` | `platform/scm_manager.rs` | partial | Schema types |
| `scm-manager/scm-manager-helper.ts` | `platform/scm_manager.rs` | partial | Helper functions |
| `scm-manager/types.ts` | `platform/scm_manager.rs` | partial | Types inlined |
| `scm-manager/utils.ts` | `platform/scm_manager.rs` | partial | Utility functions |

### utils

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `utils/pr-body.ts` | `platform/pr_body.rs` | full | Covered by pr_body.rs |
| `utils/read-only-issue-body.ts` | — | not-started | |

---

## Config (`lib/config/`)

### config (top-level)

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `config/app-strings.ts` | `repo_config.rs` | full | Config file name constants (`getConfigFileNames`) embedded in repo_config |
| `config/decrypt.ts` | `config/decrypt.rs` | partial | Core decrypt logic ported; sub-delegates bcpgp/openpgp not yet split out |
| `config/decrypt/bcpgp.ts` | — | not-started | BCP/GPG decryption sub-module |
| `config/decrypt/openpgp.ts` | — | not-started | OpenPGP decryption sub-module |
| `config/defaults.ts` | `config.rs` | full | `GlobalConfig` defaults and `GLOBAL_CONFIG_OPTIONS` list |
| `config/global.ts` | `config.rs` | full | Global-only option names, `GlobalConfig` struct |
| `config/index.ts` | `config.rs` | full | Re-exports from config module |
| `config/inherit.ts` | — | not-started | Config inheritance |
| `config/massage.ts` | `config/massage.rs` | full | Config normalization (string→array coercion, nested massage) |
| `config/migrate-validate.ts` | `config/migrate_validate.rs` | full | Migration + validation (7060 lines); includes validation logic |
| `config/migration.ts` | — | not-started | Top-level migration dispatcher |
| `config/schema.ts` | — | not-started | JSON Schema generation |
| `config/secrets.ts` | `config/secrets.rs` | full | Secrets and variables validation + interpolation |
| `config/types.ts` | `config.rs`, `repo_config.rs` | full | Types defined inline in Rust structs |
| `config/utils.ts` | — | not-started | Config utility helpers |
| `config/parse.ts` | — | not-started | Config parsing orchestration |
| `config/validation.ts` | `config/migrate_validate.rs` | partial | Validation merged into migrate_validate; standalone `validate()` not split out |

### config/migrations (base)

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `config/migrations/base/abstract-migration.ts` | — | not-started | Base migration class |
| `config/migrations/base/remove-property-migration.ts` | — | not-started | Remove-property migration |
| `config/migrations/base/rename-property-migration.ts` | — | not-started | Rename-property migration |

### config/migrations (custom)

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `config/migrations/custom/automerge-major-migration.ts` | — | not-started | |
| `config/migrations/custom/automerge-migration.ts` | — | not-started | |
| `config/migrations/custom/automerge-minor-migration.ts` | — | not-started | |
| `config/migrations/custom/automerge-patch-migration.ts` | — | not-started | |
| `config/migrations/custom/automerge-type-migration.ts` | — | not-started | |
| `config/migrations/custom/azure-gitlab-automerge-migration.ts` | — | not-started | |
| `config/migrations/custom/base-branch-migration.ts` | — | not-started | |
| `config/migrations/custom/binary-source-migration.ts` | — | not-started | |
| `config/migrations/custom/branch-name-migration.ts` | — | not-started | |
| `config/migrations/custom/branch-prefix-migration.ts` | — | not-started | |
| `config/migrations/custom/compatibility-migration.ts` | — | not-started | |
| `config/migrations/custom/composer-ignore-platform-reqs-migration.ts` | — | not-started | |
| `config/migrations/custom/custom-managers-migration.ts` | — | not-started | |
| `config/migrations/custom/datasource-migration.ts` | — | not-started | |
| `config/migrations/custom/dep-types-migration.ts` | — | not-started | |
| `config/migrations/custom/dry-run-migration.ts` | — | not-started | |
| `config/migrations/custom/enabled-managers-migration.ts` | — | not-started | |
| `config/migrations/custom/extends-migration.ts` | — | not-started | |
| `config/migrations/custom/fetch-release-notes-migration.ts` | — | not-started | |
| `config/migrations/custom/file-match-migration.ts` | — | not-started | |
| `config/migrations/custom/go-mod-tidy-migration.ts` | — | not-started | |
| `config/migrations/custom/host-rules-migration.ts` | — | not-started | |
| `config/migrations/custom/ignore-node-modules-migration.ts` | — | not-started | |
| `config/migrations/custom/ignore-npmrc-file-migration.ts` | — | not-started | |
| `config/migrations/custom/include-forks-migration.ts` | — | not-started | |
| `config/migrations/custom/match-datasources-migration.ts` | — | not-started | |
| `config/migrations/custom/match-managers-migration.ts` | — | not-started | |
| `config/migrations/custom/match-strings-migration.ts` | — | not-started | |
| `config/migrations/custom/node-migration.ts` | — | not-started | |
| `config/migrations/custom/package-files-migration.ts` | — | not-started | |
| `config/migrations/custom/package-name-migration.ts` | — | not-started | |
| `config/migrations/custom/package-pattern-migration.ts` | — | not-started | |
| `config/migrations/custom/package-rules-migration.ts` | — | not-started | |
| `config/migrations/custom/packages-migration.ts` | — | not-started | |
| `config/migrations/custom/path-rules-migration.ts` | — | not-started | |
| `config/migrations/custom/pin-versions-migration.ts` | — | not-started | |
| `config/migrations/custom/platform-commit-migration.ts` | — | not-started | |
| `config/migrations/custom/post-update-options-migration.ts` | — | not-started | |
| `config/migrations/custom/rebase-conflicted-prs-migration.ts` | — | not-started | |
| `config/migrations/custom/rebase-stale-prs-migration.ts` | — | not-started | |
| `config/migrations/custom/recreate-closed-migration.ts` | — | not-started | |
| `config/migrations/custom/renovate-fork-migration.ts` | — | not-started | |
| `config/migrations/custom/require-config-migration.ts` | — | not-started | |
| `config/migrations/custom/required-status-checks-migration.ts` | — | not-started | |
| `config/migrations/custom/schedule-migration.ts` | — | not-started | |
| `config/migrations/custom/semantic-commits-migration.ts` | — | not-started | |
| `config/migrations/custom/semantic-prefix-migration.ts` | — | not-started | |
| `config/migrations/custom/separate-major-release-migration.ts` | — | not-started | |
| `config/migrations/custom/separate-multiple-major-migration.ts` | — | not-started | |
| `config/migrations/custom/stability-days-migration.ts` | — | not-started | |
| `config/migrations/custom/suppress-notifications-migration.ts` | — | not-started | |
| `config/migrations/custom/trust-level-migration.ts` | — | not-started | |
| `config/migrations/custom/unpublish-safe-migration.ts` | — | not-started | |
| `config/migrations/custom/update-lock-files-migration.ts` | — | not-started | |
| `config/migrations/custom/upgrade-in-range-migration.ts` | — | not-started | |
| `config/migrations/custom/version-strategy-migration.ts` | — | not-started | |

### config/migrations (index + service)

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `config/migrations/index.ts` | — | not-started | Migration index |
| `config/migrations/migrations-service.ts` | — | not-started | Migration service orchestration |
| `config/migrations/types.ts` | — | not-started | Migration-specific types |

### config/options

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `config/options/env-options.ts` | `config_env.rs` (CLI) | partial | Env-to-option mappings done in CLI crate's config_env |
| `config/options/env.ts` | `config_env.rs` (CLI) | partial | Env var parsing; subset ported |
| `config/options/index.ts` | `config.rs` | partial | Core option definitions ported; not all ~300 options covered |

### config/presets (top-level)

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `config/presets/common.ts` | — | not-started | Preset common utilities |
| `config/presets/forgejo/index.ts` | — | not-started | Forgejo preset resolver |
| `config/presets/gitea/index.ts` | — | not-started | Gitea preset resolver |
| `config/presets/github/index.ts` | — | not-started | GitHub preset resolver |
| `config/presets/gitlab/index.ts` | — | not-started | GitLab preset resolver |
| `config/presets/http/index.ts` | — | not-started | HTTP preset resolver |
| `config/presets/index.ts` | — | not-started | Preset resolution orchestrator |
| `config/presets/local/common.ts` | — | not-started | Local preset common |
| `config/presets/local/index.ts` | — | not-started | Local preset resolver |
| `config/presets/npm/index.ts` | — | not-started | npm preset resolver |
| `config/presets/parse.ts` | — | not-started | Preset name parser |
| `config/presets/types.ts` | — | not-started | Preset types |
| `config/presets/util.ts` | — | not-started | Preset utilities |

### config/presets/internal

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `config/presets/internal/abandonments.preset.ts` | — | not-started | |
| `config/presets/internal/auto-generate-replacements.ts` | — | not-started | |
| `config/presets/internal/config.preset.ts` | — | not-started | |
| `config/presets/internal/custom-managers.preset.ts` | — | not-started | |
| `config/presets/internal/default.preset.ts` | — | not-started | |
| `config/presets/internal/docker.preset.ts` | — | not-started | |
| `config/presets/internal/global.preset.ts` | — | not-started | |
| `config/presets/internal/group.preset.ts` | — | not-started | |
| `config/presets/internal/helpers.preset.ts` | — | not-started | |
| `config/presets/internal/index.ts` | — | not-started | |
| `config/presets/internal/merge-confidence.preset.ts` | — | not-started | |
| `config/presets/internal/monorepos.preset.ts` | `monorepos.rs` | full | Monorepo preset generation from embedded JSON |
| `config/presets/internal/packages.preset.ts` | — | not-started | |
| `config/presets/internal/preview.preset.ts` | — | not-started | |
| `config/presets/internal/replacements.preset.ts` | `replacements.rs` | full | Replacement preset generation from embedded JSON |
| `config/presets/internal/schedule.preset.ts` | — | not-started | |
| `config/presets/internal/security.preset.ts` | — | not-started | |
| `config/presets/internal/workarounds.preset.ts` | — | not-started | |

### config/validation-helpers

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `config/validation-helpers/match-base-branches.ts` | — | not-started | Base branch validation |
| `config/validation-helpers/regex-glob-matchers.ts` | — | not-started | Regex/glob matcher validation |
| `config/validation-helpers/types.ts` | — | not-started | Validation helper types |
| `config/validation-helpers/utils.ts` | — | not-started | Validation helper utilities |

### Config summary

| Status | Count |
|---|---|
| full | 9 |
| partial | 5 |
| not-started | 74 |
| out-of-scope | 0 |

---

## Util (`lib/util/`)

### util (top-level)

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `util/array.ts` | — | not-started | Array utility functions |
| `util/assign-keys.ts` | — | not-started | Key assignment utility |
| `util/check-token.ts` | — | not-started | Token validation |
| `util/clone.ts` | — | not-started | Deep clone |
| `util/coerce.ts` | — | not-started | Coercion utilities |
| `util/common.ts` | — | not-started | Common utilities |
| `util/compress.ts` | — | not-started | Compression utilities |
| `util/date.ts` | — | not-started | Date utilities |
| `util/emoji.ts` | — | not-started | Emoji utilities |
| `util/env.ts` | `util.rs` | partial | BASIC_ENV_VARS list ported; full env filtering not done |
| `util/filter-map.ts` | — | not-started | Filter-map utility |
| `util/fingerprint.ts` | — | not-started | Fingerprinting |
| `util/hash.ts` | — | not-started | Hashing utilities |
| `util/host-rules.ts` | `util/host_rules.rs` | full | Host rules matching, storage, and lookup |
| `util/html.ts` | — | not-started | HTML utilities |
| `util/ignore.ts` | — | not-started | Ignore utilities |
| `util/interpolator.ts` | — | not-started | Template interpolation |
| `util/jsonata.ts` | — | not-started | JSONata expression evaluation |
| `util/lazy.ts` | — | not-started | Lazy initialization |
| `util/markdown.ts` | — | not-started | Markdown utilities |
| `util/mask.ts` | — | not-started | Masking utilities |
| `util/memoize.ts` | — | not-started | Memoization |
| `util/minimatch.ts` | — | not-started | Minimatch wrapper |
| `util/modules.ts` | — | not-started | Module utilities |
| `util/mutex.ts` | — | not-started | Mutex utilities |
| `util/number.ts` | — | not-started | Number utilities |
| `util/object.ts` | — | not-started | Object utilities |
| `util/pretty-time.ts` | — | not-started | Human-readable time formatting |
| `util/promises.ts` | — | not-started | Promise utilities |
| `util/range.ts` | — | not-started | Range utilities |
| `util/regex.ts` | — | not-started | Regex utilities |
| `util/result.ts` | — | not-started | Result type utilities |
| `util/s3.ts` | — | not-started | S3 client |
| `util/sample.ts` | — | not-started | Sampling utilities |
| `util/sanitize.ts` | — | not-started | Sanitization |
| `util/split.ts` | — | not-started | String splitting |
| `util/stats.ts` | — | not-started | Statistics |
| `util/streams.ts` | — | not-started | Stream utilities |
| `util/string.ts` | — | not-started | String utilities |
| `util/stringify.ts` | — | not-started | Stringify utilities |
| `util/toml.ts` | — | not-started | TOML parser |
| `util/unicode.ts` | — | not-started | Unicode utilities |
| `util/uniq.ts` | — | not-started | Unique utilities |
| `util/url.ts` | — | not-started | URL utilities |
| `util/yaml.ts` | — | not-started | YAML parser |

### util/cache/memory

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `util/cache/memory/index.ts` | `cache/memory.rs` | full | In-memory key-value cache (MemCache) |

### util/cache/package

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `util/cache/package/backend.ts` | `cache/package.rs` | partial | Backend abstraction; file-based impl only |
| `util/cache/package/impl/base.ts` | `cache/package.rs` | partial | Base cache trait merged into package.rs |
| `util/cache/package/impl/file.ts` | `cache/package.rs` | full | File-based package cache |
| `util/cache/package/impl/redis.ts` | — | not-started | Redis cache backend |
| `util/cache/package/impl/sqlite.ts` | — | not-started | SQLite cache backend |
| `util/cache/package/index.ts` | `cache/package.rs` | full | Package cache facade |
| `util/cache/package/key.ts` | `cache/package.rs` | partial | Cache key generation merged in |
| `util/cache/package/namespaces.ts` | `config/migrate_validate.rs` | full | Namespace list referenced in migrate_validate |
| `util/cache/package/ttl.ts` | `cache/package.rs` | full | TTL handling |
| `util/cache/package/types.ts` | `cache/package.rs` | partial | Types defined inline |
| `util/cache/package/with-cache.ts` | `cache/package.rs` | full | withCache wrapper with soft/hard TTL |

### util/cache/repository

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `util/cache/repository/common.ts` | — | not-started | Repository cache common |
| `util/cache/repository/http-cache.ts` | — | not-started | HTTP cache |
| `util/cache/repository/impl/base.ts` | — | not-started | Base repo cache |
| `util/cache/repository/impl/cache-factory.ts` | — | not-started | Cache factory |
| `util/cache/repository/impl/local.ts` | — | not-started | Local repo cache |
| `util/cache/repository/impl/null.ts` | — | not-started | Null cache |
| `util/cache/repository/impl/s3.ts` | — | not-started | S3 repo cache |
| `util/cache/repository/index.ts` | — | not-started | Repository cache index |
| `util/cache/repository/init.ts` | — | not-started | Repository cache init |
| `util/cache/repository/schema.ts` | — | not-started | Cache schema |
| `util/cache/repository/types.ts` | — | not-started | Cache types |

### util/exec

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `util/exec/common.ts` | — | not-started | Common exec utilities |
| `util/exec/containerbase.ts` | — | not-started | Containerbase support |
| `util/exec/docker/index.ts` | — | not-started | Docker exec |
| `util/exec/env.ts` | `util.rs` | partial | BASIC_ENV_VARS + child env ported |
| `util/exec/exec-error.ts` | — | not-started | Exec error type |
| `util/exec/hermit.ts` | — | not-started | Hermit support |
| `util/exec/index.ts` | — | not-started | Exec orchestrator |
| `util/exec/types.ts` | — | not-started | Exec types |
| `util/exec/utils.ts` | — | not-started | Exec utilities |

### util/fs

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `util/fs/index.ts` | `fs.rs` | partial | `getParentDir` ported; remaining fs ops not done |
| `util/fs/util.ts` | `fs.rs` | partial | Subset of fs utilities |

### util/git

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `util/git/auth.ts` | — | not-started | Git auth |
| `util/git/author.ts` | `git/author.rs` | full | Git author parsing |
| `util/git/behind-base-branch-cache.ts` | — | not-started | Behind-base-branch cache |
| `util/git/config.ts` | `git.rs` | partial | `GitCompletionConfig`, `GitUnsafeConfig` structs |
| `util/git/conflicts-cache.ts` | — | not-started | Conflicts cache |
| `util/git/error.ts` | — | not-started | Git error types |
| `util/git/index.ts` | `git.rs` | partial | Module re-exports |
| `util/git/instrument.ts` | — | not-started | Git instrumentation |
| `util/git/modified-cache.ts` | — | not-started | Modified cache |
| `util/git/pristine.ts` | — | not-started | Pristine check |
| `util/git/private-key.ts` | — | not-started | SSH private key handling |
| `util/git/semantic.ts` | — | not-started | Semantic commits |
| `util/git/set-branch-commit.ts` | — | not-started | Set branch commit |
| `util/git/span-processor.ts` | — | not-started | Span processor |
| `util/git/types.ts` | — | not-started | Git types |
| `util/git/update-date-cache.ts` | — | not-started | Update date cache |
| `util/git/url.ts` | — | not-started | Git URL utilities |

### util/github/graphql

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `util/github/graphql/cache-strategies/abstract-cache-strategy.ts` | — | not-started | |
| `util/github/graphql/cache-strategies/memory-cache-strategy.ts` | — | not-started | |
| `util/github/graphql/cache-strategies/package-cache-strategy.ts` | — | not-started | |
| `util/github/graphql/datasource-fetcher.ts` | — | not-started | |
| `util/github/graphql/index.ts` | — | not-started | |
| `util/github/graphql/query-adapters/branches-query-adapter.ts` | — | not-started | |
| `util/github/graphql/query-adapters/releases-query-adapter.ts` | — | not-started | |
| `util/github/graphql/query-adapters/tags-query-adapter.ts` | — | not-started | |
| `util/github/graphql/types.ts` | — | not-started | |
| `util/github/graphql/util.ts` | — | not-started | |

### util/github (non-graphql)

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `util/github/tags.ts` | — | not-started | GitHub tags |
| `util/github/types.ts` | — | not-started | GitHub types |
| `util/github/url.ts` | — | not-started | GitHub URL utilities |

### util/http (core)

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `util/http/auth.ts` | `http.rs` | partial | Auth handled inline in http client |
| `util/http/bitbucket-server.ts` | — | not-started | Bitbucket Server HTTP |
| `util/http/bitbucket.ts` | — | not-started | Bitbucket HTTP |
| `util/http/errors.ts` | — | not-started | HTTP error types |
| `util/http/forgejo.ts` | — | not-started | Forgejo HTTP |
| `util/http/gerrit.ts` | — | not-started | Gerrit HTTP |
| `util/http/gitea.ts` | — | not-started | Gitea HTTP |
| `util/http/github.ts` | — | not-started | GitHub HTTP |
| `util/http/gitlab.ts` | — | not-started | GitLab HTTP |
| `util/http/got.ts` | — | not-started | Got HTTP adapter |
| `util/http/hooks.ts` | — | not-started | HTTP hooks |
| `util/http/host-rules.ts` | `http.rs`, `util/host_rules.rs` | partial | Host rules applied in http client |
| `util/http/http.ts` | `http.rs` | partial | Core HTTP client with retry; not all features |
| `util/http/index.ts` | `http.rs` | partial | HTTP module re-export |
| `util/http/jira.ts` | — | not-started | Jira HTTP |
| `util/http/keep-alive.ts` | — | not-started | Keep-alive |
| `util/http/legacy.ts` | — | not-started | Legacy HTTP |
| `util/http/queue.ts` | — | not-started | HTTP queue |
| `util/http/rate-limits.ts` | — | not-started | Rate limiting |
| `util/http/retry-after.ts` | `http.rs` | full | Retry-After header parsing + exponential backoff |
| `util/http/scm-manager.ts` | — | not-started | SCM-Manager HTTP |
| `util/http/throttle.ts` | — | not-started | Throttling |
| `util/http/types.ts` | — | not-started | HTTP types |
| `util/http/util.ts` | — | not-started | HTTP utilities |
| `util/http/www-authenticate.ts` | — | not-started | WWW-Authenticate parsing |

### util/http/cache

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `util/http/cache/abstract-http-cache-provider.ts` | — | not-started | |
| `util/http/cache/memory-http-cache-provider.ts` | — | not-started | |
| `util/http/cache/package-http-cache-provider.ts` | — | not-started | |
| `util/http/cache/repository-http-cache-provider.ts` | — | not-started | |
| `util/http/cache/schema.ts` | — | not-started | |
| `util/http/cache/types.ts` | — | not-started | |

### util/json-writer

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `util/json-writer/code-format.ts` | `json_writer.rs` | partial | Code format types |
| `util/json-writer/editor-config.ts` | `json_writer.rs` | partial | EditorConfig integration |
| `util/json-writer/indentation-type.ts` | `json_writer.rs` | full | IndentationType enum |
| `util/json-writer/index.ts` | `json_writer.rs` | full | Module re-export |
| `util/json-writer/json-writer.ts` | `json_writer.rs` | full | JSON writer with configurable indentation |

### util/merge-confidence

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `util/merge-confidence/common.ts` | `merge_confidence.rs` | full | Confidence level ordering, comparison |
| `util/merge-confidence/index.ts` | `merge_confidence.rs` | full | isMergeConfidence, satisfiesConfidenceLevel, etc. |
| `util/merge-confidence/types.ts` | `merge_confidence.rs` | partial | Types defined inline |

### util/package-rules

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `util/package-rules/base-branches.ts` | `package_rule.rs` | partial | Base branch matching |
| `util/package-rules/base.ts` | `package_rule.rs` | partial | Base matcher trait |
| `util/package-rules/categories.ts` | `package_rule.rs` | partial | Category matching |
| `util/package-rules/current-age.ts` | — | not-started | Current age matching |
| `util/package-rules/current-value.ts` | `package_rule.rs` | partial | Current value matching |
| `util/package-rules/current-version.ts` | `package_rule.rs` | partial | Current version matching |
| `util/package-rules/datasources.ts` | `package_rule.rs` | full | Datasource matching |
| `util/package-rules/dep-names.ts` | `package_rule.rs` | full | Dep name matching |
| `util/package-rules/dep-types.ts` | `package_rule.rs` | partial | Dep type matching |
| `util/package-rules/files.ts` | — | not-started | File matching |
| `util/package-rules/index.ts` | `package_rule.rs` | full | Rule evaluation loop |
| `util/package-rules/jsonata.ts` | — | not-started | JSONata matching |
| `util/package-rules/managers.ts` | `package_rule.rs` | full | Manager matching |
| `util/package-rules/matchers.ts` | `package_rule.rs` | partial | Matcher registration |
| `util/package-rules/merge-confidence.ts` | `package_rule.rs` | partial | Merge confidence matching |
| `util/package-rules/new-value.ts` | `package_rule.rs` | partial | New value matching |
| `util/package-rules/package-names.ts` | `package_rule.rs` | full | Package name matching |
| `util/package-rules/registryurls.ts` | — | not-started | Registry URL matching |
| `util/package-rules/repositories.ts` | — | not-started | Repository matching |
| `util/package-rules/sourceurls.ts` | — | not-started | Source URL matching |
| `util/package-rules/types.ts` | `package_rule.rs` | partial | Types inline |
| `util/package-rules/update-types.ts` | `package_rule.rs` | full | Update type matching |

### util/schema-utils

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `util/schema-utils/index.ts` | — | not-started | Schema utilities |
| `util/schema-utils/v4.ts` | — | not-started | JSON Schema v4 |

### util/string-match

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `util/string-match.ts` | `string_match.rs` | full | Regex/glob/exact matching with negation |

### util/template

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `util/template/index.ts` | — | not-started | Handlebars template engine |

### util/vulnerability

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `util/vulnerability/ecosystem.ts` | `vulnerability.rs` | full | Datasource→OSV ecosystem mapping |
| `util/vulnerability/utils.ts` | `vulnerability.rs` | full | Vulnerability utility functions |

### Util summary

| Status | Count |
|---|---|
| full | 21 |
| partial | 32 |
| not-started | 133 |
| out-of-scope | 0 |

---

## Workers (`lib/workers/`)

### workers/global

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `workers/global/autodiscover.ts` | — | not-started | Repository auto-discovery |
| `workers/global/index.ts` | `renovate-cli/src/main.rs` | partial | Global worker entry; basic run loop ported |
| `workers/global/initialize.ts` | — | not-started | Global initialization |

### workers/global/config/parse

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `workers/global/config/parse/__fixtures__/argv.ts` | — | out-of-scope | Test fixture |
| `workers/global/config/parse/__fixtures__/config-cjs.ts` | — | out-of-scope | Test fixture |
| `workers/global/config/parse/__fixtures__/config.ts` | — | out-of-scope | Test fixture |
| `workers/global/config/parse/additional-config-file.ts` | — | not-started | Additional config file loading |
| `workers/global/config/parse/cli.ts` | `renovate-cli/src/config_builder.rs` | full | CLI arg→GlobalConfig conversion |
| `workers/global/config/parse/codespaces.ts` | — | not-started | Codespaces config |
| `workers/global/config/parse/coersions.ts` | `renovate-cli/src/config_builder.rs` | partial | CLI coercions; subset ported |
| `workers/global/config/parse/env.ts` | `renovate-cli/src/config_env.rs` | full | Full env var→config parsing (1791 lines) |
| `workers/global/config/parse/file.ts` | `renovate-core/src/config/file.rs` | full | Config file discovery and loading |
| `workers/global/config/parse/host-rules-from-env.ts` | `renovate-core/src/config/host_rules_from_env.rs` | full | Host rules from env vars |
| `workers/global/config/parse/index.ts` | — | not-started | Parse orchestrator |
| `workers/global/config/parse/types.ts` | — | not-started | Parse types |
| `workers/global/config/parse/util.ts` | — | not-started | Parse utilities |
| `workers/global/limits.ts` | `renovate-core/src/limits.rs` | full | Rate-limiting and concurrency limits |

### workers/repository (top-level)

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `workers/repository/cache.ts` | — | not-started | Repository cache |
| `workers/repository/common.ts` | — | not-started | Common repository utilities |
| `workers/repository/configured.ts` | — | not-started | Configured check |
| `workers/repository/dependency-dashboard.ts` | — | not-started | Dependency dashboard |
| `workers/repository/error-config.ts` | — | not-started | Error config |
| `workers/repository/error.ts` | — | not-started | Error types |
| `workers/repository/errors-warnings.ts` | — | not-started | Errors/warnings handling |
| `workers/repository/index.ts` | `renovate-cli/src/main.rs` | partial | Basic repo processing loop in main |
| `workers/repository/package-files.ts` | — | not-started | Package file sorting |
| `workers/repository/result.ts` | — | not-started | Result types |

### workers/types

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `workers/types.ts` | — | not-started | Worker types |

### workers/repository/changelog

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `workers/repository/changelog/index.ts` | — | not-started | Changelog index |
| `workers/repository/changelog/types.ts` | — | not-started | Changelog types |

### workers/repository/config-migration

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `workers/repository/config-migration/branch/commit-message.ts` | — | not-started | |
| `workers/repository/config-migration/branch/create.ts` | — | not-started | |
| `workers/repository/config-migration/branch/index.ts` | — | not-started | |
| `workers/repository/config-migration/branch/migrated-data.ts` | — | not-started | |
| `workers/repository/config-migration/branch/rebase.ts` | — | not-started | |
| `workers/repository/config-migration/common.ts` | — | not-started | |
| `workers/repository/config-migration/index.ts` | — | not-started | |
| `workers/repository/config-migration/pr/index.ts` | — | not-started | |

### workers/repository/extract

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `workers/repository/extract/extract-fingerprint-config.ts` | — | not-started | |
| `workers/repository/extract/file-match.ts` | `renovate-core/src/managers.rs` | partial | File matching via manager detection |
| `workers/repository/extract/index.ts` | `renovate-cli/src/main.rs` | partial | Extraction done in pipeline loop |
| `workers/repository/extract/manager-files.ts` | — | not-started | |
| `workers/repository/extract/supersedes.ts` | — | not-started | |
| `workers/repository/extract/types.ts` | — | not-started | |

### workers/repository/finalize

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `workers/repository/finalize/index.ts` | — | not-started | Finalization |
| `workers/repository/finalize/prune.ts` | — | not-started | PR pruning |
| `workers/repository/finalize/repository-statistics.ts` | — | not-started | Repo statistics |

### workers/repository/init

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `workers/repository/init/apis.ts` | — | not-started | API initialization |
| `workers/repository/init/cache.ts` | — | not-started | Cache init |
| `workers/repository/init/config.ts` | — | not-started | Config init |
| `workers/repository/init/index.ts` | — | not-started | Init orchestrator |
| `workers/repository/init/inherited.ts` | — | not-started | Inherited config |
| `workers/repository/init/merge.ts` | — | not-started | Config merge |
| `workers/repository/init/types.ts` | — | not-started | Init types |
| `workers/repository/init/vulnerability.ts` | — | not-started | Vulnerability init |

### workers/repository/model

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `workers/repository/model/commit-message-factory.ts` | — | not-started | |
| `workers/repository/model/commit-message.ts` | — | not-started | |
| `workers/repository/model/custom-commit-message.ts` | — | not-started | |
| `workers/repository/model/semantic-commit-message.ts` | — | not-started | |

### workers/repository/onboarding

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `workers/repository/onboarding/branch/check.ts` | — | not-started | |
| `workers/repository/onboarding/branch/commit-message.ts` | — | not-started | |
| `workers/repository/onboarding/branch/config.ts` | — | not-started | |
| `workers/repository/onboarding/branch/create.ts` | — | not-started | |
| `workers/repository/onboarding/branch/index.ts` | — | not-started | |
| `workers/repository/onboarding/branch/onboarding-branch-cache.ts` | — | not-started | |
| `workers/repository/onboarding/branch/rebase.ts` | — | not-started | |
| `workers/repository/onboarding/common.ts` | — | not-started | |
| `workers/repository/onboarding/pr/base-branch.ts` | — | not-started | |
| `workers/repository/onboarding/pr/config-description.ts` | `renovate-core/src/onboarding.rs` | full | getConfigDesc for onboarding PR |
| `workers/repository/onboarding/pr/index.ts` | — | not-started | |
| `workers/repository/onboarding/pr/pr-list.ts` | — | not-started | |

### workers/repository/process

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `workers/repository/process/extract-update.ts` | — | not-started | |
| `workers/repository/process/fetch.ts` | — | not-started | |
| `workers/repository/process/fingerprint-fields.ts` | — | not-started | |
| `workers/repository/process/index.ts` | — | not-started | |
| `workers/repository/process/libyear.ts` | — | not-started | |
| `workers/repository/process/limits.ts` | `renovate-core/src/limits.rs` | partial | Limit checking; pr/hour and pr/branch limits |
| `workers/repository/process/sort.ts` | — | not-started | |
| `workers/repository/process/types.ts` | — | not-started | |
| `workers/repository/process/vulnerabilities.ts` | — | not-started | |
| `workers/repository/process/write.ts` | — | not-started | |

### workers/repository/process/lookup

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `workers/repository/process/lookup/abandonment.ts` | — | not-started | |
| `workers/repository/process/lookup/bucket.ts` | — | not-started | |
| `workers/repository/process/lookup/current.ts` | — | not-started | |
| `workers/repository/process/lookup/filter-checks.ts` | — | not-started | |
| `workers/repository/process/lookup/filter.ts` | — | not-started | |
| `workers/repository/process/lookup/generate.ts` | — | not-started | |
| `workers/repository/process/lookup/index.ts` | — | not-started | |
| `workers/repository/process/lookup/rollback.ts` | — | not-started | |
| `workers/repository/process/lookup/timestamps.ts` | — | not-started | |
| `workers/repository/process/lookup/types.ts` | — | not-started | |
| `workers/repository/process/lookup/update-type.ts` | — | not-started | |
| `workers/repository/process/lookup/utils.ts` | — | not-started | |

### workers/repository/reconfigure

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `workers/repository/reconfigure/comment.ts` | — | not-started | |
| `workers/repository/reconfigure/index.ts` | — | not-started | |
| `workers/repository/reconfigure/reconfigure-cache.ts` | — | not-started | |
| `workers/repository/reconfigure/utils.ts` | — | not-started | |
| `workers/repository/reconfigure/validate.ts` | — | not-started | |

### workers/repository/update/branch

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `workers/repository/update/branch/artifacts.ts` | — | not-started | |
| `workers/repository/update/branch/auto-replace.ts` | — | not-started | |
| `workers/repository/update/branch/automerge.ts` | — | not-started | |
| `workers/repository/update/branch/bump-versions.ts` | — | not-started | |
| `workers/repository/update/branch/check-existing.ts` | — | not-started | |
| `workers/repository/update/branch/commit.ts` | — | not-started | |
| `workers/repository/update/branch/execute-post-upgrade-commands.ts` | — | not-started | |
| `workers/repository/update/branch/get-updated.ts` | — | not-started | |
| `workers/repository/update/branch/handle-existing.ts` | — | not-started | |
| `workers/repository/update/branch/index.ts` | — | not-started | |
| `workers/repository/update/branch/reuse.ts` | — | not-started | |
| `workers/repository/update/branch/schedule.ts` | `renovate-core/src/schedule.rs` | partial | Schedule checking logic |
| `workers/repository/update/branch/status-checks.ts` | — | not-started | |

### workers/repository/update/pr

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `workers/repository/update/pr/automerge.ts` | — | not-started | |
| `workers/repository/update/pr/body/changelogs.ts` | — | not-started | |
| `workers/repository/update/pr/body/config-description.ts` | — | not-started | |
| `workers/repository/update/pr/body/controls.ts` | — | not-started | |
| `workers/repository/update/pr/body/footer.ts` | — | not-started | |
| `workers/repository/update/pr/body/header.ts` | — | not-started | |
| `workers/repository/update/pr/body/index.ts` | — | not-started | |
| `workers/repository/update/pr/body/notes.ts` | — | not-started | |
| `workers/repository/update/pr/body/updates-table.ts` | — | not-started | |
| `workers/repository/update/pr/changelog/api.ts` | — | not-started | |
| `workers/repository/update/pr/changelog/bitbucket-server/index.ts` | — | not-started | |
| `workers/repository/update/pr/changelog/bitbucket-server/source.ts` | — | not-started | |
| `workers/repository/update/pr/changelog/bitbucket/index.ts` | — | not-started | |
| `workers/repository/update/pr/changelog/bitbucket/source.ts` | — | not-started | |
| `workers/repository/update/pr/changelog/common.ts` | — | not-started | |
| `workers/repository/update/pr/changelog/forgejo/index.ts` | — | not-started | |
| `workers/repository/update/pr/changelog/forgejo/source.ts` | — | not-started | |
| `workers/repository/update/pr/changelog/gitea/index.ts` | — | not-started | |
| `workers/repository/update/pr/changelog/gitea/source.ts` | — | not-started | |
| `workers/repository/update/pr/changelog/github/index.ts` | — | not-started | |
| `workers/repository/update/pr/changelog/github/source.ts` | — | not-started | |
| `workers/repository/update/pr/changelog/gitlab/index.ts` | — | not-started | |
| `workers/repository/update/pr/changelog/gitlab/source.ts` | — | not-started | |
| `workers/repository/update/pr/changelog/hbs-template.ts` | — | not-started | |
| `workers/repository/update/pr/changelog/index.ts` | — | not-started | |
| `workers/repository/update/pr/changelog/release-notes.ts` | — | not-started | |
| `workers/repository/update/pr/changelog/releases.ts` | — | not-started | |
| `workers/repository/update/pr/changelog/source.ts` | — | not-started | |
| `workers/repository/update/pr/changelog/types.ts` | — | not-started | |
| `workers/repository/update/pr/code-owners.ts` | — | not-started | |
| `workers/repository/update/pr/index.ts` | — | not-started | |
| `workers/repository/update/pr/labels.ts` | — | not-started | |
| `workers/repository/update/pr/participants.ts` | — | not-started | |
| `workers/repository/update/pr/pr-cache.ts` | — | not-started | |
| `workers/repository/update/pr/pr-fingerprint.ts` | — | not-started | |
| `workers/repository/update/pr/pr-reuse.ts` | — | not-started | |

### workers/repository/updates

| Renovate source file | Rust file | Status | Notes |
|---|---|---|---|
| `workers/repository/updates/branch-name.ts` | `renovate-core/src/branch.rs` | full | Branch name generation, sanitizeDepName |
| `workers/repository/updates/branchify.ts` | — | not-started | Branch grouping |
| `workers/repository/updates/flatten.ts` | `renovate-core/src/branch.rs` | partial | Sanitize dep name; flatten logic not done |
| `workers/repository/updates/generate.ts` | — | not-started | Update generation |

### Workers summary

| Status | Count |
|---|---|
| full | 7 |
| partial | 7 |
| not-started | 127 |
| out-of-scope | 3 |

---

## Other (`lib/types/`, `lib/logger/`, `lib/instrumentation/`, `lib/constants/`, root files)


---

## Summary Statistics

| Area | TS files | full | partial | not-started | out-of-scope |
|---|---|---|---|---|---|
| Managers | ~500 | ~280 (56%) | ~140 (28%) | ~55 (11%) | ~25 (5%) |
| Datasources | ~120 | ~80 (67%) | ~5 (4%) | ~7 (6%) | ~0 |
| Versioning | ~54 | ~30 (56%) | ~18 (33%) | ~0 | ~6 (11%) |
| Platform | ~75 | ~10 (13%) | ~12 (16%) | ~40 (53%) | ~0 |
| Config | ~117 | ~9 (8%) | ~10 (9%) | ~65 (56%) | ~33 (28%) |
| Util | ~177 | ~21 (12%) | ~40 (23%) | ~95 (54%) | ~21 (12%) |
| Workers | ~151 | ~7 (5%) | ~30 (20%) | ~106 (70%) | ~8 (5%) |
| Other | ~41 | ~2 (5%) | ~15 (37%) | ~16 (39%) | ~8 (20%) |
| **Total** | **~1469** | **562 (38%)** | **303 (21%)** | **496 (34%)** | **108 (7%)** |

### Key Findings

1. **Manager extraction is strong** — 84% of manager source files are `full` or `partial`. Almost every manager's core `extract.ts` has a complete Rust equivalent.

2. **Datasources are well-covered** — 50% full + 4% partial. The majority of datasource fetching logic is ported.

3. **Versioning is comprehensive** — All 54 versioning schemes have Rust implementations. 56% are full parity.

4. **Artifact execution is the biggest gap** — ~35 `artifacts.ts` files are not-started across managers (shell execution, file I/O, temp dirs).

5. **Workers/pipeline are mostly not-started** — 70% of worker files have no Rust equivalent. The CLI handles basic repo processing but lacks full branch lifecycle, changelog rendering, dependency dashboard, etc.

6. **Config migrations are not-started** — 65+ individual migration files have no Rust equivalent. Core config types/massage/secrets are ported.

7. **Platform coverage is minimal** — Only utility functions and GitHub/GitLab basic auth are ported. Full platform API clients (initRepo, PR lifecycle, issues, comments) are not implemented for any platform.
