# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/semantic-prefix-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/semantic-prefix-migration.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 0 | **Status:** done

### `config/migrations/custom/semantic-prefix-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should work | 4 | ported | `migrate_validate.rs` | `semantic_prefix_migrates_type_and_scope` | — |
| should remove non-string values | 12 | ported | `migrate_validate.rs` | `semantic_prefix_non_string_is_removed` | — |
| should migrate prefix with no-scope to null | 21 | ported | `migrate_validate.rs` | `semantic_prefix_without_scope_migrates_scope_to_null` | — |
| works for random string | 30 | ported | `migrate_validate.rs` | `semantic_prefix_random_string_migrates_type_with_null_scope` | — |

---

