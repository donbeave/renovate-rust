# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/semver/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/semver/index.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 0 | **Status:** done

### `modules/versioning/semver/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $expected | 4 | ported | crates/renovate-core/src/versioning/semver_node.rs | is_valid_matches_renovate_semver_spec | — |
| isSingleVersion("$version") === $expected | 22 | ported | crates/renovate-core/src/versioning/semver_node.rs | is_single_version_matches_renovate_semver_spec | — |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 34 | ported | crates/renovate-core/src/versioning/semver_node.rs | get_new_value_matches_renovate_semver_spec | — |
| isBreaking("$currentVersion", "$newVersion") === $expected | 51 | ported | crates/renovate-core/src/versioning/semver_node.rs | is_breaking_matches_renovate_semver_spec | — |
| isCompatible("$version") === $expected | 72 | ported | crates/renovate-core/src/versioning/semver_node.rs | is_compatible_matches_renovate_semver_spec | — |

---
