# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pep621/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pep621/extract.spec.ts
**Total tests:** 14 | **Ported:** 14 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null for empty content | 16 | ported | `pep621.rs` | `empty_content_returns_empty` | — |
| should return null for invalid toml | 21 | ported | `pep621.rs` | `invalid_toml_returns_error` | — |
| should return dependencies for valid content | 32 | ported | `pep621.rs` | `extracts_project_dependencies` (+ extracts_optional_dependencies, extracts_dependency_groups_skips_include_tables) | — |
| should return dependencies with overwritten pypi registryUrl | 233 | ported | `pep621.rs` | `pdm_sources_apply_registry_urls_to_project_optional_and_dev_dependencies` | — |
| should return dependencies with original pypi registryUrl | 309 | ported | `pep621.rs` | `pdm_sources_apply_registry_urls_to_project_dependencies` | — |
| should skip dependencies with unsupported uv sources | 340 | ported | `pep621.rs` | `uv_sources_classify_git_path_url_and_workspace_sources` | — |
| should handle SSH git URLs correctly for GitHub sources | 412 | ported | `pep621.rs` | `uv_sources_handle_ssh_github_tag_and_rev` | — |
| should extract dependencies from hatch environments | 446 | ported | `pep621.rs` | `hatch_env_dependencies_and_extra_dependencies_are_extracted` | — |
| should extract project version | 498 | ported | `pep621.rs` | `project_version_field_is_parseable` | — |
| should extract dependencies from build-system.requires | 510 | ported | `pep621.rs` | `build_system_requires_extracted_with_project_deps` | — |
| should resolve lockedVersions from pdm.lock | 551 | ported | `pep621.rs` | `pdm_fixture` | — |
| should resolve lockedVersions from uv.lock | 595 | ported | `pep621.rs` | `uv_lock_applies_locked_versions` | — |
| should resolve dependencies without locked versions on invalid uv.lock | 661 | ported | `pep621.rs` | `invalid_uv_lock_leaves_deps_without_locked_versions` | — |
| should resolve dependencies with template | 694 | ported | `pep621.rs` | `resolves_dependencies_with_template_lines` | — |

---

