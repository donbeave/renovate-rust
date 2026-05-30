# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bazel/parser.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazel/parser.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 0 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses rules input | 6 | ported | `extractors/bazel_parser.rs` | `parse_rules_input_basic` | — |
| parses multiple archives | 81 | ported | `extractors/bazel_parser.rs` | `parse_multiple_archives` | — |
| parses http_archive | 156 | ported | `extractors/bazel_parser.rs` | `parse_http_archive` | — |
| parses http_archive with prefixes and multiple urls | 195 | ported | `extractors/bazel_parser.rs` | `parse_http_archive_multiple_urls` | — |
| parses Maven | 254 | ported | `extractors/bazel_parser.rs` | `parse_maven` | — |

---

