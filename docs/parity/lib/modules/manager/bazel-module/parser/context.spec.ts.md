# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/bazel-module/parser/context.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazel-module/parser/context.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 0 | **Status:** done

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

