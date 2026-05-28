# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/changelog/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/changelog/index.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** done

### `workers/repository/update/pr/changelog/index › getChangeLogJSON`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if @types | 56 | not-applicable | — | — | Requires vi.mock release-notes/datasource mock infrastructure |
| handles unsupported changelog source | 65 | not-applicable | — | — | Requires vi.mock release-notes/datasource mock infrastructure |
| returns null if no currentVersion | 74 | not-applicable | — | — | Requires vi.mock release-notes/datasource mock infrastructure |
| returns null if currentVersion equals newVersion | 83 | not-applicable | — | — | Requires vi.mock release-notes/datasource mock infrastructure |
| skips invalid repos | 93 | not-applicable | — | — | Requires vi.mock release-notes/datasource mock infrastructure |
| works without Github | 102 | not-applicable | — | — | Requires vi.mock release-notes/datasource mock infrastructure |
| uses GitHub tags | 140 | not-applicable | — | — | Requires vi.mock release-notes/datasource mock infrastructure |
| filters unnecessary warns | 176 | not-applicable | — | — | Requires vi.mock release-notes/datasource mock infrastructure |
| supports node engines | 206 | not-applicable | — | — | Requires vi.mock release-notes/datasource mock infrastructure |
| handles no sourceUrl | 236 | not-applicable | — | — | Requires vi.mock release-notes/datasource mock infrastructure |
| handles invalid sourceUrl | 245 | not-applicable | — | — | Requires vi.mock release-notes/datasource mock infrastructure |
| handles missing Github token | 254 | not-applicable | — | — | Requires vi.mock release-notes/datasource mock infrastructure |
| handles no releases | 264 | not-applicable | — | — | Requires vi.mock release-notes/datasource mock infrastructure |
| handles not enough releases | 273 | not-applicable | — | — | Requires vi.mock release-notes/datasource mock infrastructure |
| will call getInRangeReleases when releases is undefined | 282 | not-applicable | — | — | Requires vi.mock release-notes/datasource mock infrastructure |
| supports github enterprise and github.com changelog | 291 | not-applicable | — | — | Requires vi.mock release-notes/datasource mock infrastructure |
| supports github enterprise and github enterprise changelog | 325 | not-applicable | — | — | Requires vi.mock release-notes/datasource mock infrastructure |
| supports github.com and github enterprise changelog | 364 | not-applicable | — | — | Requires vi.mock release-notes/datasource mock infrastructure |

---
