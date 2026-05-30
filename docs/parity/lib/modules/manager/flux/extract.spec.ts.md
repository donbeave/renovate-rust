# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/flux/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/flux/extract.spec.ts
**Total tests:** 59 | **Ported:** 59 | **Actionable:** 0 | **Status:** done

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

