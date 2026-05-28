# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gitlab/pr-cache.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gitlab/pr-cache.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 8 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fetches cache initially | 81 | not-applicable | — | — | Uses httpMock + memCache + repoCache; HTTP/cache mock infrastructure not portable to Rust |
| fetches cache with ignorePrAuthor=true | 110 | not-applicable | — | — | Uses httpMock + memCache + repoCache; HTTP/cache mock infrastructure not portable to Rust |
| resets cache for not matching authors | 128 | not-applicable | — | — | Uses httpMock + memCache + repoCache; HTTP/cache mock infrastructure not portable to Rust |
| resets cache for older format with milliseconds | 169 | not-applicable | — | — | Uses httpMock + memCache + repoCache; HTTP/cache mock infrastructure not portable to Rust |
| syncs cache with updated_after parameter | 210 | not-applicable | — | — | Uses httpMock + memCache + repoCache; HTTP/cache mock infrastructure not portable to Rust |
| handles empty response | 251 | not-applicable | — | — | Uses httpMock + memCache + repoCache; HTTP/cache mock infrastructure not portable to Rust |
| returns items in reverse order (most recent first) | 267 | not-applicable | — | — | Uses httpMock + memCache + repoCache; HTTP/cache mock infrastructure not portable to Rust |
| normalizes timestamps by removing milliseconds | 280 | not-applicable | — | — | Uses httpMock + memCache + repoCache; HTTP/cache mock infrastructure not portable to Rust |

---

