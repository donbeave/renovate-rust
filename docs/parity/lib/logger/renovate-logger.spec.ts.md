# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/renovate-logger.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/renovate-logger.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 4 | **Status:** not-applicable

### `logger/renovate-logger`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws | 6 | not-applicable | — | — | Uses vi.fn() logger mock infrastructure; not portable |
| should queue logs until initialized | 12 | not-applicable | — | — | Uses vi.fn() logger mock infrastructure; not portable |

### `logger/renovate-logger › before bunyan is initialized`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should log to console | 27 | not-applicable | — | — | Uses vi.spyOn(console) mock; not portable |
| should not log more than once | 36 | not-applicable | — | — | Uses vi.spyOn(console) mock; not portable |

---

