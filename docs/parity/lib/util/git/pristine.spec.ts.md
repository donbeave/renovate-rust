# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/git/pristine.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/git/pristine.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/git/pristine › getCachedPristineResult`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if cache is not populated | 18 | not-applicable | — | — | Tests vi.mock'd repo cache (getCache) returning mock data; Rust uses typed serde cache, no shared mutable global |
| returns false if branch not found | 22 | not-applicable | — | — | Tests vi.mock'd repo cache with partial<BranchCache> mock objects |
| returns true | 27 | not-applicable | — | — | Tests vi.mock'd repo cache with partial<BranchCache> mock objects |

---
