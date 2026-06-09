# `lib/config/migrations/custom/semantic-commits-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate true to "enabled" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5202`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5202) |
| 13 | should migrate false to "disabled" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5211`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5211) |
| 22 | should migrate null to "auto" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5220`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5220) |
| 31 | should migrate random string to "auto" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5229`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5229) |
| 40 | should not migrate valid enabled config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5238`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5238) |
| 50 | should not migrate valid disabled config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5247`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5247) |

