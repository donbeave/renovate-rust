# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/cache/repository/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/cache/repository/index.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 5 | **Status:** not-applicable

### `util/cache/repository/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if cache not enabled | 22 | not-applicable | — | — | Uses vi.mock(fs) filesystem mock and spy assertions; not portable |
| saves cache | 29 | not-applicable | — | — | Uses vi.mock(fs) filesystem mock and spy assertions; not portable |
| skips saves cache on dry run | 36 | not-applicable | — | — | Uses vi.mock(fs) filesystem mock and spy assertions; not portable |
| resets cache | 48 | not-applicable | — | — | Uses vi.mock(fs) filesystem mock and spy assertions; not portable |
| prints repository problems | 56 | not-applicable | — | — | Uses logger.getProblems.mockReturnValueOnce / logger.debug spy; not portable |

---

