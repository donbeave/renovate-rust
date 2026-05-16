# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gradle/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gradle/utils.spec.ts
**Total tests:** 12 | **Ported:** 12 | **Actionable:** 12 | **Status:** ported

### `versionLikeSubstring`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts the actual version | 23 | ported | `gradle.rs` | `gradle_version_like_substring_valid_versions` | — |
| returns null for invalid inputs | 41 | ported | `gradle.rs` | `gradle_version_like_substring_invalid_inputs` | — |

### `isDependencyString`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 57 | ported | `gradle.rs` | `gradle_is_dependency_string` | it.each; all 20 cases covered |

### `parseDependencyString`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 85 | ported | `gradle.rs` | `gradle_parse_dependency_string` | it.each; 10 cases covered |
| filetype checks | 105 | ported | `gradle.rs` | `gradle_filetype_checks` | — |
| reorderFiles | 120 | ported | `gradle.rs` | `gradle_reorder_files_basic`, `gradle_reorder_files_nested`, `gradle_reorder_files_alphabetical`, `gradle_reorder_files_independent_subfolders`, `gradle_reorder_files_nested_props_and_build` | split into 5 sub-tests |
| getVars | 250 | ported | `gradle.rs` | `gradle_get_vars` | — |

### `updateVars`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| empty registry | 276 | ported | `gradle.rs` | `gradle_update_vars_empty_registry` | — |
| updates the registry | 285 | ported | `gradle.rs` | `gradle_update_vars_merges` | — |

### `updateVarsFromDefaultCatalog`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no default catalog file | 306 | ported | `gradle.rs` | `gradle_update_vars_from_default_catalog_no_catalog` | — |
| adds variables with default "libs" prefix | 317 | ported | `gradle.rs` | `gradle_update_vars_from_default_catalog_default_prefix` | — |
| adds variables with custom libraries extension name | 357 | ported | `gradle.rs` | `gradle_update_vars_from_default_catalog_custom_prefix` | — |

---

