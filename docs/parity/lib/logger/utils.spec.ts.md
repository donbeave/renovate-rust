# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/utils.spec.ts
**Total tests:** 8 | **Ported:** 2 | **Actionable:** 8 | **Status:** partial

### `logger/utils`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sanitizeValue("$input") == "$output" | 11 | ported | — | — | — |
| sanitizes boxed String objects as strings | 26 | not-applicable | — | — | JavaScript boxed String objects (`new String(...)`) have no Rust equivalent |
| preserves secret template strings in redacted fields | 39 | ported | `util.rs` | `test_sanitize_value_preserves_secret_templates` | — |

### `logger/utils › prepareError`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| prepareZodIssues | 90 | not-applicable | — | — | Tests Zod-specific validation error formatting; no Zod equivalent in Rust |
| prepareError | 178 | not-applicable | — | — | Tests `ZodError` class which has no Rust equivalent |
| handles HTTP timout error | 203 | not-applicable | — | — | Tests `got` library `TimeoutError` class; no direct Rust equivalent |
| handles rawExec error | 219 | not-applicable | — | — | Tests TypeScript `ExecError` class internals; Rust uses different error types |
| handles AggregateError | 232 | not-applicable | — | — | JavaScript `AggregateError` has no Rust equivalent |

---

