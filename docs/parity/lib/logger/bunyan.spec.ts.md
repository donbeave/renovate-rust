# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/bunyan.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/bunyan.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `logger/bunyan`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| checks for valid log levels | 6 | ported | `logging.rs` | `parses_all_valid_renovate_levels` (+ `fatal_maps_to_error`) | — |
| checks for invalid log level: $input | 16 | ported | `logging.rs` | `invalid_level_returns_none` | — |

---

