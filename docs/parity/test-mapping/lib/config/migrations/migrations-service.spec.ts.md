# `lib/config/migrations/migrations-service.spec.ts`

[← `config/migrations`](../../../_by-module/config/migrations.md) · [all modules](../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | should remove deprecated properties | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6902`](../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6902) |
| 23 | should rename renamed properties | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6932`](../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6932) |
| 42 | should save original order of properties | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6967`](../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6967) |
| 60 | should allow custom migrations by regexp | ported | [`crates/renovate-core/src/config/migrate_validate.rs:7063`](../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L7063) |
| 89 | there should be a single migration per property name | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6997`](../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6997) |
| 104 | includes all defined migration classes in migrationsservice.custommigrations | ported | [`crates/renovate-core/src/config/migrate_validate.rs:7092`](../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L7092) |

