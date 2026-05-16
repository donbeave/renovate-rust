# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/platform/comment.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/comment.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/platform/comment › ensureComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| caches created comment | 20 | not-applicable | — | — | Renovate's TypeScript PR comment cache wrapper is not implemented as a Rust API; Rust platform clients do not expose PR comment mutation. |
| caches comment with no topic | 39 | not-applicable | — | — | Renovate's TypeScript PR comment cache wrapper is not implemented as a Rust API; Rust platform clients do not expose PR comment mutation. |
| does not cache failed comment | 58 | not-applicable | — | — | Renovate's TypeScript PR comment cache wrapper is not implemented as a Rust API; Rust platform clients do not expose PR comment mutation. |
| short-circuits if comment already exists | 71 | not-applicable | — | — | Renovate's TypeScript PR comment cache wrapper is not implemented as a Rust API; Rust platform clients do not expose PR comment mutation. |
| rewrites content hash | 80 | not-applicable | — | — | Renovate's TypeScript PR comment content-hash cache is not implemented as a Rust API. |
| caches comments many comments with different topics | 96 | not-applicable | — | — | Renovate's TypeScript PR comment topic cache is not implemented as a Rust API. |

### `modules/platform/comment › ensureCommentRemoval`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes cached comment by topic | 123 | not-applicable | — | — | Renovate's TypeScript PR comment cache removal wrapper is not implemented as a Rust API. |
| deletes cached comment by content | 131 | not-applicable | — | — | Renovate's TypeScript PR comment cache removal wrapper is not implemented as a Rust API. |
| deletes by content only one comment | 143 | not-applicable | — | — | Renovate's TypeScript PR comment cache removal wrapper is not implemented as a Rust API. |
| deletes only for selected PR | 160 | not-applicable | — | — | Renovate's TypeScript PR comment cache removal wrapper is not implemented as a Rust API. |

---

