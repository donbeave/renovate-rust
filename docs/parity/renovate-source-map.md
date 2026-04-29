# Renovate Source Map

Maps Renovate TypeScript **source** files to their Rust counterparts.
This file tracks source-level port coverage. Only `.ts` source files appear here (never `.spec.ts`).

**Status:** `full` · `partial` · `stub` · `not-started` · `out-of-scope`

---

## Managers (`lib/modules/manager/`)

### ansible

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/ansible/extract.ts` | `crates/renovate-core/src/extractors/ansible.rs` | partial | Core extraction ported; no artifacts |
| `lib/modules/manager/ansible/index.ts` | `crates/renovate-core/src/extractors/ansible.rs` | partial | Manager metadata in extractor |

### ansible-galaxy

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/ansible-galaxy/extract.ts` | `crates/renovate-core/src/extractors/ansible_galaxy.rs` | partial | Collections + roles extraction ported |
| `lib/modules/manager/ansible-galaxy/collections.ts` | `crates/renovate-core/src/extractors/ansible_galaxy.rs` | partial | Inlined into extractor |
| `lib/modules/manager/ansible-galaxy/roles.ts` | `crates/renovate-core/src/extractors/ansible_galaxy.rs` | partial | Inlined into extractor |
| `lib/modules/manager/ansible-galaxy/index.ts` | `crates/renovate-core/src/extractors/ansible_galaxy.rs` | partial | Manager metadata |
| `lib/modules/manager/ansible-galaxy/collections-metadata.ts` | — | not-started | Metadata helpers not ported |
| `lib/modules/manager/ansible-galaxy/dep-types.ts` | — | not-started | Type definitions only |
| `lib/modules/manager/ansible-galaxy/types.ts` | — | not-started | Type definitions only |
| `lib/modules/manager/ansible-galaxy/util.ts` | — | not-started | Utility helpers |

### ant

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/ant/extract.ts` | `crates/renovate-core/src/extractors/ant.rs` | partial | Core extraction ported |
| `lib/modules/manager/ant/properties.ts` | `crates/renovate-core/src/extractors/ant.rs` | partial | Inlined |
| `lib/modules/manager/ant/update.ts` | — | not-started | Update logic not ported |
| `lib/modules/manager/ant/index.ts` | `crates/renovate-core/src/extractors/ant.rs` | partial | Manager metadata |
| `lib/modules/manager/ant/types.ts` | — | not-started | Type definitions only |

### argocd

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/argocd/extract.ts` | `crates/renovate-core/src/extractors/argocd.rs` | partial | Core extraction ported |
| `lib/modules/manager/argocd/index.ts` | `crates/renovate-core/src/extractors/argocd.rs` | partial | Manager metadata |
| `lib/modules/manager/argocd/schema.ts` | `crates/renovate-core/src/extractors/argocd.rs` | partial | Schema inlined |
| `lib/modules/manager/argocd/util.ts` | `crates/renovate-core/src/extractors/argocd.rs` | partial | Utilities inlined |

### asdf

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/asdf/extract.ts` | `crates/renovate-core/src/extractors/asdf.rs` | partial | Core extraction ported; large tooling map |
| `lib/modules/manager/asdf/upgradeable-tooling.ts` | `crates/renovate-core/src/extractors/asdf.rs` | partial | Tooling map partially inlined |
| `lib/modules/manager/asdf/index.ts` | `crates/renovate-core/src/extractors/asdf.rs` | partial | Manager metadata |

### azure-pipelines

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/azure-pipelines/extract.ts` | `crates/renovate-core/src/extractors/azure_pipelines.rs` | partial | Core extraction ported |
| `lib/modules/manager/azure-pipelines/index.ts` | `crates/renovate-core/src/extractors/azure_pipelines.rs` | partial | Manager metadata |
| `lib/modules/manager/azure-pipelines/schema.ts` | `crates/renovate-core/src/extractors/azure_pipelines.rs` | partial | Schema inlined |
| `lib/modules/manager/azure-pipelines/dep-types.ts` | — | not-started | Type definitions only |

### batect

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/batect/extract.ts` | `crates/renovate-core/src/extractors/batect.rs` | partial | Core extraction ported |
| `lib/modules/manager/batect/index.ts` | `crates/renovate-core/src/extractors/batect.rs` | partial | Manager metadata |
| `lib/modules/manager/batect/schema.ts` | `crates/renovate-core/src/extractors/batect.rs` | partial | Schema inlined |
| `lib/modules/manager/batect/types.ts` | — | not-started | Type definitions only |

### batect-wrapper

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/batect-wrapper/extract.ts` | `crates/renovate-core/src/extractors/batect_wrapper.rs` | partial | Core extraction ported |
| `lib/modules/manager/batect-wrapper/artifacts.ts` | — | not-started | Artifacts not ported |
| `lib/modules/manager/batect-wrapper/index.ts` | `crates/renovate-core/src/extractors/batect_wrapper.rs` | partial | Manager metadata |

### bazel

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/bazel/extract.ts` | `crates/renovate-core/src/extractors/bazel.rs` | partial | Core extraction ported |
| `lib/modules/manager/bazel/parser.ts` | `crates/renovate-core/src/extractors/bazel.rs` | partial | Parser inlined |
| `lib/modules/manager/bazel/common.ts` | `crates/renovate-core/src/extractors/bazel.rs` | partial | Inlined |
| `lib/modules/manager/bazel/rules/docker.ts` | `crates/renovate-core/src/extractors/bazel.rs` | partial | Inlined |
| `lib/modules/manager/bazel/rules/git.ts` | `crates/renovate-core/src/extractors/bazel.rs` | partial | Inlined |
| `lib/modules/manager/bazel/rules/go.ts` | `crates/renovate-core/src/extractors/bazel.rs` | partial | Inlined |
| `lib/modules/manager/bazel/rules/http.ts` | `crates/renovate-core/src/extractors/bazel.rs` | partial | Inlined |
| `lib/modules/manager/bazel/rules/maven.ts` | `crates/renovate-core/src/extractors/bazel.rs` | partial | Inlined |
| `lib/modules/manager/bazel/rules/oci.ts` | `crates/renovate-core/src/extractors/bazel.rs` | partial | Inlined |
| `lib/modules/manager/bazel/rules/index.ts` | `crates/renovate-core/src/extractors/bazel.rs` | partial | Inlined |
| `lib/modules/manager/bazel/artifacts.ts` | — | not-started | Artifacts not ported |
| `lib/modules/manager/bazel/index.ts` | `crates/renovate-core/src/extractors/bazel.rs` | partial | Manager metadata |
| `lib/modules/manager/bazel/dep-types.ts` | — | not-started | Type definitions only |
| `lib/modules/manager/bazel/types.ts` | — | not-started | Type definitions only |

### bazel-module

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/bazel-module/extract.ts` | `crates/renovate-core/src/extractors/bazel_module.rs` | partial | Core extraction ported |
| `lib/modules/manager/bazel-module/parser/index.ts` | `crates/renovate-core/src/extractors/bazel_module.rs` | partial | Parser inlined |
| `lib/modules/manager/bazel-module/parser/common.ts` | `crates/renovate-core/src/extractors/bazel_module.rs` | partial | Inlined |
| `lib/modules/manager/bazel-module/parser/context.ts` | `crates/renovate-core/src/extractors/bazel_module.rs` | partial | Inlined |
| `lib/modules/manager/bazel-module/parser/crate.ts` | `crates/renovate-core/src/extractors/bazel_module.rs` | partial | Inlined |
| `lib/modules/manager/bazel-module/parser/extension-tags.ts` | `crates/renovate-core/src/extractors/bazel_module.rs` | partial | Inlined |
| `lib/modules/manager/bazel-module/parser/fragments.ts` | `crates/renovate-core/src/extractors/bazel_module.rs` | partial | Inlined |
| `lib/modules/manager/bazel-module/parser/maven.ts` | `crates/renovate-core/src/extractors/bazel_module.rs` | partial | Inlined |
| `lib/modules/manager/bazel-module/parser/oci.ts` | `crates/renovate-core/src/extractors/bazel_module.rs` | partial | Inlined |
| `lib/modules/manager/bazel-module/parser/repo-rules.ts` | `crates/renovate-core/src/extractors/bazel_module.rs` | partial | Inlined |
| `lib/modules/manager/bazel-module/parser/rules.ts` | `crates/renovate-core/src/extractors/bazel_module.rs` | partial | Inlined |
| `lib/modules/manager/bazel-module/parser/starlark.ts` | `crates/renovate-core/src/extractors/bazel_module.rs` | partial | Inlined |
| `lib/modules/manager/bazel-module/artifacts.ts` | — | not-started | Artifacts not ported |
| `lib/modules/manager/bazel-module/bazelrc.ts` | — | not-started | Not ported |
| `lib/modules/manager/bazel-module/lockfile.ts` | — | not-started | Lockfile not ported |
| `lib/modules/manager/bazel-module/rules-img.ts` | — | not-started | Not ported |
| `lib/modules/manager/bazel-module/rules.ts` | — | not-started | Not ported |
| `lib/modules/manager/bazel-module/dep-types.ts` | — | not-started | Type definitions only |
| `lib/modules/manager/bazel-module/index.ts` | `crates/renovate-core/src/extractors/bazel_module.rs` | partial | Manager metadata |

