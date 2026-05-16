# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/exact/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/exact/index.spec.ts
**Total tests:** 14 | **Ported:** 14 | **Actionable:** 14 | **Status:** ported

### `modules/versioning/exact/index › isValid`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$input") === $expected | 5 | ported | `exact.rs` | `is_valid_matches_renovate_exact_spec` | — |

### `modules/versioning/exact/index › isVersion`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isVersion($input) === $expected | 18 | ported | `exact.rs` | `is_version_matches_renovate_exact_spec` | — |

### `modules/versioning/exact/index › isSingleVersion`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true for any valid version | 30 | ported | `exact.rs` | `is_single_version_returns_true_for_any_valid_version` | — |

### `modules/versioning/exact/index › isStable`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true for any version | 38 | ported | `exact.rs` | `is_stable_returns_true_for_any_version` | — |

### `modules/versioning/exact/index › isCompatible`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true when version equals current | 46 | ported | `exact.rs` | `is_compatible_returns_true_when_version_equals_current` | — |
| returns false when version differs from current | 50 | ported | `exact.rs` | `is_compatible_returns_false_when_version_differs_from_current` | — |

### `modules/versioning/exact/index › getMajor/getMinor/getPatch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for all | 56 | ported | `exact.rs` | `component_accessors_return_none` | — |

### `modules/versioning/exact/index › equals`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals("$a", "$b") === $expected | 66 | ported | `exact.rs` | `equals_matches_renovate_exact_spec` | — |

### `modules/versioning/exact/index › isGreaterThan`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isGreaterThan("$a", "$b") === $expected | 79 | ported | `exact.rs` | `is_greater_than_matches_renovate_exact_spec` | — |

### `modules/versioning/exact/index › matches`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| matches("$version", "$range") === $expected | 90 | ported | `exact.rs` | `matches_matches_renovate_exact_spec` | — |

### `modules/versioning/exact/index › getSatisfyingVersion`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns exact match only | 106 | ported | `exact.rs` | `get_satisfying_version_returns_exact_match_only` | — |

### `modules/versioning/exact/index › minSatisfyingVersion`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns exact match only | 115 | ported | `exact.rs` | `min_satisfying_version_returns_exact_match_only` | — |

### `modules/versioning/exact/index › getNewValue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns currentValue unchanged | 124 | ported | `exact.rs` | `get_new_value_returns_current_value_unchanged` | — |

### `modules/versioning/exact/index › sortVersions`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns 0 for any comparison | 138 | ported | `exact.rs` | `sort_versions_returns_equal_for_any_comparison` | — |

---

