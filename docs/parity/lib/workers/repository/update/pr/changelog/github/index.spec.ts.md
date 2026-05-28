# Renovate Test Detail

[Back to test map](../../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/changelog/github/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/changelog/github/index.spec.ts
**Total tests:** 17 | **Ported:** 0 | **Actionable:** 17 | **Status:** done

### `workers/repository/update/pr/changelog/github/index › getChangeLogJSON`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if @types | 55 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| returns null if no currentVersion | 64 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| returns null if currentVersion equals newVersion | 73 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| skips invalid repos | 83 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| works without Github | 92 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| uses GitHub tags | 118 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| filters unnecessary warns | 144 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| supports node engines | 171 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| handles no sourceUrl | 198 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| handles invalid sourceUrl | 207 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| handles missing Github token | 216 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| handles suppressed Github warnings | 226 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| handles no releases | 236 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| handles not enough releases | 245 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| supports github enterprise and github.com changelog | 254 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| supports github enterprise and github enterprise changelog | 285 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| works with same version releases but different prefix | 318 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |

---
