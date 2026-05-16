# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/config/inherit.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/inherit.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/inherit`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| all values in OPTIONS are sorted | 4 | ported | `config.rs` | `inherit_config_options_are_sorted` | — |

### `config/inherit › InheritConfig.get()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return NOT_PRESENT if key is not set | 15 | ported | `config.rs` | `inherit_config_returns_not_present_for_missing_key` | — |
| return value if key is set | 20 | ported | `config.rs` | `inherit_config_returns_value_when_key_is_set` | — |

---

