# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/lazy.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/lazy.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `util/lazy › .getValue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets a value | 5 | ported | `util.rs` | `test_lazy_gets_value` | — |
| caches the value | 13 | ported | `util.rs` | `test_lazy_caches_value` | — |
| throws an error | 21 | ported | `util.rs` | `test_lazy_returns_error` | — |
| caches the error | 30 | ported | `util.rs` | `test_lazy_caches_error` | — |

### `util/lazy › .hasValue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| has a value | 42 | ported | `util.rs` | `test_lazy_has_value_after_get` | — |
| does not have a value | 51 | ported | `util.rs` | `test_lazy_no_value_before_get` | — |

---

