# `lib/config/migrations/custom/renovate-fork-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**3/3 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate true | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6420`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6420) |
| 15 | should migrate false | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6429`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6429) |
| 26 | should not migrate non boolean value | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6438`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6438) |

