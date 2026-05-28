# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/extract/manager-files.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/extract/manager-files.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 5 | **Status:** not-applicable

### `workers/repository/extract/manager-files › getManagerPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty of manager is disabled | 22 | not-applicable | — | — | Uses vi.mock(file-match/html/fs) infrastructure; not portable |
| returns empty of manager is not enabled | 28 | not-applicable | — | — | Uses vi.mock(file-match/html/fs) infrastructure; not portable |
| skips files if null content returned | 35 | not-applicable | — | — | Uses vi.mock(file-match/html/fs) infrastructure; not portable |
| returns files with extractPackageFile | 46 | not-applicable | — | — | Uses vi.mock(file-match/html/fs) infrastructure; not portable |
| returns files with extractAllPackageFiles | 66 | not-applicable | — | — | Uses vi.mock(file-match/html/fs) infrastructure; not portable |

---

