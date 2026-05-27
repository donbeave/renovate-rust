# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/elm/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/elm/index.spec.ts
**Total tests:** 31 | **Ported:** 31 | **Actionable:** 31 | **Status:** done

### `modules/versioning/elm/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isVersion("$input") === $expected | 5 | ported | `elm.rs` | `is_version_table` | — |
| isValid("$input") === $expected | 23 | ported | `elm.rs` | `is_valid_table` | — |
| isSingleVersion("$input") === $expected | 43 | ported | `elm.rs` | `is_single_version_table` | — |
| isStable("$input") === $expected | 55 | ported | `elm.rs` | `is_stable_table` | — |
| returns false for invalid version | 65 | ported | `elm.rs` | `is_stable_invalid_returns_false` | — |
| isCompatible("$input") === $expected | 71 | ported | `elm.rs` | `is_compatible_table` | — |
| extracts version components | 81 | ported | `elm.rs` | `extracts_version_components` | — |
| equals("$a", "$b") === $expected | 89 | ported | `elm.rs` | `equals_table` | — |
| isGreaterThan("$a", "$b") === $expected | 100 | ported | `elm.rs` | `is_greater_than_table` | — |
| sorts versions correctly | 112 | ported | `elm.rs` | `sorts_versions_correctly` | — |
| matches("$version", "$range") === $expected | 120 | ported | `elm.rs` | `matches_table` | — |
| returns false for invalid version | 139 | ported | `elm.rs` | `matches_invalid_version_returns_false` | — |
| returns false for invalid range | 143 | ported | `elm.rs` | `matches_invalid_range_returns_false` | — |
| returns false for malformed range where lower > upper | 147 | ported | `elm.rs` | `matches_lower_gt_upper_returns_false` | — |
| isLessThanRange("$version", "$range") === $expected | 153 | ported | `elm.rs` | `is_less_than_range_table` | — |
| returns false for invalid version | 170 | ported | `elm.rs` | `is_less_than_range_invalid_version_returns_false` | — |
| returns false for invalid range | 176 | ported | `elm.rs` | `is_less_than_range_invalid_range_returns_false` | — |
| getSatisfyingVersion($versions, "$range") === $expected | 182 | ported | `elm.rs` | `get_satisfying_version_table` | — |
| minSatisfyingVersion($versions, "$range") === $expected | 199 | ported | `elm.rs` | `min_satisfying_version_table` | — |
| replaces exact version with new version | 215 | ported | `elm.rs` | `get_new_value_exact_replace` | — |
| handles bump strategy for exact version | 225 | ported | `elm.rs` | `get_new_value_exact_bump` | — |
| getNewValue("$currentValue", "$rangeStrategy", "$newVersion") === "$expected" | 237 | ported | `elm.rs` | `get_new_value_range_table` | — |
| returns null for invalid new version | 266 | ported | `elm.rs` | `get_new_value_invalid_new_version_returns_none` | — |
| returns null for invalid current value | 276 | ported | `elm.rs` | `get_new_value_invalid_current_value_returns_none` | — |
| returns null for unknown range strategy | 286 | ported | `elm.rs` | `get_new_value_unknown_strategy_returns_none` | — |
| handles widen when newVersion equals upper bound exactly | 296 | ported | `elm.rs` | `get_new_value_widen_equals_upper` | — |
| widens elm-version range for new compiler release | 307 | ported | `elm.rs` | `get_new_value_widen_elm_compiler` | — |
| keeps elm-version range unchanged when version is already satisfied | 318 | ported | `elm.rs` | `get_new_value_update_lockfile_satisfied` | — |
| replaces elm-version range when explicitly requested | 328 | ported | `elm.rs` | `get_new_value_replace_elm_version` | — |
| finds highest satisfying version for elm-version range | 341 | ported | `elm.rs` | `get_satisfying_version_elm_compiler` | — |
| returns null when no compiler version satisfies range | 355 | ported | `elm.rs` | `get_satisfying_version_none_satisfies` | — |

---

