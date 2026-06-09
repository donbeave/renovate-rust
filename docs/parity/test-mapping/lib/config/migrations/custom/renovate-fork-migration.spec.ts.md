# `lib/config/migrations/custom/renovate-fork-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**3/3 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate true | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6443`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6443) |
| 15 | should migrate false | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6452`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6452) |
| 26 | should not migrate non boolean value | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6461`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6461) |

