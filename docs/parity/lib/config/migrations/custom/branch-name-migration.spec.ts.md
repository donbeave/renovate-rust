# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/branch-name-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/branch-name-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 0 | **Status:** done

### `config/migrations/custom/branch-name-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should replace pattern | 4 | ported | `migrate_validate.rs` | `branch_name_manager_branch_prefix_migrates_to_additional_branch_prefix` | — |
| should not replace another string | 14 | ported | `migrate_validate.rs` | `branch_name_without_manager_branch_prefix_is_unchanged` | — |
| should not replace non string value | 25 | ported | `migrate_validate.rs` | `branch_name_non_string_is_unchanged` | — |

---