### bazelisk

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/bazelisk/extract.ts` | — | not-started | No Rust equivalent |
| `lib/modules/manager/bazelisk/artifacts.ts` | — | not-started | Not ported |
| `lib/modules/manager/bazelisk/index.ts` | — | not-started | Not ported |

### bicep

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/bicep/extract.ts` | `crates/renovate-core/src/extractors/bicep.rs` | partial | Core extraction ported |
| `lib/modules/manager/bicep/index.ts` | `crates/renovate-core/src/extractors/bicep.rs` | partial | Manager metadata |

### bitbucket-pipelines

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/bitbucket-pipelines/extract.ts` | `crates/renovate-core/src/extractors/bitbucket_pipelines.rs` | partial | Core extraction ported |
| `lib/modules/manager/bitbucket-pipelines/index.ts` | `crates/renovate-core/src/extractors/bitbucket_pipelines.rs` | partial | Manager metadata |
| `lib/modules/manager/bitbucket-pipelines/dep-types.ts` | — | not-started | Type definitions only |
| `lib/modules/manager/bitbucket-pipelines/util.ts` | — | not-started | Utility helpers |

### bitrise

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/bitrise/extract.ts` | `crates/renovate-core/src/extractors/bitrise.rs` | partial | Core extraction ported |
| `lib/modules/manager/bitrise/index.ts` | `crates/renovate-core/src/extractors/bitrise.rs` | partial | Manager metadata |
| `lib/modules/manager/bitrise/schema.ts` | `crates/renovate-core/src/extractors/bitrise.rs` | partial | Schema inlined |
| `lib/modules/manager/bitrise/utils.ts` | `crates/renovate-core/src/extractors/bitrise.rs` | partial | Inlined |

### buildkite

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/buildkite/extract.ts` | `crates/renovate-core/src/extractors/buildkite.rs` | partial | Core extraction ported |
| `lib/modules/manager/buildkite/index.ts` | `crates/renovate-core/src/extractors/buildkite.rs` | partial | Manager metadata |

### buildpacks

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/buildpacks/extract.ts` | `crates/renovate-core/src/extractors/buildpacks.rs` | partial | Core extraction ported |
| `lib/modules/manager/buildpacks/index.ts` | `crates/renovate-core/src/extractors/buildpacks.rs` | partial | Manager metadata |
| `lib/modules/manager/buildpacks/schema.ts` | `crates/renovate-core/src/extractors/buildpacks.rs` | partial | Schema inlined |

### bun

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/bun/extract.ts` | — | not-started | No Rust equivalent |
| `lib/modules/manager/bun/artifacts.ts` | — | not-started | Not ported |
| `lib/modules/manager/bun/index.ts` | — | not-started | Not ported |
| `lib/modules/manager/bun/utils.ts` | — | not-started | Not ported |

### bun-version

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/bun-version/index.ts` | — | not-started | No Rust equivalent |

### bundler

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/bundler/extract.ts` | `crates/renovate-core/src/extractors/bundler.rs` | partial | Core extraction ported |
| `lib/modules/manager/bundler/common.ts` | `crates/renovate-core/src/extractors/bundler.rs` | partial | Inlined |
| `lib/modules/manager/bundler/index.ts` | `crates/renovate-core/src/extractors/bundler.rs` | partial | Manager metadata |
| `lib/modules/manager/bundler/artifacts.ts` | — | not-started | Artifacts not ported |
| `lib/modules/manager/bundler/host-rules.ts` | — | not-started | Not ported |
| `lib/modules/manager/bundler/locked-version.ts` | — | not-started | Not ported |
| `lib/modules/manager/bundler/update-locked.ts` | — | not-started | Not ported |

### cake

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/cake/index.ts` | `crates/renovate-core/src/extractors/cake.rs` | partial | Manager metadata; no dedicated extract.ts in TS |

### cargo

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/cargo/extract.ts` | `crates/renovate-core/src/extractors/cargo.rs` | partial | Core extraction ported |
| `lib/modules/manager/cargo/index.ts` | `crates/renovate-core/src/extractors/cargo.rs` | partial | Manager metadata |
| `lib/modules/manager/cargo/schema.ts` | `crates/renovate-core/src/extractors/cargo.rs` | partial | Schema inlined |
| `lib/modules/manager/cargo/utils.ts` | `crates/renovate-core/src/extractors/cargo.rs` | partial | Inlined |
| `lib/modules/manager/cargo/range.ts` | `crates/renovate-core/src/versioning/cargo.rs` | partial | Range logic in versioning crate |
| `lib/modules/manager/cargo/artifacts.ts` | — | not-started | Artifacts not ported |
| `lib/modules/manager/cargo/locked-version.ts` | — | not-started | Not ported |
| `lib/modules/manager/cargo/update-locked.ts` | — | not-started | Not ported |
| `lib/modules/manager/cargo/update.ts` | — | not-started | Not ported |
| `lib/modules/manager/cargo/types.ts` | — | not-started | Type definitions only |

### cdnurl

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/cdnurl/extract.ts` | — | not-started | No Rust equivalent |
| `lib/modules/manager/cdnurl/index.ts` | — | not-started | Not ported |

