# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/renovate-logger.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/renovate-logger.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 4 | **Status:** done

### `logger/renovate-logger`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws | 6 | not-applicable | — | — | TypeScript OOP class test; RenovateLogger.childLogger() TypeScript class method has no Rust equivalent |
| should queue logs until initialized | 12 | not-applicable | — | — | mocking framework internals — tests that logs are queued then flushed when bunyan is set (vi.fn() spy on BunyanLogger) |

### `logger/renovate-logger › before bunyan is initialized`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should log to console | 27 | not-applicable | — | — | mocking framework internals — tests vi.spyOn(console, 'warn') console spy |
| should not log more than once | 36 | not-applicable | — | — | mocking framework internals — tests vi.spyOn(console, 'warn') called exactly once |

---

