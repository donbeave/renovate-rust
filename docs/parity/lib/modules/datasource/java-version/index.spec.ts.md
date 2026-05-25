# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/java-version/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/java-version/index.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 10 | **Status:** done

### `modules/datasource/java-version/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for error | 16 | ported | `crates/renovate-core/src/datasources/java_version.rs` | `throws_for_network_error` | 5xx propagates as Err |
| returns null for 404 | 29 | ported | `crates/renovate-core/src/datasources/java_version.rs` | `returns_null_for_404` | page 0 404 → Ok(None) |
| returns null for empty result | 39 | ported | `crates/renovate-core/src/datasources/java_version.rs` | `returns_null_for_empty_result` | empty JSON body → Ok(None) |
| returns null for empty 200 OK | 49 | ported | `crates/renovate-core/src/datasources/java_version.rs` | `returns_null_for_empty_versions` | versions:[] → Ok(None) |
| throws for 5xx | 62 | ported | `crates/renovate-core/src/datasources/java_version.rs` | `throws_for_5xx` | 502 propagates as Err |
| processes real data | 72 | ported | `crates/renovate-core/src/datasources/java_version.rs` | `processes_real_data` | fixture page.json, 3 releases |
| processes real data (jre) | 85 | ported | `crates/renovate-core/src/datasources/java_version.rs` | `processes_real_data_jre` | fixture jre.json, 2 releases |
| processes real data (jre,windows,x64) | 98 | ported | `crates/renovate-core/src/datasources/java_version.rs` | `processes_real_data_jre_windows_x64` | parse_package filters |
| pages | 110 | ported | `crates/renovate-core/src/datasources/java_version.rs` | `pages_multiple_pages` | page 0 full → page 1 404 → 50 releases |
| processes real data (jre,system) | 128 | ported | `crates/renovate-core/src/datasources/java_version.rs` | `processes_real_data_jre_system` | system=true triggers arch/os detection |

---
