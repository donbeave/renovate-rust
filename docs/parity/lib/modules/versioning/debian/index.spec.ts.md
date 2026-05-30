# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/debian/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/debian/index.spec.ts
**Total tests:** 16 | **Ported:** 14 | **Actionable:** 0 | **Status:** done

### `modules/versioning/debian/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| test | 18 | ported | versioning/debian.rs | debian_basic_test | — |
| isValid("$version") === $expected | 22 | ported | versioning/debian.rs | test_debian_is_valid | — |
| isCompatible("$version") === $expected | 82 | ported | versioning/debian.rs | debian_is_compatible | — |
| isSingleVersion("$version") === $expected | 104 | ported | versioning/debian.rs | test_debian_is_single_version | — |
| isStable("$version") === $expected | 115 | ported | versioning/debian.rs | test_debian_is_stable | — |
| ensures that rolling release is not refreshed within frame time window: $version | 169 | not-applicable | — | — | Tests logging/debug infrastructure for cache refresh; fixed snapshot has no refresh |
| isVersion("$version") === $expected | 188 | ported | versioning/debian.rs | test_debian_is_version | — |
| getMajor, getMinor, getPatch for "$version" | 248 | ported | versioning/debian.rs | debian_get_major_minor_patch | — |
| equals($a, $b) === $expected | 273 | ported | versioning/debian.rs | test_debian_equals | — |
| isGreaterThan("$a", "$b") === $expected | 297 | ported | versioning/debian.rs | test_debian_is_greater_than | — |
| getSatisfyingVersion($versions, "$range") === "$expected" | 340 | ported | versioning/debian.rs | test_debian_get_satisfying_version | — |
| minSatisfyingVersion($versions, "$range") === "$expected" | 361 | ported | versioning/debian.rs | test_debian_min_satisfying_version | — |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 383 | ported | versioning/debian.rs | test_debian_get_new_value | — |
| debian.sortVersions($a, $b) === $expected | 409 | ported | versioning/debian.rs | test_debian_sort_versions | — |
| matches("$version", "$range") === $expected | 429 | ported | versioning/debian.rs | test_debian_matches | — |
| checks runtime date handling & refresh rolling release data | 441 | not-applicable | — | — | Tests dynamic time-based refresh of rolling release data; fixed snapshot has no refresh |

---
