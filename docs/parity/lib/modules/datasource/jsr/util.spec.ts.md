# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/jsr/util.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/jsr/util.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 0 | **Status:** done

### `modules/datasource/jsr/util`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should extract package name | 4 | ported | `crates/renovate-core/src/datasources/jsr.rs` | `extract_package_name_valid` | Valid name → scope+name |
| should return null for invalid name | 11 | ported | `crates/renovate-core/src/datasources/jsr.rs` | `extract_null_for_invalid_path` | 3 parts → None |
| should return null for below scope min length | 17 | ported | `crates/renovate-core/src/datasources/jsr.rs` | `extract_null_for_short_scope` | scope < 3 chars → None |
| should return null for exceed scope max length | 22 | ported | `crates/renovate-core/src/datasources/jsr.rs` | `extract_null_for_long_scope` | scope > 100 chars → None |
| should return null for invalid scope name | 27 | ported | `crates/renovate-core/src/datasources/jsr.rs` | `extract_null_for_non_ascii_scope` | non-ASCII scope → None |
| should return null for invalid package name starting with @ | 32 | ported | `crates/renovate-core/src/datasources/jsr.rs` | `extract_null_for_package_starting_with_at` | package starts with @ → None |
| should return null for exceed package max length | 37 | ported | `crates/renovate-core/src/datasources/jsr.rs` | `extract_null_for_long_package_name` | package > 58 chars → None |
| should return null for invalid package name | 42 | ported | `crates/renovate-core/src/datasources/jsr.rs` | `extract_null_for_uppercase_package` | uppercase package → None |
| should return null for invalid package name starting with - | 47 | ported | `crates/renovate-core/src/datasources/jsr.rs` | `extract_null_for_package_starting_with_dash` | package starts with - → None |

---
