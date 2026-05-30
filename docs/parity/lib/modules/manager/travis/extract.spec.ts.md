# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/travis/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/travis/extract.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty if fails to parse | 13 | ported | `travis.rs` | `empty_returns_empty` (+ no_node_js_key_returns_empty, invalid_content_returns_empty) | — |
| returns results | 18 | ported | `travis.rs` | `extracts_node_js_versions` (+ lts_alias_skipped, stable_skipped) | — |
| should handle invalid YAML | 24 | ported | `travis.rs` | `invalid_yaml_no_node_js_returns_empty` | — |
| handles matrix node_js syntax with node_js string | 29 | ported | `travis.rs` | `matrix_jobs_include_node_js_string` | — |
| handles matrix node_js syntax with node_js array | 42 | ported | `travis.rs` | `matrix_jobs_node_js_inline_array` | — |
| handles matrix node_js syntax with node_js array 2 | 60 | ported | `travis.rs` | `matrix_jobs_include_node_js_multiline_list` | — |
| handles matrix node_js syntax with alias | 78 | ported | `travis.rs` | `matrix_alias_node_js_string` | — |
| handles invalid matrix node_js syntax | 91 | ported | `travis.rs` | `matrix_without_node_js_returns_empty` | — |

---

