# `lib/config/migrations/custom/update-lock-files-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**3/3 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should replace false value | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6597`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6597) |
| 15 | should not replace true value | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6606`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6606) |
| 24 | should not replace skipartifactsupdate | ported | [`crates/renovate-core/src/config/migrate_validate.rs:6612`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L6612) |

