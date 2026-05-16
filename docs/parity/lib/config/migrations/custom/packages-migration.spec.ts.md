# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/packages-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/packages-migration.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `config/migrations/custom/packages-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate to package rules | 4 | ported | `migrate_validate.rs` | `packages_migrates_to_package_rules` | — |
| should concat with existing package rules | 14 | ported | `migrate_validate.rs` | `packages_appends_to_existing_package_rules` | — |
| should ignore non array value | 26 | ported | `migrate_validate.rs` | `packages_non_array_is_removed` | — |

---

