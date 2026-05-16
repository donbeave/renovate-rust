# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/once.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/once.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `logger/once › core`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call a function only once | 15 | not-applicable | — | — | JavaScript logger callsite de-duplication helper; Rust tracing logger does not expose an equivalent `once` callback/cache API. |
| supports support distinct calls | 28 | not-applicable | — | — | JavaScript logger callsite de-duplication helper; Rust tracing logger does not expose an equivalent `once` callback/cache API. |
| resets keys | 44 | not-applicable | — | — | JavaScript logger callsite de-duplication helper; Rust tracing logger does not expose an equivalent `once` callback/cache API. |

### `logger/once › logger`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| logs once per function call | 60 | not-applicable | — | — | JavaScript `logger.once.*` facade behavior; Rust tracing logger does not expose once-only log methods. |
| distincts between log levels | 73 | not-applicable | — | — | JavaScript `logger.once.*` facade behavior; Rust tracing logger does not expose once-only log methods. |
| distincts between different log statements | 89 | not-applicable | — | — | JavaScript `logger.once.*` facade behavior; Rust tracing logger does not expose once-only log methods. |
| parameters are taken into account when de-duplicating calls | 106 | not-applicable | — | — | JavaScript `logger.once.*` facade behavior; Rust tracing logger does not expose once-only log methods. |
| allows mixing single-time and regular logging | 124 | not-applicable | — | — | JavaScript `logger.once.*` facade behavior; Rust tracing logger does not expose once-only log methods. |
| supports reset method | 146 | not-applicable | — | — | JavaScript `logger.once.*` facade behavior; Rust tracing logger does not expose once-only log methods. |

---

