# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/package-rules/repositories.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/package-rules/repositories.spec.ts
**Total tests:** 15 | **Ported:** 15 | **Actionable:** 15 | **Status:** ported

### `util/package-rules/repositories › match`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null if match repositories is not defined | 7 | ported | `package_rule.rs` | `repositories_matcher_without_patterns_is_not_a_constraint` | Rust matcher uses `true` to represent "no constraint"; the TypeScript matcher returns `null` before the package-rule combiner skips it |
| should return false if repository is not defined | 19 | ported | `package_rule.rs` | `repositories_matcher_returns_false_if_repository_is_missing` | — |
| should return true if repository matches regex pattern | 31 | ported | `package_rule.rs` | `repositories_matcher_returns_true_for_regex_pattern` | — |
| should return false if repository has invalid regex pattern | 43 | ported | `package_rule.rs` | `repositories_matcher_returns_false_for_invalid_regex_pattern` | — |
| should return false if repository does not match regex pattern | 55 | ported | `package_rule.rs` | `repositories_matcher_returns_false_for_non_matching_regex_pattern` | — |
| should return true if repository matches minimatch pattern | 67 | ported | `package_rule.rs` | `repositories_matcher_returns_true_for_minimatch_pattern` | — |
| should return false if repository does not match minimatch pattern | 79 | ported | `package_rule.rs` | `repositories_matcher_returns_false_for_non_matching_minimatch_pattern` | — |
| should return true if repository matches at least one pattern | 91 | ported | `package_rule.rs` | `repositories_matcher_returns_true_if_any_pattern_matches` | — |

### `util/package-rules/repositories › excludes`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return false if exclude repository is not defined | 105 | ported | `package_rule.rs` | `repositories_matcher_returns_false_if_exclude_repository_is_missing` | — |
| should return false if exclude repository matches regex pattern | 117 | ported | `package_rule.rs` | `repositories_matcher_returns_false_if_exclude_regex_matches` | — |
| should return true if exclude repository has invalid regex pattern | 129 | ported | `package_rule.rs` | `repositories_matcher_returns_true_if_exclude_regex_is_invalid` | — |
| should return true if exclude repository does not match regex pattern | 141 | ported | `package_rule.rs` | `repositories_matcher_returns_true_if_exclude_regex_does_not_match` | — |
| should return false if exclude repository matches minimatch pattern | 153 | ported | `package_rule.rs` | `repositories_matcher_returns_false_if_exclude_minimatch_matches` | — |
| should return true if exclude repository does not match minimatch pattern | 165 | ported | `package_rule.rs` | `repositories_matcher_returns_true_if_exclude_minimatch_does_not_match` | — |
| should return false if exclude repository matches at least one pattern | 177 | ported | `package_rule.rs` | `repositories_matcher_returns_false_if_any_exclude_pattern_matches` | — |

---

