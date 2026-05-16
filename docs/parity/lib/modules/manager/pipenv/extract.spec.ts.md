# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pipenv/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pipenv/extract.spec.ts
**Total tests:** 16 | **Ported:** 16 | **Actionable:** 16 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 37 | ported | `pipfile.rs` | `empty_content_returns_no_deps` | — |
| returns null for invalid toml file | 41 | ported | `pipfile.rs` | `invalid_toml_returns_empty` | — |
| extracts dependencies | 45 | ported | `pipfile.rs` | `extracts_string_form` (+ extracts_multi_constraint, extracts_table_form, dev_packages_flagged) | — |
| marks packages with "extras" as skipReason === unspecified-version | 136 | ported | `pipfile.rs` | `packages_with_only_extras_are_skipped` | — |
| extracts multiple dependencies | 142 | ported | `pipfile.rs` | `dev_packages_flagged` (+ extracts_string_form combined coverage) | — |
| ignores git dependencies | 192 | ported | `pipfile.rs` | `git_dependency_in_mixed_list_skipped` (+ git_dep_skipped) | — |
| ignores invalid package names | 202 | ported | `pipfile.rs` | `invalid_package_name_starting_with_underscore_skipped` | — |
| ignores relative path dependencies | 213 | ported | `pipfile.rs` | `relative_path_in_mixed_list_skipped` (+ local_dep_skipped) | — |
| ignores invalid versions | 223 | ported | `pipfile.rs` | `version_with_spaces_skipped` (+ wildcard_skipped, dev_wildcard_skipped) | — |
| extracts all sources | 234 | ported | `pipfile.rs` | `extracts_all_sources` | — |
| extracts example pipfile | 247 | ported | `pipfile.rs` | `extracts_example_pipfile` | — |
| supports custom index | 313 | ported | `pipfile.rs` | `supports_custom_index` | — |
| gets python constraint from python_version | 338 | ported | `pipfile.rs` | `gets_python_constraint_from_python_version` | — |
| gets python constraint from python_full_version | 350 | ported | `pipfile.rs` | `gets_python_constraint_from_python_full_version` | — |
| gets pipenv constraint from packages | 362 | ported | `pipfile.rs` | `gets_pipenv_constraint_from_packages` | — |
| gets pipenv constraint from dev-packages | 372 | ported | `pipfile.rs` | `gets_pipenv_constraint_from_dev_packages` | — |

---

