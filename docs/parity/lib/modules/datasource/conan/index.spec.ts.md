# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/conan/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/conan/index.spec.ts
**Total tests:** 22 | **Ported:** 0 | **Actionable:** 22 | **Status:** done

### `modules/datasource/conan/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles package without digest | 38 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles digest | 43 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null for missing revision | 56 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles bad return | 69 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles empty return | 82 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles bad registries | 95 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles missing packages | 109 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| processes real versioned data | 122 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| processes mixed case names | 154 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| uses github instead of conan center | 180 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| works with empty releases | 221 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| rejects userAndChannel for Conan Center | 237 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles mismatched userAndChannel versioned data | 247 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles malformed packages | 261 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles non 404 errors | 282 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles missing slash on registries | 297 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| artifactory sourceurl | 312 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| artifactory header without api | 367 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| artifactory invalid version | 398 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| non artifactory header | 425 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| artifactory no package url | 442 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| artifactory http error | 492 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

---
