# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/utils.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `logger/utils`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sanitizeValue("$input") == "$output" | 11 | not-applicable | — | — | JavaScript logger value sanitizer for Bunyan structured fields; Rust tracing layer does not expose equivalent arbitrary JS value sanitation. |
| sanitizes boxed String objects as strings | 26 | not-applicable | — | — | JavaScript boxed `String` handling for Bunyan structured fields; Rust has no boxed JS string value model. |
| preserves secret template strings in redacted fields | 39 | not-applicable | — | — | JavaScript logger value sanitizer for config-shaped objects; Rust logging layer does not serialize arbitrary config objects through this sanitizer. |

### `logger/utils › prepareError`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| prepareZodIssues | 90 | not-applicable | — | — | JavaScript Zod error formatting for logger output; Rust uses typed errors and has no Zod error model. |
| prepareError | 178 | not-applicable | — | — | JavaScript Zod error formatting for logger output; Rust uses typed errors and has no Zod error model. |
| handles HTTP timout error | 203 | not-applicable | — | — | JavaScript got `TimeoutError` serialization for logger output; Rust HTTP errors use Rust error types. |
| handles rawExec error | 219 | not-applicable | — | — | JavaScript `ExecError` serialization for logger output; Rust CLI does not expose Renovate's JS rawExec error object. |
| handles AggregateError | 232 | not-applicable | — | — | JavaScript `AggregateError` serialization for logger output; Rust uses typed error chains instead of JS aggregate error objects. |

---

