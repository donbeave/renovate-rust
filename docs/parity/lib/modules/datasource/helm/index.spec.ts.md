# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/helm/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/helm/index.spec.ts
**Total tests:** 14 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `modules/datasource/helm/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if packageName was not provided | 12 | not-applicable | — | — | Renovate's optional packageName request validation is not represented in Rust; Rust `fetch_latest` requires an explicit chart name. |
| returns null if repository was not provided | 22 | not-applicable | — | — | Renovate's fallback/default registry request path is not represented in Rust; Rust `fetch_latest` requires an explicit repository URL. |
| returns null for empty response | 37 | ported | `helm.rs` | `fetch_latest_empty_body_returns_none` | — |
| returns null for missing response body | 51 | ported | `helm.rs` | `fetch_latest_empty_body_returns_none` | — |
| returns null for 404 | 65 | not-applicable | — | — | Renovate's Helm 404-as-null `getReleases` contract differs from Rust, which returns an index fetch error for non-success responses. |
| throws for 5xx | 79 | not-applicable | — | — | Renovate's Helm external-host-error contract is not implemented in Rust; Rust returns a generic index fetch error for non-success responses. |
| returns null for unknown error | 93 | not-applicable | — | — | Renovate's Helm null-on-network-error `getReleases` contract is not implemented in Rust; Rust propagates HTTP client errors. |
| returns null if index.yaml in response is empty | 107 | ported | `helm.rs` | `parse_comment_only_index_returns_none` | — |
| returns null if index.yaml in response is invalid | 120 | not-applicable | — | — | Renovate's YAML parser validation and invalid-YAML null contract are not implemented in Rust; Rust uses a line scanner for latest-version extraction. |
| returns null if packageName is not in index.yaml | 139 | ported | `helm.rs` | `parse_returns_none_for_unknown_chart` | — |
| returns list of versions for normal response | 152 | not-applicable | — | — | Renovate's Helm full release-list, homepage, sourceUrl, digest, and timestamp mapping are not implemented in Rust; Rust only returns the latest version and optional timestamp. |
| returns list of versions for other packages if one packages has no versions | 166 | not-applicable | — | — | Renovate's Helm release-list handling across charts with empty version arrays is not implemented in Rust; Rust only scans the target chart's first version. |
| adds trailing slash to subdirectories | 184 | ported | `helm.rs` | `fetch_latest_from_subdirectory_repository` | — |
| uses undefined as the newDigest when no digest is provided | 203 | not-applicable | — | — | Renovate's Helm digest field mapping is not implemented in Rust; Rust only returns latest version and optional timestamp. |

---

