# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/rubygems/metadata-cache.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/rubygems/metadata-cache.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 7 | **Status:** not-applicable

### `modules/datasource/rubygems/metadata-cache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fetches data | 26 | not-applicable | — | — | Uses httpMock + vi.mock(packageCache); HTTP/package-cache mock infrastructure not portable to Rust |
| handles inconsistent data between versions and endpoint | 100 | not-applicable | — | — | Uses httpMock + vi.mock(packageCache); HTTP/package-cache mock infrastructure not portable to Rust |
| handles inconsistent data between cache and endpoint | 137 | not-applicable | — | — | Uses httpMock + vi.mock(packageCache); HTTP/package-cache mock infrastructure not portable to Rust |
| returns cached data | 204 | not-applicable | — | — | Uses httpMock + vi.mock(packageCache); HTTP/package-cache mock infrastructure not portable to Rust |
| fetches for stale key | 240 | not-applicable | — | — | Uses httpMock + vi.mock(packageCache); HTTP/package-cache mock infrastructure not portable to Rust |
| returns fallback results on 404 | 288 | not-applicable | — | — | Uses httpMock + vi.mock(packageCache); HTTP/package-cache mock infrastructure not portable to Rust |
| returns fallback result on unknown error | 308 | not-applicable | — | — | Uses httpMock + vi.mock(packageCache); HTTP/package-cache mock infrastructure not portable to Rust |

---

