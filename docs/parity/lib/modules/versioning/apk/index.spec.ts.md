# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/apk/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/apk/index.spec.ts
**Total tests:** 53 | **Ported:** 53 | **Actionable:** 53 | **Status:** ported

### `modules/versioning/apk/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid($version) === $expected | 5 | ported | crates/renovate-core/src/versioning/apk.rs | is_valid_matches_renovate_apk_index_spec | — |
| isStable($version) === $expected | 19 | ported | crates/renovate-core/src/versioning/apk.rs | is_stable_matches_renovate_apk_index_spec | — |
| getMajor($version) === $expected | 41 | ported | crates/renovate-core/src/versioning/apk.rs | get_major_matches_renovate_apk_index_spec | — |
| getMinor($version) === $expected | 51 | ported | crates/renovate-core/src/versioning/apk.rs | get_minor_matches_renovate_apk_index_spec | — |
| getPatch($version) === $expected | 61 | ported | crates/renovate-core/src/versioning/apk.rs | get_patch_matches_renovate_apk_index_spec | — |
| compare($a, $b) === $expected | 74 | ported | crates/renovate-core/src/versioning/apk.rs | compare_matches_renovate_apk_index_spec | — |
| isGreaterThan($a, $b) === $expected | 102 | ported | crates/renovate-core/src/versioning/apk.rs | is_greater_than_matches_renovate_apk_index_spec | — |
| equals($a, $b) === $expected | 115 | ported | crates/renovate-core/src/versioning/apk.rs | equals_matches_renovate_apk_index_spec | — |
| getSatisfyingVersion with exact match ($range) === $expected | 136 | ported | crates/renovate-core/src/versioning/apk.rs | get_satisfying_version_exact_matches_renovate_apk_index_spec | — |
| getSatisfyingVersion with range operator ($range) === $expected | 149 | ported | crates/renovate-core/src/versioning/apk.rs | get_satisfying_version_range_operator_matches_renovate_apk_index_spec | — |
| getSatisfyingVersion with tilde range ($range) === $expected | 164 | ported | crates/renovate-core/src/versioning/apk.rs | get_satisfying_version_tilde_matches_renovate_apk_index_spec | — |
| should return null for invalid range operators | 175 | ported | crates/renovate-core/src/versioning/apk.rs | get_satisfying_version_null_invalid_range_matches_renovate_apk_index_spec | — |
| should return null for empty versions array | 179 | ported | crates/renovate-core/src/versioning/apk.rs | get_satisfying_version_null_empty_array_matches_renovate_apk_index_spec | — |
| should filter out invalid versions | 183 | ported | crates/renovate-core/src/versioning/apk.rs | get_satisfying_version_filter_invalid_matches_renovate_apk_index_spec | — |
| isSingleVersion($version) === $expected | 192 | ported | crates/renovate-core/src/versioning/apk.rs | is_single_version_matches_renovate_apk_index_spec | — |
| should return false for empty versions | 202 | ported | crates/renovate-core/src/versioning/apk.rs | is_single_version_empty_matches_renovate_apk_index_spec | — |
| isLessThanRange($version, $range) === $expected | 210 | ported | crates/renovate-core/src/versioning/apk.rs | is_less_than_range_matches_renovate_apk_index_spec | — |
| should sort versions correctly | 225 | ported | crates/renovate-core/src/versioning/apk.rs | sort_versions_sort_correctly_matches_renovate_apk_index_spec | — |
| should compare release numbers when version parts are equal | 236 | ported | crates/renovate-core/src/versioning/apk.rs | sort_versions_release_numbers_matches_renovate_apk_index_spec | — |
| should parse complex versions ($version) === $expected | 246 | ported | crates/renovate-core/src/versioning/apk.rs | complex_version_parsing_is_valid_matches_renovate_apk_index_spec | — |
| should identify stable versions ($version) === $expected | 261 | ported | crates/renovate-core/src/versioning/apk.rs | complex_version_parsing_is_stable_matches_renovate_apk_index_spec | — |
| should compare versions with prerelease identifiers ($a, $b) === $expected | 278 | ported | crates/renovate-core/src/versioning/apk.rs | version_comparison_edge_cases_matches_renovate_apk_index_spec | — |
| should handle invalid version parsing gracefully | 295 | ported | crates/renovate-core/src/versioning/apk.rs | error_handling_invalid_gracefully_matches_renovate_apk_index_spec | — |
| should handle null/undefined inputs | 305 | ported | crates/renovate-core/src/versioning/apk.rs | error_handling_null_inputs_matches_renovate_apk_index_spec | — |
| should return false for unstable versions with prerelease | 315 | ported | crates/renovate-core/src/versioning/apk.rs | error_handling_unstable_prerelease_matches_renovate_apk_index_spec | — |
| should return false for empty versions in isStable | 321 | ported | crates/renovate-core/src/versioning/apk.rs | error_handling_empty_is_stable_matches_renovate_apk_index_spec | — |
| should handle versions with different major versions in tilde range | 329 | ported | crates/renovate-core/src/versioning/apk.rs | get_satisfying_version_tilde_major_matches_renovate_apk_index_spec | — |
| should handle versions with different minor versions in tilde range | 335 | ported | crates/renovate-core/src/versioning/apk.rs | get_satisfying_version_tilde_minor_matches_renovate_apk_index_spec | — |
| should handle invalid target versions in ranges | 340 | ported | crates/renovate-core/src/versioning/apk.rs | get_satisfying_version_invalid_target_matches_renovate_apk_index_spec | — |
| should handle versions with prerelease identifiers in ranges | 346 | ported | crates/renovate-core/src/versioning/apk.rs | get_satisfying_version_prerelease_ranges_matches_renovate_apk_index_spec | — |
| should return null for versions with _p package fix suffix | 358 | ported | crates/renovate-core/src/versioning/apk.rs | get_patch_edge_cases_p_suffix_matches_renovate_apk_index_spec | — |
| should return null for invalid versions | 364 | ported | crates/renovate-core/src/versioning/apk.rs | get_patch_edge_cases_invalid_matches_renovate_apk_index_spec | — |
| should return patch version for non-_p patterns | 370 | ported | crates/renovate-core/src/versioning/apk.rs | get_patch_edge_cases_non_p_matches_renovate_apk_index_spec | — |
| should handle versions with operators | 376 | ported | crates/renovate-core/src/versioning/apk.rs | get_patch_edge_cases_operators_matches_renovate_apk_index_spec | — |
| should strip revision from newVersion when currentValue has no revision | 384 | ported | crates/renovate-core/src/versioning/apk.rs | get_new_value_strip_revision_matches_renovate_apk_index_spec | — |
| should keep revision in newVersion when currentValue has revision | 394 | ported | crates/renovate-core/src/versioning/apk.rs | get_new_value_keep_revision_matches_renovate_apk_index_spec | — |
| should handle newVersion without revision when currentValue has no revision | 404 | ported | crates/renovate-core/src/versioning/apk.rs | get_new_value_no_revision_matches_renovate_apk_index_spec | — |
| should handle newVersion without revision when currentValue has revision | 414 | ported | crates/renovate-core/src/versioning/apk.rs | get_new_value_has_revision_no_new_revision_matches_renovate_apk_index_spec | — |
| should handle complex prerelease identifier comparisons | 426 | ported | crates/renovate-core/src/versioning/apk.rs | version_comparison_prerelease_complex_matches_renovate_apk_index_spec | — |
| should handle versions with different prerelease patterns | 438 | ported | crates/renovate-core/src/versioning/apk.rs | version_comparison_prerelease_patterns_matches_renovate_apk_index_spec | — |
| should handle unknown range operators | 445 | ported | crates/renovate-core/src/versioning/apk.rs | get_satisfying_version_unknown_operators_matches_renovate_apk_index_spec | — |
| should handle unhandled range operators that match regex | 456 | ported | crates/renovate-core/src/versioning/apk.rs | get_satisfying_version_unhandled_operators_matches_renovate_apk_index_spec | — |
| should handle tilde range with invalid target version | 467 | ported | crates/renovate-core/src/versioning/apk.rs | get_satisfying_version_tilde_invalid_target_matches_renovate_apk_index_spec | — |
| should handle tilde range with invalid version in list | 474 | ported | crates/renovate-core/src/versioning/apk.rs | get_satisfying_version_tilde_invalid_in_list_matches_renovate_apk_index_spec | — |
| should handle major-only versions without minor/patch | 485 | ported | crates/renovate-core/src/versioning/apk.rs | version_comparison_major_only_matches_renovate_apk_index_spec | — |
| should handle letter vs number at same position in version parts | 494 | ported | crates/renovate-core/src/versioning/apk.rs | version_comparison_letter_vs_number_matches_renovate_apk_index_spec | — |
| should handle number vs letter comparison in version parts | 499 | ported | crates/renovate-core/src/versioning/apk.rs | version_comparison_number_vs_letter_matches_renovate_apk_index_spec | — |
| should handle extra numeric parts in remaining segments | 504 | ported | crates/renovate-core/src/versioning/apk.rs | version_comparison_extra_numeric_matches_renovate_apk_index_spec | — |
| should handle lexicographic string comparison in version parts | 509 | ported | crates/renovate-core/src/versioning/apk.rs | version_comparison_lexicographic_matches_renovate_apk_index_spec | — |
| should handle equal letter parts continuing to next segment | 514 | ported | crates/renovate-core/src/versioning/apk.rs | version_comparison_equal_letter_continues_matches_renovate_apk_index_spec | — |
| should handle trailing letter in remaining segments | 519 | ported | crates/renovate-core/src/versioning/apk.rs | version_comparison_trailing_letter_matches_renovate_apk_index_spec | — |
| should return 0 for numerically equal but string-different versions | 524 | ported | crates/renovate-core/src/versioning/apk.rs | version_comparison_numeric_equal_string_diff_matches_renovate_apk_index_spec | — |
| should handle versions with different extra segment lengths | 528 | ported | crates/renovate-core/src/versioning/apk.rs | version_comparison_extra_segments_matches_renovate_apk_index_spec | — |

---

