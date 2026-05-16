# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/config/defaults.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/defaults.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/defaults › getDefault()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns new instances of arrays when called repeatedly | 6 | ported | `config.rs` | `default_array_values_are_independent` | — |
| returns true for boolean values | 20 | ported | `config.rs` | `default_boolean_value_is_true` | — |
| returns null for %s values | 31 | ported | `config.rs` | `default_scalar_values_are_null` | — |

---

