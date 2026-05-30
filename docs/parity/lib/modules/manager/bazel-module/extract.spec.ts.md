# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bazel-module/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazel-module/extract.spec.ts
**Total tests:** 35 | **Ported:** 35 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if fails to parse | 25 | ported | `bazel_module.rs` | `malformed_content_returns_empty` | — |
| returns null if something throws an error | 33 | ported | `bazel_module.rs` | `unexpected_parser_input_returns_empty` | — |
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

