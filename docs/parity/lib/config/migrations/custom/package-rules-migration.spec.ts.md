# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/package-rules-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/package-rules-migration.spec.ts
**Total tests:** 8 | **Ported:** 7 | **Actionable:** 8 | **Status:** partial

### `config/migrations/custom/package-rules-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should preserve config order | 5 | pending | — | — | — |
| should not migrate nested packageRules | 31 | ported | `migrate_validate.rs` | `package_rules_renames_top_level_paths_without_nested_package_rules` | — |
| should migrate languages to categories | 53 | ported | `migrate_validate.rs` | `package_rules_languages_migrate_to_categories` | — |
| should migrate single match rule | 81 | ported | `migrate_validate.rs` | `package_rules_single_match_language_migrates_to_category` | — |
| should migrate excludePackageNames to matchPackageNames | 99 | ported | `migrate_validate.rs` | `package_rules_exclude_package_names_merge_into_match_package_names` | — |
| should migrate matchPackagePatterns to matchPackageNames | 127 | ported | `migrate_validate.rs` | `package_rules_match_package_patterns_merge_into_match_package_names` | — |
| should migrate all match/exclude when value is of type string | 163 | ported | `migrate_validate.rs` | `package_rules_string_matchers_merge_into_match_names` | — |
| should migrate all match/exclude at once | 222 | ported | `migrate_validate.rs` | `package_rules_array_matchers_merge_into_match_names` | — |

---

