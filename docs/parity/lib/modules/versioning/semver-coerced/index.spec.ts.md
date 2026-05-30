# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/semver-coerced/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/semver-coerced/index.spec.ts
**Total tests:** 53 | **Ported:** 53 | **Actionable:** 0 | **Status:** done

### `modules/versioning/semver-coerced/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true for strictly equal versions | 5 | ported | `semver_coerced.rs` | `equals_returns_true_for_strictly_equal_versions` | — |
| should return true for non-strictly equal versions | 9 | ported | `semver_coerced.rs` | `equals_returns_true_for_non_strictly_equal_versions` | — |
| should return false for non-equal versions | 14 | ported | `semver_coerced.rs` | `equals_returns_false_for_non_equal_versions` | — |
| invalid version | 18 | ported | `semver_coerced.rs` | `equals_returns_false_for_invalid_version` | — |
| should return major version number for strict semver | 24 | ported | `semver_coerced.rs` | `get_major_returns_major_for_strict_semver` | — |
| should return major version number for non-strict semver | 28 | ported | `semver_coerced.rs` | `get_major_returns_major_for_non_strict_semver` | — |
| invalid version | 32 | ported | `semver_coerced.rs` | `get_major_returns_none_for_invalid_version` | — |
| should return minor version number for strict semver | 38 | ported | `semver_coerced.rs` | `get_minor_returns_minor_for_strict_semver` | — |
| should return minor version number for non-strict semver | 42 | ported | `semver_coerced.rs` | `get_minor_returns_minor_for_non_strict_semver` | — |
| invalid version | 46 | ported | `semver_coerced.rs` | `get_minor_returns_none_for_invalid_version` | — |
| getPatch("$version") === $expected | 52 | ported | `semver_coerced.rs` | `get_patch_matches_renovate_semver_coerced_spec` | — |
| should return false for patch updates | 76 | ported | `semver_coerced.rs` | `is_breaking_returns_false_for_patch_updates` | — |
| should return false for minor updates | 80 | ported | `semver_coerced.rs` | `is_breaking_returns_false_for_minor_updates` | — |
| should return true for major updates | 84 | ported | `semver_coerced.rs` | `is_breaking_returns_true_for_major_updates` | — |
| should return true for major updates from v0.x | 88 | ported | `semver_coerced.rs` | `is_breaking_returns_true_for_major_updates_from_v0` | — |
| should return true for major updates within v0.x | 92 | ported | `semver_coerced.rs` | `is_breaking_returns_true_for_major_updates_within_v0` | — |
| should return true for strict semver | 98 | ported | `semver_coerced.rs` | `is_compatible_returns_true_for_strict_semver` | — |
| should return true for non-strict semver | 102 | ported | `semver_coerced.rs` | `is_compatible_returns_true_for_non_strict_semver` | — |
| should return false for non-semver | 106 | ported | `semver_coerced.rs` | `is_compatible_returns_false_for_non_semver` | — |
| should return true for a greater version in strict semver | 112 | ported | `semver_coerced.rs` | `is_greater_than_returns_true_for_greater_strict_semver` | — |
| should return false for lower version in strict semver | 116 | ported | `semver_coerced.rs` | `is_greater_than_returns_false_for_lower_strict_semver` | — |
| should return false if version cannot be coerced | 120 | ported | `semver_coerced.rs` | `is_greater_than_returns_false_if_version_cannot_be_coerced` | — |
| should return true for a lower version in strict semver | 126 | ported | `semver_coerced.rs` | `is_less_than_range_returns_true_for_lower_strict_semver` | — |
| should return false for in-range version in strict semver | 130 | ported | `semver_coerced.rs` | `is_less_than_range_returns_false_for_in_range_strict_semver` | — |
| invalid version | 134 | ported | `semver_coerced.rs` | `is_less_than_range_returns_false_for_invalid_version` | — |
| returns true if naked version | 140 | ported | `semver_coerced.rs` | `is_single_version_returns_true_for_naked_version` | — |
| returns false if equals | 145 | ported | `semver_coerced.rs` | `is_single_version_returns_false_if_equals` | — |
| returns false when not version | 150 | ported | `semver_coerced.rs` | `is_single_version_returns_false_when_not_version` | — |
| isStable("$version") === $expected | 156 | ported | `semver_coerced.rs` | `is_stable_matches_renovate_semver_coerced_spec` | — |
| should return null for non-digit version strings | 179 | ported | `semver_coerced.rs` | `is_valid_returns_false_for_non_digit_version_strings` | — |
| should return null for irregular version strings | 183 | ported | `semver_coerced.rs` | `is_valid_returns_false_for_irregular_version_strings` | — |
| should support strict semver | 187 | ported | `semver_coerced.rs` | `is_valid_supports_strict_semver` | — |
| should treat semver with dash as a valid version | 191 | ported | `semver_coerced.rs` | `is_valid_treats_semver_with_dash_as_valid_version` | — |
| should treat semver without dash as a valid version | 195 | ported | `semver_coerced.rs` | `is_valid_treats_semver_without_dash_as_valid_version` | — |
| should treat ranges as valid versions | 199 | ported | `semver_coerced.rs` | `is_valid_treats_ranges_as_valid_versions` | — |
| should reject github repositories | 205 | ported | `semver_coerced.rs` | `is_valid_rejects_github_repositories` | — |
| should return null for non-digit versions | 215 | ported | `semver_coerced.rs` | `is_version_returns_false_for_non_digit_versions` | — |
| should support strict semver versions | 219 | ported | `semver_coerced.rs` | `is_version_supports_strict_semver_versions` | — |
| should support non-strict versions | 223 | ported | `semver_coerced.rs` | `is_version_supports_non_strict_versions` | — |
| should return true when version is in range | 229 | ported | `semver_coerced.rs` | `matches_returns_true_when_version_is_in_range` | — |
| should return true with non-strict version in range | 233 | ported | `semver_coerced.rs` | `matches_returns_true_with_non_strict_version_in_range` | — |
| should return false when version is not in range | 237 | ported | `semver_coerced.rs` | `matches_returns_false_when_version_is_not_in_range` | — |
| invalid version | 241 | ported | `semver_coerced.rs` | `matches_returns_false_for_invalid_version` | — |
| should return max satisfying version in range | 247 | ported | `semver_coerced.rs` | `get_satisfying_version_returns_max_satisfying_version_in_range` | — |
| should support coercion | 253 | ported | `semver_coerced.rs` | `get_satisfying_version_supports_coercion` | — |
| should return min satisfying version in range | 261 | ported | `semver_coerced.rs` | `min_satisfying_version_returns_min_satisfying_version_in_range` | — |
| should support coercion | 267 | ported | `semver_coerced.rs` | `min_satisfying_version_supports_coercion` | — |
| uses newVersion | 275 | ported | `semver_coerced.rs` | `get_new_value_uses_new_version` | — |
| should return zero for equal versions | 304 | ported | `semver_coerced.rs` | `sort_versions_returns_zero_for_equal_versions` | — |
| should return -1 for a < b | 308 | ported | `semver_coerced.rs` | `sort_versions_returns_less_for_a_less_than_b` | — |
| should return 1 for a > b | 312 | ported | `semver_coerced.rs` | `sort_versions_returns_greater_for_a_greater_than_b` | — |
| should return zero for equal non-strict versions | 316 | ported | `semver_coerced.rs` | `sort_versions_returns_zero_for_equal_non_strict_versions` | — |
| works with invalid version | 320 | ported | `semver_coerced.rs` | `sort_versions_works_with_invalid_version` | — |

---

