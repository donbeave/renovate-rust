# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/go-mod-tidy-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/go-mod-tidy-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 0 | **Status:** done

### `config/migrations/custom/go-mod-tidy-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should add postUpdateOptions option when true | 4 | ported | `migrate_validate.rs` | `gomod_tidy_true_appends_post_update_option` | — |
| should handle case when postUpdateOptions is not defined | 16 | ported | `migrate_validate.rs` | `gomod_tidy_true_initializes_post_update_options` | — |
| should only remove when false | 27 | ported | `migrate_validate.rs` | `gomod_tidy_false_is_removed` | — |

---

