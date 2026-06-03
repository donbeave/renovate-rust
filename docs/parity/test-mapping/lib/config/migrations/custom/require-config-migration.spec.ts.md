# `lib/config/migrations/custom/require-config-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**2/2 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 5 | should migrate requireconfig=true to requireconfig=required | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6543`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6543) |
| 16 | should migrate requireconfig=false to requireconfig=optional | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6552`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6552) |

