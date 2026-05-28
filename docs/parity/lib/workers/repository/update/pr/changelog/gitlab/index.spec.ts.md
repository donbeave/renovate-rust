# Renovate Test Detail

[Back to test map](../../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/changelog/gitlab/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/changelog/gitlab/index.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 16 | **Status:** done

### `workers/repository/update/pr/changelog/gitlab/index › getChangeLogJSON`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if @types | 54 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| returns null if currentVersion equals newVersion | 63 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| skips invalid repos | 73 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| works without GitLab | 82 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| uses GitLab tags | 108 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| handles empty GitLab tags response | 151 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| uses GitLab tags with error | 187 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| handles no sourceUrl | 223 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| handles invalid sourceUrl | 232 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| handles no releases | 241 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| handles not enough releases | 250 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| supports gitlab enterprise and gitlab enterprise changelog | 259 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| supports self-hosted gitlab changelog | 291 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |

### `workers/repository/update/pr/changelog/gitlab/index › hasValidRepository`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles invalid repository | 326 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |
| handles valid repository | 330 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |

### `workers/repository/update/pr/changelog/gitlab/index › getAllTags`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles endpoint | 337 | not-applicable | — | — | Requires httpMock + vi.mock datasource mock infrastructure |

---

