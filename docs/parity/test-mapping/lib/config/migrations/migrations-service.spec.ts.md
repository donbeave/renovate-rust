# `lib/config/migrations/migrations-service.spec.ts`

[← `config/migrations`](../../../_by-module/config/migrations.md) · [all modules](../../../README.md)

**4/4 in-scope tests ported** (0 pending, 2 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | should remove deprecated properties | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6923`](../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6923) |
| 23 | should rename renamed properties | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6953`](../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6953) |
| 42 | should save original order of properties | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6988`](../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6988) |
| 60 | should allow custom migrations by regexp | opt-out | tests support for regexp-based propertyName in custom AbstractMigration subclasses (TypeScript custom migration extension mechanism with no direct 1:1 in Rust's static Migration impls and string property_name matching) |
| 89 | there should be a single migration per property name | ported | [`crates/renovate-core/src/config/migrate_validate.rs:7018`](../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L7018) |
| 104 | includes all defined migration classes in migrationsservice.custommigrations | opt-out | counts TypeScript source files in the custom/ directory vs the static customMigrations list (filesystem + module discovery / build-time reflection with no direct Rust equivalent; Rust migrations are statically registered) |

