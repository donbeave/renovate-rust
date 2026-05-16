# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/path-rules-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/path-rules-migration.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `config/migrations/custom/path-rules-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate to packageRules | 4 | ported | `migrate_validate.rs` | `path_rules_migrate_to_package_rules` | — |
| should rewrite packageRules when it is not array | 22 | ported | `migrate_validate.rs` | `path_rules_rewrite_non_array_package_rules` | — |
| should not migrate non array value | 42 | ported | `migrate_validate.rs` | `path_rules_non_array_is_removed` | — |
| should concat with existing package rules | 50 | ported | `migrate_validate.rs` | `path_rules_append_to_existing_package_rules` | — |

---

