# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/git/set-branch-commit.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/git/set-branch-commit.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable-applicable

### `util/git/set-branch-commit › setBranchCommit`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sets new branch in cache if it does not exist | 21 | not-applicable | — | — | Tests vi.mock'd repo cache (getCache) + git.getBranchCommit; Rust uses typed cache with no shared mutable global |
| sets new values in branch when old state exists | 42 | not-applicable | — | — | Tests vi.mock'd repo cache with partial<BranchCache> mock objects |
| sets commitTimestamp when DateTime is provided | 74 | not-applicable | — | — | Tests vi.mock'd repo cache + luxon DateTime integration |

---
