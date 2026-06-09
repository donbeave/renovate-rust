# `lib/config/migrations/custom/unpublish-safe-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate true | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6232`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6232) |
| 15 | should migrate true and handle extends field | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6241`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6241) |
| 27 | should migrate true and handle empty extends field | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6250`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6250) |
| 39 | should migrate true and save order of items inside extends field | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6259`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6259) |
| 71 | should migrate false and save order of items inside extends field | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6282`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6282) |
| 83 | prevent duplicates | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6291`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6291) |
| 95 | should not migrate npm:unpublishsafe | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6302`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6302) |

