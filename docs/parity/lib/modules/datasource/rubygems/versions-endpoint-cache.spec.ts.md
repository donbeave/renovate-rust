# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/rubygems/versions-endpoint-cache.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/rubygems/versions-endpoint-cache.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 10 | **Status:** not-applicable

### `modules/datasource/rubygems/versions-endpoint-cache › versionsEndpointCache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports sequential access | 25 | not-applicable | — | — | Uses httpMock + memCache mock; HTTP mock infrastructure not portable to Rust |
| supports concurrent access | 43 | not-applicable | — | — | Uses httpMock + memCache mock; HTTP mock infrastructure not portable to Rust |
| handles 404 | 57 | not-applicable | — | — | Uses httpMock + memCache mock; HTTP mock infrastructure not portable to Rust |
| handles unknown error | 69 | not-applicable | — | — | Uses httpMock + memCache mock; HTTP mock infrastructure not portable to Rust |
| refreshes after 15 minutes | 91 | not-applicable | — | — | Uses httpMock + memCache mock; HTTP mock infrastructure not portable to Rust |
| handles tail-head mismatch | 117 | not-applicable | — | — | Uses httpMock + memCache mock; HTTP mock infrastructure not portable to Rust |
| handles full body response | 154 | not-applicable | — | — | Uses httpMock + memCache mock; HTTP mock infrastructure not portable to Rust |
| handles 404 | 186 | not-applicable | — | — | Uses httpMock + memCache mock; HTTP mock infrastructure not portable to Rust |
| handles 416 | 196 | not-applicable | — | — | Uses httpMock + memCache mock; HTTP mock infrastructure not portable to Rust |
| handles unknown errors | 216 | not-applicable | — | — | Uses httpMock + memCache mock; HTTP mock infrastructure not portable to Rust |

---

