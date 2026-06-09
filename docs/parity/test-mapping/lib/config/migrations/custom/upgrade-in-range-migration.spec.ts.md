# `lib/config/migrations/custom/upgrade-in-range-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**2/2 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate upgradeinrange=true to rangestrategy="bump" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6663`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6663) |
| 15 | should just remove property when upgradeinrange not equals to true | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6672`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6672) |

