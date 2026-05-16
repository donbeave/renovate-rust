# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gomod/line-parser.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gomod/line-parser.spec.ts
**Total tests:** 32 | **Ported:** 32 | **Actionable:** 32 | **Status:** ported

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null for invalid input | 4 | ported | `gomod.rs` | `parse_line_invalid_returns_none` | — |
| should parse go version | 8 | ported | `gomod.rs` | `parse_line_go_version` | — |
| should skip invalid go version | 21 | ported | `gomod.rs` | `parse_line_go_version_invalid` | — |
| should parse toolchain version | 35 | ported | `gomod.rs` | `parse_line_toolchain_version` | — |
| should skip invalid toolchain version | 48 | ported | `gomod.rs` | `parse_line_toolchain_version_invalid` | — |
| should parse require definition | 61 | ported | `gomod.rs` | `parse_line_require_definition` | — |
| should parse require definition with pseudo-version | 73 | ported | `gomod.rs` | `parse_line_require_pseudo_version` | — |
| should parse require definition with placeholder pseudo-version | 87 | ported | `gomod.rs` | `parse_line_require_placeholder_pseudo_version` | — |
| should parse require multi-line | 102 | ported | `gomod.rs` | `parse_line_require_multiline` | — |
| should parse require definition with quotes | 117 | ported | `gomod.rs` | `parse_line_require_with_quotes` | — |
| should parse go modules without paths - 1 | 129 | ported | `gomod.rs` | `parse_line_require_without_path_1` | — |
| should parse go modules without paths - 2 | 140 | ported | `gomod.rs` | `parse_line_require_without_path_2` | — |
| should parse require multi-line definition with quotes | 151 | ported | `gomod.rs` | `parse_line_require_multiline_with_quotes` | — |
| should parse require definition with indirect dependency | 166 | ported | `gomod.rs` | `parse_line_require_indirect` | — |
| should parse require multi-line definition with indirect dependency | 179 | ported | `gomod.rs` | `parse_line_require_multiline_indirect` | — |
| should parse replace definition | 195 | ported | `gomod.rs` | `parse_line_replace_no_version` | — |
| should parse replace multi-line definition | 206 | ported | `gomod.rs` | `parse_line_replace_multiline` | — |
| should parse replace definition with quotes | 220 | ported | `gomod.rs` | `parse_line_replace_with_quotes` | — |
| should parse replace multi-line definition with quotes | 231 | ported | `gomod.rs` | `parse_line_replace_multiline_with_quotes` | — |
| should parse replace definition with version | 245 | ported | `gomod.rs` | `parse_line_replace_with_version` | — |
| should parse replace definition with pseudo-version | 257 | ported | `gomod.rs` | `parse_line_replace_pseudo_version` | — |
| should parse replace definition with placeholder pseudo-version | 272 | ported | `gomod.rs` | `parse_line_replace_placeholder_pseudo_version` | — |
| should parse replace indirect definition | 288 | ported | `gomod.rs` | `parse_line_replace_indirect` | — |
| should parse replace multi-line definition with version | 301 | ported | `gomod.rs` | `parse_line_replace_multiline_with_version` | — |
| should parse replace definition pointing to relative local path | 316 | ported | `gomod.rs` | `parse_line_replace_local_relative` | — |
| should parse replace definition pointing to absolute local path | 327 | ported | `gomod.rs` | `parse_line_replace_local_absolute` | — |
| should parse tool definition | 338 | ported | `gomod.rs` | `parse_line_tool_definition` | — |
| should parse tool multi-line | 349 | ported | `gomod.rs` | `parse_line_tool_multiline` | — |
| should parse tool definition with quotes | 363 | ported | `gomod.rs` | `parse_line_tool_with_quotes` | — |
| should parse go tool without paths - 1 | 374 | ported | `gomod.rs` | `parse_line_tool_without_path_1` | — |
| should parse go tool without paths - 2 | 385 | ported | `gomod.rs` | `parse_line_tool_without_path_2` | — |
| should parse tool multi-line definition with quotes | 396 | ported | `gomod.rs` | `parse_line_tool_multiline_with_quotes` | — |

---

