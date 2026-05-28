# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/libyear.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/libyear.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 5 | **Status:** not-applicable

### `workers/repository/process/libyear › calculateLibYears`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns early if no packageFiles | 14 | not-applicable | — | — | Uses vi.mock(instrumentation/reporting) and logger spy assertions; not portable |
| calculates libYears | 19 | not-applicable | — | — | Uses vi.mock(instrumentation/reporting) and logger spy assertions; not portable |
| skips disabled dependencies when calculating libYears | 144 | not-applicable | — | — | Uses vi.mock(instrumentation/reporting) and logger spy assertions; not portable |
| de-duplicates if same dep found in different files | 225 | not-applicable | — | — | Uses vi.mock(instrumentation/reporting) and logger spy assertions; not portable |
| ignores disabled dependencies | 304 | not-applicable | — | — | Uses vi.mock(instrumentation/reporting) and logger spy assertions; not portable |

---

