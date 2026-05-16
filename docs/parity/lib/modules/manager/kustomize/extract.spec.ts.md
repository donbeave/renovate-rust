# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

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

