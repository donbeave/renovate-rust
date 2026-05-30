# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/update.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/update.spec.ts
**Total tests:** 38 | **Ported:** 27 | **Actionable:** 38 | **Status:** ported

### `updateDependency › deno.json/jsonc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates dependency in imports | 9 | ported | `extractors/deno.rs` | `deno_update_imports` | — |
| throws when multiple imports require more than one replacement | 39 | ported | `extractors/deno.rs` | `deno_update_imports_multiple_returns_none` | Returns None instead of throw |
| updates dependency in scopes | 64 | ported | `extractors/deno.rs` | `deno_update_scopes` | — |
| returns null when scopes element not found | 98 | ported | `extractors/deno.rs` | `deno_update_scopes_not_found` | — |
| updates dependency in tasks | 127 | ported | `extractors/deno.rs` | `deno_update_tasks` | — |
| updates dependency in tasks.command | 158 | ported | `extractors/deno.rs` | `deno_update_tasks_command` | —  | — | — | — |
| returns null when tasks element not found | 191 | ported | `extractors/deno.rs` | `deno_update_tasks_not_found` | — |
| returns null when tasks.command element not found | 221 | ported | `extractors/deno.rs` | `deno_update_tasks_command_not_found` | —  | — | — | — |
| updates dependency in compilerOptions.types | 251 | ported | `extractors/deno.rs` | `deno_update_compiler_types` | — |
| returns null when compilerOptions.types is empty array | 281 | ported | `extractors/deno.rs` | `deno_update_compiler_types_empty` | — |
| returns null when compilerOptions.types element not found | 308 | ported | `extractors/deno.rs` | `deno_update_compiler_types_not_found` | —  | — | — | — |
| updates dependency in compilerOptions.jsxImportSource | 335 | ported | `extractors/deno.rs` | `deno_update_jsx_import_source` | — |
| returns null when compilerOptions.jsxImportSource does not exist | 367 | ported | `extractors/deno.rs` | `deno_update_jsx_import_source_not_found` | —  | — | — | — |
| returns null when compilerOptions.jsxImportSourceTypes does not exist | 394 | ported | `extractors/deno.rs` | `deno_update_jsx_import_source_types_not_found` | —  | — | — | — |
| updates dependency in compilerOptions.jsxImportSourceTypes | 421 | ported | `extractors/deno.rs` | `deno_update_jsx_import_source_types` | —  | — | — | — |
| updates dependency in lint plugins | 453 | ported | `extractors/deno.rs` | `deno_update_lint_plugins` | — |
| returns null when lint.plugins element not found | 481 | ported | `extractors/deno.rs` | `deno_update_lint_plugins_not_found` | —  | — | — | — |
| returns null when lint.plugins is empty array | 508 | ported | `extractors/deno.rs` | `deno_update_lint_plugins_empty` | —  | — | — | — |
| handles dependency without version | 535 | ported | `extractors/deno.rs` | `deno_update_dep_without_version` | —  | — | — | — |
| returns null if packageFile is not defined | 563 | ported | `extractors/deno.rs` | `deno_update_no_package_file` | — |
| returns null for not supported datasource | 575 | ported | `extractors/deno.rs` | `deno_update_unsupported_datasource` | — |
| currentValue is not defined when deno datasource | 602 | ported | `extractors/deno.rs` | `deno_update_no_current_value_deno` | —  | — | — | — |
| returns null for missing required values | 629 | ported | `extractors/deno.rs` | `deno_update_missing_required_values` | —  | — | — | — |
| handles complex JSON with nested structures | 648 | ported | `extractors/deno.rs` | `deno_update_complex_json` | —  | — | — | — |
| handles the case where the desired version is already supported | 689 | ported | `extractors/deno.rs` | `deno_update_already_at_version` | — |
| returns null if empty file | 712 | ported | `extractors/deno.rs` | `deno_update_empty_file` | — |
| handles error during update gracefully | 731 | ported | `extractors/deno.rs` | `deno_update_invalid_json_returns_none` | — |

---
