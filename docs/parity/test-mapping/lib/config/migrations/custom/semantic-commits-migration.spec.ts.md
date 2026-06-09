# `lib/config/migrations/custom/semantic-commits-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate true to "enabled" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5223`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5223) |
| 13 | should migrate false to "disabled" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5232`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5232) |
| 22 | should migrate null to "auto" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5241`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5241) |
| 31 | should migrate random string to "auto" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5250`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5250) |
| 40 | should not migrate valid enabled config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5259`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5259) |
| 50 | should not migrate valid disabled config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5268`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5268) |

