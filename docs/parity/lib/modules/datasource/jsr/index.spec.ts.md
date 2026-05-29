# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/jsr/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/jsr/index.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `modules/datasource/jsr/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null for invalid package name | 24 | ported | `crates/renovate-core/src/datasources/jsr.rs` | `returns_null_for_invalid_package_name` | Invalid name → None |
| should return null for no versions | 31 | ported | `crates/renovate-core/src/datasources/jsr.rs` | `returns_null_for_no_versions` | Empty versions map → None |
| should fetch package info from jsr | 44 | ported | `crates/renovate-core/src/datasources/jsr.rs` | `fetches_package_info` | versions with/without createdAt → MINIMUM_RELEASE_TIMESTAMP fallback |
| contains yanked versions | 67 | ported | `crates/renovate-core/src/datasources/jsr.rs` | `contains_yanked_versions` | yanked=true → isDeprecated=true, still included |
| should return null if lookup fails | 89 | ported | `crates/renovate-core/src/datasources/jsr.rs` | `throws_for_404` | 404 → Err |
| should throw error for unparseable | 100 | ported | `crates/renovate-core/src/datasources/jsr.rs` | `throws_for_unparseable` | Invalid JSON → Err |

---
