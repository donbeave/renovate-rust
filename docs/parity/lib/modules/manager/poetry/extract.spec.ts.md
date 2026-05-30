# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/poetry/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/poetry/extract.spec.ts
**Total tests:** 34 | **Ported:** 34 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 43 | ported | `poetry.rs` | `empty_content_returns_empty` | — |
| returns null for parsed file without poetry section | 47 | ported | `poetry.rs` | `no_poetry_section_returns_empty` | — |
| extracts multiple dependencies | 51 | ported | `poetry.rs` | `extracts_string_deps` (+ poetry_fixture_1) | — |
| extracts multiple dependencies (with dep = {version = "1.2.3"} case) | 60 | ported | `poetry.rs` | `extracts_table_deps` | — |
| handles case with no dependencies | 66 | ported | `poetry.rs` | `poetry_section_with_no_deps_returns_empty` | — |
| handles multiple constraint dependencies | 71 | ported | `poetry.rs` | `multiple_constraint_dependency_is_skipped` | — |
| extracts build-system.requires dependencies | 77 | ported | `poetry.rs` | `extracts_build_system_requires` | — |
| can parse TOML v1 heterogeneous arrays | 112 | ported | `poetry.rs` | `toml_v1_heterogeneous_arrays_are_tolerated` | — |
| extracts mixed versioning types | 118 | ported | `poetry.rs` | `name_normalized_per_pep503` | — |
| extracts dependencies from dependency groups | 160 | ported | `poetry.rs` | `extracts_group_dependencies` (+ extracts_dev_dependencies) | — |
| resolves lockedVersions from the lockfile | 197 | ported | `poetry.rs` | `lockfile_versions_are_applied` | — |
| parses git dependencies long commit hashes on http urls | 209 | ported | `poetry.rs` | `git_dependencies_with_revisions_are_extracted` | — |
| parses git dependencies short commit hashes on http urls | 234 | ported | `poetry.rs` | `git_dependencies_with_revisions_are_extracted` | — |
| parses git dependencies long commit hashes on ssh urls | 259 | ported | `poetry.rs` | `git_dependencies_with_revisions_are_extracted` | — |
| parses git dependencies long commit hashes on http urls with branch marker | 284 | ported | `poetry.rs` | `git_dependencies_with_revisions_are_extracted` | — |
| parses github dependencies tags on ssh urls | 310 | ported | `poetry.rs` | `git_dependencies_with_tags_are_extracted` | — |
| parses github dependencies tags on http urls | 325 | ported | `poetry.rs` | `git_dependencies_with_tags_are_extracted` | — |
| parses git dependencies with tags that are not on GitHub | 340 | ported | `poetry.rs` | `git_dependencies_with_tags_are_extracted` | — |
| skips git dependencies | 363 | ported | `poetry.rs` | `git_source_skipped` | — |
| skips git dependencies with version | 375 | ported | `poetry.rs` | `git_dep_with_version_shows_version` | — |
| skips path dependencies | 388 | ported | `poetry.rs` | `path_source_skipped` | — |
| skips path dependencies with version | 400 | ported | `poetry.rs` | `path_dep_with_version_shows_version` | — |
| does not include registry url for dependency python | 413 | ported | `poetry.rs` | `python_dependency_has_no_registry_urls` | — |
| can parse empty registries | 436 | ported | `poetry.rs` | `empty_registry_list_returns_no_registry_urls` | — |
| can parse missing registries | 441 | ported | `poetry.rs` | `missing_registry_list_returns_no_registry_urls` | — |
| extracts registries | 446 | ported | `poetry.rs` | `extracts_registry_urls` | — |
| dedupes registries | 455 | ported | `poetry.rs` | `dedupes_registry_urls` | — |
| source with priority="default" and implicit PyPI priority="primary" | 463 | ported | `poetry.rs` | `source_default_with_implicit_pypi_primary` | — |
| source with implicit priority and PyPI with priority="explicit" | 483 | ported | `poetry.rs` | `source_with_explicit_pypi_suppresses_implicit_pypi_url` | — |
| supports dependencies with explicit source | 500 | ported | `poetry.rs` | `dependencies_with_explicit_source_get_registry_urls` | — |
| parses package file with template | 535 | ported | `poetry.rs` | `parses_package_file_with_template_lines` | — |
| extract dependencies from the project section | 555 | ported | `poetry.rs` | `extracts_poetry_v2_project_section_dependencies` | — |
| extracts dependencies from pep735 dependency-groups | 616 | ported | `poetry.rs` | `extracts_pep735_dependency_groups` | — |
| enriches pep621/pep735 dependencies with poetry managerData | 663 | ported | `poetry.rs` | `pep621_and_pep735_deps_are_enriched_from_poetry_metadata` | — |

---

