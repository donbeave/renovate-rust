# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/docker/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/docker/index.spec.ts
**Total tests:** 12 | **Ported:** 12 | **Actionable:** 12 | **Status:** ported

### `modules/versioning/docker/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $expected | 5 | ported | crates/renovate-core/src/versioning/docker.rs | is_valid_matches_renovate_docker_index_spec | — |
| getMajor, getMinor, getPatch for "$version" | 27 | ported | crates/renovate-core/src/versioning/docker.rs | get_major_minor_patch_matches_renovate_docker_index_spec | — |
| isGreaterThan($a, $b) === $expected | 43 | ported | crates/renovate-core/src/versioning/docker.rs | is_greater_than_matches_renovate_docker_index_spec | — |
| isLessThanRange($version, $range) === $expected | 54 | ported | crates/renovate-core/src/versioning/docker.rs | is_less_than_range_matches_renovate_docker_index_spec | — |
| equals($a, $b) === $expected | 68 | ported | crates/renovate-core/src/versioning/docker.rs | equals_matches_renovate_docker_index_spec | — |
| satisfying for $version -> $expected | 92 | ported | crates/renovate-core/src/versioning/docker.rs | satisfying_matches_renovate_docker_index_spec | — |
| docker.sortVersions("$a", "$b") === semver.sortVersions("$a", "$b") | 108 | ported | crates/renovate-core/src/versioning/docker.rs | sort_versions_semver_matches_renovate_docker_index_spec | — |
| sorts unstable | 123 | ported | crates/renovate-core/src/versioning/docker.rs | sort_unstable_matches_renovate_docker_index_spec | — |
| getNewValue($currentValue, $rangeStrategy, $currentVersion, $newVersion, $expected) === $expected | 148 | ported | crates/renovate-core/src/versioning/docker.rs | get_new_value_matches_renovate_docker_index_spec | — |
| isStable("$version") === $expected | 164 | ported | crates/renovate-core/src/versioning/docker.rs | is_stable_matches_renovate_docker_index_spec | — |
| isCompatible("$version") === $expected | 177 | ported | crates/renovate-core/src/versioning/docker.rs | is_compatible_matches_renovate_docker_index_spec | — |
| valueToVersion("$value") === $expected | 199 | ported | crates/renovate-core/src/versioning/docker.rs | value_to_version_matches_renovate_docker_index_spec | — |

---

