# `lib/config/migrations/custom/semantic-prefix-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**4/4 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should work | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5277`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5277) |
| 13 | should remove non-string values | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5286`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5286) |
| 22 | should migrate prefix with no-scope to null | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5292`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5292) |
| 31 | works for random string | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5301`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5301) |

