# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/orb/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/orb/index.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `modules/datasource/orb/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty result | 32 | ported | `crates/renovate-core/src/datasources/orb.rs` | `returns_null_for_empty_result` | `{}` response → None |
| returns null for missing orb | 42 | ported | `crates/renovate-core/src/datasources/orb.rs` | `returns_null_for_missing_orb` | `{data: {}}` → None |
| returns null for 404 | 55 | ported | `crates/renovate-core/src/datasources/orb.rs` | `returns_null_for_404` | 404 → None |
| returns null for unknown error | 65 | ported | `crates/renovate-core/src/datasources/orb.rs` | `returns_null_for_unknown_error` | Network error → None |
| processes real data | 75 | ported | `crates/renovate-core/src/datasources/orb.rs` | `processes_real_data` | 10 releases sorted by semver; homeUrl empty → default URL |
| processes homeUrl | 85 | ported | `crates/renovate-core/src/datasources/orb.rs` | `processes_home_url` | Non-empty homeUrl used as homepage |
| supports other registries | 96 | ported | `crates/renovate-core/src/datasources/orb.rs` | `supports_other_registries` | Custom registry URL works |

---
