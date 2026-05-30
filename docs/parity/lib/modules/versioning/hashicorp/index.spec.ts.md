# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/hashicorp/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/hashicorp/index.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 0 | **Status:** done

### `modules/versioning/hashicorp/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| matches("$version", "$range") === $expected | 4 | ported | crates/renovate-core/src/versioning/hashicorp.rs | matches_matches_renovate_hashicorp_spec | — |
| getSatisfyingVersion($versions, "$range") === $expected | 17 | ported | crates/renovate-core/src/versioning/hashicorp.rs | get_satisfying_version_matches_renovate_hashicorp_spec | — |
| isValid("$input") === $expected | 29 | ported | crates/renovate-core/src/versioning/hashicorp.rs | is_valid_matches_renovate_hashicorp_spec | — |
| isLessThanRange($version, $range) === $expected | 48 | ported | crates/renovate-core/src/versioning/hashicorp.rs | is_less_than_range_matches_renovate_hashicorp_spec | — |
| minSatisfyingVersion($versions, "$range") === $expected | 59 | ported | crates/renovate-core/src/versioning/hashicorp.rs | min_satisfying_version_matches_renovate_hashicorp_spec | — |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 72 | ported | crates/renovate-core/src/versioning/hashicorp.rs | get_new_value_matches_renovate_hashicorp_spec | — |

---

