# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/homebrew/update.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/homebrew/update.spec.ts
**Total tests:** 19 | **Ported:** 0 | **Actionable:** 19 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates "releases" github dependency | 14 | not-applicable | — | — | Requires httpMock + vi.mock fs/git mock infrastructure |
| updates "archive" github dependency | 49 | not-applicable | — | — | Requires httpMock + vi.mock fs/git mock infrastructure |
| updates "archive" github dependency from old url format | 86 | not-applicable | — | — | Requires httpMock + vi.mock fs/git mock infrastructure |
| returns unchanged content if fromStream promise rejects | 132 | not-applicable | — | — | Requires httpMock + vi.mock fs/git mock infrastructure |
| returns unchanged content if url field in upgrade object is invalid | 165 | not-applicable | — | — | Requires httpMock + vi.mock fs/git mock infrastructure |
| returns unchanged content if repoName in upgrade object is invalid | 190 | not-applicable | — | — | Requires httpMock + vi.mock fs/git mock infrastructure |
| returns unchanged content if repoName in upgrade object is wrong | 215 | not-applicable | — | — | Requires httpMock + vi.mock fs/git mock infrastructure |
| returns unchanged content if url field in Formula file is invalid | 240 | not-applicable | — | — | Requires httpMock + vi.mock fs/git mock infrastructure |
| returns unchanged content if url field in Formula file is missing | 280 | not-applicable | — | — | Requires httpMock + vi.mock fs/git mock infrastructure |
| returns unchanged content if sha256 field in Formula file is invalid | 319 | not-applicable | — | — | Requires httpMock + vi.mock fs/git mock infrastructure |
| returns unchanged content if sha256 field in Formula file is missing | 359 | not-applicable | — | — | Requires httpMock + vi.mock fs/git mock infrastructure |
| returns unchanged content if both got requests fail | 398 | not-applicable | — | — | Requires httpMock + vi.mock fs/git mock infrastructure |
| returns unchanged content if managerData is missing required fields | 429 | not-applicable | — | — | Requires httpMock + vi.mock fs/git mock infrastructure |
| returns unchanged content for unknown handler type | 452 | not-applicable | — | — | Requires httpMock + vi.mock fs/git mock infrastructure |
| returns unchanged content if newValue is missing | 476 | not-applicable | — | — | Requires httpMock + vi.mock fs/git mock infrastructure |
| returns unchanged content if handler buildArchiveUrls returns null | 500 | not-applicable | — | — | Requires httpMock + vi.mock fs/git mock infrastructure |
| updates npm scoped package dependency | 542 | not-applicable | — | — | Requires httpMock + vi.mock fs/git mock infrastructure |
| updates npm unscoped package dependency | 586 | not-applicable | — | — | Requires httpMock + vi.mock fs/git mock infrastructure |
| returns unchanged content if npm tarball download fails | 630 | not-applicable | — | — | Requires httpMock + vi.mock fs/git mock infrastructure |

---