### circleci

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/circleci/extract.ts` | `crates/renovate-core/src/extractors/circleci.rs` | partial | Core extraction ported |
| `lib/modules/manager/circleci/index.ts` | `crates/renovate-core/src/extractors/circleci.rs` | partial | Manager metadata |
| `lib/modules/manager/circleci/schema.ts` | `crates/renovate-core/src/extractors/circleci.rs` | partial | Schema inlined |
| `lib/modules/manager/circleci/dep-types.ts` | — | not-started | Type definitions only |
| `lib/modules/manager/circleci/range.ts` | — | not-started | Not ported |

### cloudbuild

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/cloudbuild/extract.ts` | `crates/renovate-core/src/extractors/cloudbuild.rs` | partial | Core extraction ported |
| `lib/modules/manager/cloudbuild/index.ts` | `crates/renovate-core/src/extractors/cloudbuild.rs` | partial | Manager metadata |
| `lib/modules/manager/cloudbuild/schema.ts` | `crates/renovate-core/src/extractors/cloudbuild.rs` | partial | Schema inlined |

### cocoapods

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/cocoapods/extract.ts` | `crates/renovate-core/src/extractors/cocoapods.rs` | partial | Core extraction ported |
| `lib/modules/manager/cocoapods/index.ts` | `crates/renovate-core/src/extractors/cocoapods.rs` | partial | Manager metadata |
| `lib/modules/manager/cocoapods/artifacts.ts` | — | not-started | Artifacts not ported |
| `lib/modules/manager/cocoapods/types.ts` | — | not-started | Type definitions only |

### composer

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/composer/extract.ts` | `crates/renovate-core/src/extractors/composer.rs` | partial | Core extraction ported |
| `lib/modules/manager/composer/index.ts` | `crates/renovate-core/src/extractors/composer.rs` | partial | Manager metadata |
| `lib/modules/manager/composer/schema.ts` | `crates/renovate-core/src/extractors/composer.rs` | partial | Schema inlined |
| `lib/modules/manager/composer/utils.ts` | `crates/renovate-core/src/extractors/composer.rs` | partial | Inlined |
| `lib/modules/manager/composer/artifacts.ts` | — | not-started | Artifacts not ported |
| `lib/modules/manager/composer/range.ts` | — | not-started | Not ported |
| `lib/modules/manager/composer/update-locked.ts` | — | not-started | Not ported |
| `lib/modules/manager/composer/dep-types.ts` | — | not-started | Type definitions only |
| `lib/modules/manager/composer/types.ts` | — | not-started | Type definitions only |

### conan

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/conan/extract.ts` | `crates/renovate-core/src/extractors/conan.rs` | partial | Core extraction ported |
| `lib/modules/manager/conan/common.ts` | `crates/renovate-core/src/extractors/conan.rs` | partial | Inlined |
| `lib/modules/manager/conan/index.ts` | `crates/renovate-core/src/extractors/conan.rs` | partial | Manager metadata |
| `lib/modules/manager/conan/range.ts` | — | not-started | Not ported |
| `lib/modules/manager/conan/artifacts.ts` | — | not-started | Artifacts not ported |
| `lib/modules/manager/conan/dep-types.ts` | — | not-started | Type definitions only |

### copier

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/copier/extract.ts` | `crates/renovate-core/src/extractors/copier.rs` | partial | Core extraction ported |
| `lib/modules/manager/copier/index.ts` | `crates/renovate-core/src/extractors/copier.rs` | partial | Manager metadata |
| `lib/modules/manager/copier/schema.ts` | `crates/renovate-core/src/extractors/copier.rs` | partial | Schema inlined |
| `lib/modules/manager/copier/artifacts.ts` | — | not-started | Artifacts not ported |
| `lib/modules/manager/copier/update.ts` | — | not-started | Not ported |
| `lib/modules/manager/copier/utils.ts` | — | not-started | Not ported |
| `lib/modules/manager/copier/dep-types.ts` | — | not-started | Type definitions only |

### cpanfile

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/cpanfile/extract.ts` | `crates/renovate-core/src/extractors/cpanfile.rs` | partial | Core extraction ported |
| `lib/modules/manager/cpanfile/parser.ts` | `crates/renovate-core/src/extractors/cpanfile.rs` | partial | Inlined |
| `lib/modules/manager/cpanfile/index.ts` | `crates/renovate-core/src/extractors/cpanfile.rs` | partial | Manager metadata |
| `lib/modules/manager/cpanfile/language.ts` | — | not-started | Not ported |
| `lib/modules/manager/cpanfile/dep-types.ts` | — | not-started | Type definitions only |

### crossplane

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/crossplane/extract.ts` | `crates/renovate-core/src/extractors/crossplane.rs` | partial | Core extraction ported |
| `lib/modules/manager/crossplane/index.ts` | `crates/renovate-core/src/extractors/crossplane.rs` | partial | Manager metadata |
| `lib/modules/manager/crossplane/schema.ts` | `crates/renovate-core/src/extractors/crossplane.rs` | partial | Schema inlined |
| `lib/modules/manager/crossplane/dep-types.ts` | — | not-started | Type definitions only |

### crow

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/crow/extract.ts` | `crates/renovate-core/src/extractors/crow.rs` | partial | Core extraction ported |
| `lib/modules/manager/crow/index.ts` | `crates/renovate-core/src/extractors/crow.rs` | partial | Manager metadata |
| `lib/modules/manager/crow/schema.ts` | `crates/renovate-core/src/extractors/crow.rs` | partial | Schema inlined |

### custom

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/custom/regex/index.ts` | — | not-started | Custom regex manager not ported |
| `lib/modules/manager/custom/regex/strategies.ts` | — | not-started | Not ported |
| `lib/modules/manager/custom/regex/utils.ts` | — | not-started | Not ported |
| `lib/modules/manager/custom/jsonata/index.ts` | — | not-started | JSONata manager not ported |
| `lib/modules/manager/custom/jsonata/utils.ts` | — | not-started | Not ported |
| `lib/modules/manager/custom/index.ts` | — | not-started | Not ported |
| `lib/modules/manager/custom/api.ts` | — | not-started | Not ported |
| `lib/modules/manager/custom/utils.ts` | — | not-started | Not ported |

### deno

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/deno/extract.ts` | — | not-started | No Rust equivalent |
| `lib/modules/manager/deno/index.ts` | — | not-started | Not ported |
| `lib/modules/manager/deno/artifacts.ts` | — | not-started | Not ported |
| `lib/modules/manager/deno/update.ts` | — | not-started | Not ported |

### deps-edn

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/deps-edn/extract.ts` | `crates/renovate-core/src/extractors/deps_edn.rs` | partial | Core extraction ported |
| `lib/modules/manager/deps-edn/parser.ts` | `crates/renovate-core/src/extractors/deps_edn.rs` | partial | Inlined |
| `lib/modules/manager/deps-edn/index.ts` | `crates/renovate-core/src/extractors/deps_edn.rs` | partial | Manager metadata |
| `lib/modules/manager/deps-edn/types.ts` | — | not-started | Type definitions only |

