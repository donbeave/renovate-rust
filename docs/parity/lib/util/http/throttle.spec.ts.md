# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/throttle.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/throttle.spec.ts
**Total tests:** 2 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/http/throttle`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid URL | 14 | not-applicable | — | — | TypeScript `got`-based per-hostname maxRequestsPerSecond throttle; Rust uses reqwest without this abstraction |
| returns throttle for valid url | 18 | not-applicable | — | — | TypeScript `got`-based per-hostname maxRequestsPerSecond throttle; Rust uses reqwest without this abstraction |

---

