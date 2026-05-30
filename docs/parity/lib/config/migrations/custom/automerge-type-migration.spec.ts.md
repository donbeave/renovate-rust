# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/automerge-type-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/automerge-type-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 0 | **Status:** done

### `config/migrations/custom/automerge-type-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate string like "branch-" to "branch" | 4 | ported | crates/renovate-core/src/config/migrate_validate.rs | automerge_type_branch_prefix_migrates_to_branch | — |
| should not migrate another string value | 14 | ported | crates/renovate-core/src/config/migrate_validate.rs | automerge_type_non_branch_prefix_unchanged | — |
| should not migrate non string value | 25 | ported | crates/renovate-core/src/config/migrate_validate.rs | automerge_type_non_string_unchanged | — |

---
