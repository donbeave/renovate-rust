# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/cdnjs/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/cdnjs/index.spec.ts
**Total tests:** 14 | **Ported:** 14 | **Actionable:** 14 | **Status:** ported

### `modules/datasource/cdnjs/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for empty result | 18 | ported | `crates/renovate-core/src/datasources/cdnjs.rs` | `throws_for_malformed_json` | malformed JSON → Err |
| throws for error | 28 | ported | `crates/renovate-core/src/datasources/cdnjs.rs` | `throws_for_network_error` | network error → Err |
| returns null for 404 | 38 | ported | `crates/renovate-core/src/datasources/cdnjs.rs` | `returns_null_for_404` | 404 → None |
| returns null for empty 200 OK | 48 | ported | `crates/renovate-core/src/datasources/cdnjs.rs` | `returns_null_for_empty_result` | `{}` → None |
| throws for 401 | 61 | ported | `crates/renovate-core/src/datasources/cdnjs.rs` | `throws_for_401` | 401 → Err |
| throws for 429 | 71 | ported | `crates/renovate-core/src/datasources/cdnjs.rs` | `throws_for_429` | 429 → Err (after retries) |
| throws for 5xx | 81 | ported | `crates/renovate-core/src/datasources/cdnjs.rs` | `throws_for_5xx` | 502 → Err |
| throws for unknown error | 91 | ported | `crates/renovate-core/src/datasources/cdnjs.rs` | `throws_for_unknown_error` | network error → Err |
| processes real data | 101 | ported | `crates/renovate-core/src/datasources/cdnjs.rs` | `processes_real_data` | d3-force fixture; releases + sourceUrl + homepage |
| returs null for no result | 115 | ported | `crates/renovate-core/src/datasources/cdnjs.rs` | `digest_returns_null_for_empty_response` | `{}` → None |
| returs null for empty sri object | 131 | ported | `crates/renovate-core/src/datasources/cdnjs.rs` | `digest_returns_null_for_empty_sri` | `{sri:{}}` → None |
| returs null if file not found | 147 | ported | `crates/renovate-core/src/datasources/cdnjs.rs` | `digest_returns_null_if_file_not_found` | SRI has other file → None |
| returns null for 404 | 163 | ported | `crates/renovate-core/src/datasources/cdnjs.rs` | `digest_throws_for_404` | 404 → Err |
| returns digest | 176 | ported | `crates/renovate-core/src/datasources/cdnjs.rs` | `digest_returns_hash` | bootstrap SRI fixture → sha512 hash |

---

