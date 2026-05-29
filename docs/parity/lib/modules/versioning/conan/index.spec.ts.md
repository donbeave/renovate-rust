# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/conan/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/conan/index.spec.ts
**Total tests:** 16 | **Ported:** 16 | **Actionable:** 16 | **Status:** ported

### `modules/versioning/conan/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $result | 5 | ported | crates/renovate-core/src/versioning/conan.rs | is_valid_matches_renovate_conan_index_spec | — |
| isVersion("$version") === $result | 117 | ported | crates/renovate-core/src/versioning/conan.rs | is_version_matches_renovate_conan_index_spec | — |
| isCompatible("$version", "$range") === $result | 163 | ported | crates/renovate-core/src/versioning/conan.rs | is_compatible_matches_renovate_conan_index_spec | — |
| matches("$version", "$range") === $result | 358 | ported | crates/renovate-core/src/versioning/conan.rs | matches_matches_renovate_conan_index_spec | — |
| getSatisfyingVersion("$versions", "$range") === "$result" | 553 | ported | crates/renovate-core/src/versioning/conan.rs | get_satisfying_version_matches_renovate_conan_index_spec | — |
| getSatisfyingVersion("$versions", "$range") === "$result" | 565 | ported | crates/renovate-core/src/versioning/conan.rs | get_satisfying_version_matches_renovate_conan_index_spec | — |
| getSatisfyingVersion("$versions", "$range") === "$result" | 641 | ported | crates/renovate-core/src/versioning/conan.rs | get_satisfying_version_matches_renovate_conan_index_spec | — |
| minSatisfyingVersion("$versions", "$range") === "$result" | 699 | ported | crates/renovate-core/src/versioning/conan.rs | min_satisfying_version_matches_renovate_conan_index_spec | — |
| getMajor("$version") === $major getMinor("$version") === $minor getPatch("$version") === $patch | 720 | ported | crates/renovate-core/src/versioning/conan.rs | get_major_minor_patch_matches_renovate_conan_index_spec | — |
| getMajor("$version") === "$result" | 743 | ported | crates/renovate-core/src/versioning/conan.rs | get_major_matches_renovate_conan_index_spec | — |
| getMinor("$version") === "$result" | 752 | ported | crates/renovate-core/src/versioning/conan.rs | get_minor_matches_renovate_conan_index_spec | — |
| getPatch("$version") === "$result" | 763 | ported | crates/renovate-core/src/versioning/conan.rs | get_patch_matches_renovate_conan_index_spec | — |
| equals("$version", "$other) === "$result" | 774 | ported | crates/renovate-core/src/versioning/conan.rs | equals_matches_renovate_conan_index_spec | — |
| isGreaterThan("$version", "$other) === "$result" | 825 | ported | crates/renovate-core/src/versioning/conan.rs | is_greater_than_matches_renovate_conan_index_spec | — |
| sortVersions("$version", "$other) === "$result" | 871 | ported | crates/renovate-core/src/versioning/conan.rs | sort_versions_matches_renovate_conan_index_spec | — |
| isLessThanRange("$version", "$range") === "$result" | 886 | ported | crates/renovate-core/src/versioning/conan.rs | is_less_than_range_matches_renovate_conan_index_spec | — |

---

