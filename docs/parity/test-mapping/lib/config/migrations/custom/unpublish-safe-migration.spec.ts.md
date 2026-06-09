# `lib/config/migrations/custom/unpublish-safe-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate true | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6272`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6272) |
| 15 | should migrate true and handle extends field | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6281`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6281) |
| 27 | should migrate true and handle empty extends field | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6290`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6290) |
| 39 | should migrate true and save order of items inside extends field | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6299`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6299) |
| 71 | should migrate false and save order of items inside extends field | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6322`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6322) |
| 83 | prevent duplicates | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6331`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6331) |
| 95 | should not migrate npm:unpublishsafe | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6342`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6342) |

