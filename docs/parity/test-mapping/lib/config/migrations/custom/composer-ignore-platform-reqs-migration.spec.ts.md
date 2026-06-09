# `lib/config/migrations/custom/composer-ignore-platform-reqs-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**3/3 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate true to empty array | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5355`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5355) |
| 15 | should migrate false to null | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5364`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5364) |
| 26 | should not change array value | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5373`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5373) |

