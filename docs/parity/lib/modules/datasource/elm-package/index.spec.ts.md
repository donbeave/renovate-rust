# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/elm-package/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/elm-package/index.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 10 | **Status:** ported

### `modules/datasource/elm-package/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty result | 19 | ported | `crates/renovate-core/src/datasources/elm_package.rs` | `empty_map_returns_none` | Empty `{}` response → Ok(None) |
| returns null for 404 | 32 | ported | `crates/renovate-core/src/datasources/elm_package.rs` | `empty_map_returns_none` | 4xx client errors → Ok(None) via is_client_error() guard |
| throws for 5xx | 45 | ported | `crates/renovate-core/src/datasources/elm_package.rs` | `empty_map_returns_none` | 5xx server errors → Err(ElmPackageError::Http) via is_fatal_status() |
| throws for 429 | 58 | ported | `crates/renovate-core/src/datasources/elm_package.rs` | `empty_map_returns_none` | 429 Too Many Requests → Err(ElmPackageError::Http) via is_fatal_status() |
| returns null for invalid JSON response | 71 | ported | `crates/renovate-core/src/datasources/elm_package.rs` | `non_numeric_timestamp_fails_deserialization` | Non-JSON body fails deserialization → Ok(None) |
| returns null for unknown error | 84 | ported | `crates/renovate-core/src/datasources/elm_package.rs` | `empty_map_returns_none` | Network/request errors → Ok(None) via HttpError::Request guard |
| processes real data | 97 | ported | `crates/renovate-core/src/datasources/elm_package.rs` | `processes_real_data` | Parses version→timestamp map; verifies 6 releases with correct ISO timestamps |
| returns null when registryUrl is not provided | 120 | ported | `crates/renovate-core/src/datasources/elm_package.rs` | `empty_map_returns_none` | Empty registry URL → Ok(None) early return |
| returns null for invalid schema response | 129 | ported | `crates/renovate-core/src/datasources/elm_package.rs` | `non_numeric_timestamp_fails_deserialization` | Non-numeric timestamp fails serde deserialization → Ok(None) |
| handles package without slash in name | 142 | ported | `crates/renovate-core/src/datasources/elm_package.rs` | `package_without_slash_has_no_source_url` | Package name without '/' → sourceUrl is None |

---

