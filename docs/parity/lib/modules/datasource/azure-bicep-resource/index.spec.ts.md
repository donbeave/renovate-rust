# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/azure-bicep-resource/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/azure-bicep-resource/index.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** done

### `modules/datasource/azure-bicep-resource/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null when no version is found | 10 | ported | `crates/renovate-core/src/datasources/azure_bicep.rs` | `should_return_null_when_no_version_is_found` | empty resources → None |
| should return null when package is a function | 32 | ported | `crates/renovate-core/src/datasources/azure_bicep.rs` | `should_return_null_when_package_is_a_function` | resourceFunctions ignored; unknown pkg → None |
| should return versions when package is a resource | 67 | ported | `crates/renovate-core/src/datasources/azure_bicep.rs` | `should_return_versions_when_package_is_a_resource` | 2 versions; changelog URL |
| should return versions when package is a resource and a function | 109 | ported | `crates/renovate-core/src/datasources/azure_bicep.rs` | `should_return_versions_when_package_is_a_resource_and_a_function` | resource only; function ignored |

---
