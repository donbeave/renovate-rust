# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/ubuntu/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/ubuntu/index.spec.ts
**Total tests:** 13 | **Ported:** 13 | **Actionable:** 0 | **Status:** done

### `modules/versioning/ubuntu/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $expected | 7 | ported | `ubuntu.rs` | `is_valid_matches_renovate_ubuntu_spec` | — |
| isCompatible("$version") === $expected | 90 | ported | `ubuntu.rs` | `is_compatible_matches_renovate_ubuntu_spec` | — |
| isSingleVersion("$version") === $expected | 108 | ported | `ubuntu.rs` | `is_single_version_matches_renovate_ubuntu_spec` | — |
| isStable("$version") === $expected | 118 | ported | `ubuntu.rs` | `is_stable_matches_renovate_ubuntu_spec` | — |
| isVersion("$version") === $expected | 195 | ported | `ubuntu.rs` | `is_version_matches_renovate_ubuntu_spec` | — |
| getMajor, getMinor, getPatch for "$version" | 244 | ported | `ubuntu.rs` | `component_accessors_match_renovate_ubuntu_spec` | — |
| equals($a, $b) === $expected | 265 | ported | `ubuntu.rs` | `equals_matches_renovate_ubuntu_spec` | — |
| isGreaterThan("$a", "$b") === $expected | 287 | ported | `ubuntu.rs` | `is_greater_than_matches_renovate_ubuntu_spec` | — |
| getSatisfyingVersion($versions, "$range") === "$expected" | 326 | ported | `ubuntu.rs` | `get_satisfying_version_matches_renovate_ubuntu_spec` | — |
| minSatisfyingVersion($versions, "$range") === "$expected" | 354 | ported | `ubuntu.rs` | `min_satisfying_version_matches_renovate_ubuntu_spec` | — |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 382 | ported | `ubuntu.rs` | `get_new_value_matches_renovate_ubuntu_spec` | — |
| $versions -> sortVersions -> $expected | 404 | ported | `ubuntu.rs` | `sort_versions_matches_renovate_ubuntu_spec` | — |
| matches("$version", "$range") === "$expected" | 411 | ported | `ubuntu.rs` | `matches_matches_renovate_ubuntu_spec` | — |

---

