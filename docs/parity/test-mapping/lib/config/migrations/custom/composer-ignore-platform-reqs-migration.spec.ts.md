# `lib/config/migrations/custom/composer-ignore-platform-reqs-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**3/3 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | should migrate true to empty array | ported | `crates/renovate-core/src/config/migrate_validate.rs:5354` |
| 15 | should migrate false to null | ported | `crates/renovate-core/src/config/migrate_validate.rs:5363` |
| 26 | should not change array value | ported | `crates/renovate-core/src/config/migrate_validate.rs:5372` |

