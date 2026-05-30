# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/kubernetes-api/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/kubernetes-api/index.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 0 | **Status:** done

### `modules/versioning/kubernetes-api/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isStable("$version") === $expected | 6 | ported | crates/renovate-core/src/versioning/kubernetes_api.rs | is_stable_matches_renovate_kubernetes_api_index_spec | — |
| isValid("$version") === $expected | 16 | ported | crates/renovate-core/src/versioning/kubernetes_api.rs | is_valid_matches_renovate_kubernetes_api_index_spec | — |
| getMajor, getMinor, getPatch for "$version" | 37 | ported | crates/renovate-core/src/versioning/kubernetes_api.rs | get_major_minor_patch_matches_renovate_kubernetes_api_index_spec | — |
| equals("$version", "$other") === $expected | 54 | ported | crates/renovate-core/src/versioning/kubernetes_api.rs | equals_matches_renovate_kubernetes_api_index_spec | — |
| matches("$version", "$other") === $expected | 81 | ported | crates/renovate-core/src/versioning/kubernetes_api.rs | matches_matches_renovate_kubernetes_api_index_spec | — |
| isGreaterThan("$version", "$other") === $expected | 100 | ported | crates/renovate-core/src/versioning/kubernetes_api.rs | is_greater_than_matches_renovate_kubernetes_api_index_spec | — |
| sorts versions in an ascending order | 116 | ported | crates/renovate-core/src/versioning/kubernetes_api.rs | sort_versions_matches_renovate_kubernetes_api_index_spec | — |

---

