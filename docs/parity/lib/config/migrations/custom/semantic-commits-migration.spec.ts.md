# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/semantic-commits-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/semantic-commits-migration.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 0 | **Status:** done

### `config/migrations/custom/semantic-commits-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate true to "enabled" | 4 | ported | `migrate_validate.rs` | `semantic_commits_true_migrates_to_enabled` | — |
| should migrate false to "disabled" | 13 | ported | `migrate_validate.rs` | `semantic_commits_false_migrates_to_disabled` | — |
| should migrate null to "auto" | 22 | ported | `migrate_validate.rs` | `semantic_commits_null_migrates_to_auto` | — |
| should migrate random string to "auto" | 31 | ported | `migrate_validate.rs` | `semantic_commits_random_string_migrates_to_auto` | — |
| should not migrate valid enabled config | 40 | ported | `migrate_validate.rs` | `semantic_commits_enabled_is_unchanged` | — |
| should not migrate valid disabled config | 51 | ported | `migrate_validate.rs` | `semantic_commits_disabled_is_unchanged` | — |

---

