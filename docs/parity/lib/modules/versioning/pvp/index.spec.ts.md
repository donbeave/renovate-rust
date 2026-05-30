# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/pvp/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/pvp/index.spec.ts
**Total tests:** 18 | **Ported:** 18 | **Actionable:** 0 | **Status:** done

### `modules/versioning/pvp/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| pvp.isGreaterThan($first, $second) | 5 | ported | `pvp.rs` | `is_greater_than_matches_renovate_pvp_spec` | — |
| pvp.getMajor("$version") === $expected | 25 | ported | `pvp.rs` | `get_major_matches_renovate_pvp_spec` | — |
| pvp.getMinor("$version") === $expected | 38 | ported | `pvp.rs` | `get_minor_matches_renovate_pvp_spec` | — |
| pvp.getPatch("$version") === $expected | 50 | ported | `pvp.rs` | `get_patch_matches_renovate_pvp_spec` | — |
| pvp.matches("$version", "$range") === $expected | 65 | ported | `pvp.rs` | `matches_matches_renovate_pvp_spec` | — |
| pvp.getSatisfyingVersion($versions, "$range") === $expected | 90 | ported | `pvp.rs` | `get_satisfying_version_matches_renovate_pvp_spec` | — |
| should return min satisfying version in range | 108 | ported | `pvp.rs` | `min_satisfying_version_returns_min_satisfying_version_in_range` | — |
| pvp.isLessThanRange?.("$version", "$range") === $expected | 121 | ported | `pvp.rs` | `is_less_than_range_matches_renovate_pvp_spec` | — |
| pvp.isValid("$version") === $expected | 142 | ported | `pvp.rs` | `is_valid_matches_renovate_pvp_spec` | — |
| pvp.getNewValue({currentValue: "$currentValue", newVersion: "$newVersion", rangeStrategy: "$rangeStrategy"}) === $expected | 153 | ported | `pvp.rs` | `get_new_value_matches_renovate_pvp_spec` | — |
| pvp.isSame("$type", "$a", "$b") === $expected | 176 | ported | `pvp.rs` | `is_same_matches_renovate_pvp_spec` | — |
| pvp.isVersion("$version") === $expected | 210 | ported | `pvp.rs` | `is_version_matches_renovate_pvp_spec` | — |
| pvp.equals("$a", "$b") === $expected | 219 | ported | `pvp.rs` | `equals_matches_renovate_pvp_spec` | — |
| pvp.isSingleVersion("$version") === $expected | 230 | ported | `pvp.rs` | `is_single_version_matches_renovate_pvp_spec` | — |
| pvp.subbet("$subRange", "$superRange") === $expected | 242 | ported | `pvp.rs` | `subset_matches_renovate_pvp_spec` | — |
| pvp.sortVersions("$a", "$b") === $expected | 259 | ported | `pvp.rs` | `sort_versions_matches_renovate_pvp_spec` | — |
| should consider 0.0.0 stable | 269 | ported | `pvp.rs` | `is_stable_considers_all_versions_stable` | — |
| should consider 0.0.0 compatible | 277 | ported | `pvp.rs` | `is_compatible_considers_all_versions_compatible` | — |

---

