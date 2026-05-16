# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/github-actions/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/github-actions/index.spec.ts
**Total tests:** 29 | **Ported:** 28 | **Actionable:** 28 | **Status:** ported

### `modules/versioning/github-actions/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $expected | 6 | ported | `github_actions.rs` | `is_valid_matches_renovate_github_actions_spec` | — |
| isVersion("$version") === $expected | 30 | ported | `github_actions.rs` | `is_version_matches_renovate_github_actions_spec` | — |
| isStable("$version") === $expected | 54 | ported | `github_actions.rs` | `is_stable_matches_renovate_github_actions_spec` | — |
| isSingleVersion("$version") === $expected | 81 | ported | `github_actions.rs` | `is_single_version_matches_renovate_github_actions_spec` | — |
| matches("$version", "$range") === $expected | 99 | ported | `github_actions.rs` | `matches_floating_ranges_and_versions_like_renovate` | — |
| should not handle invalid range that is not ~latest or valid version | 158 | ported | `github_actions.rs` | `matches_rejects_invalid_ranges` | — |
| getSatisfyingVersion($versions, "$range") === $expected | 166 | ported | `github_actions.rs` | `get_satisfying_version_matches_renovate_github_actions_spec` | — |
| minSatisfyingVersion($versions, "$range") === $expected | 202 | ported | `github_actions.rs` | `min_satisfying_version_matches_renovate_github_actions_spec` | — |
| isLessThanRange("$version", "$range") === $expected | 226 | ported | `github_actions.rs` | `is_less_than_range_matches_renovate_github_actions_spec` | — |
| equals("$version", "$other") === $expected | 260 | ported | `github_actions.rs` | `equals_matches_renovate_github_actions_spec` | — |
| getMajor("$version") === $expected | 287 | ported | `github_actions.rs` | `component_getters_match_renovate_github_actions_spec` | — |
| getMinor("$version") === $expected | 302 | ported | `github_actions.rs` | `component_getters_match_renovate_github_actions_spec` | — |
| getPatch("$version") === $expected | 316 | ported | `github_actions.rs` | `component_getters_match_renovate_github_actions_spec` | — |
| isGreaterThan("$version", "$other") === $expected | 330 | ported | `github_actions.rs` | `is_greater_than_matches_renovate_github_actions_spec` | — |
| sortVersions("$a", "$b") === $expected | 364 | ported | `github_actions.rs` | `sort_versions_matches_renovate_github_actions_spec` | — |
| isBreaking("$version", "$current") === $expected | 394 | ported | `github_actions.rs` | `is_breaking_matches_renovate_github_actions_spec` | — |
| isCompatible("$version") === $expected | 422 | ported | `github_actions.rs` | `is_compatible_matches_renovate_github_actions_spec` | — |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 436 | ported | `github_actions.rs` | `get_new_value_matches_renovate_github_actions_spec` | — |
| does not determine if the proposed newVersion exists, if allVersions is not set | 502 | ported | `github_actions.rs` | `get_new_value_without_all_versions_returns_floating_major` | — |
| does not determine if the proposed newVersion exists, if allVersions is an empty array: %s -> %s | 514 | ported | `github_actions.rs` | `get_new_value_empty_all_versions_behaves_like_absent_all_versions` | — |
| %s | 532 | ported | `github_actions.rs` | `get_new_value_uses_shortest_existing_matching_version` | — |
| preserves floating major for non-major updates ($description) | 562 | ported | `github_actions.rs` | `get_new_value_preserves_floating_major_for_non_major_updates` | — |
| migrates from a floating major to a floating major.minor if the floating major no longer exists | 614 | ported | `github_actions.rs` | `get_new_value_migrates_to_floating_minor_when_floating_major_missing` | — |
| preserves floating minor for non-major updates ($description) | 625 | ported | `github_actions.rs` | `get_new_value_preserves_floating_minor_for_non_major_updates` | — |
| when a release candidate version exists, that exact version is used | 658 | ported | `github_actions.rs` | `get_new_value_uses_existing_release_candidate` | — |
| returns newVersion when newVersion is a floating tag and allVersions is not set | 675 | ported | `github_actions.rs` | `get_new_value_returns_floating_new_version_without_all_versions` | — |
| returns the floating newVersion when it exists in allVersions | 685 | ported | `github_actions.rs` | `get_new_value_returns_existing_floating_new_version` | — |
| newVersion is returned anyway | 698 | ported | `github_actions.rs` | `get_new_value_returns_missing_new_version_anyway` | — |
| debug logs | 709 | not-applicable | - | - | Rust versioning module does not expose Renovate's JS logger mock; fallback return behavior is covered by `get_new_value_returns_missing_new_version_anyway`. |

---

