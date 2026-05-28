# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/limits.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/limits.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 7 | **Status:** not-applicable

### `workers/repository/process/limits › getPrHourlyCount()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| calculates hourly pr count | 27 | not-applicable | — | — | Uses platform mock + scm mock + vi.mock(repositoryCache); platform/scm/cache mock infrastructure not portable to Rust |
| returns zero if errored | 45 | not-applicable | — | — | Uses platform mock + scm mock + vi.mock(repositoryCache); platform/scm/cache mock infrastructure not portable to Rust |

### `workers/repository/process/limits › getCommitHourlyCount()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| calculates hourly commit count from SCM | 53 | not-applicable | — | — | Uses platform mock + scm mock + vi.mock(repositoryCache); platform/scm/cache mock infrastructure not portable to Rust |
| uses cache when available and falls back to SCM when missing | 68 | not-applicable | — | — | Uses platform mock + scm mock + vi.mock(repositoryCache); platform/scm/cache mock infrastructure not portable to Rust |
| returns zero if errored | 106 | not-applicable | — | — | Uses platform mock + scm mock + vi.mock(repositoryCache); platform/scm/cache mock infrastructure not portable to Rust |

### `workers/repository/process/limits › getConcurrentPrsCount()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| calculates concurrent prs present | 116 | not-applicable | — | — | Uses platform mock + scm mock + vi.mock(repositoryCache); platform/scm/cache mock infrastructure not portable to Rust |

### `workers/repository/process/limits › getConcurrentBranchesCount()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| calculates concurrent branches present | 137 | not-applicable | — | — | Uses platform mock + scm mock + vi.mock(repositoryCache); platform/scm/cache mock infrastructure not portable to Rust |

---

