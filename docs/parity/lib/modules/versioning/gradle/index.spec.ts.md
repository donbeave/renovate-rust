# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/gradle/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/gradle/index.spec.ts
**Total tests:** 12 | **Ported:** 12 | **Actionable:** 12 | **Status:** done

### `modules/versioning/gradle/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| compare("$a", "$b") === $expected | 6 | ported | `versioning/gradle.rs` | `gradle_compare_equal`, `gradle_compare_less`, `gradle_compare_greater` | — |
| parsePrefixRange("$rangeStr") is null | 89 | ported | `versioning/gradle.rs` | `gradle_parse_prefix_range_null` | — |
| parseMavenBasedRange("$rangeStr") is null | 102 | ported | `versioning/gradle.rs` | `gradle_parse_maven_based_range_null` | — |
| isValid("$input") === $expected | 127 | ported | `versioning/gradle.rs` | `gradle_is_valid` | — |
| isVersion("$input") === $expected | 140 | ported | `versioning/gradle.rs` | `gradle_is_version` | — |
| isStable("$input") === $expected | 180 | ported | `versioning/gradle.rs` | `gradle_is_stable` | — |
| "$input" is represented as [$major, $minor, $patch] | 216 | ported | `versioning/gradle.rs` | `gradle_major_minor_patch` | — |
| matches("$version", "$range") === $expected | 239 | ported | `versioning/gradle.rs` | `gradle_matches` | — |
| isGreaterThan("$a", "$b") === $expected | 271 | ported | `versioning/gradle.rs` | `gradle_is_greater_than` | — |
| minSatisfyingVersion($versions, "$range") === $expected | 280 | ported | `versioning/gradle.rs` | `gradle_min_satisfying_version` | — |
| getSatisfyingVersion($versions, "$range") === $expected | 292 | ported | `versioning/gradle.rs` | `gradle_get_satisfying_version` | — |
| getNewValue($currentValue, $rangeStrategy, $currentVersion, $newVersion, $expected) === $expected | 304 | ported | `versioning/gradle.rs` | `gradle_get_new_value` | — |

---
