# `lib/config/migrations/custom/ignore-npmrc-file-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**3/3 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should init npmrc field | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6344`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6344) |
| 15 | should not change npmrc field if it represents string value | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6353`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6353) |
| 27 | should change npmrc field if it not represents string value | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6362`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6362) |

