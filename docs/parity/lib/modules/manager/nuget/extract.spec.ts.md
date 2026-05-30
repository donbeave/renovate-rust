# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/nuget/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/nuget/extract.spec.ts
**Total tests:** 35 | **Ported:** 35 | **Actionable:** 0 | **Status:** done

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

