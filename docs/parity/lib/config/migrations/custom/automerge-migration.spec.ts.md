# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/automerge-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/automerge-migration.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `config/migrations/custom/automerge-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate none | 4 | ported | `repo_config.rs` | `automerge_legacy_none_string_migrated_to_false` | — |
| should migrate patch | 16 | ported | `migrate_validate.rs` | `automerge_patch_sets_nested_update_type_configs` | — |
| should migrate minor | 34 | ported | `migrate_validate.rs` | `automerge_minor_sets_nested_update_type_configs` | — |
| should migrate any | 49 | ported | `repo_config.rs` | `automerge_legacy_any_string_migrated_to_true` | — |

---

