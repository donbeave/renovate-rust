# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/git/conflicts-cache.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/git/conflicts-cache.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 9 | **Status:** done

### `util/git/conflicts-cache › getCachedConflictResult`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if cache is not populated | 21 | not-applicable | — | — | Requires vi.mock(repository/cache) mock infrastructure |
| returns null if branch cache not found | 27 | not-applicable | — | — | Requires vi.mock(repository/cache) mock infrastructure |
| returns null if base branch SHA has changed | 42 | not-applicable | — | — | Requires vi.mock(repository/cache) mock infrastructure |
| returns null if branch SHA has changed | 57 | not-applicable | — | — | Requires vi.mock(repository/cache) mock infrastructure |
| returns null if isConfliced is undefined | 72 | not-applicable | — | — | Requires vi.mock(repository/cache) mock infrastructure |
| returns true | 86 | not-applicable | — | — | Requires vi.mock(repository/cache) mock infrastructure |

### `util/git/conflicts-cache › setCachedConflictResult`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return without updating value for unpopulated cache | 103 | not-applicable | — | — | Requires vi.mock(repository/cache) mock infrastructure |
| updates value | 108 | not-applicable | — | — | Requires vi.mock(repository/cache) mock infrastructure |
| handles multiple branches | 132 | not-applicable | — | — | Requires vi.mock(repository/cache) mock infrastructure |

---

