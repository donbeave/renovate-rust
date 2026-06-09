# `lib/config/migrations/custom/branch-name-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**3/3 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should replace pattern | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6494`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6494) |
| 15 | should not replace another string | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6503`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6503) |
| 27 | should not replace non string value | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6512`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6512) |