### devbox

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/devbox/extract.ts` | `crates/renovate-core/src/extractors/devbox.rs` | partial | Core extraction ported |
| `lib/modules/manager/devbox/index.ts` | `crates/renovate-core/src/extractors/devbox.rs` | partial | Manager metadata |
| `lib/modules/manager/devbox/schema.ts` | `crates/renovate-core/src/extractors/devbox.rs` | partial | Schema inlined |
| `lib/modules/manager/devbox/artifacts.ts` | — | not-started | Artifacts not ported |
| `lib/modules/manager/devbox/tool-versioning.ts` | — | not-started | Not ported |

### devcontainer

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/devcontainer/extract.ts` | `crates/renovate-core/src/extractors/devcontainer.rs` | partial | Core extraction ported |
| `lib/modules/manager/devcontainer/index.ts` | `crates/renovate-core/src/extractors/devcontainer.rs` | partial | Manager metadata |
| `lib/modules/manager/devcontainer/schema.ts` | `crates/renovate-core/src/extractors/devcontainer.rs` | partial | Schema inlined |
| `lib/modules/manager/devcontainer/dep-types.ts` | — | not-started | Type definitions only |

### docker-compose

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/docker-compose/extract.ts` | `crates/renovate-core/src/extractors/docker_compose.rs` | partial | Core extraction ported |
| `lib/modules/manager/docker-compose/index.ts` | `crates/renovate-core/src/extractors/docker_compose.rs` | partial | Manager metadata |
| `lib/modules/manager/docker-compose/schema.ts` | `crates/renovate-core/src/extractors/docker_compose.rs` | partial | Schema inlined |

### dockerfile

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/dockerfile/extract.ts` | `crates/renovate-core/src/extractors/dockerfile.rs` | partial | Core extraction ported |
| `lib/modules/manager/dockerfile/index.ts` | `crates/renovate-core/src/extractors/dockerfile.rs` | partial | Manager metadata |
| `lib/modules/manager/dockerfile/dep-types.ts` | — | not-started | Type definitions only |

### droneci

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/droneci/extract.ts` | `crates/renovate-core/src/extractors/droneci.rs` | partial | Core extraction ported |
| `lib/modules/manager/droneci/index.ts` | `crates/renovate-core/src/extractors/droneci.rs` | partial | Manager metadata |
| `lib/modules/manager/droneci/dep-types.ts` | — | not-started | Type definitions only |

### fleet

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/fleet/extract.ts` | `crates/renovate-core/src/extractors/fleet.rs` | partial | Core extraction ported |
| `lib/modules/manager/fleet/index.ts` | `crates/renovate-core/src/extractors/fleet.rs` | partial | Manager metadata |
| `lib/modules/manager/fleet/dep-types.ts` | — | not-started | Type definitions only |

### flux

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/flux/extract.ts` | `crates/renovate-core/src/extractors/flux.rs` | partial | Core extraction ported |
| `lib/modules/manager/flux/index.ts` | `crates/renovate-core/src/extractors/flux.rs` | partial | Manager metadata |
| `lib/modules/manager/flux/artifacts.ts` | — | not-started | Artifacts not ported |
| `lib/modules/manager/flux/common.ts` | — | not-started | Not ported |
| `lib/modules/manager/flux/schema.ts` | — | not-started | Not ported |

### fvm

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/fvm/extract.ts` | `crates/renovate-core/src/extractors/fvm.rs` | partial | Core extraction ported |
| `lib/modules/manager/fvm/index.ts` | `crates/renovate-core/src/extractors/fvm.rs` | partial | Manager metadata |
| `lib/modules/manager/fvm/schema.ts` | `crates/renovate-core/src/extractors/fvm.rs` | partial | Schema inlined |

### git-submodules

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/git-submodules/extract.ts` | `crates/renovate-core/src/extractors/git_submodules.rs` | partial | Core extraction ported |
| `lib/modules/manager/git-submodules/index.ts` | `crates/renovate-core/src/extractors/git_submodules.rs` | partial | Manager metadata |
| `lib/modules/manager/git-submodules/artifacts.ts` | — | not-started | Artifacts not ported |
| `lib/modules/manager/git-submodules/types.ts` | — | not-started | Type definitions only |

### github-actions

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/github-actions/extract.ts` | `crates/renovate-core/src/extractors/github_actions.rs` | partial | Core extraction ported |
| `lib/modules/manager/github-actions/parse.ts` | `crates/renovate-core/src/extractors/github_actions.rs` | partial | Inlined |
| `lib/modules/manager/github-actions/index.ts` | `crates/renovate-core/src/extractors/github_actions.rs` | partial | Manager metadata |
| `lib/modules/manager/github-actions/schema.ts` | `crates/renovate-core/src/extractors/github_actions.rs` | partial | Schema inlined |
| `lib/modules/manager/github-actions/community.ts` | — | not-started | Not ported |

### gitlabci

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/gitlabci/extract.ts` | `crates/renovate-core/src/extractors/gitlabci.rs` | partial | Core extraction ported |
| `lib/modules/manager/gitlabci/index.ts` | `crates/renovate-core/src/extractors/gitlabci.rs` | partial | Manager metadata |

### gitlabci-include

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/gitlabci-include/extract.ts` | `crates/renovate-core/src/extractors/gitlabci_include.rs` | partial | Core extraction ported |
| `lib/modules/manager/gitlabci-include/index.ts` | `crates/renovate-core/src/extractors/gitlabci_include.rs` | partial | Manager metadata |

### glasskube

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/glasskube/extract.ts` | `crates/renovate-core/src/extractors/glasskube.rs` | partial | Core extraction ported |
| `lib/modules/manager/glasskube/index.ts` | `crates/renovate-core/src/extractors/glasskube.rs` | partial | Manager metadata |
| `lib/modules/manager/glasskube/dep-types.ts` | — | not-started | Type definitions only |

### gleam

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/gleam/extract.ts` | `crates/renovate-core/src/extractors/gleam.rs` | partial | Core extraction ported |
| `lib/modules/manager/gleam/index.ts` | `crates/renovate-core/src/extractors/gleam.rs` | partial | Manager metadata |

### gomod

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/gomod/extract.ts` | `crates/renovate-core/src/extractors/gomod.rs` | partial | Core extraction ported |
| `lib/modules/manager/gomod/index.ts` | `crates/renovate-core/src/extractors/gomod.rs` | partial | Manager metadata |

### gradle

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/gradle/extract.ts` | `crates/renovate-core/src/extractors/gradle.rs` | partial | Core extraction ported |
| `lib/modules/manager/gradle/index.ts` | `crates/renovate-core/src/extractors/gradle.rs` | partial | Manager metadata |

### gradle-wrapper

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/gradle-wrapper/extract.ts` | `crates/renovate-core/src/extractors/gradle_wrapper.rs` | partial | Core extraction ported |
| `lib/modules/manager/gradle-wrapper/index.ts` | `crates/renovate-core/src/extractors/gradle_wrapper.rs` | partial | Manager metadata |

### helm-requirements

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/helm-requirements/extract.ts` | `crates/renovate-core/src/extractors/helm.rs` | partial | Helm requirements extraction in helm.rs |
| `lib/modules/manager/helm-requirements/index.ts` | `crates/renovate-core/src/extractors/helm.rs` | partial | Manager metadata |

### helm-values

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/helm-values/extract.ts` | `crates/renovate-core/src/extractors/helm_values.rs` | partial | Core extraction ported |
| `lib/modules/manager/helm-values/index.ts` | `crates/renovate-core/src/extractors/helm_values.rs` | partial | Manager metadata |

### helmfile

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/helmfile/extract.ts` | `crates/renovate-core/src/extractors/helmfile.rs` | partial | Core extraction ported |
| `lib/modules/manager/helmfile/index.ts` | `crates/renovate-core/src/extractors/helmfile.rs` | partial | Manager metadata |

