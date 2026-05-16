# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/node-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/node-migration.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `config/migrations/custom/node-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate node to travis | 4 | ported | `migrate_validate.rs` | `node_enabled_migrates_to_travis_enabled` | — |
| should not delete node in case it has more than one property | 14 | ported | `migrate_validate.rs` | `node_enabled_migration_preserves_other_node_options` | — |

---

