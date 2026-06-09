# `lib/config/migrations/custom/unpublish-safe-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate true | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6253`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6253) |
| 15 | should migrate true and handle extends field | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6262`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6262) |
| 27 | should migrate true and handle empty extends field | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6271`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6271) |
| 39 | should migrate true and save order of items inside extends field | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6280`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6280) |
| 71 | should migrate false and save order of items inside extends field | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6303`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6303) |
| 83 | prevent duplicates | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6312`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6312) |
| 95 | should not migrate npm:unpublishsafe | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6323`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6323) |

