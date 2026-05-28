# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/package-rules/current-version.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/package-rules/current-version.spec.ts
**Total tests:** 10 | **Ported:** 9 | **Actionable:** 10 | **Status:** partial

### `util/package-rules/current-version › match`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true for null versioning | 8 | ported | `package_rule.rs` | `current_version_matcher_returns_true_for_null_versioning_equivalent` | — |
| return false on version exception | 22 | not-applicable | — | — | mocking framework internals; tests vi.spyOn/mockImplementationOnce |
| return true for a valid match | 39 | ported | `package_rule.rs` | `current_version_matcher_pep440_four_component_range` | — |
| return false if no version could be found | 52 | ported | `package_rule.rs` | `current_version_matcher_returns_false_if_no_version_found` | — |
| case insensitive match | 66 | ported | `package_rule.rs` | `current_version_matcher_regex_is_case_insensitive` | — |
| return false for regex version non match | 79 | ported | `package_rule.rs` | `current_version_matcher_returns_false_for_regex_version_non_match` | — |
| return true for regex version match | 93 | ported | `package_rule.rs` | `current_version_matcher_returns_true_for_regex_version_match` | — |
| return false for regex value match | 107 | ported | `package_rule.rs` | `current_version_matcher_returns_false_for_regex_value_match_without_version` | — |
| return true for same-major verisioning if version lies in expected range | 120 | ported | `package_rule.rs` | `current_version_matcher_same_major_in_range` | — |
| return false for same-major verisioning if version lies outside of expected range | 133 | ported | `package_rule.rs` | `current_version_matcher_same_major_out_of_range` | — |

---

