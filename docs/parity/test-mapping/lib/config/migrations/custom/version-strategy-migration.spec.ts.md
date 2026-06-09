# `lib/config/migrations/custom/version-strategy-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**2/2 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate versionstrategy="widen" to rangestrategy="widen" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6636`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6636) |
| 15 | should just remove property when versionstrategy not equals to "widen" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6645`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6645) |

