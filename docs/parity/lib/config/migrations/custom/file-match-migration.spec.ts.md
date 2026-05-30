# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/file-match-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/file-match-migration.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 0 | **Status:** done

### `config/migrations/custom/file-match-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| migrates fileMatch of type string | 4 | ported | `migrate_validate.rs` | `file_match_string_migrates_to_manager_file_patterns` | — |
| migrates fileMatch of type array | 14 | ported | `migrate_validate.rs` | `file_match_array_migrates_to_manager_file_patterns` | — |
| concats fileMatch to managerFilePatterns | 24 | ported | `migrate_validate.rs` | `file_match_appends_to_existing_manager_file_patterns` | — |
| does nothing if fileMatch not defined | 38 | ported | `migrate_validate.rs` | `missing_file_match_leaves_manager_file_patterns_unchanged` | — |

---

