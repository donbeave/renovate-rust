# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

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

