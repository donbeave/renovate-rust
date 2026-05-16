# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/date.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/date.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/date › getElapsedDays › by default`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns elapsed days | 22 | not-applicable | — | — | Renovate's generic elapsed-time helper is not implemented as a shared Rust API; Rust date arithmetic is local to feature modules. |
| returns floor'd version of floating point when partial days | 27 | not-applicable | — | — | Renovate's generic elapsed-time helper is not implemented as a shared Rust API. |

### `util/date › getElapsedDays › when floor=false`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns floating point when partial days | 34 | not-applicable | — | — | Renovate's generic elapsed-time helper is not implemented as a shared Rust API. |
| returns all decimal places | 39 | not-applicable | — | — | Renovate's generic elapsed-time helper is not implemented as a shared Rust API. |

### `util/date › getElapsedMinutes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns elapsed minutes | 47 | not-applicable | — | — | Renovate's generic elapsed-time helper is not implemented as a shared Rust API. |

### `util/date › getElapsedHours`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns elapsed hours | 54 | not-applicable | — | — | Renovate's generic elapsed-time helper is not implemented as a shared Rust API. |
| returns zero when date passed is invalid | 60 | not-applicable | — | — | Renovate's JavaScript invalid-date fallback behavior has no shared Rust API equivalent. |

### `util/date › getElapsedMs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns elapsed time in milliseconds | 66 | not-applicable | — | — | Renovate's generic elapsed-time helper is not implemented as a shared Rust API. |

---

