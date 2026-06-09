# `lib/config/migrations/custom/extends-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 5 | migrates preset strings to array | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5108`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5108) |
| 25 | migrates presets array | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5121`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5121) |
| 36 | should remove non string values | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5130`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5130) |
| 47 | should remove removed presets | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5139`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5139) |
| 58 | migrates presets | ported | [`crates/renovate-core/src/repo_config.rs:11679`](../../../../../../../crates/renovate-core/src/repo_config.rs#L11679) |
| 76 | migrate merge confidence config preset to internal preset | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5148`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5148) |

