# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/hex/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/hex/index.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 8 | **Status:** done

### `modules/versioning/hex/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| matches("$version", "$range") === $expected | 4 | ported | `versioning/hex.rs` | `hex_matches_parametrized` | — |
| getSatisfyingVersion($versions, "$range") === $expected | 19 | ported | `versioning/hex.rs` | `hex_get_satisfying_version_parametrized` | — |
| isValid("$input") === $expected | 30 | ported | `versioning/hex.rs` | `hex_is_valid_parametrized` | — |
| isSingleVersion("$version") === $expected | 41 | ported | `versioning/hex.rs` | `hex_is_single_version_parametrized` | — |
| getPinnedValue returns == prefixed version | 52 | ported | `versioning/hex.rs` | `hex_get_pinned_value` | — |
| isLessThanRange($version, $range) === $expected | 56 | ported | `versioning/hex.rs` | `hex_is_less_than_range_parametrized` | — |
| minSatisfyingVersion($versions, "$range") === $expected | 69 | ported | `versioning/hex.rs` | `hex_min_satisfying_version_parametrized` | — |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 80 | ported | `versioning/hex.rs` | `hex_get_new_value_parametrized` | — |

---

