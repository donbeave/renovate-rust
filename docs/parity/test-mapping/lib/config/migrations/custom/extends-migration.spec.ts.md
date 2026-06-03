# `lib/config/migrations/custom/extends-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 5 | migrates preset strings to array | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5106`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5106) |
| 25 | migrates presets array | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5119`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5119) |
| 36 | should remove non string values | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5128`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5128) |
| 47 | should remove removed presets | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5137`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5137) |
| 58 | migrates presets | ported | [`crates/renovate-core/src/repo_config.rs:11679`](../../../../../../../crates/renovate-core/src/repo_config.rs#L11679) |
| 76 | migrate merge confidence config preset to internal preset | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5146`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5146) |

