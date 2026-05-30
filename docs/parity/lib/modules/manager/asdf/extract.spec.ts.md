# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/asdf/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/asdf/extract.spec.ts
**Total tests:** 13 | **Ported:** 13 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns a result | 6 | ported | `asdf.rs` | `nodejs_maps_to_node_version_datasource` | — |
| provides skipReason for lines with unsupported tooling | 19 | ported | `asdf.rs` | `unknown_tool_gets_skip_reason` | — |
| only captures the first version | 31 | ported | `asdf.rs` | `only_captures_first_version` | — |
| can handle multiple tools in one file | 44 | ported | `asdf.rs` | `extracts_github_releases_tool` (+ 6 others) | — |
| can handle multiple tools with indented versions in one file | 890 | ported | `asdf.rs` | `indented_spacing_still_parses` | — |
| can handle flutter version channel | 923 | ported | `asdf.rs` | `flutter_strips_channel_suffix` | — |
| can handle java jre / jdk | 946 | ported | `asdf.rs` | `java_adoptopenjdk_jdk` (+ 4 others) | — |
| can handle scala v 2 & 3 | 1004 | ported | `asdf.rs` | `scala_v2_uses_scala_scala` (+ 2 others) | — |

### `extractPackageFile() › comment handling › ignores proper comments at the end of lines`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| entry: '${data.entry}' | 1054 | ported | `asdf.rs` | `strips_inline_comments` | — |

### `extractPackageFile() › comment handling`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| invalid comment placements fail to parse | 1069 | ported | `asdf.rs` | `invalid_comment_no_space_fails_parse` | — |
| ignores lines that are just comments | 1076 | ported | `asdf.rs` | `comment_lines_skipped` | — |
| ignores comments across multiple lines | 1081 | ported | `asdf.rs` | `ignores_comments_across_multiple_lines` | — |
| ignores supported tooling with a renovate:ignore comment | 1096 | ported | `asdf.rs` | `renovate_ignore_comment_skips_dep` | — |

---

