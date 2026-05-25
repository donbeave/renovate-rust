# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/glasskube-packages/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/glasskube-packages/index.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** done

### `modules/datasource/glasskube-packages/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should handle error response on versions request | 29 | ported | `crates/renovate-core/src/datasources/glasskube_packages.rs` | `error_on_versions_request` | 500 versions.yaml → Err |
| should handle empty response on versions request | 41 | ported | `crates/renovate-core/src/datasources/glasskube_packages.rs` | `empty_versions_response` | Empty body → None |
| should handle error response on manifest request | 51 | ported | `crates/renovate-core/src/datasources/glasskube_packages.rs` | `error_on_manifest_request` | 500 package.yaml → Err |
| should handle empty response on manifest request | 67 | ported | `crates/renovate-core/src/datasources/glasskube_packages.rs` | `empty_manifest_response` | Empty manifest body → None |
| should handle package manifest without references | 81 | ported | `crates/renovate-core/src/datasources/glasskube_packages.rs` | `manifest_without_references` | No refs → releases+tags, no sourceUrl/homepage |
| should handle package manifest with references and default url | 100 | ported | `crates/renovate-core/src/datasources/glasskube_packages.rs` | `manifest_with_references_default_url` | refs fixture → sourceUrl+homepage |
| should handle package manifest with references and custom url | 119 | ported | `crates/renovate-core/src/datasources/glasskube_packages.rs` | `manifest_with_references_custom_url` | custom registry → same refs extraction |

---
