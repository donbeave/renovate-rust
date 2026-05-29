# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/utils.spec.ts
**Total tests:** 8 | **Ported:** 2 | **Actionable:** 8 | **Status:** partial

### `logger/utils`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sanitizeValue("$input") == "$output" | 11 | ported | — | — | — |
| sanitizes boxed String objects as strings | 26 | pending | — | — | —|
| preserves secret template strings in redacted fields | 39 | ported | `util.rs` | `test_sanitize_value_preserves_secret_templates` | — |

### `logger/utils › prepareError`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| prepareZodIssues | 90 | pending | — | — | —|
| prepareError | 178 | pending | — | — | —|
| handles HTTP timout error | 203 | pending | — | — | —|
| handles rawExec error | 219 | pending | — | — | —|
| handles AggregateError | 232 | pending | — | — | —|

---

