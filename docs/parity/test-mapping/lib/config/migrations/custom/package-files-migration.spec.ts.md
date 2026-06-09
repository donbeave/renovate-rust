# `lib/config/migrations/custom/package-files-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate value to array | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6005`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6005) |
| 21 | should handle multiple packagefile | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6018`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6018) |
| 32 | should still work for wrong config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6027`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6027) |
| 56 | should work for non-object packagefiles | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6045`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6045) |
| 67 | should work for nested rules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6054`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6054) |
| 99 | no change for empty packagefiles | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6073`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6073) |

