# `lib/config/migrations/custom/package-rules-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | should preserve config order | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5668`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5668) |
| 35 | should not migrate nested packagerules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5709`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5709) |
| 60 | should migrate languages to categories | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5728`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5728) |
| 89 | should migrate single match rule | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5747`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5747) |
| 110 | should migrate excludepackagenames to matchpackagenames | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5756`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5756) |
| 140 | should migrate matchpackagepatterns to matchpackagenames | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5774`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5774) |
| 178 | should migrate all match/exclude when value is of type string | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5793`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5793) |
| 223 | should migrate all match/exclude at once | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5836`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5836) |

