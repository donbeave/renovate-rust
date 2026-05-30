# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/git/update-date-cache.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/git/update-date-cache.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/git/update-date-cache › getCachedUpdateDateResult`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if cache is not populated | 22 | not-applicable | — | — | Git update-date cache not implemented |
| returns null if branch not found | 26 | not-applicable | — | — | Git update-date cache not implemented |
| returns null if branchSha is null | 33 | not-applicable | — | — | Git update-date cache not implemented |
| returns null if branch SHA has changed | 40 | not-applicable | — | — | Git update-date cache not implemented |
| returns null if commitTimestamp is not set | 51 | not-applicable | — | — | Git update-date cache not implemented |
| returns cached value | 58 | not-applicable | — | — | Git update-date cache not implemented |

### `util/git/update-date-cache › setCachedUpdateDateResult`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns without updating when cache not populated | 74 | not-applicable | — | — | Git update-date cache not implemented |
| returns without updating when branch not found | 85 | not-applicable | — | — | Git update-date cache not implemented |
| updates commitTimestamp | 101 | not-applicable | — | — | Git update-date cache not implemented |
| handles multiple branches | 116 | not-applicable | — | — | Git update-date cache not implemented |

---

