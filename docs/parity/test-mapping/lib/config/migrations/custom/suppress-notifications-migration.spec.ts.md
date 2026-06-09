# `lib/config/migrations/custom/suppress-notifications-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**3/3 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should remomve preditnotification from array | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6195`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6195) |
| 15 | should not migrate array without preditnotification | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6204`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6204) |
| 27 | should not migrate empty array | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6213`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6213) |

