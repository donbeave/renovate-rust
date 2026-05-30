# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/same-major/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/same-major/index.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 0 | **Status:** done

### `modules/versioning/same-major/index › .isGreaterThan(version, other)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true | 5 | ported | crates/renovate-core/src/versioning/same_major.rs | is_greater_than_true_matches_renovate_same_major_index_spec | — |
| should return false | 9 | ported | crates/renovate-core/src/versioning/same_major.rs | is_greater_than_false_matches_renovate_same_major_index_spec | — |

### `modules/versioning/same-major/index › .matches(version, range)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true when version has same major | 18 | ported | crates/renovate-core/src/versioning/same_major.rs | matches_true_matches_renovate_same_major_index_spec | — |
| should return false when version has different major | 23 | ported | crates/renovate-core/src/versioning/same_major.rs | matches_diff_major_matches_renovate_same_major_index_spec | — |
| should return false when version is out of range | 27 | ported | crates/renovate-core/src/versioning/same_major.rs | matches_out_of_range_matches_renovate_same_major_index_spec | — |
| should return false when version is invalid | 33 | ported | crates/renovate-core/src/versioning/same_major.rs | matches_invalid_matches_renovate_same_major_index_spec | — |

### `modules/versioning/same-major/index › .getSatisfyingVersion(versions, range)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return max satisfying version in range | 39 | ported | crates/renovate-core/src/versioning/same_major.rs | get_satisfying_version_matches_renovate_same_major_index_spec | — |

### `modules/versioning/same-major/index › .minSatisfyingVersion(versions, range)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return min satisfying version in range | 50 | ported | crates/renovate-core/src/versioning/same_major.rs | min_satisfying_version_matches_renovate_same_major_index_spec | — |

### `modules/versioning/same-major/index › .isLessThanRange(version, range)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true | 61 | ported | crates/renovate-core/src/versioning/same_major.rs | is_less_than_range_true_matches_renovate_same_major_index_spec | — |
| should return false | 65 | ported | crates/renovate-core/src/versioning/same_major.rs | is_less_than_range_false_matches_renovate_same_major_index_spec | — |

---

