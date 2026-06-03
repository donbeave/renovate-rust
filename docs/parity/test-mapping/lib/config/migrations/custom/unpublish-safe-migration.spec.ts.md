# `lib/config/migrations/custom/unpublish-safe-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**7/7 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | should migrate true | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6230`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6230) |
| 15 | should migrate true and handle extends field | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6239`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6239) |
| 27 | should migrate true and handle empty extends field | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6248`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6248) |
| 39 | should migrate true and save order of items inside extends field | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6257`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6257) |
| 71 | should migrate false and save order of items inside extends field | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6280`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6280) |
| 83 | prevent duplicates | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6289`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6289) |
| 95 | should not migrate npm:unpublishsafe | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6300`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6300) |

