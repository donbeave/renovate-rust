# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/renovate-logger.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/renovate-logger.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `logger/renovate-logger`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws | 6 | not-applicable | — | — | JavaScript `RenovateLogger` Bunyan wrapper initialization behavior; Rust initializes tracing directly and has no uninitialized Bunyan logger object. |
| should queue logs until initialized | 12 | not-applicable | — | — | JavaScript `RenovateLogger` pre-Bunyan queue behavior; Rust initializes tracing directly and does not queue log calls before a Bunyan instance exists. |

### `logger/renovate-logger › before bunyan is initialized`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should log to console | 27 | not-applicable | — | — | JavaScript pre-Bunyan initialization warning behavior; Rust tracing initialization has no equivalent Bunyan-not-initialized console warning. |
| should not log more than once | 36 | not-applicable | — | — | JavaScript pre-Bunyan initialization warning behavior; Rust tracing initialization has no equivalent Bunyan-not-initialized console warning. |

---

