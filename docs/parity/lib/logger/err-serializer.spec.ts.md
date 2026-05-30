# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/err-serializer.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/err-serializer.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 5 | **Status:** done

### `logger/err-serializer`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| expands errors | 9 | not-applicable | — | — | TypeScript error object spreading behavior; dynamic property expansion from Error & Record<string, unknown> has no Rust typed equivalent |
| handles missing fields | 40 | not-applicable | — | — | TypeScript error object spreading behavior; partial Error objects with optional fields are TypeScript-specific |

### `logger/err-serializer › got`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles http error | 66 | not-applicable | — | — | HTTP mock integration test; requires Http class + errSerializer impl; Rust uses reqwest with different error types |
| sanitize http error | 83 | not-applicable | — | — | HTTP mock + snapshot test; requires Http class + Bunyan sanitize; Rust HTTP error handling uses different serialization |
| handles AggregateErrors | 113 | not-applicable | — | — | JS AggregateError has no Rust equivalent; Rust uses anyhow/thiserror error chains |

---