### helmsman

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/helmsman/extract.ts` | `crates/renovate-core/src/extractors/helmsman.rs` | partial | Core extraction ported |
| `lib/modules/manager/helmsman/index.ts` | `crates/renovate-core/src/extractors/helmsman.rs` | partial | Manager metadata |

### helmv3

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/helmv3/extract.ts` | — | not-started | No Rust equivalent |
| `lib/modules/manager/helmv3/index.ts` | — | not-started | Not ported |

### hermit

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/hermit/extract.ts` | `crates/renovate-core/src/extractors/hermit.rs` | partial | Core extraction ported |
| `lib/modules/manager/hermit/index.ts` | `crates/renovate-core/src/extractors/hermit.rs` | partial | Manager metadata |

### homeassistant-manifest

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/homeassistant-manifest/extract.ts` | `crates/renovate-core/src/extractors/homeassistant.rs` | partial | Core extraction ported |
| `lib/modules/manager/homeassistant-manifest/index.ts` | `crates/renovate-core/src/extractors/homeassistant.rs` | partial | Manager metadata |
| `lib/modules/manager/homeassistant-manifest/schema.ts` | `crates/renovate-core/src/extractors/homeassistant.rs` | partial | Schema inlined |

### homebrew

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/homebrew/extract.ts` | `crates/renovate-core/src/extractors/homebrew.rs` | partial | Core extraction ported |
| `lib/modules/manager/homebrew/index.ts` | `crates/renovate-core/src/extractors/homebrew.rs` | partial | Manager metadata |

### html

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/html/extract.ts` | `crates/renovate-core/src/extractors/html.rs` | partial | Core extraction ported |
| `lib/modules/manager/html/index.ts` | `crates/renovate-core/src/extractors/html.rs` | partial | Manager metadata |

### jenkins

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/jenkins/extract.ts` | `crates/renovate-core/src/extractors/jenkins.rs` | partial | Core extraction ported |
| `lib/modules/manager/jenkins/index.ts` | `crates/renovate-core/src/extractors/jenkins.rs` | partial | Manager metadata |

### jsonnet-bundler

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/jsonnet-bundler/extract.ts` | `crates/renovate-core/src/extractors/jsonnet_bundler.rs` | partial | Core extraction ported |
| `lib/modules/manager/jsonnet-bundler/index.ts` | `crates/renovate-core/src/extractors/jsonnet_bundler.rs` | partial | Manager metadata |

### kotlin-script

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/kotlin-script/extract.ts` | `crates/renovate-core/src/extractors/kotlin_script.rs` | partial | Core extraction ported |
| `lib/modules/manager/kotlin-script/index.ts` | `crates/renovate-core/src/extractors/kotlin_script.rs` | partial | Manager metadata |

### kubernetes

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/kubernetes/extract.ts` | `crates/renovate-core/src/extractors/kubernetes.rs` | partial | Core extraction ported |
| `lib/modules/manager/kubernetes/index.ts` | `crates/renovate-core/src/extractors/kubernetes.rs` | partial | Manager metadata |

### kustomize

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/kustomize/extract.ts` | `crates/renovate-core/src/extractors/kustomize.rs` | partial | Core extraction ported |
| `lib/modules/manager/kustomize/index.ts` | `crates/renovate-core/src/extractors/kustomize.rs` | partial | Manager metadata |

### leiningen

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/leiningen/extract.ts` | `crates/renovate-core/src/extractors/leiningen.rs` | partial | Core extraction ported |
| `lib/modules/manager/leiningen/index.ts` | `crates/renovate-core/src/extractors/leiningen.rs` | partial | Manager metadata |

### maven

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/maven/extract.ts` | `crates/renovate-core/src/extractors/maven.rs` | partial | Core extraction ported |
| `lib/modules/manager/maven/index.ts` | `crates/renovate-core/src/extractors/maven.rs` | partial | Manager metadata |

### maven-wrapper

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/maven-wrapper/extract.ts` | `crates/renovate-core/src/extractors/maven_wrapper.rs` | partial | Core extraction ported |
| `lib/modules/manager/maven-wrapper/index.ts` | `crates/renovate-core/src/extractors/maven_wrapper.rs` | partial | Manager metadata |

### meteor

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/meteor/extract.ts` | `crates/renovate-core/src/extractors/meteor.rs` | partial | Core extraction ported |
| `lib/modules/manager/meteor/index.ts` | `crates/renovate-core/src/extractors/meteor.rs` | partial | Manager metadata |

### mint

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/mint/extract.ts` | `crates/renovate-core/src/extractors/mint.rs` | partial | Core extraction ported |
| `lib/modules/manager/mint/index.ts` | `crates/renovate-core/src/extractors/mint.rs` | partial | Manager metadata |

### mise

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/mise/extract.ts` | `crates/renovate-core/src/extractors/mise.rs` | partial | Core extraction ported |
| `lib/modules/manager/mise/index.ts` | `crates/renovate-core/src/extractors/mise.rs` | partial | Manager metadata |

### mix

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/mix/extract.ts` | `crates/renovate-core/src/extractors/mix.rs` | partial | Core extraction ported |
| `lib/modules/manager/mix/index.ts` | `crates/renovate-core/src/extractors/mix.rs` | partial | Manager metadata |

### nix

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/nix/extract.ts` | `crates/renovate-core/src/extractors/nix.rs` | partial | Core extraction ported |
| `lib/modules/manager/nix/index.ts` | `crates/renovate-core/src/extractors/nix.rs` | partial | Manager metadata |

### npm

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/npm/extract/index.ts` | `crates/renovate-core/src/extractors/npm.rs` | partial | Core extraction ported |
| `lib/modules/manager/npm/index.ts` | `crates/renovate-core/src/extractors/npm.rs` | partial | Manager metadata |
| `lib/modules/manager/npm/npmrc.ts` | — | not-started | npmrc parsing not ported |
| `lib/modules/manager/npm/utils.ts` | — | not-started | Not ported |

### nuget

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/nuget/extract.ts` | `crates/renovate-core/src/extractors/nuget.rs` | partial | Core extraction ported |
| `lib/modules/manager/nuget/extract/global-manifest.ts` | `crates/renovate-core/src/extractors/nuget.rs` | partial | Inlined |
| `lib/modules/manager/nuget/extract/single-csharp-file.ts` | `crates/renovate-core/src/extractors/nuget.rs` | partial | Inlined |
| `lib/modules/manager/nuget/index.ts` | `crates/renovate-core/src/extractors/nuget.rs` | partial | Manager metadata |
| `lib/modules/manager/nuget/schema.ts` | `crates/renovate-core/src/extractors/nuget.rs` | partial | Schema inlined |
| `lib/modules/manager/nuget/artifacts.ts` | — | not-started | Artifacts not ported |
| `lib/modules/manager/nuget/update.ts` | — | not-started | Not ported |
| `lib/modules/manager/nuget/util.ts` | — | not-started | Not ported |

### nvm

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/nvm/extract.ts` | — | not-started | No Rust equivalent |
| `lib/modules/manager/nvm/index.ts` | — | not-started | Not ported |

