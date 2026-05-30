# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/pep440/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/pep440/index.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 0 | **Status:** done

### `modules/versioning/pep440/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$input") === $expected | 4 | ported | `pep440.rs` | `is_valid_table` | — |
| isStable("$input") === $expected | 25 | ported | `pep440.rs` | `is_stable_table` | — |
| equals($a, $b) === $expected | 34 | ported | `pep440.rs` | `equals_table` | — |
| matches($a, $b) === $expected | 42 | ported | `pep440.rs` | `matches_table` | — |
| isSingleVersion("$version") === $isSingle | 53 | ported | `pep440.rs` | `is_single_version_table` | — |
| getSatisfyingVersion($versions, "$range") === $expected | 78 | ported | `pep440.rs` | `get_satisfying_version_table` | — |
| minSatisfyingVersion($versions, "$range") === $expected | 89 | ported | `pep440.rs` | `min_satisfying_version_table` | — |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 100 | ported | `pep440.rs` | `get_new_value_table` | — |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 190 | ported | `pep440.rs` | `get_new_value_replacement_table` | — |
| isLessThanRange("$version", "$range") === "$expected" | 307 | ported | `pep440.rs` | `is_less_than_range_table` | — |

---

