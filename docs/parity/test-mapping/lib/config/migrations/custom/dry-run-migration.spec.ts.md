# `lib/config/migrations/custom/dry-run-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**2/2 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate dryrun=true to dryrun=full | ported | [`crates/renovate-core/src/config/migration/migrations/dry_run_migration.rs:63`](../../../../../../../crates/renovate-core/src/config/migration/migrations/dry_run_migration.rs#L63) |
| 15 | should migrate dryrun=false to dryrun=null | ported | [`crates/renovate-core/src/config/migration/migrations/dry_run_migration.rs:72`](../../../../../../../crates/renovate-core/src/config/migration/migrations/dry_run_migration.rs#L72) |

