# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/regex/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/regex/index.spec.ts
**Total tests:** 24 | **Ported:** 24 | **Actionable:** 24 | **Status:** done

### `modules/versioning/regex/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| requires a valid configuration to be initialized | 10 | ported | `versioning/regex_versioning.rs` | `regex_invalid_config_throws` | — |
| works without config | 14 | ported | `versioning/regex_versioning.rs` | `regex_no_config` | — |
| works with missing version | 19 | ported | `versioning/regex_versioning.rs` | `regex_with_missing_version` | — |
| on invalid regex: "$regex" | 25 | ported | `versioning/regex_versioning.rs` | `regex_invalid_patterns` | — |
| isValid("$version") === $expected | 35 | ported | `versioning/regex_versioning.rs` | `regex_is_valid` | — |
| isCompatible("$version") === $expected | 58 | ported | `versioning/regex_versioning.rs` | `regex_is_compatible` | — |
| isSingleVersion("$version") === $expected | 83 | ported | `versioning/regex_versioning.rs` | `regex_is_single_version` | — |
| isStable("$version") === $expected | 104 | ported | `versioning/regex_versioning.rs` | `regex_is_stable` | — |
| isVersion("$version") === $expected | 115 | ported | `versioning/regex_versioning.rs` | `regex_is_version` | — |
| getMajor, getMinor, getPatch for "$version" | 135 | ported | `versioning/regex_versioning.rs` | `regex_major_minor_patch` | — |
| equals($a, $b) === $expected | 149 | ported | `versioning/regex_versioning.rs` | `regex_equals` | — |
| isGreaterThan("$a", "$b") === $expected | 171 | ported | `versioning/regex_versioning.rs` | `regex_is_greater_than` | — |
| isLessThanRange($version, $range) === $expected | 204 | ported | `versioning/regex_versioning.rs` | `regex_is_less_than_range` | — |
| getSatisfyingVersion($versions, "$range") === $expected | 253 | ported | `versioning/regex_versioning.rs` | `regex_get_satisfying_version` | — |
| minSatisfyingVersion($versions, "$range") === "$expected" | 267 | ported | `versioning/regex_versioning.rs` | `regex_min_satisfying_version` | — |
| returns newVersion | 282 | ported | `versioning/regex_versioning.rs` | `regex_get_new_value` | — |
| sorts versions in an ascending order | 295 | ported | `versioning/regex_versioning.rs` | `regex_sort_versions` | — |
| matches("$version", "$range") === $expected | 304 | ported | `versioning/regex_versioning.rs` | `regex_matches` | — |
| isValid("$version") === $expected | 365 | ported | `versioning/regex_versioning.rs` | `regex_build_revision_is_valid` | — |
| isCompatible("$version") === $expected | 373 | ported | `versioning/regex_versioning.rs` | `regex_build_revision_is_compatible` | — |
| isGreaterThan("$a", "$b") === $expected | 384 | ported | `versioning/regex_versioning.rs` | `regex_build_revision_is_greater_than` | — |
| matches("$version", "$range") === $expected | 392 | ported | `versioning/regex_versioning.rs` | `regex_build_revision_matches` | — |
| getSatisfyingVersion | 403 | ported | `versioning/regex_versioning.rs` | `regex_build_revision_satisfying` | — |
| minSatisfyingVersion | 412 | ported | `versioning/regex_versioning.rs` | `regex_build_revision_min_satisfying` | — |

---
