# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/package-files-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/package-files-migration.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 0 | **Status:** done

### `config/migrations/custom/package-files-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate value to array | 4 | ported | `migrate_validate.rs` | `package_files_object_migrates_to_include_paths_and_package_rules` | — |
| should handle multiple packageFile | 21 | ported | `migrate_validate.rs` | `package_files_nested_array_migrates_to_include_paths` | — |
| should still work for wrong config | 34 | ported | `migrate_validate.rs` | `package_files_appends_to_existing_package_rules` | — |
| should work for non-object packageFiles | 55 | ported | `migrate_validate.rs` | `package_files_string_migrates_to_include_paths` | — |
| should work for nested rules | 65 | ported | `migrate_validate.rs` | `package_files_preserves_nested_rules` | — |
| no change for empty packageFiles | 92 | ported | `migrate_validate.rs` | `package_files_empty_is_removed_without_other_changes` | — |

---

