# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/automerge-minor-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/automerge-minor-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 0 | **Status:** done

### `config/migrations/custom/automerge-minor-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate value to object | 4 | ported | crates/renovate-core/src/config/migrate_validate.rs | automerge_minor_migrates_to_minor_object | — |
| should migrate value to object and concat with existing minor object | 16 | ported | crates/renovate-core/src/config/migrate_validate.rs | automerge_minor_merges_with_existing_minor_object | — |
| should ignore non object minor value | 32 | ported | crates/renovate-core/src/config/migrate_validate.rs | automerge_minor_replaces_null_minor_with_object | — |

---
