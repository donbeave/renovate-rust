# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/package-rules/new-value.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/package-rules/new-value.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `util/package-rules/new-value › match`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return true for exact match | 7 | ported | `package_rule.rs` | `new_value_matcher_returns_true_for_exact_match` | — |
| return true for glob match | 19 | ported | `package_rule.rs` | `new_value_matcher_returns_true_for_glob_match` | — |
| return false for glob non match | 31 | ported | `package_rule.rs` | `new_value_matcher_returns_false_for_glob_non_match` | — |
| return false for regex version non match | 43 | ported | `package_rule.rs` | `new_value_matcher_returns_false_for_regex_version_non_match` | — |
| case insensitive match | 55 | ported | `package_rule.rs` | `new_value_matcher_is_case_insensitive_for_i_regex_flag` | — |
| return true for regex version match | 67 | ported | `package_rule.rs` | `new_value_matcher_returns_true_for_regex_version_match` | — |
| return false for now value | 79 | ported | `package_rule.rs` | `new_value_matcher_returns_false_for_missing_value` | — |

---

