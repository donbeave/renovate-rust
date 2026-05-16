# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/string-match.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/string-match.spec.ts
**Total tests:** 25 | **Ported:** 25 | **Actionable:** 25 | **Status:** ported

### `util/string-match › matchRegexOrGlobList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if empty patterns | 10 | ported | `string_match.rs` | `string_match_spec_empty_patterns_returns_false` | — |
| returns false if no match | 14 | ported | `string_match.rs` | `string_match_spec_no_match_returns_false` | — |
| returns true if star | 18 | ported | `string_match.rs` | `string_match_spec_star_returns_true` | — |
| returns true if any match | 22 | ported | `string_match.rs` | `string_match_spec_any_positive_match_returns_true` | — |
| returns true if one match with negative patterns | 26 | ported | `string_match.rs` | `string_match_spec_one_negative_pattern_returns_true` | — |
| returns true if every match with negative patterns | 30 | ported | `string_match.rs` | `string_match_spec_every_negative_regex_returns_true` | — |
| returns true if matching positive and negative patterns | 34 | ported | `string_match.rs` | `negative_regex_positive_pattern_returns_true` | — |
| returns true case insensitive for glob | 38 | ported | `string_match.rs` | `glob_is_case_insensitive_matching_renovate_nocase` | — |
| returns true if matching every negative pattern (regex) | 42 | ported | `string_match.rs` | `negative_regex_positive_pattern_allows_all_non_matches` | — |
| returns false if not matching every negative pattern (regex) | 48 | ported | `string_match.rs` | `all_negative_patterns_both_must_not_match` | — |
| returns true if matching every negative pattern (glob) | 52 | ported | `string_match.rs` | `negative_glob_positive_pattern_returns_true` | — |
| returns false if not matching every negative pattern (glob) | 58 | ported | `string_match.rs` | `all_negative_patterns_both_must_not_match_glob` | — |

### `util/string-match › anyMatchRegexOrGlobList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if empty patterns | 64 | ported | `string_match.rs` | `any_match_empty_patterns_returns_false` | — |
| returns false if empty inputs | 68 | ported | `string_match.rs` | `any_match_empty_inputs_returns_false` | — |
| returns true if both empty | 72 | ported | `string_match.rs` | `any_match_both_empty_returns_false` | — |
| returns true if any match with positive | 76 | ported | `string_match.rs` | `any_match_positive_list_matches` | — |
| returns true if any match with negative | 80 | ported | `string_match.rs` | `any_match_negative_list_matches_non_excluded` | — |

### `util/string-match › getRegexPredicate()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| allows valid regex pattern | 86 | ported | `string_match.rs` | `get_regex_predicate_allows_valid_regex_pattern` | — |
| invalidates invalid regex pattern | 90 | ported | `string_match.rs` | `get_regex_predicate_invalidates_invalid_regex_pattern` | — |
| allows the i flag in regex pattern | 94 | ported | `string_match.rs` | `get_regex_predicate_allows_i_flag` | — |
| allows negative regex pattern | 98 | ported | `string_match.rs` | `get_regex_predicate_allows_negative_regex_pattern` | — |
| does not allow non-regex input | 102 | ported | `string_match.rs` | `get_regex_predicate_rejects_non_regex_input` | — |

### `util/string-match › matchRegexOrGlob()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true if positive regex pattern matched | 108 | ported | `string_match.rs` | `match_regex_or_glob_positive_regex_pattern_matched` | — |
| returns true if negative regex is not matched | 112 | ported | `string_match.rs` | `match_regex_or_glob_negative_regex_not_matched_returns_true` | — |
| returns false if negative pattern is matched | 116 | ported | `string_match.rs` | `match_regex_or_glob_negative_pattern_matched_returns_false` | — |

---

