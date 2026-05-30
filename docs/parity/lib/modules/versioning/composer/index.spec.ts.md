# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/composer/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/composer/index.spec.ts
**Total tests:** 18 | **Ported:** 18 | **Actionable:** 18 | **Status:** ported

### `modules/versioning/composer/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getMajor("$version") === $expected | 4 | ported | `versioning/composer.rs` | `get_major_cases` | — |
| getMinor("$version") === $expected | 12 | ported | `versioning/composer.rs` | `get_minor_cases` | — |
| getPatch("$version") === $expected | 20 | ported | `versioning/composer.rs` | `get_patch_cases` | — |
| equals("$a", "$b") === $expected | 28 | ported | `versioning/composer.rs` | `equals_cases` | — |
| isGreaterThan("$a", "$b") === $expected | 40 | ported | `versioning/composer.rs` | `is_greater_than_cases` | — |
| isSingleVersion("$version") === $expected | 55 | ported | `versioning/composer.rs` | `is_single_version_cases` | — |
| isStable("$version") === $expected | 63 | ported | `versioning/composer.rs` | `is_stable_cases` | — |
| isValid("$version") === $expected | 75 | ported | `versioning/composer.rs` | `is_valid_cases` | — |
| isLessThanRange("$a", "$b") === $expected | 108 | ported | `versioning/composer.rs` | `is_less_than_range_cases` | — |
| getSatisfyingVersion($versions, "$range") === $expected | 116 | ported | `versioning/composer.rs` | `get_satisfying_version_cases` | — |
| minSatisfyingVersion($versions, "$range") === $expected | 131 | ported | `versioning/composer.rs` | `min_satisfying_version_cases` | — |
| matches("$a", "$b") === $expected | 147 | ported | `composer.rs` | `matches_cases` | — |
| subset("$a", "$b") === $expected | 155 | ported | `composer.rs` | `subset_cases` | — |
| intersects("$a", "$b") === $expected | 177 | ported | `composer.rs` | `intersects_cases` | — |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 199 | ported | `versioning/composer.rs` | `get_new_value_cases` | — |
| $versions -> sortVersions -> $expected | 256 | ported | `versioning/composer.rs` | `sort_versions_cases` | — |
| isCompatible("$version") === $expected | 266 | ported | `versioning/composer.rs` | `is_compatible_cases` | — |
| isBreaking("$currentVersion", "$newVersion") === $expected | 275 | ported | `composer.rs` | `is_breaking_cases` | — |

---