### ocb

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/ocb/extract.ts` | `crates/renovate-core/src/extractors/ocb.rs` | partial | Core extraction ported |
| `lib/modules/manager/ocb/index.ts` | `crates/renovate-core/src/extractors/ocb.rs` | partial | Manager metadata |
| `lib/modules/manager/ocb/schema.ts` | `crates/renovate-core/src/extractors/ocb.rs` | partial | Schema inlined |
| `lib/modules/manager/ocb/update.ts` | — | not-started | Not ported |

### osgi

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/osgi/extract.ts` | `crates/renovate-core/src/extractors/osgi.rs` | partial | Core extraction ported |
| `lib/modules/manager/osgi/index.ts` | `crates/renovate-core/src/extractors/osgi.rs` | partial | Manager metadata |
| `lib/modules/manager/osgi/types.ts` | — | not-started | Type definitions only |

### pep621

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/pep621/extract.ts` | `crates/renovate-core/src/extractors/pep621.rs` | partial | Core extraction ported |
| `lib/modules/manager/pep621/index.ts` | `crates/renovate-core/src/extractors/pep621.rs` | partial | Manager metadata |
| `lib/modules/manager/pep621/processors/hatch.ts` | `crates/renovate-core/src/extractors/pep621.rs` | partial | Processor inlined |
| `lib/modules/manager/pep621/processors/pdm.ts` | `crates/renovate-core/src/extractors/pep621.rs` | partial | Inlined |
| `lib/modules/manager/pep621/processors/uv.ts` | `crates/renovate-core/src/extractors/pep621.rs` | partial | Inlined |
| `lib/modules/manager/pep621/artifacts.ts` | — | not-started | Artifacts not ported |
| `lib/modules/manager/pep621/update.ts` | — | not-started | Not ported |

### pep723

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/pep723/extract.ts` | `crates/renovate-core/src/extractors/pep723.rs` | partial | Core extraction ported |
| `lib/modules/manager/pep723/index.ts` | `crates/renovate-core/src/extractors/pep723.rs` | partial | Manager metadata |
| `lib/modules/manager/pep723/schema.ts` | `crates/renovate-core/src/extractors/pep723.rs` | partial | Schema inlined |

### pip-compile

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/pip-compile/extract.ts` | — | not-started | No Rust equivalent |
| `lib/modules/manager/pip-compile/index.ts` | — | not-started | Not ported |
| `lib/modules/manager/pip-compile/artifacts.ts` | — | not-started | Not ported |

### pip_requirements

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/pip_requirements/extract.ts` | `crates/renovate-core/src/extractors/pip.rs` | partial | Core extraction ported |
| `lib/modules/manager/pip_requirements/index.ts` | `crates/renovate-core/src/extractors/pip.rs` | partial | Manager metadata |
| `lib/modules/manager/pip_requirements/artifacts.ts` | — | not-started | Artifacts not ported |

### pip_setup

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/pip_setup/extract.ts` | `crates/renovate-core/src/extractors/pip_setup.rs` | partial | Core extraction ported |
| `lib/modules/manager/pip_setup/index.ts` | `crates/renovate-core/src/extractors/pip_setup.rs` | partial | Manager metadata |

### pipenv

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/pipenv/extract.ts` | `crates/renovate-core/src/extractors/pipfile.rs` | partial | Core extraction ported (Pipfile) |
| `lib/modules/manager/pipenv/index.ts` | `crates/renovate-core/src/extractors/pipfile.rs` | partial | Manager metadata |
| `lib/modules/manager/pipenv/artifacts.ts` | — | not-started | Artifacts not ported |

### pixi

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/pixi/extract.ts` | `crates/renovate-core/src/extractors/pixi.rs` | partial | Core extraction ported |
| `lib/modules/manager/pixi/index.ts` | `crates/renovate-core/src/extractors/pixi.rs` | partial | Manager metadata |
| `lib/modules/manager/pixi/schema.ts` | `crates/renovate-core/src/extractors/pixi.rs` | partial | Schema inlined |
| `lib/modules/manager/pixi/artifacts.ts` | — | not-started | Artifacts not ported |

### poetry

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/poetry/extract.ts` | `crates/renovate-core/src/extractors/poetry.rs` | partial | Core extraction ported |
| `lib/modules/manager/poetry/index.ts` | `crates/renovate-core/src/extractors/poetry.rs` | partial | Manager metadata |
| `lib/modules/manager/poetry/schema.ts` | `crates/renovate-core/src/extractors/poetry.rs` | partial | Schema inlined |
| `lib/modules/manager/poetry/artifacts.ts` | — | not-started | Artifacts not ported |
| `lib/modules/manager/poetry/update-locked.ts` | — | not-started | Not ported |

### pre-commit

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/pre-commit/extract.ts` | `crates/renovate-core/src/extractors/pre_commit.rs` | partial | Core extraction ported |
| `lib/modules/manager/pre-commit/parsing.ts` | `crates/renovate-core/src/extractors/pre_commit.rs` | partial | Inlined |
| `lib/modules/manager/pre-commit/index.ts` | `crates/renovate-core/src/extractors/pre_commit.rs` | partial | Manager metadata |

### proto

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/proto/extract.ts` | — | not-started | No Rust equivalent |
| `lib/modules/manager/proto/index.ts` | — | not-started | Not ported |

### pub

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/pub/extract.ts` | `crates/renovate-core/src/extractors/pubspec.rs` | partial | Core extraction ported |
| `lib/modules/manager/pub/index.ts` | `crates/renovate-core/src/extractors/pubspec.rs` | partial | Manager metadata |
| `lib/modules/manager/pub/artifacts.ts` | — | not-started | Artifacts not ported |

### puppet

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/puppet/extract.ts` | `crates/renovate-core/src/extractors/puppet.rs` | partial | Core extraction ported |
| `lib/modules/manager/puppet/puppetfile-parser.ts` | `crates/renovate-core/src/extractors/puppet.rs` | partial | Parser inlined |
| `lib/modules/manager/puppet/common.ts` | `crates/renovate-core/src/extractors/puppet.rs` | partial | Inlined |
| `lib/modules/manager/puppet/index.ts` | `crates/renovate-core/src/extractors/puppet.rs` | partial | Manager metadata |

### pyenv

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/pyenv/extract.ts` | — | not-started | No Rust equivalent |
| `lib/modules/manager/pyenv/index.ts` | — | not-started | Not ported |

### quadlet

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/quadlet/extract.ts` | `crates/renovate-core/src/extractors/quadlet.rs` | partial | Core extraction ported |
| `lib/modules/manager/quadlet/index.ts` | `crates/renovate-core/src/extractors/quadlet.rs` | partial | Manager metadata |
| `lib/modules/manager/quadlet/schema.ts` | `crates/renovate-core/src/extractors/quadlet.rs` | partial | Schema inlined |

### renovate-config

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/renovate-config/extract.ts` | `crates/renovate-core/src/extractors/renovate_config_presets.rs` | partial | Core extraction ported |
| `lib/modules/manager/renovate-config/index.ts` | `crates/renovate-core/src/extractors/renovate_config_presets.rs` | partial | Manager metadata |

### ruby-version

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/ruby-version/extract.ts` | `crates/renovate-core/src/extractors/version_file.rs` | partial | Part of generic version file extractor |
| `lib/modules/manager/ruby-version/index.ts` | `crates/renovate-core/src/extractors/version_file.rs` | partial | Manager metadata |

