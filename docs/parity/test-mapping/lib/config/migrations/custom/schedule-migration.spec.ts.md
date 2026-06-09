# `lib/config/migrations/custom/schedule-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | migrates every friday | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5197`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5197) |
| 15 | does not migrate every weekday | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5206`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5206) |
| 27 | does not migrate multi days | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5215`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5215) |
| 39 | does not migrate hour range | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5224`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5224) |
| 51 | does not migrate invalid range | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5233`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5233) |

