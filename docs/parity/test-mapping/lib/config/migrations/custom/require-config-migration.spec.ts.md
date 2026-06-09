# `lib/config/migrations/custom/require-config-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**2/2 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 5 | should migrate requireconfig=true to requireconfig=required | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6566`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6566) |
| 16 | should migrate requireconfig=false to requireconfig=optional | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6575`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6575) |

