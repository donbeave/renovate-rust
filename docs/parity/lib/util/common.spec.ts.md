# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/common.spec.ts
**Total tests:** 22 | **Ported:** 20 | **Actionable:** 0 | **Status:** done

### `util/common › detectPlatform`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ("$url") === $hostType | 46 | ported | `util.rs` | `test_detect_platform` | — |
| uses host rules | 67 | ported | `util.rs` | `test_detect_platform_uses_host_rules` | — |

### `util/common › parseJson`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null | 119 | ported | `util.rs` | `test_parse_json_null_for_empty` | — |
| returns parsed json | 123 | ported | `util.rs` | `test_parse_json_valid` | — |
| supports jsonc | 131 | ported | `util.rs` | `test_parse_json_jsonc` | — |
| throws error for invalid json | 149 | ported | `util.rs` | `test_parse_json_invalid` | — |
| catches and warns if content parsing failed with JSONC.parse but not with JSON5.parse | 153 | ported | `util.rs` | `test_parse_json_fallback_warns` | — |
| does not warn if filename ends with .jsonc | 167 | ported | `util.rs` | `test_parse_json_no_warn_jsonc` | — |
| does not warn if filename ends with .json5 | 172 | ported | `util.rs` | `test_parse_json_no_warn_json5` | — |

### `util/common › parseJsonc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns parsed jsonc | 179 | ported | `util.rs` | `test_parse_json_jsonc` | — |
| throws error for invalid jsonc | 187 | ported | `util.rs` | `test_parse_json_invalid` | — |

### `util/common › getInheritedOrGlobal`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns undefined if not set | 198 | ported | `util.rs` | `get_inherited_or_global_returns_none_when_not_set` | — |
| returns inherited value if only inherited value is set | 202 | ported | `util.rs` | `get_inherited_or_global_returns_inherited_when_only_inherited` | — |
| returns global value if only global value is set | 209 | ported | `util.rs` | `get_inherited_or_global_returns_global_when_only_global` | — |
| returns inherited value - when both global + inherited are set | 216 | ported | `util.rs` | `get_inherited_or_global_inherited_wins_when_both_set` | — |
| handles null inherited values | 227 | not-applicable | — | — | TypeScript type-system test; upstream comment says "only for coverage" — config validation prevents null values, and Rust's type system makes this impossible at compile time |
| handles undefined inherited values | 238 | not-applicable | — | — | TypeScript type-system test; upstream comment says "only for coverage" — config validation prevents undefined values, and Rust's type system makes this impossible at compile time |

### `util/common › getInheritedOrGlobal › when requesting onboardingAutoCloseAge, do not allow inherit config to override global config`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns inherited value when inherited < global | 249 | ported | `util.rs` | `get_inherited_or_global_age_inherited_less_than_global` | — |
| returns global value when inherited > global value | 259 | ported | `util.rs` | `get_inherited_or_global_age_inherited_greater_than_global` | — |
| returns inherited value when inherited == global | 269 | ported | `util.rs` | `get_inherited_or_global_age_equal` | — |
| returns inherited value when global value is not set | 279 | ported | `util.rs` | `get_inherited_or_global_age_global_not_set` | — |
| returns global value when inherited value is not set | 289 | ported | `util.rs` | `get_inherited_or_global_age_inherited_not_set` | — |

---
