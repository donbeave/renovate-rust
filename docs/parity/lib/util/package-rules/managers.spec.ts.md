# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/package-rules/managers.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/package-rules/managers.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 0 | **Status:** done

### `util/package-rules/managers › match`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true | 7 | ported | `package_rule.rs` | `managers_matcher_returns_true_for_matching_manager` | — |
| should return false for no match | 19 | ported | `package_rule.rs` | `managers_matcher_returns_false_for_no_match` | — |
| should return null if matchManagers is undefined | 31 | ported | `package_rule.rs` | `managers_matcher_without_patterns_is_not_a_constraint` | Rust matcher uses `true` to represent "no constraint"; the TypeScript matcher returns `null` before the package-rule combiner skips it |
| should return false if no manager | 41 | ported | `package_rule.rs` | `managers_matcher_returns_false_if_no_manager` | — |
| should match custom managers | 51 | ported | `package_rule.rs` | `managers_matcher_matches_custom_managers` | — |

---

