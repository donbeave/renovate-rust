# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/extract/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/extract/index.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 5 | **Status:** not-applicable

### `workers/repository/extract/index › extractAllDependencies()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| runs | 23 | not-applicable | — | — | Uses vi.mock manager-files + scm mock; module mock infrastructure not portable |
| skips non-enabled managers | 32 | not-applicable | — | — | Uses vi.mock manager-files + scm mock; module mock infrastructure not portable |
| warns if no packages found for a enabled manager | 43 | not-applicable | — | — | Uses vi.mock manager-files + scm mock; module mock infrastructure not portable |
| warns if packageFiles is null | 54 | not-applicable | — | — | Uses vi.mock manager-files + scm mock; module mock infrastructure not portable |
| checks custom managers | 60 | not-applicable | — | — | Uses vi.mock manager-files + scm mock; module mock infrastructure not portable |

---

