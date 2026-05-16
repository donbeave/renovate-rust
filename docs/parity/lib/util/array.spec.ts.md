# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/array.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/array.spec.ts
**Total tests:** 2 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/array`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| .isNotNullOrUndefined | 4 | not-applicable | — | — | Renovate's TypeScript nullish type-guard helper is not implemented as a Rust API; Rust uses `Option` explicitly. |
| .toArray | 13 | not-applicable | — | — | Renovate's TypeScript value-to-array coercion helper is not implemented as a shared Rust API. |

---

