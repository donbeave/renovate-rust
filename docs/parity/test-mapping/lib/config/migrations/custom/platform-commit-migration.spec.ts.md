# `lib/config/migrations/custom/platform-commit-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**3/3 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate platformcommit=true to platformcommit=enabled | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6696`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6696) |
| 15 | should migrate platformcommit=false to platformcommit=disabled | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6705`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6705) |
| 26 | should not migrate platformcommit=auto | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6714`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6714) |

