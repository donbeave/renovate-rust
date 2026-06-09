# `lib/config/migrations/custom/package-files-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate value to array | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5986`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5986) |
| 21 | should handle multiple packagefile | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5999`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5999) |
| 32 | should still work for wrong config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6008`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6008) |
| 56 | should work for non-object packagefiles | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6026`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6026) |
| 67 | should work for nested rules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6035`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6035) |
| 99 | no change for empty packagefiles | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6054`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6054) |

