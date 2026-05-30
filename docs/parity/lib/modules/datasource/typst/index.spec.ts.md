# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/typst/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/typst/index.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 0 | **Status:** done

### `modules/datasource/typst/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| processes real data | 7 | ported | `crates/renovate-core/src/datasources/typst.rs` | `processes_real_data` | Unix updatedAt → ISO timestamp |
| returns null for unsupported namespace | 74 | ported | `crates/renovate-core/src/datasources/typst.rs` | `returns_null_for_unsupported_namespace` | Only "preview" supported |
| returns null when package not found in registry | 83 | ported | `crates/renovate-core/src/datasources/typst.rs` | `returns_null_when_package_not_found` | Package name not in registry → None |
| handles multiple versions of the same package | 111 | ported | `crates/renovate-core/src/datasources/typst.rs` | `handles_multiple_versions` | All matching entries returned |
| handles registry fetch errors | 163 | ported | `crates/renovate-core/src/datasources/typst.rs` | `handles_registry_fetch_errors` | HTTP error → None (caching fallback) |
| handles empty registry response | 179 | ported | `crates/renovate-core/src/datasources/typst.rs` | `handles_empty_registry_response` | Empty array → None |

---
