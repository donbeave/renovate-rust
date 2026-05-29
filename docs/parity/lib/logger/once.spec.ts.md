# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/once.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/once.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 3 | **Status:** pending

### `logger/once › core`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call a function only once | 15 | pending | — | — | —|
| supports support distinct calls | 28 | pending | — | — | —|
| resets keys | 44 | pending | — | — | —|

### `logger/once › logger`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| logs once per function call | 60 | not-applicable | — | — | mocking framework internals — tests logger.debug spy called once via once() mechanism |
| distincts between log levels | 73 | not-applicable | — | — | mocking framework internals — tests logger spy for each log level |
| distincts between different log statements | 89 | not-applicable | — | — | mocking framework internals — tests logger spy for distinct log calls |
| parameters are taken into account when de-duplicating calls | 106 | not-applicable | — | — | mocking framework internals — tests logger spy deduplication with different params |
| allows mixing single-time and regular logging | 124 | not-applicable | — | — | mocking framework internals — tests logger spy behavior mixing once/regular calls |
| supports reset method | 146 | not-applicable | — | — | mocking framework internals — tests logger spy after reset() |

---

