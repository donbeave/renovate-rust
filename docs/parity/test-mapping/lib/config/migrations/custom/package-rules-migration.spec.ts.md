# `lib/config/migrations/custom/package-rules-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | should preserve config order | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5708`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5708) |
| 35 | should not migrate nested packagerules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5749`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5749) |
| 60 | should migrate languages to categories | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5768`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5768) |
| 89 | should migrate single match rule | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5787`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5787) |
| 110 | should migrate excludepackagenames to matchpackagenames | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5796`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5796) |
| 140 | should migrate matchpackagepatterns to matchpackagenames | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5814`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5814) |
| 178 | should migrate all match/exclude when value is of type string | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5833`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5833) |
| 223 | should migrate all match/exclude at once | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5876`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5876) |

