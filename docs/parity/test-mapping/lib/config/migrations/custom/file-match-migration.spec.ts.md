# `lib/config/migrations/custom/file-match-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**4/4 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | migrates filematch of type string | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5551`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5551) |
| 15 | migrates filematch of type array | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5560`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5560) |
| 26 | concats filematch to managerfilepatterns | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5569`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5569) |
| 38 | does nothing if filematch not defined | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5580`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5580) |

