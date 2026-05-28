# Renovate Test Detail

[Back to test map](../../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/changelog/forgejo/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/changelog/forgejo/index.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 16 | **Status:** done

### `workers/repository/update/pr/changelog/forgejo/index › getChangeLogJSON`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if @types | 56 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| returns null if currentVersion equals newVersion | 65 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| skips invalid repos | 75 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| works without forgejo | 84 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| uses forgejo tags | 111 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| handles empty forgejo tags response | 224 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| uses forgejo tags with error | 259 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| handles no sourceUrl | 294 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| handles invalid sourceUrl | 303 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| handles no releases | 312 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| handles not enough releases | 321 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| supports self-hosted forgejo changelog | 330 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |

### `workers/repository/update/pr/changelog/forgejo/index › hasValidRepository`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles invalid repository | 367 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| handles valid repository | 372 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |

### `workers/repository/update/pr/changelog/forgejo/index › getAllTags`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles endpoint | 378 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |

### `workers/repository/update/pr/changelog/forgejo/index › getReleaseNotesMd`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 394 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |

---

