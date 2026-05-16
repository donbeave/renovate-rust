# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/include-forks-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/include-forks-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/migrations/custom/include-forks-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate true | 4 | ported | `migrate_validate.rs` | `include_forks_true_migrates_to_enabled_fork_processing` | — |
| should migrate false | 14 | ported | `migrate_validate.rs` | `include_forks_false_migrates_to_disabled_fork_processing` | — |
| should not migrate non boolean value | 24 | ported | `migrate_validate.rs` | `include_forks_non_boolean_is_removed` | — |

---

