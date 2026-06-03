# `lib/config/migrations/custom/extends-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 5 | migrates preset strings to array | ported | `crates/renovate-core/src/config/migrate_validate.rs:5106` |
| 25 | migrates presets array | ported | `crates/renovate-core/src/config/migrate_validate.rs:5119` |
| 36 | should remove non string values | ported | `crates/renovate-core/src/config/migrate_validate.rs:5128` |
| 47 | should remove removed presets | ported | `crates/renovate-core/src/config/migrate_validate.rs:5137` |
| 58 | migrates presets | ported | `crates/renovate-core/src/repo_config.rs:11679` |
| 76 | migrate merge confidence config preset to internal preset | ported | `crates/renovate-core/src/config/migrate_validate.rs:5146` |

