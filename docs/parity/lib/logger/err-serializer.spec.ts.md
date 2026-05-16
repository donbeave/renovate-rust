# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/err-serializer.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/err-serializer.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `logger/err-serializer`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| expands errors | 9 | not-applicable | — | — | Bunyan JavaScript error serializer hook; Rust tracing/error types are not serialized through Renovate's JS `errSerializer`. |
| handles missing fields | 40 | not-applicable | — | — | Bunyan JavaScript error serializer hook; Rust tracing/error types are not serialized through Renovate's JS `errSerializer`. |

### `logger/err-serializer › got`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles http error | 66 | not-applicable | — | — | JavaScript got HTTP error serialization for Bunyan output; Rust HTTP errors use typed Rust error enums instead of got error objects. |
| sanitize http error | 83 | not-applicable | — | — | JavaScript got HTTP error sanitation for Bunyan output; Rust logging layer does not expose the JS sanitizer or got error object model. |
| handles AggregateErrors | 113 | not-applicable | — | — | JavaScript `AggregateError` serialization for Bunyan output; Rust uses typed error chains instead of JS aggregate error objects. |

---

