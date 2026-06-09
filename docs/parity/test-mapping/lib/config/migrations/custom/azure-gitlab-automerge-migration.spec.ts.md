# `lib/config/migrations/custom/azure-gitlab-automerge-migration.spec.ts`

[← `config/migrations`](../../../../_by-module/config/migrations.md) · [all modules](../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 4 | should migrate non undefined gitlabautomerge | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5310`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5310) |
| 15 | should just remove undefined gitlabautomerge | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5346`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5346) |
| 24 | should override platformautomerge when gitlabautomerge defined | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5319`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5319) |
| 36 | should migrate non undefined azureautocomplete | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5328`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5328) |
| 47 | should just remove undefined azureautocomplete | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5354`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5354) |
| 56 | should override platformautomerge when azureautocomplete defined | ported | [`crates/renovate-core/src/config/migrate_validate.rs:5337`](../../../../../../../crates/renovate-core/src/config/migrate_validate.rs#L5337) |

