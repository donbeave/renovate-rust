# `lib/config/migrations/custom/package-rules-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | should preserve config order | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5667`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5667) |
| 35 | should not migrate nested packagerules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5708`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5708) |
| 60 | should migrate languages to categories | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5727`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5727) |
| 89 | should migrate single match rule | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5746`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5746) |
| 110 | should migrate excludepackagenames to matchpackagenames | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5755`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5755) |
| 140 | should migrate matchpackagepatterns to matchpackagenames | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5773`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5773) |
| 178 | should migrate all match/exclude when value is of type string | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5792`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5792) |
| 223 | should migrate all match/exclude at once | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5835`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5835) |

