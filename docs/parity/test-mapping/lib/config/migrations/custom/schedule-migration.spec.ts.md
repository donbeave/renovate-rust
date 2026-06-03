# `lib/config/migrations/custom/schedule-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**5/5 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | migrates every friday | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5155`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5155) |
| 15 | does not migrate every weekday | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5164`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5164) |
| 27 | does not migrate multi days | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5173`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5173) |
| 39 | does not migrate hour range | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5182`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5182) |
| 51 | does not migrate invalid range | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5191`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5191) |

