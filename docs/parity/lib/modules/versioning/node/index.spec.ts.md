# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/node/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/node/index.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 0 | **Status:** done

### `modules/versioning/node/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getNewValue($currentValue, $rangeStrategy, $currentVersion, $newVersion, $expected) === $expected | 14 | ported | crates/renovate-core/src/versioning/node.rs | get_new_value_matches_renovate_node_index_spec | — |
| isStable("$version") === $expected | 43 | ported | crates/renovate-core/src/versioning/node.rs | is_stable_matches_renovate_node_index_spec | — |
| isValid("$version") === $expected | 64 | ported | crates/renovate-core/src/versioning/node.rs | is_valid_matches_renovate_node_index_spec | — |
| matches("$version", "$range") === $expected | 75 | ported | crates/renovate-core/src/versioning/node.rs | matches_matches_renovate_node_index_spec | — |
| getSatisfyingVersion("$versions", "$range") === $expected | 87 | ported | crates/renovate-core/src/versioning/node.rs | get_satisfying_version_matches_renovate_node_index_spec | — |
| minSatisfyingVersion("$versions", "$range") === $expected | 102 | ported | crates/renovate-core/src/versioning/node.rs | min_satisfying_version_matches_renovate_node_index_spec | — |

---

