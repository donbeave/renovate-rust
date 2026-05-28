# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/platform/comment.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/comment.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 10 | **Status:** not-applicable

### `modules/platform/comment › ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| caches created comment | 20 | not-applicable | — | — | Uses platform mock + vi.mock(repoCache); platform/cache mock infrastructure not portable to Rust |
| caches comment with no topic | 39 | not-applicable | — | — | Uses platform mock + vi.mock(repoCache); platform/cache mock infrastructure not portable to Rust |
| does not cache failed comment | 58 | not-applicable | — | — | Uses platform mock + vi.mock(repoCache); platform/cache mock infrastructure not portable to Rust |
| short-circuits if comment already exists | 71 | not-applicable | — | — | Uses platform mock + vi.mock(repoCache); platform/cache mock infrastructure not portable to Rust |
| rewrites content hash | 80 | not-applicable | — | — | Uses platform mock + vi.mock(repoCache); platform/cache mock infrastructure not portable to Rust |
| caches comments many comments with different topics | 96 | not-applicable | — | — | Uses platform mock + vi.mock(repoCache); platform/cache mock infrastructure not portable to Rust |

### `modules/platform/comment › ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes cached comment by topic | 123 | not-applicable | — | — | Uses platform mock + vi.mock(repoCache); platform/cache mock infrastructure not portable to Rust |
| deletes cached comment by content | 131 | not-applicable | — | — | Uses platform mock + vi.mock(repoCache); platform/cache mock infrastructure not portable to Rust |
| deletes by content only one comment | 143 | not-applicable | — | — | Uses platform mock + vi.mock(repoCache); platform/cache mock infrastructure not portable to Rust |
| deletes only for selected PR | 160 | not-applicable | — | — | Uses platform mock + vi.mock(repoCache); platform/cache mock infrastructure not portable to Rust |

---

