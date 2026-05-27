# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/cargo/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/cargo/extract.spec.ts
**Total tests:** 32 | **Ported:** 32 | **Actionable:** 32 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid toml | 46 | ported | `cargo.rs` | `invalid_toml_returns_error` | — |
| returns null for empty dependencies | 52 | ported | `cargo.rs` | `empty_dependencies_section_returns_empty` | — |
| returns null for empty dev-dependencies | 59 | ported | `cargo.rs` | `empty_dev_dependencies_returns_empty` | — |
| returns null for empty custom target | 66 | ported | `cargo.rs` | `empty_custom_target_returns_empty` | — |
| extracts multiple dependencies simple | 73 | ported | `cargo.rs` | `extracts_simple_string_deps` | — |
| extracts multiple dependencies advanced | 79 | ported | `cargo.rs` | `version_constraint_forms_are_preserved` | — |
| handles inline tables | 85 | ported | `cargo.rs` | `handles_inline_tables` | — |
| handles standard tables | 91 | ported | `cargo.rs` | `extracts_table_deps_with_version` | — |
| extracts platform specific dependencies | 97 | ported | `cargo.rs` | `target_cfg_dependencies_extracted` | — |
| extracts registry urls from .cargo/config.toml | 103 | ported | `cargo.rs` | `extracts_registry_urls_from_cargo_config_toml` | — |
| extracts registry urls from .cargo/config (legacy path) | 112 | ported | `cargo.rs` | `extracts_registry_urls_from_cargo_config_legacy` | — |
| extracts overridden registry indexes from .cargo/config.toml | 121 | ported | `cargo.rs` | `extracts_overridden_registry_indexes` | — |
| extracts overridden source registry indexes from .cargo/config.toml | 180 | ported | `cargo.rs` | `extracts_overridden_source_registry_indexes` | — |
| extracts registries overridden to the default | 205 | ported | `cargo.rs` | `extracts_registries_overridden_to_default` | — |
| extracts registries with an empty config.toml | 249 | ported | `cargo.rs` | `extracts_registries_with_empty_config_toml` | — |
| extracts registry urls from environment | 299 | ported | `cargo.rs` | `extracts_registry_urls_from_environment` | — |
| extracts workspace dependencies | 345 | ported | `cargo.rs` | `workspace_dependencies_extracted` | — |
| skips workspace dependency | 390 | ported | `cargo.rs` | `workspace_dep_is_skipped` | — |
| skips unknown registries | 407 | ported | `cargo.rs` | `skips_unknown_registries` | — |
| fails to parse cargo config with invalid TOML | 415 | ported | `cargo.rs` | `fails_to_parse_cargo_config_with_invalid_toml` | — |
| ignore cargo config registries with missing index | 424 | ported | `cargo.rs` | `ignore_cargo_config_registries_with_missing_index` | — |
| ignore cargo config source replaced registries with missing index | 433 | ported | `cargo.rs` | `ignore_cargo_config_source_replaced_registries_with_missing_index` | — |
| ignore cargo config with circular registry source replacements | 481 | ported | `cargo.rs` | `ignore_cargo_config_with_circular_registry_source_replacements` | — |
| extracts original package name of renamed dependencies | 539 | ported | `cargo.rs` | `renamed_dep_extracts_original_package_name` | — |
| extracts locked versions | 549 | ported | `cargo.rs` | `extracts_locked_versions` | — |
| does not extract locked versions for git dependencies | 567 | ported | `cargo.rs` | `does_not_extract_locked_versions_for_git_dependencies` | — |
| extracts locked versions for renamed packages | 585 | ported | `cargo.rs` | `extracts_locked_versions_for_renamed_packages` | — |
| handles missing locked versions | 601 | ported | `cargo.rs` | `handles_missing_locked_versions` | — |
| handles invalid versions in the toml file | 617 | ported | `cargo.rs` | `handles_invalid_versions_in_toml_file` | — |
| handles invalid lock file | 635 | ported | `cargo.rs` | `handles_invalid_lock_file` | — |
| should extract project version | 650 | ported | `cargo.rs` | `extracts_project_version` | — |
| should extract project version from workspace | 664 | ported | `cargo.rs` | `extracts_project_version_from_workspace` | — |

---

