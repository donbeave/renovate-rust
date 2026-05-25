# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/endoflife-date/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/endoflife-date/index.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** done

### `modules/datasource/endoflife-date/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| processes real data | 22 | ported | `crates/renovate-core/src/datasources/endoflife.rs` | `processes_real_data` | EKS fixture; date-sensitive (frozen 2023-06-03 in TS, uses real date in Rust) |
| returns null without registryUrl | 83 | ported | `crates/renovate-core/src/datasources/endoflife.rs` | `returns_null_without_registry_url` | empty registryUrl → None |
| returns null for 404 | 92 | ported | `crates/renovate-core/src/datasources/endoflife.rs` | `returns_null_for_404` | 404 → None |
| returns null for empty result | 102 | ported | `crates/renovate-core/src/datasources/endoflife.rs` | `returns_null_for_empty_result` | empty array → None |
| throws for 5xx | 112 | ported | `crates/renovate-core/src/datasources/endoflife.rs` | `throws_for_5xx` | 502 → Err |
| detects boolean discontinuation | 122 | ported | `crates/renovate-core/src/datasources/endoflife.rs` | `detects_boolean_discontinuation` | Cassandra; discontinued=true (bool) |
| detects date discontinuation | 158 | ported | `crates/renovate-core/src/datasources/endoflife.rs` | `detects_date_discontinuation` | Fairphone; discontinued as date string |

---