### runtime-version

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/runtime-version/extract.ts` | `crates/renovate-core/src/extractors/runtime_version.rs` | partial | Core extraction ported |
| `lib/modules/manager/runtime-version/index.ts` | `crates/renovate-core/src/extractors/runtime_version.rs` | partial | Manager metadata |

### sbt

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/sbt/extract.ts` | `crates/renovate-core/src/extractors/sbt.rs` | partial | Core extraction ported |
| `lib/modules/manager/sbt/index.ts` | `crates/renovate-core/src/extractors/sbt.rs` | partial | Manager metadata |
| `lib/modules/manager/sbt/update.ts` | — | not-started | Not ported |
| `lib/modules/manager/sbt/util.ts` | — | not-started | Not ported |

### scalafmt

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/scalafmt/extract.ts` | `crates/renovate-core/src/extractors/scalafmt.rs` | partial | Core extraction ported |
| `lib/modules/manager/scalafmt/index.ts` | `crates/renovate-core/src/extractors/scalafmt.rs` | partial | Manager metadata |

### setup-cfg

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/setup-cfg/extract.ts` | `crates/renovate-core/src/extractors/setup_cfg.rs` | partial | Core extraction ported |
| `lib/modules/manager/setup-cfg/index.ts` | `crates/renovate-core/src/extractors/setup_cfg.rs` | partial | Manager metadata |

### sveltos

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/sveltos/extract.ts` | `crates/renovate-core/src/extractors/sveltos.rs` | partial | Core extraction ported |
| `lib/modules/manager/sveltos/index.ts` | `crates/renovate-core/src/extractors/sveltos.rs` | partial | Manager metadata |
| `lib/modules/manager/sveltos/schema.ts` | `crates/renovate-core/src/extractors/sveltos.rs` | partial | Schema inlined |
| `lib/modules/manager/sveltos/util.ts` | `crates/renovate-core/src/extractors/sveltos.rs` | partial | Inlined |

### tekton

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/tekton/extract.ts` | `crates/renovate-core/src/extractors/tekton.rs` | partial | Core extraction ported |
| `lib/modules/manager/tekton/index.ts` | `crates/renovate-core/src/extractors/tekton.rs` | partial | Manager metadata |

### terraform

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/terraform/extract.ts` | `crates/renovate-core/src/extractors/terraform.rs` | partial | Core extraction ported |
| `lib/modules/manager/terraform/extractors.ts` | `crates/renovate-core/src/extractors/terraform.rs` | partial | Inlined |
| `lib/modules/manager/terraform/index.ts` | `crates/renovate-core/src/extractors/terraform.rs` | partial | Manager metadata |
| `lib/modules/manager/terraform/lockfile/index.ts` | — | not-started | Lockfile not ported |
| `lib/modules/manager/terraform/lockfile/hash.ts` | — | not-started | Not ported |

### terraform-version

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/terraform-version/extract.ts` | — | not-started | No Rust equivalent |
| `lib/modules/manager/terraform-version/index.ts` | — | not-started | Not ported |

### terragrunt

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/terragrunt/extract.ts` | `crates/renovate-core/src/extractors/terragrunt.rs` | partial | Core extraction ported |
| `lib/modules/manager/terragrunt/index.ts` | `crates/renovate-core/src/extractors/terragrunt.rs` | partial | Manager metadata |
| `lib/modules/manager/terragrunt/artifacts.ts` | — | not-started | Artifacts not ported |

### terragrunt-version

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/terragrunt-version/extract.ts` | — | not-started | No Rust equivalent |
| `lib/modules/manager/terragrunt-version/index.ts` | — | not-started | Not ported |

### tflint-plugin

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/tflint-plugin/extract.ts` | `crates/renovate-core/src/extractors/tflint_plugin.rs` | partial | Core extraction ported |
| `lib/modules/manager/tflint-plugin/index.ts` | `crates/renovate-core/src/extractors/tflint_plugin.rs` | partial | Manager metadata |

### travis

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/travis/extract.ts` | `crates/renovate-core/src/extractors/travis.rs` | partial | Core extraction ported |
| `lib/modules/manager/travis/index.ts` | `crates/renovate-core/src/extractors/travis.rs` | partial | Manager metadata |

### typst

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/typst/extract.ts` | `crates/renovate-core/src/extractors/typst.rs` | partial | Core extraction ported |
| `lib/modules/manager/typst/index.ts` | `crates/renovate-core/src/extractors/typst.rs` | partial | Manager metadata |

### unity3d

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/unity3d/extract.ts` | `crates/renovate-core/src/extractors/unity3d.rs` | partial | Core extraction ported |
| `lib/modules/manager/unity3d/index.ts` | `crates/renovate-core/src/extractors/unity3d.rs` | partial | Manager metadata |

### velaci

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/velaci/extract.ts` | `crates/renovate-core/src/extractors/velaci.rs` | partial | Core extraction ported |
| `lib/modules/manager/velaci/index.ts` | `crates/renovate-core/src/extractors/velaci.rs` | partial | Manager metadata |

### vendir

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/vendir/extract.ts` | `crates/renovate-core/src/extractors/vendir.rs` | partial | Core extraction ported |
| `lib/modules/manager/vendir/index.ts` | `crates/renovate-core/src/extractors/vendir.rs` | partial | Manager metadata |
| `lib/modules/manager/vendir/schema.ts` | `crates/renovate-core/src/extractors/vendir.rs` | partial | Schema inlined |
| `lib/modules/manager/vendir/artifacts.ts` | — | not-started | Artifacts not ported |

### woodpecker

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/woodpecker/extract.ts` | `crates/renovate-core/src/extractors/woodpecker.rs` | partial | Core extraction ported |
| `lib/modules/manager/woodpecker/index.ts` | `crates/renovate-core/src/extractors/woodpecker.rs` | partial | Manager metadata |

### xcodegen

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/manager/xcodegen/extract.ts` | `crates/renovate-core/src/extractors/xcodegen.rs` | partial | Core extraction ported |
| `lib/modules/manager/xcodegen/index.ts` | `crates/renovate-core/src/extractors/xcodegen.rs` | partial | Manager metadata |
| `lib/modules/manager/xcodegen/schema.ts` | `crates/renovate-core/src/extractors/xcodegen.rs` | partial | Schema inlined |

---

