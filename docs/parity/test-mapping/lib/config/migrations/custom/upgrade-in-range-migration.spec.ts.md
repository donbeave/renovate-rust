# `lib/config/migrations/custom/upgrade-in-range-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**2/2 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | should migrate upgradeinrange=true to rangestrategy="bump" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6621`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6621) |
| 15 | should just remove property when upgradeinrange not equals to true | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6630`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6630) |

