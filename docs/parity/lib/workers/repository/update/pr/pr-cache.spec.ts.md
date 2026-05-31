# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/pr-cache.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/pr-cache.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `workers/repository/update/pr/pr-cache › getPrCache()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return null if cache is empty  | 28 | not-applicable | — | — | Mock framework internals — tests TS-specific PR cache module via vitest-mocked repository cache; Rust tests this at different architecture level |
| return null if prCache is null/undefined  | 33 | not-applicable | — | — | Mock framework internals — tests TS-specific PR cache module via vitest-mocked repository cache; Rust tests this at different architecture level |
| returns prCache  | 38 | not-applicable | — | — | Mock framework internals — tests TS-specific PR cache module via vitest-mocked repository cache; Rust tests this at different architecture level |

### `workers/repository/update/pr/pr-cache › setPrCache()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| logs if branch not found  | 52 | not-applicable | — | — | Mock framework internals — tests TS-specific PR cache module via vitest-mocked repository cache; Rust tests this at different architecture level |
| updates cache  | 61 | not-applicable | — | — | Mock framework internals — tests TS-specific PR cache module via vitest-mocked repository cache; Rust tests this at different architecture level |
| does not update details if pr not modified  | 78 | not-applicable | — | — | Mock framework internals — tests TS-specific PR cache module via vitest-mocked repository cache; Rust tests this at different architecture level |

---

