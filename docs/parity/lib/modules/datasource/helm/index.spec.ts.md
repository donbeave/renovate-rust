# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/helm/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/helm/index.spec.ts
**Total tests:** 14 | **Ported:** 14 | **Actionable:** 14 | **Status:** done

### `modules/datasource/helm/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if packageName was not provided | 12 | ported | `crates/renovate-core/src/datasources/helm.rs` | `returns_null_if_package_name_not_provided` | empty packageName → None |
| returns null if repository was not provided | 22 | ported | `crates/renovate-core/src/datasources/helm.rs` | `returns_null_if_repository_not_provided` | 404 from registry → None |
| returns null for empty response | 37 | ported | `crates/renovate-core/src/datasources/helm.rs` | `fetch_releases_empty_body_returns_none` | empty body → None |
| returns null for missing response body | 51 | ported | `crates/renovate-core/src/datasources/helm.rs` | `fetch_releases_empty_body_returns_none` | same test covers both |
| returns null for 404 | 65 | ported | `crates/renovate-core/src/datasources/helm.rs` | `fetch_releases_404_returns_none` | 404 → None |
| throws for 5xx | 79 | ported | `crates/renovate-core/src/datasources/helm.rs` | `fetch_releases_5xx_returns_err` | 5xx → Err(ExternalHost) |
| returns null for unknown error | 93 | ported | `crates/renovate-core/src/datasources/helm.rs` | `fetch_releases_network_error_returns_none` | network error → None |
| returns null if index.yaml in response is empty | 107 | ported | `crates/renovate-core/src/datasources/helm.rs` | `parse_comment_only_index_returns_none` | comment-only → None |
| returns null if index.yaml in response is invalid | 120 | ported | `crates/renovate-core/src/datasources/helm.rs` | `parse_invalid_yaml_returns_none` | invalid YAML → None |
| returns null if packageName is not in index.yaml | 139 | ported | `crates/renovate-core/src/datasources/helm.rs` | `parse_returns_none_for_unknown_chart` | chart not found → None |
| returns list of versions for normal response | 152 | ported | `crates/renovate-core/src/datasources/helm.rs` | `fetch_releases_returns_versions` | ambassador → 27 releases |
| returns list of versions for other packages if one packages has no versions | 166 | ported | `crates/renovate-core/src/datasources/helm.rs` | `fetch_releases_skips_empty_package` | empty package ignored; ambassador has 1 release |
| adds trailing slash to subdirectories | 184 | ported | `crates/renovate-core/src/datasources/helm.rs` | `fetch_releases_from_subdirectory` | subdir URL → 27 ambassador releases |
| uses undefined as the newDigest when no digest is provided | 203 | ported | `crates/renovate-core/src/datasources/helm.rs` | `fetch_releases_blank_digest_is_none` | blank digest → new_digest = None |

---
