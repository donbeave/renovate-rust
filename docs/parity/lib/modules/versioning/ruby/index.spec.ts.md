# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/ruby/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/ruby/index.spec.ts
**Total tests:** 15 | **Ported:** 15 | **Actionable:** 15 | **Status:** ported

### `modules/versioning/ruby/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| equals("$a", "$b") === $expected | 4 | ported | `ruby.rs` | `ruby_equals_cases` | — |
| getMajor, getMinor, getPatch for "$version" | 21 | ported | `ruby.rs` | `ruby_get_components` | — |
| isVersion("$version") === $expected | 38 | ported | `ruby.rs` | `ruby_is_version_cases` | — |
| isGreaterThan("$a", "$b") === $expected | 62 | ported | `ruby.rs` | `ruby_is_greater_than_cases` | — |
| isStable("$version") === $expected | 106 | ported | `ruby.rs` | `ruby_is_stable` | — |
| $versions -> sortVersions -> $expected | 122 | ported | `ruby.rs` | `ruby_sort_versions` | — |
| minSatisfyingVersion($versions, "$range") === "$expected" | 129 | ported | `ruby.rs` | `ruby_min_satisfying_version` | — |
| getSatisfyingVersion($versions, "$range") === "$expected" | 147 | ported | `ruby.rs` | `ruby_get_satisfying_version` | — |
| matches("$version", "$range") === "$expected" | 165 | ported | `ruby.rs` | `ruby_matches` | — |
| isLessThanRange("$version", "$range") === "$expected" | 185 | ported | `ruby.rs` | `ruby_is_less_than_range` | — |
| isValid("$version") === $expected | 209 | ported | `ruby.rs` | `ruby_is_valid_version_form` | — |
| isValid("$version") === $expected | 224 | ported | `ruby.rs` | `ruby_is_valid_range_form` | — |
| isSingleVersion("$version") === $expected | 247 | ported | `ruby.rs` | `ruby_is_single_version` | — |
| returns a pinned value | 276 | ported | `ruby.rs` | `ruby_get_pinned_value` | — |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 281 | ported | `ruby.rs` | `ruby_get_new_value_cases` | — |

---
