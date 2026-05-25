# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/helm/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/helm/index.spec.ts
**Total tests:** 14 | **Ported:** 5 | **Actionable:** 14 | **Status:** partial

### `modules/datasource/helm/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if packageName was not provided | 12 | pending | — | — | — |
| returns null if repository was not provided | 22 | pending | — | — | — |
| returns null for empty response | 37 | ported | `helm.rs` | `fetch_latest_empty_body_returns_none` | — |
| returns null for missing response body | 51 | ported | `helm.rs` | `fetch_latest_empty_body_returns_none` | — |
| returns null for 404 | 65 | pending | — | — | — |
| throws for 5xx | 79 | pending | — | — | — |
| returns null for unknown error | 93 | pending | — | — | — |
| returns null if index.yaml in response is empty | 107 | ported | `helm.rs` | `parse_comment_only_index_returns_none` | — |
| returns null if index.yaml in response is invalid | 120 | pending | — | — | — |
| returns null if packageName is not in index.yaml | 139 | ported | `helm.rs` | `parse_returns_none_for_unknown_chart` | — |
| returns list of versions for normal response | 152 | pending | — | — | — |
| returns list of versions for other packages if one packages has no versions | 166 | pending | — | — | — |
| adds trailing slash to subdirectories | 184 | ported | `helm.rs` | `fetch_latest_from_subdirectory_repository` | — |
| uses undefined as the newDigest when no digest is provided | 203 | pending | — | — | — |

---

