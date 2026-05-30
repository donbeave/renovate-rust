# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/nuget/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/nuget/index.spec.ts
**Total tests:** 18 | **Ported:** 18 | **Actionable:** 0 | **Status:** done

### `modules/versioning/nuget/index › isSingleVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isSingleVersion("$input") === $expected | 5 | ported | crates/renovate-core/src/versioning/nuget.rs | nuget_is_single_version_parametrized | — |

### `modules/versioning/nuget/index › isStable()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isStable("$input") === $expected | 18 | ported | crates/renovate-core/src/versioning/nuget.rs | is_stable_matches_renovate_index_spec | — |

### `modules/versioning/nuget/index › isValid()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$input") === $expected | 43 | ported | crates/renovate-core/src/versioning/nuget.rs | nuget_is_valid_parametrized | — |

### `modules/versioning/nuget/index › isVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isVersion("$input") === $expected | 118 | ported | crates/renovate-core/src/versioning/nuget.rs | nuget_is_version_parametrized | — |

### `modules/versioning/nuget/index › getMajor, getMinor, getPatch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input -> [$major, $minor, $patch] | 218 | ported | crates/renovate-core/src/versioning/nuget.rs | nuget_get_major_minor_patch_parametrized | — |

### `modules/versioning/nuget/index › equals()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals($a, $b) === $expected | 258 | ported | crates/renovate-core/src/versioning/nuget.rs | nuget_equals_parametrized | — |

### `modules/versioning/nuget/index › isGreaterThan()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isGreaterThan($a, $b) === $expected | 303 | ported | crates/renovate-core/src/versioning/nuget.rs | nuget_is_greater_than_parametrized | — |

### `modules/versioning/nuget/index › isLessThanRange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isLessThanRange("$version", "$range") === $expected | 347 | ported | crates/renovate-core/src/versioning/nuget.rs | nuget_is_less_than_range_parametrized | — |

### `modules/versioning/nuget/index › getSatisfyingVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getSatisfyingVersion($versions, $range) === "$expected" | 392 | ported | crates/renovate-core/src/versioning/nuget.rs | nuget_get_satisfying_version_parametrized | — |

### `modules/versioning/nuget/index › minSatisfyingVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| minSatisfyingVersion($versions, $range) === $expected | 420 | ported | crates/renovate-core/src/versioning/nuget.rs | nuget_min_satisfying_version_parametrized | — |

### `modules/versioning/nuget/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns a pinned value | 435 | ported | crates/renovate-core/src/versioning/nuget.rs | nuget_get_pinned_value_test | — |

### `modules/versioning/nuget/index › getNewValue()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns newVersion if the range is version too | 441 | ported | crates/renovate-core/src/versioning/nuget.rs | nuget_get_new_value_version_range_returns_new_version | — |
| returns null if version is invalid | 451 | ported | crates/renovate-core/src/versioning/nuget.rs | nuget_get_new_value_invalid_version_returns_none | — |
| returns null if range is invalid | 461 | ported | crates/renovate-core/src/versioning/nuget.rs | nuget_get_new_value_invalid_range_returns_none | — |

### `modules/versioning/nuget/index › getNewValue() › pin`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the new version | 472 | ported | crates/renovate-core/src/versioning/nuget.rs | nuget_get_pinned_value_pin | — |

### `modules/versioning/nuget/index › getNewValue() › bump`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| currentValue=$currentValue newVersion=$newVersion -> $expected | 478 | ported | crates/renovate-core/src/versioning/nuget.rs | nuget_get_new_value_bump_parametrized | — |

### `modules/versioning/nuget/index › sortVersions`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sortVersions($a, $b) === $expected | 522 | ported | crates/renovate-core/src/versioning/nuget.rs | nuget_sort_versions_parametrized | — |

### `modules/versioning/nuget/index › matches()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| matches("$version", "$range") === $expected | 547 | ported | crates/renovate-core/src/versioning/nuget.rs | nuget_matches_parametrized | — |

---

