# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/hermit/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/hermit/index.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 10 | **Status:** ported

### `modules/versioning/hermit/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isStable("$version") === $expected | 6 | ported | crates/renovate-core/src/versioning/hermit.rs | is_stable_matches_renovate_hermit_index_spec | — |
| isValid("$version") === $expected | 19 | ported | crates/renovate-core/src/versioning/hermit.rs | is_valid_matches_renovate_hermit_index_spec | — |
| getMajor, getMinor, getPatch for "$version" | 46 | ported | crates/renovate-core/src/versioning/hermit.rs | get_major_minor_patch_matches_renovate_hermit_index_spec | — |
| equals("$version", "$other") === $expected | 65 | ported | crates/renovate-core/src/versioning/hermit.rs | equals_matches_renovate_hermit_index_spec | — |
| matches("$version", "$range") === $expected | 83 | ported | crates/renovate-core/src/versioning/hermit.rs | matches_matches_renovate_hermit_index_spec | — |
| isGreaterThan("$version", "$other") === $expected | 110 | ported | crates/renovate-core/src/versioning/hermit.rs | is_greater_than_matches_renovate_hermit_index_spec | — |
| isLessThanRange("$version", "$other") === $expected | 139 | ported | crates/renovate-core/src/versioning/hermit.rs | is_less_than_range_matches_renovate_hermit_index_spec | — |
| getSatisfyingVersion | 166 | ported | crates/renovate-core/src/versioning/hermit.rs | get_satisfying_version_matches_renovate_hermit_index_spec | — |
| minSatisfyingVersion | 184 | ported | crates/renovate-core/src/versioning/hermit.rs | min_satisfying_version_matches_renovate_hermit_index_spec | — |
| sorts versions in an ascending order | 203 | ported | crates/renovate-core/src/versioning/hermit.rs | sort_versions_ascending_matches_renovate_hermit_index_spec | — |

---

