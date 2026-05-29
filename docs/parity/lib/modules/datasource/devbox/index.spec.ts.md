# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/devbox/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/devbox/index.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `modules/datasource/devbox/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for error | 29 | ported | `crates/renovate-core/src/datasources/devbox.rs` | `throws_for_network_error` | 500 → Err |
| returns null for 404 | 43 | ported | `crates/renovate-core/src/datasources/devbox.rs` | `returns_null_for_404` | 404 → None |
| returns null for empty result | 53 | ported | `crates/renovate-core/src/datasources/devbox.rs` | `returns_null_for_empty_result` | `{}` → None |
| returns null for empty 200 OK | 63 | ported | `crates/renovate-core/src/datasources/devbox.rs` | `returns_null_for_empty_releases_array` | `{releases:[]}` → None |
| throws for 5xx | 76 | ported | `crates/renovate-core/src/datasources/devbox.rs` | `throws_for_5xx` | 502 → Err |
| processes real data | 86 | ported | `crates/renovate-core/src/datasources/devbox.rs` | `processes_real_data` | 3 releases, sorted ascending by timestamp |
| processes empty data | 118 | ported | `crates/renovate-core/src/datasources/devbox.rs` | `processes_empty_data` | empty releases array → None |
| returns null when no body is returned | 133 | ported | `crates/renovate-core/src/datasources/devbox.rs` | `returns_null_for_empty_body` | `"null"` → None |
| falls back to a default homepage_url | 145 | ported | `crates/renovate-core/src/datasources/devbox.rs` | `falls_back_for_missing_homepage` | missing homepage_url → None |

---
