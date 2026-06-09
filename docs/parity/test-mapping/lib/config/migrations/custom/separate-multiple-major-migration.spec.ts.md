# `lib/config/migrations/custom/separate-multiple-major-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**2/2 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should remove if separatemajorreleases exists | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6116`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6116) |
| 16 | should skip if separatemajorreleases does not exist | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6125`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6125) |

