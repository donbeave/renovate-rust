# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/ruby-version/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/ruby-version/index.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** done

### `modules/datasource/ruby-version/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses real data | 10 | ported | `crates/renovate-core/src/datasources/ruby_version.rs` | `parses_real_data` | Parses HTML table, filters to X.Y.Z stable versions only |
| returns null for empty result | 22 | ported | `crates/renovate-core/src/datasources/ruby_version.rs` | `returns_null_for_empty_result` | Non-HTML response → no releases → None |
| throws for 404 | 34 | ported | `crates/renovate-core/src/datasources/ruby_version.rs` | `throws_for_404` | 404 → Err (all HTTP errors propagate) |

---
