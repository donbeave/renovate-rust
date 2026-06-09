# `lib/config/migrations/custom/semantic-commits-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate true to "enabled" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5201`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5201) |
| 13 | should migrate false to "disabled" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5210`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5210) |
| 22 | should migrate null to "auto" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5219`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5219) |
| 31 | should migrate random string to "auto" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5228`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5228) |
| 40 | should not migrate valid enabled config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5237`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5237) |
| 50 | should not migrate valid disabled config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5246`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5246) |

