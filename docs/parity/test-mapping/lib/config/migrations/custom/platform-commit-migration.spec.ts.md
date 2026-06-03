# `lib/config/migrations/custom/platform-commit-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**3/3 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | should migrate platformcommit=true to platformcommit=enabled | ported | `crates/renovate-core/src/config/migrate_validate.rs:6654` |
| 15 | should migrate platformcommit=false to platformcommit=disabled | ported | `crates/renovate-core/src/config/migrate_validate.rs:6663` |
| 26 | should not migrate platformcommit=auto | ported | `crates/renovate-core/src/config/migrate_validate.rs:6672` |

