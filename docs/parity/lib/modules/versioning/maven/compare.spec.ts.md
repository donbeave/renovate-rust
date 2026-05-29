# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/maven/compare.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/maven/compare.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 10 | **Status:** ported

### `modules/versioning/maven/compare`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $x == $y | 15 | ported | crates/renovate-core/src/versioning/maven.rs | compare_equals_matches_renovate_maven_compare_spec | — |
| $x < $y | 106 | ported | crates/renovate-core/src/versioning/maven.rs | compare_ordering_matches_renovate_maven_compare_spec | — |
| $qualifier | 203 | ported | crates/renovate-core/src/versioning/maven.rs | qualifier_mng7644_matches_renovate_maven_compare_spec | — |
| isSubversion("$majorVersion", "$minorVersion") === $expected | 226 | ported | crates/renovate-core/src/versioning/maven.rs | is_subversion_matches_renovate_maven_compare_spec | — |
| should tokenize | 454 | ported | `maven.rs` | `tokenize_matches_renovate_maven_compare_spec` | — |
| $x == $y | 463 | ported | crates/renovate-core/src/versioning/maven.rs | compare_nonstandard_equals_matches_renovate_maven_compare_spec | — |
| $x < $y | 478 | ported | crates/renovate-core/src/versioning/maven.rs | compare_nonstandard_ordering_matches_renovate_maven_compare_spec | — |
| filters out incorrect range: $input | 490 | ported | crates/renovate-core/src/versioning/maven.rs | parse_range_filters_invalid_matches_renovate_maven_compare_spec | — |
| parseRange("$input") | 521 | ported | crates/renovate-core/src/versioning/maven.rs | parse_range_valid_matches_renovate_maven_compare_spec | — |
| autoExtendMavenRange("$range", "$version") === $expected | 560 | ported | crates/renovate-core/src/versioning/maven.rs | auto_extend_maven_range_matches_renovate_maven_compare_spec | — |

---

