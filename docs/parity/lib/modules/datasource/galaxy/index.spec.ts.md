# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/galaxy/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/galaxy/index.spec.ts
**Total tests:** 11 | **Ported:** 11 | **Actionable:** 11 | **Status:** done

### `modules/datasource/galaxy/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty result | 11 | ported | `crates/renovate-core/src/datasources/galaxy.rs` | `returns_null_for_empty_result` | Empty body → JSON parse error → None |
| returns null for missing fields | 24 | ported | `crates/renovate-core/src/datasources/galaxy.rs` | `returns_null_for_missing_fields` | "undefined" body → JSON parse error → None |
| returns null for empty list | 37 | ported | `crates/renovate-core/src/datasources/galaxy.rs` | `returns_null_for_empty_list` | Whitespace body → JSON parse error → None |
| returns null for 404 | 50 | ported | `crates/renovate-core/src/datasources/galaxy.rs` | `returns_null_for_404` | 404 client error → None |
| returns null for unknown error | 63 | ported | `crates/renovate-core/src/datasources/galaxy.rs` | `returns_null_for_request_error` | Invalid JSON → parse error → None |
| processes real data | 76 | ported | `crates/renovate-core/src/datasources/galaxy.rs` | `processes_real_data` | timezone.json fixture → valid result |
| handles multiple results when one user matches exactly | 90 | ported | `crates/renovate-core/src/datasources/galaxy.rs` | `handles_multiple_results_matching_user` | datadog.json fixture → filter by github_user → 11 releases |
| rejects multiple results when no user matches exactly | 103 | ported | `crates/renovate-core/src/datasources/galaxy.rs` | `rejects_multiple_results_no_user_match` | No matching github_user → None |
| return null if searching random username and project name | 115 | ported | `crates/renovate-core/src/datasources/galaxy.rs` | `returns_null_for_empty_results` | empty fixture → None |
| throws for 5xx | 127 | ported | `crates/renovate-core/src/datasources/galaxy.rs` | `throws_for_5xx` | 502 → Err |
| throws for 404 | 140 | ported | `crates/renovate-core/src/datasources/galaxy.rs` | `returns_null_for_404_dotted` | 404 for foo.bar → None |

---
