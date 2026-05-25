# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/conda/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/conda/index.spec.ts
**Total tests:** 15 | **Ported:** 15 | **Actionable:** 15 | **Status:** ported

### `modules/versioning/conda/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isVersion("$input") === $expected | 4 | ported | crates/renovate-core/src/versioning/conda.rs | is_version_matches_renovate_conda_index_spec | — |
| isValid("$input") === $expected | 26 | ported | crates/renovate-core/src/versioning/conda.rs | is_valid_matches_renovate_conda_index_spec | — |
| isStable("$input") === $expected | 47 | ported | crates/renovate-core/src/versioning/conda.rs | is_stable_matches_renovate_conda_index_spec | — |
| equals("$a", "$b") === $expected | 57 | ported | crates/renovate-core/src/versioning/conda.rs | equals_matches_renovate_conda_index_spec | — |
| matches("$a", "$b") === $expected | 69 | ported | crates/renovate-core/src/versioning/conda.rs | matches_matches_renovate_conda_index_spec | — |
| getMajor("$a") === $expected | 82 | ported | crates/renovate-core/src/versioning/conda.rs | get_major_matches_renovate_conda_index_spec | — |
| getMinor($a) === $expected | 93 | ported | crates/renovate-core/src/versioning/conda.rs | get_minor_matches_renovate_conda_index_spec | — |
| getPatch("$a") === $expected | 105 | ported | crates/renovate-core/src/versioning/conda.rs | get_patch_matches_renovate_conda_index_spec | — |
| isSingleVersion("$version") === $isSingle | 116 | ported | crates/renovate-core/src/versioning/conda.rs | is_single_version_matches_renovate_conda_index_spec | — |
| always compatible | 131 | ported | crates/renovate-core/src/versioning/conda.rs | always_compatible_matches_renovate_conda_index_spec | — |
| getSatisfyingVersion($versions, "$range") === $expected | 146 | ported | crates/renovate-core/src/versioning/conda.rs | get_satisfying_version_matches_renovate_conda_index_spec | — |
| minSatisfyingVersion($versions, "$range") === $expected | 157 | ported | crates/renovate-core/src/versioning/conda.rs | min_satisfying_version_matches_renovate_conda_index_spec | — |
| isGreaterThan("$a", "$b") === $result | 168 | ported | crates/renovate-core/src/versioning/conda.rs | is_greater_than_matches_renovate_conda_index_spec | — |
| returns a pinned value | 176 | ported | crates/renovate-core/src/versioning/conda.rs | get_pinned_value_matches_renovate_conda_index_spec | — |
| getNewValue($currentValue, $rangeStrategy, $currentVersion, $newVersion) === $expected | 180 | ported | crates/renovate-core/src/versioning/conda.rs | get_new_value_matches_renovate_conda_index_spec | — |

---

