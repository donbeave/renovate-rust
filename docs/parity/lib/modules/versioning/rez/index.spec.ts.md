# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/rez/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/rez/index.spec.ts
**Total tests:** 16 | **Ported:** 16 | **Actionable:** 0 | **Status:** done

### `modules/versioning/rez/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals("$version", "$equal") === $expected | 5 | ported | crates/renovate-core/src/versioning/rez.rs | equals_table | — |
| getMajor("$version") === $expected | 21 | ported | crates/renovate-core/src/versioning/rez.rs | get_major_table | — |
| getMinor("$version") === $expected | 30 | ported | crates/renovate-core/src/versioning/rez.rs | get_minor_table | — |
| getPatch("$version") === $expected | 39 | ported | crates/renovate-core/src/versioning/rez.rs | get_patch_table | — |
| isGreaterThan("$version", "$other") === $expected | 49 | ported | crates/renovate-core/src/versioning/rez.rs | is_greater_than_table | — |
| isStable("$version") === $expected | 67 | ported | crates/renovate-core/src/versioning/rez.rs | is_stable_table | — |
| isValid("$input") === $expected | 78 | ported | crates/renovate-core/src/versioning/rez.rs | is_valid_table | — |
| isVersion("$input") === $expected | 100 | ported | crates/renovate-core/src/versioning/rez.rs | is_version_table | — |
| isSingleVersion("$input") === $expected | 108 | ported | crates/renovate-core/src/versioning/rez.rs | is_single_version_table | — |
| minSatisfyingVersion($versions, "$range") === $expected | 119 | ported | crates/renovate-core/src/versioning/rez.rs | min_satisfying_version_table | — |
| getSatisfyingVersion($versions, "$range") === $expected | 135 | ported | crates/renovate-core/src/versioning/rez.rs | get_satisfying_version_table | — |
| isLessThanRange($version, "$range") === $expected | 145 | ported | crates/renovate-core/src/versioning/rez.rs | is_less_than_range_table | — |
| matches($version, "$range") === $expected | 158 | ported | crates/renovate-core/src/versioning/rez.rs | matches_range_table | — |
| rez.sortVersions("$a", "$b") === semver.sortVersions("$a", "$b") | 178 | ported | crates/renovate-core/src/versioning/rez.rs | sort_versions_table | — |
| getNewValue($currentValue, $rangeStrategy, $currentVersion, $newVersion, $expected) === $expected | 193 | ported | crates/renovate-core/src/versioning/rez.rs | get_new_value_table | — |
| isCompatible("$version") === $expected | 443 | ported | crates/renovate-core/src/versioning/rez.rs | is_compatible_table | — |

---

