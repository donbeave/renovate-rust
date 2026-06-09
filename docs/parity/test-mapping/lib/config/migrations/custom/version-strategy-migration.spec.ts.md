# `lib/config/migrations/custom/version-strategy-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**2/2 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate versionstrategy="widen" to rangestrategy="widen" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6637`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6637) |
| 15 | should just remove property when versionstrategy not equals to "widen" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6646`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6646) |

