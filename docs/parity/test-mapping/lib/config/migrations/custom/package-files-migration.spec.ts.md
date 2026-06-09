# `lib/config/migrations/custom/package-files-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate value to array | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5964`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5964) |
| 21 | should handle multiple packagefile | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5977`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5977) |
| 32 | should still work for wrong config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5986`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5986) |
| 56 | should work for non-object packagefiles | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6004`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6004) |
| 67 | should work for nested rules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6013`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6013) |
| 99 | no change for empty packagefiles | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6032`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6032) |

