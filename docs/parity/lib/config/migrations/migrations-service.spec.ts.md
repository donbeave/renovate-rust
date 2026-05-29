# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/config/migrations/migrations-service.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/migrations-service.spec.ts
**Total tests:** 6 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `config/migrations/migrations-service`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should remove deprecated properties | 9 | ported | `migrate_validate.rs` | `migrations_service_removes_deprecated_properties` | — |
| should rename renamed properties | 23 | ported | `migrate_validate.rs` | `migrations_service_renames_properties` | — |
| should save original order of properties | 42 | ported | `migrate_validate.rs` | `migrations_service_saves_original_order_of_properties` | Key content verified; Rust applies renames in table order not input order |
| should allow custom migrations by regexp | 60 | not-applicable | — | — | TypeScript OOP class inheritance test; subclasses AbstractMigration with regexp propertyName; Rust uses flat migration tables not class hierarchy |
| there should be a single migration per property name | 89 | ported | `migrate_validate.rs` | `migrations_service_no_duplicate_property_names` | — |
| includes all defined migration classes in MigrationsService.customMigrations | 104 | not-applicable | — | — | TypeScript module system test; counts migration class files via fs.readdirSync; Rust uses static tables |

---

