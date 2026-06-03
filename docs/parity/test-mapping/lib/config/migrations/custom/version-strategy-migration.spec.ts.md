# `lib/config/migrations/custom/version-strategy-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**2/2 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | should migrate versionstrategy="widen" to rangestrategy="widen" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6636`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6636) |
| 15 | should just remove property when versionstrategy not equals to "widen" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6645`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6645) |

