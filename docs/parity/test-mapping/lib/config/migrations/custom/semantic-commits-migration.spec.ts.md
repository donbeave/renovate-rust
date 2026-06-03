# `lib/config/migrations/custom/semantic-commits-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | should migrate true to "enabled" | ported | `crates/renovate-core/src/config/migrate_validate.rs:5200` |
| 13 | should migrate false to "disabled" | ported | `crates/renovate-core/src/config/migrate_validate.rs:5209` |
| 22 | should migrate null to "auto" | ported | `crates/renovate-core/src/config/migrate_validate.rs:5218` |
| 31 | should migrate random string to "auto" | ported | `crates/renovate-core/src/config/migrate_validate.rs:5227` |
| 40 | should not migrate valid enabled config | ported | `crates/renovate-core/src/config/migrate_validate.rs:5236` |
| 50 | should not migrate valid disabled config | ported | `crates/renovate-core/src/config/migrate_validate.rs:5245` |

