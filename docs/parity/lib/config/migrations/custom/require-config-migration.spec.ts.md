# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/require-config-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/require-config-migration.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 0 | **Status:** done

### `config/migrations/custom/require-config-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate requireConfig=true to requireConfig=required | 4 | ported | `migrate_validate.rs` | `require_config_true_string_migrates_to_required` | — |
| should migrate requireConfig=false to requireConfig=optional | 14 | ported | `migrate_validate.rs` | `require_config_false_string_migrates_to_optional` | — |

---

