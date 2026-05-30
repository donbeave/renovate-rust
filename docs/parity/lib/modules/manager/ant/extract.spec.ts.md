# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/ant/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/ant/extract.spec.ts
**Total tests:** 49 | **Ported:** 49 | **Actionable:** 0 | **Status:** done

### `extractPackageFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts inline version dependencies from build.xml | 9 | ported | `ant.rs` | `extracts_inline_dependency` | — |
| extracts multiple dependencies | 33 | ported | `ant.rs` | `multiple_deps_extracted` | — |
| defaults depType to compile when no scope is set | 68 | ported | `ant.rs` | `defaults_dep_type_to_compile_without_scope` | — |
| returns null for invalid XML | 90 | ported | `ant.rs` | `invalid_xml_returns_empty` | — |
| returns null for build.xml with no dependencies | 94 | ported | `ant.rs` | `project_without_artifact_dependencies_returns_empty` | — |
| ignores dependency nodes without version | 104 | ported | `ant.rs` | `dependency_without_version_returns_empty` | — |
| extracts dependencies with single-quoted attributes | 119 | ported | `ant.rs` | `single_quoted_attributes_extracted` | — |
| returns null for unreadable build.xml | 135 | ported | `ant.rs` | `extract_all_package_files_ignores_unreadable_build_xml` | — |
| does not revisit the same file | 143 | ported | `ant.rs` | `extract_all_package_files_deduplicates_paths` | — |

### `property resolution`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| resolves inline property references | 167 | ported | `ant.rs` | `resolves_inline_property_references` | — |
| resolves properties from external .properties files | 193 | ported | `ant.rs` | `extract_all_package_files_resolves_external_properties_file` | — |
| implements first-definition-wins for inline properties | 228 | ported | `ant.rs` | `first_inline_property_definition_wins` | — |
| inline properties take precedence over file properties | 254 | ported | `ant.rs` | `extract_all_package_files_inline_properties_override_file_properties` | — |
| skips dependencies with unresolvable property references | 288 | ported | `ant.rs` | `property_ref_skipped` | — |
| detects circular property references | 312 | ported | `ant.rs` | `circular_property_reference_is_skipped` | — |
| resolves chained property references | 338 | ported | `ant.rs` | `resolves_chained_property_references` | — |
| groups multiple dependencies sharing the same property | 368 | ported | `ant.rs` | `resolves_shared_property_for_multiple_dependencies` | — |
| handles properties file in subdirectory | 400 | ported | `ant.rs` | `extract_all_package_files_resolves_subdirectory_properties_file` | — |
| handles unreadable properties file gracefully | 434 | ported | `ant.rs` | `extract_all_package_files_handles_unreadable_properties_file` | — |
| returns deps with mixed inline and property versions | 464 | ported | `ant.rs` | `returns_mixed_inline_and_property_versions` | — |
| ignores dependency without version during property resolution | 495 | ported | `ant.rs` | `ignores_dependency_without_version_during_property_resolution` | — |
| skips partial placeholder in version string | 522 | ported | `ant.rs` | `partial_placeholder_version_is_skipped` | — |

### `edge cases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles unparseable XML returned by readLocalFile | 549 | ported | `ant.rs` | `unparseable_xml_returns_empty` | — |
| handles absolute path in property file reference | 557 | ported | `ant.rs` | `extract_all_package_files_resolves_absolute_properties_file` | — |
| skips duplicate property file references | 591 | ported | `ant.rs` | `extract_all_package_files_deduplicates_properties_file_refs` | — |
| follows import file references | 628 | ported | `ant.rs` | `extract_all_package_files_follows_import_file_refs` | — |
| skips missing import files | 662 | ported | `ant.rs` | `extract_all_package_files_skips_missing_import_files` | — |
| does not loop on self-importing files | 692 | ported | `ant.rs` | `extract_all_package_files_does_not_loop_on_self_imports` | — |
| shares properties across imported files | 722 | ported | `ant.rs` | `extract_all_package_files_shares_properties_with_imported_files` | — |
| extracts dependency from 3-part coords attribute | 760 | ported | `ant.rs` | `extracts_coords_form` | — |
| extracts scope from 4-part coords attribute | 791 | ported | `ant.rs` | `four_part_coords_with_scope_at_end` | — |
| ignores coords with fewer than 3 parts | 821 | ported | `ant.rs` | `coords_with_fewer_than_3_parts_skipped` | — |
| ignores coords with empty groupId | 840 | ported | `ant.rs` | `coords_with_empty_groupid_skipped` | — |
| resolves property references in coords version | 859 | ported | `ant.rs` | `resolves_property_references_in_coords_version` | — |
| marks coords dependency with unresolvable property | 890 | ported | `ant.rs` | `coords_with_unresolvable_property_is_skipped` | — |
| treats last part as version when it is not a known scope | 919 | ported | `ant.rs` | `four_part_coords_last_segment_is_version_when_not_a_scope` | — |
| collects registry URLs from remoteRepository elements | 949 | ported | `ant.rs` | `remote_repository_collected` | — |
| passes registry URLs to coords-style dependencies | 979 | ported | `ant.rs` | `remote_repository_applies_to_coords_dependency` | — |
| collects registry URLs from settingsFile attribute | 1009 | ported | `ant.rs` | `extract_all_package_files_collects_settings_file_registries` | — |
| merges registries from settingsFile and remoteRepository | 1047 | ported | `ant.rs` | `extract_all_package_files_merges_settings_and_remote_repository_registries` | — |
| handles absolute settingsFile path | 1089 | ported | `ant.rs` | `extract_all_package_files_resolves_absolute_settings_file` | — |
| logs debug when settingsFile cannot be read | 1127 | ported | `ant.rs` | `extract_all_package_files_ignores_missing_settings_file` | — |
| does not pass registries to dependencies outside the block | 1155 | ported | `ant.rs` | `remote_repository_registry_is_scoped_to_dependency_block` | — |
| handles chain referencing undefined property | 1191 | ported | `ant.rs` | `chain_referencing_undefined_property_is_skipped` | — |
| skips property file references with unresolved placeholders in path | 1194 | ported | `ant.rs` | `extract_all_package_files_skips_property_file_with_placeholder_in_path` | — |
| skips property file references that resolve outside the repository | 1226 | ported | `ant.rs` | `extract_all_package_files_skips_property_file_outside_repository` | — |
| skips import file references that resolve outside the repository | 1261 | ported | `ant.rs` | `extract_all_package_files_skips_import_file_outside_repository` | — |
| skips settingsFile references that resolve outside the repository | 1296 | ported | `ant.rs` | `extract_all_package_files_skips_settings_file_outside_repository` | — |
| skips import file references with unresolved placeholders in path | 1330 | ported | `ant.rs` | `extract_all_package_files_skips_import_file_with_placeholder_in_path` | — |

---

