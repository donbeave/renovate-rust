# `lib/config/migrations/custom/package-files-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | should migrate value to array | ported | `crates/renovate-core/src/config/migrate_validate.rs:5963` |
| 21 | should handle multiple packagefile | ported | `crates/renovate-core/src/config/migrate_validate.rs:5976` |
| 32 | should still work for wrong config | ported | `crates/renovate-core/src/config/migrate_validate.rs:5985` |
| 56 | should work for non-object packagefiles | ported | `crates/renovate-core/src/config/migrate_validate.rs:6003` |
| 67 | should work for nested rules | ported | `crates/renovate-core/src/config/migrate_validate.rs:6012` |
| 99 | no change for empty packagefiles | ported | `crates/renovate-core/src/config/migrate_validate.rs:6031` |

