# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/poetry/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/poetry/index.spec.ts
**Total tests:** 14 | **Ported:** 14 | **Actionable:** 14 | **Status:** ported

### `modules/versioning/poetry/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals("$a", "$b") === $expected | 5 | ported | `poetry.rs` | `equals_basic` | — |
| getMajor, getMinor, getPatch for "$version" | 28 | ported | `poetry.rs` | `get_version_parts` | — |
| isGreaterThan("$a", "$b") === $expected | 47 | ported | `poetry.rs` | `is_greater_than_cases` | — |
| isStable("$version") === $expected | 82 | ported | `poetry.rs` | `is_stable_cases` | — |
| isVersion("$version") === $expected | 95 | ported | `poetry.rs` | `is_version_cases` | — |
| isValid("$version") === $expected | 107 | ported | `poetry.rs` | `is_valid_cases` | — |
| isSingleVersion("$version") === $expected | 134 | ported | `poetry.rs` | `is_single_version_cases` | — |
| matches("$version", "$range") === "$expected" | 145 | ported | `poetry.rs` | `matches_cases` | — |
| isLessThanRange("$version", "$range") === "$expected" | 167 | ported | `poetry.rs` | `is_less_than_range_cases` | — |
| minSatisfyingVersion($versions, "$range") === $expected | 178 | ported | `versioning/poetry.rs` | `min_satisfying_version_cases` | — |
| getSatisfyingVersion($versions, "$range") === $expected | 194 | ported | `versioning/poetry.rs` | `get_satisfying_version_cases` | — |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 207 | ported | `versioning/poetry.rs` | `get_new_value_cases` | Core cases ported |
| sortVersions("$a", "$b") === $expected | 269 | ported | `versioning/poetry.rs` | `sort_versions_cases` | — |
| subset("$a", "$b") === $expected | 287 | ported | `versioning/poetry.rs` | `subset_cases` | — |

---

