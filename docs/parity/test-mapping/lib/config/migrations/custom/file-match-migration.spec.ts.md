# `lib/config/migrations/custom/file-match-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**4/4 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | migrates filematch of type string | ported | `crates/renovate-core/src/config/migrate_validate.rs:5551` |
| 15 | migrates filematch of type array | ported | `crates/renovate-core/src/config/migrate_validate.rs:5560` |
| 26 | concats filematch to managerfilepatterns | ported | `crates/renovate-core/src/config/migrate_validate.rs:5569` |
| 38 | does nothing if filematch not defined | ported | `crates/renovate-core/src/config/migrate_validate.rs:5580` |

