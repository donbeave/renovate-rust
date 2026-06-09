# `lib/config/migrations/custom/file-match-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**4/4 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | migrates filematch of type string | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5552`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5552) |
| 15 | migrates filematch of type array | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5561`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5561) |
| 26 | concats filematch to managerfilepatterns | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5570`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5570) |
| 38 | does nothing if filematch not defined | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5581`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5581) |

