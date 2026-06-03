# `lib/config/migrations/custom/renovate-fork-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**3/3 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | should migrate true | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6420`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6420) |
| 15 | should migrate false | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6429`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6429) |
| 26 | should not migrate non boolean value | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6438`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6438) |

