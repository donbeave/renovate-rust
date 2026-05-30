# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/azure-rest-api/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/azure-rest-api/index.spec.ts
**Total tests:** 11 | **Ported:** 11 | **Actionable:** 0 | **Status:** done

### `modules/versioning/azure-rest-api/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $expected | 4 | ported | crates/renovate-core/src/versioning/azure_rest_api.rs | is_valid_matches_renovate_azure_rest_api_index_spec | — |
| isCompatible("$version") === $expected | 24 | ported | crates/renovate-core/src/versioning/azure_rest_api.rs | is_compatible_matches_renovate_azure_rest_api_index_spec | — |
| isStable("$version") === $expected | 32 | ported | crates/renovate-core/src/versioning/azure_rest_api.rs | is_stable_matches_renovate_azure_rest_api_index_spec | — |
| isSingleVersion("$version") === $expected | 44 | ported | crates/renovate-core/src/versioning/azure_rest_api.rs | is_single_version_matches_renovate_azure_rest_api_index_spec | — |
| isVersion("$version") === $expected | 52 | ported | crates/renovate-core/src/versioning/azure_rest_api.rs | is_version_matches_renovate_azure_rest_api_index_spec | — |
| getMajor("$version") === 1 | 64 | ported | crates/renovate-core/src/versioning/azure_rest_api.rs | get_major_matches_renovate_azure_rest_api_index_spec | — |
| getMinor("$version") === 0 | 72 | ported | crates/renovate-core/src/versioning/azure_rest_api.rs | get_minor_matches_renovate_azure_rest_api_index_spec | — |
| getPatch("$version") === 0 | 80 | ported | crates/renovate-core/src/versioning/azure_rest_api.rs | get_patch_matches_renovate_azure_rest_api_index_spec | — |
| equals("$version", "$other") === $expected | 88 | ported | crates/renovate-core/src/versioning/azure_rest_api.rs | equals_matches_renovate_azure_rest_api_index_spec | — |
| isGreaterThan("$version", "$other") === $expected | 104 | ported | crates/renovate-core/src/versioning/azure_rest_api.rs | is_greater_than_matches_renovate_azure_rest_api_index_spec | — |
| sortVersions("$version", "$other") === $expected | 124 | ported | crates/renovate-core/src/versioning/azure_rest_api.rs | sort_versions_matches_renovate_azure_rest_api_index_spec | — |

---

