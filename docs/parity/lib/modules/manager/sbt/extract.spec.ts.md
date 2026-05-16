# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/sbt/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/sbt/extract.spec.ts
**Total tests:** 26 | **Ported:** 26 | **Actionable:** 26 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 23 | ported | `sbt.rs` | `empty_returns_empty` (+ build_properties_extraction) | — |
| extracts deps for generic use-cases | 47 | ported | `sbt.rs` | `extracts_scala_style_deps` (+ extracts_java_style_deps, extracts_plugin, comment_line_skipped, dep_name_formats_correctly) | — |
| extracts deps when scala version is defined in a variable | 74 | ported | `sbt.rs` | `package_file_resolves_scala_version_variable_fixture` | — |
| extracts deps when scala version is defined in an object | 99 | ported | `sbt.rs` | `package_file_resolves_object_variables` | — |
| skips deps when dotted symbolds do not resolve to anything | 136 | ported | `sbt.rs` | `package_file_keeps_unresolved_dotted_symbols_without_current_value` | — |
| extracts packageFileVersion when scala version is defined in a variable | 159 | ported | `sbt.rs` | `package_file_resolves_package_file_version_variable` | — |
| extracts typed variables | 170 | ported | `sbt.rs` | `package_file_resolves_typed_variables` | — |
| skips deps when scala version is missing | 185 | ported | `sbt.rs` | `package_file_extracts_deps_when_scala_version_is_missing` | — |
| extract deps from native scala file with variables | 213 | ported | `sbt.rs` | `package_file_extracts_native_scala_file_variables` | — |
| extracts deps when scala version is defined with a trailing comma | 232 | ported | `sbt.rs` | `package_file_resolves_scala_version_with_trailing_comma` | — |
| extracts deps when scala version is defined in a variable with a trailing comma | 253 | ported | `sbt.rs` | `package_file_resolves_variable_scala_version_with_trailing_comma` | — |
| extracts deps when scala version is defined with ThisBuild scope | 275 | ported | `sbt.rs` | `package_file_resolves_thisbuild_scala_version` | — |
| extracts correct scala library when dealing with scala 3 | 294 | ported | `sbt.rs` | `package_file_extracts_scala3_library` | — |
| extracts deps correctly when dealing with scala 3 | 309 | ported | `sbt.rs` | `package_file_resolves_scala3_cross_dependencies` | — |
| extracts deps when scala version is defined in a variable with ThisBuild scope | 329 | ported | `sbt.rs` | `package_file_resolves_thisbuild_variable_scala_version` | — |
| extract deps from native scala file with private variables | 349 | ported | `sbt.rs` | `package_file_extracts_native_scala_private_variables` | — |
| extract deps when they are defined in a new line | 371 | ported | `sbt.rs` | `package_file_extracts_deps_defined_in_named_seq` | — |
| extract deps with comment | 412 | ported | `sbt.rs` | `extracts_dependencies_with_trailing_comments` | — |
| extract addCompilerPlugin | 452 | ported | `sbt.rs` | `extracts_add_compiler_plugin` | — |
| extract sbt version | 469 | ported | `sbt.rs` | `build_properties_extracts_sbt_version` | — |
| extract sbt version if the file contains other properties | 492 | ported | `sbt.rs` | `build_properties_with_other_props_extracts_sbt_version` | — |
| ignores build.properties file if does not contain sbt version | 516 | ported | `sbt.rs` | `build_properties_without_sbt_version_returns_none` | — |
| extracts proxy repositories | 529 | ported | `sbt.rs` | `extract_all_package_files_extracts_proxy_repositories` | — |
| should include default registryUrls if no repositories file is provided | 607 | ported | `sbt.rs` | `extract_all_package_files_uses_default_registry_urls_without_repositories_file` | — |
| should return empty packagefiles is no content is provided | 637 | ported | `sbt.rs` | `extract_all_package_files_empty_content_returns_empty` | — |
| extracts build properties correctly | 643 | ported | `sbt.rs` | `extract_all_package_files_extracts_build_properties` | — |

---

