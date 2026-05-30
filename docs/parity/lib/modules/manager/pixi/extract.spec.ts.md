# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pixi/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pixi/extract.spec.ts
**Total tests:** 16 | **Ported:** 16 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty pyproject.toml | 145 | ported | `pixi.rs` | `empty_pyproject_returns_empty` | — |
| returns null for empty pixi.toml | 151 | ported | `pixi.rs` | `empty_file_returns_empty` | — |
| returns null for parsed file without pixi section | 155 | ported | `pixi.rs` | `file_without_pixi_section_returns_empty` | — |
| returns parse pixi.toml | 161 | ported | `pixi.rs` | `extracts_pypi_deps` (+ extracts_conda_deps_as_actionable) | — |
| returns parse pixi section from pyproject.toml | 297 | ported | `pixi.rs` | `extract_from_pyproject_tool_pixi` | — |
| returns package of pyproject.toml tool.pixi section | 316 | ported | `pixi.rs` | `extract_tool_pixi_section_without_lockfile` | — |
| returns parse pixi.toml with features | 335 | ported | `pixi.rs` | `extracts_feature_pypi_deps` | — |
| returns parse non-known config file as pyproject.toml | 481 | ported | `pixi.rs` | `non_known_file_with_project_section` | — |
| returns parse non-known config file as pixi.toml | 509 | ported | `pixi.rs` | `non_known_file_with_tool_pixi_section` | — |
| extract feature with channels | 538 | ported | `pixi.rs` | `extract_feature_with_url_channel` | — |
| skip package without channels | 571 | ported | `pixi.rs` | `skip_package_without_channels` | — |
| extract package from with workspace | 601 | ported | `pixi.rs` | `extract_from_workspace_section` | — |
| extract package with channel priority | 630 | ported | `pixi.rs` | `feature_channel_priority_prepends_prioritized_channels` | — |
| returns null for non-known config file | 681 | ported | `pixi.rs` | `non_toml_content_returns_empty` | — |
| set registryStrategy='merge' for channel-priority='disabled' | 685 | ported | `pixi.rs` | `disabled_channel_priority_sets_merge_registry_strategy` | — |
| use default registryStrategy for channel-priority='strict' | 706 | ported | `pixi.rs` | `strict_channel_priority_uses_default_registry_strategy` | — |

---

