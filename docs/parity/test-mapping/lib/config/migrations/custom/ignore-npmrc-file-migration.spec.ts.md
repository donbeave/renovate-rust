# `lib/config/migrations/custom/ignore-npmrc-file-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**3/3 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | should init npmrc field | ported | `crates/renovate-core/src/config/migrate_validate.rs:6342` |
| 15 | should not change npmrc field if it represents string value | ported | `crates/renovate-core/src/config/migrate_validate.rs:6351` |
| 27 | should change npmrc field if it not represents string value | ported | `crates/renovate-core/src/config/migrate_validate.rs:6360` |

