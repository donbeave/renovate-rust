# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/repology/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/repology/index.spec.ts
**Total tests:** 19 | **Ported:** 18 | **Actionable:** 19 | **Status:** partial

### `modules/datasource/repology/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty result | 69 | ported | `crates/renovate-core/src/datasources/repology.rs` | `returns_null_for_empty_result` | 200+[] → None |
| returns null for missing repository or package | 88 | ported | `crates/renovate-core/src/datasources/repology.rs` | `returns_null_for_missing_repository_or_package` | 404 → None |
| throws error on unexpected API response | 105 | ported | `crates/renovate-core/src/datasources/repology.rs` | `throws_error_on_unexpected_api_response` | srcname 403 → API 500 → Err |
| throws error on unexpected Resolver response with binary package | 124 | ported | `crates/renovate-core/src/datasources/repology.rs` | `throws_error_on_unexpected_resolver_response_binary` | binname 500 → Err |
| throws error on unexpected Resolver response with source package | 138 | ported | `crates/renovate-core/src/datasources/repology.rs` | `throws_error_on_unexpected_resolver_response_source` | srcname 500 → Err |
| throws error on API request timeout | 156 | ported | `crates/renovate-core/src/datasources/repology.rs` | `throws_error_on_api_request_timeout` | API 500 simulates timeout → Err |
| throws error on Resolver request timeout | 175 | ported | `crates/renovate-core/src/datasources/repology.rs` | `throws_error_on_resolver_request_timeout` | binname 500 → Err |
| returns null on Resolver ambiguous binary package | 189 | ported | `crates/renovate-core/src/datasources/repology.rs` | `returns_null_on_resolver_ambiguous_binary_package` | 300 → None |
| throws without repository and package name | 204 | ported | `crates/renovate-core/src/datasources/repology.rs` | `throws_without_repository_and_package_name` | no `/` → InvalidPackageName |
| throws on disabled host | 214 | pending | — | — | —|
| returns correct version for binary package | 225 | ported | `crates/renovate-core/src/datasources/repology.rs` | `returns_correct_version_for_binary_package` | nginx fixture; origversion |
| returns correct version for source package | 241 | ported | `crates/renovate-core/src/datasources/repology.rs` | `returns_correct_version_for_source_package` | gcc-defaults fixture; srcname |
| returns correct version for api package | 260 | ported | `crates/renovate-core/src/datasources/repology.rs` | `returns_correct_version_for_api_package` | 403 → API fallback |
| returns correct version for multi-package project with same name | 276 | ported | `crates/renovate-core/src/datasources/repology.rs` | `returns_correct_version_for_multi_package_same_name` | gcc fixture; binname filter |
| returns correct version for multi-package project with different name | 292 | ported | `crates/renovate-core/src/datasources/repology.rs` | `returns_correct_version_for_multi_package_different_name` | pulseaudio fixture; single-pkg rule |
| returns multiple versions if they are present in repository | 308 | ported | `crates/renovate-core/src/datasources/repology.rs` | `returns_multiple_versions_if_present` | openjdk fixture; 6 releases |
| returns null for scenario when repo is not in package results | 328 | ported | `crates/renovate-core/src/datasources/repology.rs` | `returns_null_when_repo_not_in_results` | repo not in response → None |
| returns correct package types for api_call | 354 | ported | `crates/renovate-core/src/datasources/repology.rs` | `returns_correct_package_types_for_api_call` | API; srcname+binname filter; 2 releases |
| returns correct package versions for multi-package project | 443 | ported | `crates/renovate-core/src/datasources/repology.rs` | `returns_correct_package_versions_for_multi_package_project` | python fixture; de-dup across binname/srcname |

---
