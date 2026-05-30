# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/branch-prefix-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/branch-prefix-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 0 | **Status:** done

### `config/migrations/custom/branch-prefix-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate template | 4 | ported | `migrate_validate.rs` | `branch_prefix_parent_dir_template_migrates_to_additional_prefix` | — |
| should ignore string without template | 17 | ported | `migrate_validate.rs` | `branch_prefix_without_parent_dir_template_is_unchanged` | — |
| should ignore non string without template | 28 | ported | `migrate_validate.rs` | `branch_prefix_non_string_is_unchanged` | — |

---

