# `lib/config/migrations/custom/package-rules-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | should preserve config order | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5689`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5689) |
| 35 | should not migrate nested packagerules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5730`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5730) |
| 60 | should migrate languages to categories | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5749`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5749) |
| 89 | should migrate single match rule | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5768`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5768) |
| 110 | should migrate excludepackagenames to matchpackagenames | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5777`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5777) |
| 140 | should migrate matchpackagepatterns to matchpackagenames | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5795`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5795) |
| 178 | should migrate all match/exclude when value is of type string | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5814`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5814) |
| 223 | should migrate all match/exclude at once | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5857`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5857) |

