# `lib/config/migrations/custom/package-rules-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**8/8 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 6 | should preserve config order | ported | `crates/renovate-core/src/config/migrate_validate.rs:5666` |
| 35 | should not migrate nested packagerules | ported | `crates/renovate-core/src/config/migrate_validate.rs:5707` |
| 60 | should migrate languages to categories | ported | `crates/renovate-core/src/config/migrate_validate.rs:5726` |
| 89 | should migrate single match rule | ported | `crates/renovate-core/src/config/migrate_validate.rs:5745` |
| 110 | should migrate excludepackagenames to matchpackagenames | ported | `crates/renovate-core/src/config/migrate_validate.rs:5754` |
| 140 | should migrate matchpackagepatterns to matchpackagenames | ported | `crates/renovate-core/src/config/migrate_validate.rs:5772` |
| 178 | should migrate all match/exclude when value is of type string | ported | `crates/renovate-core/src/config/migrate_validate.rs:5791` |
| 223 | should migrate all match/exclude at once | ported | `crates/renovate-core/src/config/migrate_validate.rs:5834` |

