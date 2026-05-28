# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/err-serializer.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/err-serializer.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 5 | **Status:** not-applicable

### `logger/err-serializer`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| expands errors | 9 | not-applicable | — | — | Tests JS object spread `{ ...err }` behavior; Rust uses structured error types, no equivalent serializer |
| handles missing fields | 40 | not-applicable | — | — | Same: JS-specific property spread/enumeration behavior |

### `logger/err-serializer › got`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles http error | 66 | not-applicable | — | — | Requires HTTP mock + got/RequestError object inspection; Rust HTTP errors are typed structs |
| sanitize http error | 83 | not-applicable | — | — | Requires HTTP mock + Bunyan sanitizeValue; Rust logging uses tracing, no equivalent |
| handles AggregateErrors | 113 | not-applicable | — | — | JavaScript AggregateError type; Rust error model has no direct equivalent |

---
