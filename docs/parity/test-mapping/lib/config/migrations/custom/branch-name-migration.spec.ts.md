# `lib/config/migrations/custom/branch-name-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**3/3 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | should replace pattern | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6471`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6471) |
| 15 | should not replace another string | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6480`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6480) |
| 27 | should not replace non string value | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6489`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6489) |

