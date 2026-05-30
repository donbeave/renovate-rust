# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/utils.spec.ts
**Total tests:** 8 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `logger/utils`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sanitizeValue("$input") == "$output" | 11 | ported | `util.rs` | `test_sanitize_urls` | — |
| sanitizes boxed String objects as strings | 26 | not-applicable | — | — | TypeScript type-system test; JavaScript boxed String objects (new String(...)) have no Rust equivalent |
| preserves secret template strings in redacted fields | 39 | ported | `util.rs` | `test_sanitize_value_preserves_secret_templates` | — |

### `logger/utils › prepareError`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| prepareZodIssues | 90 | not-applicable | — | — | TS-library-specific; Zod schema error formatting is TypeScript/Zod-specific with no Rust serde equivalent |
| prepareError | 178 | not-applicable | — | — | TypeScript error class hierarchy test; formats got/Zod error types which are TypeScript-specific |
| handles HTTP timout error | 203 | not-applicable | — | — | TypeScript error class hierarchy test; TypeScript got HTTP error type has no direct Rust reqwest equivalent |
| handles rawExec error | 219 | not-applicable | — | — | TypeScript error class hierarchy test; TypeScript exec error format has no direct Rust std::process equivalent |
| handles AggregateError | 232 | not-applicable | — | — | TypeScript type-system test; JavaScript AggregateError class has no Rust equivalent |

---

