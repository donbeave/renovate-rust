# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/ansible-galaxy/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/ansible-galaxy/extract.spec.ts
**Total tests:** 14 | **Ported:** 14 | **Actionable:** 14 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 15 | ported | `ansible_galaxy.rs` | `empty_content_returns_no_deps` | — |
| extracts multiple dependencies from requirements.yml | 19 | ported | `ansible_galaxy.rs` | `requirements01_extracts_thirteen_deps` | — |
| extracts dependencies from a not beautified requirements file | 25 | ported | `ansible_galaxy.rs` | `non_beautified_requirements_extracts_two_deps` | — |
| extracts dependencies from requirements.yml with a space at the end of line | 31 | ported | `ansible_galaxy.rs` | `collections_with_git_url_name_and_version` | — |
| extracts git@ dependencies | 41 | ported | `ansible_galaxy.rs` | `collections_with_source_field_and_git_at_url` | — |
| check if an empty file returns null | 56 | ported | `ansible_galaxy.rs` | `blank_file_returns_no_deps` | — |
| check if a requirements file of other systems returns null | 61 | ported | `ansible_galaxy.rs` | `non_ansible_content_returns_empty` | — |
| check collection style requirements file | 66 | ported | `ansible_galaxy.rs` | `collections1_extracts_fourteen_deps_all_galaxy_hosted` | — |
| check collection style requirements file in reverse order and missing empty line | 73 | ported | `ansible_galaxy.rs` | `collections_before_roles_extracts_all_four` | — |
| check galaxy definition file | 79 | ported | `ansible_galaxy.rs` | `galaxy_definition_file_extracts_ten_deps` | — |

### `getSliceEndNumber()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| negative start number returns -1 | 87 | ported | `ansible_galaxy.rs` | `get_slice_end_number_negative_start` | — |
| a start number bigger then number of lines return -1 | 92 | ported | `ansible_galaxy.rs` | `get_slice_end_number_start_too_big` | — |
| choose first block | 97 | ported | `ansible_galaxy.rs` | `get_slice_end_number_first_block` | — |
| choose second block | 102 | ported | `ansible_galaxy.rs` | `get_slice_end_number_second_block` | — |

---

