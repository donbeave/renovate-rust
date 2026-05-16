# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/uniq.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/uniq.spec.ts
**Total tests:** 2 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/uniq`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return an array with unique elements | 4 | not-applicable | — | — | Renovate's TypeScript array de-duplication helper is not implemented as a shared Rust API; Rust call sites use standard collection logic. |
| should use the provided equality function to compare elements | 10 | not-applicable | — | — | Renovate's TypeScript array de-duplication helper with custom comparator is not implemented as a Rust API. |

---

