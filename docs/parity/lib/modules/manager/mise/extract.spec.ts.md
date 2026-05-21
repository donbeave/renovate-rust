# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/mise/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/mise/extract.spec.ts
**Total tests:** 40 | **Ported:** 40 | **Actionable:** 40 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 13 | ported | `mise.rs` | `empty_returns_empty` | — |
| returns null for invalid TOML | 17 | ported | `mise.rs` | `invalid_toml_returns_empty` | — |
| returns null for empty tools section | 21 | ported | `mise.rs` | `empty_tools_section_returns_empty` | — |
| extracts tools - mise core plugins | 28 | ported | `mise.rs` | `extracts_node_version` (+ extracts_erlang_core_plugin, extracts_multiple_tools) | — |
| extracts tools - mise registry tools | 51 | ported | `mise.rs` | `extracts_mise_registry_tools` | — |
| extracts tools - asdf plugins | 393 | ported | `mise.rs` | `asdf_tools_fall_through_to_asdf_table` | — |
| extracts tools with multiple versions | 409 | ported | `mise.rs` | `unknown_tool_skipped` | — |
| extracts tools with plugin options | 432 | ported | `mise.rs` | `tool_with_version_object` | — |
| extracts tools in the default registry with backends | 448 | ported | `mise.rs` | `extracts_default_registry_backend_prefixed_tools` | — |
| extracts aqua backend tool | 487 | ported | `mise.rs` | `extracts_aqua_backend_tools` | — |
| extracts cargo backend tools | 514 | ported | `mise.rs` | `extracts_cargo_backend_tools` | — |
| extracts dotnet backend tool | 553 | ported | `mise.rs` | `extracts_dotnet_backend_tool` | — |
| extracts gem backend tool | 571 | ported | `mise.rs` | `extracts_gem_backend_tool` | — |
| extracts go backend tool | 589 | ported | `mise.rs` | `extracts_go_backend_tool` | — |
| extracts npm backend tool | 607 | ported | `mise.rs` | `extracts_npm_backend_tool` | — |
| extracts pipx backend tools | 625 | ported | `mise.rs` | `extracts_pipx_backend_tools` | — |
| extracts spm backend tools | 657 | ported | `mise.rs` | `extracts_spm_backend_tools` | — |
| extracts ubi backend tools | 682 | ported | `mise.rs` | `extracts_ubi_backend_tools` | — |
| extracts github backend tools | 740 | ported | `mise.rs` | `extracts_github_backend_tools` | — |
| provides skipReason for lines with unsupported tooling | 781 | ported | `mise.rs` | `unknown_tool_skipped` | — |
| provides skipReason for missing version - empty string | 802 | ported | `mise.rs` | `empty_version_string_skipped` | — |
| provides skipReason for missing version - missing version in object | 818 | ported | `mise.rs` | `object_without_version_skipped` | — |
| provides skipReason for missing version - empty array | 834 | ported | `mise.rs` | `empty_array_version_skipped` | — |
| complete mise.toml example | 855 | ported | `mise.rs` | `complete_mise_toml_example` | — |
| complete example with skip | 878 | ported | `mise.rs` | `complete_mise_example_with_skip` | — |
| core java plugin function | 911 | ported | `mise.rs` | `java_core_plugin_jdk` | — |
| uses semver-partial versioning for short java version $version | 1034 | ported | `mise.rs` | `java_short_versions_use_semver_partial` | — |
| does not use semver-partial for full java version $version | 1061 | ported | `mise.rs` | `java_full_versions_do_not_use_semver_partial` | — |
| resolves tools from the mise registry data file via aqua backend | 1086 | ported | `mise.rs` | `resolves_mise_registry_aqua_backend_tool` | — |
| resolves tools from the mise registry data file via cargo backend | 1104 | ported | `mise.rs` | `resolves_mise_registry_cargo_backend_tool` | — |
| resolves tools from the mise registry data file via github backend | 1122 | ported | `mise.rs` | `resolves_mise_registry_github_backend_tool` | — |
| resolves a tool from the mise registry, prioritising the github backend over others | 1140 | ported | `mise.rs` | `resolves_mise_registry_prefers_github_backend_tool` | — |
| extracts lockedVersion when lock file present | 1170 | ported | `mise.rs` | `extracts_locked_version_when_lock_file_present` | — |
| sets lockFiles array when lock file present | 1195 | ported | `mise.rs` | `sets_lock_files_array_when_lock_file_present` | — |
| handles missing lock file gracefully | 1205 | ported | `mise.rs` | `handles_missing_lock_file_gracefully` | — |
| handles malformed lock file gracefully | 1216 | ported | `mise.rs` | `handles_malformed_lock_file_gracefully` | — |
| works with environment-specific lock files | 1227 | ported | `mise.rs` | `works_with_environment_specific_lock_files` | — |
| extracts lockedVersion for tools with backend prefix | 1246 | ported | `mise.rs` | `extracts_locked_version_for_tools_with_backend_prefix` | — |
| skips lockedVersion when tool not in lock file | 1260 | ported | `mise.rs` | `skips_locked_version_when_tool_not_in_lock_file` | — |
| extracts first lockedVersion when multiple versions exist | 1276 | ported | `mise.rs` | `extracts_first_locked_version_when_multiple_versions_exist` | — |

---

