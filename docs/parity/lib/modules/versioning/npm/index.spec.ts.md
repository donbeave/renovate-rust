# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/npm/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/npm/index.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 0 | **Status:** done

### `modules/versioning/npm/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $isValid | 4 | ported | `npm.rs` | `is_valid_matches_renovate_npm_spec` | — |
| getSatisfyingVersion("$versions","$range") === $maxSatisfying | 29 | ported | `npm.rs` | `get_satisfying_version_matches_renovate_npm_spec` | — |
| isSingleVersion("$version") === $isSingle | 49 | ported | `npm.rs` | `is_single_version_matches_renovate_npm_spec` | — |
| subset("$a", "$b") === $expected | 61 | ported | `npm.rs` | `subset_matches_renovate_npm_spec` | — |
| intersects("$a", "$b") === $expected | 84 | ported | `npm.rs` | `intersects_matches_renovate_npm_spec` | — |
| isBreaking("$currentVersion", "$newVersion") === $expected | 107 | ported | `npm.rs` | `is_breaking_matches_renovate_npm_spec` | — |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 122 | ported | `npm.rs` | `get_new_value_matches_renovate_npm_spec` | — |

---

