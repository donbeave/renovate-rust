# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/go-mod-directive/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/go-mod-directive/index.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 0 | **Status:** done

### `modules/versioning/go-mod-directive/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| matches("$version", "$range") === "$expected" | 4 | ported | crates/renovate-core/src/versioning/go_mod_directive.rs | matches_matches_renovate_go_mod_directive_index_spec | — |
| getSatisfyingVersion($versions, "$range") === "$expected" | 19 | ported | crates/renovate-core/src/versioning/go_mod_directive.rs | get_satisfying_version_matches_renovate_go_mod_directive_index_spec | — |
| isValid("$version") === $expected | 29 | ported | crates/renovate-core/src/versioning/go_mod_directive.rs | is_valid_matches_renovate_go_mod_directive_index_spec | — |
| isVersion("$version") === $expected | 38 | ported | crates/renovate-core/src/versioning/go_mod_directive.rs | is_version_matches_renovate_go_mod_directive_index_spec | — |
| isLessThanRange("$version", "$range") === "$expected" | 47 | ported | crates/renovate-core/src/versioning/go_mod_directive.rs | is_less_than_range_matches_renovate_go_mod_directive_index_spec | — |
| minSatisfyingVersion($versions, "$range") === "$expected" | 58 | ported | crates/renovate-core/src/versioning/go_mod_directive.rs | min_satisfying_version_matches_renovate_go_mod_directive_index_spec | — |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 69 | ported | crates/renovate-core/src/versioning/go_mod_directive.rs | get_new_value_matches_renovate_go_mod_directive_index_spec | — |

---

