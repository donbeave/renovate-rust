# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/go/goproxy-parser.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/go/goproxy-parser.spec.ts
**Total tests:** 9 | **Ported:** 7 | **Actionable:** 9 | **Status:** partial

### `modules/datasource/go/goproxy-parser › parseGoproxy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses single url | 10 | ported | `util.rs` | `test_parse_goproxy_single` | — |
| parses multiple urls | 15 | ported | `util.rs` | `test_parse_goproxy_multiple` | — |
| ignores everything starting from "direct" and "off" keywords | 25 | ported | `util.rs` | `test_parse_goproxy_empty_and_keywords` | — |
| caches results | 43 | pending | — | — | —|

### `modules/datasource/go/goproxy-parser › parseNoproxy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| produces regex | 49 | ported | `util.rs` | `test_parse_noproxy_produces_regex` | — |
| matches on real package prefixes | 68 | ported | `util.rs` | `test_parse_noproxy_real_prefixes` | — |
| matches on wildcards | 100 | ported | `util.rs` | `test_parse_noproxy_wildcards` | — |
| matches on character ranges | 126 | ported | `util.rs` | `test_parse_noproxy_char_ranges` | — |
| caches results | 131 | pending | — | — | —|

---
