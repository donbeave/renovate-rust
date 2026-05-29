# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/maven/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/maven/index.spec.ts
**Total tests:** 13 | **Ported:** 12 | **Actionable:** 12 | **Status:** ported

### `modules/versioning/maven/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses same function module export and api object | 7 | not-applicable | — | — | TypeScript module system test; verifies re-export identity (toBe same reference) which has no Rust equivalent |
| isValid("$version") === $expected | 11 | ported | crates/renovate-core/src/versioning/maven.rs | is_valid_matches_renovate_maven_index_spec | — |
| isVersion("$version") === $expected | 32 | ported | crates/renovate-core/src/versioning/maven.rs | is_version_matches_renovate_maven_index_spec | — |
| isStable("$version") === $expected | 60 | ported | crates/renovate-core/src/versioning/maven.rs | is_stable_index_matches_renovate_maven_index_spec | — |
| "$input" is represented as [$major, $minor, $patch] | 89 | ported | crates/renovate-core/src/versioning/maven.rs | get_major_minor_patch_matches_renovate_maven_index_spec | — |
| matches("$version", "$range") === $expected | 111 | ported | crates/renovate-core/src/versioning/maven.rs | matches_range_matches_renovate_maven_index_spec | — |
| isGreaterThan("$a", "$b") === $expected | 158 | ported | crates/renovate-core/src/versioning/maven.rs | is_greater_than_matches_renovate_maven_index_spec | — |
| getSatisfyingVersion($versions, "$range") === $expected | 165 | ported | crates/renovate-core/src/versioning/maven.rs | get_satisfying_version_matches_renovate_maven_index_spec | — |
| minSatisfyingVersion($versions, "$range") === $expected | 179 | ported | crates/renovate-core/src/versioning/maven.rs | min_satisfying_version_matches_renovate_maven_index_spec | — |
| getNewValue($currentValue, $rangeStrategy, $currentVersion, $newVersion, $expected) === $expected | 193 | ported | crates/renovate-core/src/versioning/maven.rs | get_new_value_matches_renovate_maven_index_spec | — |
| matches("$version", "[2.164.0,2.165.0)") === $expected | 228 | ported | crates/renovate-core/src/versioning/maven.rs | matches_jenkins_range_excl_matches_renovate_maven_index_spec | — |
| matches("$version", "[2.164.0,2.165.0]") === $expected | 247 | ported | crates/renovate-core/src/versioning/maven.rs | matches_jenkins_range_incl_matches_renovate_maven_index_spec | — |
| matches("$version", "(,2.164.0)") === $expected | 266 | ported | crates/renovate-core/src/versioning/maven.rs | matches_jenkins_range_lt_matches_renovate_maven_index_spec | — |

---

