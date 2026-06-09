# `lib/config/migrations/migrations-service.spec.ts`

[← `config/migrations`](../../../_by-module/config/migrations.md) · [all modules](../../../README.md)

**4/6 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | should remove deprecated properties | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6900`](../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6900) |
| 23 | should rename renamed properties | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6930`](../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6930) |
| 42 | should save original order of properties | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6965`](../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6965) |
| 60 | should allow custom migrations by regexp | pending | — |
| 89 | there should be a single migration per property name | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6995`](../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6995) |
| 104 | includes all defined migration classes in migrationsservice.custommigrations | pending | — |

