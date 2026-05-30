# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/unpublish-safe-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/unpublish-safe-migration.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 0 | **Status:** done

### `config/migrations/custom/unpublish-safe-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate true | 4 | ported | `migrate_validate.rs` | `unpublish_safe_true_injects_security_preset` | — |
| should migrate true and handle extends field | 14 | ported | `migrate_validate.rs` | `unpublish_safe_true_handles_string_extends` | — |
| should migrate true and handle empty extends field | 26 | ported | `migrate_validate.rs` | `unpublish_safe_true_handles_empty_extends` | — |
| should migrate true and save order of items inside extends field | 38 | ported | `migrate_validate.rs` | `unpublish_safe_true_rewrites_supported_extends_in_place` | — |
| should migrate false and save order of items inside extends field | 68 | ported | `migrate_validate.rs` | `unpublish_safe_false_is_removed_and_preserves_extends` | — |
| prevent duplicates | 80 | ported | `migrate_validate.rs` | `unpublish_safe_true_does_not_duplicate_security_preset` | — |
| should not migrate npm:unpublishSafe | 92 | ported | `migrate_validate.rs` | `unpublish_safe_absent_leaves_npm_unpublish_safe_extends` | — |

---

