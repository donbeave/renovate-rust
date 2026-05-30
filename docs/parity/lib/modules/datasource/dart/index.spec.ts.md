# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/dart/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/dart/index.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 0 | **Status:** done

### `modules/datasource/dart/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty result | 13 | ported | `crates/renovate-core/src/datasources/pub_dev.rs` | `returns_null_for_empty_result` | Parse error → None |
| returns null for empty fields | 23 | ported | `crates/renovate-core/src/datasources/pub_dev.rs` | `returns_null_for_empty_fields` | Missing versions or latest → None |
| returns null for 404 | 55 | ported | `crates/renovate-core/src/datasources/pub_dev.rs` | `returns_null_for_404` | 404 → None |
| throws for 5xx | 65 | ported | `crates/renovate-core/src/datasources/pub_dev.rs` | `throws_for_5xx` | 502 → Err |
| returns null for unknown error | 75 | ported | `crates/renovate-core/src/datasources/pub_dev.rs` | `returns_null_for_unknown_error` | Network error → None |
| processes real data | 85 | ported | `crates/renovate-core/src/datasources/pub_dev.rs` | `processes_real_data` | 44 non-retracted releases from shared_preferences.json |

---
