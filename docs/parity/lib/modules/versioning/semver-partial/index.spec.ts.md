# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/semver-partial/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/semver-partial/index.spec.ts
**Total tests:** 18 | **Ported:** 18 | **Actionable:** 18 | **Status:** ported

### `modules/versioning/semver-partial/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $expected | 5 | ported | `semver_partial.rs` | `is_valid_table` | — |
| isVersion("$version") === $expected | 24 | ported | `semver_partial.rs` | `is_version_table` | — |
| isStable("$version") === $expected | 47 | ported | `semver_partial.rs` | `is_stable_table` | — |
| isSingleVersion("$version") === $expected | 70 | ported | `semver_partial.rs` | `is_single_version_table` | — |
| matches("$version", "$range") === $expected | 87 | ported | `semver_partial.rs` | `matches_table` | — |
| should handle invalid range that is not ~latest or valid version | 141 | ported | `semver_partial.rs` | `matches_completely_invalid_range` | — |
| getSatisfyingVersion($versions, "$range") === $expected | 149 | ported | `semver_partial.rs` | `get_satisfying_version_table` | — |
| minSatisfyingVersion($versions, "$range") === $expected | 185 | ported | `semver_partial.rs` | `min_satisfying_version_table` | — |
| isLessThanRange("$version", "$range") === $expected | 209 | ported | `semver_partial.rs` | `is_less_than_range_table` | — |
| equals("$version", "$other") === $expected | 240 | ported | `semver_partial.rs` | `equals_table` | — |
| getMajor("$version") === $expected | 262 | ported | `semver_partial.rs` | `get_major_table` | — |
| getMinor("$version") === $expected | 275 | ported | `semver_partial.rs` | `get_minor_table` | — |
| getPatch("$version") === $expected | 288 | ported | `semver_partial.rs` | `get_patch_table` | — |
| isGreaterThan("$version", "$other") === $expected | 301 | ported | `semver_partial.rs` | `is_greater_than_table` | — |
| sortVersions("$a", "$b") === $expected | 326 | ported | `semver_partial.rs` | `sort_versions_table` | — |
| isBreaking("$version", "$current") === $expected | 348 | ported | `semver_partial.rs` | `is_breaking_table` | — |
| isCompatible("$version") === $expected | 376 | ported | `semver_partial.rs` | `is_compatible_table` | — |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 390 | ported | `semver_partial.rs` | `get_new_value_table` | — |

---

