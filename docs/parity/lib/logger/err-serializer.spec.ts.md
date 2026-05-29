# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/err-serializer.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/err-serializer.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 3 | **Status:** pending

### `logger/err-serializer`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| expands errors | 9 | not-applicable | — | — | TypeScript error object spreading behavior; dynamic property expansion from Error & Record<string, unknown> has no Rust typed equivalent |
| handles missing fields | 40 | not-applicable | — | — | TypeScript error object spreading behavior; partial Error objects with optional fields are TypeScript-specific |

### `logger/err-serializer › got`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles http error | 66 | pending | — | — | —|
| sanitize http error | 83 | pending | — | — | —|
| handles AggregateErrors | 113 | pending | — | — | —|

---
