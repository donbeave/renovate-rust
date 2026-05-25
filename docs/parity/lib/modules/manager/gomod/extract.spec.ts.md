# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gomod/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gomod/extract.spec.ts
**Total tests:** 22 | **Ported:** 17 | **Actionable:** 22 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 12 | ported | `gomod.rs` | `empty_content_returns_empty` | — |
| extracts single-line requires | 16 | ported | `gomod.rs` | `single_line_require` | — |
| extracts multi-line requires | 26 | ported | `gomod.rs` | `require_block` | — |
| ignores empty spaces in multi-line requires | 34 | ported | `gomod.rs` | `empty_lines_inside_require_block` | — |
| extracts replace directives from multi-line and single line | 48 | ported | `gomod.rs` | `replace_directives_multi_line_and_single_line` | — |
| extracts replace directives from non-public module path | 136 | ported | `gomod.rs` | `replace_directive_non_public_module_path` | — |
| ignores exclude directives from multi-line and single line | 193 | ported | `gomod.rs` | `exclude_block_ignored` | — |
| extracts the toolchain directive | 212 | ported | `gomod.rs` | `toolchain_directive_extracted` | — |
| extracts single-line tool directives | 263 | ported | `gomod.rs` | `tool_directive_single_line_ignored` | — |
| extracts multi-line tool directives | 282 | ported | `gomod.rs` | `tool_directive_multi_line_ignored` | — |
| extracts tool directives with required modules | 304 | ported | `gomod.rs` | `tool_directive_with_required_module_not_indirect` | — |
| extracts tool directives of sub-modules | 323 | ported | `gomod.rs` | `tool_directive_sub_modules_disable_non_matching_indirects` | — |
| extracts tool directives with exact match | 370 | ported | `gomod.rs` | `tool_directive_exact_match_keeps_indirect_enabled` | — |
| extracts tool directives with no matching dependencies | 389 | ported | `gomod.rs` | `tool_directive_alone_produces_no_deps` | — |
| ignores directives unrelated to dependencies | 402 | ported | `gomod.rs` | `unrelated_directives_ignored` | — |
| marks placeholder pseudo versions with skipReason invalid-version | 426 | ported | `gomod.rs` | `placeholder_pseudo_versions_have_skip_reason` | — |
| parses go $version directive | 528 | ported | `gomod.rs` | `go_directive_extracted` | — |
| the extracted version can be used as a SemVer constraint | 582 | pending | — | — | — |
| matches version 1.19, even though it is not valid SemVer | 586 | pending | — | — | — |
| matches the current SemVer minor | 590 | pending | — | — | — |
| does not match the next SemVer minor | 595 | pending | — | — | — |
| does not match the previous SemVer minor | 600 | pending | — | — | — |

---

