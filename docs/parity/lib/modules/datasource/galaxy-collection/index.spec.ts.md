# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/galaxy-collection/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/galaxy-collection/index.spec.ts
**Total tests:** 15 | **Ported:** 15 | **Actionable:** 15 | **Status:** ported

### `modules/datasource/galaxy-collection/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for 404 result | 29 | ported | `crates/renovate-core/src/datasources/galaxy_collection.rs` | `returns_null_for_404` | 4xx → Ok(None) |
| throws for remote host error | 39 | ported | `crates/renovate-core/src/datasources/galaxy_collection.rs` | `throws_for_remote_host_error` | 5xx → Err |
| returns null for unexpected data at base | 49 | ported | `crates/renovate-core/src/datasources/galaxy_collection.rs` | `returns_null_for_unexpected_data_at_base` | Empty/invalid base JSON → Ok(None) |
| returns null for unexpected data at versions | 62 | ported | `crates/renovate-core/src/datasources/galaxy_collection.rs` | `returns_null_for_unexpected_data_at_versions` | Empty/invalid versions JSON → Ok(None) |
| throws error for remote host versions error | 77 | ported | `crates/renovate-core/src/datasources/galaxy_collection.rs` | `throws_for_remote_host_versions_error` | versions 5xx → Err |
| throws error for remote host detailed versions error | 92 | ported | `crates/renovate-core/src/datasources/galaxy_collection.rs` | `throws_for_remote_host_detailed_versions_error` | per-version 5xx → Err |
| returns null for empty lookup | 113 | ported | `crates/renovate-core/src/datasources/galaxy_collection.rs` | `returns_null_for_empty_package_name` | empty package name → Ok(None) |
| returns null for null packageName | 122 | ported | `crates/renovate-core/src/datasources/galaxy_collection.rs` | `returns_null_for_null_package_name` | empty package name → Ok(None) |
| returns null for unknown error | 131 | ported | `crates/renovate-core/src/datasources/galaxy_collection.rs` | `returns_null_for_unknown_error` | no-dot package name → Ok(None) |
| processes real data | 144 | ported | `crates/renovate-core/src/datasources/galaxy_collection.rs` | `processes_real_data` | 3 versions with fixtures; sourceUrl from highest version |
| returns null but matches automation hub URL | 167 | ported | `crates/renovate-core/src/datasources/galaxy_collection.rs` | `automation_hub_url_500_throws` | hub URL 5xx → Err |
| processes real data with automation hub URL | 183 | ported | `crates/renovate-core/src/datasources/galaxy_collection.rs` | `processes_real_data_with_automation_hub_url` | hub URL with published repository |
| returns ansible url with artifactory URL | 212 | ported | `crates/renovate-core/src/datasources/galaxy_collection.rs` | `construct_base_url_artifactory` | ansible protocol URL |
| returns galaxy url with automation hub URL | 223 | ported | `crates/renovate-core/src/datasources/galaxy_collection.rs` | `construct_base_url_automation_hub` | automation hub URL |
| returns galaxy url with other url | 234 | ported | `crates/renovate-core/src/datasources/galaxy_collection.rs` | `construct_base_url_other` | default published repository |

---
