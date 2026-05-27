# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bazel-module/bazelrc.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazel-module/bazelrc.spec.ts
**Total tests:** 19 | **Ported:** 19 | **Actionable:** 19 | **Status:** done

### `modules/manager/bazel-module/bazelrc › BazelOption`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parse($a) | 35 | ported | `bazel_module.rs` | `bazelrc_option_parse_cases` | — |

### `modules/manager/bazel-module/bazelrc › CommandEntry`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getOption | 51 | ported | `bazel_module.rs` | `bazelrc_command_entry_get_option` | — |

### `modules/manager/bazel-module/bazelrc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parse | 62 | ported | `bazel_module.rs` | `bazelrc_parse_entries` | — |

### `modules/manager/bazel-module/bazelrc › read()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| when .bazelrc does not exist | 103 | ported | `bazel_module.rs` | `bazelrc_read_bazelrc_not_exist` | — |
| when .bazelrc has invalid lines | 110 | ported | `bazel_module.rs` | `bazelrc_read_invalid_lines_ignored` | — |
| when .bazelrc has no imports | 128 | ported | `bazel_module.rs` | `bazelrc_read_no_imports` | — |
| when .bazelrc has import and try-import, try-import exists | 148 | ported | `bazel_module.rs` | `bazelrc_read_import_and_try_import_both_exist` | — |
| when .bazelrc has import and try-import, try-import does not exist | 173 | ported | `bazel_module.rs` | `bazelrc_read_try_import_not_exist_skipped` | — |
| when .bazelrc multi-level import | 188 | ported | `bazel_module.rs` | `bazelrc_read_multi_level_import` | — |
| when bazlerc files recursively import each other | 213 | ported | `bazel_module.rs` | `bazelrc_read_cycle_returns_error` | — |
| when .bazelrc refers to a non-local file | 239 | ported | `bazel_module.rs` | `bazelrc_read_non_local_import_skipped` | — |
| when bazelrc has %workspace% paths in options | 255 | ported | `bazel_module.rs` | `bazelrc_read_workspace_paths_in_options` | — |
| when bazelrc has %workspace% paths in imported files | 274 | ported | `bazel_module.rs` | `bazelrc_read_workspace_paths_in_imported_files` | — |

### `modules/manager/bazel-module/bazelrc › expandWorkspacePath`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return original value if no workspace path | 304 | ported | `bazel_module.rs` | `bazelrc_expand_workspace_path_returns_original_without_workspace_path` | — |
| should expand valid workspace path | 310 | ported | `bazel_module.rs` | `bazelrc_expand_workspace_path_expands_valid_workspace_path` | — |
| should throw error for invalid workspace path | 320 | ported | `bazel_module.rs` | `bazelrc_expand_workspace_path_returns_none_for_invalid_workspace_path` | — |

### `modules/manager/bazel-module/bazelrc › sanitizeOptions`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should handle options without values | 328 | ported | `bazel_module.rs` | `bazelrc_sanitize_options_handles_options_without_values` | — |
| should expand valid workspace paths | 333 | ported | `bazel_module.rs` | `bazelrc_sanitize_options_expands_valid_workspace_paths` | — |
| should throw error for invalid workspace paths | 352 | ported | `bazel_module.rs` | `bazelrc_sanitize_options_drops_invalid_workspace_paths` | — |

---

