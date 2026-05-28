# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/once.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/once.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 9 | **Status:** not-applicable

### `logger/once › core`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should call a function only once | 15 | not-applicable | — | — | Requires vi.fn() mock for call counting |
| supports support distinct calls | 28 | not-applicable | — | — | Requires vi.fn() mock for call counting |
| resets keys | 44 | not-applicable | — | — | Requires vi.fn() mock for call counting |

### `logger/once › logger`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| logs once per function call | 60 | not-applicable | — | — | Requires vi.spyOn(logger) + vi.unmock + logger init |
| distincts between log levels | 73 | not-applicable | — | — | Requires vi.spyOn(logger) + vi.unmock + logger init |
| distincts between different log statements | 89 | not-applicable | — | — | Requires vi.spyOn(logger) + vi.unmock + logger init |
| parameters are taken into account when de-duplicating calls | 106 | not-applicable | — | — | Requires vi.spyOn(logger) + vi.unmock + logger init |
| allows mixing single-time and regular logging | 124 | not-applicable | — | — | Requires vi.spyOn(logger) + vi.unmock + logger init |
| supports reset method | 146 | not-applicable | — | — | Requires vi.spyOn(logger) + vi.unmock + logger init |

---

