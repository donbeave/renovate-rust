# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/http/cache/memory-http-cache-provider.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/cache/memory-http-cache-provider.spec.ts
**Total tests:** 2 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/http/cache/memory-http-cache-provider`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| reuses data with etag | 17 | not-applicable | — | — | TypeScript `got`-based in-memory ETag HTTP cache provider; Rust uses reqwest without this abstraction |
| does not allow cached responses to be mutated | 40 | not-applicable | — | — | TypeScript `got`-based in-memory ETag HTTP cache provider; Rust uses reqwest without this abstraction |

---

