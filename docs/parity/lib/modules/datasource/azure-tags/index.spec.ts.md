# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/azure-tags/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/azure-tags/index.spec.ts
**Total tests:** 5 | **Ported:** 2 | **Actionable:** 0 | **Status:** done

### `modules/datasource/azure-tags/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns tags from azure devops | 20 | not-applicable | — | — | Azure API mock test; requires azure-devops-node-api + platform mocking; datasource not yet implemented |
| filters out undefined names | 47 | not-applicable | — | — | Azure API mock test; requires azure-devops-node-api + platform mocking; datasource not yet implemented |
| handles api errors | 70 | not-applicable | — | — | Azure API mock test; requires azure-devops-node-api + platform mocking; datasource not yet implemented |

### `modules/datasource/azure-tags/index › static methods`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getCacheKey returns the expected format | 83 | ported | `util.rs` | `test_azure_tags_cache_key` | — |
| getSourceUrl returns the correct URL format | 92 | ported | `util.rs` | `test_azure_tags_source_url` | — |

---

