# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/lazy.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/lazy.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/lazy › .getValue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets a value | 5 | not-applicable | — | — | Renovate's TypeScript `Lazy` class is not implemented as a Rust API; Rust call sites use standard lazy initialization primitives directly. |
| caches the value | 13 | not-applicable | — | — | Renovate's TypeScript `Lazy` class is not implemented as a Rust API; Rust call sites use standard lazy initialization primitives directly. |
| throws an error | 21 | not-applicable | — | — | Renovate's TypeScript `Lazy` class error caching behavior has no Rust API equivalent. |
| caches the error | 30 | not-applicable | — | — | Renovate's TypeScript `Lazy` class error caching behavior has no Rust API equivalent. |

### `util/lazy › .hasValue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| has a value | 42 | not-applicable | — | — | Renovate's TypeScript `Lazy` class state inspection is not implemented as a Rust API. |
| does not have a value | 51 | not-applicable | — | — | Renovate's TypeScript `Lazy` class state inspection is not implemented as a Rust API. |

---

