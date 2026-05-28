# Renovate Test Detail

[Back to test map](../../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/changelog/bitbucket-server/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/changelog/bitbucket-server/index.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 14 | **Status:** done

### `workers/repository/update/pr/changelog/bitbucket-server/index › getChangeLogJSON`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses bitbucket-server tags | 52 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| handles empty bitbucket-server tags response | 104 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| uses bitbucket-server tags with error | 137 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |

### `workers/repository/update/pr/changelog/bitbucket-server/index › getReleaseNotesMdFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles release notes | 172 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| handles release notes with sourceDirectory | 191 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| handles missing release notes | 227 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| getReleaseList | 239 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |

### `workers/repository/update/pr/changelog/bitbucket-server/index › source › getBaseUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $sourceUrl | 249 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| getAPIBaseUrl | 267 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| getCompareURL | 271 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |

### `workers/repository/update/pr/changelog/bitbucket-server/index › source › getRepositoryFromUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 284 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |

### `workers/repository/update/pr/changelog/bitbucket-server/index › hasValidRepository`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles invalid repository | 303 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| handles valid repository | 308 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |

### `workers/repository/update/pr/changelog/bitbucket-server/index › getAllTags`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles endpoint | 314 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |

---

