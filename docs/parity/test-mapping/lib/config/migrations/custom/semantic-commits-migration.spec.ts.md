# `lib/config/migrations/custom/semantic-commits-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate true to "enabled" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5242`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5242) |
| 13 | should migrate false to "disabled" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5251`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5251) |
| 22 | should migrate null to "auto" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5260`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5260) |
| 31 | should migrate random string to "auto" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5269`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5269) |
| 40 | should not migrate valid enabled config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5278`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5278) |
| 50 | should not migrate valid disabled config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5287`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5287) |

