# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/date.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/date.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 8 | **Status:** ported

### `util/date › getElapsedDays › by default`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns elapsed days | 22 | ported | `util.rs` | `test_get_elapsed_days_exact` | — |
| returns floor'd version of floating point when partial days | 27 | ported | `util.rs` | `test_get_elapsed_days_floor_partial` | — |

### `util/date › getElapsedDays › when floor=false`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns floating point when partial days | 34 | ported | `util.rs` | `test_get_elapsed_days_no_floor` | — |
| returns all decimal places | 39 | ported | `util.rs` | `test_get_elapsed_days_decimal` | — |

### `util/date › getElapsedMinutes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns elapsed minutes | 47 | ported | `util.rs` | `test_get_elapsed_minutes` | — |

### `util/date › getElapsedHours`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns elapsed hours | 54 | ported | `util.rs` | `test_get_elapsed_hours` | — |
| returns zero when date passed is invalid | 60 | ported | `util.rs` | `test_get_elapsed_hours_invalid` | — |

### `util/date › getElapsedMs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns elapsed time in milliseconds | 66 | ported | `util.rs` | `test_get_elapsed_ms` | — |

---

