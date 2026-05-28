# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/finalize/repository-statistics.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/finalize/repository-statistics.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 4 | **Status:** not-applicable

### `workers/repository/finalize/repository-statistics › runRenovateRepoStats`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Calls runRenovateRepoStats | 41 | not-applicable | — | — | Uses vi.mock(platform), vi.mocked(platform.getPrList) and logger spy assertions; not portable |

### `workers/repository/finalize/repository-statistics › runBranchSummary`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| processes cache with baseBranches only | 63 | not-applicable | — | — | Uses vi.spyOn(cache) and logger.debug spy assertions; not portable |
| processes cache with baseBranches and branches | 94 | not-applicable | — | — | Uses vi.spyOn(cache) and logger.debug spy assertions; not portable |
| logs extended branch info if branchSummaryExtended | 159 | not-applicable | — | — | Uses vi.spyOn(cache) and logger.debug spy assertions; not portable |

---

