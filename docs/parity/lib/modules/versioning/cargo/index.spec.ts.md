# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/cargo/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/cargo/index.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 10 | **Status:** ported

### `modules/versioning/cargo/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| matches("$version", "$range") === "$expected" | 4 | ported | crates/renovate-core/src/versioning/cargo.rs | renovate_compat_tests::matches_cases | — |
| getSatisfyingVersion($versions, "$range") === "$expected" | 27 | ported | crates/renovate-core/src/versioning/cargo.rs | renovate_compat_tests::get_satisfying_version_cases | — |
| isValid("$version") === $expected | 40 | ported | crates/renovate-core/src/versioning/cargo.rs | renovate_compat_tests::is_valid_cases | — |
| isVersion("$version") === $expected | 53 | ported | crates/renovate-core/src/versioning/cargo.rs | renovate_compat_tests::is_version_cases | — |
| isLessThanRange("$version", "$range") === "$expected" | 61 | ported | crates/renovate-core/src/versioning/cargo.rs | renovate_compat_tests::is_less_than_range_cases | — |
| minSatisfyingVersion($versions, "$range") === "$expected" | 74 | ported | crates/renovate-core/src/versioning/cargo.rs | renovate_compat_tests::min_satisfying_version_cases | — |
| isSingleVersion("$version") === $expected | 92 | ported | crates/renovate-core/src/versioning/cargo.rs | renovate_compat_tests::is_single_version_cases | — |
| returns a pinned value | 107 | ported | crates/renovate-core/src/versioning/cargo.rs | renovate_compat_tests::get_pinned_value_case | — |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 111 | ported | crates/renovate-core/src/versioning/cargo.rs | renovate_compat_tests::get_new_value_cases | — |
| isBreaking("$currentVersion", "$newVersion") === $expected | 176 | ported | crates/renovate-core/src/versioning/cargo.rs | renovate_compat_tests::is_breaking_cases | — |

---

