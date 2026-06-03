# `lib/config/migrations/custom/suppress-notifications-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**3/3 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | should remomve preditnotification from array | ported | `crates/renovate-core/src/config/migrate_validate.rs:6172` |
| 15 | should not migrate array without preditnotification | ported | `crates/renovate-core/src/config/migrate_validate.rs:6181` |
| 27 | should not migrate empty array | ported | `crates/renovate-core/src/config/migrate_validate.rs:6190` |