## Datasources (`lib/modules/datasource/`)

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/modules/datasource/azure-bicep-resource/index.ts` | `crates/renovate-core/src/datasources/azure_bicep.rs` | stub | Stub only |
| `lib/modules/datasource/azure-pipelines-tasks/index.ts` | `crates/renovate-core/src/datasources/azure_pipelines_tasks.rs` | stub | Stub only |
| `lib/modules/datasource/bazel/index.ts` | `crates/renovate-core/src/datasources/bazel.rs` | stub | Stub only |
| `lib/modules/datasource/bitrise/index.ts` | `crates/renovate-core/src/datasources/bitrise.rs` | stub | Stub only |
| `lib/modules/datasource/buildpacks-registry/index.ts` | `crates/renovate-core/src/datasources/buildpacks_registry.rs` | stub | Stub only |
| `lib/modules/datasource/cdnjs/index.ts` | `crates/renovate-core/src/datasources/cdnjs.rs` | stub | Stub only |
| `lib/modules/datasource/pod/index.ts` | `crates/renovate-core/src/datasources/cocoapods.rs` | stub | Stub only |
| `lib/modules/datasource/conan/index.ts` | `crates/renovate-core/src/datasources/conan.rs` | stub | Stub only |
| `lib/modules/datasource/conda/index.ts` | `crates/renovate-core/src/datasources/conda.rs` | stub | Stub only |
| `lib/modules/datasource/cpan/index.ts` | `crates/renovate-core/src/datasources/cpan.rs` | stub | Stub only |
| `lib/modules/datasource/crate/index.ts` | `crates/renovate-core/src/datasources/crates_io.rs` | stub | Stub only |
| `lib/modules/datasource/devbox/index.ts` | `crates/renovate-core/src/datasources/devbox.rs` | stub | Stub only |
| `lib/modules/datasource/docker/index.ts` | `crates/renovate-core/src/datasources/docker_hub.rs` | stub | Stub only |
| `lib/modules/datasource/endoflife-date/index.ts` | `crates/renovate-core/src/datasources/endoflife.rs` | stub | Stub only |
| `lib/modules/datasource/github-releases/index.ts` | `crates/renovate-core/src/datasources/github_releases.rs` | stub | Stub only |
| `lib/modules/datasource/github-runners/index.ts` | `crates/renovate-core/src/datasources/github_runners.rs` | stub | Stub only |
| `lib/modules/datasource/github-tags/index.ts` | `crates/renovate-core/src/datasources/github_tags.rs` | stub | Stub only |
| `lib/modules/datasource/gitlab-tags/index.ts` | `crates/renovate-core/src/datasources/gitlab_tags.rs` | stub | Stub only |
| `lib/modules/datasource/glasskube-packages/index.ts` | `crates/renovate-core/src/datasources/glasskube_packages.rs` | stub | Stub only |
| `lib/modules/datasource/go/index.ts` | `crates/renovate-core/src/datasources/gomod.rs` | stub | Stub only |
| `lib/modules/datasource/gradle-version/index.ts` | `crates/renovate-core/src/datasources/gradle_version.rs` | stub | Stub only |
| `lib/modules/datasource/hackage/index.ts` | `crates/renovate-core/src/datasources/hackage.rs` | stub | Stub only |
| `lib/modules/datasource/helm/index.ts` | `crates/renovate-core/src/datasources/helm.rs` | stub | Stub only |
| `lib/modules/datasource/hermit/index.ts` | `crates/renovate-core/src/datasources/hermit.rs` | stub | Stub only |
| `lib/modules/datasource/hex/index.ts` | `crates/renovate-core/src/datasources/hex.rs` | stub | Stub only |
| `lib/modules/datasource/jenkins-plugins/index.ts` | `crates/renovate-core/src/datasources/jenkins_plugins.rs` | stub | Stub only |
| `lib/modules/datasource/jsr/index.ts` | `crates/renovate-core/src/datasources/jsr.rs` | stub | Stub only |
| `lib/modules/datasource/maven/index.ts` | `crates/renovate-core/src/datasources/maven.rs` | stub | Stub only |
| `lib/modules/datasource/npm/index.ts` | `crates/renovate-core/src/datasources/npm.rs` | stub | Stub only |
| `lib/modules/datasource/nuget/index.ts` | `crates/renovate-core/src/datasources/nuget.rs` | stub | Stub only |
| `lib/modules/datasource/orb/index.ts` | `crates/renovate-core/src/datasources/orb.rs` | stub | Stub only |
| `lib/modules/datasource/packagist/index.ts` | `crates/renovate-core/src/datasources/packagist.rs` | stub | Stub only |
| `lib/modules/datasource/dart/index.ts` | `crates/renovate-core/src/datasources/pub_dev.rs` | stub | Stub only |
| `lib/modules/datasource/puppet-forge/index.ts` | `crates/renovate-core/src/datasources/puppet_forge.rs` | stub | Stub only |
| `lib/modules/datasource/pypi/index.ts` | `crates/renovate-core/src/datasources/pypi.rs` | stub | Stub only |
| `lib/modules/datasource/rubygems/index.ts` | `crates/renovate-core/src/datasources/rubygems.rs` | stub | Stub only |
| `lib/modules/datasource/terraform-provider/index.ts` | `crates/renovate-core/src/datasources/terraform.rs` | stub | Stub only |
| `lib/modules/datasource/typst/index.ts` | `crates/renovate-core/src/datasources/typst.rs` | stub | Stub only |
| `lib/modules/datasource/unity3d/index.ts` | `crates/renovate-core/src/datasources/unity3d.rs` | stub | Stub only |
| `lib/modules/datasource/artifactory/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/aws-eks-addon/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/aws-machine-image/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/aws-rds/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/azure-tags/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/bitbucket-server-tags/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/bitbucket-tags/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/clojure/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/dart-version/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/deb/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/deno/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/dotnet-version/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/elm-package/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/flutter-version/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/git-refs/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/git-tags/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/gitea-releases/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/gitea-tags/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/github-digest/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/github-release-attachments/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/gitlab-packages/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/gitlab-releases/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/golang-version/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/java-version/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/kubernetes-api/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/node-version/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/python-version/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/repology/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/rpm/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/ruby-version/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/rust-version/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/sbt-package/index.ts` | — | not-started | Not ported |
| `lib/modules/datasource/sbt-plugin/index.ts` | — | not-started | Not ported |

---

## Config (`lib/config/`)

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/config/defaults.ts` | `crates/renovate-core/src/config.rs` | partial | Core defaults ported |
| `lib/config/app-strings.ts` | `crates/renovate-core/src/config.rs` | partial | App strings partially ported |
| `lib/config/parse.ts` | `crates/renovate-cli/src/config_builder.rs` | partial | Config parsing in CLI |
| `lib/config/global.ts` | `crates/renovate-core/src/config/run.rs` | partial | Global config partially ported |
| `lib/config/validation.ts` | — | not-started | Not ported |
| `lib/config/migration.ts` | — | not-started | Not ported |
| `lib/config/migrate-validate.ts` | — | not-started | Not ported |
| `lib/config/massage.ts` | — | not-started | Not ported |
| `lib/config/secrets.ts` | — | not-started | Not ported |
| `lib/config/inherit.ts` | — | not-started | Not ported |
| `lib/config/decrypt.ts` | — | out-of-scope | Platform encryption feature |

---

## Workers & Utilities

| Renovate source file | Rust file | Status | Notes |
|----------------------|-----------|--------|-------|
| `lib/workers/repository/init/merge.ts` | `crates/renovate-core/src/repo_config.rs` | partial | Repo config merge partially ported |
| `lib/workers/global/config/parse/cli.ts` | `crates/renovate-cli/src/cli.rs` | partial | CLI arg parsing ported |
| `lib/workers/global/config/parse/env.ts` | `crates/renovate-cli/src/config_builder.rs` | partial | Env config ported |
| `lib/util/string-match.ts` | `crates/renovate-core/src/string_match.rs` | partial | String matching partially ported |
| `lib/util/package-rules/index.ts` | `crates/renovate-core/src/package_rule.rs` | partial | Package rule matching partially ported |
| `lib/util/package-rules/managers.ts` | `crates/renovate-core/src/package_rule.rs` | partial | Inlined |
| `lib/util/package-rules/dep-names.ts` | `crates/renovate-core/src/package_rule.rs` | partial | Inlined |
