# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/got.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/got.spec.ts
**Total tests:** 2 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/http/got`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| configures rejectUnauthorized when forced | 15 | not-applicable | — | — | TypeScript `got` library internals; Rust uses reqwest which handles TLS differently |
| does a flat clone of options | 25 | not-applicable | — | — | TypeScript `got` options normalization; no equivalent in reqwest |

---

