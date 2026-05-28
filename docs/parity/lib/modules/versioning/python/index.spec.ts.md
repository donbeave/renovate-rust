# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/python/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/python/index.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 8 | **Status:** done

### `modules/versioning/python/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $expected | 4 | ported | `python.rs` | `is_valid_cases` | — |
| matches("$version", "$range") === "$expected" | 28 | ported | `python.rs` | `matches_cases` | — |
| isLessThanRange("$version", "$range") === "$expected" | 54 | ported | `python.rs` | `is_less_than_range_cases` | — |
| minSatisfyingVersion($versions, "$range") === $expected | 66 | ported | `python.rs` | `min_satisfying_version_cases` | — |
| getSatisfyingVersion($versions, "$range") === $expected | 83 | ported | `python.rs` | `get_satisfying_version_cases` | — |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 97 | ported | `versioning/python.rs` | `get_new_value_cases` | Delegates to poetry |
| subset("$a", "$b") === $expected | 160 | ported | `versioning/python.rs` | `subset_cases` | Delegates to poetry |
| isBreaking("$currentVersion", "$newVersion") === $expected | 182 | ported | `python.rs` | `is_breaking_cases` | — |

---

